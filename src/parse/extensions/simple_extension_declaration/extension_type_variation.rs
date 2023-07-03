// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].

use thiserror::Error;

use crate::{
    parse::{
        extensions::simple_extension_uri::SimpleExtensionURI, Anchor, Context, ContextError, Name,
        Parse,
    },
    proto,
};

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {
    /// The simple extension anchor value of this extension type variation
    anchor: Anchor<SimpleExtensionURI>,

    /// The type anchor value of this extension type variation
    type_anchor: Anchor<Self>,

    /// The name of the extension type variation
    name: Name<Self>,
}

impl ExtensionTypeVariation {
    /// Returns the simple extension anchor of this extension type variation.
    pub fn simple_extension_anchor(&self) -> Anchor<SimpleExtensionURI> {
        self.anchor
    }

    /// Returns the type variation anchor of this extension type variation.
    pub fn type_variation_anchor(&self) -> Anchor<Self> {
        self.type_anchor
    }

    /// Returns the name of this extension type variation.
    pub fn name(&self) -> &Name<Self> {
        &self.name
    }
}

/// Parse errors for [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].
#[derive(Debug, Error)]
pub enum ExtensionTypeVariationError {
    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    type Parsed = ExtensionTypeVariation;
    type Error = ExtensionTypeVariationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let Self {
            extension_uri_reference,
            type_variation_anchor,
            name,
        } = self;

        let extension_type_variation = Self::Parsed {
            anchor: Anchor::new(extension_uri_reference),
            type_anchor: Anchor::new(type_variation_anchor),
            name: Name::new(name),
        };

        // Add the extension type variation to the context.
        ctx.add_extension_type_variation(&extension_type_variation)?;

        Ok(extension_type_variation)
    }
}

impl From<ExtensionTypeVariation>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    fn from(value: ExtensionTypeVariation) -> Self {
        let ExtensionTypeVariation {
            anchor,
            type_anchor,
            name,
        } = value;

        Self {
            extension_uri_reference: anchor.into_inner(),
            type_variation_anchor: type_anchor.into_inner(),
            name: name.into_inner(),
        }
    }
}
