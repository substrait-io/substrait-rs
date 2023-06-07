// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionUri].

use crate::{
    parse::{Anchor, Context, ContextError, Parse},
    proto,
};
use thiserror::Error;
use url::Url;

/// A parsed [proto::extensions::SimpleExtensionUri].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionURI {
    /// The URI of this simple extension.
    uri: Url,

    /// The anchor value of this simple extension.
    anchor: Anchor<Self>,
}

impl SimpleExtensionURI {
    /// Returns the uri of this simple extension.
    ///
    /// See [proto::extensions::SimpleExtensionUri::uri].
    pub fn uri(&self) -> &Url {
        &self.uri
    }

    /// Returns the anchor value of this simple extension.
    ///
    /// See [proto::extensions::SimpleExtensionUri::extension_uri_anchor].
    pub fn anchor(&self) -> Anchor<Self> {
        self.anchor
    }
}

/// Parse errors for [proto::extensions::SimpleExtensionUri].
#[derive(Debug, Error)]
pub enum SimpleExtensionURIError {
    /// Invalid URI
    #[error("invalid URI: {0}")]
    InvalidURI(#[from] url::ParseError),

    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C> for proto::extensions::SimpleExtensionUri {
    type Parsed = SimpleExtensionURI;
    type Error = SimpleExtensionURIError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: anchor,
            uri,
        } = self;

        // Make sure the URI is valid.
        let uri = Url::parse(&uri)?;

        // Construct the parsed simple extension URI.
        let simple_extension_uri = SimpleExtensionURI {
            uri,
            anchor: Anchor::new(anchor),
        };

        // Make sure the URI is supported by this parse context and the anchor
        // is unique.
        ctx.add_simple_extension_uri(&simple_extension_uri)?;

        Ok(simple_extension_uri)
    }
}

impl From<SimpleExtensionURI> for proto::extensions::SimpleExtensionUri {
    fn from(simple_extension_uri: SimpleExtensionURI) -> Self {
        let SimpleExtensionURI { uri, anchor } = simple_extension_uri;
        proto::extensions::SimpleExtensionUri {
            uri: uri.to_string(),
            extension_uri_anchor: anchor.into_inner(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::context::test::TestContext;

    #[test]
    fn simple_extension_uri() -> Result<(), SimpleExtensionURIError> {
        let simple_extension_uri = proto::extensions::SimpleExtensionUri::default();
        assert!(matches!(
            simple_extension_uri.parse(&mut TestContext::default()),
            Err(SimpleExtensionURIError::InvalidURI(_))
        ));

        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "https://substrait.io".to_string(),
        };
        let simple_extension_uri = simple_extension_uri.parse(&mut TestContext::default())?;
        assert_eq!(simple_extension_uri.anchor(), Anchor::new(1));
        assert_eq!(simple_extension_uri.uri().as_str(), "https://substrait.io/");

        Ok(())
    }
}
