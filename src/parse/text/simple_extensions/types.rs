// SPDX-License-Identifier: Apache-2.0

//! Concrete type system for function validation in the registry.
//!
//! This module provides a clean, type-safe wrapper around Substrait extension types,
//! separating function signature patterns from concrete argument types.

use super::TypeExpr;
use super::argument::{
    EnumOptions as ParsedEnumOptions, EnumOptionsError as ParsedEnumOptionsError,
};
use super::extensions::TypeContext;
use super::parsed_type::TypeExprParam;
use crate::parse::Parse;
use crate::parse::text::simple_extensions::parsed_type::TypeParseError;
use crate::text::simple_extensions::{
    EnumOptions as RawEnumOptions, SimpleExtensionsTypesItem, Type as RawType, TypeParamDefs,
    TypeParamDefsItem, TypeParamDefsItemType,
};
use serde_json::Value;
use std::convert::TryFrom;
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

/// Substrait primitive built-in types (no parameters required)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
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
    /// Naive Timestamp
    Timestamp,
    /// Timestamp with time zone - `timestamp_tz`
    TimestampTz,
    /// Calendar date - `date`
    Date,
    /// Time of day - `time`
    Time,
    /// Year-month interval - `interval_year`
    IntervalYear,
    /// 128-bit UUID - `uuid`
    Uuid,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrimitiveType::Boolean => "bool",
            PrimitiveType::I8 => "i8",
            PrimitiveType::I16 => "i16",
            PrimitiveType::I32 => "i32",
            PrimitiveType::I64 => "i64",
            PrimitiveType::Fp32 => "fp32",
            PrimitiveType::Fp64 => "fp64",
            PrimitiveType::String => "string",
            PrimitiveType::Binary => "binary",
            PrimitiveType::Timestamp => "timestamp",
            PrimitiveType::TimestampTz => "timestamp_tz",
            PrimitiveType::Date => "date",
            PrimitiveType::Time => "time",
            PrimitiveType::IntervalYear => "interval_year",
            PrimitiveType::Uuid => "uuid",
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
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeParameter::Integer(i) => write!(f, "{i}"),
            TypeParameter::Type(t) => write!(f, "{t}"),
        }
    }
}

/// Parameterized builtin types that require non-type parameters, e.g. numbers
/// or enum
#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinParameterized {
    /// Fixed-length character string: `FIXEDCHAR<L>`
    FixedChar {
        /// Length (number of characters), must be >= 1
        length: i32,
    },
    /// Variable-length character string: `VARCHAR<L>`
    VarChar {
        /// Maximum length (number of characters), must be >= 1
        length: i32,
    },
    /// Fixed-length binary data: `FIXEDBINARY<L>`
    FixedBinary {
        /// Length (number of bytes), must be >= 1
        length: i32,
    },
    /// Fixed-point decimal: `DECIMAL<P, S>`
    Decimal {
        /// Precision (total digits), <= 38
        precision: i32,
        /// Scale (digits after decimal point), 0 <= S <= P
        scale: i32,
    },
    /// Time with sub-second precision: `PRECISIONTIME<P>`
    PrecisionTime {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Timestamp with sub-second precision: `PRECISIONTIMESTAMP<P>`
    PrecisionTimestamp {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Timezone-aware timestamp with precision: `PRECISIONTIMESTAMPTZ<P>`
    PrecisionTimestampTz {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: i32,
    },
    /// Day-time interval: `INTERVAL_DAY<P>`
    IntervalDay {
        /// Sub-second precision digits (0-9: seconds to nanoseconds)
        precision: i32,
    },
    /// Compound interval: `INTERVAL_COMPOUND<P>`
    IntervalCompound {
        /// Sub-second precision digits
        precision: i32,
    },
}

impl BuiltinParameterized {
    /// Check if a string is a valid name for a parameterized builtin type.
    ///
    /// Only matches lowercase.
    pub fn is_name(s: &str) -> bool {
        matches!(
            s,
            "fixedchar"
                | "varchar"
                | "fixedbinary"
                | "decimal"
                | "precisiontime"
                | "precisiontimestamp"
                | "precisiontimestamptz"
                | "interval_day"
                | "interval_compound"
        )
    }
}

impl fmt::Display for BuiltinParameterized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinParameterized::FixedChar { length } => {
                write!(f, "FIXEDCHAR<{length}>")
            }
            BuiltinParameterized::VarChar { length } => {
                write!(f, "VARCHAR<{length}>")
            }
            BuiltinParameterized::FixedBinary { length } => {
                write!(f, "FIXEDBINARY<{length}>")
            }
            BuiltinParameterized::Decimal { precision, scale } => {
                write!(f, "DECIMAL<{precision}, {scale}>")
            }
            BuiltinParameterized::PrecisionTime { precision } => {
                write!(f, "PRECISIONTIME<{precision}>")
            }
            BuiltinParameterized::PrecisionTimestamp { precision } => {
                write!(f, "PRECISIONTIMESTAMP<{precision}>")
            }
            BuiltinParameterized::PrecisionTimestampTz { precision } => {
                write!(f, "PRECISIONTIMESTAMPTZ<{precision}>")
            }
            BuiltinParameterized::IntervalDay { precision } => {
                write!(f, "INTERVAL_DAY<{precision}>")
            }
            BuiltinParameterized::IntervalCompound { precision } => {
                write!(f, "INTERVAL_COMPOUND<{precision}>")
            }
        }
    }
}

