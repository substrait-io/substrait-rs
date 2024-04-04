use crate::{
    parse::{Context, Parse},
    proto,
};

use super::{
    extension_function, extension_type, extension_type_variation,
    simple_extension_declaration::SimpleExtensionDeclarationError,
};

/// MappingType
#[derive(Clone, Debug, PartialEq)]
pub enum MappingType {
    ExtensionType(extension_type::ExtensionType),
    ExtensionTypeVariation(extension_type_variation::ExtensionTypeVariation),
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
            MappingType::ExtensionFunction(inner) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(
                    inner.into(),
                )
            }
            MappingType::ExtensionType(inner) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionType(
                    inner.into(),
                )
            }
            MappingType::ExtensionTypeVariation(inner) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(
                    inner.into(),
                )
            }
        }
    }
}
