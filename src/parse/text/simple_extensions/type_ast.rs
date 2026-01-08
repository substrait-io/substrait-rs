// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
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
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr<'a> {
    /// A type with a name, optional parameters, and nullability
    Simple(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// A user-defined extension type, indicated by `u!Name`, with optional
    /// parameters and nullability
    UserDefined(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// Type variable (e.g., any1, any2)
    TypeVariable(u32, bool),
}

/// A parsed parameter to a parameterized type
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExprParam<'a> {
    /// A nested type parameter
    Type(TypeExpr<'a>),
    /// An integer literal parameter
    Integer(i64),
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
                // Strip u! prefix when reporting linkage
                on_ext(name);
                for p in params {
                    if let TypeExprParam::Type(t) = p {
                        t.visit_references(on_ext);
                    }
                }
            }
            TypeExpr::Simple(_name, params, _) => {
                // Simple types should only be builtins.
                // Extension types MUST use u! prefix and appear as UserDefined.
                // We don't track Simple types as extension references - they'll error during conversion.
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
    if let Ok(i) = s.parse::<i64>() {
        return Ok(TypeExprParam::Integer(i));
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
}
