// SPDX-License-Identifier: Apache-2.0

//! Parsing context for extension processing.

use std::collections::{HashMap, HashSet};

use super::types::CustomType;
use crate::{
    parse::{Context, Parse},
    text::simple_extensions::SimpleExtensions as RawExtensions,
};

/// Parsing context for extension processing
///
/// The context provides access to types defined in the same extension file during parsing.
/// This allows type references to be resolved within the same extension file.
#[derive(Debug, Default)]
pub struct SimpleExtensions {
    /// Types defined in this extension file
    types: HashMap<String, CustomType>,
}

impl SimpleExtensions {
    /// Add a type to the context
    pub fn add_type(&mut self, custom_type: &CustomType) {
        self.types
            .insert(custom_type.name.clone(), custom_type.clone());
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
}

/// A context for parsing simple extensions.
#[derive(Debug, Default)]
pub struct TypeContext {
    /// Types that have been seen so far, not yet resolved.
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
    type Parsed = SimpleExtensions;
    type Error = super::SimpleExtensionsError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        let mut extension = SimpleExtensions::default();

        for type_item in self.types {
            let custom_type = Parse::parse(type_item, ctx)?;
            // Add the parsed type to the context so later types can reference it
            extension.add_type(&custom_type);
        }

        Ok(extension)
    }
}

// Implement conversion from parsed form back to raw text representation.
impl From<SimpleExtensions> for RawExtensions {
    fn from(value: SimpleExtensions) -> Self {
        // Minimal types-only conversion to satisfy tests
        let types = value.types().cloned().map(Into::into).collect();
        RawExtensions {
            types,
            // TODO: Implement conversion back to raw representation
            aggregate_functions: vec![],
            dependencies: HashMap::new(),
            scalar_functions: vec![],
            type_variations: vec![],
            window_functions: vec![],
        }
    }
}
