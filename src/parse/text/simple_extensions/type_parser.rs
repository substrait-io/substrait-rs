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
    let mut parser = Parser {
        source,
        tokens: tokenize(source)?,
        index: 0,
    };
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

    /// Parse a name with optional `u!`, nullability, and parameters.
    ///
    /// For example, `u!point?<i32>` becomes a nullable user-defined type;
    /// `any1?` becomes a nullable type variable. Unknown names remain in the AST
    /// for semantic validation rather than being rejected here. Numbered variables
    /// retain the existing case-sensitive `any` prefix behavior even though the
    /// tokenizer recognizes keyword spellings case-insensitively.
    fn parse_type(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let user_defined = self.consume(TokenKind::UserDefined);
        let name = self
            .tokens
            .get(self.index)
            .copied()
            .filter(|token| token.kind.is_name())
            .ok_or_else(|| self.error("a type name"))?;
        self.index += 1;
        let nullable = self.consume(TokenKind::QuestionMark);

        if self.consume(TokenKind::OpenAngle) {
            if !user_defined && name.kind == TokenKind::Func {
                return self.parse_function_type(nullable);
            }
            let parameters = self.parse_type_parameters()?;
            return Ok(if user_defined {
                TypeExpr::UserDefined(name.text, parameters, nullable)
            } else {
                TypeExpr::Simple(name.text, parameters, nullable)
            });
        }

        if user_defined {
            return Ok(TypeExpr::UserDefined(name.text, Vec::new(), nullable));
        }
        if name.kind == TokenKind::AnyVar
            && let Some(id) = name
                .text
                .strip_prefix("any")
                .and_then(|suffix| suffix.parse().ok())
        {
            return Ok(TypeExpr::TypeVariable(id, nullable));
        }
        Ok(TypeExpr::Simple(name.text, Vec::new(), nullable))
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

    /// Parse the body after `func<`, including the closing `>`.
    ///
    /// Read parameters, an arrow, and a return type. Multiple parameters require
    /// parentheses, as in `func<(i32, i64) -> boolean>`. A single parameter may omit
    /// them, and nested function types use the same recursive rules.
    fn parse_function_type(&mut self, nullable: bool) -> Result<TypeExpr<'a>, TypeParseError> {
        let grouped = self.consume(TokenKind::OpenParen);
        if grouped && self.peek() == Some(TokenKind::CloseParen) {
            return Err(self.error("a function parameter"));
        }

        let mut parameters = vec![self.parse_parameter()?];
        while grouped && self.consume(TokenKind::Comma) {
            parameters.push(self.parse_parameter()?);
        }
        if grouped {
            self.expect(TokenKind::CloseParen)?;
        }
        self.expect(TokenKind::Arrow)?;
        let return_type = self.parse_type()?;
        self.expect(TokenKind::CloseAngle)?;
        Ok(TypeExpr::Func(parameters, Box::new(return_type), nullable))
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

#[cfg(test)]
mod tests {
    use super::*;

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
