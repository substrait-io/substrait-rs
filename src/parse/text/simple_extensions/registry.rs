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
            .map(|(orig_urn, simple_extensions)| {
                match ExtensionFile::create(simple_extensions.clone()) {
                    Ok(ExtensionFile { urn, extension }) => {
                        debug_assert_eq!(orig_urn, &urn);
                        (urn, extension)
                    }
                    Err(err) => panic!("Error parsing core extension {orig_urn}: {err}"),
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
    fn test_core_scalar_functions_parse_correctly() {
        use super::super::types::ReturnType;
        use crate::parse::text::simple_extensions::type_ast::TypeDerivation;

        let registry = Registry::from_core_extensions();

        // Test 1: decimal<P,S> with integer parameter variables
        let decimal_urn =
            Urn::from_str("extension:io.substrait:functions_arithmetic_decimal").unwrap();
        let add_func = registry.get_scalar_function(&decimal_urn, "add").unwrap();

        assert_eq!(add_func.impls.len(), 1);

        // The return type should be a Derivation - manually construct the expected AST
        let impl_ = &add_func.impls[0];

        use crate::parse::text::simple_extensions::type_ast::{
            BinaryOperator, DerivationExpr, DerivationStatement, TypeExpr, TypeExprParam,
        };

        // Manually construct the expected derivation structure
        let expected_derivation = TypeDerivation {
            statements: vec![
                // init_scale = max(S1,S2)
                DerivationStatement {
                    variable: "init_scale".to_string(),
                    expression: DerivationExpr::FunctionCall {
                        name: "max".to_string(),
                        args: vec![
                            DerivationExpr::Parameter("S1".to_string()),
                            DerivationExpr::Parameter("S2".to_string()),
                        ],
                    },
                },
                // init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
                DerivationStatement {
                    variable: "init_prec".to_string(),
                    expression: DerivationExpr::BinaryOp {
                        op: BinaryOperator::Add,
                        left: Box::new(DerivationExpr::BinaryOp {
                            op: BinaryOperator::Add,
                            left: Box::new(DerivationExpr::Variable("init_scale".to_string())),
                            right: Box::new(DerivationExpr::FunctionCall {
                                name: "max".to_string(),
                                args: vec![
                                    DerivationExpr::BinaryOp {
                                        op: BinaryOperator::Sub,
                                        left: Box::new(DerivationExpr::Parameter("P1".to_string())),
                                        right: Box::new(DerivationExpr::Parameter(
                                            "S1".to_string(),
                                        )),
                                    },
                                    DerivationExpr::BinaryOp {
                                        op: BinaryOperator::Sub,
                                        left: Box::new(DerivationExpr::Parameter("P2".to_string())),
                                        right: Box::new(DerivationExpr::Parameter(
                                            "S2".to_string(),
                                        )),
                                    },
                                ],
                            }),
                        }),
                        right: Box::new(DerivationExpr::Literal(1)),
                    },
                },
                // min_scale = min(init_scale, 6)
                DerivationStatement {
                    variable: "min_scale".to_string(),
                    expression: DerivationExpr::FunctionCall {
                        name: "min".to_string(),
                        args: vec![
                            DerivationExpr::Variable("init_scale".to_string()),
                            DerivationExpr::Literal(6),
                        ],
                    },
                },
                // delta = init_prec - 38
                DerivationStatement {
                    variable: "delta".to_string(),
                    expression: DerivationExpr::BinaryOp {
                        op: BinaryOperator::Sub,
                        left: Box::new(DerivationExpr::Variable("init_prec".to_string())),
                        right: Box::new(DerivationExpr::Literal(38)),
                    },
                },
                // prec = min(init_prec, 38)
                DerivationStatement {
                    variable: "prec".to_string(),
                    expression: DerivationExpr::FunctionCall {
                        name: "min".to_string(),
                        args: vec![
                            DerivationExpr::Variable("init_prec".to_string()),
                            DerivationExpr::Literal(38),
                        ],
                    },
                },
                // scale_after_borrow = max(init_scale - delta, min_scale)
                DerivationStatement {
                    variable: "scale_after_borrow".to_string(),
                    expression: DerivationExpr::FunctionCall {
                        name: "max".to_string(),
                        args: vec![
                            DerivationExpr::BinaryOp {
                                op: BinaryOperator::Sub,
                                left: Box::new(DerivationExpr::Variable("init_scale".to_string())),
                                right: Box::new(DerivationExpr::Variable("delta".to_string())),
                            },
                            DerivationExpr::Variable("min_scale".to_string()),
                        ],
                    },
                },
                // scale = init_prec > 38 ? scale_after_borrow : init_scale
                DerivationStatement {
                    variable: "scale".to_string(),
                    expression: DerivationExpr::Conditional {
                        condition: Box::new(DerivationExpr::BinaryOp {
                            op: BinaryOperator::Gt,
                            left: Box::new(DerivationExpr::Variable("init_prec".to_string())),
                            right: Box::new(DerivationExpr::Literal(38)),
                        }),
                        then_expr: Box::new(DerivationExpr::Variable(
                            "scale_after_borrow".to_string(),
                        )),
                        else_expr: Box::new(DerivationExpr::Variable("init_scale".to_string())),
                    },
                },
            ],
            // DECIMAL<prec, scale>
            result_type: TypeExpr::Simple(
                "DECIMAL".to_string(),
                vec![
                    TypeExprParam::Type(TypeExpr::Simple("prec".to_string(), vec![], false)),
                    TypeExprParam::Type(TypeExpr::Simple("scale".to_string(), vec![], false)),
                ],
                false,
            ),
        };

        match &impl_.return_type {
            ReturnType::Derivation(derivation) => {
                assert_eq!(
                    derivation, &expected_derivation,
                    "Derivation should match expected structure"
                );
            }
            ReturnType::Type(_) => panic!("Expected Derivation return type, got Type"),
        }

        // Check arguments
        assert_eq!(impl_.args.len(), 2);

        // Check options
        assert!(impl_.options.is_some());
        let options = impl_.options.as_ref().unwrap();
        assert!(options.0.contains_key("overflow"));
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn test_core_functions_with_type_variables() {
        use crate::parse::text::simple_extensions::types::{
            ReturnType, SignatureType, SignatureTypeKind,
        };

        let registry = Registry::from_core_extensions();

        // Test a function that uses type variables - equal function
        let comparison_urn = Urn::from_str("extension:io.substrait:functions_comparison").unwrap();
        let equal_func = registry
            .get_scalar_function(&comparison_urn, "equal")
            .unwrap();

        assert!(!equal_func.impls.is_empty());

        // equal takes two any1 arguments and returns boolean
        let impl_ = &equal_func.impls[0];

        let expected_return = ReturnType::Type(SignatureType {
            kind: SignatureTypeKind::Boolean,
            nullable: false,
        });
        assert_eq!(impl_.return_type, expected_return);

        // Verify it has two arguments
        assert_eq!(impl_.args.len(), 2, "equal should have exactly 2 arguments");
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
