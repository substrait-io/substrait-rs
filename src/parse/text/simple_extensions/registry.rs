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

use std::collections::{HashMap, hash_map::Entry};

use super::{ExtensionFile, SimpleExtensions, SimpleExtensionsError, types::CustomType};
use crate::urn::Urn;

/// Extension Registry that manages Substrait extensions
///
/// This registry is immutable and reusable across multiple plans.
/// It provides URN + name based lookup for extension types. Function parsing will be added in a future update.
#[derive(Debug)]
pub struct Registry {
    /// Pre-validated extension files
    extensions: HashMap<Urn, SimpleExtensions>,
}

impl Registry {
    /// Create a new Global Registry from validated extension files.
    ///
    /// Any duplicate URNs will raise an error.
    pub fn new<I: IntoIterator<Item = ExtensionFile>>(
        extensions: I,
    ) -> Result<Self, SimpleExtensionsError> {
        let mut map = HashMap::new();
        for ExtensionFile { urn, extension } in extensions {
            match map.entry(urn.clone()) {
                Entry::Occupied(_) => return Err(SimpleExtensionsError::DuplicateUrn(urn)),
                Entry::Vacant(entry) => {
                    entry.insert(extension);
                }
            }
        }
        Ok(Self { extensions: map })
    }

    /// Get an iterator over all extension files in this registry
    pub fn extensions(&self) -> impl Iterator<Item = (&Urn, &SimpleExtensions)> {
        self.extensions.iter()
    }

    /// Create a Global Registry from the built-in core extensions
    #[cfg(feature = "extensions")]
    pub fn from_core_extensions() -> Self {
        use crate::extensions::EXTENSIONS;

        // Parse the core extensions from the raw extensions format to the parsed format
        let extensions: HashMap<Urn, SimpleExtensions> = EXTENSIONS
            .iter()
            .map(|(orig_urn, simple_extensions)| {
                let ExtensionFile { urn, extension } = ExtensionFile::create(simple_extensions.clone())
                    .unwrap_or_else(|err| panic!("Core extensions should be valid, but failed to create extension file for {orig_urn}: {err}"));
                debug_assert_eq!(orig_urn, &urn);
                (urn, extension)
            })
            .collect();

        Self { extensions }
    }

    fn get_extension(&self, urn: &Urn) -> Option<&SimpleExtensions> {
        self.extensions.get(urn)
    }

    /// Get a type by URN and name
    pub fn get_type(&self, urn: &Urn, name: &str) -> Option<&CustomType> {
        self.get_extension(urn)?.get_type(name)
    }
}

#[cfg(test)]
mod tests {
    use super::{ExtensionFile, Registry};
    use crate::text::simple_extensions::{SimpleExtensions, SimpleExtensionsTypesItem};
    use crate::urn::Urn;
    use std::str::FromStr;

    fn extension_file(urn: &str, type_names: &[&str]) -> ExtensionFile {
        let types = type_names
            .iter()
            .map(|name| SimpleExtensionsTypesItem {
                name: (*name).to_string(),
                description: None,
                parameters: None,
                structure: None,
                variadic: None,
            })
            .collect();

        let raw = SimpleExtensions {
            scalar_functions: vec![],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types,
            urn: urn.to_string(),
        };

        ExtensionFile::create(raw).expect("valid extension file")
    }

    #[test]
    fn test_registry_iteration() {
        let urns = vec![
            "extension:example.com:first",
            "extension:example.com:second",
        ];
        let registry =
            Registry::new(urns.iter().map(|&urn| extension_file(urn, &["type"]))).unwrap();

        let collected: Vec<&Urn> = registry.extensions().map(|(urn, _)| urn).collect();
        assert_eq!(collected.len(), 2);
        for urn in urns {
            assert!(
                collected
                    .iter()
                    .any(|candidate| candidate.to_string() == urn)
            );
        }
    }

    #[test]
    fn test_type_lookup() {
        let urn = Urn::from_str("extension:example.com:test").unwrap();
        let registry =
            Registry::new(vec![extension_file(&urn.to_string(), &["test_type"])]).unwrap();
        let other_urn = Urn::from_str("extension:example.com:other").unwrap();

        let cases = vec![
            (&urn, "test_type", true),
            (&urn, "missing", false),
            (&other_urn, "test_type", false),
        ];

        for (query_urn, type_name, expected) in cases {
            assert_eq!(
                registry.get_type(query_urn, type_name).is_some(),
                expected,
                "unexpected lookup result for {query_urn}:{type_name}"
            );
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_from_core_extensions() {
        let registry = Registry::from_core_extensions();
        assert!(registry.extensions().count() > 0);

        // Find the unknown.yaml extension dynamically
        let urn = Urn::from_str("extension:io.substrait:extension_types").unwrap();
        let core_extension = registry
            .get_extension(&urn)
            .expect("Should find extension_types extension");

        let point_type = core_extension.get_type("point");
        assert!(
            point_type.is_some(),
            "Should find 'point' type in unknown.yaml extension"
        );

        // Also test the registry's get_type method with the actual URN
        let type_via_registry = registry.get_type(&urn, "point");
        assert!(type_via_registry.is_some());
    }
}
