// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
//!
//! This module provides syntactic parsing of Substrait type strings and type derivation
//! expressions into an AST. It does NOT validate that type names are valid builtins or
//! that extension types exist - semantic validation occurs later when converting to
//! [`ConcreteType`](super::types::ConcreteType).
//!
//! Type derivation expressions are multiline programs that compute return types based on
//! input parameter values:
//!
//! ```yaml
//! return: |-
//!   init_scale = max(S1,S2)
//!   prec = min(P1 + P2, 38)
//!   DECIMAL<prec, init_scale>
//! ```
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
///
/// This represents the syntactic structure only - type names are not validated.
/// Convert to [`ConcreteType`](super::types::ConcreteType) for semantic validation.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr<'a> {
    /// A type without the `u!` prefix (expected to be a builtin like `i32`, `List`, etc.)
    ///
    /// Contains: (name, parameters, nullable)
    Simple(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// A user-defined extension type with the `u!` prefix (e.g., `u!geometry`)
    ///
    /// Contains: (name without `u!` prefix, parameters, nullable)
    UserDefined(&'a str, Vec<TypeExprParam<'a>>, bool),
    /// Type variable (e.g., `any1`, `any2`)
    ///
    /// Contains: (variable id, nullable)
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

/// A parsed type derivation expression (borrows from input string).
///
/// Type derivation expressions are multiline programs that compute return types
/// based on input parameter values. The last line is the result type expression.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct DerivationExpr<'a> {
    /// Variable assignments in order of execution
    pub assignments: Vec<AssignmentExpr<'a>>,
    /// Final type expression (e.g., `DECIMAL<prec, scale>`)
    pub result_type: TypeExpr<'a>,
}

/// A variable assignment: `name = expr`
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AssignmentExpr<'a> {
    /// Variable name
    pub name: &'a str,
    /// Value expression
    pub value: Expr<'a>,
}

