// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use super::{
    extensions::{
        simple_extension_declaration::{ExtensionFunction, ExtensionType, ExtensionTypeVariation},
        simple_extension_uri::SimpleExtensionURI,
    },
    simple_extensions::{SimpleExtensionFunction, SimpleExtensions, SimpleExtensionsError},
    Anchor, Name, Parse,
};
use crate::text::simple_extensions::{
    SimpleExtensionsTypeVariationsItem, SimpleExtensionsTypesItem,
};
use thiserror::Error;

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {
    /// Parse an item using this context.
    ///
    /// See [Parse::parse].
    #[tracing::instrument(skip_all, err, fields(item = %std::any::type_name::<T>()))]
    fn parse<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }

    /// Parse an item using this context and record the item value on the parse span.
    ///
    /// See [Parse::parse].
    #[tracing::instrument(name = "parse", skip_all, err, fields(item = %std::any::type_name::<T>(), value = ?item))]
    fn parse_record_value<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }

    /// Parse an item using this context and record the item value and its index on the parse span.
    ///
    /// See [Parse::parse].
    #[tracing::instrument(name = "parse", skip_all, err, fields(item = %std::any::type_name::<T>(), index = index, value = ?item))]
    fn parse_record_value_with_index<T: Parse<Self>>(
        &mut self,
        item: T,
        index: usize,
    ) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }

    /// Parse an item using this context and record the item index on the parse span.
    ///
    /// See [Parse::parse].
    #[tracing::instrument(name = "parse", skip_all, err, fields(item = %std::any::type_name::<T>(), index = index))]
    fn parse_record_index<T: Parse<Self>>(
        &mut self,
        item: T,
        index: usize,
    ) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }

    /// Add a [SimpleExtensionURI] to this context. Should return an error for duplicate
    /// anchors or when the URI is not supported.
    ///
    /// This function must eagerly resolve and parse the simple extension, returning an
    /// error if either fails.
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<&SimpleExtensions, ContextError>;

    /// Returns the simple extensions for the given simple extension anchor.
    fn simple_extensions(&self, anchor: Anchor<SimpleExtensionURI>) -> &SimpleExtensions;

    /// Add an extension type declaration to this context.
    fn add_extension_type(
        &mut self,
        extension_type: &ExtensionType,
    ) -> Result<&SimpleExtensionsTypesItem, ContextError>;

    /// Returns the extension type for the given extension type anchor.
    fn extension_type(
        &self,
        extension_type_anchor: Anchor<ExtensionType>,
    ) -> &SimpleExtensionsTypesItem;

    /// Add an extension type variation declaration to this context.
    fn add_extension_type_variation(
        &mut self,
        extension_type_variation: &ExtensionTypeVariation,
    ) -> Result<&SimpleExtensionsTypeVariationsItem, ContextError>;

    /// Returns the extension type variation for the given extension type variation anchor.
    // TODO(mbrobbel): return type
    fn extension_type_variation(
        &self,
        extension_type_variation_anchor: Anchor<ExtensionTypeVariation>,
    ) -> &SimpleExtensionsTypeVariationsItem;

    /// Add an extension function declaration to this context.
    fn add_extension_function(
        &mut self,
        extension_function: &ExtensionFunction,
    ) -> Result<&SimpleExtensionFunction, ContextError>;

    /// Returns the extension function for the given extension type variation anchor.
    // TODO(mbrobbel): return type
    fn extension_function(
        &self,
        extension_function_anchor: Anchor<ExtensionFunction>,
    ) -> &SimpleExtensionFunction;
}

/// Parse context errors.
#[derive(Debug, Error)]
pub enum ContextError {
    /// Undefined reference to simple extension.
    #[error("undefined reference to simple extension with anchor `{0}`")]
    UndefinedSimpleExtension(Anchor<SimpleExtensionURI>),

    /// Undefined reference to extension type.
    #[error("undefined reference to extension type with anchor `{0}`")]
    UndefinedExtensionType(Anchor<ExtensionType>),

    /// Undefined reference to extension type variation.
    #[error("undefined reference to extension type variation with anchor `{0}`")]
    UndefinedExtensionTypeVariation(Anchor<ExtensionTypeVariation>),

    /// Undefined reference to extension function.
    #[error("undefined reference to extension function with anchor `{0}`")]
    UndefinedExtensionFunction(Anchor<ExtensionFunction>),

