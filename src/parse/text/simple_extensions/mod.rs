// SPDX-License-Identifier: Apache-2.0

//! Parsing of [text::simple_extensions] types.

use std::convert::Infallible;

use thiserror::Error;

pub mod argument;
mod extensions;
mod file;
mod registry;
pub mod types;

pub use extensions::SimpleExtensions;
pub use extensions::TypeContext;
pub use file::ExtensionFile;
pub use registry::Registry;
pub use types::{ConcreteType, CustomType, ExtensionTypeError};

/// Parse errors for [text::simple_extensions::SimpleExtensions].
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
    /// Invalid URI provided
    #[error("invalid URI: {0}")]
    InvalidUri(#[from] url::ParseError),
    /// Unresolved type reference in structure field
    #[error("Type '{type_name}' referenced in '{originating}' structure not found")]
    UnresolvedTypeReference {
        /// The type name that could not be resolved
        type_name: String,
        /// The type that contains the unresolved reference
        originating: String,
    },
}

// Needed for certain conversions - e.g. Url -> Url - to succeed.
impl From<Infallible> for SimpleExtensionsError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
