// SPDX-License-Identifier: Apache-2.0

//! Unevaluated addition expressions used in numeric type parameters.

use std::fmt;

/// An integer expression retained without binding names or evaluating operations.
#[derive(Clone, Debug, PartialEq)]
pub enum NumericExpression {
    /// An integer literal.
    Literal(i64),
    /// A named numeric parameter.
    Parameter(String),
    /// Addition of two integer expressions.
    Add(Box<Self>, Box<Self>),
}

/// A syntax error at a byte offset in a numeric expression.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[error("expected an integer literal or parameter name at byte {position}")]
pub struct NumericExpressionParseError {
    /// The byte offset relative to the expression being parsed.
    pub position: usize,
}

impl NumericExpression {
    /// Parse addition of integer literals and parameter names, such as `L + 1`.
    ///
    /// Addition associates to the left. Other operators, calls, and grouping
    /// parentheses are not supported yet. No operation is evaluated.
    pub fn parse(source: &str) -> Result<Self, NumericExpressionParseError> {
        let mut operands = source.split('+');
        let first = operands.next().unwrap();
        let mut expression = parse_operand(first, 0)?;
        let mut position = first.len() + 1;
        for operand in operands {
            expression = Self::Add(
                Box::new(expression),
                Box::new(parse_operand(operand, position)?),
            );
            position += operand.len() + 1;
        }
        Ok(expression)
    }
}

fn parse_operand(
    source: &str,
    position: usize,
) -> Result<NumericExpression, NumericExpressionParseError> {
    let operand = source.trim();
    if let Ok(value) = operand.parse::<i64>() {
        return Ok(NumericExpression::Literal(value));
    }
    let mut characters = operand.chars();
    if characters
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_' || c == '$')
        && characters.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
    {
        return Ok(NumericExpression::Parameter(operand.to_string()));
    }
    Err(NumericExpressionParseError {
        position: position + source.len() - source.trim_start().len(),
    })
}

impl fmt::Display for NumericExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{value}"),
            Self::Parameter(name) => f.write_str(name),
            Self::Add(left, right) => write!(f, "{left} + {right}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> NumericExpression {
        NumericExpression::parse(source).unwrap()
    }

    #[test]
    fn addition_retains_operands_and_associates_to_the_left() {
        let NumericExpression::Add(left, right) = parse("P + S + 1") else {
            panic!("expected addition");
        };
        assert_eq!(*left, parse("P + S"));
        assert_eq!(*right, NumericExpression::Literal(1));
        assert_eq!(parse("P"), NumericExpression::Parameter("P".into()));
    }

    #[test]
    fn display_preserves_parsed_expressions() {
        for source in [
            "P + S + 1",
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
            "(P + 1)",
            "P > S",
            "P ? S : 1",
            "9223372036854775808",
            "é + 1",
        ] {
            assert!(NumericExpression::parse(source).is_err(), "{source}");
        }
        assert_eq!(NumericExpression::parse("P +").unwrap_err().position, 3);
    }
}
