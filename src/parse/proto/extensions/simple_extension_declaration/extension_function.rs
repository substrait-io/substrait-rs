use super::simple_extension_declaration::SimpleExtensionDeclarationError;
use crate::{parse::{Anchor, Context, Parse}, proto};

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {
    anchor: Anchor<Self>,
    name: Name<Self>,
    extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionFunction {
    type Parsed = ExtensionFunction;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionFunction {
            extension_uri_reference,
            function_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionFunction
        let extension_function = ExtensionFunction {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name,
            anchor: Anchor::new(function_anchor),
        };

        Ok(extension_function)
    }
}

impl From<ExtensionFunction>
    for proto::extensions::simple_extension_declaration::ExtensionFunction
{
    fn from(value: ExtensionFunction) -> Self {
        let ExtensionFunction {
            anchor,
            name,
            extension_uri_reference,
        } = value;

        Self {
            extension_uri_reference: extension_uri_reference.into_inner(),
            function_anchor: anchor.into_inner(),
            name: name,
        }
    }
}
