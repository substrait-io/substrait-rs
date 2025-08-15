// SPDX-License-Identifier: Apache-2.0

//! Extension parsing context for validation.

use std::collections::HashMap;

use url::Url;

use crate::parse::Context;
use super::types::CustomType;

/// Context for parsing and validating extension definitions.
///
/// This context accumulates validated types as they are parsed,
/// allowing later elements to reference previously validated types.
pub struct ExtensionContext {
    /// The URI of the extension being parsed.
    pub uri: Url,
    /// Map of type names to their validated definitions
    types: HashMap<String, CustomType>,
}

impl ExtensionContext {
    /// Create a new extension context for parsing.
    pub fn new(uri: Url) -> Self {
        Self {
            uri,
            types: HashMap::new(),
        }
    }

    /// Check if a type with the given name exists in the context
    pub fn has_type(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Add a type to the context after it has been validated
    pub(crate) fn add_type(&mut self, custom_type: &CustomType) {
        self.types.insert(custom_type.name.clone(), custom_type.clone());
    }

    /// Get a type by name from the context, returning the CustomType
    pub(crate) fn get_type(&self, name: &str) -> Option<&CustomType> {
        self.types.get(name)
    }
}

impl Context for ExtensionContext {
    // Implementation required by the Context trait
}
