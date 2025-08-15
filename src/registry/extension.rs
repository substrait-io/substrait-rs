// SPDX-License-Identifier: Apache-2.0

//! Validated extension file wrapper for types.
//!
//! This module provides `ExtensionFile`, a validated wrapper around SimpleExtensions
//! that focuses on type definitions and provides safe type lookup methods.

use std::collections::HashMap;
use thiserror::Error;
use url::Url;

use crate::parse::Parse;
use crate::registry::types::{CustomType, ExtensionTypeError};
use crate::text::simple_extensions::SimpleExtensions;

use super::context::ExtensionContext;

/// Errors that can occur during extension type validation
#[derive(Debug, Error)]
pub enum ValidationError {
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

/// A validated extension file containing types from a single URI.
/// All types are parsed and validated on construction.
#[derive(Debug, Clone)]
pub struct ExtensionFile {
    /// The URI this extension was loaded from
    pub uri: Url,
    /// Parsed and validated custom types
    types: HashMap<String, CustomType>,
}

impl ExtensionFile {
    /// Create a validated extension file from raw data
    pub fn create(uri: Url, extensions: SimpleExtensions) -> Result<Self, ValidationError> {
        // Parse all types (may contain unresolved Extension(String) references)
        let mut ctx = ExtensionContext::new(uri.clone());
        let mut types = HashMap::new();
        
        for type_item in &extensions.types {
            let custom_type = type_item.clone().parse(&mut ctx)?;
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