/// Unified representation of simple builtin types (primitive or parameterized).
/// Does not include container types like List, Map, or Struct.
#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinKind {
    /// Primitive builtins like `i32`
    Primitive(PrimitiveType),
    /// Parameterized builtins like `DECIMAL<P, S>`
    Parameterized(BuiltinParameterized),
}

impl fmt::Display for BuiltinKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuiltinKind::Primitive(p) => write!(f, "{p}"),
            BuiltinKind::Parameterized(p) => write!(f, "{p}"),
        }
    }
}

impl From<PrimitiveType> for BuiltinKind {
    fn from(value: PrimitiveType) -> Self {
        BuiltinKind::Primitive(value)
    }
}

impl From<BuiltinParameterized> for BuiltinKind {
    fn from(value: BuiltinParameterized) -> Self {
        BuiltinKind::Parameterized(value)
    }
}

/// Check if a name corresponds to any built-in type (primitive, parameterized,
/// or container)
pub fn is_builtin_type_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    PrimitiveType::from_str(&lower).is_ok()
        || BuiltinParameterized::is_name(&lower)
        || matches!(lower.as_str(), "list" | "map" | "struct")
}

/// Error when a builtin type name is not recognized
#[derive(Debug, Error)]
#[error("Unrecognized builtin type: {0}")]
pub struct UnrecognizedBuiltin(String);

impl FromStr for PrimitiveType {
    type Err = UnrecognizedBuiltin;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bool" => Ok(PrimitiveType::Boolean),
            "i8" => Ok(PrimitiveType::I8),
            "i16" => Ok(PrimitiveType::I16),
            "i32" => Ok(PrimitiveType::I32),
            "i64" => Ok(PrimitiveType::I64),
            "fp32" => Ok(PrimitiveType::Fp32),
            "fp64" => Ok(PrimitiveType::Fp64),
            "string" => Ok(PrimitiveType::String),
            "binary" => Ok(PrimitiveType::Binary),
            "timestamp" => Ok(PrimitiveType::Timestamp),
            "timestamp_tz" => Ok(PrimitiveType::TimestampTz),
            "date" => Ok(PrimitiveType::Date),
            "time" => Ok(PrimitiveType::Time),
            "interval_year" => Ok(PrimitiveType::IntervalYear),
            "uuid" => Ok(PrimitiveType::Uuid),
            _ => Err(UnrecognizedBuiltin(s.to_string())),
        }
    }
}

/// Parameter type information for type definitions
#[derive(Clone, Debug, PartialEq)]
pub enum ParameterConstraint {
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
        /// Valid enumeration values (validated, deduplicated)
        options: ParsedEnumOptions,
    },
    /// Boolean parameter
    Boolean,
    /// String parameter
    String,
}

impl ParameterConstraint {
    /// Convert back to raw TypeParamDefsItemType
    fn raw_type(&self) -> TypeParamDefsItemType {
        match self {
            ParameterConstraint::DataType => TypeParamDefsItemType::DataType,
            ParameterConstraint::Boolean => TypeParamDefsItemType::Boolean,
            ParameterConstraint::Integer { .. } => TypeParamDefsItemType::Integer,
            ParameterConstraint::Enumeration { .. } => TypeParamDefsItemType::Enumeration,
            ParameterConstraint::String => TypeParamDefsItemType::String,
        }
    }

    /// Extract raw bounds for integer parameters (min, max)
    fn raw_bounds(&self) -> (Option<f64>, Option<f64>) {
        match self {
            ParameterConstraint::Integer { min, max } => {
                (min.map(|i| i as f64), max.map(|i| i as f64))
            }
            _ => (None, None),
        }
    }

