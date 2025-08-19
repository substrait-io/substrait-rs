// SPDX-License-Identifier: Apache-2.0

//! Concrete type system for function validation in the registry.
//!
//! This module provides a clean, type-safe wrapper around Substrait extension types,
//! separating function signature patterns from concrete argument types.

use super::extensions::TypeContext;
use crate::parse::Parse;
use crate::text::simple_extensions::{
    EnumOptions, SimpleExtensionsTypesItem, Type as RawType, TypeParamDefs, TypeParamDefsItem,
    TypeParamDefsItemType,
};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// Write a sequence of items separated by a separator, with a start and end
/// delimiter.
///
/// Start and end are only included in the output if there is at least one item.
fn write_separated<I, T>(
    f: &mut fmt::Formatter<'_>,
    iter: I,
    start: &str,
    end: &str,
    sep: &str,
) -> fmt::Result
where
    I: IntoIterator<Item = T>,
    T: fmt::Display,
{
    let mut it = iter.into_iter();
    if let Some(first) = it.next() {
        f.write_str(start)?;
        write!(f, "{first}")?;
        for item in it {
            f.write_str(sep)?;
            write!(f, "{item}")?;
        }
        f.write_str(end)
    } else {
        Ok(())
    }
}

/// A pair of a key and a value, separated by a separator. For display purposes.
struct KeyValueDisplay<K, V>(K, V, &'static str);

impl<K, V> fmt::Display for KeyValueDisplay<K, V>
where
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.0, self.2, self.1)
    }
}

/// Substrait built-in primitive types (no parameters required)
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
    /// Time of day - `time` (deprecated, use CompoundType::PrecisionTime)
    Time,
    /// Date and time - `timestamp` (deprecated, use CompoundType::PrecisionTimestamp)
    Timestamp,
    /// Date and time with timezone - `timestamp_tz` (deprecated, use CompoundType::PrecisionTimestampTz)
    TimestampTz,
    /// Year-month interval - `interval_year`
    IntervalYear,
    /// 128-bit UUID - `uuid`
    Uuid,
}

impl fmt::Display for BuiltinType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BuiltinType::Boolean => "bool",
            BuiltinType::I8 => "i8",
            BuiltinType::I16 => "i16",
            BuiltinType::I32 => "i32",
            BuiltinType::I64 => "i64",
            BuiltinType::Fp32 => "fp32",
            BuiltinType::Fp64 => "fp64",
            BuiltinType::String => "string",
            BuiltinType::Binary => "binary",
            BuiltinType::Date => "date",
            BuiltinType::Time => "time",
            BuiltinType::Timestamp => "timestamp",
            BuiltinType::TimestampTz => "timestamp_tz",
            BuiltinType::IntervalYear => "interval_year",
            BuiltinType::Uuid => "uuid",
        };
        f.write_str(s)
    }
}

/// Parameter for parameterized types
#[derive(Clone, Debug, PartialEq)]
pub enum TypeParameter {
    /// Integer parameter (e.g., precision, scale)
    Integer(i64),
    /// Type parameter (nested type)
    Type(ConcreteType),
    /// String parameter
    String(String),
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeParameter::Integer(i) => write!(f, "{i}"),
            TypeParameter::Type(t) => write!(f, "{t}"),
            TypeParameter::String(s) => write!(f, "{s}"),
        }
    }
}

/// Parameterized builtin types that require non-type parameters
#[derive(Clone, Debug, PartialEq)]
pub enum CompoundType {
    /// Fixed-length character string FIXEDCHAR<L>
    FixedChar {
        /// Length (number of characters), must be >= 1
        length: i32,
    },
    /// Variable-length character string VARCHAR<L>
    VarChar {
        /// Maximum length (number of characters), must be >= 1
        length: i32,
    },
    /// Fixed-length binary data FIXEDBINARY<L>
    FixedBinary {
        /// Length (number of bytes), must be >= 1
        length: i32,
    },
    /// Fixed-point decimal DECIMAL<P, S>
    Decimal {
        /// Precision (total digits), <= 38
        precision: i32,
        /// Scale (digits after decimal point), 0 <= S <= P
        scale: i32,
    },
    /// Time with sub-second precision PRECISIONTIME<P>
    PrecisionTime {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Timestamp with sub-second precision PRECISIONTIMESTAMP<P>
    PrecisionTimestamp {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Timezone-aware timestamp with precision PRECISIONTIMESTAMPTZ<P>
    PrecisionTimestampTz {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Day-time interval INTERVAL_DAY<P>
    IntervalDay {
        /// Sub-second precision digits (0-9: seconds to nanoseconds)
        precision: i32,
    },
    /// Compound interval INTERVAL_COMPOUND<P>
    IntervalCompound {
        /// Sub-second precision digits
        precision: i32,
    },
}

impl fmt::Display for CompoundType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompoundType::FixedChar { length } => write!(f, "FIXEDCHAR<{length}>"),
            CompoundType::VarChar { length } => write!(f, "VARCHAR<{length}>"),
            CompoundType::FixedBinary { length } => write!(f, "FIXEDBINARY<{length}>"),
            CompoundType::Decimal { precision, scale } => {
                write!(f, "DECIMAL<{precision}, {scale}>")
            }
            CompoundType::PrecisionTime { precision } => write!(f, "PRECISIONTIME<{precision}>"),
            CompoundType::PrecisionTimestamp { precision } => {
                write!(f, "PRECISIONTIMESTAMP<{precision}>")
            }
            CompoundType::PrecisionTimestampTz { precision } => {
                write!(f, "PRECISIONTIMESTAMPTZ<{precision}>")
            }
            CompoundType::IntervalDay { precision } => write!(f, "INTERVAL_DAY<{precision}>"),
            CompoundType::IntervalCompound { precision } => {
                write!(f, "INTERVAL_COMPOUND<{precision}>")
            }
        }
    }
}

