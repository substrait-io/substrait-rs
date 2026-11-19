// SPDX-License-Identifier: Apache-2.0

//! Parse type syntax with the ANTLR parser generated from the Substrait type
//! grammar ([`substrait_antlr::substrait_type`]), then convert its parse tree
//! into a [`TypeExpr`].
//!
//! The grammar also describes type derivation expressions (arithmetic,
//! conditionals, multiline assignments). Those parse successfully but are
//! rejected here as [`TypeParseError::UnsupportedNumericExpression`].
//!
//! Names borrow from the source using the byte offsets of their tokens.
//! Built-in names, parameter counts, and literal ranges are validated later
//! when converting the AST to [`Type`](super::types::Type).

use std::cell::RefCell;
use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::Token;
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::tree::TerminalNode;
use antlr4rust::{InputStream, TidExt};
use substrait_antlr::substrait_type::substraittypeparser::*;
use substrait_antlr::substrait_type::{SubstraitTypeLexer, SubstraitTypeParser};

use super::type_ast::{TypeExpr, TypeExprParam, TypeParseError};

/// Parse one complete type, rejecting any tokens left after it.
/// For example, `list<i32>` succeeds, but `list<i32> i64` does not.
pub(super) fn parse(source: &str) -> Result<TypeExpr<'_>, TypeParseError> {
    // The type grammar has no variation syntax, so report it before ANTLR
    // turns it into a generic syntax error.
    if source.contains(['[', ']']) {
        return Err(TypeParseError::UnsupportedVariation(source.to_string()));
    }

    let first_error = Rc::new(RefCell::new(None));

    let mut lexer = SubstraitTypeLexer::new(InputStream::new(source));
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(FirstSyntaxError(Rc::clone(&first_error))));

    let mut parser = SubstraitTypeParser::new(CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(FirstSyntaxError(Rc::clone(&first_error))));

    let tree = parser.startRule();
    if let Some(error) = first_error.take() {
        return Err(error);
    }
    let tree = tree.map_err(|error| TypeParseError::Syntax {
        position: 0,
        message: error.to_string(),
    })?;

    let converter = TreeConverter { source };
    converter.type_expr(&tree.expr().expect("startRule always has an expr"))
}

/// Records the first error reported by the lexer or parser.
///
/// ANTLR recovers from syntax errors and keeps going, so the parse tree is
/// only trustworthy when no error was reported.
struct FirstSyntaxError(Rc<RefCell<Option<TypeParseError>>>);

impl<'input, T: Recognizer<'input>> ErrorListener<'input, T> for FirstSyntaxError {
    fn syntax_error(
        &self,
        _recognizer: &T,
        offending_symbol: Option<&<T::TF as TokenFactory<'input>>::Inner>,
        _line: isize,
        column: isize,
        message: &str,
        _error: Option<&ANTLRError>,
    ) {
        let mut first_error = self.0.borrow_mut();
        if first_error.is_none() {
            let position = offending_symbol.map_or(column, |token| token.get_start());
            *first_error = Some(TypeParseError::Syntax {
                position: position.max(0) as usize,
                message: message.to_string(),
            });
        }
    }
}

/// Converts the ANTLR parse tree into a [`TypeExpr`] borrowing from `source`.
struct TreeConverter<'a> {
    source: &'a str,
}

