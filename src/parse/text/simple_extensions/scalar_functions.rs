// SPDX-License-Identifier: Apache-2.0

//! Scalar function definitions with validated signatures and resolved types.
//!
//! This module provides typed wrappers around scalar functions parsed from extension
//! YAML files, validating constraints and resolving type strings to concrete types.

use std::collections::HashMap;

use crate::text::simple_extensions::{
    NullabilityHandling as RawNullabilityHandling, Options as RawOptions,
    ScalarFunction as RawScalarFunction, ScalarFunctionImplsItem as RawImpl, Type as RawType,
    VariadicBehavior as RawVariadicBehavior, VariadicBehaviorParameterConsistency,
};

use super::argument::{ArgumentsItem, ArgumentsItemError};
use super::derivation::TypeDerivation;
use super::extensions::TypeContext;
use super::type_ast::{DerivationExpr, DerivationParseError, TypeExpr, TypeParseError};
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
    /// Error parsing type derivation expression
    #[error("Derivation parse error: {0}")]
    DerivationParseError(#[from] DerivationParseError),
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
    pub options: Options,
    /// Variadic argument behavior.
    ///
    /// `None` indicates the function is not variadic.
    pub variadic: Option<VariadicBehavior>,
    /// Whether the function output depends on session state (e.g., timezone, locale).
    ///
    /// Defaults to `false` per the Substrait spec.
    pub session_dependent: bool,
    /// Whether the function is deterministic (same inputs always produce same output).
    ///
    /// Defaults to `true` per the Substrait spec.
    pub deterministic: bool,
    /// How the function handles null inputs and produces nullable outputs.
    ///
    /// Defaults to [`NullabilityHandling::Mirror`] per the Substrait spec.
    pub nullability: NullabilityHandling,
    /// Return type of the function implementation.
    ///
    /// Can be either a concrete type (e.g., `i32`) or a type derivation expression
    /// for parameterized types like DECIMAL where the return type depends on input types.
    pub return_type: ReturnType,
    /// Language-specific implementation code (e.g., SQL, C++, Python)
    ///
    /// Maps language identifiers to implementation source code snippets.
    pub implementation: HashMap<String, String>,
}

impl Impl {
    /// Parse an implementation from raw YAML, resolving types with the provided context
    pub(super) fn from_raw(
        raw: RawImpl,
        ctx: &mut TypeContext,
    ) -> Result<Self, ScalarFunctionError> {
        // Parse and validate the return type
        let return_type = match raw.return_.0 {
            RawType::String(s) => {
                // Multiline strings indicate type derivation expressions
                if s.contains('\n') {
                    let derivation_expr = DerivationExpr::parse(&s)?;
                    // Visit type references in the result type
                    derivation_expr
                        .result_type
                        .visit_references(&mut |name| ctx.linked(name));
                    let derivation = TypeDerivation::from(derivation_expr);
                    ReturnType::Derivation(derivation)
                } else {
                    let type_expr = TypeExpr::parse(&s)?;
                    type_expr.visit_references(&mut |name| ctx.linked(name));
                    match ConcreteType::try_from(type_expr) {
                        Ok(concrete) => ReturnType::Concrete(concrete),
                        Err(ExtensionTypeError::InvalidAnyTypeVariable { .. })
                        | Err(ExtensionTypeError::InvalidParameter(_))
                        | Err(ExtensionTypeError::InvalidParameterKind { .. }) => {
                            // Type has type/parameter variables (any1, L1, P, etc.) - not yet supported
                            // See: https://github.com/substrait-io/substrait-rs/issues/452
                            return Err(ScalarFunctionError::NotYetImplemented(
                                "Type variables in function signatures - issue #452".to_string(),
                            ));
                        }
                        Err(ExtensionTypeError::UnknownTypeName { name }) => {
                            return Err(ScalarFunctionError::TypeError(
                                ExtensionTypeError::UnknownTypeName { name },
                            ));
                        }
                        Err(e) => return Err(ScalarFunctionError::TypeError(e)),
                    }
                }
            }
            RawType::Object(_) => {
                // Struct return types (YAML syntactic sugar) are not yet supported
                // See: https://github.com/substrait-io/substrait-rs/issues/450
                return Err(ScalarFunctionError::NotYetImplemented(
                    "Struct return types - issue #450".to_string(),
                ));
            }
        };

        let variadic = raw.variadic.map(|v| v.try_into()).transpose()?;

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
            options: raw.options.as_ref().map(Options::from).unwrap_or_default(),
            variadic,
            session_dependent: raw.session_dependent.map(|b| b.0).unwrap_or(false),
            deterministic: raw.deterministic.map(|b| b.0).unwrap_or(true),
            nullability: raw
                .nullability
                .map(Into::into)
                .unwrap_or(NullabilityHandling::Mirror),
            return_type,
            implementation: raw
                .implementation
                .map(|i| i.0.into_iter().collect())
                .unwrap_or_default(),
        })
    }
}

/// Validated variadic behavior with min/max constraints
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicBehavior {
    /// Minimum number of arguments
    pub min: u32,
    /// Maximum number of arguments (unlimited when None)
    pub max: Option<u32>,
    /// Whether all variadic parameters must have the same type.
    ///
    /// `None` when the parameter is not specified in the YAML. We cannot assume a default
    /// because the Substrait spec does not define default behavior for missing values
    /// (see <https://github.com/substrait-io/substrait/issues/928>).
    ///
    /// TODO: Once the spec defines default behavior, apply it and change this to a non-Option
    /// type (see issue #454).
    pub parameter_consistency: Option<ParameterConsistency>,
}

