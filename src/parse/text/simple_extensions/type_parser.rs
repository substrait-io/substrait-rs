// SPDX-License-Identifier: Apache-2.0

//! Parse type syntax in two steps: tokenize the source, then consume tokens
//! recursively to build a [`TypeExpr`].
//!
//! For `func<func<i32 -> i64> -> bool>`, each `->` is a single arrow token.
//! Parsing the inner function consumes its closing `>` before the outer rule
//! expects its own arrow. No rule needs to count brackets or search ahead.
//!
//! Names borrow from the source, and token positions are byte offsets into that
//! source. Built-in types select whether their parameters are types or integers.
//! Parameter counts and literal ranges are validated later when
//! converting the AST to [`Type`](super::types::Type).

use super::type_ast::{TypeExpr, TypeExprParam, TypeParseError};
use super::type_tokenizer::{Token, TokenKind, tokenize};
use super::types::{IntegerParameter, TypeKind};

/// Parse one complete type, rejecting any tokens left after it.
///
/// For example, `list<i32>` succeeds, but `list<i32> i64` does not.
pub(super) fn parse(source: &str) -> Result<TypeExpr<'_>, TypeParseError> {
    let mut parser = Parser::new(source)?;
    let expression = parser.parse_type()?;
    if parser.peek().is_some() {
        return Err(parser.error("end of type"));
    }
    Ok(expression)
}

enum ParameterKind {
    Integer,
    Type,
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

    /// Consume one token matching any of the given kinds and return its kind.
    /// Leave non-matching tokens unconsumed.
    fn consume_one_of(&mut self, kinds: &[TokenKind]) -> Option<TokenKind> {
        let kind = self.peek()?;
        if kinds.contains(&kind) {
            self.consume(kind);
            Some(kind)
        } else {
            None
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
            _ => {
                let name = self.expect_name()?;
                self.parse_named_type(name)
            }
        }
    }

