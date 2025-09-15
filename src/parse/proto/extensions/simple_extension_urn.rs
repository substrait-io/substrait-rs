// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionUrn].

use std::str::FromStr;

use thiserror::Error;

use crate::{
    parse::{Anchor, Parse, context::ContextError, context::ProtoContext},
    proto,
    urn::{InvalidUrn, Urn},
};

/// A parsed [proto::extensions::SimpleExtensionUrn].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionUrn {
    /// The urn of this simple extension.
    urn: Urn,

    /// The anchor value of this simple extension.
    anchor: Anchor<Self>,
}

impl SimpleExtensionUrn {
    /// Returns the urn of this simple extension.
    ///
    /// See [proto::extensions::SimpleExtensionUrn::urn].
    pub fn urn(&self) -> &Urn {
        &self.urn
    }

    /// Returns the anchor value of this simple extension.
    ///
    /// See [proto::extensions::SimpleExtensionUrn::extension_urn_anchor].
    pub fn anchor(&self) -> Anchor<Self> {
        self.anchor
    }
}

/// Parse errors for [proto::extensions::SimpleExtensionUrn].
#[derive(Debug, Error, PartialEq)]
pub enum SimpleExtensionUrnError {
    /// Invalid Urn
    #[error("invalid urn")]
    InvalidUrn(#[from] InvalidUrn),

    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: ProtoContext> Parse<C> for proto::extensions::SimpleExtensionUrn {
    type Parsed = SimpleExtensionUrn;
    type Error = SimpleExtensionUrnError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::SimpleExtensionUrn {
            extension_urn_anchor: anchor,
            urn,
        } = self;

        // The urn is is required and must be valid.
        let urn = Urn::from_str(&urn)?;

        // Construct the parsed simple extension urn.
        let simple_extension_urn = SimpleExtensionUrn {
            urn,
            anchor: Anchor::new(anchor),
        };

        // Make sure the urn is supported by this parse context, resolves and
        // parses, and the anchor is unique.
        ctx.add_simple_extension_urn(&simple_extension_urn)?;

        Ok(simple_extension_urn)
    }
}

impl From<SimpleExtensionUrn> for proto::extensions::SimpleExtensionUrn {
    fn from(simple_extension_urn: SimpleExtensionUrn) -> Self {
        let SimpleExtensionUrn { urn, anchor } = simple_extension_urn;
        proto::extensions::SimpleExtensionUrn {
            urn: urn.to_string(),
            extension_urn_anchor: anchor.into_inner(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{Context as _, context::fixtures::Context};

    #[test]
    fn parse() -> Result<(), SimpleExtensionUrnError> {
        let simple_extension_urn = proto::extensions::SimpleExtensionUrn {
            extension_urn_anchor: 1,
            urn: "extension:substrait-rs:test".to_string(),
        };
        let simple_extension_urn = simple_extension_urn.parse(&mut Context::default())?;
        assert_eq!(simple_extension_urn.anchor(), Anchor::new(1));
        assert_eq!(
            simple_extension_urn.urn().to_string().as_str(),
            "extension:substrait-rs:test"
        );
        Ok(())
    }

    #[test]
    fn invalid_urn() {
        let simple_extension_urn = proto::extensions::SimpleExtensionUrn::default();
        assert_eq!(
            simple_extension_urn.parse(&mut Context::default()),
            Err(SimpleExtensionUrnError::InvalidUrn(InvalidUrn))
        );
        let simple_extension_urn = proto::extensions::SimpleExtensionUrn {
            extension_urn_anchor: 1,
            urn: "urn::".to_string(),
        };
        assert_eq!(
            simple_extension_urn.parse(&mut Context::default()),
            Err(SimpleExtensionUrnError::InvalidUrn(InvalidUrn))
        );
    }

    #[test]
    fn duplicate_simple_extension() {
        let mut ctx = Context::default();
        let simple_extension_urn = proto::extensions::SimpleExtensionUrn {
            extension_urn_anchor: 1,
            urn: "extension:substrait-rs:test".to_string(),
        };
        assert!(ctx.parse(simple_extension_urn.clone()).is_ok());
        assert_eq!(
            ctx.parse(simple_extension_urn),
            Err(SimpleExtensionUrnError::Context(
                ContextError::DuplicateSimpleExtension(Anchor::new(1))
            ))
        );
    }
}
