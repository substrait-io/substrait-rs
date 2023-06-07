// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionDeclaration].

use crate::{
    parse::{Context, Parse},
    proto,
};
use thiserror::Error;

pub use self::{
    extension_function::{ExtensionFunction, ExtensionFunctionError},
    extension_type::{ExtensionType, ExtensionTypeError},
    extension_type_variation::{ExtensionTypeVariation, ExtensionTypeVariationError},
};

mod extension_function;
mod extension_type;
mod extension_type_variation;

/// A parsed [proto::extensions::simple_extension_declaration::MappingType].
#[derive(Clone, Debug, PartialEq)]
pub enum MappingType {
    /// Describes a type
    ExtensionType(ExtensionType),

    /// Describes a type variation
    ExtensionTypeVariation(ExtensionTypeVariation),

    /// Describes a function
    ExtensionFunction(ExtensionFunction),
}

/// Parse errors for [proto::extensions::simple_extension_declaration::MappingType].
#[derive(Debug, Error)]
pub enum MappingTypeError {
    /// Invalid extension type.
    #[error("invalid extension type")]
    ExtensionType(#[from] ExtensionTypeError),

    /// Invalid extension type variation.
    #[error("invalid extension type variation")]
    ExtensionTypeVariation(#[from] ExtensionTypeVariationError),

    /// Invalid extension function.
    #[error("invalid extension function")]
    ExtensionFunction(#[from] ExtensionFunctionError),
}

impl<C: Context> Parse<C> for proto::extensions::simple_extension_declaration::MappingType {
    type Parsed = MappingType;
    type Error = MappingTypeError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        match self {
            Self::ExtensionType(extension_type) => ctx
                .parse_record_value(extension_type)
                .map_err(Self::Error::ExtensionType)
                .map(Self::Parsed::ExtensionType),
            Self::ExtensionTypeVariation(extension_type_variation) => ctx
                .parse_record_value(extension_type_variation)
                .map_err(Self::Error::ExtensionTypeVariation)
                .map(Self::Parsed::ExtensionTypeVariation),
            Self::ExtensionFunction(extension_function) => ctx
                .parse_record_value(extension_function)
                .map_err(Self::Error::ExtensionFunction)
                .map(Self::Parsed::ExtensionFunction),
        }
    }
}

impl From<MappingType> for proto::extensions::simple_extension_declaration::MappingType {
    fn from(value: MappingType) -> Self {
        match value {
            MappingType::ExtensionType(extension_type) => {
                Self::ExtensionType(extension_type.into())
            }
            MappingType::ExtensionTypeVariation(extension_type_variant) => {
                Self::ExtensionTypeVariation(extension_type_variant.into())
            }
            MappingType::ExtensionFunction(extension_function) => {
                Self::ExtensionFunction(extension_function.into())
            }
        }
    }
}

/// A parsed [proto::extensions::SimpleExtensionDeclaration].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionDeclaration {
    /// The type of mapping.
    mapping_type: MappingType,
}

impl SimpleExtensionDeclaration {
    /// Returns the mapping type of this simple extension declaration.
    pub fn mapping_type(&self) -> &MappingType {
        &self.mapping_type
    }
}

/// Parse errors for [proto::extensions::SimpleExtensionDeclaration].
#[derive(Debug, Error)]
pub enum SimpleExtensionDeclarationError {
    /// Mapping type is missing.
    #[error("mapping type must be specified")]
    Missing,

    /// Invalid mapping type.
    #[error("invalid mapping type")]
    MappingType(#[from] MappingTypeError),
}

impl<C: Context> Parse<C> for proto::extensions::SimpleExtensionDeclaration {
    type Parsed = SimpleExtensionDeclaration;
    type Error = SimpleExtensionDeclarationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let Self { mapping_type } = self;

        // A simple extension declaration requires a mapping type and it must be valid.
        let mapping_type = mapping_type
            .map(|mapping_type| ctx.parse(mapping_type))
            .transpose()?
            .ok_or(SimpleExtensionDeclarationError::Missing)?;

        Ok(Self::Parsed { mapping_type })
    }
}

impl From<SimpleExtensionDeclaration> for proto::extensions::SimpleExtensionDeclaration {
    fn from(value: SimpleExtensionDeclaration) -> Self {
        let SimpleExtensionDeclaration { mapping_type } = value;

        Self {
            mapping_type: Some(mapping_type.into()),
        }
    }
}
