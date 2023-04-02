// SPDX-License-Identifier: Apache-2.0

//! Substrait version information.
//!
//! The contents of this module are auto-generated using `build.rs`. It is
//! included in the packaged crate, ignored by git, and automatically kept
//! in-sync.

use crate::proto::Version;

include!("version.in");

/// Returns the version of Substrait used to build this crate.
///
/// Note that this does not set [Version::producer]. See
/// [version_with_producer].
pub fn version() -> Version {
    Version {
        major_number: SUBSTRAIT_MAJOR_VERSION.try_into().unwrap(),
        minor_number: SUBSTRAIT_MINOR_VERSION.try_into().unwrap(),
        patch_number: SUBSTRAIT_PATCH_VERSION.try_into().unwrap(),
        git_hash: if SUBSTRAIT_GIT_DEPTH != 0 {
            String::from(SUBSTRAIT_GIT_SHA)
        } else {
            String::default()
        },
        ..Default::default()
    }
}

/// Returns the version of Substrait used to build this crate with
/// [Version::producer] set to the passed `producer`.
pub fn version_with_producer(producer: impl Into<String>) -> Version {
    Version {
        producer: producer.into(),
        ..version()
    }
}

/// Returns the semantic version of Substrait used to build this crate.
#[cfg(feature = "semver")]
pub fn semver() -> semver::Version {
    semver::Version {
        major: SUBSTRAIT_MAJOR_VERSION,
        minor: SUBSTRAIT_MINOR_VERSION,
        patch: SUBSTRAIT_PATCH_VERSION,
        pre: if SUBSTRAIT_GIT_DEPTH != 0 {
            semver::Prerelease::new(&SUBSTRAIT_GIT_DEPTH.to_string()).unwrap()
        } else {
            semver::Prerelease::EMPTY
        },
        build: semver::BuildMetadata::new(SUBSTRAIT_GIT_SHA).unwrap(),
    }
}

#[cfg(test)]
// These tests ensure this crate uses a tagged Substrait release.
mod tests {
    #[test]
    fn no_git_hash() {
        // An empty `git_hash` indicates that there are no additional commits
        // since the last tag.
        assert!(super::version().git_hash.is_empty());
    }

    #[test]
    fn not_dirty() {
        // There should be no `dirty` in the describe output.
        assert!(!super::SUBSTRAIT_GIT_DESCRIBE.contains("dirty"));
        assert!(!super::SUBSTRAIT_GIT_DIRTY);
    }

    #[test]
    fn no_pre_release() {
        // The pre-release should be unset. If set it indicates additional
        // commits after the tag.
        #[cfg(feature = "semver")]
        assert!(super::semver().pre.is_empty());
        assert_eq!(super::SUBSTRAIT_GIT_DEPTH, 0);
    }
}
