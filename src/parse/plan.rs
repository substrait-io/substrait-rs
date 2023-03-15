// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::Plan].

use super::{
    extensions::simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError},
    plan_relation::PlanRelationError,
    version::{Version, VersionError},
    Context, ContextError, Parse,
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
#[derive(Debug, PartialEq)]
pub struct Plan {
    version: Version,
    extension_uris: Vec<SimpleExtensionURI>,
    // ..
}

impl Plan {
    /// Returns the version of this plan.
    ///
    /// See [proto::Plan::version].
    pub fn version(&self) -> &Version {
        &self.version
    }
}

impl From<Plan> for proto::Plan {
    fn from(plan: Plan) -> Self {
        let Plan {
            version,
            extension_uris,
        } = plan;

        let _ = proto::Plan {
            version: Some(version.into()),
            extension_uris: extension_uris.into_iter().map(Into::into).collect(),
            ..Default::default()
        };
        todo!()
    }
}

/// Parse errors for [proto::Plan].
#[derive(Debug, Error)]
pub enum PlanError {
    /// The version of this plan is invalid.
    #[error("the version of this plan is invalid")]
    Version(#[from] VersionError),

    /// There is an error with a simple extension URI.
    #[error("invalid simple extension URI")]
    SimpleExtensionURI(#[from] SimpleExtensionURIError),

    /// There is an error with a simple extension.
    #[error("invalid simple extension")]
    SimpleExtension(#[from] ContextError),

    /// There must be a least one relation.
    #[error("plan has no relations, expected at least one")]
    MissingRelations,

    /// An issue with a plan relation.
    #[error("plan has an invalid relation")]
    PlanRelation(#[from] PlanRelationError),
}

impl<C: Context> Parse<C> for proto::Plan {
    type Parsed = Plan;
    type Error = PlanError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::Plan {
            version,
            extension_uris,
            extensions: _,
            relations,
            advanced_extensions: _,
            expected_type_urls: _,
        } = self;

        // A plan requires a version, and it must be valid.
        let version = version
            .map(|version| version.parse(ctx))
            .transpose()?
            .ok_or(VersionError::Missing)?;

        // The version must be compatible.
        version.compatible()?;

        // parse simple extension URIs.
        let _extension_uris = extension_uris
            .into_iter()
            .map(|extension_uri| extension_uri.parse(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        for x in _extension_uris {
            ctx.simple_extension(x.anchor())
                .map_err(PlanError::SimpleExtension)?;
        }

        // parse simple extension definitions.
        // extensions
        //     .into_iter()
        //     .try_for_each(|extension| extension.parse(context).map(|_| ()))?;

        // parse plan relations.
        let _plan_relations = relations
            .into_iter()
            .map(|plan_relation| plan_relation.parse(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        // Make sure there is at least one root relation?

        // Ok(Plan {
        //     version,
        //     extension_uris,
        // })

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Context;

    impl super::Context for Context {}

    #[test]
    fn version() {
        // Version could be missing.
        let plan = proto::Plan {
            version: None,
            ..Default::default()
        };
        // assert_eq!(
        //     plan.parse(&mut Context),
        //     Err(PlanError::Version(VersionError::Missing))
        // );

        // Version could be invalid.
        let plan = proto::Plan {
            version: Some(proto::Version::default()),
            ..Default::default()
        };
        // assert_eq!(
        //     plan.parse(&mut Context),
        //     Err(PlanError::Version(VersionError::Missing))
        // );

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
            plan.parse(&mut Context),
            Err(PlanError::Version(VersionError::Substrait(_, _)))
        ));
    }
}
