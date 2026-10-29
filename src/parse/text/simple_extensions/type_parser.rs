// SPDX-License-Identifier: Apache-2.0

//! Parse type syntax in two steps: tokenize the source, then consume tokens
//! recursively to build a [`TypeExpr`].
//!
//! For `func<func<i32 -> i64> -> bool>`, each `->` is a single arrow token.
//! Parsing the inner function consumes its closing `>` before the outer rule
//! expects its own arrow. No rule needs to count brackets or search ahead.
//!
//! Names borrow from the source, and token positions are byte offsets into that
//! source. Built-in names, parameter counts, and literal ranges are validated
//! later when converting the AST to [`Type`](super::types::Type).

use super::type_ast::{TypeExpr, TypeExprParam, TypeParseError};
use super::type_tokenizer::{Token, TokenKind, tokenize};

/// Parse one complete type, rejecting any tokens left after it.
/// For example, `list<i32>` succeeds, but `list<i32> i64` does not.
pub(super) fn parse(source: &str) -> Result<TypeExpr<'_>, TypeParseError> {
    let mut parser = Parser::new(source)?;
    let expression = parser.parse_type()?;
    if parser.peek().is_some() {
        return Err(parser.error("end of type"));
    }
    Ok(expression)
}

/// Recursive-descent parser whose index points to the next unconsumed token.
/// Nested rules share this position and leave their caller's delimiters intact.
struct Parser<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    index: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a str) -> Result<Self, TypeParseError> {
        Ok(Self {
            source,
            tokens: tokenize(source)?,
            index: 0,
        })
    }

    fn peek(&self) -> Option<TokenKind> {
        self.tokens.get(self.index).map(|token| token.kind)
    }

    fn error(&self, expected: &'static str) -> TypeParseError {
        TypeParseError::UnexpectedToken {
            position: self
                .tokens
                .get(self.index)
                .map_or(self.source.len(), |token| token.position),
            expected,
        }
    }

    /// Advance only if the next token matches; otherwise leave it for another rule.
    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.peek() == Some(kind) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    /// Consume a required token or report what was expected at the current position.
    fn expect(&mut self, kind: TokenKind) -> Result<Token<'a>, TypeParseError> {
        if self.peek() != Some(kind) {
            return Err(self.error(kind.description()));
        }
        let token = self.tokens[self.index];
        self.index += 1;
        Ok(token)
    }

    /// Consume a type or parameter name, including names with keyword tokens.
    fn expect_name(&mut self) -> Result<Token<'a>, TypeParseError> {
        let token = self
            .tokens
            .get(self.index)
            .copied()
            .filter(|token| token.kind.is_name())
            .ok_or_else(|| self.error("a type name"))?;
        self.index += 1;
        Ok(token)
    }

    /// Parse one type expression using the rule selected by its first token.
    fn parse_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        match self.peek() {
            Some(TokenKind::UserDefined) => self.parse_user_defined_type(),
            Some(TokenKind::Func) => self.parse_function_type(),
            Some(TokenKind::AnyVar) => self.parse_type_variable(),
            _ => self.parse_named_type(),
        }
    }

    /// Parse a user-defined type such as `u!point?<i32>`.
    fn parse_user_defined_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        self.expect(TokenKind::UserDefined)?;
        let name = self.expect_name()?;
        let nullable = self.consume(TokenKind::QuestionMark);
        let parameters = self.parse_optional_type_parameters()?;
        Ok(TypeExpr::UserDefined(name.text, parameters, nullable))
    }

    /// Parse a name with optional nullability and parameters, such as `list?<i32>`.
    /// Unknown names are retained for semantic validation.
    fn parse_named_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let name = self.expect_name()?;
        let nullable = self.consume(TokenKind::QuestionMark);
        let parameters = self.parse_optional_type_parameters()?;
        Ok(TypeExpr::Simple(name.text, parameters, nullable))
    }

    /// Parse a numbered variable such as `any1?`.
    /// Uppercase prefixes, out-of-range IDs, and parameterized names remain named types.
    fn parse_type_variable(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let name = self.expect(TokenKind::AnyVar)?;
        let nullable = self.consume(TokenKind::QuestionMark);
        if self.peek() == Some(TokenKind::OpenAngle) {
            let parameters = self.parse_optional_type_parameters()?;
            return Ok(TypeExpr::Simple(name.text, parameters, nullable));
        }
        let Some(id) = parse_type_variable_id(name.text) else {
            return Ok(TypeExpr::Simple(name.text, Vec::new(), nullable));
        };
        Ok(TypeExpr::TypeVariable(id, nullable))
    }

    /// Parse an optional `<...>` parameter list, consuming both angle brackets.
    fn parse_optional_type_parameters(&mut self) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        if self.consume(TokenKind::OpenAngle) {
            self.parse_type_parameters()
        } else {
            Ok(Vec::new())
        }
    }

    /// Parse comma-separated parameters after `<`, consuming the closing `>`.
    ///
    /// In `map<string, list<i32>>`, parsing `list<i32>` consumes the inner `>`,
    /// leaving the outer `>` for this rule. Empty lists are retained so semantic
    /// validation can check arity, but empty entries such as `list<i32,>` fail.
    fn parse_type_parameters(&mut self) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        let mut parameters = Vec::new();
        if self.consume(TokenKind::CloseAngle) {
            return Ok(parameters);
        }
        loop {
            parameters.push(self.parse_parameter()?);
            if matches!(self.peek(), Some(TokenKind::Plus | TokenKind::Minus)) {
                return Err(TypeParseError::UnsupportedNumericExpression(
                    self.source.to_string(),
                ));
            }
            if self.consume(TokenKind::CloseAngle) {
                return Ok(parameters);
            }
            self.expect(TokenKind::Comma)?;
        }
    }

    /// Parse a function type such as `func<(i32, i64) -> boolean>`.
    /// Multiple parameters require parentheses; a single parameter may omit them.
    /// A bare `func` name is retained for semantic validation.
    fn parse_function_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let name = self.expect(TokenKind::Func)?;
        let nullable = self.consume(TokenKind::QuestionMark);
        if !self.consume(TokenKind::OpenAngle) {
            return Ok(TypeExpr::Simple(name.text, Vec::new(), nullable));
        }
        let parameters = self.parse_function_parameters()?;
        self.expect(TokenKind::Arrow)?;
        let return_type = self.parse_type()?;
        self.expect(TokenKind::CloseAngle)?;
        Ok(TypeExpr::Func(parameters, Box::new(return_type), nullable))
    }

    /// Parse one function parameter or a nonempty parenthesized parameter list.
    fn parse_function_parameters(&mut self) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        if !self.consume(TokenKind::OpenParen) {
            return Ok(vec![self.parse_parameter()?]);
        }
        if self.peek() == Some(TokenKind::CloseParen) {
            return Err(self.error("a function parameter"));
        }

        let mut parameters = vec![self.parse_parameter()?];
        while self.consume(TokenKind::Comma) {
            parameters.push(self.parse_parameter()?);
        }
        self.expect(TokenKind::CloseParen)?;
        Ok(parameters)
    }

    /// Parse one integer literal or nested type, leaving its separator unconsumed.
    ///
    /// For `decimal<10, 2>`, the first call consumes only `10`, not the comma.
    /// A sign is combined with its digits here, allowing the full `i64` range
    /// without making `-` part of every integer token.
    fn parse_parameter(&mut self) -> Result<TypeExprParam<'a>, TypeParseError> {
        match self.peek() {
            Some(TokenKind::Integer | TokenKind::Plus | TokenKind::Minus) => {
                let start = self.tokens[self.index].position;
                if !self.consume(TokenKind::Plus) {
                    self.consume(TokenKind::Minus);
                }
                let token = self.expect(TokenKind::Integer)?;
                let end = token.position + token.text.len();
                let value = self.source[start..end]
                    .parse()
                    .map_err(|_| self.error("an i64 integer literal"))?;
                Ok(TypeExprParam::Integer(value))
            }
            Some(TokenKind::Comma | TokenKind::CloseAngle | TokenKind::CloseParen) | None => {
                Err(TypeParseError::EmptyParameter)
            }
            _ => Ok(TypeExprParam::Type(self.parse_type()?)),
        }
    }
}

