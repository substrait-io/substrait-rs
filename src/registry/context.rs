// SPDX-License-Identifier: Apache-2.0

//! Extension parsing context for validation.

use std::collections::HashMap;

use url::Url;

use crate::parse::Context;
use crate::text::simple_extensions::SimpleExtensionsTypesItem;

/// Context for parsing and validating extension definitions.
///
/// This context accumulates validated types as they are parsed,
/// allowing later elements to reference previously validated types.
pub struct ExtensionContext<'a> {
    /// The URI of the extension being parsed.
    pub uri: &'a Url,
    /// Map of type names to their definitions
    types: HashMap<&'a str, &'a SimpleExtensionsTypesItem>,
}

impl<'a> ExtensionContext<'a> {
    /// Create a new extension context for parsing.
    pub fn new(uri: &'a Url) -> Self {
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
    pub(crate) fn add_type(&mut self, type_item: &'a SimpleExtensionsTypesItem) {
        self.types.insert(&type_item.name, type_item);
    }
}

impl Context for ExtensionContext<'_> {
    // Implementation required by the Context trait
}
