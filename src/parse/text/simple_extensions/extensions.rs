// SPDX-License-Identifier: Apache-2.0

//! Validated simple extensions: [`SimpleExtensions`].
//!
//! This module provides access to parsed and validated extension types and scalar functions.

use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::{SimpleExtensionsError, scalar_functions::ScalarFunction, types::CustomType};
use crate::{
    parse::{Context, Parse},
    text::simple_extensions::SimpleExtensions as RawExtensions,
    urn::Urn,
};

/// The contents (types and scalar functions) in an [`ExtensionFile`](super::file::ExtensionFile).
///
/// This structure stores and provides access to the individual objects defined
/// in an [`ExtensionFile`](super::file::ExtensionFile); [`SimpleExtensions`]
/// represents the contents of an extensions file.
///
/// Currently, [`CustomType`]s and scalar functions are included. Window and
/// aggregate functions will be added in future updates.
#[derive(Clone, Debug, Default)]
pub struct SimpleExtensions {
    /// Types defined in this extension file
    types: HashMap<String, CustomType>,
    /// Scalar functions defined in this extension file
    scalar_functions: HashMap<String, ScalarFunction>,
}

impl SimpleExtensions {
    /// Add a type to the context. Name must be unique.
    pub fn add_type(&mut self, custom_type: &CustomType) -> Result<(), SimpleExtensionsError> {
        if self.types.contains_key(&custom_type.name) {
            return Err(SimpleExtensionsError::DuplicateTypeName {
                name: custom_type.name.clone(),
            });
        }

        self.types
            .insert(custom_type.name.clone(), custom_type.clone());
        Ok(())
    }

    /// Check if a type with the given name exists in the context
    pub fn has_type(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Get a type by name from the context
    pub fn get_type(&self, name: &str) -> Option<&CustomType> {
        self.types.get(name)
    }

    /// Get an iterator over all types in the context
    pub fn types(&self) -> impl Iterator<Item = &CustomType> {
        self.types.values()
    }

    /// Check if a scalar function with the given name exists in the context
    pub fn has_scalar_function(&self, name: &str) -> bool {
        self.scalar_functions.contains_key(name)
    }

    /// Get a scalar function by name from the context
    pub fn get_scalar_function(&self, name: &str) -> Option<&ScalarFunction> {
        self.scalar_functions.get(name)
    }

    /// Get an iterator over all scalar functions in the context
    pub fn scalar_functions(&self) -> impl Iterator<Item = &ScalarFunction> {
        self.scalar_functions.values()
    }

    /// Add a scalar function to the context. Name must be unique.
    pub(super) fn add_scalar_function(
        &mut self,
        scalar_function: ScalarFunction,
    ) -> Result<(), SimpleExtensionsError> {
        use std::collections::hash_map::Entry;
        match self.scalar_functions.entry(scalar_function.name.clone()) {
            Entry::Vacant(e) => {
                e.insert(scalar_function);
                Ok(())
            }
            Entry::Occupied(_) => Err(SimpleExtensionsError::DuplicateFunctionName {
                name: scalar_function.name.clone(),
            }),
        }
    }

    /// Consume the parsed extension and return its types.
    pub(crate) fn into_types(self) -> HashMap<String, CustomType> {
        self.types
    }
}

/// resolved or unresolved.
#[derive(Clone, Debug, Default)]
pub(crate) struct TypeContext {
    /// Types that have been seen so far, now resolved.
    known: HashSet<String>,
    /// Types that have been linked to, not yet resolved.
    linked: HashSet<String>,
}

impl TypeContext {
    /// Mark a type as found
    pub fn found(&mut self, name: &str) {
        self.linked.remove(name);
        self.known.insert(name.to_string());
    }

    /// Mark a type as linked to - some other type or function references it,
    /// but we haven't seen it.
    pub fn linked(&mut self, name: &str) {
        if !self.known.contains(name) {
            self.linked.insert(name.to_string());
        }
    }
}

impl Context for TypeContext {}

// Implement parsing for the raw text representation to produce an `ExtensionFile`.
impl Parse<TypeContext> for RawExtensions {
    type Parsed = (Urn, SimpleExtensions);
    type Error = super::SimpleExtensionsError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        let RawExtensions {
            urn,
            types,
            scalar_functions,
            ..
        } = self;
        let urn = Urn::from_str(&urn)?;
        let mut extension = SimpleExtensions::default();

        for type_item in types {
            let custom_type = Parse::parse(type_item, ctx)?;
            extension.add_type(&custom_type)?;
        }

        if let Some(missing) = ctx.linked.iter().next() {
            // TODO: Track originating type(s) to improve this error message.
            return Err(super::SimpleExtensionsError::UnresolvedTypeReference {
                type_name: missing.clone(),
            });
        }

        for scalar_fn in scalar_functions {
            let parsed_fn = ScalarFunction::from_raw(scalar_fn, ctx)?;
            extension.add_scalar_function(parsed_fn)?;
        }

