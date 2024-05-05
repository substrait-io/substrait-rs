// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use thiserror::Error;

use crate::parse::{
    proto::extensions::SimpleExtensionUri, text::simple_extensions::SimpleExtensions, Anchor, Parse,
};

use super::proto::extensions::{ExtensionFunction, ExtensionType, ExtensionTypeVariation};

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

    /// Add a [ExtensionFunction] to this context. Must return an error for duplicate
    /// anchors, when the URI is not supported or if the definition does not contain a matching function with the given name.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_extension_function(
        &mut self,
        extension_function: &ExtensionFunction,
    ) -> Result<(), ContextError>;

    /// Add an [ExtensionType] to this context. Must return an error for duplicate
    /// anchors, when the URI is not supported or if the definition does not contain a matching function with the given name.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_extension_type(&mut self, extension: &ExtensionType) -> Result<(), ContextError>;

    /// Add a [ExtensionTypeVariation] to this context. Must return an error for duplicate
    /// anchors, when the URI is not supported or if the definition does not contain a matching function with the given name.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_extension_type_variation(
        &mut self,
        extension_type_variation: &ExtensionTypeVariation,
    ) -> Result<(), ContextError>;
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

    /// Duplicate anchor for [ExtensionType].
    #[error("duplicate anchor `{0}` for extension_type")]
    DuplicateExtensionType(Anchor<ExtensionType>),

    /// Duplicate anchor for [ExtensionFunction].
    #[error("duplicate anchor `{0}` for extension_function")]
    DuplicateExtensionFunction(Anchor<ExtensionFunction>),

    /// Duplicate anchor for [ExtensionTypeVariation].
    #[error("duplicate anchor `{0}` for extension_type_variation")]
    DuplicateExtensionTypeVariation(Anchor<ExtensionTypeVariation>),

    /// Unsupported simple extension URI.
    #[error("unsupported simple extension URI: {0}")]
    UnsupportedURI(String),
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::{hash_map::Entry, HashMap};

    use crate::parse::{
        context::ContextError,
        proto::extensions::{
            ExtensionFunction, ExtensionType, ExtensionTypeVariation, SimpleExtensionUri,
        },
        text::simple_extensions::SimpleExtensions,
        Anchor,
    };

    /// A test context.
    ///
    /// This currently mocks support for simple extensions (does not resolve or
    /// parse).
    pub struct Context {
        empty_simple_extensions: SimpleExtensions,
        simple_extensions: HashMap<Anchor<SimpleExtensionUri>, SimpleExtensionUri>,
        extension_types: HashMap<Anchor<ExtensionType>, ExtensionType>,
        extension_functions: HashMap<Anchor<ExtensionFunction>, ExtensionFunction>,
        extension_type_variations: HashMap<Anchor<ExtensionTypeVariation>, ExtensionTypeVariation>,
    }

    impl Default for Context {
        fn default() -> Self {
            Self {
                empty_simple_extensions: SimpleExtensions {},
                simple_extensions: Default::default(),
                extension_types: Default::default(),
                extension_functions: Default::default(),
                extension_type_variations: Default::default(),
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

        fn add_extension_function(
            &mut self,
            extension_function: &crate::parse::proto::extensions::ExtensionFunction,
        ) -> Result<(), ContextError> {
            match self.extension_functions.entry(extension_function.anchor()) {
                Entry::Occupied(_) => Err(ContextError::DuplicateExtensionFunction(
                    extension_function.anchor(),
                )),
                Entry::Vacant(entry) => {
                    entry.insert(extension_function.clone());

                    Ok(())
                }
            }
        }

        fn add_extension_type(
            &mut self,
            extension_type: &crate::parse::proto::extensions::ExtensionType,
        ) -> Result<(), ContextError> {
            match self.extension_types.entry(extension_type.anchor()) {
                Entry::Occupied(_) => Err(ContextError::DuplicateExtensionType(
                    extension_type.anchor(),
                )),
                Entry::Vacant(entry) => {
                    entry.insert(extension_type.clone());

                    Ok(())
                }
            }
        }

        fn add_extension_type_variation(
            &mut self,
            extension_type_variation: &crate::parse::proto::extensions::ExtensionTypeVariation,
        ) -> Result<(), ContextError> {
            match self
                .extension_type_variations
                .entry(extension_type_variation.anchor())
            {
                Entry::Occupied(_) => Err(ContextError::DuplicateExtensionTypeVariation(
                    extension_type_variation.anchor(),
                )),
                Entry::Vacant(entry) => {
                    entry.insert(extension_type_variation.clone());

                    Ok(())
                }
            }
        }
    }
}
