use crate::{
    parse::{
        extensions::simple_extension_uri::SimpleExtensionURI, Anchor, Context, ContextError, Name,
        Parse,
    },
    proto,
};
use thiserror::Error;

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionType].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {
    /// The simple extension anchor value of this extension type.
    anchor: Anchor<SimpleExtensionURI>,

    /// The type anchor value of this extension type.
    type_anchor: Anchor<Self>,

    /// The name of the extension type.
    name: Name<Self>,
}

impl ExtensionType {
    /// Returns the simple extension anchor of this extension type.
    pub fn simple_extension_anchor(&self) -> Anchor<SimpleExtensionURI> {
        self.anchor
    }

    /// Returns the type anchor of this extension type.
    pub fn type_anchor(&self) -> Anchor<Self> {
        self.type_anchor
    }

    /// Returns the name of this extension type.
    pub fn name(&self) -> &Name<Self> {
        &self.name
    }
}

/// Parse errors for [proto::extensions::simple_extension_declaration::ExtensionType].
#[derive(Debug, Error)]
pub enum ExtensionTypeError {
    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::ExtensionType {
    type Parsed = ExtensionType;
    type Error = ExtensionTypeError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let Self {
            extension_uri_reference,
            type_anchor,
            name,
        } = self;

        let extension_type = Self::Parsed {
            anchor: Anchor::new(extension_uri_reference),
            type_anchor: Anchor::new(type_anchor),
            name: Name::new(name),
        };

        // Add the extension type to the context.
        ctx.add_extension_type(&extension_type)?;

        Ok(extension_type)
    }
}

impl From<ExtensionType> for proto::extensions::simple_extension_declaration::ExtensionType {
    fn from(value: ExtensionType) -> Self {
        let ExtensionType {
            anchor,
            type_anchor,
            name,
        } = value;

        Self {
            extension_uri_reference: anchor.into_inner(),
            type_anchor: type_anchor.into_inner(),
            name: name.into_inner(),
        }
    }
}