        // NOTE: We don't check ctx.linked here because argument types in functions
        // are not currently parsed to ConcreteType during function parsing - they remain
        // as raw strings. This means undefined custom types in function signatures won't
        // be caught until runtime. See: #444

        Ok((urn, extension))
    }
}

impl From<(Urn, SimpleExtensions)> for RawExtensions {
    fn from((urn, extension): (Urn, SimpleExtensions)) -> Self {
        let types = extension
            .into_types()
            .into_values()
            .map(Into::into)
            .collect();

        RawExtensions {
            urn: urn.to_string(),
            aggregate_functions: vec![],
            dependencies: IndexMap::new(),
            scalar_functions: vec![],
            type_variations: vec![],
            types,
            window_functions: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::simple_extensions::{
        Arguments, ArgumentsItem as RawArgumentItem, ReturnValue,
        ScalarFunction as RawScalarFunction, ScalarFunctionImplsItem as RawImpl, Type as RawType,
        ValueArg,
    };

    #[test]
    fn test_parse_extensions_with_scalar_functions() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let raw_scalar_fn = RawScalarFunction {
            name: "add".to_string(),
            description: Some("Addition function".to_string()),
            impls: vec![raw_impl],
        };

        let raw_extensions = RawExtensions {
            urn: "extension:example.com:test".to_string(),
            aggregate_functions: vec![],
            dependencies: IndexMap::new(),
            scalar_functions: vec![raw_scalar_fn],
            type_variations: vec![],
            types: vec![],
            window_functions: vec![],
        };

        let mut ctx = TypeContext::default();
        let (urn, extensions) = raw_extensions
            .parse(&mut ctx)
            .expect("Parse should succeed");

        assert_eq!(urn.to_string(), "extension:example.com:test");
        assert_eq!(extensions.scalar_functions.len(), 1);
        assert!(extensions.scalar_functions.contains_key("add"));
    }

    #[test]
    fn test_duplicate_function_name_error() {
        let raw_impl = RawImpl {
            args: None,
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let raw_scalar_fn1 = RawScalarFunction {
            name: "add".to_string(),
            description: Some("Addition function".to_string()),
            impls: vec![raw_impl.clone()],
        };

        let raw_scalar_fn2 = RawScalarFunction {
            name: "add".to_string(),
            description: Some("Another addition function".to_string()),
            impls: vec![raw_impl],
        };

        let raw_extensions = RawExtensions {
            urn: "extension:example.com:test".to_string(),
            aggregate_functions: vec![],
            dependencies: IndexMap::new(),
            scalar_functions: vec![raw_scalar_fn1, raw_scalar_fn2],
            type_variations: vec![],
            types: vec![],
            window_functions: vec![],
        };

        let mut ctx = TypeContext::default();
        let result = raw_extensions.parse(&mut ctx);

        assert!(result.is_err());
        match result.unwrap_err() {
            SimpleExtensionsError::DuplicateFunctionName { name } => {
                assert_eq!(name, "add");
            }
            other => panic!("Expected DuplicateFunctionName error, got: {:?}", other),
        }
    }

    #[test]
    #[ignore = "Known issue: argument types are not parsed during function parsing, so undefined custom types in function signatures are not detected. See #444"]
    fn test_function_with_undefined_type_reference() {
        let raw_impl = RawImpl {
            args: Some(Arguments(vec![RawArgumentItem::ValueArg(ValueArg {
                name: Some("x".to_string()),
                description: None,
                value: RawType::String("u!NonExistentCustomType".to_string()),
                constant: None,
            })])),
            options: None,
            variadic: None,
            session_dependent: None,
            deterministic: None,
            nullability: None,
            return_: ReturnValue(RawType::String("i32".to_string())),
            implementation: None,
        };

        let raw_scalar_fn = RawScalarFunction {
            name: "test_fn".to_string(),
            description: Some("Function that references undefined type".to_string()),
            impls: vec![raw_impl],
        };

        let raw_extensions = RawExtensions {
            urn: "extension:example.com:test".to_string(),
            aggregate_functions: vec![],
            dependencies: IndexMap::new(),
            scalar_functions: vec![raw_scalar_fn],
            type_variations: vec![],
            types: vec![],
            window_functions: vec![],
        };

        let mut ctx = TypeContext::default();
        let result = raw_extensions.parse(&mut ctx);

        // This test demonstrates the known issue: currently this will succeed
        // when it should fail with UnresolvedTypeReference
        assert!(
            result.is_err(),
            "Should error when function references undefined custom type"
        );
        match result.unwrap_err() {
            SimpleExtensionsError::UnresolvedTypeReference { type_name } => {
                assert_eq!(type_name, "NonExistentCustomType");
            }
            other => panic!("Expected UnresolvedTypeReference error, got: {:?}", other),
        }
    }
}
