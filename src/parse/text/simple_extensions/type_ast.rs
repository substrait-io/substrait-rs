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

use super::numeric_expression::{NumericExpression, NumericExpressionParseError};

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
    Expression(NumericExpression),
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
    #[error("invalid numeric expression: {0}")]
    InvalidNumericExpression(#[from] NumericExpressionParseError),
}

impl<'a> TypeExpr<'a> {
    /// Parse a type string into a [`TypeExpr`].
    pub fn parse(type_str: &'a str) -> Result<Self, TypeParseError> {
        if let Some(variable) = parse_type_variable(type_str) {
            return Ok(variable);
        }

        let (user_defined, rest) = match type_str.strip_prefix("u!") {
            Some(right) => (true, right),
            None => (false, type_str),
        };

        let (name, nullable, params) = split_name_and_params(rest)?;
        if name.contains('[') || name.contains(']') {
            return Err(TypeParseError::UnsupportedVariation(type_str.to_string()));
        }

        // The arrow in `func<P -> R>` is not a parameter separator, so function
        // types must be parsed before the generic parameter parser runs.
        if !user_defined
            && name.eq_ignore_ascii_case("func")
            && let Some(p) = params
        {
            return parse_func_type(p, nullable);
        }

        let params = match params {
            Some(p) => parse_params(p)?,
            None => Vec::new(),
        };

        if user_defined {
            Ok(TypeExpr::UserDefined(name, params, nullable))
        } else {
            Ok(TypeExpr::Simple(name, params, nullable))
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

/// Parse a numbered type variable such as `any1` or `any2?`.
/// Returns `None` for anything else, including a bare `any`.
fn parse_type_variable(s: &str) -> Option<TypeExpr<'_>> {
    let suffix = s.strip_prefix("any")?;
    let (middle, nullable) = match suffix.strip_suffix('?') {
        Some(middle) => (middle, true),
        None => (suffix, false),
    };
    let id = middle.parse::<u32>().ok()?;
    Some(TypeExpr::TypeVariable(id, nullable))
}

/// Split a type string into its name, trailing nullability, and raw parameters.
/// For example, `List?<i32, i64>` yields `("List", true, Some("i32, i64"))`.
fn split_name_and_params(s: &str) -> Result<(&str, bool, Option<&str>), TypeParseError> {
    let (name_and_nullable, params) = match s.split_once('<') {
        Some((name, params)) => match params.strip_suffix('>') {
            Some(params) => (name, Some(params)),
            None => {
                return Err(TypeParseError::ExpectedClosingAngleBracket(
                    params.to_string(),
                ));
            }
        },
        None => (s, None),
    };

    let (name, nullable) = match name_and_nullable.strip_suffix('?') {
        Some(name) => (name, true),
        None => (name_and_nullable, false),
    };

    Ok((name, nullable, params))
}

/// Parse the body of a function type: parameters, an arrow, and a return type.
///
/// For example, `"any1 -> any2"` parses the parameter `any1`, skips the arrow,
/// and recursively parses the return type `any2`; `"(i32, i64) -> boolean?"`
/// strips the grouping parentheses before parsing the two parameters.
fn parse_func_type<'a>(s: &'a str, nullable: bool) -> Result<TypeExpr<'a>, TypeParseError> {
    let Some((params, return_type)) = split_arrow(s) else {
        return Err(TypeParseError::MissingArrow(s.to_string()));
    };

    let params = params
        .strip_prefix('(')
        .and_then(|p| p.strip_suffix(')'))
        .unwrap_or(params);
    if params.is_empty() {
        return Err(TypeParseError::EmptyFunctionParameters(s.to_string()));
    }

    let parameters = parse_params(params)?;
    let return_type = TypeExpr::parse(return_type)?;
    Ok(TypeExpr::Func(parameters, Box::new(return_type), nullable))
}

