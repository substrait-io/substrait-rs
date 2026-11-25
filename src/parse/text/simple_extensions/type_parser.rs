// SPDX-License-Identifier: Apache-2.0

//! Parse type strings with the ANTLR parser generated from the Substrait type
//! grammar ([`substrait_antlr::substrait_type`]), then convert the parse tree
//! into a [`TypeExpr`] or [`Expr`]. [`TreeConverter`] has one method per
//! grammar rule, each documented with the rule it converts.
//!
//! [`parse`] reads a type, and rejects type derivation expressions such as
//! `P + 1` as [`TypeParseError::UnsupportedNumericExpression`].
//! [`parse_derivation`] reads a whole derivation, including multiline programs,
//! for evaluation by [`derivations`](super::derivations). Built-in names,
//! parameter counts and literal ranges are validated later, when converting to
//! [`Type`](super::types::Type) or evaluating.

use std::cell::RefCell;
use std::rc::Rc;

use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::parser_rule_context::ParserRuleContext;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, Token};
use antlr4rust::token_factory::TokenFactory;
use antlr4rust::tree::TerminalNode;
use antlr4rust::{InputStream, TidExt, TokenSource};
use substrait_antlr::substrait_type::substraittypeparser::*;
use substrait_antlr::substrait_type::{SubstraitTypeLexer, SubstraitTypeParser};

use super::type_ast::{BinaryOp, Expr, TypeExpr, TypeParseError};

/// Parse one complete type, rejecting any tokens left after it.
/// For example, `list<i32>` succeeds, but `list<i32> i64` does not.
pub(super) fn parse(source: &str) -> Result<TypeExpr<'_>, TypeParseError> {
    let type_expr = match convert(source)? {
        Expr::Type(type_expr) => type_expr,
        Expr::Integer(value) => return Err(TypeParseError::ExpectedType(value.to_string())),
        _ => return Err(unsupported_derivation(source)),
    };
    if type_expr.has_derivation() {
        return Err(unsupported_derivation(source));
    }
    Ok(type_expr)
}

/// The most (non-whitespace) tokens a derivation may have. This bounds how
/// deep chains such as `1 + 1 + …` recurse. The longest derivations in the
/// Substrait extensions have about 110 tokens.
pub(super) const MAX_TOKENS: usize = 1024;

/// The most opening `(` and `<` tokens a derivation may have, which bounds its
/// nesting depth. The generated parser recurses several frames per nesting
/// level. The longest derivations in the Substrait extensions (decimal `add`,
/// `subtract` and `modulus`) have 6 openings and nest 2 levels deep.
///
/// Unoptimized (debug) builds use much larger stack frames in the generated
/// parser and exhaust a 2 MiB thread stack at 10 levels of nesting, so the
/// limit is kept low enough to hold in debug builds too, not just optimized
/// ones.
pub(super) const MAX_OPENINGS: usize = 8;

/// Parse a type derivation, such as `decimal<P1 + 1, S1>`, a condition like
/// `P > 10 ? i64 : i32`, or a multiline program ending in a type.
///
/// Surrounding whitespace is ignored, since YAML block scalars (`return: |`)
/// end with a newline. Derivations larger than [`MAX_TOKENS`] or
/// [`MAX_OPENINGS`] are rejected as [`TypeParseError::TooComplex`] before
/// parsing, so the recursive parser cannot exhaust the stack.
pub(super) fn parse_derivation(source: &str) -> Result<Expr<'_>, TypeParseError> {
    let source = source.trim();
    check_complexity(source)?;
    convert(source)
}

/// Parse `source` as an `expr` and convert it, without restricting which
/// forms it may take.
fn convert(source: &str) -> Result<Expr<'_>, TypeParseError> {
    // The type grammar has no variation syntax, so report it before ANTLR
    // turns it into a generic syntax error.
    if source.contains(['[', ']']) {
        return Err(TypeParseError::UnsupportedVariation(source.to_string()));
    }

    let tree = parse_tree(source)?;
    let expr = tree.expr().expect("startRule is `expr EOF`");
    TreeConverter { source }.expr(&expr)
}

