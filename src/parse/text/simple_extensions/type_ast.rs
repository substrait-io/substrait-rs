// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
//!
//! This module provides syntactic parsing of Substrait type strings into an AST.
//! It does NOT validate that type names are valid builtins or that extension types exist -
//! semantic validation occurs later when converting to [`Type`](super::types::Type).
//!
//! This module is based on the official substrait type grammar defined
//! [here](https://github.com/substrait-io/substrait/blob/5f031b69ed211e1ec307be3db7989d64c65d33a2/grammar/SubstraitType.g4).

// Ideally, this module would be generated from the grammar, but there are no
// well-maintained ANTLR grammar modules for Rust available at the time of this
// writing:
// - Official Antlr4 [language
//   support](https://github.com/antlr/antlr4/blob/594cf95e5460435c7521e80618666741e33e4d91/doc/targets.md)
//   does not support Rust
// - The ([`antlr4rust`](https://github.com/rrevenantt/antlr4rust) ) crate has
//   not been updated in 3 years
//
// Therefore, the grammar is manually implemented.

use super::types::IntegerParameter;

/// A parsed type expression from a type string, with lifetime tied to the original string.
///
/// This represents the syntactic structure only - type names are not validated.
/// Convert to [`Type`](super::types::Type) for semantic validation.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr<'a> {
    /// A type without the `u!` prefix (expected to be a builtin like `i32`, `List`, etc.)
    ///
    /// Contains: (name, parameters, nullable)
    Simple(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// A user-defined extension type with the `u!` prefix (e.g., `u!geometry`)
    ///
    /// Contains: (name without `u!` prefix, parameters, nullable)
    UserDefined(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// Type variable (e.g., `any1`, `any2`)
    ///
    /// Contains: (variable id, nullable)
    TypeVariable(u32, bool),
    /// A function type (e.g., `func<i32 -> i64>`)
    ///
    /// Contains: (parameters, return type, nullable)
    Func(Vec<TypeExprParam<'a>>, Box<TypeExpr<'a>>, bool),
}

/// A parsed parameter to a parameterized type
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExprParam<'a> {
    /// A nested type parameter
    Type(TypeExpr<'a>),
    /// An integer literal parameter
    Integer(i64),
    /// An unevaluated arithmetic expression in a numeric parameter.
    Expression(IntegerParameter),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("missing closing angle bracket in parameter list: {0}")]
    ExpectedClosingAngleBracket(String),
    #[error("Type variation syntax is not supported: {0}")]
    UnsupportedVariation(String),
    #[error("missing return arrow in function type: {0}")]
    MissingArrow(String),
    #[error("function type must declare at least one parameter: {0}")]
    EmptyFunctionParameters(String),
    #[error("multiple function parameters must be enclosed in parentheses: {0}")]
    UnparenthesizedFunctionParameters(String),
    #[error("type parameter must not be empty")]
    EmptyParameter,
    #[error("numeric expressions are not supported yet: {0}")]
    UnsupportedNumericExpression(String),
    #[error("expected {expected} at byte {position}")]
    UnexpectedToken {
        position: usize,
        expected: &'static str,
    },
}

