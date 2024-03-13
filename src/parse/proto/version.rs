// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::Version].

use crate::parse::{context::Context, Parse};
use crate::{proto, version};
use hex::FromHex;
use thiserror::Error;

/// A parsed [proto::Version].
///
/// This parses only for compatible versions. See [`version::semver_req`].
#[derive(Clone, Debug, PartialEq)]
pub struct Version {
    /// The semantic version.
    version: semver::Version,
    /// The git hash if set as bytes.
    git_hash: Option<[u8; 20]>,
    /// The producer string if set.
    producer: Option<String>,
}

impl Version {
    /// Returns the semantic version of this version.
    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    /// Returns the git hash of this version.
    pub fn git_hash(&self) -> Option<&[u8; 20]> {
        self.git_hash.as_ref()
    }

    /// Returns the producer of this version.
    pub fn producer(&self) -> Option<&str> {
        self.producer.as_deref()
    }

    /// Returns [VersionError::Substrait] if this version is incompatible with
    /// the Substrait [version::version] of this crate.
    pub(crate) fn compatible(&self) -> Result<(), VersionError> {
        let version = self.version();
        let version_req = version::semver_req();
        version_req
            .matches(version)
            .then_some(())
            .ok_or_else(|| VersionError::Substrait(version.clone(), version_req))
    }
}

/// Parse errors for [proto::Version].
#[derive(Debug, Error, PartialEq)]
pub enum VersionError {
    /// Git hash is incorrect.
    #[error(
        "git hash must be a lowercase hex ASCII string, 40 characters in length: (git hash: {0})"
    )]
    GitHash(String),

    /// Version is missing.
    #[error("version must be specified")]
    Missing,

    /// Version is incompatible.
    #[error("substrait version incompatible (version: `{0}`, supported: `{1}`)")]
    Substrait(semver::Version, semver::VersionReq),
}

impl<C: Context> Parse<C> for proto::Version {
    type Parsed = Version;
    type Error = VersionError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::Version {
            major_number,
            minor_number,
            patch_number,
            git_hash,
            producer,
        } = self;

        // All version numbers unset (u32::default()) is an error, because
        // version is required.
        if major_number == u32::default()
            && minor_number == u32::default()
            && patch_number == u32::default()
        {
            return Err(VersionError::Missing);
        }

        // The git hash, when set, must be a lowercase hex ASCII string, 40
        // characters in length.
        if !git_hash.is_empty()
            && (git_hash.len() != 40
                || !git_hash.chars().all(|x| matches!(x, '0'..='9' | 'a'..='f')))
        {
            return Err(VersionError::GitHash(git_hash));
        }

        let version = Version {
            version: semver::Version::new(major_number as _, minor_number as _, patch_number as _),
            git_hash: (!git_hash.is_empty()).then(|| <[u8; 20]>::from_hex(git_hash).unwrap()),
            producer: (!producer.is_empty()).then_some(producer),
        };

        // The version must be compatible with the substrait version of this crate.
        version.compatible()?;

        Ok(version)
    }
}

impl From<Version> for proto::Version {
    fn from(version: Version) -> Self {
        let Version {
            version,
            git_hash,
            producer,
        } = version;

        proto::Version {
            // Note: we can use `as _` here because this Version is always
            // constructed from `u32` values.
            major_number: version.major as _,
            minor_number: version.minor as _,
            patch_number: version.patch as _,
            git_hash: git_hash.map(hex::encode).unwrap_or_default(),
            producer: producer.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::context::tests::Context;

    #[test]
    fn version() {
        let version = proto::Version::default();
        assert_eq!(
            version.parse(&mut Context::default()),
            Err(VersionError::Missing)
        );

        let version = version::version();
        assert!(version.parse(&mut Context::default()).is_ok());
    }

    #[test]
    fn git_hash() {
        let base = version::version();

        // Bad length.
        let git_hash = String::from("short");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.parse(&mut Context::default()),
            Err(VersionError::GitHash(git_hash))
        );

        // Not lowercase.
        let git_hash = String::from("2FD4E1C67A2D28FCED849EE1BB76E7391B93EB12");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.parse(&mut Context::default()),
            Err(VersionError::GitHash(git_hash))
        );

        // Not all hex digits.
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb1g");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.parse(&mut Context::default()),
            Err(VersionError::GitHash(git_hash))
        );

        // Not all ascii.
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb1å");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.parse(&mut Context::default()),
            Err(VersionError::GitHash(git_hash))
        );

        // Valid.
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
        let version = proto::Version { git_hash, ..base };
        assert!(version.parse(&mut Context::default()).is_ok());
    }

    #[test]
    fn producer() -> Result<(), VersionError> {
        // Empty producer maps to `None`
        let version = proto::Version {
            producer: String::from(""),
            ..version::version()
        };
        assert!(version.parse(&mut Context::default())?.producer.is_none());
        Ok(())
    }

    #[test]
    fn convert() -> Result<(), VersionError> {
        let version = version::version();
        assert_eq!(
            proto::Version::from(version.clone().parse(&mut Context::default())?),
            version
        );
        Ok(())
    }

    #[test]
    fn compatible() -> Result<(), VersionError> {
        let _version = version::version().parse(&mut Context::default())?;

        let mut version = version::version();
        version.major_number += 1;
        let version = version.parse(&mut Context::default());
        matches!(version, Err(VersionError::Substrait(_, _)));

        let mut version = version::version();
        version.minor_number += 1;
        let version = version.parse(&mut Context::default());
        matches!(version, Err(VersionError::Substrait(_, _)));

        let mut version = version::version();
        version.patch_number += 1;
        let _version = version.parse(&mut Context::default())?;

        Ok(())
    }
}
