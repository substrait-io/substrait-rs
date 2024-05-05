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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parse::{
        context::tests::Context,
        proto::extensions::{ExtensionFunction, ExtensionType, ExtensionTypeVariation},
        typed::Name,
        Anchor,
    };

    #[test]
    fn parse_extension_function_mapping_type() -> Result<(), SimpleExtensionDeclarationError> {
        let exntesion_function =
            proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(
                proto::extensions::simple_extension_declaration::ExtensionFunction {
                    function_anchor: 1,
                    extension_uri_reference: 2,
                    name: "test_value".to_string(),
                },
            );

        assert!(matches!(
            exntesion_function.parse(&mut Context::default())?,
            MappingType::ExtensionFunction(_)
        ));

        Ok(())
    }

    #[test]
    fn parse_extension_type_mapping_type() -> Result<(), SimpleExtensionDeclarationError> {
        let extension_type =
            proto::extensions::simple_extension_declaration::MappingType::ExtensionType(
                proto::extensions::simple_extension_declaration::ExtensionType {
                    type_anchor: 1,
                    extension_uri_reference: 2,
                    name: "test_value".to_string(),
                },
            );

        assert!(matches!(
            extension_type.parse(&mut Context::default())?,
            MappingType::ExtensionType(_)
        ));

        Ok(())
    }

    #[test]
    fn parse_extension_variation_mapping_type() -> Result<(), SimpleExtensionDeclarationError> {
        let extension_type_variation =
            proto::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(
                proto::extensions::simple_extension_declaration::ExtensionTypeVariation {
                    type_variation_anchor: 1,
                    extension_uri_reference: 2,
                    name: "test_value".to_string(),
                },
            );

        assert!(matches!(
            extension_type_variation.parse(&mut Context::default())?,
            MappingType::ExtensionTypeVariation(_)
        ));

        Ok(())
    }

    #[test]
    fn can_convert_extension_function_mapping_type_from_protobuf() {
        let extension_function = MappingType::ExtensionFunction(ExtensionFunction {
            extension_uri_reference: Anchor::new(1),
            anchor: Anchor::new(1),
            name: Name::new("test".to_string()),
        });

        let protobuf_extension_function =
            proto::extensions::simple_extension_declaration::MappingType::from(extension_function);

        assert!(matches!(
            protobuf_extension_function,
            proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(_)
        ));
    }

    #[test]
    fn can_convert_extension_type_mapping_type_from_protobuf() {
        let extension_type = MappingType::ExtensionType(ExtensionType {
            extension_uri_reference: Anchor::new(1),
            anchor: Anchor::new(1),
            name: Name::new("test".to_string()),
        });

        let protobuf_extension =
            proto::extensions::simple_extension_declaration::MappingType::from(extension_type);

        assert!(matches!(
            protobuf_extension,
            proto::extensions::simple_extension_declaration::MappingType::ExtensionType(_)
        ));
    }

    #[test]
    fn can_convert_extension_type_variation_mapping_type_from_protobuf() {
        let extension_type_variation =
            MappingType::ExtensionTypeVariation(ExtensionTypeVariation {
                extension_uri_reference: Anchor::new(1),
                anchor: Anchor::new(1),
                name: Name::new("test".to_string()),
            });

        let protobuf_extension_type_variation =
            proto::extensions::simple_extension_declaration::MappingType::from(
                extension_type_variation,
            );

        assert!(matches!(
            protobuf_extension_type_variation,
            proto::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(_)
        ));
    }
}
