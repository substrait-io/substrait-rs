// SPDX-License-Identifier: Apache-2.0

//! Substrait Extension Registry
//!
//! This module provides registries for Substrait extensions:
//! - **Global Registry**: Immutable, reusable across plans, URI+name based lookup
//! - **Local Registry**: Per-plan, anchor-based, references Global Registry (TODO)
//!
//! This module is only available when the `parse` feature is enabled.

use std::collections::{HashMap, hash_map::Entry};

use super::{
    ExtensionFile, SimpleExtensions, SimpleExtensionsError, scalar_functions::ScalarFunction,
    types::CustomType,
};
use crate::urn::Urn;

/// Extension Registry that manages Substrait extensions
///
/// This registry is immutable and reusable across multiple plans.
/// It provides URN + name based lookup for extension types and scalar functions.
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
            .filter_map(|(orig_urn, simple_extensions)| {
                match ExtensionFile::create(simple_extensions.clone()) {
                    Ok(ExtensionFile { urn, extension }) => {
                        debug_assert_eq!(orig_urn, &urn);
                        Some((urn, extension))
                    }
                    Err(err) => {
                        // Skip extensions that fail to parse (e.g., those with unsupported type parameter variables)
                        eprintln!("Warning: Skipping extension {orig_urn}: {err}");
                        None
                    }
                }
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

    /// Get a scalar function by URN and name
    pub fn get_scalar_function(&self, urn: &Urn, name: &str) -> Option<&ScalarFunction> {
        self.get_extension(urn)?.get_scalar_function(name)
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

        let types_urn = Urn::from_str("extension:io.substrait:extension_types").unwrap();
        let core_extension = registry
            .get_extension(&types_urn)
            .expect("Should find extension_types extension");

        let point_type = core_extension.get_type("point");
        assert!(
            point_type.is_some(),
            "Should find 'point' type in unknown.yaml extension"
        );

        let type_via_registry = registry.get_type(&types_urn, "point");
        assert!(type_via_registry.is_some());

        // Test scalar function lookup
        let functions_urn = Urn::from_str("extension:io.substrait:functions_boolean").unwrap();
        let or_function = registry.get_scalar_function(&functions_urn, "or");
        assert!(
            or_function.is_some(),
            "Should find 'or' function in functions_boolean extension"
        );

        let and_function = registry.get_scalar_function(&functions_urn, "and");
        assert!(and_function.is_some());
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_parse_scalar_function_with_variadic() {
        use std::fs;

        // Load and parse the boolean functions extension directly
        let yaml_content = fs::read_to_string("substrait/extensions/functions_boolean.yaml")
            .expect("Should read functions_boolean.yaml");
        let raw: crate::text::simple_extensions::SimpleExtensions =
            serde_yaml::from_str(&yaml_content).expect("Should parse YAML");

        let extension_file = ExtensionFile::create(raw).expect("Should create extension file");

        // Test the "or" function which has variadic behavior
        let or_function = extension_file
            .extension
            .get_scalar_function("or")
            .expect("Should find 'or' function");

        assert_eq!(or_function.name, "or");
        assert!(or_function.description.is_some());
        assert_eq!(or_function.impls.len(), 1);

        let impl_ = &or_function.impls[0];
        assert!(impl_.variadic.is_some());

        let variadic = impl_.variadic.as_ref().unwrap();
        assert_eq!(variadic.min, Some(0));
        assert_eq!(variadic.max, None);
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_parse_scalar_function_with_options() {
        use std::fs;

        // Load and parse the arithmetic functions extension directly
        let yaml_content = fs::read_to_string("substrait/extensions/functions_arithmetic.yaml")
            .expect("Should read functions_arithmetic.yaml");
        let raw: crate::text::simple_extensions::SimpleExtensions =
            serde_yaml::from_str(&yaml_content).expect("Should parse YAML");

        let extension_file = ExtensionFile::create(raw).expect("Should create extension file");

        // Test the "add" function which has options
        let add_function = extension_file
            .extension
            .get_scalar_function("add")
            .expect("Should find 'add' function");

        assert_eq!(add_function.name, "add");
        assert!(add_function.description.is_some());
        assert!(!add_function.impls.is_empty());

        // Check that at least one implementation has options
        let impl_with_options = add_function
            .impls
            .iter()
            .find(|i| i.options.is_some())
            .expect("Should find at least one impl with options");

        let options = impl_with_options.options.as_ref().unwrap();
        assert!(
            options.0.contains_key("overflow") || options.0.contains_key("rounding"),
            "Should have overflow or rounding option"
        );
    }
}
