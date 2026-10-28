// SPDX-License-Identifier: Apache-2.0

//! Parsing for unevaluated addition in numeric type parameters.

use super::types::IntegerParameter;

/// A syntax error at a byte offset in a numeric expression.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[error("invalid numeric expression at byte {position}")]
pub struct NumericExpressionParseError {
    /// The byte offset relative to the expression being parsed.
    pub position: usize,
}

impl IntegerParameter {
    /// Parse addition of integer literals and parameter names, such as `L + 1`.
    ///
    /// Addition associates to the left unless grouped with parentheses, as in
    /// `P + (S + 1)`. Other operators and calls are not supported yet.
    /// No operation is evaluated.
    pub fn parse(source: &str) -> Result<Self, NumericExpressionParseError> {
        let mut parser = Parser {
            source,
            position: 0,
        };
        let expression = parser.parse_sum()?;
        parser.skip_whitespace();
        if parser.position != source.len() {
            return Err(parser.error());
        }
        Ok(expression)
    }
}

struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl Parser<'_> {
    fn error(&self) -> NumericExpressionParseError {
        NumericExpressionParseError {
            position: self.position,
        }
    }

    fn skip_whitespace(&mut self) {
        let remaining = &self.source[self.position..];
        self.position += remaining.len() - remaining.trim_start().len();
    }

    fn consume(&mut self, character: char) -> bool {
        self.skip_whitespace();
        if self.source[self.position..].starts_with(character) {
            self.position += character.len_utf8();
            true
        } else {
            false
        }
    }

    fn parse_sum(&mut self) -> Result<IntegerParameter, NumericExpressionParseError> {
        let mut expression = self.parse_primary()?;
        while self.consume('+') {
            expression =
                IntegerParameter::Add(Box::new(expression), Box::new(self.parse_primary()?));
        }
        Ok(expression)
    }

    fn parse_primary(&mut self) -> Result<IntegerParameter, NumericExpressionParseError> {
        if self.consume('(') {
            let expression = self.parse_sum()?;
            if !self.consume(')') {
                return Err(self.error());
            }
            return Ok(expression);
        }
        let start = self.position;
        let remaining = &self.source[start..];
        let length = remaining
            .find(|c: char| c.is_whitespace() || matches!(c, '+' | '(' | ')'))
            .unwrap_or(remaining.len());
        self.position += length;
        parse_operand(&self.source[start..self.position], start)
    }
}

fn parse_operand(
    source: &str,
    position: usize,
) -> Result<IntegerParameter, NumericExpressionParseError> {
    let operand = source.trim();
    if let Ok(value) = operand.parse::<i64>() {
        return Ok(IntegerParameter::Literal(value));
    }
    let mut characters = operand.chars();
    if characters
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
    {
        return Ok(IntegerParameter::Variable(operand.to_string()));
    }
    Err(NumericExpressionParseError {
        position: position + source.len() - source.trim_start().len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> IntegerParameter {
        IntegerParameter::parse(source).unwrap()
    }

    #[test]
    fn addition_retains_operands_and_associates_to_the_left() {
        let IntegerParameter::Add(left, right) = parse("P + S + 1") else {
            panic!("expected addition");
        };
        assert_eq!(*left, parse("P + S"));
        assert_eq!(*right, IntegerParameter::Literal(1));
        assert_eq!(parse("P"), IntegerParameter::Variable("P".into()));
    }

    #[test]
    fn parentheses_preserve_grouping() {
        let IntegerParameter::Add(left, right) = parse("P + (S + 1)") else {
            panic!("expected addition");
        };
        assert_eq!(*left, IntegerParameter::Variable("P".into()));
        assert_eq!(*right, parse("S + 1"));
        assert_ne!(parse("P + (S + 1)"), parse("P + S + 1"));
        assert_eq!(parse("((P))"), parse("P"));
    }

    #[test]
    fn display_preserves_parsed_expressions() {
        for source in [
            "P + S + 1",
            "P + (S + 1)",
            "(P + S) + (1 + (T + 2))",
            "L+1",
            " $P + _S ",
            "P + -2",
            "9223372036854775807 + 1",
        ] {
            let expression = parse(source);
            assert_eq!(parse(&expression.to_string()), expression, "{source}");
        }
        assert_eq!(parse("L+1").to_string(), "L + 1");
    }

    #[test]
    fn invalid_or_unsupported_expressions() {
        for source in [
            "",
            "P +",
            "+ P",
            "P ++ 1",
            "P S",
            "P - 1",
            "P * 2",
            "P / 2",
            "min(P, S)",
            "max(P, S)",
            "()",
            "(P + 1",
            "P + 1)",
            "P + ()",
            "(P)(S)",
            "(P +)",
            "P > S",
            "P ? S : 1",
            "9223372036854775808",
            "é + 1",
        ] {
            assert!(IntegerParameter::parse(source).is_err(), "{source}");
        }
        assert_eq!(IntegerParameter::parse("P +").unwrap_err().position, 3);
    }
}