    /// Extract raw enum options for enumeration parameters
    fn raw_options(&self) -> Option<RawEnumOptions> {
        match self {
            ParameterConstraint::Enumeration { options } => Some(options.clone().into()),
            _ => None,
        }
    }

    /// Check if a parameter value is valid for this parameter type
    pub fn is_valid_value(&self, value: &Value) -> bool {
        match (self, value) {
            (ParameterConstraint::DataType, Value::String(_)) => true,
            (ParameterConstraint::Integer { min, max }, Value::Number(n)) => {
                if let Some(i) = n.as_i64() {
                    min.is_none_or(|min_val| i >= min_val) && max.is_none_or(|max_val| i <= max_val)
                } else {
                    false
                }
            }
            (ParameterConstraint::Enumeration { options }, Value::String(s)) => options.contains(s),
            (ParameterConstraint::Boolean, Value::Bool(_)) => true,
            (ParameterConstraint::String, Value::String(_)) => true,
            _ => false,
        }
    }

    fn from_raw(
        t: TypeParamDefsItemType,
        opts: Option<RawEnumOptions>,
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
                let options: ParsedEnumOptions =
                    opts.ok_or(TypeParamError::MissingEnumOptions)?.try_into()?;
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
    pub param_type: ParameterConstraint,
    /// Human-readable description
    pub description: Option<String>,
}

impl TypeParam {
    /// Create a new type parameter
    pub fn new(name: String, param_type: ParameterConstraint, description: Option<String>) -> Self {
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
        let param_type =
            ParameterConstraint::from_raw(item.type_, item.options, item.min, item.max)?;

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
    /// Type parameter count is invalid for the given type name
    #[error("Type '{type_name}' expects {expected} parameters, got {actual}")]
    InvalidParameterCount {
        /// The type name being validated
        type_name: String,
        /// Human-readable description of the expected parameter count
        expected: &'static str,
        /// The actual number of parameters provided
        actual: usize,
    },
    /// Type parameter is of the wrong kind for the given position
    #[error("Type '{type_name}' parameter {index} must be {expected}")]
    InvalidParameterKind {
        /// The type name being validated
        type_name: String,
        /// Zero-based index of the offending parameter
        index: usize,
        /// Expected parameter kind (e.g., integer, type)
        expected: &'static str,
    },
    /// Provided parameter value does not fit within the expected bounds
    #[error("Type '{type_name}' parameter {index} value {value} is out of range for {expected}")]
    InvalidParameterValue {
        /// The type name being validated
        type_name: String,
        /// Zero-based index of the offending parameter
        index: usize,
        /// Provided parameter value
        value: i64,
        /// Description of the expected range or type
        expected: &'static str,
    },
    /// Structure representation cannot be nullable
    #[error("Structure representation cannot be nullable: {type_string}")]
    StructureCannotBeNullable {
        /// The type string that was nullable
        type_string: String,
    },
    /// Error parsing type
    #[error("Error parsing type: {0}")]
    ParseType(#[from] TypeParseError),
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
    /// Enumeration parameter has invalid options
    #[error("Enumeration parameter has invalid options: {0}")]
    InvalidEnumOptions(#[from] ParsedEnumOptionsError),
}

/// A validated Simple Extension type definition
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
        match self {
            RawType::Variant0(type_string) => {
                let parsed_type = TypeExpr::parse(&type_string)?;
                let mut link = |name: &str| ctx.linked(name);
                parsed_type.visit_references(&mut link);
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
                            ));
                        }
                    };

                    let parsed_field_type = TypeExpr::parse(&type_string)?;
                    let mut link = |name: &str| ctx.linked(name);
                    parsed_field_type.visit_references(&mut link);
                    let field_concrete_type = ConcreteType::try_from(parsed_field_type)?;

                    field_types.push(field_concrete_type);
                }

