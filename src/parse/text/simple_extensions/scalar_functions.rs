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
use super::types::{ExtensionTypeError, ReturnType, parse_return_type};
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
    /// during function parsing since argument types remain as raw strings and are
    /// not parsed to ConcreteType. See #444.
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
    /// Return type, which may be a simple type or a type derivation expression
    ///
    /// The raw YAML contains either:
    /// - A simple type string (e.g., "i64", "varchar<L1>", "any1", "decimal<P,S>")
    /// - A multi-line type derivation expression with intermediate computations
    ///
    /// Simple types are parsed into `ReturnType::Type(SignatureType)`.
    /// Multi-line expressions are parsed into `ReturnType::Derivation(TypeDerivation)`.
    pub return_type: ReturnType,
    /// Language-specific implementation code (e.g., SQL, C++, Python)
    ///
    /// Maps language identifiers to implementation source code snippets.
    pub implementation: Option<std::collections::HashMap<String, String>>,
}

impl Impl {
    /// Parse an implementation from raw YAML, resolving types with the provided context
    ///
    /// Note: ctx is `&mut` to match the Parse trait, but is not actually mutated
    /// during function parsing since argument types remain as raw strings and are
    /// not parsed to ConcreteType. See #444.
    pub(super) fn from_raw(
        raw: RawImpl,
        ctx: &mut TypeContext,
    ) -> Result<Self, ScalarFunctionError> {
        // Parse the return type - may be a simple type or a type derivation expression
        let return_type = parse_return_type(raw.return_.0, ctx)?;

        // Convert and validate variadic behavior if present
        let variadic = raw.variadic.map(|v| v.try_into()).transpose()?;

        // Parse and validate arguments
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

/// Variadic argument behavior with validated min/max bounds
///
/// Specifies constraints on variable-length argument lists.
/// Both min and max are validated to be non-negative integers.
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicBehavior {
    /// Minimum number of variadic arguments (non-negative integer)
    pub min: Option<u32>,
    /// Maximum number of variadic arguments (non-negative integer)
    pub max: Option<u32>,
    /// Whether all variadic arguments must have the same type
    ///
    /// Only applicable when the variadic argument contains type parameters.
    /// The spec does not currently define a default; this implementation uses CONSISTENT.
    /// See: https://github.com/substrait-io/substrait/issues/928
    pub parameter_consistency: ParameterConsistency,
}

/// Indicates whether variadic arguments must have consistent types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParameterConsistency {
    /// All variadic arguments must have the same type
    Consistent,
    /// Variadic arguments can have different types (as long as they satisfy type constraints)
    Inconsistent,
}

impl std::convert::TryFrom<RawVariadicBehavior> for VariadicBehavior {
    type Error = ScalarFunctionError;

