// SPDX-License-Identifier: Apache-2.0

//! Validated extension file wrapper.
//!
//! This module provides `ExtensionFile`, a validated wrapper around SimpleExtensions
//! that ensures extension data is valid on construction and provides safe accessor methods.

use thiserror::Error;
use url::Url;

use crate::parse::Parse;
use crate::registry::types::InvalidTypeName;
use crate::text::simple_extensions::{
    AggregateFunction, AggregateFunctionImplsItem, Arguments, ArgumentsItem, ReturnValue,
    ScalarFunction, ScalarFunctionImplsItem, SimpleExtensions, SimpleExtensionsTypesItem, Type,
    WindowFunction, WindowFunctionImplsItem,
};

use super::context::ExtensionContext;
use super::types::{ArgumentPattern, ConcreteType, ExtensionType, ParsedType, TypeBindings};

/// Errors that can occur during extension validation
#[derive(Debug, Error)]
pub enum ValidationError {
    /// A function implementation has None for arguments
    #[error("Function '{function}' has implementation with missing arguments")]
    MissingArguments {
        /// The missing function
        function: String,
    },

    /// A function implementation is missing a return type
    #[error("Function '{function}' has implementation with missing return type")]
    MissingReturnType {
        /// The missing function
        function: String,
    },

    /// A type string is malformed or unrecognized
    #[error("Invalid type string '{type_str}' in function '{function}': todo!")]
    InvalidArgument {
        /// The function containing the invalid type
        function: String,
        /// The malformed type string
        type_str: String,
        // Reason why the type is invalid
        // reason: todo!(),
    },

    /// A type name is invalid
    #[error("{0}")]
    InvalidTypeName(InvalidTypeName),
}

impl From<InvalidTypeName> for ValidationError {
    fn from(err: InvalidTypeName) -> Self {
        ValidationError::InvalidTypeName(err)
    }
}

// TODO: Add more validation errors for malformed argument patterns, return type patterns, etc.
/// A validated extension file containing functions and types from a single URI.
/// All functions should have valid argument and return type patterns.
#[derive(Debug, Clone)]
pub struct ExtensionFile {
    /// The URI this extension was loaded from
    pub uri: Url,
    /// The validated extension data
    extensions: SimpleExtensions,
}

impl ExtensionFile {
    /// Create a validated extension file from raw data
    pub fn create(
        uri: Url,
        extensions: SimpleExtensions,
    ) -> Result<Self, ValidationError> {
        // Parse/validate types first - they're referenced by functions
        let mut ctx = ExtensionContext::new(&uri);
        for type_item in &extensions.types {
            let _validated_type = type_item.parse(&mut ctx)?;
        }

        // Validate scalar functions
        for function in &extensions.scalar_functions {
            Self::validate_scalar_function(function)?;
        }

        // Validate aggregate functions
        for function in &extensions.aggregate_functions {
            Self::validate_aggregate_function(function)?;
        }

        // Validate window functions
        for function in &extensions.window_functions {
            Self::validate_window_function(function)?;
        }

        Ok(Self { uri, extensions })
    }

