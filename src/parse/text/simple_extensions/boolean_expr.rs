// SPDX-License-Identifier: Apache-2.0

//! Boolean expressions retained in type derivations without evaluation.

use std::fmt;

use super::types::IntegerParameter;

/// A boolean-valued expression. Integer and type expressions cannot be used as conditions.
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanExpr {
    /// A named boolean binding.
    Variable(String),
    /// Boolean negation.
    Not(Box<Self>),
    /// A conjunction or disjunction.
    Binary {
        /// The logical operator.
        op: BooleanOp,
        /// The left boolean operand.
        left: Box<Self>,
        /// The right boolean operand.
        right: Box<Self>,
    },
    /// A comparison of two integers.
    Compare {
        /// The comparison operator.
        op: ComparisonOp,
        /// The left integer operand.
        left: Box<IntegerParameter>,
        /// The right integer operand.
        right: Box<IntegerParameter>,
    },
    /// A boolean-valued conditional.
    Conditional {
        /// The condition selecting a branch.
        condition: Box<Self>,
        /// The boolean expression when the condition is true.
        then_value: Box<Self>,
        /// The boolean expression when the condition is false.
        else_value: Box<Self>,
    },
}

/// A boolean binary operator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BooleanOp {
    /// Conjunction.
    And,
    /// Disjunction.
    Or,
}

/// A comparison of integer expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonOp {
    /// Less than.
    Less,
    /// Less than or equal.
    LessOrEqual,
    /// Greater than.
    Greater,
    /// Greater than or equal.
    GreaterOrEqual,
    /// Equal.
    Equal,
    /// Not equal.
    NotEqual,
}

impl fmt::Display for BooleanOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::And => "AND",
            Self::Or => "OR",
        })
    }
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Less => "<",
            Self::LessOrEqual => "<=",
            Self::Greater => ">",
            Self::GreaterOrEqual => ">=",
            Self::Equal => "=",
            Self::NotEqual => "!=",
        })
    }
}

impl fmt::Display for BooleanExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(name) => f.write_str(name),
            Self::Not(operand) => write!(f, "!({operand})"),
            Self::Binary { op, left, right } => write!(f, "({left}) {op} ({right})"),
            Self::Compare { op, left, right } => write!(f, "({left}) {op} ({right})"),
            Self::Conditional {
                condition,
                then_value,
                else_value,
            } => {
                write!(f, "({condition}) ? ({then_value}) : ({else_value})")
            }
        }
    }
}