    fn try_from(raw: RawVariadicBehavior) -> Result<Self, Self::Error> {
        let min = if let Some(min_val) = raw.min {
            if min_val < 0.0 || min_val.fract() != 0.0 {
                return Err(ScalarFunctionError::InvalidVariadicBehavior {
                    field: "min".to_string(),
                    value: min_val,
                });
            }
            Some(min_val as u32)
        } else {
            None
        };

        let max = if let Some(max_val) = raw.max {
            if max_val < 0.0 || max_val.fract() != 0.0 {
                return Err(ScalarFunctionError::InvalidVariadicBehavior {
                    field: "max".to_string(),
                    value: max_val,
                });
            }
            Some(max_val as u32)
        } else {
            None
        };

        if let (Some(min_val), Some(max_val)) = (min, max) {
            if min_val > max_val {
                return Err(ScalarFunctionError::VariadicMinGreaterThanMax {
                    min: min_val,
                    max: max_val,
                });
            }
        }

        let parameter_consistency = match raw.parameter_consistency {
            Some(
                crate::text::simple_extensions::VariadicBehaviorParameterConsistency::Consistent,
            ) => ParameterConsistency::Consistent,
            Some(
                crate::text::simple_extensions::VariadicBehaviorParameterConsistency::Inconsistent,
            ) => ParameterConsistency::Inconsistent,
            None => ParameterConsistency::Consistent, // Default to CONSISTENT
        };

        Ok(VariadicBehavior {
            min,
            max,
            parameter_consistency,
        })
    }
}

/// Function options controlling behavior like overflow, rounding, null handling
#[derive(Clone, Debug, PartialEq)]
pub struct Options(pub std::collections::HashMap<String, OptionValue>);

/// A single configurable option with its allowed values
///
/// Example from arithmetic functions:
/// - name: "overflow"
/// - description: "How to handle integer overflow"
/// - values: ["SILENT", "SATURATE", "ERROR"]
#[derive(Clone, Debug, PartialEq)]
pub struct OptionValue {
    /// Human-readable explanation of what this option controls
    pub description: Option<String>,
    /// List of valid string values for this option
    pub values: Vec<String>,
}

impl From<&RawOptions> for Options {
    fn from(raw: &RawOptions) -> Self {
        Options(
            raw.iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        OptionValue {
                            description: v.description.clone(),
                            values: v.values.clone(),
                        },
                    )
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::simple_extensions::{
        ArgumentsItem as RawArgumentItem, Options as RawOptions, OptionsValue as RawOptionValue,
        ReturnValue, ScalarFunctionImplsItem as RawImpl, Type as RawType,
        VariadicBehavior as RawVariadicBehavior, VariadicBehaviorParameterConsistency,
    };

    fn raw_impl_minimal() -> RawImpl {
        RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i64".to_string())),
            implementation: None,
        }
    }

    #[test]
    fn test_minimal_impl_parsing() {
        let raw = raw_impl_minimal();
        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw, &mut ctx);
        assert!(result.is_ok());
        let impl_ = result.unwrap();
        assert!(impl_.args.is_empty());
        assert!(impl_.options.is_none());
        assert!(impl_.variadic.is_none());
        assert!(impl_.session_dependent.is_none());
        assert!(impl_.deterministic.is_none());
        assert!(impl_.nullability.is_none());
        assert!(impl_.implementation.is_none());
    }

