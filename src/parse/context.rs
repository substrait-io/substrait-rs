// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use thiserror::Error;

use crate::parse::proto::extensions::SimpleExtensionUrn;
use crate::parse::text::simple_extensions::ExtensionFile;
use crate::parse::{Anchor, Parse};

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {
    /// Parse an item with this context.
    ///
    /// See [Parse::parse].
    fn parse<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }
}

pub trait ProtoContext: Context {
    /// Add a [SimpleExtensionUrn] to this context. Must return an error for duplicate
    /// anchors or when the URI is not supported.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_simple_extension_urn(
        &mut self,
        simple_extension_urn: &SimpleExtensionUrn,
    ) -> Result<&ExtensionFile, ContextError>;

    /// Returns the simple extensions for the given simple extension anchor.
    fn simple_extensions(
        &self,
        anchor: &Anchor<SimpleExtensionUrn>,
    ) -> Result<&ExtensionFile, ContextError>;
}

/// Parse context errors.
#[derive(Debug, Error, PartialEq)]
pub enum ContextError {
    /// Undefined reference to simple extension.
    #[error("undefined reference to simple extension with anchor `{0}`")]
    UndefinedSimpleExtension(Anchor<SimpleExtensionUrn>),

    /// Duplicate anchor for simple extension.
    #[error("duplicate anchor `{0}` for simple extension")]
    DuplicateSimpleExtension(Anchor<SimpleExtensionUrn>),

    /// Unsupported simple extension urn.
    #[error("unsupported simple extension urn: {0}")]
    UnsupportedUrn(String),
}

#[cfg(test)]
pub(crate) mod fixtures {
    use std::collections::{HashMap, hash_map::Entry};

    use crate::parse::{
        Anchor, context::ContextError, proto::extensions::SimpleExtensionUrn,
        text::simple_extensions::ExtensionFile,
    };

    /// A test context.
    ///
    /// This currently mocks support for simple extensions (does not resolve or
    /// parse).
    #[derive(Default)]
    pub struct Context {
        simple_extensions: HashMap<Anchor<SimpleExtensionUrn>, ExtensionFile>,
    }

    impl super::Context for Context {}

    impl super::ProtoContext for Context {
        fn add_simple_extension_urn(
            &mut self,
            simple_extension_urn: &crate::parse::proto::extensions::SimpleExtensionUrn,
        ) -> Result<&ExtensionFile, ContextError> {
            match self.simple_extensions.entry(simple_extension_urn.anchor()) {
                Entry::Occupied(_) => Err(ContextError::DuplicateSimpleExtension(
                    simple_extension_urn.anchor(),
                )),
                Entry::Vacant(entry) => {
                    let f = ExtensionFile::empty(simple_extension_urn.urn().clone());
                    let ext_ref = entry.insert(f);

                    Ok(ext_ref)
                }
            }
        }

        fn simple_extensions(
            &self,
            anchor: &Anchor<SimpleExtensionUrn>,
        ) -> Result<&ExtensionFile, ContextError> {
            self.simple_extensions
                .get(anchor)
                .ok_or(ContextError::UndefinedSimpleExtension(*anchor))
        }
    }
}
