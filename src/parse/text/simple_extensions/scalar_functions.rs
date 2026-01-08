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
    /// Return type as a string
    ///
    /// The raw YAML type string (e.g., "i64", "varchar<L1>", "any1").
    /// In this basic implementation, the type string is stored as-is without parsing.
    pub return_type: String,
    /// Language-specific implementation code (e.g., SQL, C++, Python)
    ///
    /// Maps language identifiers to implementation source code snippets.
    pub implementation: Option<std::collections::HashMap<String, String>>,
}

impl Impl {
    /// Parse an implementation from raw YAML, resolving types with the provided context
    ///
    /// Note: ctx is `&mut` to match the Parse trait, but is not actually mutated
    /// during function parsing since argument types remain as raw strings.
    pub(super) fn from_raw(
        raw: RawImpl,
        _ctx: &mut TypeContext,
    ) -> Result<Self, ScalarFunctionError> {
        // Extract the return type string
        let return_type = match raw.return_.0 {
            crate::text::simple_extensions::Type::String(s) => s,
            crate::text::simple_extensions::Type::Object(_) => {
                // For this basic implementation, we'll just convert complex types to a string
                // This would be properly handled in a later PR
                "complex_type".to_string()
            }
        };

        // Convert and validate variadic behavior if present
        let variadic = raw.variadic.map(|v| v.try_into()).transpose()?;

        // Parse and validate arguments (but note: in this basic version we're not parsing their types)
        let args = match raw.args {
            Some(a) => {
                a.0.into_iter()
                    .map(|raw_arg| raw_arg.parse(_ctx))
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