/// Error when a builtin type name is not recognized
#[derive(Debug, Error)]
#[error("Unrecognized builtin type: {0}")]
pub struct UnrecognizedBuiltin(String);

impl FromStr for BuiltinType {
    type Err = UnrecognizedBuiltin;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bool" => Ok(BuiltinType::Boolean),
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
            "uuid" => Ok(BuiltinType::Uuid),
            _ => Err(UnrecognizedBuiltin(s.to_string())),
        }
    }
}

/// Parameter type information for type definitions
#[derive(Clone, Debug, PartialEq)]
pub enum ParameterType {
    /// Data type parameter
    DataType,
    /// Integer parameter with range constraints
    Integer {
        /// Minimum value (inclusive), if specified
        min: Option<i64>,
        /// Maximum value (inclusive), if specified
        max: Option<i64>,
    },
    /// Enumeration parameter
    Enumeration {
        /// Valid enumeration values
        options: Vec<String>,
    },
    /// Boolean parameter
    Boolean,
    /// String parameter
    String,
}

impl ParameterType {
    /// Convert back to raw TypeParamDefsItemType
    fn raw_type(&self) -> TypeParamDefsItemType {
        match self {
            ParameterType::DataType => TypeParamDefsItemType::DataType,
            ParameterType::Boolean => TypeParamDefsItemType::Boolean,
            ParameterType::Integer { .. } => TypeParamDefsItemType::Integer,
            ParameterType::Enumeration { .. } => TypeParamDefsItemType::Enumeration,
            ParameterType::String => TypeParamDefsItemType::String,
        }
    }

    /// Extract raw bounds for integer parameters (min, max)
    fn raw_bounds(&self) -> (Option<f64>, Option<f64>) {
        match self {
            ParameterType::Integer { min, max } => (min.map(|i| i as f64), max.map(|i| i as f64)),
            _ => (None, None),
        }
    }

    /// Extract raw enum options for enumeration parameters
    fn raw_options(&self) -> Option<EnumOptions> {
        match self {
            ParameterType::Enumeration { options } => Some(EnumOptions(options.clone())),
            _ => None,
        }
    }

    /// Check if a parameter value is valid for this parameter type
    pub fn is_valid_value(&self, value: &Value) -> bool {
        match (self, value) {
            (ParameterType::DataType, Value::String(_)) => true,
            (ParameterType::Integer { min, max }, Value::Number(n)) => {
                if let Some(i) = n.as_i64() {
                    min.is_none_or(|min_val| i >= min_val) && max.is_none_or(|max_val| i <= max_val)
                } else {
                    false
                }
            }
            (ParameterType::Enumeration { options }, Value::String(s)) => options.contains(s),
            (ParameterType::Boolean, Value::Bool(_)) => true,
            (ParameterType::String, Value::String(_)) => true,
            _ => false,
        }
    }

    fn from_raw(
        t: TypeParamDefsItemType,
        opts: Option<EnumOptions>,
        min: Option<f64>,
        max: Option<f64>,
    ) -> Result<Self, TypeParamError> {
        Ok(match t {
            TypeParamDefsItemType::DataType => Self::DataType,
            TypeParamDefsItemType::Boolean => Self::Boolean,
            TypeParamDefsItemType::Integer => {
                // TODO: This truncates from float to int; probably fine
                let min_i = min.map(|n| n as i64);
                let max_i = max.map(|n| n as i64);
                Self::Integer {
                    min: min_i,
                    max: max_i,
                }
            }
            TypeParamDefsItemType::Enumeration => {
                let options = opts.ok_or(TypeParamError::MissingEnumOptions)?.0; // Extract Vec<String> from EnumOptions
                Self::Enumeration { options }
            }
            TypeParamDefsItemType::String => Self::String,
        })
    }
}

/// A validated type parameter with name and constraints
#[derive(Clone, Debug, PartialEq)]
pub struct TypeParam {
    /// Parameter name (e.g., "K" for a type variable)
    pub name: String,
    /// Parameter type constraints
    pub param_type: ParameterType,
    /// Human-readable description
    pub description: Option<String>,
}

