use crate::{
    parse::{Context, Parse},
    proto,
};

use super::{
    declaration::SimpleExtensionDeclarationError, extension_function, extension_type,
    extension_type_variation,
};

/// A parse [`MappingType`]
#[derive(Clone, Debug, PartialEq)]
pub enum MappingType {
    /// A parsed [`ExtensionType`]
    ExtensionType(extension_type::ExtensionType),
    /// A parsed [`ExtensionTypeVariation`]
    ExtensionTypeVariation(extension_type_variation::ExtensionTypeVariation),
    /// A parsed [`ExtensionFunction`]
    ExtensionFunction(extension_function::ExtensionFunction),
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::MappingType {
    type Parsed = MappingType;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        Ok(match self {
            proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(inner) => MappingType::ExtensionFunction(inner.parse(ctx)?),
            proto::extensions::simple_extension_declaration::MappingType::ExtensionType(inner) => MappingType::ExtensionType(inner.parse(ctx)?),
            proto::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(inner) => MappingType::ExtensionTypeVariation(inner.parse(ctx)?),
        })
    }
}

impl From<MappingType> for proto::extensions::simple_extension_declaration::MappingType {
    fn from(value: MappingType) -> Self {
        match value {
            MappingType::ExtensionFunction(inner) => Self::ExtensionFunction(inner.into()),
            MappingType::ExtensionType(inner) => Self::ExtensionType(inner.into()),
            MappingType::ExtensionTypeVariation(inner) => {
                Self::ExtensionTypeVariation(inner.into())
            }
        }
    }
}
