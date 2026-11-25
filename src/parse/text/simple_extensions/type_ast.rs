// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
//!
//! This module provides syntactic parsing of Substrait type strings into an AST.
//! It does NOT validate that type names are valid builtins or that extension types exist -
//! semantic validation occurs later when converting to [`Type`](super::types::Type).
//!
//! Parsing uses the parser generated from the official Substrait type grammar,
//! published as the [`substrait_antlr`] crate.

/// A parsed type expression from a type string, with lifetime tied to the original string.
///
/// This represents the syntactic structure only - type names are not validated.
/// Convert to [`Type`](super::types::Type) for semantic validation.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr<'a> {
    /// A type without the `u!` prefix (expected to be a builtin like `i32`, `List`, etc.)
    ///
    /// Contains: (name, parameters, nullable)
    Simple(&'a str, Vec<Expr<'a>>, bool),
    /// A user-defined extension type with the `u!` prefix (e.g., `u!geometry`)
    ///
    /// Contains: (name without `u!` prefix, parameters, nullable)
    UserDefined(&'a str, Vec<Expr<'a>>, bool),
    /// Type variable (e.g., `any1`, `any2`)
    ///
    /// Contains: (variable id, nullable)
    TypeVariable(u32, bool),
    /// A function type (e.g., `func<i32 -> i64>`)
    ///
    /// Contains: (parameters, return type, nullable)
    Func(Vec<Expr<'a>>, Box<TypeExpr<'a>>, bool),
}

/// A parsed expression: a type parameter, or (part of) a type derivation such
/// as `decimal<P1 + 1, S1>` or a multiline program.
///
/// [`TypeExpr::parse`] only produces the [`Type`](Expr::Type) and
/// [`Integer`](Expr::Integer) forms; the others come from
/// [`parse_derivation`](super::type_parser::parse_derivation).
#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'a> {
    /// A type, or a parameter name such as `P` or `T?`, which parses as a
    /// [`TypeExpr::Simple`] without parameters.
    Type(TypeExpr<'a>),
    /// An integer literal
    Integer(i64),
    /// A binary operation, e.g. `P1 + 1` or `S > 0 AND P > S`
    Binary(BinaryOp, Box<Expr<'a>>, Box<Expr<'a>>),
    /// Boolean negation, `!c`
    Not(Box<Expr<'a>>),
    /// A conditional: `c ? a : b` or `IF c THEN a ELSE b`
    If(Box<Expr<'a>>, Box<Expr<'a>>, Box<Expr<'a>>),
    /// A function call such as `max(S1, S2)`
    ///
    /// Contains: (function name, arguments)
    Call(&'a str, Vec<Expr<'a>>),
    /// A multiline definition: `name = expr` assignments followed by a final
    /// type, which may use them
    ///
    /// Contains: (assignments in order, final type)
    Program(Vec<(&'a str, Expr<'a>)>, Box<TypeExpr<'a>>),
}

/// A binary operator in a type derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    /// `+`
    Add,
    /// `-`
    Subtract,
    /// `*`
    Multiply,
    /// `/`
    Divide,
    /// `<`
    Less,
    /// `<=`
    LessOrEqual,
    /// `>`
    Greater,
    /// `>=`
    GreaterOrEqual,
    /// `=`
    Equal,
    /// `!=`
    NotEqual,
    /// `AND`
    And,
    /// `OR`
    Or,
}

impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Less => "<",
            BinaryOp::LessOrEqual => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterOrEqual => ">=",
            BinaryOp::Equal => "=",
            BinaryOp::NotEqual => "!=",
            BinaryOp::And => "AND",
            BinaryOp::Or => "OR",
        })
    }
}

impl Expr<'_> {
    /// Visit all extension type references in this expression; see
    /// [`TypeExpr::visit_references`].
    pub fn visit_references<F>(&self, on_ext: &mut F)
    where
        F: FnMut(&str),
    {
        match self {
            Expr::Type(type_expr) => type_expr.visit_references(on_ext),
            Expr::Integer(_) => {}
            Expr::Binary(_, left, right) => {
                left.visit_references(on_ext);
                right.visit_references(on_ext);
            }
            Expr::Not(operand) => operand.visit_references(on_ext),
            Expr::If(condition, then, otherwise) => {
                condition.visit_references(on_ext);
                then.visit_references(on_ext);
                otherwise.visit_references(on_ext);
            }
            Expr::Call(_, arguments) => {
                for argument in arguments {
                    argument.visit_references(on_ext);
                }
            }
            Expr::Program(assignments, final_type) => {
                for (_, value) in assignments {
                    value.visit_references(on_ext);
                }
                final_type.visit_references(on_ext);
            }
        }
    }

    /// Whether this is a derivation (an operator, condition, function call or
    /// program) rather than a type or integer, at any depth.
    pub(super) fn is_derivation(&self) -> bool {
        match self {
            Expr::Type(type_expr) => type_expr.has_derivation(),
            Expr::Integer(_) => false,
            Expr::Binary(..) | Expr::Not(_) | Expr::If(..) | Expr::Call(..) | Expr::Program(..) => {
                true
            }
        }
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("Type variation syntax is not supported: {0}")]
    UnsupportedVariation(String),
    #[error("numeric expressions are not supported yet: {0}")]
    UnsupportedNumericExpression(String),
    #[error("{construct} types are not supported yet: {type_string}")]
    UnsupportedSyntax {
        construct: &'static str,
        type_string: String,
    },
    #[error("expected a type, found integer literal {0}")]
    ExpectedType(String),
    #[error("integer literal does not fit in an i64: {0}")]
    InvalidInteger(String),
    #[error("syntax error at column {column}: {message}")]
    Syntax { column: usize, message: String },
    #[error("type derivation too complex: {0}")]
    TooComplex(String),
}

