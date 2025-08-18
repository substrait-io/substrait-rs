// SPDX-License-Identifier: Apache-2.0

//! Concrete type system for function validation in the registry.
//!
//! This module provides a clean, type-safe wrapper around Substrait extension types,
//! separating function signature patterns from concrete argument types.

use crate::parse::Parse;
use crate::registry::context::ExtensionContext;
use crate::text::simple_extensions::{
    EnumOptions, SimpleExtensionsTypesItem, Type as ExtType, TypeParamDefs, TypeParamDefsItem,
    TypeParamDefsItemType,
};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

/// Substrait built-in primitive types
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuiltinType {
    /// Boolean type - `bool`
    Boolean,
    /// 8-bit signed integer - `i8`
    I8,
    /// 16-bit signed integer - `i16`
    I16,
    /// 32-bit signed integer - `i32`
    I32,
    /// 64-bit signed integer - `i64`
    I64,
    /// 32-bit floating point - `fp32`
    Fp32,
    /// 64-bit floating point - `fp64`
    Fp64,
    /// Variable-length string - `string`
    String,
    /// Variable-length binary data - `binary`
    Binary,
    /// Calendar date - `date`
    Date,
    /// Time of day - `time` (deprecated, use precision_time)
    Time,
    /// Date and time - `timestamp` (deprecated, use precision_timestamp)
    Timestamp,
    /// Date and time with timezone - `timestamp_tz` (deprecated, use precision_timestamp_tz)
    TimestampTz,
    /// Year-month interval - `interval_year`
    IntervalYear,
    /// Day-time interval - `interval_day`
    IntervalDay,
    /// Compound interval - `interval_compound`
    IntervalCompound,
    /// UUID type - `uuid`
    Uuid,
    /// Fixed-length character string - `fixed_char`
    FixedChar,
    /// Variable-length character string - `varchar`
    VarChar,
    /// Fixed-length binary data - `fixed_binary`
    FixedBinary,
    /// Decimal number - `decimal`
    Decimal,
    /// Time with precision - `precision_time`
    PrecisionTime,
    /// Timestamp with precision - `precision_timestamp`
    PrecisionTimestamp,
    /// Timestamp with timezone and precision - `precision_timestamp_tz`
    PrecisionTimestampTz,
    /// Struct/record type - `struct`
    Struct,
    /// List/array type - `list`
    List,
    /// Map/dictionary type - `map`
    Map,
    /// User-defined type - `user_defined`
    UserDefined,
}

#[derive(Debug, thiserror::Error)]
/// Error for unrecognized builtin type strings
#[error("Unrecognized builtin type")]
pub struct UnrecognizedBuiltin;

impl FromStr for BuiltinType {
    type Err = UnrecognizedBuiltin;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "boolean" => Ok(BuiltinType::Boolean),
            "i8" => Ok(BuiltinType::I8),
            "i16" => Ok(BuiltinType::I16),
            "i32" => Ok(BuiltinType::I32),
            "i64" => Ok(BuiltinType::I64),
            "fp32" => Ok(BuiltinType::Fp32),
            "fp64" => Ok(BuiltinType::Fp64),
            "string" => Ok(BuiltinType::String),
            "binary" => Ok(BuiltinType::Binary),
            "date" => Ok(BuiltinType::Date),
            "time" => Ok(BuiltinType::Time),
            "timestamp" => Ok(BuiltinType::Timestamp),
            "timestamp_tz" => Ok(BuiltinType::TimestampTz),
            "interval_year" => Ok(BuiltinType::IntervalYear),
            "interval_day" => Ok(BuiltinType::IntervalDay),
            "interval_compound" => Ok(BuiltinType::IntervalCompound),
            "uuid" => Ok(BuiltinType::Uuid),
            "fixed_char" => Ok(BuiltinType::FixedChar),
            "varchar" => Ok(BuiltinType::VarChar),
            "fixed_binary" => Ok(BuiltinType::FixedBinary),
            "decimal" => Ok(BuiltinType::Decimal),
            "precision_time" => Ok(BuiltinType::PrecisionTime),
            "precision_timestamp" => Ok(BuiltinType::PrecisionTimestamp),
            "precision_timestamp_tz" => Ok(BuiltinType::PrecisionTimestampTz),
            "struct" => Ok(BuiltinType::Struct),
            "list" => Ok(BuiltinType::List),
            "map" => Ok(BuiltinType::Map),
            "user_defined" => Ok(BuiltinType::UserDefined),
            _ => Err(UnrecognizedBuiltin),
        }
    }
}
/// Parameter type for extension type definitions
#[derive(Clone, Debug, PartialEq)]
pub enum ParameterType {
    /// A type name
    DataType,
    /// True/False
    Boolean,
    /// Integer
    Integer,
    /// A particular enum
    Enum,
    /// A string
    String,
}

