// SPDX-License-Identifier: Apache-2.0

//! Parse type syntax with the ANTLR parser generated from the Substrait type
//! grammar ([`substrait_antlr::substrait_type`]), then convert its parse tree
//! into a [`TypeExpr`] with a generated visitor.
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

use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::Token;
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::tree::{ParseTreeVisitorCompat, TerminalNode, Tree};
use substrait_antlr::substrait_type::substraittypeparser::*;
use substrait_antlr::substrait_type::substraittypevisitor::SubstraitTypeVisitorCompat;
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

    let mut builder = TypeExprBuilder {
        source,
        temp_result: None,
    };
    builder.type_value(&*tree.expr().expect("startRule always has an expr"))
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

/// What visiting one parse tree node produces.
///
/// `None` is the visitor's default and only comes back for tokens; every rule
/// node produces `Some`.
type Visited<'a> = Option<Result<TypeExprParam<'a>, TypeParseError>>;

/// Builds a [`TypeExpr`] borrowing from `source` by visiting the parse tree.
struct TypeExprBuilder<'a> {
    source: &'a str,
    /// Scratch space the `antlr4rust` visitor adapter requires.
    temp_result: Visited<'a>,
}

impl<'a> TypeExprBuilder<'a> {
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

    fn integer(&self, number: &TerminalNode<'a, SubstraitTypeParserContextType>) -> Visited<'a> {
        let text = self.terminal_text(number);
        Some(
            text.parse()
                .map(TypeExprParam::Integer)
                .map_err(|_| TypeParseError::InvalidInteger(text.to_string())),
        )
    }

    /// Visit a rule node, which always produces a value.
    fn value(
        &mut self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
    ) -> Result<TypeExprParam<'a>, TypeParseError> {
        self.visit(node).expect("rule nodes always produce a value")
    }

    /// Visit a rule node that must be a type, rejecting integer literals.
    fn type_value(
        &mut self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
    ) -> Result<TypeExpr<'a>, TypeParseError> {
        match self.value(node)? {
            TypeExprParam::Type(type_expr) => Ok(type_expr),
            TypeExprParam::Integer(value) => Err(TypeParseError::ExpectedType(value.to_string())),
        }
    }

    /// Visit every child of `node`, keeping the values of its rule children.
    /// Tokens such as `<`, `,` and `?` produce nothing and are skipped.
    fn child_values(
        &mut self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
    ) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        node.get_children()
            .filter_map(|child| self.visit(&*child))
            .collect()
    }

    /// A builtin introduced by a keyword, such as `list?<i32>` or `decimal<P, S>`,
    /// whose parameters are its rule children in order.
    fn builtin(
        &mut self,
        node: &(dyn SubstraitTypeParserContext<'a> + 'a),
        nullable: bool,
    ) -> Visited<'a> {
        let name = self.token_text(&*node.start());
        Some(
            self.child_values(node).map(|parameters| {
                TypeExprParam::Type(TypeExpr::Simple(name, parameters, nullable))
            }),
        )
    }

    fn func(&mut self, ctx: &FuncContext<'a>) -> Result<TypeExprParam<'a>, TypeParseError> {
        let parameters = self.child_values(&**ctx.params.as_ref().expect("func has parameters"))?;
        let return_type =
            self.type_value(&**ctx.returnType.as_ref().expect("func has a return type"))?;
        Ok(TypeExprParam::Type(TypeExpr::Func(
            parameters,
            Box::new(return_type),
            ctx.QMark().is_some(),
        )))
    }

    fn user_defined(
        &mut self,
        ctx: &UserDefinedContext<'a>,
    ) -> Result<TypeExprParam<'a>, TypeParseError> {
        let name = self.identifier(&ctx.Identifier(0).expect("u! has a name"))?;
        let parameters = self.child_values(ctx)?;
        Ok(TypeExprParam::Type(TypeExpr::UserDefined(
            name,
            parameters,
            ctx.QMark().is_some(),
        )))
    }

    fn unsupported(&self, construct: &'static str) -> Visited<'a> {
        Some(Err(TypeParseError::UnsupportedSyntax {
            construct,
            type_string: self.source.to_string(),
        }))
    }
}

impl<'a> ParseTreeVisitorCompat<'a> for TypeExprBuilder<'a> {
    type Node = SubstraitTypeParserContextType;
    type Return = Visited<'a>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp_result
    }

    /// Every rule without a `visit_*` override below is part of a derivation
    /// expression (`P + 1`, `max(P, 2)`, ternaries, ...), which `TypeExpr`
    /// cannot represent yet.
    fn visit_children(&mut self, _node: &(dyn SubstraitTypeParserContext<'a> + 'a)) -> Visited<'a> {
        Some(Err(TypeParseError::UnsupportedNumericExpression(
            self.source.to_string(),
        )))
    }
}