/// Specifies whether variadic parameters must have consistent types.
///
/// When a function's last argument is variadic with a type parameter (e.g., `fn(A, B, C...)`),
/// this controls type binding for the variadic arguments.
///
/// See: <https://github.com/substrait-io/substrait/issues/928>
#[derive(Clone, Debug, PartialEq)]
pub enum ParameterConsistency {
    /// All variadic arguments must have the same concrete type.
    ///
    /// For example, if `C` binds to `i32`, all variadic arguments must be `i32`.
    Consistent,
    /// Each variadic argument can have a different type.
    ///
    /// Each instance of the variadic parameter can bind to different types
    /// within the constraints of the type parameter.
    Inconsistent,
}

impl From<VariadicBehaviorParameterConsistency> for ParameterConsistency {
    fn from(raw: VariadicBehaviorParameterConsistency) -> Self {
        match raw {
            VariadicBehaviorParameterConsistency::Consistent => ParameterConsistency::Consistent,
            VariadicBehaviorParameterConsistency::Inconsistent => {
                ParameterConsistency::Inconsistent
            }
        }
    }
}

impl TryFrom<RawVariadicBehavior> for VariadicBehavior {
    type Error = ScalarFunctionError;

    fn try_from(raw: RawVariadicBehavior) -> Result<Self, Self::Error> {
        fn parse_bound(value: f64, field: &str) -> Result<u32, ScalarFunctionError> {
            if value < 0.0 || value.fract() != 0.0 {
                return Err(ScalarFunctionError::InvalidVariadicBehavior {
                    field: field.to_string(),
                    value,
                });
            }
            Ok(value as u32)
        }

        let min = raw
            .min
            .map(|v| parse_bound(v, "min"))
            .transpose()?
            .unwrap_or(0);
        let max = raw.max.map(|v| parse_bound(v, "max")).transpose()?;

        if let Some(max_val) = max {
            if min > max_val {
                return Err(ScalarFunctionError::VariadicMinGreaterThanMax { min, max: max_val });
            }
        }

        Ok(VariadicBehavior {
            min,
            max,
            parameter_consistency: raw.parameter_consistency.map(Into::into),
        })
    }
}

/// How a function handles null inputs and produces nullable outputs
#[derive(Clone, Debug, PartialEq)]
pub enum NullabilityHandling {
    /// Nullability of output mirrors the nullability of input(s)
    Mirror,
    /// Function explicitly declares the nullability of its output
    DeclaredOutput,
    /// Function handles nulls in a custom way per implementation
    Discrete,
}

/// The return type of a function implementation.
///
/// Return types can be either:
/// - A concrete type (e.g., `i32`, `fp64`, `DECIMAL<38,10>`)
/// - A type derivation expression that computes the return type based on input parameter values
///
/// Type derivation expressions are multiline programs used for parameterized types like DECIMAL,
/// where the return precision and scale depend on the input types:
///
/// ```yaml
/// return: |-
///   init_scale = max(S1,S2)
///   prec = min(P1 + P2, 38)
///   DECIMAL<prec, init_scale>
/// ```
#[derive(Clone, Debug, PartialEq)]
pub enum ReturnType {
    /// A concrete, resolved type
    Concrete(ConcreteType),
    /// A type derivation expression to evaluate at type-checking time
    Derivation(TypeDerivation),
}

impl From<RawNullabilityHandling> for NullabilityHandling {
    fn from(raw: RawNullabilityHandling) -> Self {
        match raw {
            RawNullabilityHandling::Mirror => NullabilityHandling::Mirror,
            RawNullabilityHandling::DeclaredOutput => NullabilityHandling::DeclaredOutput,
            RawNullabilityHandling::Discrete => NullabilityHandling::Discrete,
        }
    }
}

/// Validated function options
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Options(pub HashMap<String, Vec<String>>);

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
    fn test_variadic_invalid_values() {
        let invalid_cases = vec![
            (Some(-1.0), None, "negative min"),
            (None, Some(-2.5), "negative max"),
            (Some(7.2), None, "non-integer min"),
            (None, Some(3.5), "non-integer max"),
            (Some(5.0), Some(3.0), "min greater than max"),
        ];

        for (min, max, description) in invalid_cases {
            let raw = RawVariadicBehavior {
                min,
                max,
                parameter_consistency: None,
            };
            assert!(
                VariadicBehavior::try_from(raw).is_err(),
                "expected error for {}",
                description
            );
        }
    }

    #[test]
    fn test_variadic_valid() {
        let raw = RawVariadicBehavior {
            min: Some(1.0),
            max: Some(5.0),
            parameter_consistency: None,
        };
        let result = VariadicBehavior::try_from(raw).unwrap();
        assert_eq!(result.min, 1);
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
        assert_eq!(result.min, 0);
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
        match &result.impls[0].return_type {
            ReturnType::Concrete(return_type) => {
                assert!(!return_type.nullable, "i32 should not be nullable");
                assert!(matches!(
                    &return_type.kind,
                    ConcreteTypeKind::Builtin(BasicBuiltinType::I32)
                ));
            }
            ReturnType::Derivation(_) => panic!("expected Concrete, got Derivation"),
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