/// Type-safe parameter constraints based on parameter kind
#[derive(Clone, Debug)]
pub enum ParamKind {
    /// A type name parameter
    DataType,
    /// True/False parameter
    Boolean,
    /// Integer parameter with optional bounds
    Integer {
        /// Minimum value constraint
        min: Option<i64>,
        /// Maximum value constraint
        max: Option<i64>,
    },
    /// Enumeration parameter with predefined options
    Enumeration {
        /// Valid enumeration values
        options: Vec<String>,
    },
    /// String parameter
    String,
}

impl ParamKind {
    fn get_integer_bounds(
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<(Option<i64>, Option<i64>), TypeParamError> {
        // Convert float bounds to integers, validating they are whole numbers
        let min_bound = if let Some(min_f) = min {
            if min_f.fract() != 0.0 {
                return Err(TypeParamError::InvalidIntegerBounds { min, max });
            }
            Some(min_f as i64)
        } else {
            None
        };

        let max_bound = if let Some(max_f) = max {
            if max_f.fract() != 0.0 {
                return Err(TypeParamError::InvalidIntegerBounds { min, max });
            }
            Some(max_f as i64)
        } else {
            None
        };

        Ok((min_bound, max_bound))
    }

    /// Create a ParamKind from TypeParamDefsItemType and associated fields
    fn try_from_item_parts(
        item_type: TypeParamDefsItemType,
        min: Option<f64>,
        max: Option<f64>,
        options: Option<EnumOptions>,
    ) -> Result<Self, TypeParamError> {
        match (item_type, min, max, options) {
            // Valid cases - each type with its expected parameters
            (TypeParamDefsItemType::DataType, None, None, None) => Ok(ParamKind::DataType),
            (TypeParamDefsItemType::Boolean, None, None, None) => Ok(ParamKind::Boolean),
            (TypeParamDefsItemType::Integer, min, max, None) => {
                let (min_bound, max_bound) = Self::get_integer_bounds(min, max)?;
                Ok(ParamKind::Integer {
                    min: min_bound,
                    max: max_bound,
                })
            }
            (TypeParamDefsItemType::Enumeration, None, None, Some(enum_options)) => {
                Ok(ParamKind::Enumeration {
                    options: enum_options.0,
                })
            }
            (TypeParamDefsItemType::String, None, None, None) => Ok(ParamKind::String),

            // Error cases - DataType with unexpected parameters
            (TypeParamDefsItemType::DataType, Some(_), _, _)
            | (TypeParamDefsItemType::DataType, _, Some(_), _) => {
                Err(TypeParamError::UnexpectedMinMaxBounds {
                    param_type: TypeParamDefsItemType::DataType,
                })
            }
            (TypeParamDefsItemType::DataType, None, None, Some(_)) => {
                Err(TypeParamError::UnexpectedEnumOptions {
                    param_type: TypeParamDefsItemType::DataType,
                })
            }

            // Error cases - Boolean with unexpected parameters
            (TypeParamDefsItemType::Boolean, Some(_), _, _)
            | (TypeParamDefsItemType::Boolean, _, Some(_), _) => {
                Err(TypeParamError::UnexpectedMinMaxBounds {
                    param_type: TypeParamDefsItemType::Boolean,
                })
            }
            (TypeParamDefsItemType::Boolean, None, None, Some(_)) => {
                Err(TypeParamError::UnexpectedEnumOptions {
                    param_type: TypeParamDefsItemType::Boolean,
                })
            }

            // Error cases - Integer with enum options
            (TypeParamDefsItemType::Integer, _, _, Some(_)) => {
                Err(TypeParamError::UnexpectedEnumOptions {
                    param_type: TypeParamDefsItemType::Integer,
                })
            }

            // Error cases - Enumeration with unexpected parameters
            (TypeParamDefsItemType::Enumeration, Some(_), _, _)
            | (TypeParamDefsItemType::Enumeration, _, Some(_), _) => {
                Err(TypeParamError::UnexpectedMinMaxBounds {
                    param_type: TypeParamDefsItemType::Enumeration,
                })
            }
            (TypeParamDefsItemType::Enumeration, None, None, None) => {
                Err(TypeParamError::MissingEnumOptions)
            }

            // Error cases - String with unexpected parameters
            (TypeParamDefsItemType::String, Some(_), _, _)
            | (TypeParamDefsItemType::String, _, Some(_), _) => {
                Err(TypeParamError::UnexpectedMinMaxBounds {
                    param_type: TypeParamDefsItemType::String,
                })
            }
            (TypeParamDefsItemType::String, None, None, Some(_)) => {
                Err(TypeParamError::UnexpectedEnumOptions {
                    param_type: TypeParamDefsItemType::String,
                })
            }
        }
    }
}

/// Type parameter definition for custom types
#[derive(Clone, Debug)]
pub struct TypeParam {
    /// Name of the parameter (required)
    pub name: String,
    /// Optional description of the parameter
    pub description: Option<String>,
    /// Type-safe parameter constraints
    pub kind: ParamKind,
}

impl TryFrom<TypeParamDefsItem> for TypeParam {
    type Error = TypeParamError;