impl TypeParam {
    /// Create a new type parameter
    pub fn new(name: String, param_type: ParameterType, description: Option<String>) -> Self {
        Self {
            name,
            param_type,
            description,
        }
    }

    /// Check if a parameter value is valid
    pub fn is_valid_value(&self, value: &Value) -> bool {
        self.param_type.is_valid_value(value)
    }
}

impl TryFrom<TypeParamDefsItem> for TypeParam {
    type Error = TypeParamError;

    fn try_from(item: TypeParamDefsItem) -> Result<Self, Self::Error> {
        let name = item.name.ok_or(TypeParamError::MissingName)?;
        let param_type = ParameterType::from_raw(item.type_, item.options, item.min, item.max)?;

        Ok(Self {
            name,
            param_type,
            description: item.description,
        })
    }
}

/// Error types for extension type validation
#[derive(Debug, Error, PartialEq)]
pub enum ExtensionTypeError {
    /// Extension type name is invalid
    #[error("{0}")]
    InvalidName(#[from] InvalidTypeName),
    /// Any type variable is invalid for concrete types
    #[error("Any type variable is invalid for concrete types: any{}{}", id, nullability.then_some("?").unwrap_or(""))]
    InvalidAnyTypeVariable {
        /// The type variable name
        id: u32,
        /// Whether the type variable is nullable
        nullability: bool,
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
    /// Enumeration parameter is missing options
    #[error("Enumeration parameter is missing options")]
    MissingEnumOptions,
}

/// A validated custom extension type definition
#[derive(Clone, Debug, PartialEq)]
pub struct CustomType {
    /// Type name
    pub name: String,
    /// Type parameters (e.g., for generic types)
    pub parameters: Vec<TypeParam>,
    /// Concrete structure definition, if any
    pub structure: Option<ConcreteType>,
    /// Whether this type can have variadic parameters
    pub variadic: Option<bool>,
    /// Human-readable description
    pub description: Option<String>,
}

impl CustomType {
    /// Check if this type name is valid according to Substrait naming rules
    pub fn validate_name(name: &str) -> Result<(), InvalidTypeName> {
        if name.is_empty() {
            return Err(InvalidTypeName(name.to_string()));
        }

        // Basic validation - could be extended with more rules
        if name.contains(|c: char| c.is_whitespace()) {
            return Err(InvalidTypeName(name.to_string()));
        }

        Ok(())
    }

    /// Create a new custom type with validation
    pub fn new(
        name: String,
        parameters: Vec<TypeParam>,
        structure: Option<ConcreteType>,
        variadic: Option<bool>,
        description: Option<String>,
    ) -> Result<Self, ExtensionTypeError> {
        Self::validate_name(&name)?;

        Ok(Self {
            name,
            parameters,
            structure,
            variadic,
            description,
        })
    }
}

impl From<CustomType> for SimpleExtensionsTypesItem {
    fn from(value: CustomType) -> Self {
        // Convert parameters back to TypeParamDefs if any
        let parameters = if value.parameters.is_empty() {
            None
        } else {
            Some(TypeParamDefs(
                value
                    .parameters
                    .into_iter()
                    .map(|param| {
                        let (min, max) = param.param_type.raw_bounds();
                        TypeParamDefsItem {
                            name: Some(param.name),
                            description: param.description,
                            type_: param.param_type.raw_type(),
                            min,
                            max,
                            options: param.param_type.raw_options(),
                            optional: None,
                        }
                    })
                    .collect(),
            ))
        };

        // Convert structure back to Type if any
        let structure = value.structure.map(Into::into);

        SimpleExtensionsTypesItem {
            name: value.name,
            description: value.description,
            parameters,
            structure,
            variadic: value.variadic,
        }
    }
}

impl Parse<TypeContext> for SimpleExtensionsTypesItem {
    type Parsed = CustomType;
    type Error = ExtensionTypeError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        let name = self.name;
        CustomType::validate_name(&name)?;

        // Register this type as found
        ctx.found(&name);

        let parameters = if let Some(param_defs) = self.parameters {
            param_defs
                .0
                .into_iter()
                .map(TypeParam::try_from)
                .collect::<Result<Vec<_>, _>>()?
        } else {
            Vec::new()
        };

        // Parse structure with context, so referenced extension types are recorded as linked
        let structure = match self.structure {
            Some(structure_data) => Some(Parse::parse(structure_data, ctx)?),
            None => None,
        };

        Ok(CustomType {
            name,
            parameters,
            structure,
            variadic: self.variadic,
            description: self.description,
        })
    }
}

impl Parse<TypeContext> for RawType {
    type Parsed = ConcreteType;
    type Error = ExtensionTypeError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        // Walk a ParsedType and record extension type references in the context
        fn visit_parsed_type_references<'a, F>(p: &'a ParsedType<'a>, on_ext: &mut F)
        where
            F: FnMut(&'a str),
        {
            match p {
                ParsedType::Extension(name, _) | ParsedType::NamedExtension(name, _) => {
                    on_ext(name)
                }
                ParsedType::List(t, _) => visit_parsed_type_references(t, on_ext),
                ParsedType::Map(k, v, _) => {
                    visit_parsed_type_references(k, on_ext);
                    visit_parsed_type_references(v, on_ext);
                }
                ParsedType::Struct(ts, _) => {
                    for t in ts {
                        visit_parsed_type_references(t, on_ext)
                    }
                }
                ParsedType::Builtin(..) | ParsedType::TypeVariable(..) => {}
            }
        }

        match self {
            RawType::Variant0(type_string) => {
                let parsed_type = ParsedType::parse(&type_string);
                let mut link = |name: &str| ctx.linked(name);
                visit_parsed_type_references(&parsed_type, &mut link);
                let concrete = ConcreteType::try_from(parsed_type)?;

                // Structure representation cannot be nullable
                if concrete.nullable {
                    return Err(ExtensionTypeError::StructureCannotBeNullable { type_string });
                }

                Ok(concrete)
            }
            RawType::Variant1(field_map) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();

                for (field_name, field_type_value) in field_map {
                    field_names.push(field_name);

                    let type_string = match field_type_value {
                        serde_json::Value::String(s) => s,
                        _ => {
                            return Err(ExtensionTypeError::InvalidFieldType(
                                "Struct field types must be strings".to_string(),
                            ))
                        }
                    };

                    let parsed_field_type = ParsedType::parse(&type_string);
                    let mut link = |name: &str| ctx.linked(name);
                    visit_parsed_type_references(&parsed_field_type, &mut link);
                    let field_concrete_type = ConcreteType::try_from(parsed_field_type)?;

                    field_types.push(field_concrete_type);
                }

                Ok(ConcreteType {
                    known_type: KnownType::NamedStruct {
                        field_names,
                        field_types,
                    },
                    nullable: false,
                })
            }
        }
    }
}