fn unsupported_derivation(source: &str) -> TypeParseError {
    TypeParseError::UnsupportedNumericExpression(source.to_string())
}

/// Reject derivations whose size or nesting could exhaust the stack in the
/// recursive parser. Lexing is iterative, so this is safe to run on any
/// input. Lexer errors are ignored here; parsing reports them.
fn check_complexity(source: &str) -> Result<(), TypeParseError> {
    let mut lexer = SubstraitTypeLexer::new(InputStream::new(source));
    lexer.remove_error_listeners();
    let (mut tokens, mut openings) = (0, 0);
    loop {
        let token = lexer.next_token();
        let token_type = token.get_token_type();
        if token_type == TOKEN_EOF {
            return Ok(());
        }
        if token.get_channel() != TOKEN_DEFAULT_CHANNEL {
            continue;
        }
        tokens += 1;
        // Counting openings (closers need no tracking) bounds the nesting
        // depth. `<` is also the less-than operator; counting it anyway only
        // makes the bound more conservative.
        if token_type == SubstraitType_OParen || token_type == SubstraitType_Lt {
            openings += 1;
        }
        if tokens > MAX_TOKENS {
            return Err(TypeParseError::TooComplex(format!(
                "more than {MAX_TOKENS} tokens"
            )));
        }
        if openings > MAX_OPENINGS {
            return Err(TypeParseError::TooComplex(format!(
                "more than {MAX_OPENINGS} opening brackets"
            )));
        }
    }
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
            Expr::Type(type_expr) => Ok(type_expr),
            Expr::Integer(value) => Err(TypeParseError::ExpectedType(value.to_string())),
            _ => Err(unsupported_derivation(self.source)),
        }
    }

    /// Convert an `expr`: a type, parameter name or integer, or a derivation.
    ///
    /// ```text
    /// expr: typeDef | Identifier '?'? | Number | '(' expr ')'
    ///     | expr op expr | '!' expr | expr '?' expr ':' expr
    ///     | IF expr THEN expr ELSE expr | Identifier '(' (expr (',' expr)*)? ')'
    ///     | (Identifier '=' expr Newline+)+ typeDef Newline*
    /// ```
    fn expr(&self, expr: &ExprContextAll<'a>) -> Result<Expr<'a>, TypeParseError> {
        match expr {
            ExprContextAll::TypeLiteralContext(literal) => Ok(Expr::Type(
                self.type_def(&literal.typeDef().expect("#TypeLiteral is a typeDef"))?,
            )),
            ExprContextAll::ParameterNameContext(name) => Ok(Expr::Type(TypeExpr::Simple(
                self.terminal_text(&name.Identifier().expect("#ParameterName is an Identifier")),
                Vec::new(),
                name.QMark().is_some(),
            ))),
            ExprContextAll::LiteralNumberContext(number) => {
                self.integer(&number.Number().expect("#LiteralNumber is a Number"))
            }
            ExprContextAll::ParenExpressionContext(parenthesized) => self.expr(
                &parenthesized
                    .expr()
                    .expect("#ParenExpression wraps an expr"),
            ),
            ExprContextAll::MulDivContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::AddSubContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::ComparisonContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::EqualityContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::AndContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::OrContext(ctx) => self.binary(&ctx.left, &ctx.op, &ctx.right),
            ExprContextAll::NotExprContext(ctx) => Ok(Expr::Not(Box::new(
                self.expr(&ctx.expr().expect("#NotExpr negates an expr"))?,
            ))),
            ExprContextAll::TernaryContext(ctx) => {
                self.conditional(&ctx.ifExpr, &ctx.thenExpr, &ctx.elseExpr)
            }
            ExprContextAll::IfExprContext(ctx) => {
                self.conditional(&ctx.ifExpr, &ctx.thenExpr, &ctx.elseExpr)
            }
            ExprContextAll::FunctionCallContext(ctx) => {
                let name = self.terminal_text(&ctx.Identifier().expect("#FunctionCall has a name"));
                let arguments = ctx
                    .expr_all()
                    .iter()
                    .map(|argument| self.expr(argument))
                    .collect::<Result<_, _>>()?;
                Ok(Expr::Call(name, arguments))
            }
            ExprContextAll::MultilineDefinitionContext(ctx) => {
                // `Identifier_all()` returns every terminal child (antlr4rust
                // does not tell token types apart in its Rust types), so keep
                // only the identifiers: the assignments' left-hand sides.
                let names = ctx
                    .Identifier_all()
                    .into_iter()
                    .filter(|terminal| terminal.symbol.get_token_type() == SubstraitType_Identifier)
                    .map(|terminal| self.terminal_text(&terminal));
                let assignments = names
                    .zip(ctx.expr_all().iter())
                    .map(|(name, value)| Ok((name, self.expr(value)?)))
                    .collect::<Result<_, TypeParseError>>()?;
                let final_type = self.type_def(
                    ctx.finalType
                        .as_ref()
                        .expect("#MultilineDefinition ends in a typeDef"),
                )?;
                Ok(Expr::Program(assignments, Box::new(final_type)))
            }
            ExprContextAll::Error(_) => unreachable!("parse_tree rejects trees with errors"),
        }
    }

    /// Convert a binary operation such as `P + 1`, `S > 0` or `a AND b`.
    fn binary(
        &self,
        left: &Option<Rc<ExprContextAll<'a>>>,
        op: &Option<<LocalTokenFactory<'a> as TokenFactory<'a>>::Tok>,
        right: &Option<Rc<ExprContextAll<'a>>>,
    ) -> Result<Expr<'a>, TypeParseError> {
        let op_text = self.token_text(&**op.as_ref().expect("binary expressions have an operator"));
        let op = match op_text.to_ascii_uppercase().as_str() {
            "+" => BinaryOp::Add,
            "-" => BinaryOp::Subtract,
            "*" => BinaryOp::Multiply,
            "/" => BinaryOp::Divide,
            "<" => BinaryOp::Less,
            "<=" => BinaryOp::LessOrEqual,
            ">" => BinaryOp::Greater,
            ">=" => BinaryOp::GreaterOrEqual,
            "=" => BinaryOp::Equal,
            "!=" => BinaryOp::NotEqual,
            "AND" => BinaryOp::And,
            "OR" => BinaryOp::Or,
            // The lexer has a `%` token, but no grammar rule uses it.
            other => unreachable!("the grammar has no binary operator {other:?}"),
        };
        let left = self.expr(
            left.as_ref()
                .expect("binary expressions have a left operand"),
        )?;
        let right = self.expr(
            right
                .as_ref()
                .expect("binary expressions have a right operand"),
        )?;
        Ok(Expr::Binary(op, Box::new(left), Box::new(right)))
    }

    /// Convert `c ? a : b` or `IF c THEN a ELSE b`.
    fn conditional(
        &self,
        condition: &Option<Rc<ExprContextAll<'a>>>,
        then: &Option<Rc<ExprContextAll<'a>>>,
        otherwise: &Option<Rc<ExprContextAll<'a>>>,
    ) -> Result<Expr<'a>, TypeParseError> {
        let part = |expr: &Option<Rc<ExprContextAll<'a>>>| {
            self.expr(expr.as_ref().expect("conditionals have three parts"))
                .map(Box::new)
        };
        Ok(Expr::If(part(condition)?, part(then)?, part(otherwise)?))
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
    ) -> Result<Expr<'a>, TypeParseError> {
        match parameter {
            NumericParameterContextAll::NumericLiteralContext(literal) => {
                self.integer(&literal.Number().expect("#numericLiteral is a Number"))
            }
            NumericParameterContextAll::NumericParameterNameContext(name) => {
                Ok(Expr::Type(TypeExpr::Simple(
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
    ) -> Result<Vec<Expr<'a>>, TypeParseError> {
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
    ) -> Result<Expr<'a>, TypeParseError> {
        let text = self.terminal_text(number);
        text.parse()
            .map(Expr::Integer)
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
                    vec![Expr::Integer(expected)],
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
                vec![Expr::Type(parse("DeCiMaL<P, S>").unwrap())],
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

    fn name(name: &str) -> Expr<'_> {
        Expr::Type(TypeExpr::Simple(name, vec![], false))
    }

    fn binary<'a>(op: BinaryOp, left: Expr<'a>, right: Expr<'a>) -> Expr<'a> {
        Expr::Binary(op, Box::new(left), Box::new(right))
    }

    #[test]
    fn derivations_parse_into_expressions() {
        use BinaryOp::*;
        assert_eq!(
            parse_derivation("decimal<P1 + 1, S1>"),
            Ok(Expr::Type(TypeExpr::Simple(
                "decimal",
                vec![binary(Add, name("P1"), Expr::Integer(1)), name("S1")],
                false
            )))
        );
        // Precedence and associativity come from the grammar.
        assert_eq!(
            parse_derivation("1 + 2 * 3 - 4"),
            Ok(binary(
                Subtract,
                binary(
                    Add,
                    Expr::Integer(1),
                    binary(Multiply, Expr::Integer(2), Expr::Integer(3))
                ),
                Expr::Integer(4)
            ))
        );
        // Both conditional forms parse alike.
        let conditional = Expr::If(
            Box::new(binary(Greater, name("P"), Expr::Integer(10))),
            Box::new(name("i64")),
            Box::new(name("i32")),
        );
        assert_eq!(
            parse_derivation("P > 10 ? i64 : i32"),
            Ok(conditional.clone())
        );
        assert_eq!(
            parse_derivation("IF P > 10 THEN i64 ELSE i32"),
            Ok(conditional)
        );
        // `!` applies to everything after it, as written in the grammar
        // (`'!' expr`), so it binds more loosely than `OR`.
        assert_eq!(
            parse_derivation("!(a AND b) OR c"),
            Ok(Expr::Not(Box::new(binary(
                Or,
                binary(And, name("a"), name("b")),
                name("c")
            ))))
        );
        assert_eq!(
            parse_derivation("max(S1, 2)"),
            Ok(Expr::Call("max", vec![name("S1"), Expr::Integer(2)]))
        );
    }

    #[test]
    fn multiline_derivations_parse_into_programs() {
        // A YAML block scalar ends with a newline.
        assert_eq!(
            parse_derivation("p = P + 1\ns = min(S, p)\ndecimal<p, s>\n"),
            Ok(Expr::Program(
                vec![
                    ("p", binary(BinaryOp::Add, name("P"), Expr::Integer(1))),
                    ("s", Expr::Call("min", vec![name("S"), name("p")])),
                ],
                Box::new(TypeExpr::Simple(
                    "decimal",
                    vec![name("p"), name("s")],
                    false
                ))
            ))
        );
        // Types reject programs, as they reject other derivations.
        assert_eq!(
            parse("p = 1\ndecimal<p, 0>"),
            Err(TypeParseError::UnsupportedNumericExpression(
                "p = 1\ndecimal<p, 0>".to_string()
            ))
        );
    }

    #[test]
    fn modulo_is_not_an_operator() {
        // The lexer knows `%`, but no grammar rule uses it.
        assert!(matches!(
            parse_derivation("P % 2"),
            Err(TypeParseError::Syntax { column: 2, .. })
        ));
    }

    #[test]
    fn derivation_references_are_visited() {
        let visit = |source| {
            let mut names = Vec::new();
            parse_derivation(source)
                .unwrap()
                .visit_references(&mut |name| names.push(name.to_string()));
            names
        };
        assert_eq!(visit("P > 1 ? map<u!a, i32> : list<u!b>"), ["a", "b"]);
        assert_eq!(
            visit("t = max(P, 1) > 2 ? u!a : u!b\nlist<u!c>"),
            ["a", "b", "c"]
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
