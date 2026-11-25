// SPDX-License-Identifier: Apache-2.0

//! Parse type strings with the ANTLR parser generated from the Substrait type
//! grammar ([`substrait_antlr::substrait_type`]), then convert the parse tree
//! into a [`TypeExpr`]. [`TreeConverter`] has one method per grammar rule,
//! each documented with the rule it converts.
//!
//! Type derivation expressions such as `P + 1` parse but are rejected as
//! [`TypeParseError::UnsupportedNumericExpression`]. Built-in names, parameter
//! counts and literal ranges are validated later, when converting to
//! [`Type`](super::types::Type).

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

    let tree = parse_tree(source)?;
    let expr = tree.expr().expect("startRule is `expr EOF`");
    TreeConverter { source }.expr_as_type(&expr)
}

/// Run the generated parser on `source`, returning its `startRule` tree.
///
/// The tree is only returned when neither the lexer nor the parser reported
/// an error, so [`TreeConverter`] can rely on it matching the grammar.
fn parse_tree(source: &str) -> Result<Rc<StartRuleContextAll<'_>>, TypeParseError> {
    // ANTLR's default listeners print errors to stderr, then recover and still
    // return a tree. Replace them with one that records the first error, which
    // is the only way to tell that the input was invalid.
    let first_error = FirstSyntaxError::default();

    let mut lexer = SubstraitTypeLexer::new(InputStream::new(source));
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(first_error.clone()));

    let mut parser = SubstraitTypeParser::new(CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(first_error.clone()));

    let tree = parser.startRule();
    if let Some(error) = first_error.0.take() {
        return Err(error);
    }
    tree.map_err(|error| TypeParseError::Syntax {
        column: 0,
        message: error.to_string(),
    })
}

/// Keeps the first syntax error reported by the lexer or parser.
///
/// ANTLR reports errors to listeners and then recovers, so `startRule` can
/// return a tree even for invalid input. The lexer and parser each own a
/// boxed listener, so the error is shared back to [`parse_tree`] through an
/// `Rc`.
#[derive(Clone, Default)]
struct FirstSyntaxError(Rc<RefCell<Option<TypeParseError>>>);

// The signature is fixed by `antlr4rust`'s `ErrorListener` trait. Only
// `column` and `message` are used.
impl<'input, T: Recognizer<'input>> ErrorListener<'input, T> for FirstSyntaxError {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<T::TF as TokenFactory<'input>>::Inner>,
        _line: isize,
        column: isize,
        message: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.0
            .borrow_mut()
            .get_or_insert_with(|| TypeParseError::Syntax {
                column: column as usize,
                message: message.to_string(),
            });
    }
}

/// Converts the parse tree into a [`TypeExpr`], with one method per grammar
/// rule. Names borrow from `source` using the byte offsets of their tokens.
///
/// The `expect`s on child accessors rely on the tree matching the grammar,
/// which [`parse_tree`] guarantees.
struct TreeConverter<'a> {
    source: &'a str,
}