/// Invalid type name error
#[derive(Debug, Error, PartialEq)]
#[error("{0}")]
pub struct InvalidTypeName(String);

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
// impl Parse<ExtensionContext> for &RawType {
//     type Parsed = ValidatedType;
//     type Error = TypeParseError;
//     fn parse(self, ctx: &mut ExtensionContext) -> Result<Self::Parsed, Self::Error> {
//         todo!("Update when ValidatedType and ParsedType are owned")
//     }
// }

/// Error for invalid function call specifications
#[derive(Debug, thiserror::Error)]
pub enum FunctionCallError {
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
// impl Parse<ExtensionContext> for &simple_extensions::ArgumentsItem {
//     type Parsed = ArgumentPattern;
//     type Error = FunctionCallError;
//     fn parse(self, ctx: &mut ExtensionContext) -> Result<Self::Parsed, Self::Error> {
//         todo!("Update when ArgumentPattern is owned")
//     }
// }

/// Known Substrait types (builtin + extension references)
#[derive(Clone, Debug, PartialEq)]
pub enum KnownType {
    /// Simple built-in Substrait primitive type (no parameters)
    Builtin(BuiltinType),
    /// Parameterized built-in types
    Compound(CompoundType),
    /// Extension type with optional parameters
    Extension {
        /// Extension type name
        name: String,
        /// Type parameters
        parameters: Vec<TypeParameter>,
    },
    /// List type with element type
    List(Box<ConcreteType>),
    /// Map type with key and value types
    Map {
        /// Key type
        key: Box<ConcreteType>,
        /// Value type
        value: Box<ConcreteType>,
    },
    /// Struct type (ordered fields without names)
    Struct(Vec<ConcreteType>),
    /// Named struct type (nstruct - ordered fields with names)
    NamedStruct {
        /// Field names
        field_names: Vec<String>,
        /// Field types (same order as field_names)
        field_types: Vec<ConcreteType>,
    },
}

impl fmt::Display for KnownType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KnownType::Builtin(b) => write!(f, "{b}"),
            KnownType::Compound(c) => write!(f, "{c}"),
            KnownType::Extension { name, parameters } => {
                write!(f, "{name}")?;
                write_separated(f, parameters.iter(), "<", ">", ", ")
            }
            KnownType::List(elem) => write!(f, "List<{elem}>"),
            KnownType::Map { key, value } => write!(f, "Map<{key}, {value}>"),
            KnownType::Struct(types) => write_separated(f, types.iter(), "Struct<", ">", ", "),
            KnownType::NamedStruct {
                field_names,
                field_types,
            } => {
                let kvs = field_names
                    .iter()
                    .zip(field_types.iter())
                    .map(|(k, v)| KeyValueDisplay(k, v, ": "));

                write_separated(f, kvs, "{", "}", ", ")
            }
        }
    }
}