    /// Undefined reference to extension function.
    #[error("undefined reference to extension function with name `{0}`")]
    UndefinedExtensionFunctionDefinition(Name<ExtensionFunction>),

    /// Duplicate anchor for simple extension.
    #[error("duplicate anchor `{0}` for simple extension")]
    DuplicateSimpleExtension(Anchor<SimpleExtensionURI>),

    /// Duplicate anchor for extension type.
    #[error("duplicate anchor `{0}` for extension type")]
    DuplicateExtensionType(Anchor<ExtensionType>),

    /// Duplicate anchor for extension type variation.
    #[error("duplicate anchor `{0}` for extension type variation")]
    DuplicateExtensionTypeVariation(Anchor<ExtensionTypeVariation>),

    /// Duplicate anchor for extension function.
    #[error("duplicate anchor `{0}` for extension function")]
    DuplicateExtensionFunction(Anchor<ExtensionFunction>),

    /// Duplicate simple extension URI definition.
    #[error("duplicate definition for simple extension")]
    DuplicateSimpleExtensionDefinition,

    /// The resolved extension is invalid.
    #[error("invalid simple extensions file: {0}")]
    InvalidSimpleExtensions(serde_yaml::Error),

    /// Failed to resolve simple extension URI.
    #[error("failed to resolve simple extension URI: {0}")]
    FailedSimpleExtensionResolve(anyhow::Error),

    /// Unsupported simple extension URI.
    #[error("unsupported simple extension URI: {0}")]
    UnsupportedURI(String),

    /// Simple extensions error.
    #[error(transparent)]
    SimpleExtensions(#[from] SimpleExtensionsError),
}

#[cfg(test)]
pub(super) mod test {
    use crate::parse::{
        extensions::{
            simple_extension_declaration::{
                ExtensionFunction, ExtensionType, ExtensionTypeVariation,
            },
            simple_extension_uri::SimpleExtensionURI,
        },
        simple_extensions::{SimpleExtensionFunction, SimpleExtensions},
        *,
    };
    use crate::text::{
        self,
        simple_extensions::{SimpleExtensionsTypeVariationsItem, SimpleExtensionsTypesItem},
    };

    pub(crate) struct TestContext {
        mock_simple_extensions: SimpleExtensions,
    }

    impl Default for TestContext {
        fn default() -> Self {
            Self {
                mock_simple_extensions: SimpleExtensions {
                    simple_extensions: text::simple_extensions::SimpleExtensions {
                        aggregate_functions: Default::default(),
                        scalar_functions: Default::default(),
                        type_variations: Default::default(),
                        types: Default::default(),
                        window_functions: Default::default(),
                    },
                },
            }
        }
    }

    impl Context for TestContext {
        fn add_simple_extension_uri(
            &mut self,
            _simple_extension_uri: &SimpleExtensionURI,
        ) -> Result<&SimpleExtensions, ContextError> {
            Ok(&self.mock_simple_extensions)
        }

        fn simple_extensions(&self, _anchor: Anchor<SimpleExtensionURI>) -> &SimpleExtensions {
            todo!()
        }

        fn add_extension_type(
            &mut self,
            _extension_type: &ExtensionType,
        ) -> Result<&SimpleExtensionsTypesItem, ContextError> {
            todo!()
        }

        fn extension_type(
            &self,
            _extension_type_anchor: Anchor<ExtensionType>,
        ) -> &SimpleExtensionsTypesItem {
            todo!()
        }

        fn add_extension_type_variation(
            &mut self,
            _extension_type_variation: &ExtensionTypeVariation,
        ) -> Result<&SimpleExtensionsTypeVariationsItem, ContextError> {
            todo!()
        }

        fn extension_type_variation(
            &self,
            _extension_type_variation_anchor: Anchor<ExtensionTypeVariation>,
        ) -> &SimpleExtensionsTypeVariationsItem {
            todo!()
        }

        fn add_extension_function(
            &mut self,
            _extension_function: &ExtensionFunction,
        ) -> Result<&SimpleExtensionFunction, ContextError> {
            todo!()
        }

        fn extension_function(
            &self,
            _extension_function_anchor: Anchor<ExtensionFunction>,
        ) -> &SimpleExtensionFunction {
            todo!()
        }
    }
}