    #[test]
    fn test_impl_with_all_fields() {
        use crate::text::simple_extensions::{Arguments, ValueArg};

        let mut options_map = indexmap::IndexMap::new();
        options_map.insert(
            "overflow".to_string(),
            RawOptionValue {
                description: Some("Overflow behavior".to_string()),
                values: vec![
                    "SILENT".to_string(),
                    "SATURATE".to_string(),
                    "ERROR".to_string(),
                ],
            },
        );

        let arg = ValueArg {
            name: Some("x".to_string()),
            description: Some("First operand".to_string()),
            value: RawType::String("i32".to_string()),
            constant: None,
        };

        let raw_impl = RawImpl {
            args: Some(Arguments(vec![RawArgumentItem::ValueArg(arg)])),
            options: Some(RawOptions(options_map)),
            variadic: Some(RawVariadicBehavior {
                min: Some(1.0),
                max: Some(5.0),
                parameter_consistency: Some(VariadicBehaviorParameterConsistency::Consistent),
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i64".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let impl_ = Impl::from_raw(raw_impl, &mut ctx).expect("parsing should succeed");

        // Construct the expected parsed argument
        use crate::parse::Parse;
        let mut ctx_for_arg = TypeContext::default();
        let raw_arg_item = RawArgumentItem::ValueArg(ValueArg {
            name: Some("x".to_string()),
            description: Some("First operand".to_string()),
            value: RawType::String("i32".to_string()),
            constant: None,
        });
        let expected_arg = raw_arg_item
            .parse(&mut ctx_for_arg)
            .expect("Should parse argument");

        let mut expected_options_map = std::collections::HashMap::new();
        expected_options_map.insert(
            "overflow".to_string(),
            OptionValue {
                description: Some("Overflow behavior".to_string()),
                values: vec![
                    "SILENT".to_string(),
                    "SATURATE".to_string(),
                    "ERROR".to_string(),
                ],
            },
        );

        let expected = Impl {
            args: vec![expected_arg],
            options: Some(Options(expected_options_map)),
            variadic: Some(VariadicBehavior {
                min: Some(1),
                max: Some(5),
                parameter_consistency: ParameterConsistency::Consistent,
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_type: ReturnType::Type(
                crate::parse::text::simple_extensions::types::SignatureType {
                    kind: crate::parse::text::simple_extensions::types::SignatureTypeKind::I64,
                    nullable: false,
                },
            ),
            implementation: None,
        };

        assert_eq!(impl_, expected);
    }

    #[test]
    fn test_variadic_behavior_valid() {
        let raw = RawVariadicBehavior {
            min: Some(1.0),
            max: Some(10.0),
            parameter_consistency: Some(VariadicBehaviorParameterConsistency::Consistent),
        };

        let result: Result<VariadicBehavior, _> = raw.try_into();
        assert!(result.is_ok());

        let variadic = result.unwrap();
        assert_eq!(variadic.min, Some(1));
        assert_eq!(variadic.max, Some(10));
        assert_eq!(
            variadic.parameter_consistency,
            ParameterConsistency::Consistent
        );
    }

    #[test]
    fn test_variadic_behavior_default_consistent() {
        // When parameter_consistency is not specified, it should default to Consistent
        let raw = RawVariadicBehavior {
            min: Some(0.0),
            max: None,
            parameter_consistency: None,
        };

        let result: Result<VariadicBehavior, _> = raw.try_into();
        assert!(result.is_ok());

        let variadic = result.unwrap();
        assert_eq!(
            variadic.parameter_consistency,
            ParameterConsistency::Consistent
        );
    }

    #[test]
    fn test_variadic_behavior_invalid_negative_min() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: Some(RawVariadicBehavior {
                min: Some(-1.0),
                max: None,
                parameter_consistency: None,
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_err());
        match result.unwrap_err() {
            ScalarFunctionError::InvalidVariadicBehavior { field, value } => {
                assert_eq!(field, "min");
                assert_eq!(value, -1.0);
            }
            _ => panic!("Expected InvalidVariadicBehavior error"),
        }
    }

    #[test]
    fn test_variadic_behavior_invalid_fractional_min() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: Some(RawVariadicBehavior {
                min: Some(1.5),
                max: None,
                parameter_consistency: None,
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_err());
        match result.unwrap_err() {
            ScalarFunctionError::InvalidVariadicBehavior { field, value } => {
                assert_eq!(field, "min");
                assert_eq!(value, 1.5);
            }
            _ => panic!("Expected InvalidVariadicBehavior error"),
        }
    }

    #[test]
    fn test_variadic_behavior_invalid_negative_max() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: Some(RawVariadicBehavior {
                min: None,
                max: Some(-5.0),
                parameter_consistency: None,
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_err());
        match result.unwrap_err() {
            ScalarFunctionError::InvalidVariadicBehavior { field, value } => {
                assert_eq!(field, "max");
                assert_eq!(value, -5.0);
            }
            _ => panic!("Expected InvalidVariadicBehavior error"),
        }
    }

    #[test]
    fn test_variadic_behavior_min_greater_than_max() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: Some(RawVariadicBehavior {
                min: Some(10.0),
                max: Some(5.0),
                parameter_consistency: None,
            }),
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_err());
        match result.unwrap_err() {
            ScalarFunctionError::VariadicMinGreaterThanMax { min, max } => {
                assert_eq!(min, 10);
                assert_eq!(max, 5);
            }
            _ => panic!("Expected VariadicMinGreaterThanMax error"),
        }
    }

    #[test]
    fn test_options_conversion() {
        let mut inner = indexmap::IndexMap::new();
        inner.insert(
            "rounding".to_string(),
            RawOptionValue {
                description: Some("Rounding mode".to_string()),
                values: vec!["ROUND_UP".to_string(), "ROUND_DOWN".to_string()],
            },
        );
        let raw_opts = RawOptions(inner);

        let options = Options::from(&raw_opts);
        assert_eq!(options.0.len(), 1);
        let opt = options.0.get("rounding").unwrap();
        assert_eq!(opt.description, Some("Rounding mode".to_string()));
        assert_eq!(opt.values, vec!["ROUND_UP", "ROUND_DOWN"]);
    }

    #[test]
    fn test_scalar_function_parsing() {
        let raw = RawScalarFunction {
            name: "add".to_string(),
            description: Some("Addition function".to_string()),
            impls: vec![raw_impl_minimal()],
        };
        let mut ctx = TypeContext::default();
        let result = ScalarFunction::from_raw(raw, &mut ctx);
        assert!(result.is_ok());
        let func = result.unwrap();
        assert_eq!(func.name, "add");
        assert_eq!(func.description, Some("Addition function".to_string()));
        assert_eq!(func.impls.len(), 1);
    }

    #[test]
    fn test_scalar_function_no_implementations() {
        let raw = RawScalarFunction {
            name: "empty_function".to_string(),
            description: Some("Function with no implementations".to_string()),
            impls: vec![],
        };
        let mut ctx = TypeContext::default();
        let result = ScalarFunction::from_raw(raw, &mut ctx);
        assert!(result.is_err());
        match result.unwrap_err() {
            ScalarFunctionError::NoImplementations { name } => {
                assert_eq!(name, "empty_function");
            }
            _ => panic!("Expected NoImplementations error"),
        }
    }

    #[test]
    fn test_function_with_type_variable_return() {
        // Test that functions with type variable return types (like any1) parse correctly
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("any1".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_ok(), "Should parse type variable return type");

        let impl_ = result.unwrap();
        match &impl_.return_type {
            ReturnType::Type(sig_type) => {
                match sig_type.kind {
                    crate::parse::text::simple_extensions::types::SignatureTypeKind::TypeVariable {
                        id,
                    } => {
                        assert_eq!(id, 1, "Type variable any1 should have id 1");
                    }
                    _ => panic!("Expected TypeVariable kind"),
                }
                assert!(!sig_type.nullable);
            }
            _ => panic!("Expected Type return type, not Derivation"),
        }
    }

    #[test]
    fn test_function_with_integer_parameter_variables() {
        // Test that functions with integer parameter variables (like decimal<P,S>) parse correctly
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("decimal<P,S>".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(
            result.is_ok(),
            "Should parse integer parameter variables in return type"
        );

        let impl_ = result.unwrap();
        match &impl_.return_type {
            ReturnType::Type(sig_type) => match &sig_type.kind {
                crate::parse::text::simple_extensions::types::SignatureTypeKind::Decimal {
                    precision,
                    scale,
                } => {
                    // Check first parameter is IntegerVariable "P"
                    match precision {
                        crate::parse::text::simple_extensions::types::IntParam::Variable(var) => {
                            assert_eq!(var, "P");
                        }
                        _ => panic!("Expected IntegerVariable for precision parameter"),
                    }

                    // Check second parameter is IntegerVariable "S"
                    match scale {
                        crate::parse::text::simple_extensions::types::IntParam::Variable(var) => {
                            assert_eq!(var, "S");
                        }
                        _ => panic!("Expected IntegerVariable for scale parameter"),
                    }
                }
                _ => panic!("Expected Decimal kind for decimal<P,S>"),
            },
            _ => panic!("Expected Type return type, not Derivation"),
        }
    }

    #[test]
    fn test_function_with_nullable_type_variable() {
        // Test that nullable type variables parse correctly
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("any1?".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(result.is_ok(), "Should parse nullable type variable");

        let impl_ = result.unwrap();
        match &impl_.return_type {
            ReturnType::Type(sig_type) => {
                match sig_type.kind {
                    crate::parse::text::simple_extensions::types::SignatureTypeKind::TypeVariable {
                        id,
                    } => {
                        assert_eq!(id, 1);
                    }
                    _ => panic!("Expected TypeVariable kind"),
                }
                assert!(sig_type.nullable);
            }
            _ => panic!("Expected Type return type, not Derivation"),
        }
    }

    #[test]
    fn test_parameterized_builtin_with_variable() {
        // Test that parameterized builtins with variables (like varchar<L1>) are handled
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("varchar<L1>".to_string())),
            implementation: None,
        };

        let mut ctx = TypeContext::default();
        let result = Impl::from_raw(raw_impl, &mut ctx);
        assert!(
            result.is_ok(),
            "Should parse parameterized builtin with variable: {:?}",
            result.err()
        );

        let impl_ = result.unwrap();
        // With variables, varchar is represented directly as VarChar variant
        match &impl_.return_type {
            ReturnType::Type(sig_type) => match &sig_type.kind {
                crate::parse::text::simple_extensions::types::SignatureTypeKind::VarChar {
                    length,
                } => match length {
                    crate::parse::text::simple_extensions::types::IntParam::Variable(var) => {
                        assert_eq!(var, "L1");
                    }
                    _ => panic!("Expected IntegerVariable for varchar parameter"),
                },
                _ => panic!("Expected VarChar kind for varchar<L1>"),
            },
            _ => panic!("Expected Type return type, not Derivation"),
        }
    }

    #[test]
    fn test_parameterized_builtin_concrete_vs_variable() {
        let mut ctx = TypeContext::default();

        // varchar<10> with concrete parameter should parse as Builtin
        let concrete_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("varchar<10>".to_string())),
            implementation: None,
        };
        let result = Impl::from_raw(concrete_impl, &mut ctx).unwrap();
        match &result.return_type {
            ReturnType::Type(sig_type) => match &sig_type.kind {
                crate::parse::text::simple_extensions::types::SignatureTypeKind::VarChar {
                    length,
                } => match length {
                    crate::parse::text::simple_extensions::types::IntParam::Concrete(v) => {
                        assert_eq!(*v, 10);
                    }
                    _ => panic!("Expected Concrete integer for varchar<10>"),
                },
                _ => panic!("Expected VarChar for varchar<10>"),
            },
            _ => panic!("Expected Type return type, not Derivation"),
        }

        // varchar<L1> with variable should also parse as VarChar
        let variable_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("varchar<L1>".to_string())),
            implementation: None,
        };
        let result = Impl::from_raw(variable_impl, &mut ctx).unwrap();
        match &result.return_type {
            ReturnType::Type(sig_type) => match &sig_type.kind {
                crate::parse::text::simple_extensions::types::SignatureTypeKind::VarChar {
                    length,
                } => match length {
                    crate::parse::text::simple_extensions::types::IntParam::Variable(var) => {
                        assert_eq!(var, "L1");
                    }
                    _ => panic!("Expected Variable for varchar<L1>"),
                },
                _ => panic!("Expected VarChar for varchar<L1>"),
            },
            _ => panic!("Expected Type return type, not Derivation"),
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_core_functions_with_type_variables_parse() {
        // Test that we can load core extensions that contain functions with type variables
        use crate::parse::text::simple_extensions::Registry;
        use crate::urn::Urn;
        use std::str::FromStr;

        let registry = Registry::from_core_extensions();

        // Test a function from functions_comparison that uses any1
        let comparison_urn = Urn::from_str("extension:io.substrait:functions_comparison").unwrap();
        if let Some(func) = registry.get_scalar_function(&comparison_urn, "equal") {
            assert_eq!(func.name, "equal");
            assert!(
                !func.impls.is_empty(),
                "equal function should have implementations"
            );

            // Check that at least one implementation has a return type
            // Just verify it parsed without checking the exact structure
            // since different versions may have different signatures
            assert!(
                !func.impls.is_empty(),
                "Function should have at least one implementation with a return type"
            );
        }

        // Test a function from functions_arithmetic that uses decimal<P,S>
        let arithmetic_urn = Urn::from_str("extension:io.substrait:functions_arithmetic").unwrap();
        if let Some(func) = registry.get_scalar_function(&arithmetic_urn, "add") {
            assert_eq!(func.name, "add");
            assert!(
                !func.impls.is_empty(),
                "add function should have implementations"
            );
        }
    }
}
