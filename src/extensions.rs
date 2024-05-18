// SPDX-License-Identifier: Apache-2.0

//! Substrait core extensions
//!
//! The contents of this module are auto-generated using `build.rs`. It is
//! included in the packaged crate, ignored by git, and automatically kept
//! in-sync.

use super::text::simple_extensions;

include!(concat!(env!("OUT_DIR"), "/extensions.in"));

/// Map with Substrait core extensions. Maps URIs to included deserialized extensions.
pub fn deserialized_extensions(
) -> &'static HashMap<&'static str, simple_extensions::SimpleExtensions> {
    use std::sync::OnceLock;

    static DESERIALIZED_EXTENSIONS: OnceLock<
        HashMap<&'static str, simple_extensions::SimpleExtensions>,
    > = OnceLock::new();

    DESERIALIZED_EXTENSIONS.get_or_init(|| {
        raw_extensions()
            .iter()
            .map(|(&url, data)| {
                (
                    url,
                    serde_yaml::from_str::<simple_extensions::SimpleExtensions>(data).unwrap(),
                )
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use crate::extensions::deserialized_extensions;

    #[test]
    fn deserialize_core_extensions() {
        // Ensure that the deserialization process does not result in any failures.
        deserialized_extensions();
    }
}
