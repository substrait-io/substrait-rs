// SPDX-License-Identifier: Apache-2.0

//! Parsing of [text::simple_extensions] types.

use std::collections::HashMap;
use thiserror::Error;
use url::Url;

use crate::{
    parse::{Context, Parse},
    text,
};

pub mod argument;
pub mod context;
pub mod extension;
pub mod registry;
pub mod types;

pub use extension::ExtensionFile;
pub use registry::Registry;
pub use types::{ConcreteType, CustomType, ExtensionTypeError};

/// A parsed and validated [text::simple_extensions::SimpleExtensions].
/// This replaces the TODO implementation with ExtensionFile functionality.
pub struct SimpleExtensions {
    /// The URI this extension was loaded from
    pub uri: Url,
    /// Parsed and validated custom types
    types: HashMap<String, CustomType>,
}

/// Parse errors for [text::simple_extensions::SimpleExtensions].
#[derive(Debug, Error)]
pub enum SimpleExtensionsError {
    /// Extension type error
    #[error("Extension type error: {0}")]
    ExtensionTypeError(#[from] ExtensionTypeError),
    /// Unresolved type reference in structure field
    #[error("Type '{type_name}' referenced in '{originating}' structure not found")]
    UnresolvedTypeReference {
        /// The type name that could not be resolved
        type_name: String,
        /// The type that contains the unresolved reference
        originating: String,
    },
    /// Structure field cannot be nullable
    #[error("Structure representation in type '{originating}' cannot be nullable")]
    StructureCannotBeNullable {
        /// The type that has a nullable structure
        originating: String,
    },
}

impl SimpleExtensions {
    /// Create a new, empty SimpleExtensions
    pub fn empty(uri: Url) -> Self {
        Self {
            uri,
            types: HashMap::new(),
        }
    }

    /// Create a validated SimpleExtensions from raw data and URI
    pub fn create(
        uri: Url,
        extensions: text::simple_extensions::SimpleExtensions,
    ) -> Result<Self, SimpleExtensionsError> {
        // Parse all types (may contain unresolved Extension(String) references)
        let mut ctx = context::ExtensionContext::new(uri.clone());
        let mut types = HashMap::new();

        for type_item in &extensions.types {
            let custom_type = type_item.clone().parse(&mut ctx)?;
            // Add the parsed type to the context so later types can reference it
            ctx.add_type(&custom_type);
            types.insert(custom_type.name.clone(), custom_type);
        }

        // TODO: Validate that all Extension(String) references in structure
        // fields exist Walk through all CustomType.structure fields and check
        // that Extension(String) references can be resolved to actual types in
        // the registry.

        Ok(Self { uri, types })
    }

    /// Get a type by name
    pub fn get_type(&self, name: &str) -> Option<&CustomType> {
        self.types.get(name)
    }

    /// Get an iterator over all types in this extension
    pub fn types(&self) -> impl Iterator<Item = &CustomType> {
        self.types.values()
    }
}

impl<C: Context> Parse<C> for text::simple_extensions::SimpleExtensions {
    type Parsed = SimpleExtensions;
    type Error = SimpleExtensionsError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        // For parsing without URI context, create a dummy URI
        let dummy_uri = Url::parse("file:///unknown").unwrap();
        SimpleExtensions::create(dummy_uri, self)
    }
}

impl From<SimpleExtensions> for text::simple_extensions::SimpleExtensions {
    fn from(_value: SimpleExtensions) -> Self {
        // TODO: Implement conversion back to text representation
        unimplemented!("Conversion from parsed SimpleExtensions back to text representation not yet implemented")
    }
}
