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
//! - [ ] [plan::Plan]
//!   - [x] Version
//!   - [x] SimpleExtensionURIs
//!   - [ ] ...
//! - [x] [plan_version::PlanVersion]
//!   - [x] Version
//! - [ ] [extended_expression::ExtendedExpression]
//!   - [x] Version
//!   - [x] SimpleExtensionURIs
//!   - [ ] ...
//!

use crate::text;

use self::extensions::simple_extension_uri::{
    SimpleExtensionAnchor, SimpleExtensionURI, SimpleExtensionURIError,
};
use std::error::Error;
use thiserror::Error;

pub mod extended_expression;
pub mod extensions;
pub mod parser;
pub mod plan;
pub mod plan_relation;
pub mod plan_version;
pub mod relation;
pub mod root_relation;
pub mod version;

#[derive(Debug, Error)]
/// Context errors.
pub enum ContextError {
    /// Failed to add simple extension URI to context.
    #[error("failed to add simple extension URI to context")]
    SimpleExtensionURI(#[from] SimpleExtensionURIError),

    /// Failed to parse simple extension yaml.
    #[error("failed to parse simple extension yaml")]
    SimpleExtension(#[from] serde_yaml::Error),
}

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {
    /// Parse an item using this context.
    ///
    /// See [Parse::parse].
    fn parse<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }

    /// Add a [SimpleExtensionURI] to this context. Should return an error for
    /// duplicate anchors ([ContextError::SimpleExtensionURI(SimpleExtensionURIError::DuplicateAnchor)]) or
    /// when the URI is not supported
    /// ([ContextError::SimpleExtensionURI(SimpleExtensionURIError::Unsupported)]).
    ///
    /// This function may try to eagerly resolve and parse the simple extension,
    /// but it can't report resolve errors until the extension is referenced, or
    /// when checking for unused extensions.
    ///
    /// The default implementation returns
    /// [ContextError::SimpleExtensionURI(SimpleExtensionURIError::Unsupported)].
    fn add_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<(), ContextError> {
        let simple_extension_uri = Box::new(simple_extension_uri.clone());
        Err(ContextError::SimpleExtensionURI(
            SimpleExtensionURIError::Unsupported {
                simple_extension_uri,
                reason: "unimplemented".to_string(),
            },
        ))
    }

    /// ...
    fn simple_extension(
        &mut self,
        anchor: SimpleExtensionAnchor,
    ) -> Result<text::simple_extensions::SimpleExtensions, ContextError> {
        unimplemented!()
    }

    // /// Returns an function
    // fn extension_function(&self, )
    // fn extension_type
    // fn extension_type_variation

    /// Check the state of this context.
    ///
    /// Should be called after all relevant items are parsed using this context.
    fn check_context(&self) -> Result<(), ContextError> {
        todo!()
    }
}

/// A parse trait.
pub trait Parse<C: Context>: Sized {
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
    type Error: Error;

    /// Parse and return a parsed type or error.
    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error>;
}