    fn try_from(item: TypeParamDefsItem) -> Result<Self, Self::Error> {
        let name = item.name.ok_or(TypeParamError::MissingName)?;

        let kind = ParamKind::try_from_item_parts(item.type_, item.min, item.max, item.options)?;

        Ok(Self {
            name,
            description: item.description,
            kind,
        })
    }
}

impl From<TypeParam> for TypeParamDefsItem {
    fn from(param_def: TypeParam) -> Self {
        let (param_type, min, max, options) = match param_def.kind {
            ParamKind::DataType => (TypeParamDefsItemType::DataType, None, None, None),
            ParamKind::Boolean => (TypeParamDefsItemType::Boolean, None, None, None),
            ParamKind::Integer { min, max } => (
                TypeParamDefsItemType::Integer,
                min.map(|i| i as f64),
                max.map(|i| i as f64),
                None,
            ),
            ParamKind::Enumeration { options } => (
                TypeParamDefsItemType::Enumeration,
                None,
                None,
                Some(EnumOptions(options)),
            ),
            ParamKind::String => (TypeParamDefsItemType::String, None, None, None),
        };

        Self {
            name: Some(param_def.name),
            description: param_def.description,
            type_: param_type,
            min,
            max,
            optional: None, // Not needed for type definitions
            options,
        }
    }
}

/// Error types for ExtensionType parsing
#[derive(Debug, Error, PartialEq)]
pub enum ExtensionTypeError {
    /// Extension type name is invalid
    #[error("Invalid extension type name: {name}")]
    InvalidName {
        /// The invalid name
        name: String,
    },
    /// Parameter validation failed
    #[error("Invalid parameter: {0}")]
    InvalidParameter(#[from] TypeParamError),
    /// Field type is invalid
    #[error("Invalid structure field type: {0}")]
    InvalidFieldType(String),
    /// Structure representation cannot be nullable
    #[error("Structure representation cannot be nullable: {type_string}")]
    StructureCannotBeNullable {
        /// The type string that was nullable
        type_string: String,
    },
}

/// Error types for TypeParam validation
#[derive(Debug, Error, PartialEq)]
pub enum TypeParamError {
    /// Parameter name is missing
    #[error("Parameter name is required")]
    MissingName,
    /// Integer parameter has non-integer min/max values
    #[error("Integer parameter has invalid min/max values: min={min:?}, max={max:?}")]
    InvalidIntegerBounds {
        /// The invalid minimum value
        min: Option<f64>,
        /// The invalid maximum value
        max: Option<f64>,
    },
    /// Parameter type cannot have min/max bounds
    #[error("Parameter type '{param_type}' cannot have min/max bounds")]
    UnexpectedMinMaxBounds {
        /// The parameter type that cannot have bounds
        param_type: TypeParamDefsItemType,
    },
    /// Parameter type cannot have enumeration options
    #[error("Parameter type '{param_type}' cannot have enumeration options")]
    UnexpectedEnumOptions {
        /// The parameter type that cannot have options
        param_type: TypeParamDefsItemType,
    },
    /// Enumeration parameter is missing required options
    #[error("Enumeration parameter is missing required options")]
    MissingEnumOptions,
}

/// A custom type definition
#[derive(Clone, Debug)]
pub struct CustomType {
    /// The name of this custom type
    pub name: String,
    /// Optional description of this type
    pub description: Option<String>,
    /// How this type is represented (None = opaque, Some = structured representation)
    /// If Some, nullable MUST be false
    pub structure: Option<ConcreteType>,
    /// Parameters for this type (empty if none)
    pub parameters: Vec<TypeParam>,
    // TODO: Add variadic field for variadic type support
}

impl PartialEq for CustomType {
    fn eq(&self, other: &Self) -> bool {
        // Name should be unique for a given extension file
        self.name == other.name
    }
}

impl CustomType {
    /// Get the name of this custom type
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<CustomType> for SimpleExtensionsTypesItem {
    fn from(custom_type: CustomType) -> Self {
        Self {
            name: custom_type.name,
            description: custom_type.description,
            parameters: if custom_type.parameters.is_empty() {
                None
            } else {
                Some(TypeParamDefs(
                    custom_type.parameters.into_iter().map(Into::into).collect(),
                ))
            },
            structure: None, // TODO: Add structure support
            variadic: None,  // TODO: Add variadic support
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid type name: {0}")]
/// Error for invalid type names in extension definitions
pub struct InvalidTypeName(String);

impl Parse<ExtensionContext> for SimpleExtensionsTypesItem {
    type Parsed = CustomType;
    type Error = ExtensionTypeError;

    fn parse(self, ctx: &mut ExtensionContext) -> Result<Self::Parsed, Self::Error> {
        let SimpleExtensionsTypesItem {
            name,
            description,
            parameters,
            structure,
            variadic: _, // TODO: Add variadic support
        } = self;

        // TODO: Not all names are valid for types, we should validate that
        if name.is_empty() {
            return Err(ExtensionTypeError::InvalidName { name });
        }

        let parameters = match parameters {
            Some(type_param_defs) => {
                let mut parsed_params = Vec::new();
                for item in type_param_defs.0 {
                    parsed_params.push(TypeParam::try_from(item)?);
                }
                parsed_params
            }
            None => Vec::new(),
        };

        // Parse structure field if present
        let structure = match structure {
            Some(structure_data) => Some(ConcreteType::try_from(structure_data)?),
            None => None, // Opaque type
        };

        let custom_type = CustomType {
            name: name.clone(),
            description,
            structure,
            parameters,
        };

        ctx.add_type(&custom_type);
        Ok(custom_type)
    }
}

impl TryFrom<ExtType> for ConcreteType {
    type Error = ExtensionTypeError;

    fn try_from(ext_type: ExtType) -> Result<Self, Self::Error> {
        match ext_type {
            // Case: structure: "BINARY" (alias to another type)
            ExtType::Variant0(type_string) => {
                let parsed_type = ParsedType::parse(&type_string);
                let concrete_type = ConcreteType::try_from(parsed_type)?;

                // Structure representation cannot be nullable
                if concrete_type.nullable {
                    return Err(ExtensionTypeError::InvalidName {
                        name: format!("Structure representation '{type_string}' cannot be nullable"),
                    });
                }

                Ok(concrete_type)
            }
            // Case: structure: { field1: type1, field2: type2 } (named struct)
            ExtType::Variant1(field_map) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();

                for (field_name, field_type_variant) in field_map {
                    field_names.push(field_name);

                    let field_type_str = match field_type_variant {
                        Value::String(s) => s,
                        _ => {
                            return Err(ExtensionTypeError::InvalidName {
                                name: field_type_variant.to_string(),
                            })
                        }
                    };

                    let parsed_field_type = ParsedType::parse(&field_type_str);
                    let field_concrete_type = ConcreteType::try_from(parsed_field_type)?;
                    field_types.push(field_concrete_type);
                }

                Ok(ConcreteType {
                    base: KnownType::NStruct(field_names),
                    nullable: false, // Structure representation cannot be nullable
                    parameters: field_types,
                })
            }
        }
    }
}

/// Error for invalid Type specifications
#[derive(Debug, thiserror::Error)]
pub enum TypeParseError {
    /// Extension type name not found in context
    #[error("Extension type '{name}' not found")]
    ExtensionTypeNotFound {
        /// The extension type name that was not found
        name: String,
    },
    /// Type variable ID is invalid (must be >= 1)
    #[error("Type variable 'any{id}' is invalid (must be >= 1)")]
    InvalidTypeVariableId {
        /// The invalid type variable ID
        id: u32,
    },
    /// Unimplemented Type variant
    #[error("Unimplemented Type variant")]
    UnimplementedVariant,
}

// TODO: ValidatedType will be updated when we implement proper type validation

// TODO: Update this Parse implementation when ValidatedType and ParsedType are converted to owned types
// impl Parse<ExtensionContext> for &extType {
//     type Parsed = ValidatedType;
//     type Error = TypeParseError;
//     fn parse(self, ctx: &mut ExtensionContext) -> Result<Self::Parsed, Self::Error> {
//         todo!("Update when ValidatedType and ParsedType are owned")
//     }
// }

/// Error for invalid ArgumentsItem specifications (TODO: Update when ArgumentPattern is owned)
#[derive(Debug, thiserror::Error)]
pub enum ArgumentsItemError {
    /// Type parsing failed
    #[error("Type parsing failed: {0}")]
    TypeParseError(#[from] TypeParseError),
    /// Unsupported ArgumentsItem variant
    #[error("Unimplemented ArgumentsItem variant: {variant}")]
    UnimplementedVariant {
        /// The unsupported variant name
        variant: String,
    },
}

// TODO: Update this Parse implementation when ArgumentPattern is converted to owned type
// impl Parse<ExtensionContext> for &ArgumentsItem {
//     type Parsed = ArgumentPattern;
//     type Error = ArgumentsItemError;
//     fn parse(self, ctx: &mut ExtensionContext) -> Result<Self::Parsed, Self::Error> {
//         todo!("Update when ArgumentPattern is owned")
//     }
// }

/// Represents a known, specific type, either builtin, extension reference, or structured
#[derive(Clone, Debug, PartialEq)]
pub enum KnownType {
    /// Built-in primitive types
    Builtin(BuiltinType),
    /// Custom types defined in extension YAML files (unresolved reference)
    Extension(String),
    /// Named struct with field names (corresponds to Substrait's NSTRUCT pseudo-type)
    NStruct(Vec<String>),
}

impl FromStr for KnownType {
    type Err = ExtensionTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First try to parse as a builtin type
        match BuiltinType::from_str(s) {
            Ok(builtin) => Ok(KnownType::Builtin(builtin)),
            Err(_) => {
                // TODO: Validate that the string is a valid type name
                // For now, treat all non-builtin strings as extension type references
                Ok(KnownType::Extension(s.to_string()))
            }
        }
    }
}

/// A concrete type, fully specified with nullability and parameters
#[derive(Clone, Debug, PartialEq)]
pub struct ConcreteType {
    /// Base type, can be builtin or extension
    pub base: KnownType,
    /// Is the overall type nullable?
    pub nullable: bool,
    // TODO: Add non-type parameters (e.g. integers, enum, etc.)
    /// Parameters for the type, if there are any
    pub parameters: Vec<ConcreteType>,
}

impl ConcreteType {
    /// Create a concrete type from a builtin type
    pub fn builtin(builtin_type: BuiltinType, nullable: bool) -> ConcreteType {
        ConcreteType {
            base: KnownType::Builtin(builtin_type),
            nullable,
            parameters: Vec::new(),
        }
    }

    /// Create a concrete type from an extension type name
    pub fn extension(type_name: impl Into<String>, nullable: bool) -> Self {
        Self {
            base: KnownType::Extension(type_name.into()),
            nullable,
            parameters: Vec::new(),
        }
    }

    /// Create a concrete type for a named struct (NSTRUCT)
    pub fn nstruct(
        field_names: Vec<String>,
        field_types: Vec<ConcreteType>,
        nullable: bool,
    ) -> Self {
        Self {
            base: KnownType::NStruct(field_names),
            nullable,
            parameters: field_types,
        }
    }

    /// Create a parameterized concrete type
    pub fn parameterized(base: KnownType, nullable: bool, parameters: Vec<ConcreteType>) -> Self {
        Self {
            base,
            nullable,
            parameters,
        }
    }
}

impl<'a> TryFrom<ParsedType<'a>> for ConcreteType {
    type Error = ExtensionTypeError;

    fn try_from(parsed: ParsedType<'a>) -> Result<Self, Self::Error> {
        match parsed {
            ParsedType::Builtin(builtin_type, nullable) => {
                Ok(ConcreteType::builtin(builtin_type, nullable))
            }
            ParsedType::NamedExtension(type_name, nullable) => {
                Ok(ConcreteType::extension(type_name.to_string(), nullable))
            }
            ParsedType::TypeVariable(_) | ParsedType::NullableTypeVariable(_) => {
                Err(ExtensionTypeError::InvalidName {
                    name: "Type variables not allowed in structure definitions".to_string(),
                })
            }
            ParsedType::Parameterized {
                base,
                parameters,
                nullable,
            } => {
                let base_concrete = ConcreteType::try_from(*base)?;
                let param_concretes: Result<Vec<_>, _> =
                    parameters.into_iter().map(ConcreteType::try_from).collect();
                Ok(ConcreteType::parameterized(
                    base_concrete.base,
                    nullable,
                    param_concretes?,
                ))
            }
        }
    }
}

/// A parsed type that can represent type variables, builtin types, extension types, or parameterized types
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedType<'a> {
    /// Type variable like any1, any2, etc.
    TypeVariable(u32),
    /// Nullable type variable like any1?, any2?, etc.; used in return types
    NullableTypeVariable(u32),
    /// Built-in primitive type, with nullability
    Builtin(BuiltinType, bool),
    /// Extension type for the given name, with nullability. URI not known at this level.
    NamedExtension(&'a str, bool),
    /// Parameterized type
    Parameterized {
        /// Base type, can be builtin or extension
        base: Box<ParsedType<'a>>,
        /// Parameters for that type
        parameters: Vec<ParsedType<'a>>,
        /// Is the overall type nullable?
        nullable: bool,
    },
}

impl<'a> ParsedType<'a> {
    /// Parse a type string into a ParsedType
    pub fn parse(type_str: &'a str) -> ParsedType<'a> {
        // Strip nullability
        let (type_str, nullability) = if let Some(rest) = type_str.strip_suffix('?') {
            (rest, true)
        } else {
            (type_str, false)
        };

        // Handle any expressions
        if let Some(rest) = type_str.strip_prefix("any") {
            if let Ok(id) = rest.parse::<u32>() {
                if nullability {
                    // any1? etc. are nullable type variables - permissible in
                    // return position
                    return ParsedType::NullableTypeVariable(id);
                } else {
                    return ParsedType::TypeVariable(id);
                }
            }
        }

        // Handle parameterized types like "list<i32>" (future implementation)
        if type_str.contains('<') && type_str.ends_with('>') {
            unimplemented!("Parameterized types not yet implemented: {}", type_str);
        }

        // Try to parse as builtin type
        if let Ok(builtin_type) = BuiltinType::from_str(type_str) {
            return ParsedType::Builtin(builtin_type, nullability);
        }

        // Not a builtin or type variable - assume it's an extension type name
        ParsedType::NamedExtension(type_str, nullability)
    }
}

/// A pattern for function arguments that can match concrete types or type variables (TODO: Remove lifetime when ArgumentPattern is owned)
#[derive(Clone, Debug, PartialEq)]
pub enum ArgumentPattern {
    /// Type variable like any1, any2, etc.
    TypeVariable(u32),
    /// Concrete type pattern
    Concrete(ConcreteType),
}

/// Result of matching an argument pattern against a concrete type (TODO: Remove lifetime when Match is owned)
#[derive(Clone, Debug, PartialEq)]
pub enum Match {
    /// Pattern matched exactly (for concrete patterns)
    Concrete,
    /// Type variable bound to concrete type
    Variable(u32, ConcreteType),
    /// Match failed
    Fail,
}

impl ArgumentPattern {
    /// Check if this pattern matches the given concrete type
    pub fn matches(&self, concrete: &ConcreteType) -> Match {
        match self {
            ArgumentPattern::TypeVariable(id) => Match::Variable(*id, concrete.clone()),
            ArgumentPattern::Concrete(pattern_type) => {
                if pattern_type == concrete {
                    Match::Concrete
                } else {
                    Match::Fail
                }
            }
        }
    }
}

/// Type variable bindings from matching function arguments (TODO: Remove lifetime when TypeBindings is owned)
#[derive(Debug, Clone, PartialEq)]
pub struct TypeBindings {
    /// Map of type variable IDs (e.g. 1 for 'any1') to their concrete types
    pub vars: HashMap<u32, ConcreteType>,
}

impl TypeBindings {
    /// Create type bindings by matching argument patterns against concrete arguments
    pub fn new(patterns: &[ArgumentPattern], args: &[ConcreteType]) -> Option<Self> {
        // Check length compatibility
        if patterns.len() != args.len() {
            unimplemented!("Handle variadic functions");
        }

        let mut vars = HashMap::new();

        // Match each pattern against corresponding argument
        for (pattern, arg) in patterns.iter().zip(args.iter()) {
            match pattern.matches(arg) {
                Match::Concrete => {
                    // Concrete pattern matched, continue
                    continue;
                }
                Match::Variable(id, concrete_type) => {
                    // Check for consistency with existing bindings
                    if let Some(existing_binding) = vars.get(&id) {
                        if existing_binding != &concrete_type {
                            // Conflicting binding - type variable bound to different types
                            return None;
                        }
                    } else {
                        // New binding
                        vars.insert(id, concrete_type);
                    }
                }
                Match::Fail => {
                    // Pattern didn't match
                    return None;
                }
            }
        }

        Some(TypeBindings { vars })
    }

    /// Get the bound type for a type variable, if any
    pub fn get(&self, var_id: u32) -> Option<&ConcreteType> {
        self.vars.get(&var_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use url::Url;

    #[test]
    fn test_extension_type_parse_basic() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri.clone());

        let original_type_item = SimpleExtensionsTypesItem {
            name: "MyType".to_string(),
            description: Some("A test type".to_string()),
            parameters: None,
            structure: None,
            variadic: None,
        };

        let result = original_type_item.clone().parse(&mut ctx);
        assert!(result.is_ok());

        let custom_type = result.unwrap();
        assert_eq!(custom_type.name, "MyType");
        assert_eq!(custom_type.description, Some("A test type".to_string()));
        assert!(custom_type.parameters.is_empty());

        // Test round-trip conversion
        let converted_back: SimpleExtensionsTypesItem = custom_type.into();
        assert_eq!(converted_back.name, original_type_item.name);
        assert_eq!(converted_back.description, original_type_item.description);
        // Note: structure and variadic are TODO fields
    }

    #[test]
    fn test_extension_type_parse_with_parameters() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri.clone());

        let original_type_item = SimpleExtensionsTypesItem {
            name: "ParameterizedType".to_string(),
            description: None,
            parameters: Some(TypeParamDefs(vec![
                TypeParamDefsItem {
                    name: Some("length".to_string()),
                    description: Some("The length parameter".to_string()),
                    type_: TypeParamDefsItemType::Integer,
                    min: Some(1.0),
                    max: Some(1000.0),
                    optional: Some(false),
                    options: None,
                },
            ])),
            structure: None,
            variadic: None,
        };

        let result = original_type_item.clone().parse(&mut ctx);
        assert!(result.is_ok());

        let custom_type = result.unwrap();
        assert_eq!(custom_type.name, "ParameterizedType");
        assert_eq!(custom_type.parameters.len(), 1);

        let param = &custom_type.parameters[0];
        assert_eq!(param.name, "length");
        assert_eq!(param.description, Some("The length parameter".to_string()));
        if let ParamKind::Integer { min, max } = &param.kind {
            assert_eq!(*min, Some(1));
            assert_eq!(*max, Some(1000));
        } else {
            panic!("Expected Integer parameter kind");
        }

        // Test round-trip conversion
        let converted_back: SimpleExtensionsTypesItem = custom_type.into();
        assert_eq!(converted_back.name, original_type_item.name);
        assert_eq!(converted_back.description, original_type_item.description);
        // Note: parameter and structure comparisons would require PartialEq implementations
    }

    #[test]
    fn test_extension_type_parse_empty_name_error() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri);

        let type_item = SimpleExtensionsTypesItem {
            name: "".to_string(), // Empty name should cause error
            description: None,
            parameters: None,
            structure: None,
            variadic: None,
        };

        let result = type_item.parse(&mut ctx);
        assert!(result.is_err());

        if let Err(ExtensionTypeError::InvalidName { name }) = result {
            assert_eq!(name, "");
        } else {
            panic!("Expected InvalidName error");
        }
    }

