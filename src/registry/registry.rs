// SPDX-License-Identifier: Apache-2.0

//! Substrait Extension Registry
//!
//! This module provides registries for Substrait extensions:
//! - **Global Registry**: Immutable, reusable across plans, URI+name based lookup
//! - **Local Registry**: Per-plan, anchor-based, references Global Registry (TODO)
//!
//! This module is only available when the `registry` feature is enabled.

#![cfg(feature = "registry")]

use thiserror::Error;
use url::Url;

use crate::registry::ScalarFunctionRef;

use super::{types::ConcreteType, ExtensionFile};

/// Errors that can occur when using the Global Registry
#[derive(Debug, Error, PartialEq)]
pub enum GlobalRegistryError {
    /// The specified extension URI is not registered in this registry
    #[error("Unknown extension URI: {0}")]
    UnknownExtensionUri(String),
    /// The specified function was not found in the given extension
    #[error("Function '{function}' not found in extension '{uri}'")]
    FunctionNotFound {
        /// The extension URI where the function was expected
        uri: String,
        /// The name of the function that was not found
        function: String,
    },
    /// No function signature matches the provided arguments
    #[error("No matching signature for function '{function}' in extension '{uri}' with provided arguments")]
    NoMatchingSignature {
        /// The extension URI containing the function
        uri: String,
        /// The name of the function with no matching signature
        function: String,
    },
}

impl GlobalRegistryError {
    /// Create a FunctionNotFound error
    pub fn not_found(uri: &Url, function: &str) -> Self {
        Self::FunctionNotFound {
            uri: uri.to_string(),
            function: function.to_string(),
        }
    }
}

/// Global Extension Registry that manages Substrait extensions
///
/// This registry is immutable and reusable across multiple plans.
/// It provides URI + name based lookup for function validation and signature matching.
#[derive(Debug)]
pub struct GlobalRegistry {
    /// Pre-validated extension files
    extensions: Vec<ExtensionFile>,
}

impl GlobalRegistry {
    /// Create a new Global Registry from validated extension files
    pub fn new(extensions: Vec<ExtensionFile>) -> Self {
        Self { extensions }
    }

    /// Get an iterator over all extension files in this registry
    pub fn extensions(&self) -> impl Iterator<Item = &ExtensionFile> {
        self.extensions.iter()
    }

    /// Create a Global Registry from the built-in core extensions
    #[cfg(feature = "extensions")]
    pub fn from_core_extensions() -> Self {
        use crate::extensions::EXTENSIONS;
        use std::sync::LazyLock;

        // Force evaluation of core extensions
        LazyLock::force(&EXTENSIONS);

        // Convert HashMap<Url, SimpleExtensions> to Vec<ExtensionFile>
        let extensions: Vec<ExtensionFile> = EXTENSIONS
            .iter()
            .map(|(uri, simple_extensions)| {
                ExtensionFile::create(uri.clone(), simple_extensions.clone())
                    .map_err(|err| {
                        eprintln!("Failed to create extension file for {}: {}", uri, err);
                    })
                    .expect("Core extensions should be valid")
            })
            .collect();

        Self { extensions }
    }

    // Private helper methods

    fn get_extension(&self, uri: &Url) -> Result<&ExtensionFile, GlobalRegistryError> {
        self.extensions
            .iter()
            .find(|ext| &ext.uri == uri)
            .ok_or_else(|| GlobalRegistryError::UnknownExtensionUri(uri.to_string()))
    }

    /// Validate a scalar function call and return the concrete return type
    pub fn validate_scalar_call<'a>(
        &'a self,
        uri: &Url,
        name: &str,
        args: &[ConcreteType],
    ) -> Result<ConcreteType, GlobalRegistryError> {
        let extension: &'a ExtensionFile = self.get_extension(uri)?;
        let function_ref: ScalarFunctionRef<'a> = extension
            .find_scalar_function(name)
            .ok_or_else(|| GlobalRegistryError::not_found(uri, name))?;

        // Try each implementation until one matches
        for impl_ref in function_ref.implementations() {
            if let Some(return_type) = impl_ref.call_with(args) {
                return Ok(return_type);
            }
        }

        Err(GlobalRegistryError::NoMatchingSignature {
            uri: uri.to_string(),
            function: name.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::types::{BuiltinType, ConcreteType};
    use crate::text::simple_extensions::*;

    fn create_test_extension() -> SimpleExtensions {
        SimpleExtensions {
            scalar_functions: vec![ScalarFunction {
                name: "add".to_string(),
                description: Some("Addition function".to_string()),
                impls: vec![ScalarFunctionImplsItem {
                    args: Some(Arguments(vec![])), // Simplified for testing. TODO: Add real args
                    return_: ReturnValue(Type::Variant0("i32".to_string())),
                    deterministic: None,
                    implementation: None,
                    nullability: None,
                    options: None,
                    session_dependent: None,
                    variadic: None,
                }],
            }],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types: vec![],
        }
    }

    #[test]
    fn test_new_registry() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let extension_file = ExtensionFile::create(uri.clone(), create_test_extension()).unwrap();
        let extensions = vec![extension_file];

        let registry = GlobalRegistry::new(extensions);
        assert_eq!(registry.extensions().count(), 1);
        let extension_uris: Vec<&Url> = registry.extensions().map(|ext| &ext.uri).collect();
        assert!(extension_uris.contains(&&uri));
    }

    #[test]
    fn test_validate_scalar_call_with_test_extension() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let extension_file = ExtensionFile::create(uri.clone(), create_test_extension()).unwrap();
        let extensions = vec![extension_file];

        let registry = GlobalRegistry::new(extensions);
        let args: &[ConcreteType] = &[]; // Empty ConcreteType args

        let result = registry.validate_scalar_call(&uri, "add", args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_standard_extension() {
        let registry = GlobalRegistry::from_core_extensions();
        let arithmetic_uri = Url::parse("https://github.com/substrait-io/substrait/raw/v0.57.0/extensions/functions_arithmetic.yaml").unwrap();

        // Test that add function fails with no arguments (should require 2 arguments)
        let no_args: &[ConcreteType] = &[];
        let result_no_args = registry.validate_scalar_call(&arithmetic_uri, "add", no_args);
        assert!(
            result_no_args.is_err(),
            "add function should fail with no arguments"
        );

        // Test that add function succeeds with two i32 arguments and returns i32
        let i32_args = &[
            ConcreteType::builtin(BuiltinType::I32, false),
            ConcreteType::builtin(BuiltinType::I32, false),
        ];
        let result_with_args = registry.validate_scalar_call(&arithmetic_uri, "add", i32_args);
        assert!(
            result_with_args.is_ok(),
            "add function should succeed with two i32 arguments"
        );

        // Verify it returns the correct concrete type (i32)
        let return_type = result_with_args.unwrap();
        assert_eq!(
            return_type,
            ConcreteType::builtin(BuiltinType::I32, false),
            "add(i32, i32) should return i32"
        );
    }

    #[test]
    fn test_from_core_extensions() {
        let registry = GlobalRegistry::from_core_extensions();
        assert!(registry.extensions().count() > 0);
    }
}
