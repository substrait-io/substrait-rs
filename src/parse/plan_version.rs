// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::PlanVersion].

use super::{
    version::{Version, VersionError},
    Context, Parse,
};
use crate::proto;
use thiserror::Error;

/// A parsed [proto::PlanVersion].
///
/// This plan version is parsed from a [proto::PlanVersion] if:
/// - The plan version has a version
///     - The version is parsed ([proto::Version::parse])
///     - The version is compatible ([Version::compatible])
#[derive(Clone, Debug, PartialEq)]
pub struct PlanVersion {
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

impl From<PlanVersion> for proto::PlanVersion {
    fn from(value: PlanVersion) -> Self {
        let PlanVersion { version } = value;

        proto::PlanVersion {
            version: Some(version.into()),
        }
    }
}

/// Parse errors for [proto::PlanVersion].
#[derive(Debug, Error, PartialEq)]
pub enum PlanVersionError {
    /// The version of this plan version is invalid.
    #[error("the version of this plan version is invalid")]
    Version(#[from] VersionError),
}

impl<C: Context> Parse<C> for proto::PlanVersion {
    type Parsed = PlanVersion;
    type Error = PlanVersionError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::PlanVersion { version } = self;

        // A plan version requires a version, and it must be valid and compatible.
        let version = version
            .map(|version| ctx.parse_record_value(version))
            .transpose()?
            .ok_or(VersionError::Missing)?;

        Ok(PlanVersion { version })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse::test::TestContext, version};

    #[test]
    fn ok() -> Result<(), PlanVersionError> {
        let plan_version = proto::PlanVersion {
            version: Some(version::version()),
        };
        assert_eq!(
            plan_version.parse(&mut TestContext)?,
            PlanVersion {
                version: version::version().parse(&mut TestContext)?
            }
        );

        Ok(())
    }

    #[test]
    fn missing() {
        let plan_version = proto::PlanVersion::default();
        assert!(matches!(
            plan_version.parse(&mut TestContext),
            Err(PlanVersionError::Version(VersionError::Missing))
        ));
    }

    #[test]
    fn incompatible() {
        let plan_version = proto::PlanVersion {
            version: Some(proto::Version {
                major_number: u32::MAX,
                ..Default::default()
            }),
        };
        assert!(matches!(
            plan_version.parse(&mut TestContext),
            Err(PlanVersionError::Version(VersionError::Substrait(_, _)))
        ));
    }
}
