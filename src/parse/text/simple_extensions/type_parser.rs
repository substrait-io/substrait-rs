// SPDX-License-Identifier: Apache-2.0

//! Parse type strings with the ANTLR parser generated from the Substrait type
//! grammar ([`substrait_antlr::substrait_type`]), then convert the parse tree
//! into a [`TypeExpr`]. [`TreeConverter`] has one method per grammar rule,
//! each documented with the rule it converts.
//!
//! Addition and subtraction in numeric parameters are retained without evaluation. Other
//! derivations return [`TypeParseError::UnsupportedNumericExpression`]. Built-in names, parameter
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

use super::return_type::{IntegerAssignment, ReturnTypeExpr};
use super::type_ast::{TypeExpr, TypeExprParam, TypeParseError};
use super::types::{BinaryOp, CustomType, IntegerParameter, NumericFunction};

/// Parse one complete type, rejecting any tokens left after it.
/// For example, `list<i32>` succeeds, but `list<i32> i64` does not.
pub(super) fn parse(source: &str) -> Result<TypeExpr<'_>, TypeParseError> {
    let tree = parse_tree(source)?;
    let expr = tree.expr().expect("startRule is `expr EOF`");
    TreeConverter { source }.expr_as_type(&expr)
}

#[cfg(test)]
pub(super) fn parse_integer_parameter(source: &str) -> Result<IntegerParameter, TypeParseError> {
    let tree = parse_tree(source)?;
    TreeConverter { source }.integer_expr(&tree.expr().expect("startRule has an expression"))
}

/// Parse ordered assignments followed by a final type, or a plain type.
pub(super) fn parse_return_type(source: &str) -> Result<ReturnTypeExpr<'_>, TypeParseError> {
    let source = source.trim_end();
    let tree = parse_tree(source)?;
    let expr = tree.expr().expect("startRule has an expression");
    let converter = TreeConverter { source };
    match expr.as_ref() {
        ExprContextAll::MultilineDefinitionContext(program) => {
            let values = program.expr_all();
            let assignments = values
                .iter()
                .enumerate()
                .map(|(index, value)| {
                    let name = converter
                        .terminal_text(&program.Identifier(index).expect("assignment has a name"));
                    converter.integer_name(name)?;
                    Ok(IntegerAssignment {
                        name: name.to_string(),
                        value: converter.integer_expr(value)?,
                    })
                })
                .collect::<Result<_, TypeParseError>>()?;
            let return_type = converter.type_def(
                program
                    .finalType
                    .as_ref()
                    .expect("derivation has a final type"),
            )?;
            Ok(ReturnTypeExpr {
                assignments,
                return_type,
            })
        }
        _ => Ok(ReturnTypeExpr {
            assignments: Vec::new(),
            return_type: converter.expr_as_type(&expr)?,
        }),
    }
}

