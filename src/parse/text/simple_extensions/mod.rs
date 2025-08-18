// SPDX-License-Identifier: Apache-2.0

//! Parsing of [text::simple_extensions] types.

use thiserror::Error;

pub mod argument;
mod extensions;
mod file;
mod registry;
pub mod types;

pub use extensions::SimpleExtensions;
pub use file::ExtensionFile;
pub use registry::Registry;
pub use types::{ConcreteType, CustomType, ExtensionTypeError};

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