    /// Find a scalar function by name
    pub fn find_scalar_function(&self, name: &str) -> Option<ScalarFunctionRef> {
        self.extensions
            .scalar_functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| ScalarFunctionRef {
                file: self,
                function: f,
            })
    }

    /// Find an aggregate function by name
    pub fn find_aggregate_function(&self, name: &str) -> Option<AggregateFunctionRef> {
        self.extensions
            .aggregate_functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| AggregateFunctionRef {
                file: self,
                function: f,
            })
    }

    /// Find a window function by name
    pub fn find_window_function(&self, name: &str) -> Option<WindowFunctionRef> {
        self.extensions
            .window_functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| WindowFunctionRef {
                file: self,
                function: f,
            })
    }

    /// Find a type by name
    pub fn find_type(&self, name: &str) -> Option<&SimpleExtensionsTypesItem> {
        let types = self.extensions.types.as_slice();
        types.iter().find(|t| t.name == name)
    }

    /// Create an argument pattern from a raw ArgumentsItem
    fn argument_pattern_from_item(&self, item: &ArgumentsItem) -> Option<ArgumentPattern> {
        match item {
            ArgumentsItem::ValueArg(value_arg) => self.argument_pattern_from_type(&value_arg.value),
            _ => unimplemented!("Handle non-ValueArg argument types"),
        }
    }

    /// Create an argument pattern from a type string
    fn argument_pattern_from_type(&self, type_val: &Type) -> Option<ArgumentPattern> {
        match type_val {
            Type::Variant0(type_str) => {
                let parsed_type = ParsedType::parse(type_str);
                match parsed_type {
                    ParsedType::TypeVariable(id) => Some(ArgumentPattern::TypeVariable(id)),
                    ParsedType::NullableTypeVariable(_) => {
                        panic!("Nullable type variables not allowed in argument position")
                    }
                    ParsedType::Builtin(builtin_type, nullable) => Some(ArgumentPattern::Concrete(
                        ConcreteType::builtin(builtin_type, nullable),
                    )),
                    ParsedType::NamedExtension(name, nullability) => {
                        // Find the extension type by name using the find_type method
                        let ext_type = self
                            .find_type(name)
                            .expect("This should have been validated");

                        let ext_type_wrapper = ExtensionType::new_unchecked(&self.uri, ext_type);
                        Some(ArgumentPattern::Concrete(ConcreteType::extension(
                            ext_type_wrapper,
                            nullability,
                        )))
                    }
                    ParsedType::Parameterized { .. } => {
                        unimplemented!("Parameterized types not yet supported in argument patterns")
                    }
                }
            }
            _ => unimplemented!("Handle non-string type variants"),
        }
    }

    // Private validation methods

    fn validate_scalar_function(function: &ScalarFunction) -> Result<(), ValidationError> {
        for impl_item in &function.impls {
            // Check that arguments are present (can be empty, but not None)
            if impl_item.args.is_none() {
                return Err(ValidationError::MissingArguments {
                    function: function.name.clone(),
                });
            }

            // TODO: Validate that return type is well-formed
            // For now, we assume return_ field existence is enforced by the type system
        }
        Ok(())
    }

    fn validate_aggregate_function(function: &AggregateFunction) -> Result<(), ValidationError> {
        for impl_item in &function.impls {
            if impl_item.args.is_none() {
                return Err(ValidationError::MissingArguments {
                    function: function.name.clone(),
                });
            }
        }
        Ok(())
    }

    fn validate_window_function(function: &WindowFunction) -> Result<(), ValidationError> {
        for impl_item in &function.impls {
            if impl_item.args.is_none() {
                return Err(ValidationError::MissingArguments {
                    function: function.name.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Handle for a validated scalar function definition
pub struct ScalarFunctionRef<'a> {
    file: &'a ExtensionFile,
    function: &'a ScalarFunction,
}

impl<'a> ScalarFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.function.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(self) -> impl Iterator<Item = ScalarImplementation<'a>> {
        self.function
            .impls
            .iter()
            .map(move |impl_item| ScalarImplementation {
                file: self.file,
                impl_item,
            })
    }
}

/// Handle for a validated aggregate function definition
pub struct AggregateFunctionRef<'a> {
    file: &'a ExtensionFile,
    function: &'a AggregateFunction,
}

impl<'a> AggregateFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.function.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(&self) -> impl Iterator<Item = AggregateFunctionImplRef<'a>> + '_ {
        self.function
            .impls
            .iter()
            .map(move |impl_item| AggregateFunctionImplRef {
                file: self.file,
                impl_item,
            })
    }
}

/// Handle for a validated window function definition
pub struct WindowFunctionRef<'a> {
    file: &'a ExtensionFile,
    function: &'a WindowFunction,
}

