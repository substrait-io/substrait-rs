// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.

use crate::parse::text::simple_extensions::types::is_builtin_type_name;

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
    /// A string literal parameter (unquoted)
    String(&'a str),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("Parameter list {0} Must start and end with angle brackets")]
    ExpectedClosingAngleBracket(String),
}

impl<'a> TypeExpr<'a> {
    /// Parse a type string into a ParsedType
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
            TypeExpr::Simple(name, params, _) => {
                if !is_builtin_type_name(name) {
                    on_ext(name);
                }
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
        let expr = "u!geo?<i32?, point<i32, i32>>";
        match parse(expr) {
            TypeExpr::UserDefined(name, params, nullable) => {
                assert_eq!(name, "geo", "unexpected name for {expr}");
                assert!(nullable, "{expr} should be nullable");
                assert_eq!(
                    params,
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
                    ]
                );
            }
            other => panic!("unexpected parse result: {other:?}"),
        }

        let map_expr = "Map?<i32, string>";
        assert_eq!(
            parse(map_expr),
            TypeExpr::Simple(
                "Map",
                vec![
                    TypeExprParam::Type(TypeExpr::Simple("i32", vec![], false)),
                    TypeExprParam::Type(TypeExpr::Simple("string", vec![], false)),
                ],
                true,
            ),
            "unexpected map parse"
        );
    }

    #[test]
    fn test_visit_references_builtin_case_insensitive() {
        let cases = vec![
            ("DECIMAL<10,2>", Vec::<String>::new()),
            ("List<string>", Vec::<String>::new()),
            ("u!custom<i32>", vec!["custom".to_string()]),
            ("Geo<string>", vec!["Geo".to_string()]),
        ];

        for (expr, expected_refs) in cases {
            let mut refs = Vec::new();
            parse(expr).visit_references(&mut |name| refs.push(name.to_string()));
            assert_eq!(refs, expected_refs, "unexpected references for {expr}");
        }
    }
}
