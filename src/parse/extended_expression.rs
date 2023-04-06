// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::ExtendedExpression].

use super::{
    extensions::simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError},
    version::{Version, VersionError},
    Context, Parse,
};
use crate::proto;
use thiserror::Error;

/// A parsed [proto::ExtendedExpression].
pub struct ExtendedExpression {
    version: Version,
    extension_uris: Vec<SimpleExtensionURI>,
    // ...
}

impl ExtendedExpression {
    /// Returns the version of this extended expression.
    ///
    /// See [proto::ExtendedExpression::version].
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the simple extension URIs of this extended expression.
    ///
    /// See [proto::ExtendedExpression::extension_uris].
    pub fn simple_extension_uris(&self) -> &[SimpleExtensionURI] {
        &self.extension_uris
    }
}

/// Parse errors for [proto::ExtendedExpression].
#[derive(Debug, Error, PartialEq)]
pub enum ExtendedExpressionError {
    /// The version of this extended expression is invalid.
    #[error("the version of this extended expression is invalid")]
    Version(#[from] VersionError),

    /// A simple extension URI of this extended expression is invalid.
    #[error("a simple extension URI of this extended expression is invalid")]
    SimpleExtensionURI(#[from] SimpleExtensionURIError),
}

impl From<ExtendedExpression> for proto::ExtendedExpression {
    fn from(value: ExtendedExpression) -> Self {
        let ExtendedExpression {
            version: _,
            extension_uris: _,
            // ...
        } = value;

        todo!()
    }
}

impl<C: Context> Parse<C> for proto::ExtendedExpression {
    type Parsed = ExtendedExpression;
    type Error = ExtendedExpressionError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::ExtendedExpression {
            version,
            extension_uris,
            extensions: _,
            referred_expr: _,
            base_schema: _,
            advanced_extensions: _,
            expected_type_urls: _,
        } = self;

        // An extended expression requires a version.
        let version = version
            .map(|version| version.parse(ctx))
            .transpose()?
            .ok_or(VersionError::Missing)?;

        // The version must be compatible.
        version.compatible()?;

        // Parse simple extension URIs.
        let extension_uris = extension_uris
            .into_iter()
            .map(|simple_extension_uri| simple_extension_uri.parse(ctx))
            .collect::<Result<_, _>>()?;

        let _: Self::Parsed = Self::Parsed {
            version,
            extension_uris,
            // ...
        };

        todo!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        // todo
    }
}
