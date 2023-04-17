// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::Plan].

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

/// A parsed [proto::Plan].
///
/// This plan is parsed from a [proto::Plan] if:
/// - The plan has a version
///     - The version is parsed ([proto::Version::parse])
///     - The version is compatible ([Version::is_compatible])
/// - If the plan defines simple extension URIs:
///     - The simple extension URIs are parsed ([proto::extensions::SimpleExtensionURI::parse])
/// - If the plan defines simple extensions declarations:
///     - The simple extension declarations are parsed ([proto::extensions::SimpleExtensionDeclaration::parse])
#[derive(Debug, PartialEq)]
pub struct Plan {
    version: Version,
    simple_extension_uris: Vec<SimpleExtensionURI>,
    simple_extension_declarations: Vec<SimpleExtensionDeclaration>,
    // ...
}

impl Plan {
    /// Returns the version of this plan.
    ///
    /// See [proto::Plan::version].
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the simple extension URIs of this plan.
    ///
    /// See [proto::Plan::extension_uris].
    pub fn simple_extension_uris(&self) -> &[SimpleExtensionURI] {
        &self.simple_extension_uris
    }

    /// Returns the simple extension declarations of this plan.
    ///
    /// See [proto::Plan::extensions].
    pub fn simple_extension_declarations(&self) -> &[SimpleExtensionDeclaration] {
        &self.simple_extension_declarations
    }
}

impl From<Plan> for proto::Plan {
    fn from(plan: Plan) -> Self {
        let Plan {
            version,
            simple_extension_uris,
            simple_extension_declarations,
        } = plan;

        let _ = proto::Plan {
            version: Some(version.into()),
            extension_uris: simple_extension_uris.into_iter().map(Into::into).collect(),
            extensions: simple_extension_declarations
                .into_iter()
                .map(Into::into)
                .collect(),
            ..Default::default()
        };
        todo!()
    }
}

/// Parse errors for [proto::Plan].
#[derive(Debug, Error)]
pub enum PlanError {
    /// The version of this plan is invalid.
    #[error("invalid version")]
    Version(#[from] VersionError),

    /// There is an error with a simple extension URI.
    #[error("invalid simple extension URI")]
    SimpleExtensionURI(#[from] SimpleExtensionURIError),

    /// There is an error with a simple extension declaration.
    #[error("invalid simple extension declaration")]
    SimpleExtensionDeclaration(#[from] SimpleExtensionDeclarationError),
}

impl<C: Context> Parse<C> for proto::Plan {
    type Parsed = Plan;
    type Error = PlanError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::Plan {
            version,
            extension_uris,
            extensions,
            relations: _,
            advanced_extensions: _,
            expected_type_urls: _,
        } = self;

        // A plan requires a version, and it must be valid and compatible.
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

        // Check for unused simple extension URIs.
        // todo(mb): is this an error?

        todo!("relations, advanced_extensions, expected_type_urls")
    }
}

#[cfg(test)]
mod tests {
    use crate::parse::test::TestContext;

    use super::*;

    #[test]
    fn version() {
        // Version could be missing.
        let plan = proto::Plan {
            version: None,
            ..Default::default()
        };
        matches!(
            plan.parse(&mut TestContext),
            Err(PlanError::Version(VersionError::Missing))
        );

        // Version could be invalid.
        let plan = proto::Plan {
            version: Some(proto::Version::default()),
            ..Default::default()
        };
        matches!(
            plan.parse(&mut TestContext),
            Err(PlanError::Version(VersionError::Missing))
        );

        // Version could be incompatible.
        let plan = proto::Plan {
            version: Some(proto::Version {
                major_number: 0,
                minor_number: 0,
                patch_number: 42,
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(matches!(
            plan.parse(&mut TestContext),
            Err(PlanError::Version(VersionError::Substrait(_, _)))
        ));
    }
}
