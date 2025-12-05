// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto](crate::proto) types.

mod version;
use std::collections::{HashMap, hash_map::Entry};

pub use version::{Version, VersionError};

mod plan_version;
pub use plan_version::{PlanVersion, PlanVersionError};

use crate::{
    parse::{Anchor, context::ContextError, proto::extensions::SimpleExtensionUrn},
    urn::Urn,
};

pub mod extensions;

/// Tracks the known anchors for simple extensions, as needed for parsing a
/// Substrait protobuf.
///
/// At the moment, this only tracks simple extension URNs by anchor, as further
/// parsing of simple extensions, or the protobuf plan itself, is not supported
/// yet.
#[derive(Default)]
pub struct ExtensionAnchors {
    simple_extensions: HashMap<Anchor<SimpleExtensionUrn>, Urn>,
}

impl super::Context for ExtensionAnchors {}

impl ExtensionAnchors {
    /// Register a [SimpleExtensionUrn] with this registry, rejecting duplicate
    /// anchors.
    pub fn add_simple_extension_urn(
        &mut self,
        simple_extension_urn: &SimpleExtensionUrn,
    ) -> Result<(), ContextError> {
        let anchor = simple_extension_urn.anchor();
        match self.simple_extensions.entry(anchor) {
            Entry::Occupied(_) => Err(ContextError::DuplicateSimpleExtension(anchor)),
            Entry::Vacant(entry) => {
                entry.insert(simple_extension_urn.urn().clone());
                Ok(())
            }
        }
    }

    /// Look up the [Urn] for a previously registered anchor. Returns an error
    /// if the anchor has not been seen.
    pub fn get_simple_extension_urn(
        &self,
        anchor: &Anchor<SimpleExtensionUrn>,
    ) -> Result<&Urn, ContextError> {
        self.simple_extensions
            .get(anchor)
            .ok_or(ContextError::UndefinedSimpleExtension(*anchor))
    }
}
