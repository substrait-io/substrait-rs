// SPDX-License-Identifier: Apache-2.0

//! Parsing context for extension processing.

use std::collections::HashMap;

use super::types::CustomType;
use crate::parse::Context;

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

impl Context for SimpleExtensions {
    // ExtensionContext implements the Context trait
}