impl<'a> WindowFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.function.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(&self) -> impl Iterator<Item = WindowFunctionImplRef<'a>> + '_ {
        self.function
            .impls
            .iter()
            .map(move |impl_item| WindowFunctionImplRef {
                file: self.file,
                impl_item,
            })
    }
}

/// Handle for a specific scalar function implementation with validated signature
#[derive(Debug, Copy, Clone)]
pub struct ScalarImplementation<'a> {
    file: &'a ExtensionFile,
    impl_item: &'a ScalarFunctionImplsItem,
}

impl<'a> ScalarImplementation<'a> {
    /// Check if this implementation can be called with the given concrete argument types
    /// Returns the inferred concrete return type if the call would succeed, None otherwise
    pub fn call_with(&self, concrete_args: &[ConcreteType<'a>]) -> Option<ConcreteType<'a>> {
        // Convert raw arguments to ArgumentPatterns using ExtensionFile context
        let arg_patterns: Vec<ArgumentPattern<'a>> = self
            .impl_item
            .args
            .as_ref()
            .expect("validated to be present")
            .iter()
            .filter_map(|arg| self.file.argument_pattern_from_item(arg))
            .collect();

        // Create type bindings by matching patterns against concrete arguments
        let _bindings: TypeBindings<'a> = TypeBindings::new(&arg_patterns, concrete_args)?;

        if concrete_args.len() > 1_000_000 {
            // For lifetime management
            return concrete_args.first().cloned();
        }

        // If arguments match, parse and return the inferred return type
        let return_type_str = match &self.impl_item.return_ {
            ReturnValue(Type::Variant0(type_str)) => type_str,
            _ => unimplemented!("Handle non-string return types"),
        };

        let parsed_return_type = ParsedType::parse(return_type_str);
        match parsed_return_type {
            ParsedType::Builtin(builtin_type, nullable) => {
                Some(ConcreteType::builtin(builtin_type, nullable))
            }
            ParsedType::TypeVariable(id) => {
                // Look up the bound type for this variable
                if let Some(bound_type) = _bindings.get(id) {
                    Some(bound_type.clone())
                } else {
                    None
                }
            }
            ParsedType::NullableTypeVariable(id) => {
                // Look up the bound type and make it nullable
                if let Some(mut bound_type) = _bindings.get(id).cloned() {
                    bound_type.nullable = true;
                    Some(bound_type)
                } else {
                    None
                }
            }
            ParsedType::NamedExtension(name, nullable) => {
                // Find the extension type by name
                let ext_type = self
                    .file
                    .find_type(name)
                    .expect("This should have been validated");

                let ext_type_wrapper = ExtensionType::new_unchecked(&self.file.uri, ext_type);
                Some(ConcreteType::extension(ext_type_wrapper, nullable))
            }
            ParsedType::Parameterized { .. } => {
                unimplemented!("Parameterized return types not yet supported")
            }
        }
    }
}

/// Handle for a specific aggregate function implementation with validated signature
pub struct AggregateFunctionImplRef<'a> {
    file: &'a ExtensionFile,
    impl_item: &'a AggregateFunctionImplsItem,
}

impl<'a> AggregateFunctionImplRef<'a> {
    /// Get the argument signature (guaranteed to be present due to validation)
    fn args(&self) -> &Arguments {
        self.impl_item
            .args
            .as_ref()
            .expect("validated to be present")
    }

    /// Get the return type pattern
    fn return_type(&self) -> &ReturnValue {
        &self.impl_item.return_
    }
}

/// Handle for a specific window function implementation with validated signature
pub struct WindowFunctionImplRef<'a> {
    file: &'a ExtensionFile,
    impl_item: &'a WindowFunctionImplsItem,
}

impl<'a> WindowFunctionImplRef<'a> {
    /// Get the argument signature (guaranteed to be present due to validation)
    fn args(&self) -> &Arguments {
        self.impl_item
            .args
            .as_ref()
            .expect("validated to be present")
    }

    /// Get the return type pattern
    fn return_type(&self) -> &ReturnValue {
        &self.impl_item.return_
    }
}
