// SPDX-License-Identifier: Apache-2.0

//! Parsing of Substrait data.
//!
//! Some requirements of Substrait data can not be expressed via Protobuf
//! definition or schema files. This module provides new types for the generated
//! types, that when constructed are known to be checked. This enables producers
//! and consumers to skip redundant checking of invariants described by the
//! specification.
//!
//! This is based on the idea described in the [Parse don't
//! validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/)
//! blog post.
//!
//! # Progress
//!
//! ## Proto
//!
//! - [ ] [plan::Plan]
//!   - [x] Version
//!   - [x] SimpleExtensionURI
//!   - [x] SimpleExtensionDeclaration
//!   - [ ] ...
//! - [x] [plan_version::PlanVersion]
//!   - [x] Version
//! - [ ] [extended_expression::ExtendedExpression]
//!   - [x] Version
//!   - [x] SimpleExtensionURI
//!   - [x] SimpleExtensionDeclaration
//!   - [ ] ...
//!
//! ## Text
//!
//! - [ ] [simple_extensions::SimpleExtensions]
//!   - [ ] ...
//!

use self::{
    extensions::{
        simple_extension_declaration::{
            ExtensionFunction, ExtensionFunctionAnchor, ExtensionFunctionName, ExtensionType,
            ExtensionTypeAnchor, ExtensionTypeVariation, ExtensionTypeVariationAnchor,
        },
        simple_extension_uri::{SimpleExtensionAnchor, SimpleExtensionURI},
    },
    simple_extensions::{SimpleExtensionFunction, SimpleExtensions, SimpleExtensionsError},
};
use crate::text::simple_extensions::{
    SimpleExtensionsTypeVariationsItem, SimpleExtensionsTypesItem,
};
use thiserror::Error;

pub mod extended_expression;
pub mod extensions;
pub mod parser;
pub mod plan;
pub mod plan_relation;
pub mod plan_version;
pub mod relation;
pub mod root_relation;
pub mod simple_extensions;
pub mod version;

/// Parse context errors.
#[derive(Debug, Error)]
pub enum ContextError {
    /// Undefined reference to simple extension.
    #[error("undefined reference to simple extension with anchor `{0}`")]
    UndefinedSimpleExtension(SimpleExtensionAnchor),

    /// Undefined reference to extension type.
    #[error("undefined reference to extension type with anchor `{0}`")]
    UndefinedExtensionType(ExtensionTypeAnchor),

    /// Undefined reference to extension type variation.
    #[error("undefined reference to extension type variation with anchor `{0}`")]
    UndefinedExtensionTypeVariation(ExtensionTypeVariationAnchor),

    /// Undefined reference to extension function.
    #[error("undefined reference to extension function with anchor `{0}`")]
    UndefinedExtensionFunction(ExtensionFunctionAnchor),

    /// Undefined reference to extension function.
    #[error("undefined reference to extension function with name `{0}`")]
    UndefinedExtensionFunctionDefinition(ExtensionFunctionName),

    /// Duplicate anchor for simple extension.
    #[error("duplicate anchor `{0}` for simple extension")]
    DuplicateSimpleExtension(SimpleExtensionAnchor),

    /// Duplicate anchor for extension type.
    #[error("duplicate anchor `{0}` for extension type")]
    DuplicateExtensionType(ExtensionTypeAnchor),

    /// Duplicate anchor for extension type variation.
    #[error("duplicate anchor `{0}` for extension type variation")]
    DuplicateExtensionTypeVariation(ExtensionTypeVariationAnchor),

    /// Duplicate anchor for extension function.
    #[error("duplicate anchor `{0}` for extension function")]
    DuplicateExtensionFunction(ExtensionFunctionAnchor),

    /// Duplicate simple extension URI definition.
    #[error("duplicate definition for simple extension")]
    DuplicateSimpleExtensionDefinition,

    /// The resolved extension is invalid.
    #[error("invalid simple extensions file: {0}")]
    InvalidSimpleExtensions(serde_yaml::Error),

    /// Failed to resolve simple extension URI.
    #[error("failed to resolve simple extension URI: {0}")]
    FailedSimpleExtensionResolve(reqwest::Error),

    /// Unsupported simple extension URI.
    #[error("unsupported simple extension URI: {0}")]
    UnsupportedURI(String),

    /// Simple extensions error.
    #[error(transparent)]
    SimpleExtensions(#[from] SimpleExtensionsError),
}

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
    fn simple_extensions(&self, anchor: SimpleExtensionAnchor) -> &SimpleExtensions;

    /// Add an extension type declaration to this context.
    fn add_extension_type(
        &mut self,
        extension_type: &ExtensionType,
    ) -> Result<&SimpleExtensionsTypesItem, ContextError>;

    /// Returns the extension type for the given extension type anchor.
    fn extension_type(
        &self,
        extension_type_anchor: ExtensionTypeAnchor,
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
        extension_type_variation_anchor: ExtensionTypeVariationAnchor,
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
        extension_function_anchor: ExtensionFunctionAnchor,
    ) -> &SimpleExtensionFunction;
}

/// A parse trait.
pub trait Parse<C: Context>: std::fmt::Debug + Sized {
    /// The parsed type.
    ///
    /// After parsing this type must be able to convert back. Note that it is
    /// not required for the conversion to be lossless, as long as the semantics
    /// don't change.
    ///
    /// This bound also helps with tracking breaking Protobuf definition changes
    /// via compilation errors.
    type Parsed: Into<Self>;

    /// The error type for this parser.
    type Error: std::error::Error;

    /// Parse and return a parsed type or error.
    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error>;
}

#[cfg(test)]
mod test {
    use super::*;

    pub(super) struct TestContext;

    impl Context for TestContext {
        fn add_simple_extension_uri(
            &mut self,
            _simple_extension_uri: &SimpleExtensionURI,
        ) -> Result<&SimpleExtensions, ContextError> {
            todo!()
        }

        fn simple_extensions(&self, _anchor: SimpleExtensionAnchor) -> &SimpleExtensions {
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
            _extension_type_anchor: ExtensionTypeAnchor,
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
            _extension_type_variation_anchor: ExtensionTypeVariationAnchor,
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
            _extension_function_anchor: ExtensionFunctionAnchor,
        ) -> &SimpleExtensionFunction {
            todo!()
        }
    }
}