/// An arithmetic/boolean expression in a type derivation program.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expr<'a> {
    /// Integer literal
    Integer(i64),
    /// Identifier (variable or parameter reference)
    Ident(&'a str),
    /// Binary operation
    BinaryOp {
        /// Left operand
        left: Box<Expr<'a>>,
        /// Operator
        op: BinaryOp,
        /// Right operand
        right: Box<Expr<'a>>,
    },
    /// Function call (e.g., `max(a, b)`)
    FunctionCall {
        /// Function name
        name: &'a str,
        /// Function arguments
        args: Vec<Expr<'a>>,
    },
    /// Ternary conditional (`cond ? then : else`)
    Ternary {
        /// Condition expression
        condition: Box<Expr<'a>>,
        /// Value when condition is true
        then_expr: Box<Expr<'a>>,
        /// Value when condition is false
        else_expr: Box<Expr<'a>>,
    },
    /// Parenthesized expression
    Paren(Box<Expr<'a>>),
    /// Unary NOT
    Not(Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    pub(crate) fn binary_op(left: Expr<'a>, op: BinaryOp, right: Expr<'a>) -> Self {
        Expr::BinaryOp {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub(crate) fn ternary(condition: Expr<'a>, then_expr: Expr<'a>, else_expr: Expr<'a>) -> Self {
        Expr::Ternary {
            condition: Box::new(condition),
            then_expr: Box::new(then_expr),
            else_expr: Box::new(else_expr),
        }
    }

    pub(crate) fn paren(inner: Expr<'a>) -> Self {
        Expr::Paren(Box::new(inner))
    }

    pub(crate) fn not(inner: Expr<'a>) -> Self {
        Expr::Not(Box::new(inner))
    }
}

/// Binary operators used in type derivation expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
    /// Addition (`+`)
    Add,
    /// Subtraction (`-`)
    Sub,
    /// Multiplication (`*`)
    Mul,
    /// Division (`/`)
    Div,
    /// Less than (`<`)
    Lt,
    /// Greater than (`>`)
    Gt,
    /// Less than or equal (`<=`)
    Le,
    /// Greater than or equal (`>=`)
    Ge,
    /// Equality (`==`)
    Eq,
    /// Inequality (`!=`)
    Ne,
    /// Logical AND (`&&` or `AND`)
    And,
    /// Logical OR (`||` or `OR`)
    Or,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("missing closing angle bracket in parameter list: {0}")]
    ExpectedClosingAngleBracket(String),
    #[error("Type variation syntax is not supported: {0}")]
    UnsupportedVariation(String),
}

/// Errors that can occur while parsing a type derivation expression.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DerivationParseError {
    /// The input string was empty
    #[error("derivation expression is empty")]
    Empty,
    /// Invalid assignment syntax (missing `=` or empty left/right side)
    #[error("invalid assignment syntax: {0}")]
    InvalidAssignment(String),
    /// Encountered an unexpected token
    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
    /// Unclosed parenthesis
    #[error("unclosed parenthesis")]
    UnclosedParen,
    /// Expected an expression but found something else
    #[error("expected expression, found: {0}")]
    ExpectedExpression(String),
    /// Invalid ternary expression (missing `:` after `?`)
    #[error("invalid ternary expression: {0}")]
    InvalidTernary(String),
    /// Error parsing the result type expression
    #[error("type parse error: {0}")]
    TypeParseError(#[from] TypeParseError),
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
                on_ext(name);
                for p in params {
                    if let TypeExprParam::Type(t) = p {
                        t.visit_references(on_ext);
                    }
                }
            }
            TypeExpr::Simple(_name, params, _) => {
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

impl<'a> DerivationExpr<'a> {
    /// Parse a type derivation expression from a multiline string.
    pub fn parse(input: &'a str) -> Result<Self, DerivationParseError> {
        let lines: Vec<&str> = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        // The last line is the result type; all preceding lines are variable assignments.
        let (result_line, assignment_lines) =
            lines.split_last().ok_or(DerivationParseError::Empty)?;

        let assignments = assignment_lines
            .iter()
            .map(|line| parse_assignment(line))
            .collect::<Result<Vec<_>, _>>()?;

        let result_type = TypeExpr::parse(result_line)?;

        Ok(DerivationExpr {
            assignments,
            result_type,
        })
    }
}

fn parse_assignment<'a>(line: &'a str) -> Result<AssignmentExpr<'a>, DerivationParseError> {
    let mut chars = line.char_indices().peekable();
    let mut eq_pos = None;

    while let Some((i, c)) = chars.next() {
        if c == '=' {
            let prev_char = if i > 0 { line.chars().nth(i - 1) } else { None };
            let next_char = chars.peek().map(|(_, c)| *c);

            if next_char != Some('=')
                && prev_char != Some('!')
                && prev_char != Some('<')
                && prev_char != Some('>')
            {
                eq_pos = Some(i);
                break;
            }
        }
    }

    let eq_pos = eq_pos.ok_or_else(|| DerivationParseError::InvalidAssignment(line.to_string()))?;

    let name = line[..eq_pos].trim();
    let value_str = line[eq_pos + 1..].trim();

    if name.is_empty() || value_str.is_empty() {
        return Err(DerivationParseError::InvalidAssignment(line.to_string()));
    }

    let value = parse_expr(value_str)?;
    Ok(AssignmentExpr { name, value })
}

fn parse_expr<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(DerivationParseError::ExpectedExpression(input.to_string()));
    }
    parse_ternary(input)
}

fn parse_ternary<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    if let Some((cond_str, rest)) = split_at_operator(input, "?") {
        if let Some((then_str, else_str)) = split_at_operator(rest, ":") {
            let condition = parse_or(cond_str.trim())?;
            let then_expr = parse_or(then_str.trim())?;
            let else_expr = parse_ternary(else_str.trim())?;
            return Ok(Expr::ternary(condition, then_expr, else_expr));
        } else {
            return Err(DerivationParseError::InvalidTernary(input.to_string()));
        }
    }
    parse_or(input)
}

fn parse_or<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    for (op_str, op) in &[("||", BinaryOp::Or), ("OR", BinaryOp::Or)] {
        if let Some((left, right)) = split_at_operator_rightmost(input, op_str) {
            let left_expr = parse_or(left.trim())?;
            let right_expr = parse_and(right.trim())?;
            return Ok(Expr::binary_op(left_expr, *op, right_expr));
        }
    }
    parse_and(input)
}