impl<'a> TreeConverter<'a> {
    /// The source text of a single token.
    fn token_text(&self, token: &impl Token) -> &'a str {
        &self.source[token.get_start() as usize..=token.get_stop() as usize]
    }

    fn terminal_text(&self, node: &TerminalNode<'a, SubstraitTypeParserContextType>) -> &'a str {
        self.token_text(&*node.symbol)
    }

    /// The text of an identifier, rejecting multi-digit `anyN` spellings.
    ///
    /// The grammar lexes `any12` as an ordinary identifier, but those names are
    /// reserved for future type-variable support.
    /// https://github.com/substrait-io/substrait/issues/1335
    fn identifier(
        &self,
        node: &TerminalNode<'a, SubstraitTypeParserContextType>,
    ) -> Result<&'a str, TypeParseError> {
        let name = self.terminal_text(node);
        let is_multi_digit_variable = name.len() > 4
            && name[..3].eq_ignore_ascii_case("any")
            && name[3..].bytes().all(|byte| byte.is_ascii_digit());
        if is_multi_digit_variable {
            return Err(TypeParseError::MultiDigitTypeVariable(name.to_string()));
        }
        Ok(name)
    }

    /// Convert an expression that must be a type, rejecting integer literals.
    fn type_expr(&self, expr: &ExprContextAll<'a>) -> Result<TypeExpr<'a>, TypeParseError> {
        match self.parameter(expr)? {
            TypeExprParam::Type(type_expr) => Ok(type_expr),
            TypeExprParam::Integer(value) => Err(TypeParseError::ExpectedType(value.to_string())),
        }
    }

    /// Convert an expression in parameter position: a type or an integer literal.
    fn parameter(&self, expr: &ExprContextAll<'a>) -> Result<TypeExprParam<'a>, TypeParseError> {
        match expr {
            ExprContextAll::TypeLiteralContext(literal) => Ok(TypeExprParam::Type(
                self.type_def(&literal.typeDef().expect("type literal has a typeDef"))?,
            )),
            ExprContextAll::ParameterNameContext(name) => {
                Ok(TypeExprParam::Type(TypeExpr::Simple(
                    self.identifier(&name.Identifier().expect("parameter name has an identifier"))?,
                    Vec::new(),
                    name.QMark().is_some(),
                )))
            }
            ExprContextAll::LiteralNumberContext(number) => {
                self.integer(&number.Number().expect("number literal has a number"))
            }
            ExprContextAll::ParenExpressionContext(parenthesized) => self.parameter(
                &parenthesized
                    .expr()
                    .expect("parenthesized expr has an expr"),
            ),
            _ => Err(TypeParseError::UnsupportedNumericExpression(
                self.source.to_string(),
            )),
        }
    }

    fn integer(
        &self,
        number: &TerminalNode<'a, SubstraitTypeParserContextType>,
    ) -> Result<TypeExprParam<'a>, TypeParseError> {
        let text = self.terminal_text(number);
        text.parse()
            .map(TypeExprParam::Integer)
            .map_err(|_| TypeParseError::InvalidInteger(text.to_string()))
    }

    fn type_def(&self, type_def: &TypeDefContextAll<'a>) -> Result<TypeExpr<'a>, TypeParseError> {
        if let Some(scalar) = type_def.scalarType() {
            let name = self.token_text(&*scalar.start());
            return Ok(TypeExpr::Simple(
                name,
                Vec::new(),
                type_def.QMark().is_some(),
            ));
        }
        if let Some(any) = type_def.anyType() {
            return Ok(self.any_type(&any));
        }
        let parameterized = type_def
            .parameterizedType()
            .expect("typeDef is a scalar, any, or parameterized type");
        self.parameterized_type(&parameterized)
    }

    fn any_type(&self, any: &AnyTypeContextAll<'a>) -> TypeExpr<'a> {
        let nullable = any.QMark().is_some();
        if let Some(variable) = any.AnyVar() {
            let digit = &self.terminal_text(&variable)[3..];
            return TypeExpr::TypeVariable(
                digit.parse().expect("AnyVar ends in a digit"),
                nullable,
            );
        }
        let name = self.terminal_text(&any.Any().expect("anyType is Any or AnyVar"));
        TypeExpr::Simple(name, Vec::new(), nullable)
    }

    fn parameterized_type(
        &self,
        parameterized: &ParameterizedTypeContextAll<'a>,
    ) -> Result<TypeExpr<'a>, TypeParseError> {
        use ParameterizedTypeContextAll as P;

        let nullable = has_question_mark(parameterized);
        match parameterized {
            P::FixedCharContext(_)
            | P::VarCharContext(_)
            | P::FixedBinaryContext(_)
            | P::DecimalContext(_)
            | P::PrecisionIntervalDayContext(_)
            | P::PrecisionIntervalCompoundContext(_)
            | P::PrecisionTimeContext(_)
            | P::PrecisionTimestampContext(_)
            | P::PrecisionTimestampTZContext(_)
            | P::StructContext(_)
            | P::ListContext(_)
            | P::MapContext(_) => Ok(TypeExpr::Simple(
                self.token_text(&*parameterized.start()),
                self.child_parameters(parameterized)?,
                nullable,
            )),
            P::FuncContext(ctx) => {
                let parameters =
                    self.child_parameters(&**ctx.params.as_ref().expect("func has parameters"))?;
                let return_type =
                    self.type_expr(ctx.returnType.as_ref().expect("func has a return type"))?;
                Ok(TypeExpr::Func(parameters, Box::new(return_type), nullable))
            }
            P::UserDefinedContext(ctx) => {
                if ctx.dependencyAlias.is_some() {
                    return Err(TypeParseError::UnsupportedSyntax {
                        construct: "dependency-qualified user-defined",
                        type_string: self.source.to_string(),
                    });
                }
                let name = self.identifier(&ctx.Identifier(0).expect("u! has a name"))?;
                Ok(TypeExpr::UserDefined(
                    name,
                    self.child_parameters(parameterized)?,
                    nullable,
                ))
            }
            P::NStructContext(_) => Err(TypeParseError::UnsupportedSyntax {
                construct: "named struct",
                type_string: self.source.to_string(),
            }),
            P::Error(_) => unreachable!("syntax errors are reported"),
        }
    }

    /// Convert the parameters among `node`'s direct children, in order.
    ///
    /// For `decimal<P, 2>` these are the two numeric parameters; for
    /// `map<string, i64>` or `(i32, i64)` they are the type expressions.
    /// Tokens such as `<`, `,` and `?` are skipped.
    fn child_parameters(
        &self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
    ) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        node.get_children()
            .filter_map(|child| {
                if let Some(expr) = child.downcast_ref::<ExprContextAll<'a>>() {
                    Some(self.parameter(expr))
                } else {
                    child
                        .downcast_ref::<NumericParameterContextAll<'a>>()
                        .map(|parameter| self.numeric_parameter(parameter))
                }
            })
            .collect()
    }

    /// Convert a numeric parameter: an integer literal, a named parameter such
    /// as `P`, or an expression.
    fn numeric_parameter(
        &self,
        parameter: &NumericParameterContextAll<'a>,
    ) -> Result<TypeExprParam<'a>, TypeParseError> {
        match parameter {
            NumericParameterContextAll::NumericLiteralContext(literal) => {
                self.integer(&literal.Number().expect("numeric literal has a number"))
            }
            NumericParameterContextAll::NumericParameterNameContext(name) => {
                Ok(TypeExprParam::Type(TypeExpr::Simple(
                    self.identifier(&name.Identifier().expect("parameter name"))?,
                    Vec::new(),
                    false,
                )))
            }
            NumericParameterContextAll::NumericExpressionContext(expression) => {
                self.parameter(&expression.expr().expect("numeric expression has an expr"))
            }
            NumericParameterContextAll::Error(_) => unreachable!("syntax errors are reported"),
        }
    }
}

