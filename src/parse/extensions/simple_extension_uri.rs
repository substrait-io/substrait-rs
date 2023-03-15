// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionUri].

use core::fmt;

use crate::{
    parse::{Context, ContextError, Parse},
    proto,
};
use thiserror::Error;
use url::Url;

/// An anchor value for a [SimpleExtensionURI].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SimpleExtensionAnchor(u32);

impl fmt::Display for SimpleExtensionAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A parsed [proto::extensions::SimpleExtensionUri].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionURI {
    /// The URI of this simple extension.
    uri: Url,
    /// The anchor value of this simple extension.
    anchor: SimpleExtensionAnchor,
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
    pub fn anchor(&self) -> SimpleExtensionAnchor {
        self.anchor
    }
}

/// Parse errors for [proto::extensions::SimpleExtensionUri].
#[derive(Debug, Error, PartialEq)]
pub enum SimpleExtensionURIError {
    /// The URI must be valid.
    #[error("simple extension URI is invalid")]
    InvalidURI(String, #[source] url::ParseError),

    /// The referenced anchor is undefined.
    #[error("referenced simple extension anchor `{0}` is undefined")]
    Undefined(SimpleExtensionAnchor),

    /// Failed to resolve simple extension URI.
    #[error("failed to resolve simple extension URI: {reason} for URI `{}` with anchor `{}`", .simple_extension_uri.uri(), .simple_extension_uri.anchor())]
    Resolve {
        /// The simple extension URI that failed to resolve.
        simple_extension_uri: Box<SimpleExtensionURI>,
        /// The reason why resolving failed.
        reason: String,
    },
    /// The anchor must be unique.
    #[error("duplicate anchor `{}` for URI `{}`, existing URI `{}`", .added.anchor(), .added.uri(), .existing)]
    DuplicateAnchor {
        /// The simple extension URI that was added later.
        added: Box<SimpleExtensionURI>,
        /// The simple extension URI that was already defined with this anchor.
        existing: Box<Url>,
    },

    /// Depending on the parse context, a simple extension URI might be unsupported.
    #[error("unsupported simple extension URI: {reason} for URI `{}` with anchor `{}`)", .simple_extension_uri.uri(), .simple_extension_uri.anchor())]
    Unsupported {
        /// The unsupported simple extension URI.
        simple_extension_uri: Box<SimpleExtensionURI>,
        /// The reason why this URI is unsupported.
        reason: String,
    },
}

impl From<ContextError> for SimpleExtensionURIError {
    fn from(value: ContextError) -> Self {
        match value {
            ContextError::SimpleExtensionURI(simple_extension_uri_error) => {
                simple_extension_uri_error
            }
            _ => unreachable!(),
        }
    }
}

impl From<SimpleExtensionURI> for proto::extensions::SimpleExtensionUri {
    fn from(simple_extension_uri: SimpleExtensionURI) -> Self {
        let SimpleExtensionURI { uri, anchor } = simple_extension_uri;
        proto::extensions::SimpleExtensionUri {
            uri: uri.to_string(),
            extension_uri_anchor: anchor.0,
        }
    }
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
        let uri = Url::parse(&uri).map_err(|e| SimpleExtensionURIError::InvalidURI(uri, e))?;

        // Construct the parsed simple extension URI.
        let simple_extension_uri = SimpleExtensionURI {
            uri,
            anchor: SimpleExtensionAnchor(anchor),
        };

        // Make sure the URI is supported by this parse context and the anchor
        // is unique.
        ctx.add_simple_extension_uri(&simple_extension_uri)?;

        Ok(simple_extension_uri)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Context;
    impl super::Context for Context {}

    struct Context2;
    impl super::Context for Context2 {
        fn add_simple_extension_uri(
            &mut self,
            _simple_extension_uri: &SimpleExtensionURI,
        ) -> Result<(), ContextError> {
            Ok(())
        }
    }

    #[test]
    fn simple_extension_uri() -> Result<(), SimpleExtensionURIError> {
        let simple_extension_uri = proto::extensions::SimpleExtensionUri::default();
        assert!(matches!(
            simple_extension_uri.parse(&mut Context),
            Err(SimpleExtensionURIError::InvalidURI(_, _))
        ));

        let simple_extension_uri = proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: 1,
            uri: "https://example.com".to_string(),
        };
        let simple_extension_uri = simple_extension_uri.parse(&mut Context2)?;
        assert_eq!(simple_extension_uri.anchor(), SimpleExtensionAnchor(1));
        assert_eq!(simple_extension_uri.uri().as_str(), "https://example.com/");

        Ok(())
    }
}
