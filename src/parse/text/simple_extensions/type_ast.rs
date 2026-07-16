// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
//!
//! This module provides syntactic parsing of Substrait type strings into an AST.
//! It does NOT validate that type names are valid builtins or that extension types exist -
//! semantic validation occurs later when converting to [`ConcreteType`](super::types::ConcreteType).
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

/// A parsed type expression from a type string, with lifetime tied to the original string.
///
/// This represents the syntactic structure only - type names are not validated.
/// Convert to [`ConcreteType`](super::types::ConcreteType) for semantic validation.
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
}

/// A parsed parameter to a parameterized type
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExprParam<'a> {
    /// A nested type parameter
    Type(TypeExpr<'a>),
    /// An integer literal parameter
    Integer(i64),
    /// A named parameter reference (e.g., `P`, `S` in `DECIMAL<P, S>`)
    ///
    /// These are used in function signatures to indicate that the actual
    /// parameter value is determined at call time based on input types.
    ParameterName(&'a str),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("missing closing angle bracket in parameter list: {0}")]
    ExpectedClosingAngleBracket(String),
    #[error("Type variation syntax is not supported: {0}")]
    UnsupportedVariation(String),
}

impl<'a> TypeExpr<'a> {
    /// Parse a type string into a [`TypeExpr`].
    pub fn parse(type_str: &'a str) -> Result<Self, TypeParseError> {
        // Handle type variables like any1, any2, etc.
        if let Some(suffix) = type_str.strip_prefix("any") {
            let (middle, nullable) = match suffix.strip_suffix('?') {
                Some(middle) => (middle, true),
                None => (suffix, false),
            };

            if let Ok(id) = middle.parse::<u32>() {
                return Ok(TypeExpr::TypeVariable(id, nullable));
            }
        }

        let (user_defined, rest) = match type_str.strip_prefix("u!") {
            Some(right) => (true, right),
            None => (false, type_str),
        };

        let (name_and_nullable, params): (&'a str, Vec<TypeExprParam<'a>>) =
            match rest.split_once('<') {
                Some((n, p)) => match p.strip_suffix('>') {
                    Some(p) => (n, parse_params(p)?),
                    None => return Err(TypeParseError::ExpectedClosingAngleBracket(p.to_string())),
                },
                None => (rest, vec![]),
            };

        if name_and_nullable.contains('[') || name_and_nullable.contains(']') {
            return Err(TypeParseError::UnsupportedVariation(type_str.to_string()));
        }

        let (name, nullable) = match name_and_nullable.strip_suffix('?') {
            Some(name) => (name, true),
            None => (name_and_nullable, false),
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
        }
    }
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
    // Try to parse as integer literal first
    if let Ok(i) = s.parse::<i64>() {
        return Ok(TypeExprParam::Integer(i));
    }

    // Check if this looks like a simple parameter name (single identifier, not a type)
    // Parameter names are typically short identifiers like P, S, L, N
    // They don't contain '<', '>', '?', '!' or other type-related syntax
    if is_parameter_name(s) {
        return Ok(TypeExprParam::ParameterName(s));
    }

    // Otherwise parse as a nested type
    Ok(TypeExprParam::Type(TypeExpr::parse(s)?))
}

/// Check if a string looks like an integer parameter name (e.g., `P`, `S`, `L1`)
/// rather than a type expression.
///
/// Integer parameter names are identifiers that:
/// - Don't contain type syntax characters like `<`, `>`, `?`, `!`
/// - Don't match known type names
/// - Are NOT type variables (any1, any2, etc.)
/// - Typically are short uppercase identifiers, but we accept any valid identifier
fn is_parameter_name(s: &str) -> bool {
    // Must not be empty
    if s.is_empty() {
        return false;
    }

    // Must not contain type syntax
    if s.contains('<') || s.contains('>') || s.contains('?') || s.contains('!') {
        return false;
    }

    // Check for type variables (any1, any2, etc.) - these are types, not integer parameters
    if is_type_variable(s) {
        return false;
    }

    // Must be a valid identifier (start with letter or _, followed by letters, digits, _)
    let mut chars = s.chars();
    let first = match chars.next() {
        Some(c) => c,
        None => return false,
    };

    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }

    // All subsequent characters must be alphanumeric or underscore
    if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return false;
    }

    // Check that it's NOT a known type name (those should be parsed as types)
    let lower = s.to_ascii_lowercase();
    !matches!(
        lower.as_str(),
        "bool"
            | "boolean"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "fp32"
            | "fp64"
            | "string"
            | "binary"
            | "timestamp"
            | "timestamp_tz"
            | "date"
            | "time"
            | "interval_year"
            | "uuid"
            | "list"
            | "map"
            | "struct"
            | "any"
    )
}

/// Check if a string is a type variable pattern (any1, any2, any1?, any2?, etc.)
fn is_type_variable(s: &str) -> bool {
    let s = s.strip_suffix('?').unwrap_or(s);
    if let Some(suffix) = s.strip_prefix("any") {
        suffix.parse::<u32>().is_ok()
    } else {
        false
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
            ("timestamp", TypeExpr::Simple("timestamp", vec![], false)),
            (
                "timestamp_tz?",
                TypeExpr::Simple("timestamp_tz", vec![], true),
            ),
            ("time", TypeExpr::Simple("time", vec![], false)),
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
    fn test_variation_not_supported() {
        let cases = vec!["i32[1]", "Foo?[1]", "u!bar[2]"];

        for expr in cases {
            match TypeExpr::parse(expr) {
                Err(TypeParseError::UnsupportedVariation(s)) => assert_eq!(s, expr),
                other => panic!("expected UnsupportedVariation for {expr}, got {other:?}"),
            }
        }
    }

    #[test]
    fn test_parameter_names() {
        // Integer parameter names like P, S, L
        let cases = vec![
            (
                "DECIMAL<P, S>",
                TypeExpr::Simple(
                    "DECIMAL",
                    vec![
                        TypeExprParam::ParameterName("P"),
                        TypeExprParam::ParameterName("S"),
                    ],
                    false,
                ),
            ),
            (
                "fixedchar?<L>",
                TypeExpr::Simple("fixedchar", vec![TypeExprParam::ParameterName("L")], true),
            ),
            (
                "precision_timestamp_tz<P>",
                TypeExpr::Simple(
                    "precision_timestamp_tz",
                    vec![TypeExprParam::ParameterName("P")],
                    false,
                ),
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(parse(expr), expected, "unexpected parse for {expr}");
        }
    }

    #[test]
    fn test_type_variable_in_container() {
        // any1 inside a container should be parsed as a type variable, not a parameter name
        let cases = vec![
            (
                "List<any1>",
                TypeExpr::Simple(
                    "List",
                    vec![TypeExprParam::Type(TypeExpr::TypeVariable(1, false))],
                    false,
                ),
            ),
            (
                "Map<any1, any2?>",
                TypeExpr::Simple(
                    "Map",
                    vec![
                        TypeExprParam::Type(TypeExpr::TypeVariable(1, false)),
                        TypeExprParam::Type(TypeExpr::TypeVariable(2, true)),
                    ],
                    false,
                ),
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(parse(expr), expected, "unexpected parse for {expr}");
        }
    }
}