/// Whether `node` has a `?` among its direct children, marking it nullable.
fn has_question_mark<'a>(node: &(dyn SubstraitTypeParserContext<'a> + 'a)) -> bool {
    node.get_children().any(|child| {
        child
            .downcast_ref::<TerminalNode<'a, SubstraitTypeParserContextType>>()
            .is_some_and(|token| token.symbol.get_token_type() == SubstraitType_QMark)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer_parameters_cover_the_i64_range() {
        for (source, expected) in [
            ("decimal<0, 0>", 0),
            ("u!box<-2>", -2),
            ("u!box<9223372036854775807>", i64::MAX),
            ("u!box<-9223372036854775808>", i64::MIN),
        ] {
            let (TypeExpr::Simple(_, parameters, _) | TypeExpr::UserDefined(_, parameters, _)) =
                parse(source).unwrap()
            else {
                panic!("{source} should be a named type");
            };
            assert_eq!(parameters[0], TypeExprParam::Integer(expected), "{source}");
        }
        assert_eq!(
            parse("u!box<9223372036854775808>"),
            Err(TypeParseError::InvalidInteger(
                "9223372036854775808".to_string()
            ))
        );
    }

    #[test]
    fn keyword_names_keep_their_spelling() {
        assert_eq!(
            parse("LiSt<DeCiMaL<P, S>>").unwrap(),
            TypeExpr::Simple(
                "LiSt",
                vec![TypeExprParam::Type(parse("DeCiMaL<P, S>").unwrap())],
                false
            ),
        );
    }

    #[test]
    fn numbered_type_variables_are_case_insensitive() {
        for (source, id, nullable) in [("any0", 0, false), ("ANY1", 1, false), ("Any9?", 9, true)] {
            assert_eq!(
                parse(source).unwrap(),
                TypeExpr::TypeVariable(id, nullable),
                "{source}"
            );
        }
    }

    #[test]
    fn multi_digit_type_variable_names_are_rejected() {
        for source in ["any10", "list<ANY12>", "decimal<any12, 0>", "u!any10"] {
            assert!(
                matches!(
                    parse(source),
                    Err(TypeParseError::MultiDigitTypeVariable(_))
                ),
                "{source}"
            );
        }
    }

    #[test]
    fn derivation_expressions_are_not_supported() {
        for source in [
            "decimal<P1 + 1, S>",
            "u!box<max(P, 2)>",
            "decimal<P > 1 ? P : 1, 0>",
        ] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnsupportedNumericExpression(
                    source.to_string()
                )),
            );
        }
    }

    #[test]
    fn grammar_features_without_an_ast_form_are_rejected() {
        for source in ["nstruct<a: i32>", "ext.u!point"] {
            assert!(
                matches!(parse(source), Err(TypeParseError::UnsupportedSyntax { .. })),
                "{source}"
            );
        }
        assert_eq!(
            parse("10"),
            Err(TypeParseError::ExpectedType("10".to_string()))
        );
    }

    #[test]
    fn syntax_errors_report_the_offending_byte() {
        assert!(matches!(
            parse("list<i32)"),
            Err(TypeParseError::Syntax { position: 8, .. })
        ));
    }
}