/// A concrete, fully-resolved type instance
#[derive(Clone, Debug, PartialEq)]
pub struct ConcreteType {
    /// The known type information
    pub known_type: KnownType,
    /// Whether this type is nullable
    pub nullable: bool,
}

impl ConcreteType {
    /// Create a new builtin type
    pub fn builtin(builtin_type: BuiltinType, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Builtin(builtin_type),
            nullable,
        }
    }

    /// Create a new compound (parameterized) type
    pub fn compound(compound_type: CompoundType, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Compound(compound_type),
            nullable,
        }
    }

    /// Create a new extension type reference (without parameters)
    pub fn extension(name: String, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Extension {
                name,
                parameters: Vec::new(),
            },
            nullable,
        }
    }

    /// Create a new parameterized extension type
    pub fn extension_with_params(
        name: String,
        parameters: Vec<TypeParameter>,
        nullable: bool,
    ) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Extension { name, parameters },
            nullable,
        }
    }

    /// Create a new list type
    pub fn list(element_type: ConcreteType, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::List(Box::new(element_type)),
            nullable,
        }
    }

    /// Create a new struct type (ordered fields without names)
    pub fn r#struct(field_types: Vec<ConcreteType>, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Struct(field_types),
            nullable,
        }
    }

    /// Create a new map type
    pub fn map(key_type: ConcreteType, value_type: ConcreteType, nullable: bool) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::Map {
                key: Box::new(key_type),
                value: Box::new(value_type),
            },
            nullable,
        }
    }

    /// Create a new named struct type (nstruct - ordered fields with names)
    pub fn named_struct(
        field_names: Vec<String>,
        field_types: Vec<ConcreteType>,
        nullable: bool,
    ) -> ConcreteType {
        ConcreteType {
            known_type: KnownType::NamedStruct {
                field_names,
                field_types,
            },
            nullable,
        }
    }

    /// Check if this type matches another type exactly
    pub fn matches(&self, other: &ConcreteType) -> bool {
        self == other
    }

    /// Check if this type is compatible with another type (considering nullability)
    pub fn is_compatible_with(&self, other: &ConcreteType) -> bool {
        // Types must match exactly, but nullable types can accept non-nullable values
        self.known_type == other.known_type && (self.nullable || !other.nullable)
    }
}

impl fmt::Display for ConcreteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.known_type)?;
        if self.nullable {
            write!(f, "?")?;
        }
        Ok(())
    }
}

impl From<ConcreteType> for RawType {
    fn from(val: ConcreteType) -> Self {
        match val.known_type {
            KnownType::NamedStruct {
                field_names,
                field_types,
            } => {
                let mut map = serde_json::Map::new();
                for (name, ty) in field_names.into_iter().zip(field_types.into_iter()) {
                    if let Some(v) = map.insert(name, serde_json::Value::String(ty.to_string())) {
                        // This should not happen - you should not have
                        // duplicate field names in a NamedStruct
                        panic!("duplicate value '{v:?}' in NamedStruct");
                    }
                }
                RawType::Variant1(map)
            }
            _ => RawType::Variant0(val.to_string()),
        }
    }
}

impl<'a> TryFrom<ParsedType<'a>> for ConcreteType {
    type Error = ExtensionTypeError;

