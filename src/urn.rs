// SPDX-License-Identifier: Apache-2.0

//! Extension URNs

use std::{error::Error, fmt, str::FromStr};

/// Extension URN
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Urn {
    /// Represents the organization or entity providing the extension and
    /// should follow [reverse domain name convention](https://en.wikipedia.org/wiki/Reverse_domain_name_notation)
    /// (e.g., `io.substrait`, `com.example`, `org.apache.arrow`) to prevent
    /// name collisions.
    pub owner: String,

    /// The specific identifier for the extension (e.g.,
    /// `functions_arithmetic`, `custom_types`).
    pub id: String,
}

impl fmt::Display for Urn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "extension:{}:{}", self.owner, self.id)
    }
}

/// Invalid Urn
#[derive(Debug, PartialEq)]
pub struct InvalidUrn;

impl fmt::Display for InvalidUrn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid urn")
    }
}

impl Error for InvalidUrn {}

impl FromStr for Urn {
    type Err = InvalidUrn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(':')
            .filter(|(extension, _)| *extension == "extension")
            .map(|(_, segments)| segments)
            .and_then(|segments| segments.split_once(':'))
            .filter(|(owner, id)| !owner.is_empty() && !id.is_empty())
            .map(|(owner, id)| Urn {
                owner: owner.to_owned(),
                id: id.to_owned(),
            })
            .ok_or(InvalidUrn)
    }
}