fn parse_and<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    for (op_str, op) in &[("&&", BinaryOp::And), ("AND", BinaryOp::And)] {
        if let Some((left, right)) = split_at_operator_rightmost(input, op_str) {
            let left_expr = parse_and(left.trim())?;
            let right_expr = parse_comparison(right.trim())?;
            return Ok(Expr::binary_op(left_expr, *op, right_expr));
        }
    }
    parse_comparison(input)
}

fn parse_comparison<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    let operators = &[
        ("<=", BinaryOp::Le),
        (">=", BinaryOp::Ge),
        ("!=", BinaryOp::Ne),
        ("==", BinaryOp::Eq),
        ("<", BinaryOp::Lt),
        (">", BinaryOp::Gt),
    ];
    for (op_str, op) in operators {
        if let Some((left, right)) = split_at_operator_rightmost(input, op_str) {
            let left_expr = parse_comparison(left.trim())?;
            let right_expr = parse_additive(right.trim())?;
            return Ok(Expr::binary_op(left_expr, *op, right_expr));
        }
    }
    parse_additive(input)
}

fn parse_additive<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    for (op_str, op) in &[("+", BinaryOp::Add), ("-", BinaryOp::Sub)] {
        if let Some((left, right)) = split_at_operator_rightmost(input, op_str) {
            let left_expr = parse_additive(left.trim())?;
            let right_expr = parse_multiplicative(right.trim())?;
            return Ok(Expr::binary_op(left_expr, *op, right_expr));
        }
    }
    parse_multiplicative(input)
}

fn parse_multiplicative<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    for (op_str, op) in &[("*", BinaryOp::Mul), ("/", BinaryOp::Div)] {
        if let Some((left, right)) = split_at_operator_rightmost(input, op_str) {
            let left_expr = parse_multiplicative(left.trim())?;
            let right_expr = parse_unary(right.trim())?;
            return Ok(Expr::binary_op(left_expr, *op, right_expr));
        }
    }
    parse_unary(input)
}

fn parse_unary<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    let input = input.trim();
    if let Some(rest) = input.strip_prefix('!') {
        let inner = parse_unary(rest.trim())?;
        return Ok(Expr::not(inner));
    }
    parse_primary(input)
}

fn parse_primary<'a>(input: &'a str) -> Result<Expr<'a>, DerivationParseError> {
    let input = input.trim();

    if input.is_empty() {
        return Err(DerivationParseError::ExpectedExpression(input.to_string()));
    }

    // Parenthesized expression
    if input.starts_with('(') {
        if let Some(inner) = extract_parens(input)? {
            let expr = parse_expr(inner)?;
            return Ok(Expr::paren(expr));
        }
    }

    // Integer literal
    if let Ok(n) = input.parse::<i64>() {
        return Ok(Expr::Integer(n));
    }

    // Function call
    if let Some(paren_pos) = input.find('(') {
        let name = input[..paren_pos].trim();
        if is_valid_identifier(name) {
            let args_str = &input[paren_pos..];
            if let Some(inner) = extract_parens(args_str)? {
                let args = parse_function_args(inner)?;
                return Ok(Expr::FunctionCall { name, args });
            }
        }
    }

    // Identifier
    if is_valid_identifier(input) {
        return Ok(Expr::Ident(input));
    }

    Err(DerivationParseError::UnexpectedToken(input.to_string()))
}

fn split_at_operator<'a>(input: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth = 0;
    let bytes = input.as_bytes();

    for i in 0..input.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && input[i..].starts_with(op) => {
                if op.len() == 1 {
                    let c = bytes[i];
                    if c == b'=' && i > 0 && matches!(bytes[i - 1], b'!' | b'<' | b'>') {
                        continue;
                    }
                    if c == b'=' && i + 1 < input.len() && bytes[i + 1] == b'=' {
                        continue;
                    }
                    if (c == b'<' || c == b'>') && i + 1 < input.len() && bytes[i + 1] == b'=' {
                        continue;
                    }
                }
                return Some((&input[..i], &input[i + op.len()..]));
            }
            _ => {}
        }
    }
    None
}