    fn try_from(parsed_type: ParsedType<'a>) -> Result<Self, Self::Error> {
        match parsed_type {
            ParsedType::Builtin(builtin_type, nullability) => {
                Ok(ConcreteType::builtin(builtin_type, nullability))
            }
            ParsedType::Extension(ext_name, nullability) => {
                Ok(ConcreteType::extension(ext_name.to_string(), nullability))
            }
            ParsedType::List(element_type, nullability) => {
                let element_concrete = ConcreteType::try_from(*element_type)?;
                Ok(ConcreteType::list(element_concrete, nullability))
            }
            ParsedType::Map(key_type, value_type, nullability) => {
                let key_concrete = ConcreteType::try_from(*key_type)?;
                let value_concrete = ConcreteType::try_from(*value_type)?;
                Ok(ConcreteType::map(key_concrete, value_concrete, nullability))
            }
            ParsedType::Struct(field_types, nullability) => {
                let concrete_field_types: Result<Vec<ConcreteType>, _> = field_types
                    .into_iter()
                    .map(ConcreteType::try_from)
                    .collect();
                Ok(ConcreteType::r#struct(concrete_field_types?, nullability))
            }
            ParsedType::TypeVariable(id, nullability) => {
                Err(ExtensionTypeError::InvalidAnyTypeVariable { id, nullability })
            }
            ParsedType::NamedExtension(type_str, nullability) => {
                Ok(ConcreteType::extension(type_str.to_string(), nullability))
            }
        }
    }
}

/// A parsed type from a type string, with lifetime tied to the original string
#[derive(Clone, Debug, PartialEq)]
pub enum ParsedType<'a> {
    /// Built-in type
    Builtin(BuiltinType, bool),
    /// Extension type reference
    Extension(&'a str, bool),
    /// List type
    List(Box<ParsedType<'a>>, bool),
    /// Map type
    Map(Box<ParsedType<'a>>, Box<ParsedType<'a>>, bool),
    /// Struct type
    Struct(Vec<ParsedType<'a>>, bool),
    /// Type variable (e.g., any1, any2)
    TypeVariable(u32, bool),
    /// Named extension type (unresolved)
    NamedExtension(&'a str, bool),
}

impl<'a> ParsedType<'a> {
    /// Parse a type string into a ParsedType
    pub fn parse(type_str: &'a str) -> Self {
        // Simple parsing implementation - could be more sophisticated
        let (base_type, nullable) = match type_str.strip_suffix('?') {
            Some(base) => (base, true),
            None => (type_str, false),
        };

        // Handle type variables like any1, any2, etc.
        if let Some(suffix) = base_type.strip_prefix("any") {
            if let Ok(id) = suffix.parse::<u32>() {
                if id >= 1 {
                    return ParsedType::TypeVariable(id, nullable);
                }
            }
        }

        // Try to parse as builtin type
        if let Ok(builtin_type) = BuiltinType::from_str(base_type) {
            return ParsedType::Builtin(builtin_type, nullable);
        }

        // Otherwise, treat as extension type
        ParsedType::NamedExtension(base_type, nullable)
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
        let mut vars = HashMap::new();

        if patterns.len() != args.len() {
            return None;
        }

        for (pattern, arg) in patterns.iter().zip(args.iter()) {
            match pattern.matches(arg) {
                Match::Concrete => {} // Pattern matched, nothing to bind
                Match::Variable(id, concrete_type) => {
                    // Check if this type variable is already bound to a different type
                    if let Some(existing_type) = vars.get(&id) {
                        if existing_type != &concrete_type {
                            return None; // Conflict: same variable bound to different types
                        }
                    } else {
                        vars.insert(id, concrete_type);
                    }
                }
                Match::Fail => return None, // Pattern did not match
            }
        }

        Some(TypeBindings { vars })
    }

    /// Get the concrete type bound to a type variable
    pub fn get_binding(&self, var_id: u32) -> Option<&ConcreteType> {
        self.vars.get(&var_id)
    }

    /// Check if all type variables are bound
    pub fn is_complete(&self, expected_vars: &[u32]) -> bool {
        expected_vars.iter().all(|var| self.vars.contains_key(var))
    }
}

#[cfg(test)]
mod tests {
    use super::super::extensions::TypeContext;
    use super::*;
    use crate::text::simple_extensions;

    #[test]
    fn test_builtin_type_parsing() {
        assert_eq!(BuiltinType::from_str("i32").unwrap(), BuiltinType::I32);
        assert_eq!(
            BuiltinType::from_str("string").unwrap(),
            BuiltinType::String
        );
        assert!(BuiltinType::from_str("invalid").is_err());
    }

    #[test]
    fn test_parsed_type_simple() {
        let parsed = ParsedType::parse("i32");
        assert_eq!(parsed, ParsedType::Builtin(BuiltinType::I32, false));

        let parsed_nullable = ParsedType::parse("i32?");
        assert_eq!(parsed_nullable, ParsedType::Builtin(BuiltinType::I32, true));
    }

    #[test]
    fn test_parsed_type_variables() {
        let parsed = ParsedType::parse("any1");
        assert_eq!(parsed, ParsedType::TypeVariable(1, false));

        let parsed_nullable = ParsedType::parse("any2?");
        assert_eq!(parsed_nullable, ParsedType::TypeVariable(2, true));

        // Invalid type variable ID (must be >= 1)
        let parsed_invalid = ParsedType::parse("any0");
        assert_eq!(parsed_invalid, ParsedType::NamedExtension("any0", false));
    }

    #[test]
    fn test_concrete_type_creation() {
        let int_type = ConcreteType::builtin(BuiltinType::I32, false);
        assert_eq!(
            int_type,
            ConcreteType {
                known_type: KnownType::Builtin(BuiltinType::I32),
                nullable: false
            }
        );

        let list_type = ConcreteType::list(int_type.clone(), true);
        assert_eq!(
            list_type,
            ConcreteType {
                known_type: KnownType::List(Box::new(int_type)),
                nullable: true
            }
        );
    }

    #[test]
    fn test_argument_pattern_matching() {
        let concrete_int = ConcreteType::builtin(BuiltinType::I32, false);
        let concrete_string = ConcreteType::builtin(BuiltinType::String, false);

        // Test concrete pattern matching
        let concrete_pattern = ArgumentPattern::Concrete(concrete_int.clone());
        assert_eq!(concrete_pattern.matches(&concrete_int), Match::Concrete);
        assert_eq!(concrete_pattern.matches(&concrete_string), Match::Fail);

        // Test type variable pattern
        let var_pattern = ArgumentPattern::TypeVariable(1);
        assert_eq!(
            var_pattern.matches(&concrete_int),
            Match::Variable(1, concrete_int.clone())
        );
    }

    #[test]
    fn test_type_bindings() {
        let patterns = vec![
            ArgumentPattern::TypeVariable(1),
            ArgumentPattern::TypeVariable(1), // Same variable should bind to same type
        ];
        let args = vec![
            ConcreteType::builtin(BuiltinType::I32, false),
            ConcreteType::builtin(BuiltinType::I32, false),
        ];

        let bindings = TypeBindings::new(&patterns, &args).unwrap();
        assert_eq!(
            bindings.get_binding(1),
            Some(&ConcreteType::builtin(BuiltinType::I32, false))
        );

        // Test conflicting bindings
        let conflicting_args = vec![
            ConcreteType::builtin(BuiltinType::I32, false),
            ConcreteType::builtin(BuiltinType::String, false),
        ];
        assert!(TypeBindings::new(&patterns, &conflicting_args).is_none());
    }

    #[test]
    fn test_parameter_type_validation() {
        let int_param = ParameterType::Integer {
            min: Some(1),
            max: Some(10),
        };

        assert!(int_param.is_valid_value(&Value::Number(5.into())));
        assert!(!int_param.is_valid_value(&Value::Number(0.into())));
        assert!(!int_param.is_valid_value(&Value::Number(11.into())));
        assert!(!int_param.is_valid_value(&Value::String("not a number".into())));

        let enum_param = ParameterType::Enumeration {
            options: vec!["OVERFLOW".to_string(), "ERROR".to_string()],
        };

        assert!(enum_param.is_valid_value(&Value::String("OVERFLOW".into())));
        assert!(!enum_param.is_valid_value(&Value::String("INVALID".into())));
    }

    #[test]
    fn test_integer_param_bounds_round_trip() {
        // Valid bounds now use lossy cast from f64 to i64; fractional parts are truncated toward zero
        let item = simple_extensions::TypeParamDefsItem {
            name: Some("K".to_string()),
            description: None,
            type_: simple_extensions::TypeParamDefsItemType::Integer,
            min: Some(1.0),
            max: Some(10.0),
            options: None,
            optional: None,
        };
        let tp = TypeParam::try_from(item).expect("should parse integer bounds");
        match tp.param_type {
            ParameterType::Integer { min, max } => {
                assert_eq!(min, Some(1));
                assert_eq!(max, Some(10));
            }
            _ => panic!("expected integer param type"),
        }

        // Fractional min is truncated
        let trunc = simple_extensions::TypeParamDefsItem {
            name: Some("K".to_string()),
            description: None,
            type_: simple_extensions::TypeParamDefsItemType::Integer,
            min: Some(1.5),
            max: None,
            options: None,
            optional: None,
        };
        let tp = TypeParam::try_from(trunc).expect("should parse with truncation");
        match tp.param_type {
            ParameterType::Integer { min, max } => {
                assert_eq!(min, Some(1));
                assert_eq!(max, None);
            }
            _ => panic!("expected integer param type"),
        }
    }

    #[test]
    fn test_custom_type_round_trip_alias() -> Result<(), ExtensionTypeError> {
        let custom = CustomType::new(
            "AliasType".to_string(),
            vec![],
            Some(ConcreteType::builtin(BuiltinType::I32, false)),
            None,
            Some("desc".to_string()),
        )?;
        let item: simple_extensions::SimpleExtensionsTypesItem = custom.clone().into();
        let mut ctx = TypeContext::default();
        let parsed = Parse::parse(item, &mut ctx)?;
        assert_eq!(parsed.name, custom.name);
        assert_eq!(parsed.description, custom.description);
        assert_eq!(parsed.structure, custom.structure);
        Ok(())
    }

    #[test]
    fn test_custom_type_round_trip_named_struct() -> Result<(), ExtensionTypeError> {
        let fields = vec![
            (
                "x".to_string(),
                ConcreteType::builtin(BuiltinType::Fp64, false),
            ),
            (
                "y".to_string(),
                ConcreteType::builtin(BuiltinType::Fp64, false),
            ),
        ];
        let (names, types): (Vec<_>, Vec<_>) = fields.into_iter().unzip();
        let custom = CustomType::new(
            "Point".to_string(),
            vec![],
            Some(ConcreteType::named_struct(
                names.clone(),
                types.clone(),
                false,
            )),
            None,
            None,
        )?;
        let item: simple_extensions::SimpleExtensionsTypesItem = custom.clone().into();
        let mut ctx = TypeContext::default();
        let parsed = Parse::parse(item, &mut ctx)?;
        assert_eq!(parsed.name, custom.name);
        if let Some(ConcreteType {
            known_type:
                KnownType::NamedStruct {
                    field_names,
                    field_types,
                },
            nullable,
        }) = parsed.structure
        {
            assert!(!nullable);
            assert_eq!(field_names, names);
            assert_eq!(field_types, types);
        } else {
            panic!("expected named struct after round-trip");
        }
        Ok(())
    }

    #[test]
    fn test_custom_type_creation() -> Result<(), ExtensionTypeError> {
        let custom_type = CustomType::new(
            "MyType".to_string(),
            vec![],
            Some(ConcreteType::builtin(BuiltinType::I32, false)),
            None,
            Some("A custom type".to_string()),
        )?;

        assert_eq!(custom_type.name, "MyType");
        assert_eq!(custom_type.parameters.len(), 0);
        assert!(custom_type.structure.is_some());
        Ok(())
    }

    #[test]
    fn test_invalid_type_names() {
        // Empty name should be invalid
        assert!(CustomType::validate_name("").is_err());
        // Name with whitespace should be invalid
        assert!(CustomType::validate_name("bad name").is_err());
        // Valid name should pass
        assert!(CustomType::validate_name("GoodName").is_ok());
    }

    #[test]
    fn test_ext_type_to_concrete_type() -> Result<(), ExtensionTypeError> {
        // Test simple type string alias
        let ext_type = RawType::Variant0("i32".to_string());
        let mut ctx = TypeContext::default();
        let concrete = Parse::parse(ext_type, &mut ctx)?;
        assert_eq!(concrete, ConcreteType::builtin(BuiltinType::I32, false));

        // Test struct type
        let mut field_map = serde_json::Map::new();
        field_map.insert(
            "field1".to_string(),
            serde_json::Value::String("fp64".to_string()),
        );
        let ext_type = RawType::Variant1(field_map);
        let mut ctx = TypeContext::default();
        let concrete = Parse::parse(ext_type, &mut ctx)?;

        if let KnownType::NamedStruct {
            field_names,
            field_types,
        } = &concrete.known_type
        {
            assert_eq!(field_names, &vec!["field1".to_string()]);
            assert_eq!(field_types.len(), 1);
            assert_eq!(
                field_types[0],
                ConcreteType::builtin(BuiltinType::Fp64, false)
            );
        } else {
            panic!("Expected named struct type");
        }

        Ok(())
    }

    #[test]
    fn test_custom_type_parsing() -> Result<(), ExtensionTypeError> {
        let type_item = simple_extensions::SimpleExtensionsTypesItem {
            name: "TestType".to_string(),
            description: Some("A test type".to_string()),
            parameters: None,
            structure: Some(RawType::Variant0("BINARY".to_string())), // Alias to fp64
            variadic: None,
        };

        let mut ctx = TypeContext::default();
        let custom_type = Parse::parse(type_item, &mut ctx)?;
        assert_eq!(custom_type.name, "TestType");
        assert_eq!(custom_type.description, Some("A test type".to_string()));
        assert!(custom_type.structure.is_some());

        if let Some(structure) = &custom_type.structure {
            assert_eq!(
                structure.known_type,
                KnownType::Builtin(BuiltinType::Binary)
            );
        }

        Ok(())
    }

    #[test]
    fn test_custom_type_with_struct() -> Result<(), ExtensionTypeError> {
        let mut field_map = serde_json::Map::new();
        field_map.insert(
            "x".to_string(),
            serde_json::Value::String("fp64".to_string()),
        );
        field_map.insert(
            "y".to_string(),
            serde_json::Value::String("fp64".to_string()),
        );

        let type_item = simple_extensions::SimpleExtensionsTypesItem {
            name: "Point".to_string(),
            description: Some("A 2D point".to_string()),
            parameters: None,
            structure: Some(RawType::Variant1(field_map)),
            variadic: None,
        };

        let mut ctx = TypeContext::default();
        let custom_type = Parse::parse(type_item, &mut ctx)?;
        assert_eq!(custom_type.name, "Point");

        if let Some(ConcreteType {
            known_type:
                KnownType::NamedStruct {
                    field_names,
                    field_types,
                },
            ..
        }) = &custom_type.structure
        {
            assert!(field_names.contains(&"x".to_string()));
            assert!(field_names.contains(&"y".to_string()));
            assert_eq!(field_types.len(), 2);
            // Note: HashMap iteration order is not guaranteed, so we just check the types exist
            assert!(field_types
                .iter()
                .all(|t| matches!(t.known_type, KnownType::Builtin(BuiltinType::Fp64))));
        } else {
            panic!("Expected struct type");
        }

        Ok(())
    }

    #[test]
    fn test_nullable_structure_rejected() {
        let ext_type = RawType::Variant0("i32?".to_string());
        let mut ctx = TypeContext::default();
        let result = Parse::parse(ext_type, &mut ctx);
        if let Err(ExtensionTypeError::StructureCannotBeNullable { type_string }) = result {
            assert!(type_string.contains("i32?"));
        } else {
            panic!("Expected nullable structure to be rejected, got: {result:?}");
        }
    }
}