impl<'a> TreeConverter<'a> {
    /// Convert an `expr` that must be a type, such as a top-level type or a
    /// function's return type. `10` is rejected here.
    fn expr_as_type(&self, expr: &ExprContextAll<'a>) -> Result<TypeExpr<'a>, TypeParseError> {
        match self.expr(expr)? {
            TypeExprParam::Type(type_expr) => Ok(type_expr),
            TypeExprParam::Integer(value) => Err(TypeParseError::ExpectedType(value.to_string())),
        }
    }

    /// Convert an `expr`, which in parameter position is a type or an integer.
    ///
    /// ```text
    /// expr: typeDef | Identifier '?'? | Number | '(' expr ')' | <derivations>
    /// ```
    fn expr(&self, expr: &ExprContextAll<'a>) -> Result<TypeExprParam<'a>, TypeParseError> {
        match expr {
            ExprContextAll::TypeLiteralContext(literal) => Ok(TypeExprParam::Type(
                self.type_def(&literal.typeDef().expect("#TypeLiteral is a typeDef"))?,
            )),
            ExprContextAll::ParameterNameContext(name) => {
                Ok(TypeExprParam::Type(TypeExpr::Simple(
                    self.terminal_text(
                        &name.Identifier().expect("#ParameterName is an Identifier"),
                    ),
                    Vec::new(),
                    name.QMark().is_some(),
                )))
            }
            ExprContextAll::LiteralNumberContext(number) => {
                self.integer(&number.Number().expect("#LiteralNumber is a Number"))
            }
            ExprContextAll::ParenExpressionContext(parenthesized) => self.expr(
                &parenthesized
                    .expr()
                    .expect("#ParenExpression wraps an expr"),
            ),
            _ => Err(TypeParseError::UnsupportedNumericExpression(
                self.source.to_string(),
            )),
        }
    }

    /// Convert a `typeDef`, such as `i32?`, `list<i32>` or `any1`.
    ///
    /// ```text
    /// typeDef: scalarType '?'? | parameterizedType | anyType
    /// ```
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
            .expect("typeDef is a scalarType, parameterizedType or anyType");
        self.parameterized_type(&parameterized)
    }

    /// Convert an `anyType`, such as `any` or `any1?`. `AnyVar` is always `any`
    /// followed by one digit.
    ///
    /// ```text
    /// anyType: Any '?'? | AnyVar '?'?
    /// ```
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

    /// Convert a `parameterizedType`, which is a keyword builtin such as
    /// `decimal<P, 2>` or `list?<i32>`, a `func<...>`, a `u!name<...>` or an
    /// `nstruct<...>`.
    ///
    /// The keyword builtins only differ in which parameters they take, so they
    /// share one arm that reads the parameters from the node's children.
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
                    self.child_parameters(&**ctx.params.as_ref().expect("#func has funcParams"))?;
                let return_type =
                    self.expr_as_type(ctx.returnType.as_ref().expect("#func has a return type"))?;
                Ok(TypeExpr::Func(parameters, Box::new(return_type), nullable))
            }
            P::UserDefinedContext(ctx) => {
                if ctx.dependencyAlias.is_some() {
                    return Err(TypeParseError::UnsupportedSyntax {
                        construct: "dependency-qualified user-defined",
                        type_string: self.source.to_string(),
                    });
                }
                let name = self.terminal_text(&ctx.Identifier(0).expect("#userDefined has a name"));
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
            P::Error(_) => unreachable!("parse_tree rejects trees with errors"),
        }
    }

    /// Convert a `numericParameter`, such as decimal's precision and scale.
    ///
    /// ```text
    /// numericParameter: Number | Identifier | expr
    /// ```
    fn numeric_parameter(
        &self,
        parameter: &NumericParameterContextAll<'a>,
    ) -> Result<TypeExprParam<'a>, TypeParseError> {
        match parameter {
            NumericParameterContextAll::NumericLiteralContext(literal) => {
                self.integer(&literal.Number().expect("#numericLiteral is a Number"))
            }
            NumericParameterContextAll::NumericParameterNameContext(name) => {
                Ok(TypeExprParam::Type(TypeExpr::Simple(
                    self.terminal_text(
                        &name
                            .Identifier()
                            .expect("#numericParameterName is an Identifier"),
                    ),
                    Vec::new(),
                    false,
                )))
            }
            NumericParameterContextAll::NumericExpressionContext(expression) => {
                self.expr(&expression.expr().expect("#numericExpression is an expr"))
            }
            NumericParameterContextAll::Error(_) => {
                unreachable!("parse_tree rejects trees with errors")
            }
        }
    }

    /// Convert the parameters among `node`'s direct children, in order.
    ///
    /// For `decimal<P, 2>` these are the two `numericParameter`s. For
    /// `map<string, i64>` or the `funcParams` `(i32, i64)` they are `expr`s.
    /// Tokens such as `<`, `,` and `?` are skipped.
    fn child_parameters(
        &self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
    ) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        node.get_children()
            .filter_map(|child| {
                if let Some(expr) = child.downcast_ref::<ExprContextAll<'a>>() {
                    Some(self.expr(expr))
                } else {
                    child
                        .downcast_ref::<NumericParameterContextAll<'a>>()
                        .map(|parameter| self.numeric_parameter(parameter))
                }
            })
            .collect()
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

    /// The source text of a single token.
    fn token_text(&self, token: &impl Token) -> &'a str {
        &self.source[token.get_start() as usize..=token.get_stop() as usize]
    }

    fn terminal_text(&self, node: &TerminalNode<'a, SubstraitTypeParserContextType>) -> &'a str {
        self.token_text(&*node.symbol)
    }
}

/// Whether `node` has a `?` among its direct children, marking it nullable.
/// A `?` inside a nested parameter belongs to that parameter instead.
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
            ("u!box<0>", 0),
            ("u!box<-2>", -2),
            ("u!box<9223372036854775807>", i64::MAX),
            ("u!box<-9223372036854775808>", i64::MIN),
        ] {
            assert_eq!(
                parse(source),
                Ok(TypeExpr::UserDefined(
                    "box",
                    vec![TypeExprParam::Integer(expected)],
                    false
                )),
                "{source}"
            );
        }
        assert_eq!(
            parse("u!box<9223372036854775808>"),
            Err(TypeParseError::InvalidInteger(
                "9223372036854775808".to_string()
            ))
        );
    }

    #[test]
    fn parenthesized_types_parse_as_their_contents() {
        for (source, expected) in [
            ("(i32)", "i32"),
            ("list<(i32?)>", "list<i32?>"),
            ("decimal<(10), 2>", "decimal<10, 2>"),
        ] {
            assert_eq!(parse(source), parse(expected), "{source}");
        }
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
            Err(TypeParseError::Syntax { column: 8, .. })
        ));
    }
}
