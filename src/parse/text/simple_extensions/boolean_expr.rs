// SPDX-License-Identifier: Apache-2.0

//! Boolean expressions retained in type derivations without evaluation.

use std::fmt;

use super::types::IntegerParameter;

/// A boolean-valued expression. Integer and type expressions cannot be used as conditions.
#[derive(Clone, Debug, PartialEq)]
pub enum BooleanExpr {
    /// A comparison of two integers.
    Compare {
        /// The comparison operator.
        op: ComparisonOp,
        /// The left integer operand.
        left: Box<IntegerParameter>,
        /// The right integer operand.
        right: Box<IntegerParameter>,
    },
}

/// A comparison of integer expressions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComparisonOp {
    /// Greater than.
    Greater,
}

impl fmt::Display for ComparisonOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Greater => f.write_str(">"),
        }
    }
}

impl fmt::Display for BooleanExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compare { op, left, right } => write!(f, "({left}) {op} ({right})"),
        }
    }
}
