// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::AdvancedExtension].

use crate::proto;

/// A parsed [proto::extensions::AdvancedExtension].
#[derive(Clone, Debug, PartialEq)]
pub struct AdvancedExtension {}

impl From<AdvancedExtension> for proto::extensions::AdvancedExtension {
    fn from(_value: AdvancedExtension) -> Self {
        todo!()
    }
}
