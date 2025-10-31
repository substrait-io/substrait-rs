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
        let parts: Vec<&str> = s.split(':').collect();
        match parts.as_slice() {
            ["extension", owner, id] if !owner.is_empty() && !id.is_empty() => Ok(Urn {
                owner: owner.to_string(),
                id: id.to_string(),
            }),
            _ => Err(InvalidUrn),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_valid_urn() {
        let valid_urn = "extension:io.substrait:fake_functions";
        let result = valid_urn.parse::<Urn>();
        assert_eq!(
            result,
            Ok(Urn {
                owner: "io.substrait".to_string(),
                id: "fake_functions".to_string()
            })
        );
    }

    #[test]
    fn parse_fail_invalid_urns() {
        let invalid_urns = vec![
            "io.substrait:something",         // missing "extension:" at beginning
            "other_type:substrait:something", // incorrect type at beginning
            "extension:something",            // doesn't have both owner and id
            "extension:one:two:three",        //has too many parts
        ];
        for invalid_urn in invalid_urns {
            assert_eq!(invalid_urn.parse::<Urn>(), Err(InvalidUrn))
        }
    }
}
