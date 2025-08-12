// SPDX-License-Identifier: Apache-2.0

//! Validated extension file wrapper.
//!
//! This module provides `ExtensionFile`, a validated wrapper around SimpleExtensions
//! that ensures extension data is valid on construction and provides safe accessor methods.

use thiserror::Error;
use url::Url;

use crate::text::simple_extensions::{
    AggregateFunction, AggregateFunctionImplsItem, Arguments, ReturnValue, ScalarFunction,
    ScalarFunctionImplsItem, SimpleExtensions, WindowFunction, WindowFunctionImplsItem,
};

/// Errors that can occur during extension validation
#[derive(Debug, Error, PartialEq)]
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
    // TODO: Add more validation errors for malformed argument patterns, return type patterns, etc.
}

/// A validated extension file containing functions and types from a single URI
#[derive(Debug)]
pub struct ExtensionFile {
    /// The URI this extension was loaded from
    pub uri: Url,
    /// The validated extension data
    extensions: SimpleExtensions,
}

impl ExtensionFile {
    /// Create a validated extension file from raw data
    pub fn create(uri: Url, extensions: SimpleExtensions) -> Result<Self, ValidationError> {
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
            .map(|f| ScalarFunctionRef(&self.uri, f))
    }

    /// Find an aggregate function by name
    pub fn find_aggregate_function(&self, name: &str) -> Option<AggregateFunctionRef> {
        self.extensions
            .aggregate_functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| AggregateFunctionRef(&self.uri, f))
    }

    /// Find a window function by name
    pub fn find_window_function(&self, name: &str) -> Option<WindowFunctionRef> {
        self.extensions
            .window_functions
            .iter()
            .find(|f| f.name == name)
            .map(|f| WindowFunctionRef(&self.uri, f))
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
pub struct ScalarFunctionRef<'a>(&'a Url, &'a ScalarFunction);

impl<'a> ScalarFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.1.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(&self) -> impl Iterator<Item = ScalarFunctionImplRef<'_>> {
        self.1
            .impls
            .iter()
            .map(move |impl_item| ScalarFunctionImplRef(self.0, impl_item))
    }
}

/// Handle for a validated aggregate function definition
pub struct AggregateFunctionRef<'a>(&'a Url, &'a AggregateFunction);

impl<'a> AggregateFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.1.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(&self) -> impl Iterator<Item = AggregateFunctionImplRef<'_>> {
        self.1
            .impls
            .iter()
            .map(move |impl_item| AggregateFunctionImplRef(self.0, impl_item))
    }
}

/// Handle for a validated window function definition
pub struct WindowFunctionRef<'a>(&'a Url, &'a WindowFunction);

impl<'a> WindowFunctionRef<'a> {
    /// Get the function name
    pub fn name(&self) -> &str {
        &self.1.name
    }

    /// Get all implementations as handles to specific type signatures
    pub fn implementations(&self) -> impl Iterator<Item = WindowFunctionImplRef<'_>> {
        self.1
            .impls
            .iter()
            .map(move |impl_item| WindowFunctionImplRef(self.0, impl_item))
    }
}

/// Handle for a specific scalar function implementation with validated signature
#[derive(Debug, Copy, Clone)]
pub struct ScalarFunctionImplRef<'a>(&'a Url, &'a ScalarFunctionImplsItem);

impl<'a> ScalarFunctionImplRef<'a> {
    /// Get the argument signature (guaranteed to be present due to validation)
    pub fn args(&self) -> &Arguments {
        self.1.args.as_ref().expect("validated to be present")
    }

    /// Get the return type pattern
    pub fn return_type(&self) -> &ReturnValue {
        &self.1.return_
    }
}

/// Handle for a specific aggregate function implementation with validated signature
pub struct AggregateFunctionImplRef<'a>(&'a Url, &'a AggregateFunctionImplsItem);

impl<'a> AggregateFunctionImplRef<'a> {
    /// Get the argument signature (guaranteed to be present due to validation)
    pub fn args(&self) -> &Arguments {
        self.1.args.as_ref().expect("validated to be present")
    }

    /// Get the return type pattern
    pub fn return_type(&self) -> &ReturnValue {
        &self.1.return_
    }
}

/// Handle for a specific window function implementation with validated signature
pub struct WindowFunctionImplRef<'a>(&'a Url, &'a WindowFunctionImplsItem);

impl<'a> WindowFunctionImplRef<'a> {
    /// Get the argument signature (guaranteed to be present due to validation)
    pub fn args(&self) -> &Arguments {
        self.1.args.as_ref().expect("validated to be present")
    }

    /// Get the return type pattern
    pub fn return_type(&self) -> &ReturnValue {
        &self.1.return_
    }
}