/// Split function type parameters from the return type at the top-level arrow,
/// ignoring arrows nested inside angle brackets or parentheses, and trim both halves.
///
/// For example, `"i32 -> func<i64 -> i64>"` splits into `"i32"` and
/// `"func<i64 -> i64>"`, stopping at the first arrow that is not nested.
fn split_arrow(s: &str) -> Option<(&str, &str)> {
    let mut depth = 0;
    for (i, c) in s.char_indices() {
        match c {
            '<' | '(' => depth += 1,
            '>' | ')' => depth -= 1,
            '-' if depth == 0 && s[i..].starts_with("->") => {
                return Some((s[..i].trim(), s[i + 2..].trim()));
            }
            _ => {}
        }
    }
    None
}

fn parse_params<'a>(s: &'a str) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0;

    for (i, c) in s.char_indices() {
        match c {
            '<' => depth += 1,
            '>' => depth -= 1,
            ',' if depth == 0 => {
                result.push(parse_param(s[start..i].trim())?);
                start = i + 1;
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(TypeParseError::ExpectedClosingAngleBracket(s.to_string()));
    }

    if start < s.len() {
        result.push(parse_param(s[start..].trim())?);
    }

    Ok(result)
}

fn parse_param<'a>(s: &'a str) -> Result<TypeExprParam<'a>, TypeParseError> {
    if let Ok(i) = s.parse::<i64>() {
        return Ok(TypeExprParam::Integer(i));
    }
    if !s.contains('<') && s.contains('+') {
        return Ok(TypeExprParam::Expression(NumericExpression::parse(s)?));
    }
    Ok(TypeExprParam::Type(TypeExpr::parse(s)?))
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
            TypeExprParam::Expression(NumericExpression::parse("P1 + P2").unwrap())
        );
        assert_eq!(
            parameters[1],
            TypeExprParam::Type(TypeExpr::Simple("S", vec![], false))
        );
    }

    #[test]
    fn test_invalid_numeric_parameter_expressions() {
        for source in ["varchar<L +>", "varchar<L + * 2>", "varchar<(L + 1)>"] {
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
    fn test_function_type_errors() {
        let cases = [
            (
                "func<i32, i64>",
                TypeParseError::MissingArrow("i32, i64".to_string()),
            ),
            (
                "func<() -> i64>",
                TypeParseError::EmptyFunctionParameters("() -> i64".to_string()),
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
        parse("func<u!point, any1 -> u!edge?>")
            .visit_references(&mut |name| refs.push(name.to_string()));
        assert_eq!(refs, vec!["point".to_string(), "edge".to_string()]);
    }

    #[test]
    fn test_parse_type_variable() {
        let cases = [
            ("any1", Some(TypeExpr::TypeVariable(1, false))),
            ("any2?", Some(TypeExpr::TypeVariable(2, true))),
            ("any", None),
            ("anyfoo", None),
            ("i32", None),
        ];

        for (input, expected) in cases {
            assert_eq!(
                parse_type_variable(input),
                expected,
                "unexpected result for {input}"
            );
        }
    }

    #[test]
    fn test_split_name_and_params() {
        let cases = [
            ("i32", ("i32", false, None)),
            ("i32?", ("i32", true, None)),
            ("List?<i32, i64>", ("List", true, Some("i32, i64"))),
            (
                "Map<string, List<i32>>",
                ("Map", false, Some("string, List<i32>")),
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                split_name_and_params(input),
                Ok(expected),
                "unexpected split for {input}"
            );
        }

        assert_eq!(
            split_name_and_params("List<i32"),
            Err(TypeParseError::ExpectedClosingAngleBracket(
                "i32".to_string()
            ))
        );
    }

    #[test]
    fn test_split_arrow() {
        let cases = [
            ("i32 -> i64", Some(("i32", "i64"))),
            ("(i32, i64) -> boolean?", Some(("(i32, i64)", "boolean?"))),
            ("i32 -> func<i64 -> i64>", Some(("i32", "func<i64 -> i64>"))),
            ("i32, i64", None),
        ];

        for (input, expected) in cases {
            assert_eq!(split_arrow(input), expected, "unexpected split for {input}");
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