    /// Parse an unparameterized user-defined type such as `u!point?`.
    fn parse_user_defined_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        self.expect(TokenKind::UserDefined)?;
        let name = self.expect_name()?;
        let nullable = self.consume(TokenKind::QuestionMark);
        if self.peek() == Some(TokenKind::OpenAngle) {
            return Err(TypeParseError::UnsupportedTypeParameters(
                self.source.into(),
            ));
        }
        Ok(TypeExpr::UserDefined(name.text, Vec::new(), nullable))
    }

    /// Parse nullability and parameters for an already-consumed type name.
    ///
    /// `decimal<P, S>` takes integer parameters, while `list<i32>` takes types.
    fn parse_named_type(&mut self, name: Token<'a>) -> Result<TypeExpr<'a>, TypeParseError> {
        debug_assert!(name.kind.is_name() && name.kind != TokenKind::AnyVar);
        let nullable = self.consume(TokenKind::QuestionMark);
        let parameters = match name.text.to_ascii_lowercase().as_str() {
            "fixedchar"
            | "varchar"
            | "fixedbinary"
            | "decimal"
            | "precision_time"
            | "precision_timestamp"
            | "precision_timestamp_tz"
            | "interval_day"
            | "interval_compound" => self.parse_parameters(ParameterKind::Integer)?,
            "list" | "map" | "struct" => self.parse_parameters(ParameterKind::Type)?,
            _ => {
                if self.peek() == Some(TokenKind::OpenAngle) {
                    return Err(TypeParseError::UnsupportedTypeParameters(
                        self.source.into(),
                    ));
                }
                Vec::new()
            }
        };
        Ok(TypeExpr::Simple(name.text, parameters, nullable))
    }

    /// Parse a numbered variable such as `any1?`.
    /// Variables do not take parameters.
    fn parse_type_variable(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let name = self.expect(TokenKind::AnyVar)?;
        let id = name.text[3..].parse().unwrap();
        let nullable = self.consume(TokenKind::QuestionMark);
        Ok(TypeExpr::TypeVariable(id, nullable))
    }

    /// Parse an optional `<...>` list using the enclosing type's parameter grammar.
    ///
    /// In `map<string, list<i32>>`, each call consumes only its own angle brackets.
    /// Empty lists are retained for arity validation, but empty entries fail.
    fn parse_parameters(
        &mut self,
        kind: ParameterKind,
    ) -> Result<Vec<TypeExprParam<'a>>, TypeParseError> {
        if !self.consume(TokenKind::OpenAngle) {
            return Ok(Vec::new());
        }
        let mut parameters = Vec::new();
        if self.consume(TokenKind::CloseAngle) {
            return Ok(parameters);
        }
        loop {
            parameters.push(match kind {
                ParameterKind::Integer => TypeExprParam::Integer(self.parse_integer_parameter()?),
                ParameterKind::Type => self.parse_type_parameter()?,
            });
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
            return Ok(vec![self.parse_type_parameter()?]);
        }
        if self.peek() == Some(TokenKind::CloseParen) {
            return Err(self.error("a function parameter"));
        }

        let mut parameters = vec![self.parse_type_parameter()?];
        while self.consume(TokenKind::Comma) {
            parameters.push(self.parse_type_parameter()?);
        }
        self.expect(TokenKind::CloseParen)?;
        Ok(parameters)
    }

    /// Parse a type parameter, leaving its separator unconsumed.
    ///
    /// In `map<string, i32>`, consume `string` but leave the comma.
    fn parse_type_parameter(&mut self) -> Result<TypeExprParam<'a>, TypeParseError> {
        match self.peek() {
            Some(TokenKind::Comma | TokenKind::CloseAngle | TokenKind::CloseParen) | None => {
                Err(TypeParseError::EmptyParameter)
            }
            _ => self.parse_type().map(TypeExprParam::Type),
        }
    }

    /// Parse an integer literal or parameter name, leaving its separator unconsumed.
    ///
    /// In `decimal<P, S>`, consume `P` but leave the comma.
    fn parse_integer_parameter(&mut self) -> Result<IntegerParameter, TypeParseError> {
        let parameter = match self.peek() {
            Some(TokenKind::Integer | TokenKind::Plus | TokenKind::Minus) => {
                IntegerParameter::Literal(self.parse_integer_literal()?)
            }
            Some(kind) if kind.is_name() => self.parse_integer_parameter_name()?,
            _ => return Err(self.error("an integer literal or parameter name")),
        };
        if matches!(self.peek(), Some(TokenKind::Plus | TokenKind::Minus)) {
            return Err(TypeParseError::UnsupportedNumericExpression(
                self.source.into(),
            ));
        }
        Ok(parameter)
    }

    /// Consume an integer parameter name, rejecting built-ins and numbered type variables.
    fn parse_integer_parameter_name(&mut self) -> Result<IntegerParameter, TypeParseError> {
        let name = self.expect_name()?;
        if !is_integer_parameter_name(name) {
            return Err(TypeParseError::UnexpectedToken {
                position: name.position,
                expected: "an integer literal or parameter name",
            });
        }
        Ok(IntegerParameter::Variable(name.text.into()))
    }

    /// Combine a sign with its digits, preserving the full `i64` range.
    ///
    /// For example, `-9223372036854775808` is parsed as one value, including its sign.
    fn parse_integer_literal(&mut self) -> Result<i64, TypeParseError> {
        let start = self.tokens[self.index].position;
        self.consume_one_of(&[TokenKind::Plus, TokenKind::Minus]);
        let token = self.expect(TokenKind::Integer)?;
        let end = token.position + token.text.len();
        self.source[start..end]
            .parse()
            .map_err(|_| TypeParseError::UnexpectedToken {
                position: start,
                expected: "an i64 integer literal",
            })
    }
}

