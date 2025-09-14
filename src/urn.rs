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
        if let Some(("extension", segments)) = s.split_once(":")
            && let Some((owner, id)) = segments.split_once(":")
            && !owner.is_empty()
            && !id.is_empty()
        {
            Ok(Urn {
                owner: owner.to_string(),
                id: id.to_string(),
            })
        } else {
            Err(InvalidUrn)
        }
    }
}