fn split_at_operator_rightmost<'a>(input: &'a str, op: &str) -> Option<(&'a str, &'a str)> {
    let mut depth = 0;
    let mut last_pos = None;
    let bytes = input.as_bytes();

    for i in 0..input.len() {
        match bytes[i] {
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if depth == 0 && input[i..].starts_with(op) => {
                if op.len() == 1 {
                    let c = bytes[i];
                    if c == b'=' && i > 0 && matches!(bytes[i - 1], b'!' | b'<' | b'>') {
                        continue;
                    }
                    if c == b'=' && i + 1 < input.len() && bytes[i + 1] == b'=' {
                        continue;
                    }
                    if (c == b'<' || c == b'>') && i + 1 < input.len() && bytes[i + 1] == b'=' {
                        continue;
                    }
                }
                last_pos = Some(i);
            }
            _ => {}
        }
    }

    last_pos.map(|pos| (&input[..pos], &input[pos + op.len()..]))
}

fn extract_parens(input: &str) -> Result<Option<&str>, DerivationParseError> {
    if !input.starts_with('(') {
        return Ok(None);
    }

    let mut depth = 0;
    for (i, c) in input.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Ok(Some(&input[1..i]));
                }
            }
            _ => {}
        }
    }
    Err(DerivationParseError::UnclosedParen)
}

fn parse_function_args<'a>(input: &'a str) -> Result<Vec<Expr<'a>>, DerivationParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(vec![]);
    }

    let mut args = Vec::new();
    let mut depth = 0;
    let mut start = 0;

    for (i, c) in input.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                args.push(parse_expr(input[start..i].trim())?);
                start = i + 1;
            }
            _ => {}
        }
    }

    if start < input.len() {
        args.push(parse_expr(input[start..].trim())?);
    }

    Ok(args)
}

