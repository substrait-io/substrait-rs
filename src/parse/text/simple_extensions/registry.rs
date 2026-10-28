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

    /// Create a Global Registry from the built-in core extensions.
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

    /// Get a scalar function by URN and name.
    ///
    /// TODO: Add support for retrieving functions by their full signature shorthand
    /// (e.g., "add:i32_i32").
    pub fn get_scalar_function(&self, urn: &Urn, name: &str) -> Option<&super::ScalarFunction> {
        self.get_extension(urn)?.get_scalar_function(name)
    }
}

#[cfg(test)]
mod tests {
    use super::{ExtensionFile, Registry};
    use crate::parse::text::simple_extensions::{
        SimpleExtensionsError, scalar_functions::ScalarFunctionError, types::ExtensionTypeError,
    };
    use crate::text::simple_extensions::{SimpleExtensions, SimpleExtensionsTypesItem};
    use crate::urn::Urn;
    use std::str::FromStr;

    fn extension_file(urn: &str, type_names: &[&str]) -> ExtensionFile {
        let types = type_names
            .iter()
            .map(|name| SimpleExtensionsTypesItem {
                name: (*name).to_string(),
                deprecated: None,
                description: None,
                metadata: Default::default(),
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
            metadata: Default::default(),
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

        // `unsigned_integers` was added to the catalog in Substrait v0.101.0.
        let unsigned_integers_urn =
            Urn::from_str("extension:io.substrait:unsigned_integers").unwrap();
        assert!(
            registry.get_extension(&unsigned_integers_urn).is_some(),
            "unsigned_integers should be a core extension"
        );
    }

    #[test]
    fn test_unknown_type_without_prefix_fails() {
        use crate::text::simple_extensions;

        // Function that references a type without u! prefix - should fail with UnknownTypeName
        let invalid_extension = SimpleExtensions {
            scalar_functions: vec![simple_extensions::ScalarFunction {
                name: "bad_function".to_string(),
                description: None,
                metadata: Default::default(),
                deprecated: None,
                impls: vec![simple_extensions::ScalarFunctionImplsItem {
                    args: None,
                    deprecated: None,
                    description: None,
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
            metadata: Default::default(),
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
            Err(SimpleExtensionsError::ScalarFunctionError(ScalarFunctionError::TypeError(
                ExtensionTypeError::UnknownTypeName { name },
            ))) => {
                assert_eq!(name, "point");
            }
            other => panic!("Expected UnknownTypeName error, got {:?}", other),
        }
    }

    /// Helper to create a minimal extension with a scalar function returning a custom type
    fn extension_with_custom_type_reference(
        urn: &str,
        function_name: &str,
        return_type: &str,
        defined_types: Vec<&str>,
    ) -> SimpleExtensions {
        use crate::text::simple_extensions;

        SimpleExtensions {
            scalar_functions: vec![simple_extensions::ScalarFunction {
                name: function_name.to_string(),
                description: None,
                metadata: Default::default(),
                deprecated: None,
                impls: vec![simple_extensions::ScalarFunctionImplsItem {
                    args: None,
                    deprecated: None,
                    description: None,
                    options: None,
                    variadic: None,
                    session_dependent: None,
                    deterministic: None,
                    nullability: None,
                    return_: simple_extensions::ReturnValue(simple_extensions::Type::String(
                        return_type.to_string(),
                    )),
                    implementation: None,
                }],
            }],
            aggregate_functions: vec![],
            window_functions: vec![],
            dependencies: Default::default(),
            metadata: Default::default(),
            type_variations: vec![],
            types: defined_types
                .into_iter()
                .map(|name| SimpleExtensionsTypesItem {
                    name: name.to_string(),
                    deprecated: None,
                    description: None,
                    metadata: Default::default(),
                    parameters: None,
                    structure: None,
                    variadic: None,
                })
                .collect(),
            urn: urn.to_string(),
        }
    }

    #[test]
    fn test_custom_type_reference_valid() {
        let extension = extension_with_custom_type_reference(
            "extension:example.com:valid",
            "get_point",
            "u!point",
            vec!["point"],
        );

        let result = ExtensionFile::create(extension);
        assert!(
            result.is_ok(),
            "Should succeed when referenced type exists with u! prefix"
        );
    }

    #[test]
    fn test_custom_type_reference_missing() {
        let extension = extension_with_custom_type_reference(
            "extension:example.com:invalid",
            "get_rectangle",
            "u!rectangle",
            vec![], // rectangle type not defined
        );

        let result = ExtensionFile::create(extension);
        assert!(
            result.is_err(),
            "Should fail when referenced type doesn't exist"
        );

        match result {
            Err(SimpleExtensionsError::UnresolvedTypeReference { type_name }) => {
                assert_eq!(type_name, "rectangle");
            }
            other => panic!("Expected UnresolvedTypeReference error, got {:?}", other),
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_core_functions_with_variable_returns() {
        let registry = Registry::from_core_extensions();
        let urn = Urn::from_str("extension:io.substrait:functions_comparison").unwrap();
        for name in ["coalesce", "nullif"] {
            assert!(
                registry.get_scalar_function(&urn, name).is_some(),
                "{name} should not be skipped because of its variable return type"
            );
        }
    }

    #[test]
    fn test_numbered_variable_returns() {
        use super::super::{Type, TypeKind, argument::ArgumentsItem};

        for (return_type, id, nullable) in [("any1", 1, false), ("any2?", 2, true)] {
            let yaml = format!(
                r#"urn: extension:example.com:variables
scalar_functions:
  - name: identity
    impls:
      - args:
          - value: {return_type}
        return: {return_type}
"#
            );
            let file = ExtensionFile::read_from_str(yaml).unwrap();
            let urn = file.urn().clone();
            let registry = Registry::new(vec![file]).unwrap();
            let function = registry.get_scalar_function(&urn, "identity").unwrap();
            assert_eq!(
                function.impls[0].return_type,
                Type {
                    kind: TypeKind::Variable { id },
                    nullable
                }
            );

            let ArgumentsItem::ValueArgument(argument) = function.impls[0].args[0].clone() else {
                panic!("expected value argument");
            };
            let raw: crate::text::simple_extensions::ValueArg = argument.into();
            assert_eq!(
                raw.value,
                crate::text::simple_extensions::Type::String(return_type.to_string())
            );
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_core_functions_with_numeric_parameter_returns() {
        let registry = Registry::from_core_extensions();
        let urn = Urn::from_str("extension:io.substrait:functions_string").unwrap();
        for name in ["lower", "upper"] {
            assert!(
                registry.get_scalar_function(&urn, name).is_some(),
                "{name} should not be skipped because of its named numeric parameters"
            );
        }
    }

    #[test]
    fn test_numeric_parameter_returns() {
        use super::super::{IntegerParameter, Type, TypeKind};

        let yaml = r#"urn: extension:example.com:numeric_parameters
scalar_functions:
  - name: upper
    impls:
      - args:
          - value: varchar<L>
        return: varchar<L>
"#;
        let file = ExtensionFile::read_from_str(yaml).unwrap();
        let urn = file.urn().clone();
        let registry = Registry::new(vec![file]).unwrap();
        let function = registry.get_scalar_function(&urn, "upper").unwrap();
        assert_eq!(
            function.impls[0].return_type,
            Type::new(
                TypeKind::VarChar {
                    length: IntegerParameter::Variable("L".to_string()),
                },
                false
            )
        );
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_core_function_with_nested_symbolic_return() {
        use super::super::{IntegerParameter, Type, TypeKind};

        let registry = Registry::from_core_extensions();
        let urn = Urn::from_str("extension:io.substrait:functions_string").unwrap();
        let function = registry
            .get_scalar_function(&urn, "regexp_match_substring_all")
            .expect("regexp_match_substring_all should not be skipped because it returns List<varchar<L1>>");
        assert_eq!(
            function.impls[0].return_type,
            Type::list(
                Type::new(
                    TypeKind::VarChar {
                        length: IntegerParameter::Variable("L1".to_string()),
                    },
                    false,
                ),
                false,
            )
        );
    }

    #[test]
    fn test_nested_return_preserves_object_arguments() {
        use super::super::{Type, TypeKind, argument::ArgumentsItem};
        use crate::text::simple_extensions as raw;

        let yaml = r#"urn: extension:example.com:nested
scalar_functions:
  - name: wrap_field
    impls:
      - args:
          - value: {field: any1}
        return: list<any1>
"#;
        let file = ExtensionFile::read_from_str(yaml).unwrap();
        let urn = file.urn().clone();
        let registry = Registry::new(vec![file]).unwrap();
        let function = registry.get_scalar_function(&urn, "wrap_field").unwrap();
        assert_eq!(
            function.impls[0].return_type,
            Type::list(Type::new(TypeKind::Variable { id: 1 }, false), false)
        );
        let ArgumentsItem::ValueArgument(argument) = function.impls[0].args[0].clone() else {
            panic!("expected value argument");
        };
        let argument: raw::ValueArg = argument.into();
        let raw::Type::Object(fields) = argument.value else {
            panic!("expected object-form type to be preserved");
        };
        assert_eq!(
            fields["field"],
            serde_json::Value::String("any1".to_string())
        );
    }

    #[test]
    fn test_missing_symbolic_type_reference() {
        let raw = extension_with_custom_type_reference(
            "extension:example.com:missing",
            "wrap",
            "list<u!missing<any1>>",
            vec![],
        );
        assert!(matches!(
            ExtensionFile::create(raw),
            Err(SimpleExtensionsError::UnresolvedTypeReference { type_name }) if type_name == "missing"
        ));
    }

    #[test]
    fn test_numeric_expression_returns_are_still_skipped() {
        for return_type in ["varchar<L + 1>", "decimal<P + 1, S>"] {
            let raw = extension_with_custom_type_reference(
                "extension:example.com:unsupported",
                "unsupported",
                return_type,
                vec![],
            );
            let file = ExtensionFile::create(raw).unwrap();
            assert!(
                file.extension()
                    .get_scalar_function("unsupported")
                    .is_none()
            );
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_scalar_function_parses_completely() {
        use super::super::{
            argument::ArgumentsItem,
            scalar_functions::{Impl, NullabilityHandling, Options},
            types::*,
        };
        use crate::parse::Parse;
        use crate::text::simple_extensions;
        use std::collections::HashMap;

        let registry = Registry::from_core_extensions();
        let functions_arithmetic_urn =
            Urn::from_str("extension:io.substrait:functions_arithmetic").unwrap();

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
            options: Options({
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
            }),
            variadic: None,
            session_dependent: false,
            deterministic: true,
            nullability: NullabilityHandling::Mirror,
            return_type: Type::new(TypeKind::I8, false),
            implementation: HashMap::new(),
        };

        assert_eq!(&add.impls[0], &expected_impl);
    }
}
