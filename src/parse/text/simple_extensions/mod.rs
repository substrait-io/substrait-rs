// SPDX-License-Identifier: Apache-2.0

//! Rustic types for validating and working with Substrait simple extensions.
//!
//! The raw YAML structs live in [`crate::text::simple_extensions`].  This
//! module parses those values into the typed representations used by this
//! crate:
//! * [`ExtensionFile`] – a fully validated extension document (URN plus its
//!   definitions).
//! * [`SimpleExtensions`] – the validated objects declared by a single
//!   extension file.
//! * [`CustomType`] / [`ConcreteType`] – type definitions and resolved type
//!   structures used when checking function signatures.
//! * [`Registry`] – a reusable lookup structure that stores validated extension
//!   files and exposes typed access to their contents.

use std::convert::Infallible;

use thiserror::Error;

pub mod argument;
mod extensions;
mod file;
mod parsed_type;
mod registry;
mod types;

pub use extensions::SimpleExtensions;
pub use file::ExtensionFile;
pub use parsed_type::TypeExpr;
pub use registry::Registry;
pub use types::{ConcreteType, CustomType, ExtensionTypeError};

/// Errors for converting from YAML to [SimpleExtensions].
#[derive(Debug, Error)]
pub enum SimpleExtensionsError {
    /// Extension type error
    #[error("Extension type error: {0}")]
    ExtensionTypeError(#[from] ExtensionTypeError),
    /// Failed to parse SimpleExtensions YAML
    #[error("YAML parse error: {0}")]
    YamlParse(#[from] serde_yaml::Error),
    /// I/O error while reading extension content
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// Invalid URN provided
    #[error("invalid urn")]
    InvalidUrn(#[from] crate::urn::InvalidUrn),
    /// Unresolved type reference in structure field
    #[error("Type '{type_name}' referenced in structure not found")]
    UnresolvedTypeReference {
        /// The type name that could not be resolved
        type_name: String,
        // TODO: the location in the file where this came from would be nice
    },
    /// Duplicate type definition within the same extension
    #[error("duplicate type definition for `{name}`")]
    DuplicateTypeName {
        /// The repeated type name
        name: String,
    },
}

// Needed for certain conversions - e.g. Urn -> Urn - to succeed.
impl From<Infallible> for SimpleExtensionsError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
