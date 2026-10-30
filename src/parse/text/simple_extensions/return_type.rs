// SPDX-License-Identifier: Apache-2.0

//! Return declarations retained without binding parameters or evaluating derivations.

use std::fmt;

use super::type_ast::TypeExpr;
use super::types::{ExtensionTypeError, IntegerParameter, Type};

/// A return type or a program deriving its numeric parameters.
#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    /// A type without preceding assignments.
    Type(Type),
    /// Ordered assignments followed by the final return type.
    Derivation {
        /// Assignments in source order, retained without evaluation.
        assignments: Vec<IntegerAssignment>,
        /// The final type, which may refer to assigned names.
        return_type: Type,
    },
}

/// An assignment to a named integer parameter in a return derivation.
#[derive(Clone, Debug, PartialEq)]
pub struct IntegerAssignment {
    /// The parameter being assigned.
    pub name: String,
    /// The unevaluated expression assigned to the parameter.
    pub value: IntegerParameter,
}

pub(super) struct ReturnTypeExpr<'a> {
    pub assignments: Vec<IntegerAssignment>,
    pub return_type: TypeExpr<'a>,
}

impl ReturnType {
    /// Parse a plain type or ordered integer assignments followed by a final type.
    ///
    /// Assignment names are retained without binding or evaluation.
    pub fn parse(source: &str) -> Result<Self, ExtensionTypeError> {
        Self::try_from(super::type_parser::parse_return_type(source)?)
    }
}

impl From<Type> for ReturnType {
    fn from(value: Type) -> Self {
        Self::Type(value)
    }
}

impl TryFrom<ReturnTypeExpr<'_>> for ReturnType {
    type Error = ExtensionTypeError;

    fn try_from(value: ReturnTypeExpr<'_>) -> Result<Self, Self::Error> {
        let return_type = Type::try_from(value.return_type)?;
        Ok(if value.assignments.is_empty() {
            Self::Type(return_type)
        } else {
            Self::Derivation {
                assignments: value.assignments,
                return_type,
            }
        })
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type(return_type) => write!(f, "{return_type}"),
            Self::Derivation {
                assignments,
                return_type,
            } => {
                for assignment in assignments {
                    writeln!(f, "{} = {}", assignment.name, assignment.value)?;
                }
                write!(f, "{return_type}")
            }
        }
    }
}
