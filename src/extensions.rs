// SPDX-License-Identifier: Apache-2.0

//! Substrait core extensions
//!
//! The contents of this module are auto-generated using `build.rs`. It is
//! included in the packaged crate, ignored by git, and automatically kept
//! in-sync.

include!(concat!(env!("OUT_DIR"), "/extensions.in"));

#[cfg(test)]
mod tests {
    use super::*;

    use once_cell::sync::Lazy;

    #[test]
    fn core_extensions() {
        // Force evaluation of core extensions.
        Lazy::force(&EXTENSIONS);
    }
}
