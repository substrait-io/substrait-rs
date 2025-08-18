// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use thiserror::Error;

use crate::parse::{
    proto::extensions::SimpleExtensionUri, text::simple_extensions::SimpleExtensions, Anchor, Parse,
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
}

pub trait ProtoContext: Context {
    /// Add a [SimpleExtensionUri] to this context. Must return an error for duplicate
    /// anchors or when the URI is not supported.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionUri,
    ) -> Result<&SimpleExtensions, ContextError>;

    /// Returns the simple extensions for the given simple extension anchor.
    fn simple_extensions(
        &self,
        anchor: &Anchor<SimpleExtensionUri>,
    ) -> Result<&SimpleExtensions, ContextError>;
}

/// Parse context errors.
#[derive(Debug, Error, PartialEq)]
pub enum ContextError {
    /// Undefined reference to simple extension.
    #[error("undefined reference to simple extension with anchor `{0}`")]
    UndefinedSimpleExtension(Anchor<SimpleExtensionUri>),

    /// Duplicate anchor for simple extension.
    #[error("duplicate anchor `{0}` for simple extension")]
    DuplicateSimpleExtension(Anchor<SimpleExtensionUri>),

    /// Unsupported simple extension URI.
    #[error("unsupported simple extension URI: {0}")]
    UnsupportedURI(String),
}

#[cfg(test)]
pub(crate) mod fixtures {
    use std::collections::{hash_map::Entry, HashMap};

    use crate::parse::{
        context::ContextError, proto::extensions::SimpleExtensionUri,
        text::simple_extensions::SimpleExtensions, Anchor,
    };

    /// A test context.
    ///
    /// This currently mocks support for simple extensions (does not resolve or
    /// parse).
    pub struct Context {
        simple_extensions: HashMap<Anchor<SimpleExtensionUri>, SimpleExtensions>,
    }

    impl Default for Context {
        fn default() -> Self {
            Self {
                simple_extensions: Default::default(),
            }
        }
    }

    impl super::Context for Context {}

    impl super::ProtoContext for Context {
        fn add_simple_extension_uri(
            &mut self,
            simple_extension_uri: &crate::parse::proto::extensions::SimpleExtensionUri,
        ) -> Result<&SimpleExtensions, ContextError> {
            match self.simple_extensions.entry(simple_extension_uri.anchor()) {
                Entry::Occupied(_) => Err(ContextError::DuplicateSimpleExtension(
                    simple_extension_uri.anchor(),
                )),
                Entry::Vacant(entry) => {
                    // This is where we would resolve and then parse.
                    // This check shows the use of the unsupported uri error.
                    if let "http" | "https" | "file" = simple_extension_uri.uri().scheme() {
                        let ext = entry
                            .insert(SimpleExtensions::empty(simple_extension_uri.uri().clone()));
                        // Here we just return an empty simple extensions.
                        Ok(ext)
                    } else {
                        Err(ContextError::UnsupportedURI(format!(
                            "`{}` scheme not supported",
                            simple_extension_uri.uri().scheme()
                        )))
                    }
                }
            }
        }

        fn simple_extensions(
            &self,
            anchor: &Anchor<SimpleExtensionUri>,
        ) -> Result<&SimpleExtensions, ContextError> {
            self.simple_extensions
                .get(anchor)
                .ok_or(ContextError::UndefinedSimpleExtension(*anchor))
        }
    }
}
