use super::declaration::SimpleExtensionDeclarationError;
use crate::{
    parse::{proto::extensions::SimpleExtensionUri, typed::Name, Anchor, Context, Parse},
    proto,
};

/// A parsed [`ExtensionFunction`]
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {
    /// The parsed [`Anchor`] for this [`ExtensionFunction`]
    anchor: Anchor<Self>,
    /// The parsed [`Name`] for this [`ExtensionFunction`]
    name: Name<Self>,
    /// The parsed [`SimpleExtensionUri`] for this [`ExtensionFunction`]
    extension_uri_reference: Anchor<SimpleExtensionUri>,
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionFunction {
    type Parsed = ExtensionFunction;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::simple_extension_declaration::ExtensionFunction {
            extension_uri_reference,
            function_anchor,
            name,
        } = self;

        // Construct the parsed ExtensionFunction.
        let extension_function = ExtensionFunction {
            extension_uri_reference: Anchor::new(extension_uri_reference),
            name: Name::new(name),
            anchor: Anchor::new(function_anchor),
        };

        // Add the ExtensionFunction to the given context.
        ctx.add_extension_function(&extension_function)?;

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
            name: name.into_inner(),
        }
    }
}
