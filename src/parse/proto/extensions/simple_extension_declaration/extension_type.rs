use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::simple_extension_declaration::SimpleExtensionDeclarationError;

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {
    anchor: Anchor<Self>,
    name: String,
    extension_uri_reference: Anchor<Self>,
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionType {
    type Parsed = ExtensionType;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionType {
            extension_uri_reference,
            type_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionType
        let extension_type_variation = ExtensionType {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name,
            anchor: Anchor::new(type_anchor),
        };

        Ok(extension_type_variation)
    }
}

impl From<ExtensionType> for proto::extensions::simple_extension_declaration::ExtensionType {
    fn from(value: ExtensionType) -> Self {
        let ExtensionType {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            type_anchor: anchor.into_inner(),
            name: name,
        }
    }
}
