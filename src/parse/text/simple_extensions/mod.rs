// SPDX-License-Identifier: Apache-2.0

//! Parsing of [text::simple_extensions] types.

use thiserror::Error;

use crate::{
    parse::{Context, Parse},
    text,
};

#[allow(missing_docs)]
pub mod argument;

/// A parsed [text::simple_extensions::SimpleExtensions].
pub struct SimpleExtensions {
    // TODO
}

/// Parse errors for [text::simple_extensions::SimpleExtensions].
#[derive(Debug, Error, PartialEq)]
pub enum SimpleExtensionsError {
    // TODO
}

impl<C: Context> Parse<C> for text::simple_extensions::SimpleExtensions {
    type Parsed = SimpleExtensions;
    type Error = SimpleExtensionsError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        // let text::simple_extensions::SimpleExtensions {
        //     aggregate_functions,
        //     dependencies,
        //     scalar_functions,
        //     type_variations,
        //     types,
        //     window_functions,
        // } = self;

        todo!("text::simple_extensions::SimpleExtensions - https://github.com/substrait-io/substrait-rs/issues/157")
    }
}

impl From<SimpleExtensions> for text::simple_extensions::SimpleExtensions {
    fn from(_value: SimpleExtensions) -> Self {
        todo!("text::simple_extensions::SimpleExtensions - https://github.com/substrait-io/substrait-rs/issues/157")
    }
}
