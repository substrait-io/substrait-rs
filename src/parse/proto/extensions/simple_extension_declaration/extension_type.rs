use crate::parse::proto::extensions::SimpleExtensionUri;
use crate::parse::typed::Name;
use crate::parse::{Anchor, Context, Parse};
use crate::proto;

use super::declaration::SimpleExtensionDeclarationError;

/// A parsed [`ExtensionType`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {
    /// The parsed [`Anchor`] for this [`ExtensionType`]
    anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionType`]
    name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionType`]
    extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionType {
    type Parsed = ExtensionType;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionType {
            extension_uri_reference,
            type_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionType.
        let extension_type = ExtensionType {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(type_anchor),
        };

        // Add the ExtensionType to the given context.
        ctx.add_extension_type(&extension_type)?;

        Ok(extension_type)
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
            name: name.into_inner(),
        }
    }
}