    #[test]
    fn test_extension_context_type_tracking() {
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri.clone());

        // Initially no types
        assert!(!ctx.has_type("MyType"));

        let type_item = SimpleExtensionsTypesItem {
            name: "MyType".to_string(),
            description: None,
            parameters: None,
            structure: None,
            variadic: None,
        };

        // Parse the type - this should add it to context
        let _custom_type = type_item.parse(&mut ctx).unwrap();

        // Now the context should have the type
        assert!(ctx.has_type("MyType"));

        let retrieved_type = ctx.get_type("MyType");
        assert!(retrieved_type.is_some());
        assert_eq!(retrieved_type.unwrap().name, "MyType");
    }

    #[test]
    fn test_type_param_conversion() {
        let original_param = TypeParamDefsItem {
            name: Some("test_param".to_string()),
            description: Some("A test parameter".to_string()),
            type_: TypeParamDefsItemType::Integer,
            min: Some(0.0),
            max: Some(100.0),
            optional: Some(true),
            options: None,
        };

        // Convert to owned TypeParam
        let type_param = TypeParam::try_from(original_param.clone()).unwrap();
        assert_eq!(type_param.name, "test_param");
        assert_eq!(type_param.description, Some("A test parameter".to_string()));

        if let ParamKind::Integer { min, max } = type_param.kind {
            assert_eq!(min, Some(0));
            assert_eq!(max, Some(100));
        } else {
            panic!("Expected Integer parameter kind");
        }

        // Convert back to original type
        let converted_back = TypeParamDefsItem::from(type_param);
        assert_eq!(converted_back.name, original_param.name);
        assert_eq!(converted_back.description, original_param.description);
        assert_eq!(converted_back.type_, original_param.type_);
        assert_eq!(converted_back.min, original_param.min);
        assert_eq!(converted_back.max, original_param.max);
        // Note: optional field is not used in our new structure
    }