/// Recognize a case-sensitive `any` prefix followed by a `u32` variable ID.
fn parse_type_variable_id(name: &str) -> Option<u32> {
    name.strip_prefix("any")?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_parameters_leave_the_arrow_for_the_caller() {
        for (source, parameter_types) in [
            ("i32 -> i64", vec!["i32"]),
            ("map<string, i64> -> boolean", vec!["map<string, i64>"]),
            (
                "(func<i32 -> i64>, list<any1>) -> boolean",
                vec!["func<i32 -> i64>", "list<any1>"],
            ),
        ] {
            let mut parser = Parser::new(source).unwrap();
            let expected: Vec<_> = parameter_types
                .iter()
                .map(|parameter| TypeExprParam::Type(parse(parameter).unwrap()))
                .collect();

            assert_eq!(
                parser.parse_function_parameters().unwrap(),
                expected,
                "{source}"
            );
            assert_eq!(parser.peek(), Some(TokenKind::Arrow), "{source}");
        }
    }

    #[test]
    fn type_parameters_consume_only_their_own_closing_bracket() {
        for (source, expected) in [
            ("<>>", vec![]),
            (
                "<list<i32>, 2>>",
                vec![
                    TypeExprParam::Type(parse("list<i32>").unwrap()),
                    TypeExprParam::Integer(2),
                ],
            ),
        ] {
            let mut parser = Parser::new(source).unwrap();
            parser.expect(TokenKind::OpenAngle).unwrap();

            assert_eq!(
                parser.parse_type_parameters().unwrap(),
                expected,
                "{source}"
            );
            assert_eq!(parser.peek(), Some(TokenKind::CloseAngle), "{source}");
            parser.expect(TokenKind::CloseAngle).unwrap();
            assert_eq!(parser.peek(), None, "{source}");
        }
    }

    #[test]
    fn type_parameters_reject_empty_entries() {
        for source in [",i32>", "i32,>", "i32,,i64>"] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_type_parameters(),
                Err(TypeParseError::EmptyParameter),
                "{source}"
            );
        }
    }

    #[test]
    fn integer_parameters_support_signs_and_the_full_i64_range() {
        for (source, expected) in [
            ("0,", 0),
            ("+10,", 10),
            ("-2,", -2),
            ("9223372036854775807,", i64::MAX),
            ("+9223372036854775807,", i64::MAX),
            ("-9223372036854775808,", i64::MIN),
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_parameter().unwrap(),
                TypeExprParam::Integer(expected),
                "{source}"
            );
            assert_eq!(parser.peek(), Some(TokenKind::Comma), "{source}");
        }
    }

    #[test]
    fn integer_parameters_reject_overflow_and_signs_without_digits() {
        for source in [
            "9223372036854775808",
            "-9223372036854775809",
            "+",
            "-",
            "+i32",
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert!(parser.parse_parameter().is_err(), "{source}");
        }
    }

    #[test]
    fn nested_type_parameters_leave_the_separator_for_the_caller() {
        let mut parser = Parser::new("map<string, func<i32 -> i64>>, boolean").unwrap();
        assert_eq!(
            parser.parse_parameter().unwrap(),
            TypeExprParam::Type(parse("map<string, func<i32 -> i64>>").unwrap())
        );
        assert_eq!(parser.peek(), Some(TokenKind::Comma));
    }

    #[test]
    fn grouped_parameters_can_contain_multi_parameter_functions() {
        assert_eq!(
            parse("func<(func<(i32, i64) -> bool>, i32) -> bool>").unwrap(),
            TypeExpr::Func(
                vec![
                    TypeExprParam::Type(parse("func<(i32, i64) -> bool>").unwrap()),
                    TypeExprParam::Type(parse("i32").unwrap()),
                ],
                Box::new(parse("bool").unwrap()),
                false,
            )
        );
    }

    #[test]
    fn keyword_tokens_preserve_names_in_the_ast() {
        assert_eq!(
            parse("LiSt<DeCiMaL<P, S>>").unwrap(),
            TypeExpr::Simple(
                "LiSt",
                vec![TypeExprParam::Type(parse("DeCiMaL<P, S>").unwrap())],
                false
            ),
        );
        for name in [
            "func", "List", "DECIMAL", "any", "any1", "i32", "Map", "VARCHAR", "string", "dec",
            "NStruct",
        ] {
            let source = format!("u!{name}<i32>");
            assert_eq!(
                parse(&source).unwrap(),
                TypeExpr::UserDefined(
                    name,
                    vec![TypeExprParam::Type(parse("i32").unwrap())],
                    false
                ),
            );
        }
        assert_eq!(parse("any1").unwrap(), TypeExpr::TypeVariable(1, false));
        assert_eq!(
            parse("ANY1").unwrap(),
            TypeExpr::Simple("ANY1", vec![], false)
        );
        assert_eq!(
            parse("ANY").unwrap(),
            TypeExpr::Simple("ANY", vec![], false)
        );
    }

    #[test]
    fn bare_function_keyword_is_retained_as_a_name() {
        assert_eq!(
            parse("FuNc?").unwrap(),
            TypeExpr::Simple("FuNc", vec![], true)
        );
    }

    #[test]
    fn type_variable_ids_require_a_lowercase_prefix_and_fit_in_u32() {
        for (name, expected) in [
            ("any0", Some(0)),
            ("any1", Some(1)),
            ("any4294967295", Some(u32::MAX)),
            ("any4294967296", None),
            ("ANY1", None),
            ("any", None),
            ("anything", None),
        ] {
            assert_eq!(parse_type_variable_id(name), expected, "{name}");
        }
    }

    #[test]
    fn numbered_variable_spellings_preserve_named_type_fallbacks() {
        for name in ["ANY1", "any4294967296"] {
            assert_eq!(parse(name).unwrap(), TypeExpr::Simple(name, vec![], false));
        }
        assert_eq!(
            parse("any1<>").unwrap(),
            TypeExpr::Simple("any1", vec![], false)
        );
        assert_eq!(
            parse("any1?<i32>").unwrap(),
            TypeExpr::Simple(
                "any1",
                vec![TypeExprParam::Type(parse("i32").unwrap())],
                true,
            )
        );
    }

    #[test]
    fn malformed_type_rules_propagate_parameter_and_delimiter_errors() {
        for (source, expected) in [
            (
                "u!",
                TypeParseError::UnexpectedToken {
                    position: 2,
                    expected: "a type name",
                },
            ),
            ("u!point<i32,>", TypeParseError::EmptyParameter),
            ("any1<i32,>", TypeParseError::EmptyParameter),
            (
                "func<i32 -> i64",
                TypeParseError::UnexpectedToken {
                    position: 15,
                    expected: "'>'",
                },
            ),
            (
                "func<-> i64>",
                TypeParseError::UnexpectedToken {
                    position: 5,
                    expected: "a type name",
                },
            ),
        ] {
            assert_eq!(parse(source), Err(expected), "{source}");
        }
    }

    #[test]
    fn unexpected_input_has_a_byte_position() {
        assert_eq!(
            parse("list<i32)"),
            Err(TypeParseError::UnexpectedToken {
                position: 8,
                expected: "','"
            })
        );
    }

    #[test]
    fn nested_function_parameters_parse_without_scanning_for_delimiters() {
        assert_eq!(
            parse("func<func<i32 -> i64> -> bool>").unwrap(),
            TypeExpr::Func(
                vec![TypeExprParam::Type(parse("func<i32 -> i64>").unwrap())],
                Box::new(parse("bool").unwrap()),
                false,
            )
        );
    }
}
