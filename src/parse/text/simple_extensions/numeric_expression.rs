// SPDX-License-Identifier: Apache-2.0

//! Unevaluated arithmetic expressions used in numeric type parameters.

use std::fmt;

/// An arithmetic expression retained without binding names or evaluating calls.
#[derive(Clone, Debug, PartialEq)]
pub enum NumericExpression {
    /// An integer literal.
    Literal(i64),
    /// A named numeric parameter.
    Parameter(String),
    /// An arithmetic operation.
    Binary {
        /// The left operand.
        left: Box<Self>,
        /// The arithmetic operator.
        operator: ArithmeticOperator,
        /// The right operand.
        right: Box<Self>,
    },
    /// The minimum of two expressions, such as `min(S1, S2)`.
    Min(Box<Self>, Box<Self>),
    /// The maximum of two expressions, such as `max(S1, S2)`.
    Max(Box<Self>, Box<Self>),
}

/// An operator in a numeric parameter expression.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ArithmeticOperator {
    /// Addition.
    Add,
    /// Subtraction.
    Subtract,
    /// Multiplication.
    Multiply,
    /// Division.
    Divide,
}

/// A syntax error at a byte offset in a numeric expression.
#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[error("expected {expected} at byte {position}")]
pub struct NumericExpressionParseError {
    /// The byte offset relative to the expression being parsed.
    pub position: usize,
    /// The syntax expected at that offset.
    pub expected: &'static str,
}

impl NumericExpression {
    /// Parse arithmetic, parentheses, names, integer literals, and two-argument
    /// `min` and `max` calls. Function names are case-insensitive.
    ///
    /// For example, `P + max(S1, S2) * 2` retains the multiplication inside
    /// the right operand of the addition. No operation is evaluated.
    pub fn parse(source: &str) -> Result<Self, NumericExpressionParseError> {
        let mut parser = Parser {
            source,
            position: 0,
        };
        let expression = parser.parse_sum()?;
        parser.skip_whitespace();
        if parser.position != source.len() {
            return Err(parser.error("end of expression"));
        }
        Ok(expression)
    }
}

impl fmt::Display for NumericExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{value}"),
            Self::Parameter(name) => f.write_str(name),
            Self::Binary {
                left,
                operator,
                right,
            } => {
                let operator = match operator {
                    ArithmeticOperator::Add => '+',
                    ArithmeticOperator::Subtract => '-',
                    ArithmeticOperator::Multiply => '*',
                    ArithmeticOperator::Divide => '/',
                };
                write!(f, "({left} {operator} {right})")
            }
            Self::Min(left, right) => write!(f, "min({left}, {right})"),
            Self::Max(left, right) => write!(f, "max({left}, {right})"),
        }
    }
}

struct Parser<'a> {
    source: &'a str,
    position: usize,
}

