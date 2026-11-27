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

#[cfg(test)]
mod tests {
    use super::super::types::TypeKind;
    use super::*;

    #[test]
    fn parse_plain_return_types() {
        assert_eq!(
            ReturnType::parse("i32\n").unwrap(),
            ReturnType::Type(Type::new(TypeKind::I32, false))
        );
    }

    #[test]
    fn parse_derivations_validate_the_final_type() {
        assert!(matches!(
            ReturnType::parse("P = 1\ndecimal<39, 0>"),
            Err(ExtensionTypeError::InvalidParameterRange { .. })
        ));
    }

    #[test]
    fn type_conditionals_preserve_both_branches_without_evaluation() {
        let parsed = ReturnType::parse("P > 10 ? i64 : i32").unwrap();
        let ReturnType::Type(return_type) = &parsed else {
            panic!("expected a plain return");
        };
        let TypeKind::Conditional {
            condition,
            then_type,
            else_type,
        } = &return_type.kind
        else {
            panic!("expected a conditional type");
        };
        assert!(matches!(
            condition.as_ref(),
            super::super::BooleanExpr::Compare { .. }
        ));
        assert_eq!(then_type.kind, TypeKind::I64);
        assert_eq!(else_type.kind, TypeKind::I32);
        assert!(!return_type.is_concrete());
        assert_eq!(ReturnType::parse(&parsed.to_string()).unwrap(), parsed);
    }

    #[test]
    fn typed_conditional_returns_roundtrip() {
        for source in [
            "if P > 10 then decimal<10, 2> else decimal<P, 2>",
            "list<(P > 1) ? i32? : i64>",
            "map<string, (P > 1) ? list<i32> : list<i64>>",
            "func<i32 -> (P > 1) ? i32 : i64>",
            "P > 1 ? (Q > 2 ? i32 : i64) : fp64",
            "varchar<(P >= 1 AND P <= 10) ? P : 10>",
            "varchar<(!(P = 1) OR P != 2) ? 1 : 2>",
            "varchar<IF P < 1 THEN 1 ELSE P>",
            "varchar<((P > 1) ? Q > 2 : Q < 3) ? 1 : 2>",
            "P = 10\nlist<(P > 1) ? i32 : i64>",
            "u!box<(P > 1) ? 2 : 3>",
            "u!box<(P > 1) ? i32 : i64>",
        ] {
            let parsed =
                ReturnType::parse(source).unwrap_or_else(|error| panic!("{source}: {error}"));
            assert_eq!(
                ReturnType::parse(&parsed.to_string()).unwrap(),
                parsed,
                "{source}"
            );
        }
    }

    #[test]
    fn type_conditionals_validate_both_branches() {
        for source in [
            "P > 1 ? decimal<39, 0> : i32",
            "P > 1 ? i32 : decimal<39, 0>",
        ] {
            assert!(matches!(
                ReturnType::parse(source),
                Err(ExtensionTypeError::InvalidParameterRange { .. })
            ));
        }
    }

    #[test]
    fn return_type_display_roundtrips() {
        for source in ["i32", "P = 10\nS = 2\ndecimal<P, S>"] {
            assert_eq!(ReturnType::parse(source).unwrap().to_string(), source);
        }
    }
}
