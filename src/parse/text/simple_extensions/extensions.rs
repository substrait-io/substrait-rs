// SPDX-License-Identifier: Apache-2.0

//! Validated simple extensions: [`SimpleExtensions`].
//!
//! Currently only type definitions are supported; function parsing will be
//! added in a future update.

use indexmap::IndexMap;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::{SimpleExtensionsError, scalar_functions::ScalarFunction, types::CustomType};
use crate::{
    parse::{Context, Parse},
    text::simple_extensions::SimpleExtensions as RawExtensions,
    urn::Urn,
};

/// The contents (types and functions) in an [`ExtensionFile`](super::file::ExtensionFile).
///
/// This structure stores and provides access to the individual objects defined
/// in an [`ExtensionFile`](super::file::ExtensionFile); [`SimpleExtensions`]
/// represents the contents of an extensions file.
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

    /// Consume the parsed extension and return its types.
    pub(crate) fn into_types(self) -> HashMap<String, CustomType> {
        self.types
    }

    /// Add a scalar function to the context, merging with existing functions of the same name.
    ///
    /// When duplicate function names are encountered, implementations are merged (unioned).
    /// If descriptions differ, the description is dropped. This matches the behavior of
    /// substrait-java and substrait-python implementations.
    ///
    /// See: https://github.com/substrait-io/substrait/issues/931
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
            Entry::Occupied(mut e) => {
                let existing = e.get_mut();
                // Union the implementations
                existing.impls.extend(scalar_function.impls);
                // Drop description if they differ
                if existing.description != scalar_function.description {
                    existing.description = None;
                }
                Ok(())
            }
        }
    }

    /// Check if a scalar function with the given name exists
    pub fn has_scalar_function(&self, name: &str) -> bool {
        self.scalar_functions.contains_key(name)
    }

    /// Get a scalar function by name
    pub fn get_scalar_function(&self, name: &str) -> Option<&ScalarFunction> {
        self.scalar_functions.get(name)
    }

    /// Get an iterator over all scalar functions
    pub fn scalar_functions(&self) -> impl Iterator<Item = &ScalarFunction> {
        self.scalar_functions.values()
    }
}

/// resolved or unresolved.
#[derive(Debug, Default)]
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

        for scalar_fn in scalar_functions {
            let parsed_fn = ScalarFunction::from_raw(scalar_fn, ctx)?;
            extension.add_scalar_function(parsed_fn)?;
        }

        if let Some(missing) = ctx.linked.iter().next() {
            // TODO: Track originating type(s) to improve this error message.
            return Err(super::SimpleExtensionsError::UnresolvedTypeReference {
                type_name: missing.clone(),
            });
        }

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
