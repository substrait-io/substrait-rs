// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionUri].

use thiserror::Error;
use url::Url;

use crate::{
    parse::{
        context::{ContextError, ProtoContext},
        Anchor, Context, Parse,
    },
    proto,
};

/// A parsed [proto::extensions::SimpleExtensionUri].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionUri {
    /// The URI of this simple extension.
    uri: Url,

    /// The anchor value of this simple extension.
    anchor: Anchor<Self>,
}

impl SimpleExtensionUri {
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
#[derive(Debug, Error, PartialEq)]
pub enum SimpleExtensionUriError {
    /// Invalid URI
    #[error("invalid URI: {0}")]
    InvalidURI(#[from] url::ParseError),

    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: ProtoContext> Parse<C> for proto::extensions::SimpleExtensionUri {
    type Parsed = SimpleExtensionUri;
    type Error = SimpleExtensionUriError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: anchor,
            uri,
        } = self;

        // The uri is is required and must be valid.
        let uri = Url::parse(&uri)?;

        // Construct the parsed simple extension URI.
        let simple_extension_uri = SimpleExtensionUri {
            uri,
            anchor: Anchor::new(anchor),
        };

        // Make sure the URI is supported by this parse context, resolves and
        // parses, and the anchor is unique.
        ctx.add_simple_extension_uri(&simple_extension_uri)?;

        Ok(simple_extension_uri)
    }
}

impl From<SimpleExtensionUri> for proto::extensions::SimpleExtensionUri {
    fn from(simple_extension_uri: SimpleExtensionUri) -> Self {
        let SimpleExtensionUri { uri, anchor } = simple_extension_uri;
        proto::extensions::SimpleExtensionUri {
            uri: uri.to_string(),
            extension_uri_anchor: anchor.into_inner(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{context::tests::Context, Context as _};

    #[test]
    fn parse() -> Result<(), SimpleExtensionUriError> {
        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "https://substrait.io".to_string(),
        };
        let simple_extension_uri = simple_extension_uri.parse(&mut Context::default())?;
        assert_eq!(simple_extension_uri.anchor(), Anchor::new(1));
        assert_eq!(simple_extension_uri.uri().as_str(), "https://substrait.io/");
        Ok(())
    }

    #[test]
    fn invalid_uri() {
        let simple_extension_uri = proto::extensions::SimpleExtensionUri::default();
        assert_eq!(
            simple_extension_uri.parse(&mut Context::default()),
            Err(SimpleExtensionUriError::InvalidURI(
                url::ParseError::RelativeUrlWithoutBase
            ))
        );
        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "http://".to_string(),
        };
        assert_eq!(
            simple_extension_uri.parse(&mut Context::default()),
            Err(SimpleExtensionUriError::InvalidURI(
                url::ParseError::EmptyHost
            ))
        );
    }

    #[test]
    fn duplicate_simple_extension() {
        let mut ctx = Context::default();
        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "https://substrait.io".to_string(),
        };
        assert!(ctx.parse(simple_extension_uri.clone()).is_ok());
        assert_eq!(
            ctx.parse(simple_extension_uri),
            Err(SimpleExtensionUriError::Context(
                ContextError::DuplicateSimpleExtension(Anchor::new(1))
            ))
        );
    }

    #[test]
    fn unsupported_uri() {
        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "ftp://substrait.io".to_string(),
        };
        assert_eq!(
            simple_extension_uri.parse(&mut Context::default()),
            Err(SimpleExtensionUriError::Context(
                ContextError::UnsupportedURI("`ftp` scheme not supported".to_string())
            ))
        );
    }
}