impl<'a> TypeExpr<'a> {
    /// Parse a type string into a [`TypeExpr`].
    pub fn parse(type_str: &'a str) -> Result<Self, TypeParseError> {
        super::type_parser::parse(type_str)
    }

    /// Whether any parameter, at any depth, is a derivation.
    pub(super) fn has_derivation(&self) -> bool {
        match self {
            TypeExpr::Simple(_, params, _) | TypeExpr::UserDefined(_, params, _) => {
                params.iter().any(Expr::is_derivation)
            }
            TypeExpr::TypeVariable(..) => false,
            TypeExpr::Func(params, return_type, _) => {
                params.iter().any(Expr::is_derivation) || return_type.has_derivation()
            }
        }
    }

    /// Visit all extension type references contained in a parsed type, calling `on_ext`
    /// for each encountered extension name (including named extension forms).
    pub fn visit_references<F>(&self, on_ext: &mut F)
    where
        F: FnMut(&str),
    {
        match self {
            TypeExpr::UserDefined(name, params, _) => {
                on_ext(name);
                for p in params {
                    p.visit_references(on_ext);
                }
            }
            TypeExpr::Simple(_name, params, _) => {
                for p in params {
                    p.visit_references(on_ext);
                }
            }
            TypeExpr::TypeVariable(..) => {}
            TypeExpr::Func(parameters, return_type, _) => {
                for parameter in parameters {
                    parameter.visit_references(on_ext);
                }
                return_type.visit_references(on_ext);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(expr: &str) -> TypeExpr<'_> {
        TypeExpr::parse(expr).expect("parse succeeds")
    }

    #[test]
    fn test_simple_types() {
        let cases = vec![
            ("i32", TypeExpr::Simple("i32", vec![], false)),
            ("i32?", TypeExpr::Simple("i32", vec![], true)),
            ("date", TypeExpr::Simple("date", vec![], false)),
            (
                "interval_year?",
                TypeExpr::Simple("interval_year", vec![], true),
            ),
            ("uuid", TypeExpr::Simple("uuid", vec![], false)),
            ("any", TypeExpr::Simple("any", vec![], false)),
        ];

        for (expr, expected) in cases {
            assert_eq!(parse(expr), expected, "unexpected parse for {expr}");
        }
    }

    #[test]
    fn test_type_variables() {
        let cases = vec![
            ("any1", TypeExpr::TypeVariable(1, false)),
            ("any2?", TypeExpr::TypeVariable(2, true)),
        ];

        for (expr, expected) in cases {
            assert_eq!(
                parse(expr),
                expected,
                "unexpected variable parse for {expr}"
            );
        }
    }

    #[test]
    fn test_user_defined_and_parameters() {
        let cases = [
            (
                "u!geo?<i32?, u!point<i32, i32>>",
                TypeExpr::UserDefined(
                    "geo",
                    vec![
                        Expr::Type(TypeExpr::Simple("i32", vec![], true)),
                        Expr::Type(TypeExpr::UserDefined(
                            "point",
                            vec![
                                Expr::Type(TypeExpr::Simple("i32", vec![], false)),
                                Expr::Type(TypeExpr::Simple("i32", vec![], false)),
                            ],
                            false,
                        )),
                    ],
                    true,
                ),
            ),
            (
                "Map?<i32, string>",
                TypeExpr::Simple(
                    "Map",
                    vec![
                        Expr::Type(TypeExpr::Simple("i32", vec![], false)),
                        Expr::Type(TypeExpr::Simple("string", vec![], false)),
                    ],
                    true,
                ),
            ),
        ];
        for (expr, expected) in cases {
            assert_eq!(parse(expr), expected, "unexpected parse for {expr}");
        }
    }

    #[test]
    fn test_visit_references_builtin_case_insensitive() {
        let cases = vec![
            ("DECIMAL<10,2>", Vec::<String>::new()),
            ("List<string>", Vec::<String>::new()),
            ("u!custom<i32>", vec!["custom".to_string()]),
            ("u!Geo<string>", vec!["Geo".to_string()]),
        ];

        for (expr, expected_refs) in cases {
            let mut refs = Vec::new();
            parse(expr).visit_references(&mut |name| refs.push(name.to_string()));
            assert_eq!(refs, expected_refs, "unexpected references for {expr}");
        }
    }

    #[test]
    fn test_function_types() {
        let cases = vec![
            (
                "func<i32 -> i64>",
                TypeExpr::Func(
                    vec![Expr::Type(TypeExpr::Simple("i32", vec![], false))],
                    Box::new(TypeExpr::Simple("i64", vec![], false)),
                    false,
                ),
            ),
            (
                "FUNC?<any1 -> any2>",
                TypeExpr::Func(
                    vec![Expr::Type(TypeExpr::TypeVariable(1, false))],
                    Box::new(TypeExpr::TypeVariable(2, false)),
                    true,
                ),
            ),
            (
                "func<(i32, list<any1>) -> i64?>",
                TypeExpr::Func(
                    vec![
                        Expr::Type(TypeExpr::Simple("i32", vec![], false)),
                        Expr::Type(TypeExpr::Simple(
                            "list",
                            vec![Expr::Type(TypeExpr::TypeVariable(1, false))],
                            false,
                        )),
                    ],
                    Box::new(TypeExpr::Simple("i64", vec![], true)),
                    false,
                ),
            ),
            (
                "func<i32 -> func<i64 -> i64>>",
                TypeExpr::Func(
                    vec![Expr::Type(TypeExpr::Simple("i32", vec![], false))],
                    Box::new(TypeExpr::Func(
                        vec![Expr::Type(TypeExpr::Simple("i64", vec![], false))],
                        Box::new(TypeExpr::Simple("i64", vec![], false)),
                        false,
                    )),
                    false,
                ),
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(parse(expr), expected, "unexpected parse for {expr}");
        }
    }

    #[test]
    fn test_nested_function_parameters_and_containers() {
        let inner = parse("func<i32 -> i64>");
        assert_eq!(
            parse("func<func<i32 -> i64> -> boolean>"),
            TypeExpr::Func(
                vec![Expr::Type(inner.clone())],
                Box::new(parse("boolean")),
                false,
            )
        );
        assert_eq!(
            parse("list<func<i32 -> i64>>"),
            TypeExpr::Simple("list", vec![Expr::Type(inner)], false)
        );
    }

    #[test]
    fn test_multiple_function_parameters_require_parentheses() {
        for input in [
            "func<i32, i64 -> boolean>",
            "func<list<i32>, map<string, i64> -> boolean>",
            "func<func<i32 -> i64>, i32 -> boolean>",
            "list<func<i32, i64 -> boolean>>",
            "func<i32 -> func<i32, i64 -> boolean>>",
        ] {
            assert!(TypeExpr::parse(input).is_err(), "{input}");
        }
    }

    #[test]
    fn test_single_function_parameter_can_contain_commas() {
        assert_eq!(
            parse("func<map<string, i64> -> boolean>"),
            TypeExpr::Func(
                vec![Expr::Type(parse("map<string, i64>"))],
                Box::new(parse("boolean")),
                false,
            )
        );
    }

    #[test]
    fn test_function_parameters_cannot_be_empty() {
        for input in [
            "func<(i32,) -> i64>",
            "func<(i32, ) -> i64>",
            "func<(,i32) -> i64>",
            "func<(i32,,i64) -> boolean>",
            "func<(func<i32 -> i64>,) -> boolean>",
            "func<() -> i64>",
            "func<i32, i64>",
        ] {
            assert!(
                matches!(TypeExpr::parse(input), Err(TypeParseError::Syntax { .. })),
                "{input}"
            );
        }
    }

    #[test]
    fn test_visit_references_function_types() {
        let mut refs = Vec::new();
        parse("func<(u!point, any1) -> u!edge?>")
            .visit_references(&mut |name| refs.push(name.to_string()));
        assert_eq!(refs, vec!["point".to_string(), "edge".to_string()]);
    }

    #[test]
    fn test_whitespace_between_tokens() {
        assert_eq!(
            parse(" Map ? < string , List < i32 > > "),
            TypeExpr::Simple(
                "Map",
                vec![Expr::Type(parse("string")), Expr::Type(parse("List<i32>"))],
                true,
            )
        );
    }

    #[test]
    fn test_malformed_types_and_trailing_tokens() {
        for input in [
            "",
            "list<i32",
            "i32 i64",
            "list<i32>>",
            "func<i32 -> i64>garbage",
            "func<(i32 -> i64>",
        ] {
            assert!(TypeExpr::parse(input).is_err(), "{input}");
        }
    }

    #[test]
    fn test_variation_not_supported() {
        let cases = vec!["i32[1]", "Foo?[1]", "u!bar[2]"];

        for expr in cases {
            match TypeExpr::parse(expr) {
                Err(TypeParseError::UnsupportedVariation(s)) => assert_eq!(s, expr),
                other => panic!("expected UnsupportedVariation for {expr}, got {other:?}"),
            }
        }
    }
}
