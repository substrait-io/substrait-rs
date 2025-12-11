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

use std::{error::Error, fmt::Debug};

mod context;
pub(crate) use context::Context;

pub mod proto;
pub mod text;

mod typed;
use typed::Anchor;

/// A parse trait.
pub(crate) trait Parse<C: Context>: Debug + Sized {
    /// The parsed type.
    ///
    /// After parsing this type must be able to convert back. Note that it is
    /// not required for the conversion to be lossless as long as the semantics
    /// don't change.
    ///
    // This bound also helps with tracking breaking Protobuf definition changes
    // via compilation errors.
    type Parsed: Into<Self>;

    /// The error type for this parser.
    type Error: Error;

    /// Parse and return a parsed type or error.
    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error>;
}
