// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionDeclaration].

use crate::proto;

/// The mapping type.
#[derive(Clone, Debug, PartialEq)]
pub enum MappingType {
    /// Describes a type
    ExtensionType(ExtensionType),
    /// Describes a type variation
    ExtensionTypeVariation(ExtensionTypeVariation),
    /// Describes a function
    ExtensionFunction(ExtensionFunction),
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionType].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {}

impl From<ExtensionType> for proto::extensions::simple_extension_declaration::ExtensionType {
    fn from(_value: ExtensionType) -> Self {
        todo!()
    }
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {}

impl From<ExtensionTypeVariation>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    fn from(_value: ExtensionTypeVariation) -> Self {
        todo!()
    }
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionFunction].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {}

impl From<ExtensionFunction>
    for proto::extensions::simple_extension_declaration::ExtensionFunction
{
    fn from(_value: ExtensionFunction) -> Self {
        todo!()
    }
}

/// A parsed [proto::extensions::SimpleExtensionDeclaration].
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionDeclaration {
    mapping_type: MappingType,
}

impl From<SimpleExtensionDeclaration> for proto::extensions::SimpleExtensionDeclaration {
    fn from(value: SimpleExtensionDeclaration) -> Self {
        let SimpleExtensionDeclaration { mapping_type } = value;
        Self {
            mapping_type: Some(mapping_type.into()),
        }
    }
}

impl From<MappingType> for proto::extensions::simple_extension_declaration::MappingType {
    fn from(value: MappingType) -> Self {
        match value {
            MappingType::ExtensionType(extension_type) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionType(
                    extension_type.into(),
                )
            }
            MappingType::ExtensionTypeVariation(extension_type_variant) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionTypeVariation(
                    extension_type_variant.into(),
                )
            }
            MappingType::ExtensionFunction(extension_function) => {
                proto::extensions::simple_extension_declaration::MappingType::ExtensionFunction(
                    extension_function.into(),
                )
            }
        }
    }
}
