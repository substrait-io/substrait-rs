use crate::parse::proto::extensions::SimpleExtensionUri;
use crate::parse::typed::Name;
use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::declaration::SimpleExtensionDeclarationError;

/// A parsed [`ExtensionTypeVariation`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {
    /// The parsed [`Anchor`] for this [`ExtensionTypeVariation`]
    anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionTypeVariation`]
    name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionTypeVariation`]
    extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl<C: Context> Parse<C>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    type Parsed = ExtensionTypeVariation;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
            extension_uri_reference,
            type_variation_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionTypeVariation.
        let extension_type_variation = ExtensionTypeVariation {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(type_variation_anchor),
        };

        // Add the ExtensionTypeVariation to the given context.
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
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            type_variation_anchor: anchor.into_inner(),
            name: name.into_inner(),
        }
    }
}
