// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use thiserror::Error;

use crate::parse::{
    Anchor, Parse, proto::extensions::SimpleExtensionUrn, text::simple_extensions::SimpleExtensions,
};

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

    /// Add a [SimpleExtensionUrn] to this context. Must return an error for duplicate
    /// anchors or when the urn is not supported.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_simple_extension_urn(
        &mut self,
        simple_extension_urn: &SimpleExtensionUrn,
    ) -> Result<&SimpleExtensions, ContextError>;

    /// Returns the simple extensions for the given simple extension anchor.
    fn simple_extensions(
        &self,
        anchor: &Anchor<SimpleExtensionUrn>,
    ) -> Result<&SimpleExtensions, ContextError>;
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
pub(crate) mod tests {
    use std::collections::{HashMap, hash_map::Entry};

    use crate::parse::{
        Anchor, context::ContextError, proto::extensions::SimpleExtensionUrn,
        text::simple_extensions::SimpleExtensions,
    };

    /// A test context.
    ///
    /// This currently mocks support for simple extensions (does not resolve or
    /// parse).
    pub struct Context {
        empty_simple_extensions: SimpleExtensions,
        simple_extensions: HashMap<Anchor<SimpleExtensionUrn>, SimpleExtensionUrn>,
    }

    impl Default for Context {
        fn default() -> Self {
            Self {
                empty_simple_extensions: SimpleExtensions {},
                simple_extensions: Default::default(),
            }
        }
    }

    impl super::Context for Context {
        fn add_simple_extension_urn(
            &mut self,
            simple_extension_urn: &crate::parse::proto::extensions::SimpleExtensionUrn,
        ) -> Result<&SimpleExtensions, ContextError> {
            match self.simple_extensions.entry(simple_extension_urn.anchor()) {
                Entry::Occupied(_) => Err(ContextError::DuplicateSimpleExtension(
                    simple_extension_urn.anchor(),
                )),
                Entry::Vacant(entry) => {
                    // TODO: fetch
                    entry.insert(simple_extension_urn.clone());
                    // For now just return an empty extension
                    Ok(&self.empty_simple_extensions)
                }
            }
        }

        fn simple_extensions(
            &self,
            anchor: &Anchor<SimpleExtensionUrn>,
        ) -> Result<&SimpleExtensions, ContextError> {
            self.simple_extensions
                .contains_key(anchor)
                .then_some(&self.empty_simple_extensions)
                .ok_or(ContextError::UndefinedSimpleExtension(*anchor))
        }
    }
}