/// Recognize integer parameter names without treating type names as variables.
fn is_integer_parameter_name(name: Token<'_>) -> bool {
    !TypeKind::is_builtin_name(name.text) && name.kind != TokenKind::AnyVar
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consume_one_of_returns_the_matched_kind() {
        for (source, expected, consumed) in [
            ("+1", Some(TokenKind::Plus), 1),
            ("-1", Some(TokenKind::Minus), 1),
            ("P", None, 0),
            ("", None, 0),
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.consume_one_of(&[TokenKind::Plus, TokenKind::Minus]),
                expected,
                "{source}"
            );
            assert_eq!(parser.index, consumed, "{source}");
        }
    }

    #[test]
    fn parse_numeric_parameters_reject_numbered_type_variables() {
        for (source, position) in [
            ("decimal<any1, S>", 8),
            ("decimal<P, any1>", 11),
            ("decimal<Any1, S>", 8),
            ("decimal<P, ANY1>", 11),
        ] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnexpectedToken {
                    position,
                    expected: "an integer literal or parameter name",
                }),
                "{source}"
            );
        }
    }

    #[test]
    fn numeric_parameter_names_are_explicit_in_the_ast() {
        for name in ["P", "$L", "_S", "any1extra"] {
            let source = format!("decimal<{name}, 0>");
            assert_eq!(
                parse(&source).unwrap(),
                TypeExpr::Simple(
                    "decimal",
                    vec![
                        TypeExprParam::Integer(IntegerParameter::Variable(name.into())),
                        TypeExprParam::Integer(IntegerParameter::Literal(0)),
                    ],
                    false,
                ),
                "{source}"
            );
        }
    }

    #[test]
    fn parameterized_user_defined_and_unknown_types_are_unsupported() {
        for source in [
            "u!foo<>",
            "u!foo<P>",
            "u!foo?<i32>",
            "list<u!foo<1>>",
            "unknown<i32>",
        ] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnsupportedTypeParameters(source.into())),
                "{source}"
            );
        }
    }

    #[test]
    fn type_parameters_reject_integer_arguments() {
        for source in [
            "list<1>",
            "map<i32, 2>",
            "struct<1, i32>",
            "func<5 -> i32>",
            "list<P + 1>",
        ] {
            assert!(parse(source).is_err(), "{source}");
        }
    }

    #[test]
    fn unknown_names_remain_types_in_type_parameters() {
        assert_eq!(
            parse("list<P>").unwrap(),
            TypeExpr::Simple(
                "list",
                vec![TypeExprParam::Type(TypeExpr::Simple("P", vec![], false))],
                false,
            )
        );
    }

    #[test]
    fn arithmetic_parameters_remain_unsupported() {
        for source in ["varchar<P + 1>", "varchar<1 + 2>", "decimal<P - 1, S>"] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnsupportedNumericExpression(source.into())),
                "{source}"
            );
        }
        assert!(parse("varchar<(P)>").is_err());
    }

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
                "<list<i32>, any1>>",
                vec![
                    TypeExprParam::Type(parse("list<i32>").unwrap()),
                    TypeExprParam::Type(TypeExpr::TypeVariable(1, false)),
                ],
            ),
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_parameters(ParameterKind::Type).unwrap(),
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
        for source in ["<,i32>", "<i32,>", "<i32,,i64>"] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_parameters(ParameterKind::Type),
                Err(TypeParseError::EmptyParameter),
                "{source}"
            );
        }
    }

    #[test]
    fn integer_parameters_leave_the_separator_for_the_caller() {
        for (source, separator) in [
            ("P, S", TokenKind::Comma),
            ("P>", TokenKind::CloseAngle),
            ("P)", TokenKind::CloseParen),
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_integer_parameter().unwrap(),
                IntegerParameter::Variable("P".into()),
                "{source}"
            );
            assert_eq!(parser.peek(), Some(separator), "{source}");
        }
    }

    #[test]
    fn parse_integer_literals_support_signs_and_the_full_i64_range() {
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
                parser.parse_integer_literal().unwrap(),
                expected,
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
            assert!(parser.parse_integer_parameter().is_err(), "{source}");
        }
    }

    #[test]
    fn overflowing_integer_errors_point_to_the_literal_start() {
        for source in [
            "decimal<9223372036854775808, 2>",
            "decimal<+9223372036854775808, 2>",
            "decimal<-9223372036854775809, 2>",
        ] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnexpectedToken {
                    position: 8,
                    expected: "an i64 integer literal",
                }),
                "{source}"
            );
        }
    }

    #[test]
    fn nested_type_parameters_leave_the_separator_for_the_caller() {
        let mut parser = Parser::new("map<string, func<i32 -> i64>>, boolean").unwrap();
        assert_eq!(
            parser.parse_type_parameter().unwrap(),
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
            let source = format!("u!{name}");
            assert_eq!(
                parse(&source).unwrap(),
                TypeExpr::UserDefined(name, vec![], false),
            );
        }
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
    fn numbered_type_variables_are_case_insensitive() {
        for (source, id, nullable) in [
            ("any0", 0, false),
            ("any1", 1, false),
            ("ANY1", 1, false),
            ("Any1?", 1, true),
            ("any9", 9, false),
        ] {
            assert_eq!(
                parse(source).unwrap(),
                TypeExpr::TypeVariable(id, nullable),
                "{source}"
            );
        }
    }

    #[test]
    fn multi_digit_type_variable_names_are_rejected_everywhere() {
        for (source, position) in [
            ("any10", 0),
            ("ANY12?", 0),
            ("any01", 0),
            ("any4294967296", 0),
            ("list<any10>", 5),
            ("decimal<ANY12, 0>", 8),
            ("func<i32 -> any12>", 12),
            ("u!any10", 2),
            ("u!Box<any12>", 6),
        ] {
            assert_eq!(
                parse(source),
                Err(TypeParseError::UnexpectedToken {
                    position,
                    expected: "a single-digit type variable ID",
                }),
                "{source}",
            );
        }
    }

    #[test]
    fn numbered_type_variables_cannot_have_parameters() {
        for source in [
            "any1<>",
            "any1<i32>",
            "any1?<i32>",
            "ANY1<i32>",
            "Any1?<i32>",
            "any1<i32,>",
            "list<any1<i32>>",
            "func<any1<i32> -> i64>",
            "func<i32 -> any1<>>",
        ] {
            assert!(parse(source).is_err(), "{source}");
        }
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
            ("list<i32,>", TypeParseError::EmptyParameter),
            (
                "unknown<i32,>",
                TypeParseError::UnsupportedTypeParameters("unknown<i32,>".into()),
            ),
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
