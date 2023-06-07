use thiserror::Error;

use crate::{
    parse::{
        extensions::simple_extension_uri::SimpleExtensionURI, Anchor, Context, ContextError, Name,
        Parse,
    },
    proto,
};

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionFunction].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {
    /// The simple extension anchor value of this extension function
    anchor: Anchor<SimpleExtensionURI>,

    /// The type anchor value of this extension function
    function_anchor: Anchor<Self>,

    /// The name of the extension function
    name: Name<Self>,
}

impl ExtensionFunction {
    /// Returns the simple extension anchor of this extension function.
    pub fn simple_extension_anchor(&self) -> Anchor<SimpleExtensionURI> {
        self.anchor
    }

    /// Returns the function anchor of this extension function.
    pub fn function_anchor(&self) -> Anchor<Self> {
        self.function_anchor
    }

    /// Returns the name of this extension function.
    pub fn name(&self) -> &Name<Self> {
        &self.name
    }
}

/// Parse errors for [proto::extensions::simple_extension_declaration::ExtensionFunction].
#[derive(Debug, Error)]
pub enum ExtensionFunctionError {
    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionFunction {
    type Parsed = ExtensionFunction;
    type Error = ExtensionFunctionError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let Self {
            extension_uri_reference,
            function_anchor,
            name,
        } = self;

        let extension_function = Self::Parsed {
            anchor: Anchor::new(extension_uri_reference),
            function_anchor: Anchor::new(function_anchor),
            name: Name::new(name),
        };

        // Add the extension function to the context.
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
            function_anchor,
            name,
        } = value;

        Self {
            extension_uri_reference: anchor.into_inner(),
            function_anchor: function_anchor.into_inner(),
            name: name.into_inner(),
        }
    }
}