pub(crate) fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
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

    #[test]
    fn test_parse_binary_ops() {
        use BinaryOp::*;
        use Expr::{Ident, Integer};

        let cases = vec![
            ("1 + 2", Expr::binary_op(Integer(1), Add, Integer(2))),
            ("1 - 2", Expr::binary_op(Integer(1), Sub, Integer(2))),
            ("1 * 2", Expr::binary_op(Integer(1), Mul, Integer(2))),
            ("1 / 2", Expr::binary_op(Integer(1), Div, Integer(2))),
            ("a < b", Expr::binary_op(Ident("a"), Lt, Ident("b"))),
            ("a > b", Expr::binary_op(Ident("a"), Gt, Ident("b"))),
            ("a <= b", Expr::binary_op(Ident("a"), Le, Ident("b"))),
            ("a >= b", Expr::binary_op(Ident("a"), Ge, Ident("b"))),
            ("a == b", Expr::binary_op(Ident("a"), Eq, Ident("b"))),
            ("a != b", Expr::binary_op(Ident("a"), Ne, Ident("b"))),
        ];

        for (input, expected) in cases {
            let result = super::parse_expr(input).expect(input);
            assert_eq!(result, expected, "failed for '{}'", input);
        }
    }

    #[test]
    fn test_parse_ternary() {
        use BinaryOp::Gt;
        use Expr::{Ident, Integer};

        let result = super::parse_expr("a > 5 ? 1 : 0").unwrap();
        let expected = Expr::ternary(
            Expr::binary_op(Ident("a"), Gt, Integer(5)),
            Integer(1),
            Integer(0),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_function_call() {
        use BinaryOp::Sub;
        use Expr::{FunctionCall, Ident, Integer};

        let result = super::parse_expr("max(1, 2)").unwrap();
        assert_eq!(
            result,
            FunctionCall {
                name: "max",
                args: vec![Integer(1), Integer(2)]
            }
        );

        let result = super::parse_expr("max(P1 - S1, P2 - S2)").unwrap();
        assert_eq!(
            result,
            FunctionCall {
                name: "max",
                args: vec![
                    Expr::binary_op(Ident("P1"), Sub, Ident("S1")),
                    Expr::binary_op(Ident("P2"), Sub, Ident("S2")),
                ]
            }
        );
    }

    #[test]
    fn test_parse_precedence() {
        use BinaryOp::{Add, Mul};
        use Expr::Integer;

        // 1 + 2 * 3 should be 1 + (2 * 3)
        let result = super::parse_expr("1 + 2 * 3").unwrap();
        let expected = Expr::binary_op(
            Integer(1),
            Add,
            Expr::binary_op(Integer(2), Mul, Integer(3)),
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_decimal_derivation() {
        let input = r#"init_scale = max(S1,S2)
init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
min_scale = min(init_scale, 6)
delta = init_prec - 38
prec = min(init_prec, 38)
scale_after_borrow = max(init_scale - delta, min_scale)
scale = init_prec > 38 ? scale_after_borrow : init_scale
DECIMAL<prec, scale>"#;

        let result = DerivationExpr::parse(input).unwrap();
        assert_eq!(result.assignments.len(), 7);
        assert_eq!(result.assignments[0].name, "init_scale");
        assert_eq!(result.assignments[6].name, "scale");
    }

    #[test]
    fn test_parse_assignment() {
        use BinaryOp::{Add, Gt, Sub};
        use Expr::{FunctionCall, Ident, Integer};

        let cases: Vec<(&str, Result<AssignmentExpr, DerivationParseError>)> = vec![
            (
                "x = 1",
                Ok(AssignmentExpr {
                    name: "x",
                    value: Integer(1),
                }),
            ),
            (
                "foo = a + b",
                Ok(AssignmentExpr {
                    name: "foo",
                    value: Expr::binary_op(Ident("a"), Add, Ident("b")),
                }),
            ),
            (
                "result = max(P1, P2)",
                Ok(AssignmentExpr {
                    name: "result",
                    value: FunctionCall {
                        name: "max",
                        args: vec![Ident("P1"), Ident("P2")],
                    },
                }),
            ),
            (
                "scale = init_prec > 38 ? scale_after_borrow : init_scale",
                Ok(AssignmentExpr {
                    name: "scale",
                    value: Expr::ternary(
                        Expr::binary_op(Ident("init_prec"), Gt, Integer(38)),
                        Ident("scale_after_borrow"),
                        Ident("init_scale"),
                    ),
                }),
            ),
            (
                "delta = P1 - S1",
                Ok(AssignmentExpr {
                    name: "delta",
                    value: Expr::binary_op(Ident("P1"), Sub, Ident("S1")),
                }),
            ),
            (
                "cmp = a == b",
                Ok(AssignmentExpr {
                    name: "cmp",
                    value: Expr::binary_op(Ident("a"), BinaryOp::Eq, Ident("b")),
                }),
            ),
            (
                "neq = a != b",
                Ok(AssignmentExpr {
                    name: "neq",
                    value: Expr::binary_op(Ident("a"), BinaryOp::Ne, Ident("b")),
                }),
            ),
            (
                "le = a <= b",
                Ok(AssignmentExpr {
                    name: "le",
                    value: Expr::binary_op(Ident("a"), BinaryOp::Le, Ident("b")),
                }),
            ),
            (
                "ge = a >= b",
                Ok(AssignmentExpr {
                    name: "ge",
                    value: Expr::binary_op(Ident("a"), BinaryOp::Ge, Ident("b")),
                }),
            ),
            (
                "no_equals",
                Err(DerivationParseError::InvalidAssignment(
                    "no_equals".to_string(),
                )),
            ),
            (
                "= 42",
                Err(DerivationParseError::InvalidAssignment("= 42".to_string())),
            ),
            (
                "x = a + b + x > 10 ? 5 : (a + 10",
                Err(DerivationParseError::UnclosedParen),
            ),
            ("x = (a + b", Err(DerivationParseError::UnclosedParen)),
        ];

        for (input, expected) in cases {
            let result = parse_assignment(input);
            assert_eq!(result, expected, "failed for '{input}'");
        }
    }
}