                Ok(ConcreteType {
                    kind: ConcreteTypeKind::NamedStruct {
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

/// Known Substrait types (builtin + extension references)
#[derive(Clone, Debug, PartialEq)]
pub enum ConcreteTypeKind {
    /// Built-in Substrait type (primitive or parameterized)
    Builtin(BuiltinKind),
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

impl fmt::Display for ConcreteTypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConcreteTypeKind::Builtin(b) => write!(f, "{b}"),
            ConcreteTypeKind::Extension { name, parameters } => {
                write!(f, "{name}")?;
                write_separated(f, parameters.iter(), "<", ">", ", ")
            }
            ConcreteTypeKind::List(elem) => write!(f, "List<{elem}>"),
            ConcreteTypeKind::Map { key, value } => write!(f, "Map<{key}, {value}>"),
            ConcreteTypeKind::Struct(types) => {
                write_separated(f, types.iter(), "Struct<", ">", ", ")
            }
            ConcreteTypeKind::NamedStruct {
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
    /// The resolved type shape
    pub kind: ConcreteTypeKind,
    /// Whether this type is nullable
    pub nullable: bool,
}

impl ConcreteType {
    /// Create a new primitive builtin type
    pub fn builtin(builtin_type: PrimitiveType, nullable: bool) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::Builtin(BuiltinKind::Primitive(builtin_type)),
            nullable,
        }
    }

    /// Create a new parameterized builtin type
    pub fn parameterized_builtin(
        builtin_type: BuiltinParameterized,
        nullable: bool,
    ) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::Builtin(BuiltinKind::Parameterized(builtin_type)),
            nullable,
        }
    }

    /// Create a new extension type reference (without parameters)
    pub fn extension(name: String, nullable: bool) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::Extension {
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
            kind: ConcreteTypeKind::Extension { name, parameters },
            nullable,
        }
    }

    /// Create a new list type
    pub fn list(element_type: ConcreteType, nullable: bool) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::List(Box::new(element_type)),
            nullable,
        }
    }

    /// Create a new struct type (ordered fields without names)
    pub fn r#struct(field_types: Vec<ConcreteType>, nullable: bool) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::Struct(field_types),
            nullable,
        }
    }

    /// Create a new map type
    pub fn map(key_type: ConcreteType, value_type: ConcreteType, nullable: bool) -> ConcreteType {
        ConcreteType {
            kind: ConcreteTypeKind::Map {
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
            kind: ConcreteTypeKind::NamedStruct {
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
        self.kind == other.kind && (self.nullable || !other.nullable)
    }
}

impl fmt::Display for ConcreteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)?;
        if self.nullable {
            write!(f, "?")?;
        }
        Ok(())
    }
}

impl From<ConcreteType> for RawType {
    fn from(val: ConcreteType) -> Self {
        match val.kind {
            ConcreteTypeKind::NamedStruct {
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

fn expect_integer_param(
    type_name: &str,
    index: usize,
    param: &TypeExprParam<'_>,
) -> Result<i32, ExtensionTypeError> {
    match param {
        TypeExprParam::Integer(value) => {
            i32::try_from(*value).map_err(|_| ExtensionTypeError::InvalidParameterValue {
                type_name: type_name.to_string(),
                index,
                value: *value,
                expected: "an i32",
            })
        }
        _ => Err(ExtensionTypeError::InvalidParameterKind {
            type_name: type_name.to_string(),
            index,
            expected: "an integer",
        }),
    }
}

fn expect_type_argument<'a>(
    type_name: &str,
    index: usize,
    param: TypeExprParam<'a>,
) -> Result<ConcreteType, ExtensionTypeError> {
    match param {
        TypeExprParam::Type(t) => ConcreteType::try_from(t),
        TypeExprParam::Integer(_) => Err(ExtensionTypeError::InvalidParameterKind {
            type_name: type_name.to_string(),
            index,
            expected: "a type",
        }),
    }
}

fn type_expr_param_to_type_parameter<'a>(
    param: TypeExprParam<'a>,
) -> Result<TypeParameter, ExtensionTypeError> {
    Ok(match param {
        TypeExprParam::Integer(v) => TypeParameter::Integer(v),
        TypeExprParam::Type(t) => TypeParameter::Type(ConcreteType::try_from(t)?),
    })
}

fn parse_parameterized_builtin<'a>(
    display_name: &str,
    lower_name: &str,
    params: &[TypeExprParam<'a>],
) -> Result<Option<BuiltinParameterized>, ExtensionTypeError> {
    match lower_name {
        "fixedchar" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let length = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::FixedChar { length }))
        }
        "varchar" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let length = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::VarChar { length }))
        }
        "fixedbinary" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let length = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::FixedBinary { length }))
        }
        "decimal" => {
            if params.len() != 2 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "2",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            let scale = expect_integer_param(display_name, 1, &params[1])?;
            Ok(Some(BuiltinParameterized::Decimal { precision, scale }))
        }
        // Should we accept both "precision_time" and "precisiontime"? The
        // docs/spec say PRECISIONTIME. The protos use underscores, so it could
        // show up in generated code, although maybe that's out of spec.
        "precisiontime" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::PrecisionTime { precision }))
        }
        "precisiontimestamp" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::PrecisionTimestamp { precision }))
        }
        "precisiontimestamptz" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::PrecisionTimestampTz {
                precision,
            }))
        }
        "interval_day" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::IntervalDay { precision }))
        }
        "interval_compound" => {
            if params.len() != 1 {
                return Err(ExtensionTypeError::InvalidParameterCount {
                    type_name: display_name.to_string(),
                    expected: "1",
                    actual: params.len(),
                });
            }
            let precision = expect_integer_param(display_name, 0, &params[0])?;
            Ok(Some(BuiltinParameterized::IntervalCompound { precision }))
        }
        _ => Ok(None),
    }
}

