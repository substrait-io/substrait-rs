// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::PlanVersion].

use crate::{
    parse::{context::Context, proto::Version, Parse},
    proto,
};
use thiserror::Error;

use super::VersionError;

/// A parsed [proto::PlanVersion].
#[derive(Clone, Debug, PartialEq)]
pub struct PlanVersion {
    /// The version of the plan.
    version: Version,
}

impl PlanVersion {
    /// Returns the version of this plan version.
    ///
    /// See [proto::PlanVersion::version].
    pub fn version(&self) -> &Version {
        &self.version
    }
}

/// Parse errors for [proto::PlanVersion].
#[derive(Debug, Error, PartialEq)]
pub enum PlanVersionError {
    /// Version is missing.
    #[error("version must be specified")]
    Missing,

    /// Version error.
    #[error("version must be valid")]
    Version(#[from] VersionError),
}

impl<C: Context> Parse<C> for proto::PlanVersion {
    type Parsed = PlanVersion;
    type Error = PlanVersionError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::PlanVersion { version } = self;

        // The version is required, and must be valid.
        let version = version
            .map(|version| ctx.parse(version))
            .transpose()?
            .ok_or(PlanVersionError::Missing)?;

        let plan_version = PlanVersion { version };

        Ok(plan_version)
    }
}

impl From<PlanVersion> for proto::PlanVersion {
    fn from(plan_version: PlanVersion) -> Self {
        let PlanVersion { version } = plan_version;

        proto::PlanVersion {
            version: Some(version.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        parse::{context::tests::Context, proto::VersionError},
        version,
    };

    #[test]
    fn parse() -> Result<(), PlanVersionError> {
        let plan_version = proto::PlanVersion {
            version: Some(version::version()),
        };
        plan_version.parse(&mut Context::default())?;
        Ok(())
    }

    #[test]
    fn missing() {
        let plan_version = proto::PlanVersion::default();
        assert_eq!(
            plan_version.parse(&mut Context::default()),
            Err(PlanVersionError::Missing)
        );
    }

    #[test]
    fn version_error() {
        let plan_version = proto::PlanVersion {
            version: Some(proto::Version::default()),
        };
        assert_eq!(
            plan_version.parse(&mut Context::default()),
            Err(PlanVersionError::Version(VersionError::Missing))
        );
    }
}