impl Parser<'_> {
    fn error(&self, expected: &'static str) -> NumericExpressionParseError {
        NumericExpressionParseError {
            position: self.position,
            expected,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.source[self.position..].chars().next() {
            if !character.is_whitespace() {
                break;
            }
            self.position += character.len_utf8();
        }
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

    fn parse_sum(&mut self) -> Result<NumericExpression, NumericExpressionParseError> {
        let mut left = self.parse_product()?;
        loop {
            let operator = if self.consume('+') {
                ArithmeticOperator::Add
            } else if self.consume('-') {
                ArithmeticOperator::Subtract
            } else {
                return Ok(left);
            };
            let right = self.parse_product()?;
            left = NumericExpression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
    }

    fn parse_product(&mut self) -> Result<NumericExpression, NumericExpressionParseError> {
        let mut left = self.parse_primary()?;
        loop {
            let operator = if self.consume('*') {
                ArithmeticOperator::Multiply
            } else if self.consume('/') {
                ArithmeticOperator::Divide
            } else {
                return Ok(left);
            };
            let right = self.parse_primary()?;
            left = NumericExpression::Binary {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }
    }

    fn parse_primary(&mut self) -> Result<NumericExpression, NumericExpressionParseError> {
        if self.consume('(') {
            let expression = self.parse_sum()?;
            if !self.consume(')') {
                return Err(self.error("')'"));
            }
            return Ok(expression);
        }

        let start = self.position;
        let bytes = self.source.as_bytes();
        if bytes
            .get(start)
            .is_some_and(|byte| byte.is_ascii_digit() || *byte == b'-')
        {
            if bytes.get(self.position) == Some(&b'-') {
                self.position += 1;
            }
            while bytes.get(self.position).is_some_and(u8::is_ascii_digit) {
                self.position += 1;
            }
            return self.source[start..self.position]
                .parse::<i64>()
                .map(NumericExpression::Literal)
                .map_err(|_| self.error("an i64 integer literal"));
        }

        if !bytes
            .get(start)
            .is_some_and(|byte| byte.is_ascii_alphabetic() || matches!(byte, b'_' | b'$'))
        {
            return Err(self.error("an integer, parameter name, function call, or '('"));
        }
        self.position += 1;
        while bytes
            .get(self.position)
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'$'))
        {
            self.position += 1;
        }
        let name = self.source[start..self.position].to_string();
        if !self.consume('(') {
            return Ok(NumericExpression::Parameter(name));
        }

        let is_min = name.eq_ignore_ascii_case("min");
        if !is_min && !name.eq_ignore_ascii_case("max") {
            return Err(NumericExpressionParseError {
                position: start,
                expected: "'min' or 'max'",
            });
        }
        let left = Box::new(self.parse_sum()?);
        if !self.consume(',') {
            return Err(self.error("',' followed by a second argument"));
        }
        let right = Box::new(self.parse_sum()?);
        if !self.consume(')') {
            return Err(self.error("')' after the second argument"));
        }
        Ok(if is_min {
            NumericExpression::Min(left, right)
        } else {
            NumericExpression::Max(left, right)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(source: &str) -> NumericExpression {
        NumericExpression::parse(source).unwrap()
    }

    #[test]
    fn precedence_and_associativity() {
        let cases = [
            ("P + S * 2", "(P + (S * 2))"),
            ("P - S - 1", "((P - S) - 1)"),
            ("P / S * 2", "((P / S) * 2)"),
            ("(P + S) * 2", "((P + S) * 2)"),
            ("P-2", "(P - 2)"),
            ("P * -2", "(P * -2)"),
        ];
        for (source, expected) in cases {
            assert_eq!(parse(source).to_string(), expected, "{source}");
        }
    }

    #[test]
    fn calls_and_literals_are_retained_without_evaluation() {
        let expression = parse("max(S1, min(S2 + 1, 38))");
        let NumericExpression::Max(left, right) = &expression else {
            panic!("expected max");
        };
        assert_eq!(**left, NumericExpression::Parameter("S1".into()));
        let NumericExpression::Min(left, right) = &**right else {
            panic!("expected nested min");
        };
        assert_eq!(**left, parse("S2 + 1"));
        assert_eq!(**right, NumericExpression::Literal(38));
        assert_eq!(
            parse("-9223372036854775808"),
            NumericExpression::Literal(i64::MIN)
        );
        assert_eq!(parse("1 / 0").to_string(), "(1 / 0)");
    }

    #[test]
    fn function_names_are_case_insensitive() {
        for source in ["MIN(P, S)", "Min(P, S)", "mIn(P, S)"] {
            assert_eq!(parse(source), parse("min(P, S)"));
        }
        for source in ["MAX(P, S)", "Max(P, S)", "mAx(P, S)"] {
            assert_eq!(parse(source), parse("max(P, S)"));
        }
    }

    #[test]
    fn calls_require_known_names_and_two_arguments() {
        for source in [
            "custom()",
            "custom(P, S)",
            "minimum(P, S)",
            "min()",
            "min(P)",
            "min(P, S, 1)",
            "max()",
            "max(P)",
            "max(P, S, 1)",
        ] {
            assert!(NumericExpression::parse(source).is_err(), "{source}");
        }
        let error = NumericExpression::parse("custom(P, S)").unwrap_err();
        assert_eq!(error.position, 0);
        assert_eq!(error.expected, "'min' or 'max'");
    }

    #[test]
    fn display_preserves_expression_structure() {
        for source in [
            "P + S * 2",
            "P - (S - 1)",
            "max(P, min(S, 38))",
            " $P + _S ",
        ] {
            let expression = parse(source);
            assert_eq!(parse(&expression.to_string()), expression);
        }
    }

    #[test]
    fn invalid_expressions() {
        for source in [
            "",
            "P +",
            "max(P,)",
            "max(P",
            "(P + 1",
            "P S",
            "P > S",
            "P ? S : 1",
            "9223372036854775808",
            "é + 1",
            "-P",
        ] {
            assert!(NumericExpression::parse(source).is_err(), "{source}");
        }
        let error = NumericExpression::parse("P +").unwrap_err();
        assert_eq!(error.position, 3);
    }
}