/// Run the generated parser on `source`, returning its `startRule` tree.
///
/// The tree is only returned when neither the lexer nor the parser reported
/// an error, so [`TreeConverter`] can rely on it matching the grammar.
fn parse_tree(source: &str) -> Result<Rc<StartRuleContextAll<'_>>, TypeParseError> {
    // The type grammar has no variation syntax.
    if source.contains(['[', ']']) {
        return Err(TypeParseError::UnsupportedVariation(source.to_string()));
    }

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
            ExprContextAll::AddSubContext(_) | ExprContextAll::FunctionCallContext(_) => {
                self.integer_expr(expr).map(TypeExprParam::Integer)
            }
            _ => Err(TypeParseError::UnsupportedNumericExpression(
                self.source.to_string(),
            )),
        }
    }

    /// Convert a numeric expression without binding names or evaluating operators.
    fn integer_expr(&self, expr: &ExprContextAll<'a>) -> Result<IntegerParameter, TypeParseError> {
        match expr {
            ExprContextAll::LiteralNumberContext(number) => {
                self.integer_value(&number.Number().expect("number literal has a number"))
            }
            ExprContextAll::ParameterNameContext(name) if name.QMark().is_none() => self
                .integer_name(
                    self.terminal_text(&name.Identifier().expect("parameter has a name")),
                ),
            ExprContextAll::ParenExpressionContext(grouped) => {
                self.integer_expr(&grouped.expr().expect("parentheses contain an expression"))
            }
            ExprContextAll::AddSubContext(binary) => {
                let operator = binary
                    .op
                    .as_ref()
                    .expect("binary expression has an operator");
                let op = match operator.get_token_type() {
                    SubstraitType_Plus => BinaryOp::Add,
                    SubstraitType_Minus => BinaryOp::Subtract,
                    _ => {
                        return Err(TypeParseError::UnsupportedNumericExpression(
                            self.source.to_string(),
                        ));
                    }
                };
                Ok(IntegerParameter::BinaryExpression {
                    op,
                    left: Box::new(
                        self.integer_expr(
                            binary
                                .left
                                .as_ref()
                                .expect("binary expression has a left operand"),
                        )?,
                    ),
                    right: Box::new(
                        self.integer_expr(
                            binary
                                .right
                                .as_ref()
                                .expect("binary expression has a right operand"),
                        )?,
                    ),
                })
            }
            ExprContextAll::FunctionCallContext(call) => {
                let name = self.terminal_text(&call.Identifier().expect("call has a name"));
                let function = match name.to_ascii_lowercase().as_str() {
                    "min" => NumericFunction::Min,
                    "max" => NumericFunction::Max,
                    "integer_parameter" => NumericFunction::IntegerParameter,
                    _ => {
                        return Err(TypeParseError::UnsupportedNumericExpression(
                            self.source.to_string(),
                        ));
                    }
                };
                let arguments = call
                    .expr_all()
                    .iter()
                    .map(|expr| self.integer_expr(expr))
                    .collect::<Result<_, _>>()?;
                Ok(IntegerParameter::Call {
                    function,
                    arguments,
                })
            }
            ExprContextAll::TypeLiteralContext(_) | ExprContextAll::ParameterNameContext(_) => {
                Err(TypeParseError::ExpectedInteger(self.source.to_string()))
            }
            _ => Err(TypeParseError::UnsupportedNumericExpression(
                self.source.to_string(),
            )),
        }
    }

    fn integer_name(&self, name: &str) -> Result<IntegerParameter, TypeParseError> {
        CustomType::validate_name(name)
            .map_err(|_| TypeParseError::ExpectedInteger(self.source.to_string()))?;
        Ok(IntegerParameter::Variable(name.to_string()))
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
            NumericParameterContextAll::NumericParameterNameContext(name) => self
                .integer_name(
                    self.terminal_text(
                        &name
                            .Identifier()
                            .expect("#numericParameterName is an Identifier"),
                    ),
                )
                .map(TypeExprParam::Integer),
            NumericParameterContextAll::NumericExpressionContext(expression) => self
                .integer_expr(&expression.expr().expect("#numericExpression is an expr"))
                .map(TypeExprParam::Integer),
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
        self.integer_value(number).map(TypeExprParam::Integer)
    }

    fn integer_value(
        &self,
        number: &TerminalNode<'a, SubstraitTypeParserContextType>,
    ) -> Result<IntegerParameter, TypeParseError> {
        let text = self.terminal_text(number);
        text.parse()
            .map(IntegerParameter::Literal)
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
    fn parse_multiline_returns_preserve_assignment_order() {
        let parsed = parse_return_type("P = 10\nS = 2\ndecimal<P, S>").unwrap();
        assert_eq!(
            parsed.assignments,
            vec![
                IntegerAssignment {
                    name: "P".into(),
                    value: IntegerParameter::Literal(10)
                },
                IntegerAssignment {
                    name: "S".into(),
                    value: IntegerParameter::Literal(2)
                },
            ]
        );
        assert_eq!(
            parsed.return_type,
            TypeExpr::Simple(
                "decimal",
                vec![
                    TypeExprParam::Integer(IntegerParameter::Variable("P".into())),
                    TypeExprParam::Integer(IntegerParameter::Variable("S".into())),
                ],
                false,
            )
        );
    }

    #[test]
    fn malformed_multiline_returns_are_rejected() {
        for source in [
            "P = 1 i32",
            "P = 1\n",
            "P =\ni32",
            "P = 1\nQ =\ni32",
            "P = 1\ni32 garbage",
            "P = 1;\ni32",
            "P = 1\nQ\ni32",
            "P = i32\ndecimal<P, 0>",
        ] {
            assert!(parse_return_type(source).is_err(), "{source}");
        }
    }

    #[test]
    fn parse_return_types_allow_no_assignments() {
        let parsed = parse_return_type("i32\n").unwrap();
        assert!(parsed.assignments.is_empty());
        assert_eq!(parsed.return_type, TypeExpr::Simple("i32", vec![], false));
    }

    #[test]
    fn calls_retain_function_and_ordered_arguments() {
        assert_eq!(
            parse_integer_parameter("MAX(P, 2)").unwrap(),
            IntegerParameter::Call {
                function: NumericFunction::Max,
                arguments: vec![
                    IntegerParameter::Variable("P".into()),
                    IntegerParameter::Literal(2)
                ],
            }
        );
        for source in ["min(max(P - S, 2) + 1, 38)", "integer_parameter(precision)"] {
            let expression = parse_integer_parameter(source).unwrap();
            assert_eq!(
                parse_integer_parameter(&expression.to_string()).unwrap(),
                expression
            );
        }
        for source in ["max(i32, 2)", "max(any1, 2)", "unknown(P)"] {
            assert!(parse_integer_parameter(source).is_err(), "{source}");
        }
    }

    #[test]
    fn subtraction_preserves_order_and_grouping() {
        for (source, expected) in [
            ("P - S + 1", "P - S + 1"),
            ("P - (S - 1)", "P - (S - 1)"),
            ("P - -1", "P - -1"),
        ] {
            let value = parse_integer_parameter(source).unwrap();
            assert_eq!(value.to_string(), expected);
            assert_eq!(parse_integer_parameter(&value.to_string()).unwrap(), value);
        }
        assert_eq!(
            parse_integer_parameter("P - S").unwrap(),
            IntegerParameter::BinaryExpression {
                op: BinaryOp::Subtract,
                left: Box::new(IntegerParameter::Variable("P".into())),
                right: Box::new(IntegerParameter::Variable("S".into())),
            }
        );
    }

    #[test]
    fn numeric_parameters_have_explicit_integer_nodes() {
        assert_eq!(
            parse("decimal<P, 2>").unwrap(),
            TypeExpr::Simple(
                "decimal",
                vec![
                    TypeExprParam::Integer(IntegerParameter::Variable("P".into())),
                    TypeExprParam::Integer(IntegerParameter::Literal(2)),
                ],
                false
            )
        );
        for source in [
            "decimal<any1, S>",
            "varchar<i32>",
            "varchar<P?>",
            "varchar<i32 + 1>",
        ] {
            assert!(parse(source).is_err(), "{source}");
        }
    }

    #[test]
    fn addition_keeps_operand_order_and_left_associativity() {
        let expression = parse_integer_parameter("P + S + 1").unwrap();
        assert_eq!(
            expression,
            IntegerParameter::BinaryExpression {
                op: BinaryOp::Add,
                left: Box::new(IntegerParameter::BinaryExpression {
                    op: BinaryOp::Add,
                    left: Box::new(IntegerParameter::Variable("P".into())),
                    right: Box::new(IntegerParameter::Variable("S".into())),
                }),
                right: Box::new(IntegerParameter::Literal(1)),
            }
        );
    }

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
                    vec![TypeExprParam::Integer(IntegerParameter::Literal(expected))],
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
            "decimal<P1 * 2, S>",
            "u!box<unsupported(P, 2)>",
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
