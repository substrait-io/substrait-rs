// SPDX-License-Identifier: Apache-2.0

//! Substrait core extensions
//!
//! The extension files are provided by the
//! [`substrait-extensions`](https://docs.rs/substrait-extensions) crate. This
//! module exposes them keyed by their [`Urn`].

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;

use crate::text::simple_extensions::SimpleExtensions;
use crate::urn::Urn;

/// Map with Substrait core extensions. Maps [`Urn`]s to included extensions.
pub static EXTENSIONS: LazyLock<HashMap<Urn, SimpleExtensions>> = LazyLock::new(|| {
    substrait_extensions::extensions::EXTENSIONS
        .iter()
        .map(|(name, extension)| {
            let urn = Urn::from_str(&format!("extension:io.substrait:{name}")).expect("valid urn");
            (urn, extension.clone())
        })
        .collect()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_extensions() {
        // Force evaluation of core extensions.
        LazyLock::force(&EXTENSIONS);
    }
}
