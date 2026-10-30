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

use super::return_type::{IntegerAssignment, ReturnTypeExpr};
use super::type_ast::{TypeExpr, TypeExprParam, TypeParseError};
use super::type_tokenizer::{Token, TokenKind, tokenize};
use super::types::{BinaryOp, IntegerParameter};

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

/// Parse a standalone numeric expression for tests, rejecting trailing tokens.
#[cfg(test)]
pub(super) fn parse_integer_parameter(source: &str) -> Result<IntegerParameter, TypeParseError> {
    let mut parser = Parser::new(source)?;
    let expression = parser.parse_integer_expression()?;
    if parser.peek().is_some() {
        return Err(parser.error("end of numeric expression"));
    }
    Ok(expression)
}

/// Parse ordered numeric assignments and a final type, or just a type.
/// For `P = min(L, 38)\ndecimal<P, 0>`, retain the assignment followed by `decimal<P, 0>`.
pub(super) fn parse_return_type(source: &str) -> Result<ReturnTypeExpr<'_>, TypeParseError> {
    let mut parser = Parser::new(source)?;
    let mut assignments = Vec::new();
    while parser.peek().is_some_and(TokenKind::is_name)
        && parser.peek_next() == Some(TokenKind::Equals)
    {
        let name = parser.expect_name()?.text.to_string();
        parser.expect(TokenKind::Equals)?;
        let value = parser.parse_integer_expression()?;
        let previous = parser.tokens[parser.index - 1];
        let end = previous.position + previous.text.len();
        let next = parser
            .tokens
            .get(parser.index)
            .map_or(source.len(), |token| token.position);
        if !source[end..next].contains('\n') {
            return Err(parser.error("a newline after an assignment"));
        }
        assignments.push(IntegerAssignment { name, value });
    }
    let return_type = parser.parse_type()?;
    if parser.peek().is_some() {
        return Err(parser.error("end of return declaration"));
    }
    Ok(ReturnTypeExpr {
        assignments,
        return_type,
    })
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

    fn peek_next(&self) -> Option<TokenKind> {
        self.tokens.get(self.index + 1).map(|token| token.kind)
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
    /// Variables do not take parameters. Uppercase prefixes and out-of-range IDs
    /// remain named types.
    fn parse_type_variable(&mut self) -> Result<TypeExpr<'a>, TypeParseError> {
        let name = self.expect(TokenKind::AnyVar)?;
        let nullable = self.consume(TokenKind::QuestionMark);
        let Some(id) = parse_type_variable_id(name.text) else {
            let parameters = self.parse_optional_type_parameters()?;
            return Ok(TypeExpr::Simple(name.text, parameters, nullable));
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

    /// Parse a type or numeric parameter, leaving its separator unconsumed.
    ///
    /// For `decimal<10, 2>`, the first call consumes only `10`, not the comma.
    /// For `decimal<P + 1, S>`, it consumes the whole `P + 1` expression.
    fn parse_parameter(&mut self) -> Result<TypeExprParam<'a>, TypeParseError> {
        match self.peek() {
            Some(
                TokenKind::Integer | TokenKind::Plus | TokenKind::Minus | TokenKind::OpenParen,
            ) => self.parse_numeric_parameter(),
            Some(kind) if kind.is_name() && self.name_begins_numeric_expression() => {
                self.parse_numeric_parameter()
            }
            Some(TokenKind::Comma | TokenKind::CloseAngle | TokenKind::CloseParen) | None => {
                Err(TypeParseError::EmptyParameter)
            }
            _ => Ok(TypeExprParam::Type(self.parse_type()?)),
        }
    }

    /// Distinguish a numeric expression starting with a name from a type name.
    /// For `L > 10`, `>` starts a comparison; in `varchar<L>`, it closes the type.
    fn name_begins_numeric_expression(&self) -> bool {
        match self.peek_next() {
            Some(TokenKind::Plus | TokenKind::Minus | TokenKind::OpenParen) => true,
            Some(TokenKind::CloseAngle | TokenKind::QuestionMark) => self
                .tokens
                .get(self.index + 2)
                .is_some_and(|token| Self::starts_integer_operand(token.kind)),
            _ => false,
        }
    }

    /// Preserve plain literals separately so built-in range validation still applies.
    /// For example, `(10)` becomes an integer parameter, while `P + 1` stays an expression.
    fn parse_numeric_parameter(&mut self) -> Result<TypeExprParam<'a>, TypeParseError> {
        match self.parse_integer_expression()? {
            IntegerParameter::Literal(value) => Ok(TypeExprParam::Integer(value)),
            expression => Ok(TypeExprParam::Expression(expression)),
        }
    }

    /// Parse a right-associative ternary, with comparison and arithmetic binding tighter.
    /// For `P > 38 ? 38 : P + 1`, the condition is `P > 38` and the else branch is `P + 1`.
    fn parse_integer_expression(&mut self) -> Result<IntegerParameter, TypeParseError> {
        let condition = self.parse_integer_comparison()?;
        if !self.consume(TokenKind::QuestionMark) {
            return Ok(condition);
        }
        let then_value = self.parse_integer_expression()?;
        self.expect(TokenKind::Colon)?;
        let else_value = self.parse_integer_expression()?;
        Ok(IntegerParameter::Conditional {
            condition: Box::new(condition),
            then_value: Box::new(then_value),
            else_value: Box::new(else_value),
        })
    }

    /// A `>` followed by an operand is a comparison; otherwise it closes a type.
    /// For `P + 1 > S`, compare `P + 1` with `S`; for `varchar<P + 1>`, leave `>` unconsumed.
    fn parse_integer_comparison(&mut self) -> Result<IntegerParameter, TypeParseError> {
        let mut expression = self.parse_integer_sum()?;
        while self.peek() == Some(TokenKind::CloseAngle)
            && self.peek_next().is_some_and(Self::starts_integer_operand)
        {
            self.consume(TokenKind::CloseAngle);
            let right = self.parse_integer_sum()?;
            expression = IntegerParameter::Binary {
                op: BinaryOp::GreaterThan,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    fn starts_integer_operand(kind: TokenKind) -> bool {
        kind.is_name()
            || matches!(
                kind,
                TokenKind::Integer | TokenKind::Plus | TokenKind::Minus | TokenKind::OpenParen
            )
    }

    /// Parse left-associative addition and subtraction, with grouped operands.
    /// For example, `P - S + 1` groups as `(P - S) + 1`, not `P - (S + 1)`.
    fn parse_integer_sum(&mut self) -> Result<IntegerParameter, TypeParseError> {
        let mut expression = if self.peek() == Some(TokenKind::Plus) {
            IntegerParameter::Literal(self.parse_integer_literal()?)
        } else {
            self.parse_integer_operand()?
        };
        loop {
            let op = match self.peek() {
                Some(TokenKind::Plus) => {
                    self.consume(TokenKind::Plus);
                    BinaryOp::Add
                }
                Some(TokenKind::Minus) => {
                    self.consume(TokenKind::Minus);
                    BinaryOp::Subtract
                }
                _ => break,
            };
            let right = self.parse_integer_operand()?;
            expression = IntegerParameter::Binary {
                op,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Ok(expression)
    }

    /// Parse one integer, parameter name, call, or parenthesized arithmetic expression.
    /// For `(P + 1) + S`, consume `(P + 1)` and leave the following `+` for the caller.
    /// For `max(P, S) + 1`, consume the whole call, including its arguments and parentheses.
    fn parse_integer_operand(&mut self) -> Result<IntegerParameter, TypeParseError> {
        match self.peek() {
            Some(TokenKind::Integer | TokenKind::Minus) => {
                Ok(IntegerParameter::Literal(self.parse_integer_literal()?))
            }
            Some(TokenKind::OpenParen) => {
                self.expect(TokenKind::OpenParen)?;
                let expression = self.parse_integer_expression()?;
                self.expect(TokenKind::CloseParen)?;
                Ok(expression)
            }
            Some(kind) if kind.is_name() => {
                let name = self.expect_name()?;
                if self.consume(TokenKind::OpenParen) {
                    let mut arguments = Vec::new();
                    if !self.consume(TokenKind::CloseParen) {
                        loop {
                            arguments.push(self.parse_integer_expression()?);
                            if self.consume(TokenKind::CloseParen) {
                                break;
                            }
                            self.expect(TokenKind::Comma)?;
                        }
                    }
                    Ok(IntegerParameter::Call {
                        name: name.text.to_string(),
                        arguments,
                    })
                } else {
                    Ok(IntegerParameter::Variable(name.text.to_string()))
                }
            }
            _ => Err(self.error("an integer literal, parameter name, or '('")),
        }
    }

    /// Combine a sign with its digits, preserving the full `i64` range.
    /// For example, `-9223372036854775808` is parsed as one value, including its sign.
    fn parse_integer_literal(&mut self) -> Result<i64, TypeParseError> {
        let start = self.tokens[self.index].position;
        if !self.consume(TokenKind::Plus) {
            self.consume(TokenKind::Minus);
        }
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

/// Recognize a case-sensitive `any` prefix followed by a `u32` variable ID.
fn parse_type_variable_id(name: &str) -> Option<u32> {
    name.strip_prefix("any")?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_integer(source: &str) -> IntegerParameter {
        parse_integer_parameter(source).unwrap()
    }

    #[test]
    fn addition_retains_operands_and_associates_to_the_left() {
        let IntegerParameter::Binary {
            op: BinaryOp::Add,
            left,
            right,
        } = parse_integer("P + S + 1")
        else {
            panic!("expected addition");
        };
        assert_eq!(*left, parse_integer("P + S"));
        assert_eq!(*right, IntegerParameter::Literal(1));
        assert_eq!(parse_integer("P"), IntegerParameter::Variable("P".into()));
    }

    #[test]
    fn subtraction_shares_addition_precedence_and_preserves_grouping() {
        let IntegerParameter::Binary {
            op: BinaryOp::Add,
            left,
            right,
        } = parse_integer("P - S + 1")
        else {
            panic!("expected addition");
        };
        assert_eq!(*left, parse_integer("P - S"));
        assert_eq!(*right, IntegerParameter::Literal(1));
        let IntegerParameter::Binary {
            op: BinaryOp::Subtract,
            left,
            right,
        } = parse_integer("P - S - 1")
        else {
            panic!("expected subtraction");
        };
        assert_eq!(*left, parse_integer("P - S"));
        assert_eq!(*right, IntegerParameter::Literal(1));
        for source in ["P - (S + 1)", "P + (S - 1)", "P - (S - 1)", "P - -1"] {
            let expression = parse_integer(source);
            assert_eq!(
                parse_integer(&expression.to_string()),
                expression,
                "{source}"
            );
            assert!(!expression.is_concrete());
        }
        for source in ["P -", "P -- S", "P - ()", "P - - 1"] {
            assert!(parse_integer_parameter(source).is_err(), "{source}");
        }
    }

    #[test]
    fn parentheses_preserve_grouping() {
        let IntegerParameter::Binary {
            op: BinaryOp::Add,
            left,
            right,
        } = parse_integer("P + (S + 1)")
        else {
            panic!("expected addition");
        };
        assert_eq!(*left, IntegerParameter::Variable("P".into()));
        assert_eq!(*right, parse_integer("S + 1"));
        assert_ne!(parse_integer("P + (S + 1)"), parse_integer("P + S + 1"));
        assert_eq!(parse_integer("((P))"), parse_integer("P"));
    }

    #[test]
    fn multiline_returns_retain_ordered_assignments_and_the_final_type() {
        let source = "digits = P - S + 1\nprecision = min(digits, 38)\ndecimal?<precision, 0>\n";
        let parsed = parse_return_type(source).unwrap();
        assert_eq!(parsed.assignments.len(), 2);
        assert_eq!(parsed.assignments[0].name, "digits");
        assert_eq!(parsed.assignments[0].value, parse_integer("P - S + 1"));
        assert_eq!(parsed.assignments[1].name, "precision");
        assert_eq!(
            parsed.assignments[1].value,
            parse_integer("min(digits, 38)")
        );
        assert_eq!(parsed.return_type, parse("decimal?<precision, 0>").unwrap());
        let declaration = super::super::ReturnType::try_from(parsed).unwrap();
        let reparsed = super::super::ReturnType::parse(&declaration.to_string()).unwrap();
        assert_eq!(declaration, reparsed);
        assert!(parse_return_type("i32\n").unwrap().assignments.is_empty());
    }

    #[test]
    fn malformed_multiline_returns_are_rejected() {
        for source in [
            "P = 1 i32",
            "P = 1\n",
            "P =\ni32",
            "P = 1\nQ =\ni32",
            "P = 1\ni32 garbage",
            "P = 1;\ni32",
            "P = 1\nQ\ni32",
        ] {
            assert!(parse_return_type(source).is_err(), "{source}");
        }
    }

    #[test]
    fn calls_retain_names_and_ordered_arguments_without_evaluation() {
        let IntegerParameter::Call { name, arguments } = parse_integer("max(S1, S2 + 1)") else {
            panic!("expected call");
        };
        assert_eq!(name, "max");
        assert_eq!(
            arguments,
            vec![parse_integer("S1"), parse_integer("S2 + 1")]
        );
        for source in [
            "min(P, 38)",
            "max(S1, S2)",
            "integer_parameter(precision)",
            "min(max(P1 - S1, P2 - S2) + 1, 38)",
            "custom()",
        ] {
            let expression = parse_integer(source);
            assert_eq!(
                parse_integer(&expression.to_string()),
                expression,
                "{source}"
            );
            assert!(!expression.is_concrete());
        }
        for source in ["min(", "min(P,)", "min(,P)", "min(P S)", "min(P,,S)"] {
            assert!(parse_integer_parameter(source).is_err(), "{source}");
        }
    }

    #[test]
    fn conditionals_retain_comparisons_and_both_branches() {
        let IntegerParameter::Conditional {
            condition,
            then_value,
            else_value,
        } = parse_integer("init_prec > 38 ? scale_after_borrow : init_scale")
        else {
            panic!("expected conditional");
        };
        assert_eq!(
            *condition,
            IntegerParameter::Binary {
                op: BinaryOp::GreaterThan,
                left: Box::new(parse_integer("init_prec")),
                right: Box::new(IntegerParameter::Literal(38)),
            }
        );
        assert_eq!(*then_value, parse_integer("scale_after_borrow"));
        assert_eq!(*else_value, parse_integer("init_scale"));
        let IntegerParameter::Conditional {
            condition,
            then_value,
            else_value,
        } = parse_integer("P + 1 > S - 1 ? 38 : P > 0 ? P : 1")
        else {
            panic!("expected conditional");
        };
        assert_eq!(*condition, parse_integer("P + 1 > S - 1"));
        assert_eq!(*then_value, IntegerParameter::Literal(38));
        assert_eq!(*else_value, parse_integer("P > 0 ? P : 1"));
    }

    #[test]
    fn conditional_display_preserves_precedence_and_grouping() {
        for source in [
            "P > S",
            "P > +1",
            "P > -1",
            "P > S > 1",
            "P > (S > 1)",
            "(P > 38 ? 38 : P) + 1",
            "P - (S > 38 ? 38 : S)",
            "(P > 1) + (S > 1)",
            "(P > 1 ? 1 : 0) > S",
            "(P ? S : 1) ? 2 : 3",
            "P ? S ? 1 : 2 : 3",
            "P ? 1 : S ? 2 : 3",
            "min(P > 38 ? 38 : P, 10)",
        ] {
            let expression = parse_integer(source);
            assert_eq!(
                parse_integer(&expression.to_string()),
                expression,
                "{source}"
            );
            assert!(!expression.is_concrete());
        }
    }

    #[test]
    fn conditionals_do_not_consume_type_delimiters_or_nullability() {
        for source in [
            "decimal<P > 38 ? 38 : P, S>",
            "list<varchar<L > 10 ? 10 : L>>",
            "func<varchar<L + 1> -> decimal<P + 1, S + 1>>",
            "list<any1?>",
            "map<string, any?>",
        ] {
            let type_ = super::super::Type::try_from(parse(source).unwrap()).unwrap();
            assert_eq!(
                super::super::Type::try_from(parse(&type_.to_string()).unwrap()).unwrap(),
                type_,
                "{source}"
            );
        }
        for source in [
            "P ?",
            "P ? S",
            "P ? : S",
            "P ? S :",
            "P >",
            "P > ? S : 1",
            "P : S",
        ] {
            assert!(parse_integer_parameter(source).is_err(), "{source}");
        }
    }

    #[test]
    fn invalid_or_unsupported_expressions() {
        for source in [
            "",
            "P +",
            "+ P",
            "P ++ 1",
            "P S",
            "P * 2",
            "P / 2",
            "()",
            "(P + 1",
            "P + 1)",
            "P + ()",
            "(P)(S)",
            "(P +)",
            "9223372036854775808",
            "@ + 1",
        ] {
            assert!(parse_integer_parameter(source).is_err(), "{source}");
        }
        assert_eq!(
            parse_integer_parameter("P +"),
            Err(TypeParseError::UnexpectedToken {
                position: 3,
                expected: "an integer literal, parameter name, or '('",
            })
        );
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
    fn integer_expressions_leave_the_separator_for_the_caller() {
        for (source, separator) in [
            ("P + (S + 1), i32", TokenKind::Comma),
            ("P + (S + 1)>", TokenKind::CloseAngle),
            ("P + (S + 1))", TokenKind::CloseParen),
        ] {
            let mut parser = Parser::new(source).unwrap();
            assert_eq!(
                parser.parse_integer_expression().unwrap(),
                parse_integer("P + (S + 1)"),
                "{source}"
            );
            assert_eq!(parser.peek(), Some(separator), "{source}");
        }
    }

    #[test]
    fn grouped_integer_literals_remain_literal_parameters() {
        let mut parser = Parser::new("((10)), S").unwrap();
        assert_eq!(
            parser.parse_parameter().unwrap(),
            TypeExprParam::Integer(10)
        );
        assert_eq!(parser.peek(), Some(TokenKind::Comma));
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
            let source = format!("{name}?<i32>");
            assert_eq!(
                parse(&source).unwrap(),
                TypeExpr::Simple(name, vec![TypeExprParam::Type(parse("i32").unwrap())], true,)
            );
        }
    }

    #[test]
    fn numbered_type_variables_cannot_have_parameters() {
        for source in [
            "any1<>",
            "any1<i32>",
            "any1?<i32>",
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
            ("u!point<i32,>", TypeParseError::EmptyParameter),
            ("ANY1<i32,>", TypeParseError::EmptyParameter),
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
