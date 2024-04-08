use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::simple_extension_declaration::SimpleExtensionDeclarationError;

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {
    anchor: Anchor<Self>,
    name: String,
    extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl<C: Context> Parse<C>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    type Parsed = ExtensionTypeVariation;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
            extension_uri_reference,
            type_variation_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionTypeVariation
        let extension_type_variation = ExtensionTypeVariation {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name,
            anchor: Anchor::new(type_variation_anchor),
        };

        Ok(extension_type_variation)
    }
}

impl From<ExtensionTypeVariation>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    fn from(value: ExtensionTypeVariation) -> Self {
        let ExtensionTypeVariation {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            type_variation_anchor: anchor.into_inner(),
            name: name,
        }
    }
}
