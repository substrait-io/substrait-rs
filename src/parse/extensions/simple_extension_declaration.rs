// SPDX-License-Identifier: Apache-2.0

//! Parsing of [proto::extensions::SimpleExtensionDeclaration].

use crate::{
    parse::{
        extensions::simple_extension_uri::SimpleExtensionAnchor, Context, ContextError, Parse,
    },
    proto,
};
use std::fmt;
use thiserror::Error;

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

/// The name of an extension type.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeName(String);

impl ExtensionTypeName {
    /// Returns this extension type name as string slice.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// An anchor value for an [ExtensionType].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtensionTypeAnchor(u32);

impl ExtensionTypeAnchor {
    /// Returns the anchor value of this extension type anchor.
    pub fn anchor(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ExtensionTypeAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionType].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionType {
    /// The simple extension anchor value of this extension type.
    anchor: SimpleExtensionAnchor,

    /// The type anchor value of this extension type.
    type_anchor: ExtensionTypeAnchor,

    /// The name of the extension type.
    name: ExtensionTypeName,
}

impl ExtensionType {
    /// Returns the simple extension anchor of this extension type.
    pub fn simple_extension_anchor(&self) -> SimpleExtensionAnchor {
        self.anchor
    }

    /// Returns the type anchor of this extension type.
    pub fn type_anchor(&self) -> ExtensionTypeAnchor {
        self.type_anchor
    }

    /// Returns the name of this extension type.
    pub fn name(&self) -> &ExtensionTypeName {
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
            anchor: SimpleExtensionAnchor::new(extension_uri_reference),
            type_anchor: ExtensionTypeAnchor(type_anchor),
            name: ExtensionTypeName(name),
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
            extension_uri_reference: anchor.anchor(),
            type_anchor: type_anchor.anchor(),
            name: name.0,
        }
    }
}

/// The name of an extension type variation.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariationName(String);

impl ExtensionTypeVariationName {
    /// Returns this extension type variation name as string slice.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// An anchor value for an [ExtensionTypeVariation].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtensionTypeVariationAnchor(u32);

impl ExtensionTypeVariationAnchor {
    /// Returns the anchor value of this extension type variation anchor.
    pub fn anchor(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ExtensionTypeVariationAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionTypeVariation {
    /// The simple extension anchor value of this extension type variation
    anchor: SimpleExtensionAnchor,

    /// The type anchor value of this extension type variation
    type_anchor: ExtensionTypeVariationAnchor,

    /// The name of the extension type variation
    name: ExtensionTypeVariationName,
}

impl ExtensionTypeVariation {
    /// Returns the simple extension anchor of this extension type variation.
    pub fn simple_extension_anchor(&self) -> SimpleExtensionAnchor {
        self.anchor
    }

    /// Returns the type variation anchor of this extension type variation.
    pub fn type_variation_anchor(&self) -> ExtensionTypeVariationAnchor {
        self.type_anchor
    }

    /// Returns the name of this extension type variation.
    pub fn name(&self) -> &ExtensionTypeVariationName {
        &self.name
    }
}

/// Parse errors for [proto::extensions::simple_extension_declaration::ExtensionTypeVariation].
#[derive(Debug, Error)]
pub enum ExtensionTypeVariationError {
    /// Context error
    #[error(transparent)]
    Context(#[from] ContextError),
}

impl<C: Context> Parse<C>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    type Parsed = ExtensionTypeVariation;
    type Error = ExtensionTypeVariationError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let Self {
            extension_uri_reference,
            type_variation_anchor,
            name,
        } = self;

        let extension_type_variation = Self::Parsed {
            anchor: SimpleExtensionAnchor::new(extension_uri_reference),
            type_anchor: ExtensionTypeVariationAnchor(type_variation_anchor),
            name: ExtensionTypeVariationName(name),
        };

        // Add the extension type variation to the context.
        ctx.add_extension_type_variation(&extension_type_variation)?;

        Ok(extension_type_variation)
    }
}

impl From<ExtensionTypeVariation>
    for proto::extensions::simple_extension_declaration::ExtensionTypeVariation
{
    fn from(value: ExtensionTypeVariation) -> Self {
        let ExtensionTypeVariation {
            anchor,
            type_anchor,
            name,
        } = value;

        Self {
            extension_uri_reference: anchor.anchor(),
            type_variation_anchor: type_anchor.anchor(),
            name: name.name().to_string(),
        }
    }
}

/// An anchor value for an [ExtensionFunction].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExtensionFunctionAnchor(u32);

impl ExtensionFunctionAnchor {
    /// Returns the anchor value of this extension function anchor.
    pub fn anchor(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for ExtensionFunctionAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// The name of an extension function.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunctionName(String);

impl fmt::Display for ExtensionFunctionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ExtensionFunctionName {
    /// Returns this extension function name as string slice.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// A parsed [proto::extensions::simple_extension_declaration::ExtensionFunction].
#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionFunction {
    /// The simple extension anchor value of this extension function
    anchor: SimpleExtensionAnchor,

    /// The type anchor value of this extension function
    function_anchor: ExtensionFunctionAnchor,

    /// The name of the extension function
    name: ExtensionFunctionName,
}

impl ExtensionFunction {
    /// Returns the simple extension anchor of this extension function.
    pub fn simple_extension_anchor(&self) -> SimpleExtensionAnchor {
        self.anchor
    }

    /// Returns the function anchor of this extension function.
    pub fn function_anchor(&self) -> ExtensionFunctionAnchor {
        self.function_anchor
    }

    /// Returns the name of this extension function.
    pub fn name(&self) -> &ExtensionFunctionName {
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
            anchor: SimpleExtensionAnchor::new(extension_uri_reference),
            function_anchor: ExtensionFunctionAnchor(function_anchor),
            name: ExtensionFunctionName(name),
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
            extension_uri_reference: anchor.anchor(),
            function_anchor: function_anchor.anchor(),
            name: name.name().to_string(),
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
