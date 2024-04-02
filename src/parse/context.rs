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
pub(crate) mod tests {
    use std::collections::{hash_map::Entry, HashMap};

    use crate::parse::{
        context::ContextError, proto::extensions::SimpleExtensionUri,
        text::simple_extensions::SimpleExtensions, Anchor,
    };

    /// A test context.
    ///
    /// This currently mocks support for simple extensions (always returns an
    /// empty simple extensions).
    pub struct Context {
        empty_simple_extensions: SimpleExtensions,
        simple_extensions: HashMap<Anchor<SimpleExtensionUri>, SimpleExtensionUri>,
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
                        entry.insert(simple_extension_uri.clone());
                        // Here we just return an empty simple extensions.
                        Ok(&self.empty_simple_extensions)
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
                .contains_key(anchor)
                .then_some(&self.empty_simple_extensions)
                .ok_or(ContextError::UndefinedSimpleExtension(*anchor))
        }
    }
}
