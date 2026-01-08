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
            .filter_map(|(orig_urn, simple_extensions)| {
                // Skip extensions with bugs where they reference extension types without u! prefix
                // See: https://github.com/substrait-io/substrait/issues/903
                let urn_str = orig_urn.to_string();
                if urn_str == "extension:io.substrait:extension_types" ||
                   urn_str == "extension:io.substrait:unknown" {
                    return None;
                }

                let ExtensionFile { urn, extension } = ExtensionFile::create(simple_extensions.clone())
                    .unwrap_or_else(|err| panic!("Core extensions should be valid, but failed to create extension file for {orig_urn}: {err}"));
                debug_assert_eq!(orig_urn, &urn);
                Some((urn, extension))
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
    pub fn get_scalar_function(&self, urn: &Urn, name: &str) -> Option<&super::ScalarFunction> {
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

        // Test that functions_geometry.yaml loaded correctly with its geometry type
        let urn = Urn::from_str("extension:io.substrait:functions_geometry").unwrap();
        let core_extension = registry
            .get_extension(&urn)
            .expect("Should find functions_geometry extension");

        let geometry_type = core_extension.get_type("geometry");
        assert!(
            geometry_type.is_some(),
            "Should find 'geometry' type in functions_geometry extension"
        );

        // Also test the registry's get_type method with the actual URN
        let type_via_registry = registry.get_type(&urn, "geometry");
        assert!(type_via_registry.is_some());

        // Verify extension_types is skipped due to u! prefix bug
        let extension_types_urn = Urn::from_str("extension:io.substrait:extension_types").unwrap();
        assert!(
            registry.get_extension(&extension_types_urn).is_none(),
            "extension_types should be skipped due to missing u! prefix bug"
        );
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_scalar_function_lookup() {
        let registry = Registry::from_core_extensions();

        let functions_arithmetic_urn =
            Urn::from_str("extension:io.substrait:functions_arithmetic").unwrap();

        let add_function = registry.get_scalar_function(&functions_arithmetic_urn, "add");
        assert!(
            add_function.is_some(),
            "Should find 'add' function in functions_arithmetic extension"
        );

        let add = add_function.unwrap();
        assert_eq!(add.name, "add");
        assert!(
            !add.impls.is_empty(),
            "add function should have implementations"
        );

        let missing_function =
            registry.get_scalar_function(&functions_arithmetic_urn, "nonexistent");
        assert!(missing_function.is_none());

        let wrong_urn = Urn::from_str("extension:example.com:test").unwrap();
        let wrong_urn_lookup = registry.get_scalar_function(&wrong_urn, "add");
        assert!(wrong_urn_lookup.is_none());
    }

    #[test]
    fn test_unknown_type_without_prefix_fails() {
        use crate::text::simple_extensions;

        // Function that references a type without u! prefix - should fail with UnknownTypeName
        let invalid_extension = SimpleExtensions {
            scalar_functions: vec![simple_extensions::ScalarFunction {
                name: "bad_function".to_string(),
                description: None,
                impls: vec![simple_extensions::ScalarFunctionImplsItem {
                    args: None,
                    options: None,
                    variadic: None,
                    session_dependent: None,
                    deterministic: None,
                    nullability: None,
                    return_: simple_extensions::ReturnValue(simple_extensions::Type::String(
                        "point".to_string(), // Missing u! prefix - this is an error, not NYI
                    )),
                    implementation: None,
                }],
            }],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types: vec![],
            urn: "extension:example.com:invalid".to_string(),
        };

        let result = ExtensionFile::create(invalid_extension);
        assert!(
            result.is_err(),
            "Should fail when type is missing u! prefix"
        );

        match result {
            Err(crate::parse::text::simple_extensions::SimpleExtensionsError::ScalarFunctionError(
                crate::parse::text::simple_extensions::scalar_functions::ScalarFunctionError::TypeError(
                    crate::parse::text::simple_extensions::types::ExtensionTypeError::UnknownTypeName { name }
                )
            )) => {
                assert_eq!(name, "point");
            }
            other => panic!("Expected UnknownTypeName error, got {:?}", other),
        }
    }

    #[test]
    fn test_custom_type_validation() {
        use crate::text::simple_extensions;

        // Extension with type and function that references it with u! prefix - should succeed
        let valid_extension = SimpleExtensions {
            scalar_functions: vec![simple_extensions::ScalarFunction {
                name: "get_point".to_string(),
                description: None,
                impls: vec![simple_extensions::ScalarFunctionImplsItem {
                    args: None,
                    options: None,
                    variadic: None,
                    session_dependent: None,
                    deterministic: None,
                    nullability: None,
                    return_: simple_extensions::ReturnValue(simple_extensions::Type::String(
                        "u!point".to_string(), // Extension types MUST use u! prefix
                    )),
                    implementation: None,
                }],
            }],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types: vec![SimpleExtensionsTypesItem {
                name: "point".to_string(),
                description: None,
                parameters: None,
                structure: None,
                variadic: None,
            }],
            urn: "extension:example.com:valid".to_string(),
        };

        // Extension with function that references non-existent type - should fail
        let invalid_extension = SimpleExtensions {
            scalar_functions: vec![simple_extensions::ScalarFunction {
                name: "get_rectangle".to_string(),
                description: None,
                impls: vec![simple_extensions::ScalarFunctionImplsItem {
                    args: None,
                    options: None,
                    variadic: None,
                    session_dependent: None,
                    deterministic: None,
                    nullability: None,
                    return_: simple_extensions::ReturnValue(simple_extensions::Type::String(
                        "u!rectangle".to_string(), // This type doesn't exist
                    )),
                    implementation: None,
                }],
            }],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            type_variations: vec![],
            types: vec![],
            urn: "extension:example.com:invalid".to_string(),
        };

        // Valid case: type exists in same extension
        let valid_result = ExtensionFile::create(valid_extension);
        assert!(
            valid_result.is_ok(),
            "Should succeed when referenced type exists with u! prefix"
        );

        // Invalid case: type doesn't exist
        let invalid_result = ExtensionFile::create(invalid_extension);
        assert!(
            invalid_result.is_err(),
            "Should fail when referenced type doesn't exist"
        );
        match invalid_result {
            Err(crate::parse::text::simple_extensions::SimpleExtensionsError::UnresolvedTypeReference { type_name }) => {
                assert_eq!(type_name, "rectangle");
            }
            other => panic!("Expected UnresolvedTypeReference error, got {:?}", other),
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_scalar_function_parses_completely() {
        use super::super::{
            argument::ArgumentsItem, scalar_functions::{Impl, Options}, types::*,
        };
        use crate::parse::Parse;
        use crate::text::simple_extensions;
        use std::collections::HashMap;

        let registry = Registry::from_core_extensions();
        let functions_arithmetic_urn =
            Urn::from_str("extension:io.substrait:functions_arithmetic").unwrap();

        // Get the "add" function from functions_arithmetic.yaml
        let add = registry
            .get_scalar_function(&functions_arithmetic_urn, "add")
            .expect("add function should exist");

        // Verify function-level metadata
        assert_eq!(add.name, "add");
        assert_eq!(add.description, Some("Add two values.".to_string()));
        assert!(
            !add.impls.is_empty(),
            "add should have at least one implementation"
        );

        // Create the expected first implementation (i8 + i8 -> i8)
        let mut ctx = super::super::extensions::TypeContext::default();
        let expected_impl = Impl {
            args: vec![
                ArgumentsItem::ValueArgument(
                    simple_extensions::ValueArg {
                        name: Some("x".to_string()),
                        description: None,
                        value: simple_extensions::Type::String("i8".to_string()),
                        constant: None,
                    }
                    .parse(&mut ctx)
                    .unwrap(),
                ),
                ArgumentsItem::ValueArgument(
                    simple_extensions::ValueArg {
                        name: Some("y".to_string()),
                        description: None,
                        value: simple_extensions::Type::String("i8".to_string()),
                        constant: None,
                    }
                    .parse(&mut ctx)
                    .unwrap(),
                ),
            ],
            options: Some(Options({
                let mut map = HashMap::new();
                map.insert(
                    "overflow".to_string(),
                    vec![
                        "SILENT".to_string(),
                        "SATURATE".to_string(),
                        "ERROR".to_string(),
                    ],
                );
                map
            })),
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_type: ConcreteType {
                kind: ConcreteTypeKind::Builtin(BasicBuiltinType::I8),
                nullable: false,
            },
            implementation: None,
        };

        // Compare the actual first implementation to the expected
        assert_eq!(&add.impls[0], &expected_impl);
    }
}