impl<'a> TypeExpr<'a> {
    /// Parse a type string into a [`TypeExpr`].
    pub fn parse(type_str: &'a str) -> Result<Self, TypeParseError> {
        super::type_parser::parse(type_str)
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
                    if let TypeExprParam::Type(t) = p {
                        t.visit_references(on_ext);
                    }
                }
            }
            TypeExpr::Simple(_name, params, _) => {
                for p in params {
                    if let TypeExprParam::Type(t) = p {
                        t.visit_references(on_ext);
                    }
                }
            }
            TypeExpr::TypeVariable(..) => {}
            TypeExpr::Func(parameters, return_type, _) => {
                for parameter in parameters {
                    if let TypeExprParam::Type(t) = parameter {
                        t.visit_references(on_ext);
                    }
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
            ("MAP", TypeExpr::Simple("MAP", vec![], false)),
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
                "u!geo?<i32?, point<i32, i32>>",
                TypeExpr::UserDefined(
                    "geo",
                    vec![
                        TypeExprParam::Type(TypeExpr::Simple("i32", vec![], true)),
                        TypeExprParam::Type(TypeExpr::Simple(
                            "point",
                            vec![
                                TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false)),
                                TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false)),
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
                        TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false)),
                        TypeExprParam::Type(TypeExpr::Simple("string", vec![], false)),
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
    fn test_numeric_parameter_expressions() {
        let expression = parse("decimal<P1 + P2, S>");
        let TypeExpr::Simple("decimal", parameters, false) = expression else {
            panic!("expected decimal");
        };
        assert_eq!(parameters.len(), 2);
        assert_eq!(
            parameters[0],
            TypeExprParam::Expression(IntegerParameter::parse("P1 + P2").unwrap())
        );
        assert_eq!(
            parameters[1],
            TypeExprParam::Type(TypeExpr::Simple("S", vec![], false))
        );
    }

    #[test]
    fn test_invalid_numeric_parameter_expressions() {
        for source in [
            "varchar<L +>",
            "varchar<L + * 2>",
            "varchar<(L + 1>",
            "varchar<L + 1)>",
            "varchar<()>",
        ] {
            assert!(TypeExpr::parse(source).is_err(), "{source}");
        }
    }

    #[test]
    fn test_numeric_parameters_are_not_extension_type_references() {
        let mut references = Vec::new();
        parse("u!custom<P + 1>").visit_references(&mut |name| references.push(name.to_string()));
        assert_eq!(references, vec!["custom"]);
    }

    #[test]
    fn test_function_types() {
        let cases = vec![
            (
                "func<i32 -> i64>",
                TypeExpr::Func(
                    vec![TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false))],
                    Box::new(TypeExpr::Simple("i64", vec![], false)),
                    false,
                ),
            ),
            (
                "FUNC?<any1 -> any2>",
                TypeExpr::Func(
                    vec![TypeExprParam::Type(TypeExpr::TypeVariable(1, false))],
                    Box::new(TypeExpr::TypeVariable(2, false)),
                    true,
                ),
            ),
            (
                "func<(i32, list<any1>) -> i64?>",
                TypeExpr::Func(
                    vec![
                        TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false)),
                        TypeExprParam::Type(TypeExpr::Simple(
                            "list",
                            vec![TypeExprParam::Type(TypeExpr::TypeVariable(1, false))],
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
                    vec![TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false))],
                    Box::new(TypeExpr::Func(
                        vec![TypeExprParam::Type(TypeExpr::Simple("i64", vec![], false))],
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
            parse("func<func<i32 -> i64> -> bool>"),
            TypeExpr::Func(
                vec![TypeExprParam::Type(inner.clone())],
                Box::new(parse("bool")),
                false,
            )
        );
        assert_eq!(
            parse("list<func<i32 -> i64>>"),
            TypeExpr::Simple("list", vec![TypeExprParam::Type(inner)], false)
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
                vec![TypeExprParam::Type(parse("map<string, i64>"))],
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
            "func<(i32,,i64) -> bool>",
            "func<(func<i32 -> i64>,) -> bool>",
        ] {
            assert_eq!(
                TypeExpr::parse(input),
                Err(TypeParseError::EmptyParameter),
                "{input}"
            );
        }
    }

    #[test]
    fn test_function_type_errors() {
        let cases = [
            (
                "func<i32, i64>",
                TypeParseError::UnexpectedToken {
                    position: 8,
                    expected: "'->'",
                },
            ),
            (
                "func<() -> i64>",
                TypeParseError::UnexpectedToken {
                    position: 6,
                    expected: "a function parameter",
                },
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(
                TypeExpr::parse(expr),
                Err(expected),
                "unexpected error for {expr}"
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
                vec![
                    TypeExprParam::Type(parse("string")),
                    TypeExprParam::Type(parse("List<i32>"))
                ],
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
