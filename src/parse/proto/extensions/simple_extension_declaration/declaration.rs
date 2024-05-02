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
