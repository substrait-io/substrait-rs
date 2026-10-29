// SPDX-License-Identifier: Apache-2.0

//! Parsing for unevaluated addition in numeric type parameters.

use super::type_ast::TypeParseError;
use super::types::IntegerParameter;

impl IntegerParameter {
    /// Parse addition of integer literals and parameter names, such as `L + 1`.
    ///
    /// Addition associates to the left unless grouped with parentheses, as in
    /// `P + (S + 1)`. Other operators and calls are not supported yet.
    /// No operation is evaluated.
    pub fn parse(source: &str) -> Result<Self, TypeParseError> {
        super::type_parser::parse_integer_parameter(source)
    }
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
            "@ + 1",
        ] {
            assert!(IntegerParameter::parse(source).is_err(), "{source}");
        }
        assert_eq!(
            IntegerParameter::parse("P +"),
            Err(TypeParseError::UnexpectedToken {
                position: 3,
                expected: "an integer literal, parameter name, or '('",
            })
        );
    }
}