impl<'a> TryFrom<TypeExpr<'a>> for ConcreteType {
    type Error = ExtensionTypeError;

    fn try_from(parsed_type: TypeExpr<'a>) -> Result<Self, Self::Error> {
        match parsed_type {
            TypeExpr::Simple(name, params, nullable) => {
                let lower = name.to_ascii_lowercase();

                match lower.as_str() {
                    "list" => {
                        if params.len() != 1 {
                            return Err(ExtensionTypeError::InvalidParameterCount {
                                type_name: name.to_string(),
                                expected: "1",
                                actual: params.len(),
                            });
                        }
                        let element =
                            expect_type_argument(name, 0, params.into_iter().next().unwrap())?;
                        return Ok(ConcreteType::list(element, nullable));
                    }
                    "map" => {
                        if params.len() != 2 {
                            return Err(ExtensionTypeError::InvalidParameterCount {
                                type_name: name.to_string(),
                                expected: "2",
                                actual: params.len(),
                            });
                        }
                        let mut iter = params.into_iter();
                        let key = expect_type_argument(name, 0, iter.next().unwrap())?;
                        let value = expect_type_argument(name, 1, iter.next().unwrap())?;
                        return Ok(ConcreteType::map(key, value, nullable));
                    }
                    "struct" => {
                        let field_types = params
                            .into_iter()
                            .enumerate()
                            .map(|(idx, param)| expect_type_argument(name, idx, param))
                            .collect::<Result<Vec<_>, _>>()?;
                        return Ok(ConcreteType::r#struct(field_types, nullable));
                    }
                    _ => {}
                }

                if let Some(parameterized) =
                    parse_parameterized_builtin(name, lower.as_str(), &params)?
                {
                    return Ok(ConcreteType::parameterized_builtin(parameterized, nullable));
                }

                match PrimitiveType::from_str(&lower) {
                    Ok(builtin) => {
                        if !params.is_empty() {
                            return Err(ExtensionTypeError::InvalidParameterCount {
                                type_name: name.to_string(),
                                expected: "0",
                                actual: params.len(),
                            });
                        }
                        Ok(ConcreteType::builtin(builtin, nullable))
                    }
                    Err(_) => {
                        let parameters = params
                            .into_iter()
                            .map(type_expr_param_to_type_parameter)
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(ConcreteType::extension_with_params(
                            name.to_string(),
                            parameters,
                            nullable,
                        ))
                    }
                }
            }
            TypeExpr::UserDefined(name, params, nullable) => {
                let parameters = params
                    .into_iter()
                    .map(type_expr_param_to_type_parameter)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(ConcreteType::extension_with_params(
                    name.to_string(),
                    parameters,
                    nullable,
                ))
            }
            TypeExpr::TypeVariable(id, nullability) => {
                Err(ExtensionTypeError::InvalidAnyTypeVariable { id, nullability })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::extensions::TypeContext;
    use super::*;
    use crate::parse::text::simple_extensions::TypeExpr;
    use crate::parse::text::simple_extensions::argument::EnumOptions as ParsedEnumOptions;
    use crate::text::simple_extensions;
    use std::iter::FromIterator;

    /// Create a [ConcreteType] from a [BuiltinParameterized]
    fn concretize(builtin: BuiltinParameterized) -> ConcreteType {
        ConcreteType::parameterized_builtin(builtin, false)
    }

    /// Parse a string into a [ConcreteType]
    fn parse_type(expr: &str) -> ConcreteType {
        let parsed = TypeExpr::parse(expr).unwrap();
        ConcreteType::try_from(parsed).unwrap()
    }

    /// Parse a string into a builtin [ConcreteType], with no unresolved
    /// extension references
    fn parse_simple(s: &str) -> ConcreteType {
        let parsed = TypeExpr::parse(s).unwrap();

        let mut refs = Vec::new();
        parsed.visit_references(&mut |name| refs.push(name.to_string()));
        assert!(refs.is_empty(), "{s} should add a builtin type");

        ConcreteType::try_from(parsed).unwrap()
    }

    /// Create a type parameter from a type expression string
    fn type_param(expr: &str) -> TypeParameter {
        TypeParameter::Type(parse_type(expr))
    }

    /// Create an extension type
    fn extension(name: &str, parameters: Vec<TypeParameter>, nullable: bool) -> ConcreteType {
        ConcreteType::extension_with_params(name.to_string(), parameters, nullable)
    }

    /// Convert a custom type to raw and back, ensuring round-trip consistency
    fn round_trip(custom: &CustomType) {
        let item: simple_extensions::SimpleExtensionsTypesItem = custom.clone().into();
        let mut ctx = TypeContext::default();
        let parsed = Parse::parse(item, &mut ctx).unwrap();
        assert_eq!(&parsed, custom);
    }

    /// Create a raw named struct (e.g. straight from YAML) from field name and
    /// type pairs
    fn raw_named_struct(fields: &[(&str, &str)]) -> RawType {
        let map = serde_json::Map::from_iter(
            fields
                .iter()
                .map(|(name, ty)| ((*name).into(), serde_json::Value::String((*ty).into()))),
        );
        RawType::Variant1(map)
    }

    #[test]
    fn test_primitive_type_parsing() {
        let cases = vec![
            ("bool", Some(PrimitiveType::Boolean)),
            ("i8", Some(PrimitiveType::I8)),
            ("i16", Some(PrimitiveType::I16)),
            ("i32", Some(PrimitiveType::I32)),
            ("i64", Some(PrimitiveType::I64)),
            ("fp32", Some(PrimitiveType::Fp32)),
            ("fp64", Some(PrimitiveType::Fp64)),
            ("STRING", Some(PrimitiveType::String)),
            ("binary", Some(PrimitiveType::Binary)),
            ("uuid", Some(PrimitiveType::Uuid)),
            ("date", Some(PrimitiveType::Date)),
            ("interval_year", Some(PrimitiveType::IntervalYear)),
            ("time", Some(PrimitiveType::Time)),
            ("timestamp", Some(PrimitiveType::Timestamp)),
            ("timestamp_tz", Some(PrimitiveType::TimestampTz)),
            ("invalid", None),
        ];

        for (input, expected) in cases {
            match expected {
                Some(expected_type) => {
                    assert_eq!(
                        PrimitiveType::from_str(input).unwrap(),
                        expected_type,
                        "expected primitive type for {input}"
                    );
                }
                None => {
                    assert!(
                        PrimitiveType::from_str(input).is_err(),
                        "expected parsing {input} to fail"
                    );
                }
            }
        }
    }

    #[test]
    fn test_parameterized_builtin_types() {
        let cases = vec![
            (
                "precisiontime<2>",
                concretize(BuiltinParameterized::PrecisionTime { precision: 2 }),
            ),
            (
                "precisiontimestamp<1>",
                concretize(BuiltinParameterized::PrecisionTimestamp { precision: 1 }),
            ),
            (
                "precisiontimestamptz<5>",
                concretize(BuiltinParameterized::PrecisionTimestampTz { precision: 5 }),
            ),
            (
                "DECIMAL<10,2>",
                concretize(BuiltinParameterized::Decimal {
                    precision: 10,
                    scale: 2,
                }),
            ),
            (
                "fixedchar<12>",
                concretize(BuiltinParameterized::FixedChar { length: 12 }),
            ),
            (
                "VarChar<42>",
                concretize(BuiltinParameterized::VarChar { length: 42 }),
            ),
            (
                "fixedbinary<8>",
                concretize(BuiltinParameterized::FixedBinary { length: 8 }),
            ),
            (
                "interval_day<7>",
                concretize(BuiltinParameterized::IntervalDay { precision: 7 }),
            ),
            (
                "interval_compound<6>",
                concretize(BuiltinParameterized::IntervalCompound { precision: 6 }),
            ),
        ];

        for (expr, expected) in cases {
            let found = parse_simple(expr);
            assert_eq!(found, expected, "unexpected type for {expr}");
        }
    }

    #[test]
    fn test_container_types() {
        let cases = vec![
            (
                "List<i32>",
                ConcreteType::list(ConcreteType::builtin(PrimitiveType::I32, false), false),
            ),
            (
                "List<fp64?>",
                ConcreteType::list(ConcreteType::builtin(PrimitiveType::Fp64, true), false),
            ),
            (
                "Map?<i64, string?>",
                ConcreteType::map(
                    ConcreteType::builtin(PrimitiveType::I64, false),
                    ConcreteType::builtin(PrimitiveType::String, true),
                    true,
                ),
            ),
            (
                "Struct?<i8, string?>",
                ConcreteType::r#struct(
                    vec![
                        ConcreteType::builtin(PrimitiveType::I8, false),
                        ConcreteType::builtin(PrimitiveType::String, true),
                    ],
                    true,
                ),
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(parse_type(expr), expected, "unexpected parse for {expr}");
        }
    }

    #[test]
    fn test_extension_types() {
        let cases = vec![
            (
                "u!geo<List<i32>, 10>",
                extension(
                    "geo",
                    vec![type_param("List<i32>"), TypeParameter::Integer(10)],
                    false,
                ),
            ),
            (
                "Geo?<List<i32?>>",
                extension("Geo", vec![type_param("List<i32?>")], true),
            ),
            (
                "Custom<string?, bool>",
                extension(
                    "Custom",
                    vec![
                        type_param("string?"),
                        TypeParameter::Type(ConcreteType::builtin(PrimitiveType::Boolean, false)),
                    ],
                    false,
                ),
            ),
        ];

        for (expr, expected) in cases {
            assert_eq!(
                parse_type(expr),
                expected,
                "unexpected extension for {expr}"
            );
        }
    }

    #[test]
    fn test_parameter_type_validation() {
        let int_param = ParameterConstraint::Integer {
            min: Some(1),
            max: Some(10),
        };
        let enum_param = ParameterConstraint::Enumeration {
            options: ParsedEnumOptions::try_from(simple_extensions::EnumOptions(vec![
                "OVERFLOW".to_string(),
                "ERROR".to_string(),
            ]))
            .unwrap(),
        };

        let cases = vec![
            (&int_param, Value::Number(5.into()), true),
            (&int_param, Value::Number(0.into()), false),
            (&int_param, Value::Number(11.into()), false),
            (&int_param, Value::String("not a number".into()), false),
            (&enum_param, Value::String("OVERFLOW".into()), true),
            (&enum_param, Value::String("INVALID".into()), false),
        ];

        for (param, value, expected) in cases {
            assert_eq!(
                param.is_valid_value(&value),
                expected,
                "unexpected validation result for {value:?}"
            );
        }
    }

    #[test]
    fn test_integer_param_bounds_round_trip() {
        let cases = vec![
            (
                "bounded",
                simple_extensions::TypeParamDefsItem {
                    name: Some("K".to_string()),
                    description: None,
                    type_: simple_extensions::TypeParamDefsItemType::Integer,
                    min: Some(1.0),
                    max: Some(10.0),
                    options: None,
                    optional: None,
                },
                (Some(1), Some(10)),
            ),
            (
                "truncated",
                simple_extensions::TypeParamDefsItem {
                    name: Some("K".to_string()),
                    description: None,
                    type_: simple_extensions::TypeParamDefsItemType::Integer,
                    min: Some(1.5),
                    max: None,
                    options: None,
                    optional: None,
                },
                (Some(1), None),
            ),
        ];

        for (label, item, (expected_min, expected_max)) in cases {
            let tp = TypeParam::try_from(item).expect("should parse integer bounds");
            match tp.param_type {
                ParameterConstraint::Integer { min, max } => {
                    assert_eq!(min, expected_min, "min mismatch for {label}");
                    assert_eq!(max, expected_max, "max mismatch for {label}");
                }
                _ => panic!("expected integer param type for {label}"),
            }
        }
    }

    #[test]
    fn test_custom_type_round_trip() -> Result<(), ExtensionTypeError> {
        let fields = vec![
            (
                "x".to_string(),
                ConcreteType::builtin(PrimitiveType::Fp64, false),
            ),
            (
                "y".to_string(),
                ConcreteType::builtin(PrimitiveType::Fp64, false),
            ),
        ];
        let (names, types): (Vec<_>, Vec<_>) = fields.into_iter().unzip();

        let cases = vec![
            CustomType::new(
                "AliasType".to_string(),
                vec![],
                Some(ConcreteType::builtin(PrimitiveType::I32, false)),
                None,
                Some("a test alias type".to_string()),
            )?,
            CustomType::new(
                "Point".to_string(),
                vec![TypeParam::new(
                    "T".to_string(),
                    ParameterConstraint::DataType,
                    Some("a numeric value".to_string()),
                )],
                Some(ConcreteType::named_struct(names, types, false)),
                None,
                None,
            )?,
        ];

        for custom in cases {
            round_trip(&custom);
        }

        Ok(())
    }

    #[test]
    fn test_invalid_type_names() {
        let cases = vec![
            ("", false),
            ("bad name", false),
            ("GoodName", true),
            ("also_good", true),
        ];

        for (name, expected_ok) in cases {
            let result = CustomType::validate_name(name);
            assert_eq!(
                result.is_ok(),
                expected_ok,
                "unexpected validation for {name}"
            );
        }
    }

    #[test]
    fn test_ext_type_to_concrete_type() -> Result<(), ExtensionTypeError> {
        let cases = vec![
            (
                "alias",
                RawType::Variant0("i32".to_string()),
                ConcreteType::builtin(PrimitiveType::I32, false),
            ),
            (
                "named_struct",
                raw_named_struct(&[("field1", "fp64"), ("field2", "i32?")]),
                ConcreteType::named_struct(
                    vec!["field1".to_string(), "field2".to_string()],
                    vec![
                        ConcreteType::builtin(PrimitiveType::Fp64, false),
                        ConcreteType::builtin(PrimitiveType::I32, true),
                    ],
                    false,
                ),
            ),
        ];

        for (label, raw, expected) in cases {
            let mut ctx = TypeContext::default();
            let parsed = Parse::parse(raw, &mut ctx)?;
            assert_eq!(parsed, expected, "unexpected type for {label}");
        }

        Ok(())
    }

    #[test]
    fn test_custom_type_parsing() -> Result<(), ExtensionTypeError> {
        let cases = vec![
            (
                "alias",
                simple_extensions::SimpleExtensionsTypesItem {
                    name: "Alias".to_string(),
                    description: Some("Alias type".to_string()),
                    parameters: None,
                    structure: Some(RawType::Variant0("BINARY".to_string())),
                    variadic: None,
                },
                "Alias",
                Some("Alias type"),
                Some(ConcreteType::builtin(PrimitiveType::Binary, false)),
            ),
            (
                "named_struct",
                simple_extensions::SimpleExtensionsTypesItem {
                    name: "Point".to_string(),
                    description: Some("A 2D point".to_string()),
                    parameters: None,
                    structure: Some(raw_named_struct(&[("x", "fp64"), ("y", "fp64?")])),
                    variadic: None,
                },
                "Point",
                Some("A 2D point"),
                Some(ConcreteType::named_struct(
                    vec!["x".to_string(), "y".to_string()],
                    vec![
                        ConcreteType::builtin(PrimitiveType::Fp64, false),
                        ConcreteType::builtin(PrimitiveType::Fp64, true),
                    ],
                    false,
                )),
            ),
            (
                "no_structure",
                simple_extensions::SimpleExtensionsTypesItem {
                    name: "Opaque".to_string(),
                    description: None,
                    parameters: None,
                    structure: None,
                    variadic: Some(true),
                },
                "Opaque",
                None,
                None,
            ),
        ];

        for (label, item, expected_name, expected_description, expected_structure) in cases {
            let mut ctx = TypeContext::default();
            let parsed = Parse::parse(item, &mut ctx)?;
            assert_eq!(parsed.name, expected_name);
            assert_eq!(
                parsed.description.as_deref(),
                expected_description,
                "description mismatch for {label}"
            );
            assert_eq!(
                parsed.structure, expected_structure,
                "structure mismatch for {label}"
            );
        }

        Ok(())
    }

    /// A type defined with a structure cannot be defined as nullable; e.g. if
    /// you define 'Integer' as an alias for 'i64?', then what do you mean by
    /// 'INTEGER?' - is that now equal to `i64??`
    #[test]
    fn test_nullable_structure_rejected() {
        let cases = vec![RawType::Variant0("i32?".to_string())];

        for raw in cases {
            let mut ctx = TypeContext::default();
            let result = Parse::parse(raw, &mut ctx);
            match result {
                Err(ExtensionTypeError::StructureCannotBeNullable { type_string }) => {
                    assert!(type_string.contains('?'));
                }
                other => panic!("Expected nullable structure error, got {other:?}"),
            }
        }
    }
}
