// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::ExtendedExpression].

use super::{
    extensions::{
        simple_extension_declaration::{
            SimpleExtensionDeclaration, SimpleExtensionDeclarationError,
        },
        simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError},
    },
    version::{Version, VersionError},
    Context, Parse,
};
use crate::proto;
use thiserror::Error;

/// A parsed [proto::ExtendedExpression].
pub struct ExtendedExpression {
    version: Version,
    simple_extension_uris: Vec<SimpleExtensionURI>,
    simple_extension_declarations: Vec<SimpleExtensionDeclaration>,
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
        &self.simple_extension_uris
    }

    /// Returns the simple extension declarations of this extended expression.
    ///
    /// See [proto::ExtendedExpression::extensions].
    pub fn simple_extension_declarations(&self) -> &[SimpleExtensionDeclaration] {
        &self.simple_extension_declarations
    }
}

/// Parse errors for [proto::ExtendedExpression].
#[derive(Debug, Error)]
pub enum ExtendedExpressionError {
    /// The version of this extended expression is invalid.
    #[error("invalid version")]
    Version(#[from] VersionError),

    /// A simple extension URI of this extended expression is invalid.
    #[error("invalid simple extension URI")]
    SimpleExtensionURI(#[from] SimpleExtensionURIError),

    /// Invalid simple extension declaration
    #[error("invalid simple extension declaration")]
    SimpleExtensionDeclaration(#[from] SimpleExtensionDeclarationError),
}

impl From<ExtendedExpression> for proto::ExtendedExpression {
    fn from(value: ExtendedExpression) -> Self {
        let ExtendedExpression {
            version: _,
            simple_extension_uris: _,
            simple_extension_declarations: _,
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
            extensions,
            referred_expr: _,
            base_schema: _,
            advanced_extensions: _,
            expected_type_urls: _,
        } = self;

        // An extended expression requires a version.
        let _version = version
            .map(|version| ctx.parse_record_value(version))
            .transpose()?
            .ok_or(VersionError::Missing)?;

        // Parse simple extension URIs.
        let _simple_extension_uris = extension_uris
            .into_iter()
            .enumerate()
            .map(|(index, simple_extension_uri)| {
                ctx.parse_record_value_with_index(simple_extension_uri, index)
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Parse simple extension declarations.
        let _simple_extension_declarations = extensions
            .into_iter()
            .enumerate()
            .map(|(index, simple_extension_declaration)| {
                ctx.parse_record_index(simple_extension_declaration, index)
            })
            .collect::<Result<Vec<_>, _>>()?;

        todo!("referred_expr, base_schema, advanced_extensions, expected_type_urls");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok() {
        // todo
    }
}