    #[test]
    fn test_simple_type_no_structure() {
        // Test a simple opaque type (no structure field)
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri);

        let type_item = SimpleExtensionsTypesItem {
            name: "unknown".to_string(),
            description: Some("An opaque type".to_string()),
            parameters: None,
            structure: None, // Opaque type
            variadic: None,
        };

        let result = type_item.parse(&mut ctx);
        assert!(result.is_ok());

        let custom_type = result.unwrap();
        assert_eq!(custom_type.name, "unknown");
        assert_eq!(custom_type.description, Some("An opaque type".to_string()));
        assert!(custom_type.structure.is_none()); // Should be None for opaque type
        assert!(custom_type.parameters.is_empty());
    }

    #[test]
    fn test_types_with_structure() {
        // Test a type with structure: "BINARY" (alias)
        let uri = Url::parse("https://example.com/test.yaml").unwrap();
        let mut ctx = ExtensionContext::new(uri);

        let type_item = SimpleExtensionsTypesItem {
            name: "coordinate".to_string(),
            description: Some("A coordinate in some form".to_string()),
            parameters: None,
            structure: Some(ExtType::Variant0("fp64".to_string())), // Alias to fp64
            variadic: None,
        };

        let result = type_item.parse(&mut ctx);
        assert!(result.is_ok());

        let custom_type = result.unwrap();
        assert_eq!(custom_type.name, "coordinate");
        assert!(custom_type.structure.is_some());

        let structure = custom_type.structure.unwrap();
        assert!(!structure.nullable); // Structure cannot be nullable
        assert!(matches!(
            structure.base,
            KnownType::Builtin(BuiltinType::Fp64)
        ));

        // Create a map structure like { latitude: "coordinate", longitude: "coordinate" }
        let mut field_map = serde_json::Map::new();
        field_map.insert("latitude".to_string(), json!("coordinate"));
        field_map.insert("longitude".to_string(), json!("coordinate"));

        let type_item = SimpleExtensionsTypesItem {
            name: "point".to_string(),
            description: Some("A 2D point".to_string()),
            parameters: None,
            structure: Some(ExtType::Variant1(field_map)),
            variadic: None,
        };

        let result = type_item.parse(&mut ctx);
        assert!(result.is_ok());

        let custom_type = result.unwrap();
        assert_eq!(custom_type.name, "point");
        assert!(custom_type.structure.is_some());

        let structure = custom_type.structure.unwrap();
        assert!(!structure.nullable); // Structure cannot be nullable

        // Should be NStruct with field names
        if let KnownType::NStruct(field_names) = structure.base {
            assert_eq!(field_names.len(), 2);
            assert!(field_names.contains(&"latitude".to_string()));
            assert!(field_names.contains(&"longitude".to_string()));
        } else {
            panic!("Expected NStruct base type");
        }

        // Should have 2 field types (parameters)
        assert_eq!(structure.parameters.len(), 2);
        for param in &structure.parameters {
            if let KnownType::Extension(ref type_name) = param.base {
                assert_eq!(type_name, "coordinate");
            } else {
                panic!("Expected Extension type for coordinate reference");
            }
        }
    }
}
