use crate::{
    parse::{context::ContextError, Context, Parse},
    proto,
};
use thiserror::Error;

use super::mapping_type::MappingType;

/// A parsed [`SimpleExtensionDeclaration`].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionDeclaration {
    /// The underlying mapping type of this extension declaration
    pub mapping_type: MappingType,
}

#[derive(Debug, PartialEq, Error)]
pub enum SimpleExtensionDeclarationError {
    #[error("No Mapping Type specified on Extension Declaration.")]
    MissingMappingType,

    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C> for proto::extensions::SimpleExtensionDeclaration {
    type Parsed = SimpleExtensionDeclaration;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let proto::extensions::SimpleExtensionDeclaration { mapping_type } = self;

        Ok(SimpleExtensionDeclaration {
            mapping_type: mapping_type
                .ok_or(SimpleExtensionDeclarationError::MissingMappingType)?
                .parse(ctx)?,
        })
    }
}

impl From<SimpleExtensionDeclaration> for proto::extensions::SimpleExtensionDeclaration {
    fn from(declaration: SimpleExtensionDeclaration) -> Self {
        let SimpleExtensionDeclaration { mapping_type } = declaration;

        proto::extensions::SimpleExtensionDeclaration {
            mapping_type: Some(mapping_type.into()),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::parse::{
        context::tests::Context, proto::extensions::ExtensionFunction, typed::Name, Anchor,
    };

    #[test]
    fn parse_from_protobuf() -> Result<(), SimpleExtensionDeclarationError> {
        let declaration = proto::extensions::SimpleExtensionDeclaration {
            mapping_type: Some(
                proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(
                    proto::extensions::simple_extension_declaration::ExtensionFunction {
                        extension_uri_reference: 1,
                        function_anchor: 1,
                        name: "test_name".to_string(),
                    },
                ),
            ),
        };
        let simple_extension = declaration.parse(&mut Context::default())?;

        assert!(matches!(
            simple_extension.mapping_type,
            MappingType::ExtensionFunction(_)
        ));

        Ok(())
    }

    #[test]
    fn convert_from_parsed() {
        let declaration = SimpleExtensionDeclaration {
            mapping_type: MappingType::ExtensionFunction(ExtensionFunction {
                anchor: Anchor::new(1),
                name: Name::new("test".to_string()),
                extension_uri_reference: Anchor::new(1),
            }),
        };

        let protobuf_declaration = proto::extensions::SimpleExtensionDeclaration::from(declaration);

        assert!(matches!(
            protobuf_declaration
                .mapping_type
                .expect("No mapping_type returned from declaration conversion."),
            proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(_)
        ));
    }
}