impl<'a> SubstraitTypeVisitorCompat<'a> for TypeExprBuilder<'a> {
    fn visit_TypeLiteral(&mut self, ctx: &TypeLiteralContext<'a>) -> Visited<'a> {
        self.visit(&*ctx.typeDef().expect("type literal has a typeDef"))
    }

    fn visit_ParenExpression(&mut self, ctx: &ParenExpressionContext<'a>) -> Visited<'a> {
        self.visit(&*ctx.expr().expect("parenthesized expr has an expr"))
    }

    fn visit_LiteralNumber(&mut self, ctx: &LiteralNumberContext<'a>) -> Visited<'a> {
        self.integer(&ctx.Number().expect("number literal has a number"))
    }

    fn visit_ParameterName(&mut self, ctx: &ParameterNameContext<'a>) -> Visited<'a> {
        let name = self.identifier(&ctx.Identifier().expect("parameter name has an identifier"));
        Some(name.map(|name| {
            TypeExprParam::Type(TypeExpr::Simple(name, Vec::new(), ctx.QMark().is_some()))
        }))
    }

    fn visit_typeDef(&mut self, ctx: &TypeDefContext<'a>) -> Visited<'a> {
        match ctx.scalarType() {
            Some(scalar) => {
                let name = self.token_text(&*scalar.start());
                Some(Ok(TypeExprParam::Type(TypeExpr::Simple(
                    name,
                    Vec::new(),
                    ctx.QMark().is_some(),
                ))))
            }
            None => self.visit(&*ctx.get_child(0).expect("typeDef has a child")),
        }
    }

    fn visit_anyType(&mut self, ctx: &AnyTypeContext<'a>) -> Visited<'a> {
        let nullable = ctx.QMark().is_some();
        let type_expr = match ctx.AnyVar() {
            Some(variable) => {
                let digit = &self.terminal_text(&variable)[3..];
                TypeExpr::TypeVariable(digit.parse().expect("AnyVar ends in a digit"), nullable)
            }
            None => {
                let name = self.terminal_text(&ctx.Any().expect("anyType is Any or AnyVar"));
                TypeExpr::Simple(name, Vec::new(), nullable)
            }
        };
        Some(Ok(TypeExprParam::Type(type_expr)))
    }

    fn visit_numericLiteral(&mut self, ctx: &NumericLiteralContext<'a>) -> Visited<'a> {
        self.integer(&ctx.Number().expect("numeric literal has a number"))
    }

    fn visit_numericParameterName(&mut self, ctx: &NumericParameterNameContext<'a>) -> Visited<'a> {
        let name = self.identifier(&ctx.Identifier().expect("parameter name"));
        Some(name.map(|name| TypeExprParam::Type(TypeExpr::Simple(name, Vec::new(), false))))
    }

    fn visit_numericExpression(&mut self, ctx: &NumericExpressionContext<'a>) -> Visited<'a> {
        self.visit(&*ctx.expr().expect("numeric expression has an expr"))
    }

    fn visit_fixedChar(&mut self, ctx: &FixedCharContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_varChar(&mut self, ctx: &VarCharContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_fixedBinary(&mut self, ctx: &FixedBinaryContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_decimal(&mut self, ctx: &DecimalContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_precisionIntervalDay(&mut self, ctx: &PrecisionIntervalDayContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_precisionIntervalCompound(
        &mut self,
        ctx: &PrecisionIntervalCompoundContext<'a>,
    ) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_precisionTime(&mut self, ctx: &PrecisionTimeContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_precisionTimestamp(&mut self, ctx: &PrecisionTimestampContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_precisionTimestampTZ(&mut self, ctx: &PrecisionTimestampTZContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_struct(&mut self, ctx: &StructContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_list(&mut self, ctx: &ListContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_map(&mut self, ctx: &MapContext<'a>) -> Visited<'a> {
        self.builtin(ctx, ctx.QMark().is_some())
    }

    fn visit_func(&mut self, ctx: &FuncContext<'a>) -> Visited<'a> {
        Some(self.func(ctx))
    }

    fn visit_userDefined(&mut self, ctx: &UserDefinedContext<'a>) -> Visited<'a> {
        if ctx.dependencyAlias.is_some() {
            return self.unsupported("dependency-qualified user-defined");
        }
        Some(self.user_defined(ctx))
    }

    fn visit_nStruct(&mut self, _ctx: &NStructContext<'a>) -> Visited<'a> {
        self.unsupported("named struct")
    }
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
