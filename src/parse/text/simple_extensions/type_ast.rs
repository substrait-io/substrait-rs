// SPDX-License-Identifier: Apache-2.0

//! Parsed type AST used by the simple extensions type parser.
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

use crate::parse::text::simple_extensions::types::is_builtin_type_name;

/// A parsed type expression from a type string.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExpr {
    /// A type with a name, optional parameters, and nullability
    Simple(String, Vec<TypeExprParam>, bool),
    /// A user-defined extension type, indicated by `u!Name`, with optional
    /// parameters and nullability
    UserDefined(String, Vec<TypeExprParam>, bool),
    /// Type variable (e.g., any1, any2)
    TypeVariable(u32, bool),
}

/// A parsed parameter to a parameterized type
#[derive(Clone, Debug, PartialEq)]
pub enum TypeExprParam {
    /// A nested type parameter
    Type(TypeExpr),
    /// An integer literal parameter
    Integer(i64),
    /// An integer variable parameter (e.g., P, P1, S, L1)
    IntegerVariable(String),
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum TypeParseError {
    #[error("missing closing angle bracket in parameter list: {0}")]
    ExpectedClosingAngleBracket(String),
    #[error("Type variation syntax is not supported: {0}")]
    UnsupportedVariation(String),
    #[error("invalid derivation expression: {0}")]
    InvalidDerivationExpression(String),
    #[error("derivation must end with a type expression, found: {0}")]
    MissingResultType(String),
    #[error("expected '=' in assignment statement: {0}")]
    MissingEquals(String),
    #[error("unexpected token in expression: {0}")]
    UnexpectedToken(String),
}

impl TypeExpr {
    /// Parse a type string into a [`TypeExpr`].
    pub fn parse(type_str: &str) -> Result<Self, TypeParseError> {
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

        let (name_and_nullable, params) = match rest.split_once('<') {
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
            Some(name) => (name.to_string(), true),
            None => (name_and_nullable.to_string(), false),
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

fn parse_params(s: &str) -> Result<Vec<TypeExprParam>, TypeParseError> {
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

fn parse_param(s: &str) -> Result<TypeExprParam, TypeParseError> {
    // Try integer literal
    if let Ok(i) = s.parse::<i64>() {
        return Ok(TypeExprParam::Integer(i));
    }

    // Check if it's a variable pattern (single uppercase letter + optional digits)
    // Examples: P, P1, S, S1, L, L1
    if !s.is_empty() {
        let mut chars = s.chars();
        if let Some(first) = chars.next() {
            if first.is_uppercase() && chars.all(|c| c.is_ascii_digit()) {
                return Ok(TypeExprParam::IntegerVariable(s.to_string()));
            }
        }
    }

    // Otherwise parse as nested type
    Ok(TypeExprParam::Type(TypeExpr::parse(s)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(expr: &str) -> TypeExpr {
        TypeExpr::parse(expr).expect("parse succeeds")
    }

    #[test]
    fn test_simple_types() {
        let cases = vec![
            ("i32", TypeExpr::Simple("i32".to_string(), vec![], false)),
            ("i32?", TypeExpr::Simple("i32".to_string(), vec![], true)),
            ("MAP", TypeExpr::Simple("MAP".to_string(), vec![], false)),
            (
                "timestamp",
                TypeExpr::Simple("timestamp".to_string(), vec![], false),
            ),
            (
                "timestamp_tz?",
                TypeExpr::Simple("timestamp_tz".to_string(), vec![], true),
            ),
            ("time", TypeExpr::Simple("time".to_string(), vec![], false)),
            ("any", TypeExpr::Simple("any".to_string(), vec![], false)),
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
                    "geo".to_string(),
                    vec![
                        TypeExprParam::Type(TypeExpr::Simple("i32".to_string(), vec![], true)),
                        TypeExprParam::Type(TypeExpr::Simple(
                            "point".to_string(),
                            vec![
                                TypeExprParam::Type(TypeExpr::Simple(
                                    "i32".to_string(),
                                    vec![],
                                    false,
                                )),
                                TypeExprParam::Type(TypeExpr::Simple(
                                    "i32".to_string(),
                                    vec![],
                                    false,
                                )),
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
                    "Map".to_string(),
                    vec![
                        TypeExprParam::Type(TypeExpr::Simple("i32".to_string(), vec![], false)),
                        TypeExprParam::Type(TypeExpr::Simple("string".to_string(), vec![], false)),
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
            ("Geo<string>", vec!["Geo".to_string()]),
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
}

/// Unary operators for type derivation expressions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    /// Boolean not (!)
    Not,
}

/// Binary operators for type derivation expressions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    /// Addition (+)
    Add,
    /// Subtraction (-)
    Sub,
    /// Multiplication (*)
    Mul,
    /// Division (/)
    Div,
    /// Less than (<)
    Lt,
    /// Greater than (>)
    Gt,
    /// Equal (==)
    Eq,
    /// Logical and (&&)
    And,
    /// Logical or (||)
    Or,
}

/// Expression AST for type derivation computations
#[derive(Clone, Debug, PartialEq)]
pub enum DerivationExpr {
    /// Integer parameter from input types (e.g., P1, S2)
    Parameter(String),
    /// Variable defined in previous statement (e.g., init_scale)
    Variable(String),
    /// Integer literal (e.g., 38, 6)
    Literal(i64),
    /// Unary operation (!)
    UnaryOp {
        /// The unary operator
        op: UnaryOperator,
        /// The operand
        operand: Box<DerivationExpr>,
    },
    /// Binary operation
    BinaryOp {
        /// The binary operator
        op: BinaryOperator,
        /// Left operand
        left: Box<DerivationExpr>,
        /// Right operand
        right: Box<DerivationExpr>,
    },
    /// Ternary conditional: condition ? then_expr : else_expr
    Conditional {
        /// Condition expression
        condition: Box<DerivationExpr>,
        /// Expression to evaluate if condition is true
        then_expr: Box<DerivationExpr>,
        /// Expression to evaluate if condition is false
        else_expr: Box<DerivationExpr>,
    },
    /// Function call (min, max)
    FunctionCall {
        /// Function name
        name: String,
        /// Function arguments
        args: Vec<DerivationExpr>,
    },
}

/// A single assignment statement: `variable = expression`
#[derive(Clone, Debug, PartialEq)]
pub struct DerivationStatement {
    /// Variable name being assigned
    pub variable: String,
    /// Expression to compute
    pub expression: DerivationExpr,
}

/// A type derivation with intermediate computations and final result
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDerivation {
    /// Intermediate assignment statements
    pub statements: Vec<DerivationStatement>,
    /// Final result type expression
    pub result_type: TypeExpr,
}

impl TypeDerivation {
    /// Parse a multi-line type derivation expression
    ///
    /// Expected format:
    /// ```text
    /// var1 = expr1
    /// var2 = expr2
    /// ...
    /// TYPENAME<...>
    /// ```
    pub fn parse(input: &str) -> Result<Self, TypeParseError> {
        let lines: Vec<&str> = input
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        if lines.is_empty() {
            return Err(TypeParseError::MissingResultType("empty input".to_string()));
        }

        let mut statements = Vec::new();
        let mut i = 0;

        // Parse assignment statements
        while i < lines.len() - 1 {
            let line = lines[i];
            if let Some((var, expr)) = line.split_once('=') {
                let var = var.trim().to_string();
                let expr = expr.trim();
                let expression = DerivationExpr::parse(expr)?;
                statements.push(DerivationStatement {
                    variable: var,
                    expression,
                });
            } else if !line.contains('<') && !line.contains('>') {
                // Non-assignment line that's not a type - could be an error
                return Err(TypeParseError::MissingEquals(line.to_string()));
            }
            i += 1;
        }

        // Last line must be a type
        let last_line = lines[lines.len() - 1];

        let result_type = TypeExpr::parse(last_line)
            .map_err(|_| TypeParseError::MissingResultType(last_line.to_string()))?;

        Ok(TypeDerivation {
            statements,
            result_type,
        })
    }
}

impl DerivationExpr {
    /// Parse a derivation expression using recursive descent
    pub fn parse(input: &str) -> Result<Self, TypeParseError> {
        Parser::new(input).parse_ternary()
    }
}

/// Simple tokenizer for derivation expressions
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input: input.trim(),
            pos: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume_char(&mut self) -> Option<char> {
        let ch = self.peek_char()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek_char(), Some(c) if c.is_whitespace()) {
            self.consume_char();
        }
    }

    fn consume_if(&mut self, expected: char) -> bool {
        self.skip_whitespace();
        if self.peek_char() == Some(expected) {
            self.consume_char();
            true
        } else {
            false
        }
    }

    fn consume_str(&mut self, s: &str) -> bool {
        self.skip_whitespace();
        if self.input[self.pos..].starts_with(s) {
            self.pos += s.len();
            true
        } else {
            false
        }
    }

    // Ternary: lowest precedence
    fn parse_ternary(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let expr = self.parse_or()?;

        self.skip_whitespace();
        if self.consume_if('?') {
            let then_expr = Box::new(self.parse_ternary()?);
            self.skip_whitespace();
            if !self.consume_if(':') {
                return Err(TypeParseError::UnexpectedToken(
                    "expected ':' in ternary".to_string(),
                ));
            }
            let else_expr = Box::new(self.parse_ternary()?);
            Ok(DerivationExpr::Conditional {
                condition: Box::new(expr),
                then_expr,
                else_expr,
            })
        } else {
            Ok(expr)
        }
    }

    // Or: ||
    fn parse_or(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let mut left = self.parse_and()?;

        while self.consume_str("||") {
            let right = self.parse_and()?;
            left = DerivationExpr::BinaryOp {
                op: BinaryOperator::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // And: &&
    fn parse_and(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let mut left = self.parse_comparison()?;

        while self.consume_str("&&") {
            let right = self.parse_comparison()?;
            left = DerivationExpr::BinaryOp {
                op: BinaryOperator::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    // Comparison: <, >, ==
    fn parse_comparison(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let left = self.parse_additive()?;

        self.skip_whitespace();
        let op = if self.consume_str("==") {
            Some(BinaryOperator::Eq)
        } else if self.consume_if('<') {
            Some(BinaryOperator::Lt)
        } else if self.consume_if('>') {
            Some(BinaryOperator::Gt)
        } else {
            None
        };

        if let Some(op) = op {
            let right = self.parse_additive()?;
            Ok(DerivationExpr::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            Ok(left)
        }
    }

    // Additive: +, -
    fn parse_additive(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let mut left = self.parse_multiplicative()?;

        loop {
            self.skip_whitespace();
            let op = if self.consume_if('+') {
                Some(BinaryOperator::Add)
            } else if self.consume_if('-') {
                Some(BinaryOperator::Sub)
            } else {
                None
            };

            if let Some(op) = op {
                let right = self.parse_multiplicative()?;
                left = DerivationExpr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Multiplicative: *, /
    fn parse_multiplicative(&mut self) -> Result<DerivationExpr, TypeParseError> {
        let mut left = self.parse_unary()?;

        loop {
            self.skip_whitespace();
            let op = if self.consume_if('*') {
                Some(BinaryOperator::Mul)
            } else if self.consume_if('/') {
                Some(BinaryOperator::Div)
            } else {
                None
            };

            if let Some(op) = op {
                let right = self.parse_unary()?;
                left = DerivationExpr::BinaryOp {
                    op,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    // Unary: !
    fn parse_unary(&mut self) -> Result<DerivationExpr, TypeParseError> {
        self.skip_whitespace();
        if self.consume_if('!') {
            let operand = Box::new(self.parse_unary()?);
            Ok(DerivationExpr::UnaryOp {
                op: UnaryOperator::Not,
                operand,
            })
        } else {
            self.parse_primary()
        }
    }

    // Primary: function_call, parenthesized expr, or atom
    fn parse_primary(&mut self) -> Result<DerivationExpr, TypeParseError> {
        self.skip_whitespace();

        // Parenthesized expression
        if self.consume_if('(') {
            let expr = self.parse_ternary()?;
            self.skip_whitespace();
            if !self.consume_if(')') {
                return Err(TypeParseError::UnexpectedToken("expected ')'".to_string()));
            }
            return Ok(expr);
        }

        // Try to parse identifier or number
        self.parse_atom()
    }

    // Atom: integer literal, function call, parameter, or variable
    fn parse_atom(&mut self) -> Result<DerivationExpr, TypeParseError> {
        self.skip_whitespace();

        let start = self.pos;

        // Parse identifier or number
        let mut has_letter = false;
        while let Some(ch) = self.peek_char() {
            if ch.is_alphanumeric() || ch == '_' {
                if ch.is_alphabetic() {
                    has_letter = true;
                }
                self.consume_char();
            } else {
                break;
            }
        }

        if start == self.pos {
            return Err(TypeParseError::UnexpectedToken(format!(
                "unexpected character: {:?}",
                self.peek_char()
            )));
        }

        let token = &self.input[start..self.pos];

        // Check if it's a function call
        self.skip_whitespace();
        if self.consume_if('(') {
            return self.parse_function_call(token);
        }

        // Try to parse as integer literal
        if !has_letter {
            if let Ok(val) = token.parse::<i64>() {
                return Ok(DerivationExpr::Literal(val));
            }
        }

        // Check if it's a parameter (uppercase letter + optional digits)
        if token.chars().next().is_some_and(|c| c.is_uppercase())
            && token.chars().skip(1).all(|c| c.is_ascii_digit())
        {
            return Ok(DerivationExpr::Parameter(token.to_string()));
        }

        // Otherwise it's a variable
        Ok(DerivationExpr::Variable(token.to_string()))
    }

    fn parse_function_call(&mut self, name: &str) -> Result<DerivationExpr, TypeParseError> {
        let mut args = Vec::new();

        self.skip_whitespace();
        if !self.consume_if(')') {
            loop {
                args.push(self.parse_ternary()?);
                self.skip_whitespace();
                if self.consume_if(')') {
                    break;
                }
                if !self.consume_if(',') {
                    return Err(TypeParseError::UnexpectedToken(
                        "expected ',' or ')' in function call".to_string(),
                    ));
                }
            }
        }

        Ok(DerivationExpr::FunctionCall {
            name: name.to_string(),
            args,
        })
    }
}
