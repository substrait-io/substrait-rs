// SPDX-License-Identifier: Apache-2.0

//! Parsing context for extension processing.

use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use super::{SimpleExtensionsError, types::CustomType};
use crate::{
    parse::{Context, Parse},
    text::simple_extensions::SimpleExtensions as RawExtensions,
    urn::Urn,
};

/// Parsing context for extension processing
///
/// The context provides access to types defined in the same extension file during parsing.
/// This allows type references to be resolved within the same extension file. The corresponding
/// URN is tracked by [`ExtensionFile`](super::file::ExtensionFile) so this structure can focus on
/// validated type information.
#[derive(Clone, Debug, Default)]
pub struct SimpleExtensions {
    /// Types defined in this extension file
    types: HashMap<String, CustomType>,
}

impl SimpleExtensions {
    /// Add a type to the context
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
}

/// A context for parsing simple extensions.
#[derive(Debug, Default)]
pub struct TypeContext {
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

    /// Mark a type as linked to - some other type or function references it, but we haven't seen it.
    pub fn linked(&mut self, name: &str) {
        if !self.known.contains(name) {
            self.linked.insert(name.to_string());
        }
    }
}

impl Context for TypeContext {
    // ExtensionContext implements the Context trait
}

// Implement parsing for the raw text representation to produce an `ExtensionFile`.
impl Parse<TypeContext> for RawExtensions {
    type Parsed = (Urn, SimpleExtensions);
    type Error = super::SimpleExtensionsError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        let RawExtensions { urn, types, .. } = self;
        let urn = Urn::from_str(&urn)?;
        let mut extension = SimpleExtensions::default();

        for type_item in types {
            let custom_type = Parse::parse(type_item, ctx)?;
            // Add the parsed type to the context so later types can reference it
            extension.add_type(&custom_type)?;
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
            dependencies: HashMap::new(),
            scalar_functions: vec![],
            type_variations: vec![],
            types,
            window_functions: vec![],
        }
    }
}
