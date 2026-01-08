// SPDX-License-Identifier: Apache-2.0

//! Scalar function definitions with validated signatures and resolved types.
//!
//! This module provides typed wrappers around scalar functions parsed from extension
//! YAML files, validating constraints and resolving type strings to concrete types.

use crate::text::simple_extensions::{
    Options as RawOptions, ScalarFunction as RawScalarFunction, ScalarFunctionImplsItem as RawImpl,
    VariadicBehavior as RawVariadicBehavior,
};

use super::argument::{ArgumentsItem, ArgumentsItemError};
use super::extensions::TypeContext;
use super::type_ast::{TypeExpr, TypeParseError};
use super::types::{ConcreteType, ExtensionTypeError};
use crate::parse::Parse;
use thiserror::Error;

/// Errors that can occur when parsing scalar functions
#[derive(Debug, Error)]
pub enum ScalarFunctionError {
    /// Scalar function has no implementations
    #[error("Scalar function '{name}' must have at least one implementation")]
    NoImplementations {
        /// The function name
        name: String,
    },
    /// Invalid variadic behavior
    #[error("Variadic behavior {field} must be a non-negative integer, got {value}")]
    InvalidVariadicBehavior {
        /// The field that was invalid (min or max)
        field: String,
        /// The invalid value
        value: f64,
    },
    /// Variadic min is greater than max
    #[error("Variadic min ({min}) must be less than or equal to max ({max})")]
    VariadicMinGreaterThanMax {
        /// The minimum value
        min: u32,
        /// The maximum value
        max: u32,
    },
    /// Error parsing function argument
    #[error("Argument error: {0}")]
    ArgumentError(#[from] ArgumentsItemError),
    /// Error parsing type in function signature
    #[error("Type error: {0}")]
    TypeError(#[from] ExtensionTypeError),
    /// Error parsing type expression
    #[error("Type parse error: {0}")]
    TypeParseError(#[from] TypeParseError),
    /// Feature not yet implemented
    #[error("Not yet implemented: {0}")]
    NotYetImplemented(String),
}

/// A validated scalar function definition with one or more implementations
#[derive(Clone, Debug, PartialEq)]
pub struct ScalarFunction {
    /// Function name
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Function implementations (overloads)
    pub impls: Vec<Impl>,
}

impl ScalarFunction {
    /// Parse a scalar function from raw YAML, resolving types with the provided context
    ///
    /// Note: ctx is `&mut` to match the Parse trait, but is not actually mutated
    /// during function parsing since argument types remain as raw strings.
    pub(super) fn from_raw(
        raw: RawScalarFunction,
        ctx: &mut TypeContext,
    ) -> Result<Self, ScalarFunctionError> {
        if raw.impls.is_empty() {
            return Err(ScalarFunctionError::NoImplementations { name: raw.name });
        }

        let impls = raw
            .impls
            .into_iter()
            .map(|impl_| Impl::from_raw(impl_, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(ScalarFunction {
            name: raw.name,
            description: raw.description,
            impls,
        })
    }
}

/// A single function implementation (overload) with signature and resolved types
#[derive(Clone, Debug, PartialEq)]
pub struct Impl {
    /// Function arguments with types and optional names/descriptions
    pub args: Vec<ArgumentsItem>,
    /// Configurable function options (e.g., overflow behavior, rounding modes)
    pub options: Option<Options>,
    /// Variadic argument behavior
    pub variadic: Option<VariadicBehavior>,
    /// Whether the function output depends on session state (e.g., timezone, locale)
    pub session_dependent: Option<crate::text::simple_extensions::SessionDependent>,
    /// Whether the function is deterministic (same inputs always produce same output)
    pub deterministic: Option<crate::text::simple_extensions::Deterministic>,
    /// How the function handles null inputs and produces nullable outputs
    pub nullability: Option<crate::text::simple_extensions::NullabilityHandling>,
    /// Return type resolved to a concrete type
    ///
    /// The raw YAML type string is parsed and validated. Only concrete types
    /// (without type variables) are supported; functions with type variables
    /// are skipped in this basic implementation.
    pub return_type: ConcreteType,
    /// Language-specific implementation code (e.g., SQL, C++, Python)
    ///
    /// Maps language identifiers to implementation source code snippets.
    pub implementation: Option<std::collections::HashMap<String, String>>,
}

impl Impl {
    /// Parse an implementation from raw YAML, resolving types with the provided context
    pub(super) fn from_raw(
        raw: RawImpl,
        ctx: &mut TypeContext,
    ) -> Result<Self, ScalarFunctionError> {
        // Parse and validate the return type
        let return_type = match raw.return_.0 {
            crate::text::simple_extensions::Type::String(s) => {
                // Multiline strings indicate type derivation expressions
                // See: https://github.com/substrait-io/substrait-rs/issues/449
                if s.contains('\n') {
                    return Err(ScalarFunctionError::NotYetImplemented(
                        "Type derivation expressions - issue #449".to_string(),
                    ));
                }

                // Parse the type string into TypeExpr
                let type_expr = TypeExpr::parse(&s)?;

                // Track custom type references (but not type parameters like P, L, S)
                // This validates that referenced extension types actually exist
                type_expr.visit_references(&mut |name| ctx.linked(name));

                // Try to convert to ConcreteType (fails if it has type variables)
                match ConcreteType::try_from(type_expr) {
                    Ok(concrete) => {
                        // Successfully parsed to concrete type
                        concrete
                    }
                    Err(ExtensionTypeError::InvalidAnyTypeVariable { .. }) |
                    Err(ExtensionTypeError::InvalidParameter(_)) |
                    Err(ExtensionTypeError::InvalidParameterKind { .. }) => {
                        // Type has type/parameter variables (any1, L1, P, etc.) - not yet supported
                        // See: https://github.com/substrait-io/substrait-rs/issues/452
                        return Err(ScalarFunctionError::NotYetImplemented(
                            "Type variables in function signatures - issue #452".to_string(),
                        ));
                    }
                    Err(ExtensionTypeError::UnknownTypeName { name }) => {
                        // Unknown type name - missing u! prefix
                        // This should fail, not be silently skipped
                        return Err(ScalarFunctionError::TypeError(
                            ExtensionTypeError::UnknownTypeName { name }
                        ));
                    }
                    Err(e) => {
                        // Other errors should also fail
                        return Err(ScalarFunctionError::TypeError(e));
                    }
                }
            }
            crate::text::simple_extensions::Type::Object(_) => {
                // Struct return types (YAML syntactic sugar) are not yet supported
                // See: https://github.com/substrait-io/substrait-rs/issues/450
                return Err(ScalarFunctionError::NotYetImplemented(
                    "Struct return types - issue #450".to_string(),
                ));
            }
        };

        // Convert and validate variadic behavior if present
        let variadic = raw.variadic.map(|v| v.try_into()).transpose()?;

        // Parse and validate arguments (but note: in this basic version we're not parsing their types)
        let args = match raw.args {
            Some(a) => {
                a.0.into_iter()
                    .map(|raw_arg| raw_arg.parse(ctx))
                    .collect::<Result<Vec<_>, _>>()?
            }
            None => Vec::new(),
        };

        Ok(Impl {
            args,
            options: raw.options.as_ref().map(Options::from),
            variadic,
            session_dependent: raw.session_dependent,
            deterministic: raw.deterministic,
            nullability: raw.nullability,
            return_type,
            implementation: raw.implementation.map(|i| i.0.into_iter().collect()),
        })
    }
}

/// Validated variadic behavior with min/max constraints
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicBehavior {
    /// Minimum number of arguments (None = 0)
    pub min: Option<u32>,
    /// Maximum number of arguments (None = unlimited)
    pub max: Option<u32>,
}

impl TryFrom<RawVariadicBehavior> for VariadicBehavior {
    type Error = ScalarFunctionError;

    fn try_from(raw: RawVariadicBehavior) -> Result<Self, Self::Error> {
        let min = raw
            .min
            .map(|v| {
                if v < 0.0 {
                    Err(ScalarFunctionError::InvalidVariadicBehavior {
                        field: "min".to_string(),
                        value: v,
                    })
                } else {
                    Ok(v as u32)
                }
            })
            .transpose()?;

        let max = raw
            .max
            .map(|v| {
                if v < 0.0 {
                    Err(ScalarFunctionError::InvalidVariadicBehavior {
                        field: "max".to_string(),
                        value: v,
                    })
                } else {
                    Ok(v as u32)
                }
            })
            .transpose()?;

        // Validate min <= max if both are present
        if let (Some(min_val), Some(max_val)) = (min, max) {
            if min_val > max_val {
                return Err(ScalarFunctionError::VariadicMinGreaterThanMax {
                    min: min_val,
                    max: max_val,
                });
            }
        }

        Ok(VariadicBehavior { min, max })
    }
}

/// Validated function options
#[derive(Clone, Debug, PartialEq)]
pub struct Options(pub std::collections::HashMap<String, Vec<String>>);

impl From<&RawOptions> for Options {
    fn from(raw: &RawOptions) -> Self {
        Options(
            raw.0
                .iter()
                .map(|(k, v)| (k.clone(), v.values.clone()))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variadic_min_greater_than_max() {
        let raw = RawVariadicBehavior {
            min: Some(5.0),
            max: Some(3.0),
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw);
        assert!(matches!(
            result,
            Err(ScalarFunctionError::VariadicMinGreaterThanMax { min: 5, max: 3 })
        ));
    }

    #[test]
    fn test_variadic_negative_min() {
        let raw = RawVariadicBehavior {
            min: Some(-1.0),
            max: None,
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw);
        assert!(matches!(
            result,
            Err(ScalarFunctionError::InvalidVariadicBehavior { field, value })
            if field == "min" && value == -1.0
        ));
    }

    #[test]
    fn test_variadic_negative_max() {
        let raw = RawVariadicBehavior {
            min: None,
            max: Some(-2.5),
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw);
        assert!(matches!(
            result,
            Err(ScalarFunctionError::InvalidVariadicBehavior { field, value })
            if field == "max" && value == -2.5
        ));
    }

    #[test]
    fn test_variadic_valid() {
        let raw = RawVariadicBehavior {
            min: Some(1.0),
            max: Some(5.0),
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw).unwrap();
        assert_eq!(result.min, Some(1));
        assert_eq!(result.max, Some(5));
    }

    #[test]
    fn test_variadic_none_values() {
        let raw = RawVariadicBehavior {
            min: None,
            max: None,
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw).unwrap();
        assert_eq!(result.min, None);
        assert_eq!(result.max, None);
    }

    #[test]
    fn test_no_implementations_error() {
        use crate::text::simple_extensions::ScalarFunction as RawScalarFunction;

        let raw = RawScalarFunction {
            name: "empty_function".to_string(),
            description: None,
            impls: vec![],
        };

        let mut ctx = super::super::extensions::TypeContext::default();
        let result = ScalarFunction::from_raw(raw, &mut ctx);

        assert!(matches!(
            result,
            Err(ScalarFunctionError::NoImplementations { name })
            if name == "empty_function"
        ));
    }

    #[test]
    fn test_scalar_function_with_single_impl() {
        use crate::text::simple_extensions::{
            ReturnValue, ScalarFunction as RawScalarFunction, ScalarFunctionImplsItem, Type,
        };

        let raw = RawScalarFunction {
            name: "add".to_string(),
            description: Some("Addition function".to_string()),
            impls: vec![ScalarFunctionImplsItem {
                args: None,
                options: None,
                variadic: None,
                session_dependent: None,
                deterministic: None,
                nullability: None,
                return_: ReturnValue(Type::String("i32".to_string())),
                implementation: None,
            }],
        };

        let mut ctx = super::super::extensions::TypeContext::default();
        let result = ScalarFunction::from_raw(raw, &mut ctx).unwrap();

        assert_eq!(result.name, "add");
        assert_eq!(result.description, Some("Addition function".to_string()));
        assert_eq!(result.impls.len(), 1);

        // Verify return type is properly parsed to ConcreteType
        use super::super::types::{BasicBuiltinType, ConcreteTypeKind};
        let return_type = &result.impls[0].return_type;
        assert!(!return_type.nullable, "i32 should not be nullable");
        match &return_type.kind {
            ConcreteTypeKind::Builtin(BasicBuiltinType::I32) => {
                // Expected - test passes
            }
            other => panic!("Expected I32, got {:?}", other),
        }
    }

    #[test]
    fn test_options_conversion() {
        use crate::text::simple_extensions::{Options as RawOptions, OptionsValue};
        use indexmap::IndexMap;

        let mut raw_map = IndexMap::new();
        raw_map.insert(
            "overflow".to_string(),
            OptionsValue {
                values: vec!["SILENT".to_string(), "ERROR".to_string()],
                description: None,
            },
        );

        let raw = RawOptions(raw_map);
        let options = Options::from(&raw);

        assert_eq!(options.0.len(), 1);
        assert_eq!(
            options.0.get("overflow").unwrap(),
            &vec!["SILENT".to_string(), "ERROR".to_string()]
        );
    }
}
