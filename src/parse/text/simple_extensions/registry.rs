// SPDX-License-Identifier: Apache-2.0

//! Substrait Extension Registry
//!
//! This module provides registries for Substrait extensions:
//! - **Global Registry**: Immutable, reusable across plans, URI+name based lookup
//! - **Local Registry**: Per-plan, anchor-based, references Global Registry (TODO)
//!
//! Currently only type definitions are supported. Function parsing will be added in a future update.
//!
//! This module is only available when the `parse` feature is enabled.

#![cfg(feature = "parse")]

use super::{ExtensionFile, types::CustomType};
use crate::urn::Urn;

/// Extension Registry that manages Substrait extensions
///
/// This registry is immutable and reusable across multiple plans.
/// It provides URN + name based lookup for extension types. Function parsing will be added in a future update.
#[derive(Debug)]
pub struct Registry {
    /// Pre-validated extension files
    extensions: Vec<ExtensionFile>,
}

impl Registry {
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

        // Convert HashMap<Urn, SimpleExtensions> to Vec<SimpleExtensions>
        let extensions: Vec<ExtensionFile> = EXTENSIONS
            .iter()
            .map(|(urn, simple_extensions)| {
                let extension_file = ExtensionFile::create(simple_extensions.clone())
                    .unwrap_or_else(|err| panic!("Core extensions should be valid, but failed to create extension file for {urn}: {err}"));
                debug_assert_eq!(extension_file.urn(), urn);
                extension_file
            })
            .collect();

        Self { extensions }
    }

    // Private helper methods

    fn get_extension(&self, urn: &Urn) -> Option<&ExtensionFile> {
        self.extensions.iter().find(|ext| ext.urn() == urn)
    }

    /// Get a type by URI and name
    pub fn get_type(&self, urn: &Urn, name: &str) -> Option<&CustomType> {
        self.get_extension(urn)?.get_type(name)
    }
}

#[cfg(test)]
mod tests {
    use super::ExtensionFile as ParsedSimpleExtensions;
    use super::Registry;
    use crate::text::simple_extensions::{SimpleExtensions, SimpleExtensionsTypesItem};
    use crate::urn::Urn;
    use std::str::FromStr;

    fn create_test_extension_with_types() -> SimpleExtensions {
        SimpleExtensions {
            scalar_functions: vec![],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types: vec![SimpleExtensionsTypesItem {
                name: "test_type".to_string(),
                description: Some("A test type".to_string()),
                parameters: None,
                structure: None,
                variadic: None,
            }],
            urn: "extension:example.com:test".to_string(),
        }
    }

    #[test]
    fn test_new_registry() {
        let urn = Urn::from_str("extension:example.com:test").unwrap();
        let extension_file =
            ParsedSimpleExtensions::create(create_test_extension_with_types()).unwrap();
        let extensions = vec![extension_file];

        let registry = Registry::new(extensions);
        assert_eq!(registry.extensions().count(), 1);
        let extension_urns: Vec<&Urn> = registry.extensions().map(|ext| ext.urn()).collect();
        assert!(extension_urns.contains(&&urn));
    }

    #[test]
    fn test_type_lookup() {
        let urn = Urn::from_str("extension:example.com:test").unwrap();
        let extension_file =
            ParsedSimpleExtensions::create(create_test_extension_with_types()).unwrap();
        let extensions = vec![extension_file];

        let registry = Registry::new(extensions);

        // Test successful type lookup
        let found_type = registry.get_type(&urn, "test_type");
        assert!(found_type.is_some());
        assert_eq!(found_type.unwrap().name, "test_type");

        // Test missing type lookup
        let missing_type = registry.get_type(&urn, "nonexistent_type");
        assert!(missing_type.is_none());

        // Test missing extension lookup
        let wrong_urn = Urn::from_str("extension:example.com:wrong").unwrap();
        let missing_extension = registry.get_type(&wrong_urn, "test_type");
        assert!(missing_extension.is_none());
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_from_core_extensions() {
        let registry = Registry::from_core_extensions();
        assert!(registry.extensions().count() > 0);

        // Find the unknown.yaml extension dynamically
        let unknown_extension = registry
            .extensions()
            .find(|ext| ext.urn().to_string() == "extension:io.substrait:unknown")
            .expect("Should find unknown extension");

        let unknown_type = unknown_extension.get_type("unknown");
        assert!(
            unknown_type.is_some(),
            "Should find 'unknown' type in unknown.yaml extension"
        );

        // Also test the registry's get_type method with the actual URI
        let unknown_type_via_registry = registry.get_type(unknown_extension.urn(), "unknown");
        assert!(unknown_type_via_registry.is_some());
    }
}
