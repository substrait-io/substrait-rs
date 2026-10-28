// SPDX-License-Identifier: Apache-2.0

//! Owned type expressions used by the extension registry.

use super::TypeExpr;
use super::argument::{
    EnumOptions as ParsedEnumOptions, EnumOptionsError as ParsedEnumOptionsError,
};
use super::extensions::TypeContext;
use super::numeric_expression::NumericExpression;
use super::type_ast::TypeExprParam;
use crate::parse::Parse;
use crate::parse::text::simple_extensions::type_ast::TypeParseError;
use crate::text::simple_extensions::{
    EnumOptions as RawEnumOptions, SimpleExtensionsTypesItem, Type as RawType, TypeParamDefs,
    TypeParamDefsItem, TypeParamDefsItemType,
};
use indexmap::IndexMap;
use serde_json::Value;
use std::convert::TryFrom;
use std::fmt;
use std::ops::RangeInclusive;
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

/// The structure of a type expression, without its nullability.
#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
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
    /// Year-month interval - `interval_year`
    IntervalYear,
    /// 128-bit UUID - `uuid`
    Uuid,
    /// Fixed-length character string: `fixedchar<L>`
    FixedChar {
        /// Length (number of characters), must be >= 1
        length: IntegerParameter,
    },
    /// Variable-length character string: `varchar<L>`
    VarChar {
        /// Maximum length (number of characters), must be >= 1
        length: IntegerParameter,
    },
    /// Fixed-length binary data: `fixedbinary<L>`
    FixedBinary {
        /// Length (number of bytes), must be >= 1
        length: IntegerParameter,
    },
    /// Fixed-point decimal: `decimal<P, S>`
    Decimal {
        /// Precision (total digits), <= 38
        precision: IntegerParameter,
        /// Scale (digits after decimal point), 0 <= S <= P
        scale: IntegerParameter,
    },
    /// Time with sub-second precision: `precision_time<P>`
    PrecisionTime {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: IntegerParameter,
    },
    /// Timestamp with sub-second precision: `precision_timestamp<P>`
    PrecisionTimestamp {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: IntegerParameter,
    },
    /// Timezone-aware timestamp with precision: `precision_timestamp_tz<P>`
    PrecisionTimestampTz {
        /// Sub-second precision digits (0-12: seconds to picoseconds)
        precision: IntegerParameter,
    },
    /// Day-time interval: `interval_day<P>`
    IntervalDay {
        /// Sub-second precision digits (0-9: seconds to nanoseconds)
        precision: IntegerParameter,
    },
    /// Compound interval: `interval_compound<P>`
    IntervalCompound {
        /// Sub-second precision digits
        precision: IntegerParameter,
    },
    /// Extension type with optional parameters
    Extension {
        /// Extension type name
        name: String,
        /// Type parameters
        parameters: Vec<TypeParameter>,
    },
    /// List type with element type
    List(Box<Type>),
    /// Map type with key and value types
    Map {
        /// Key type
        key: Box<Type>,
        /// Value type
        value: Box<Type>,
    },
    /// Struct type (ordered fields without names)
    Struct(Vec<Type>),
    /// Named struct type (nstruct - ordered fields with names)
    NamedStruct {
        /// Ordered field names and types. They are in the order they should
        /// appear in the struct - hence the use of [`IndexMap`].
        fields: IndexMap<String, Type>,
    },
    /// A function type: `func<P1, P2 -> R>`
    Func {
        /// Parameter types in declaration order
        parameters: Vec<Type>,
        /// The return type
        return_type: Box<Type>,
    },
    /// An unnumbered wildcard, such as `any`.
    Any,
    /// A numbered type variable, such as `any1`.
    Variable {
        /// The variable identifier shared within a function signature.
        id: u32,
    },
}

impl TypeKind {
    /// Check if a string is a valid name for a builtin scalar type
    pub fn is_name(name: &str) -> bool {
        let lower = name.to_ascii_lowercase();
        primitive_builtin(&lower).is_some()
            || matches!(
                lower.as_str(),
                "fixedchar"
                    | "varchar"
                    | "fixedbinary"
                    | "decimal"
                    | "precision_time"
                    | "precision_timestamp"
                    | "precision_timestamp_tz"
                    | "interval_day"
                    | "interval_compound"
            )
    }

    fn fmt_with_nullability(&self, f: &mut fmt::Formatter<'_>, nullable: bool) -> fmt::Result {
        let nullability = if nullable { "?" } else { "" };
        let name = match self {
            Self::Boolean => "bool",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::Fp32 => "fp32",
            Self::Fp64 => "fp64",
            Self::String => "string",
            Self::Binary => "binary",
            Self::Date => "date",
            Self::IntervalYear => "interval_year",
            Self::Uuid => "uuid",
            Self::FixedChar { .. } => "fixedchar",
            Self::VarChar { .. } => "varchar",
            Self::FixedBinary { .. } => "fixedbinary",
            Self::Decimal { .. } => "decimal",
            Self::PrecisionTime { .. } => "precision_time",
            Self::PrecisionTimestamp { .. } => "precision_timestamp",
            Self::PrecisionTimestampTz { .. } => "precision_timestamp_tz",
            Self::IntervalDay { .. } => "interval_day",
            Self::IntervalCompound { .. } => "interval_compound",
            Self::List(_) => "list",
            Self::Map { .. } => "map",
            Self::Struct(_) => "struct",
            Self::Func { .. } => "func",
            Self::Extension { name, parameters } => {
                write!(f, "u!{name}{nullability}")?;
                return write_separated(f, parameters.iter(), "<", ">", ", ");
            }
            Self::NamedStruct { fields } => {
                let fields = fields
                    .iter()
                    .map(|(name, field)| KeyValueDisplay(name, field, ": "));
                write_separated(f, fields, "{", "}", ", ")?;
                return f.write_str(nullability);
            }
            Self::Any => return write!(f, "any{nullability}"),
            Self::Variable { id } => return write!(f, "any{id}{nullability}"),
        };
        write!(f, "{name}{nullability}")?;
        match self {
            Self::FixedChar { length }
            | Self::VarChar { length }
            | Self::FixedBinary { length } => {
                write!(f, "<{length}>")
            }
            Self::Decimal { precision, scale } => write!(f, "<{precision}, {scale}>"),
            Self::PrecisionTime { precision }
            | Self::PrecisionTimestamp { precision }
            | Self::PrecisionTimestampTz { precision }
            | Self::IntervalDay { precision }
            | Self::IntervalCompound { precision } => write!(f, "<{precision}>"),
            Self::List(element) => write!(f, "<{element}>"),
            Self::Map { key, value } => write!(f, "<{key}, {value}>"),
            Self::Struct(fields) => {
                f.write_str("<")?;
                write_separated(f, fields.iter(), "", "", ", ")?;
                f.write_str(">")
            }
            Self::Func {
                parameters,
                return_type,
            } => {
                if parameters.len() == 1 {
                    write!(f, "<{} -> {return_type}>", parameters[0])
                } else {
                    f.write_str("<(")?;
                    write_separated(f, parameters.iter(), "", "", ", ")?;
                    write!(f, ") -> {return_type}>")
                }
            }
            _ => Ok(()),
        }
    }
}

/// Formats the full type spelling used in extension YAML, not the short name
/// used in function signatures (e.g., `precision_timestamp<3>`, not `pts`).
impl fmt::Display for TypeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_nullability(f, false)
    }
}

/// An integer literal, named parameter, or unevaluated numeric expression.
#[derive(Clone, Debug, PartialEq)]
pub enum IntegerParameter {
    /// A known integer value.
    Literal(i64),
    /// A name whose value depends on a function call.
    Variable(String),
    /// An arithmetic expression retained without evaluation.
    Expression(NumericExpression),
}

impl IntegerParameter {
    /// Returns whether this parameter is a literal rather than a variable.
    pub fn is_concrete(&self) -> bool {
        matches!(self, Self::Literal(_))
    }
}

impl fmt::Display for IntegerParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(value) => write!(f, "{value}"),
            Self::Variable(name) => f.write_str(name),
            Self::Expression(expression) => write!(f, "{expression}"),
        }
    }
}

/// A parameter, used in parameterized types
#[derive(Clone, Debug, PartialEq)]
pub enum TypeParameter {
    /// Integer parameter (e.g., precision, scale)
    Integer(IntegerParameter),
    /// Type parameter (nested type)
    Type(Type),
    /// A name whose parameter kind depends on the user-defined type declaration.
    Name(String),
    // TODO: Add support for other type parameters, as described in
    // https://github.com/substrait-io/substrait/blob/35101020d961eda48f8dd1aafbc794c9e5cac077/proto/substrait/type.proto#L250-L265
}

impl TypeParameter {
    /// Returns whether this parameter contains no unresolved variables.
    pub fn is_concrete(&self) -> bool {
        match self {
            Self::Integer(parameter) => parameter.is_concrete(),
            Self::Type(type_) => type_.is_concrete(),
            Self::Name(_) => false,
        }
    }
}

impl fmt::Display for TypeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeParameter::Integer(i) => write!(f, "{i}"),
            TypeParameter::Type(t) => write!(f, "{t}"),
            TypeParameter::Name(name) => f.write_str(name),
        }
    }
}

/// Parse a primitive (no type parameters) builtin type name
fn primitive_builtin(lower_name: &str) -> Option<TypeKind> {
    match lower_name {
        "bool" | "boolean" => Some(TypeKind::Boolean),
        "i8" => Some(TypeKind::I8),
        "i16" => Some(TypeKind::I16),
        "i32" => Some(TypeKind::I32),
        "i64" => Some(TypeKind::I64),
        "fp32" => Some(TypeKind::Fp32),
        "fp64" => Some(TypeKind::Fp64),
        "string" => Some(TypeKind::String),
        "binary" => Some(TypeKind::Binary),
        "date" => Some(TypeKind::Date),
        "interval_year" => Some(TypeKind::IntervalYear),
        "uuid" => Some(TypeKind::Uuid),
        _ => None,
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
                match (min, max) {
                    (Some(min_f), _) if min_f.fract() != 0.0 => {
                        return Err(TypeParamError::InvalidIntegerBounds { min, max });
                    }
                    (_, Some(max_f)) if max_f.fract() != 0.0 => {
                        return Err(TypeParamError::InvalidIntegerBounds { min, max });
                    }
                    _ => (),
                }

                let min_i = min.map(|v| v as i64);
                let max_i = max.map(|v| v as i64);
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
    /// Type variables or named numeric parameters are not supported in this context.
    #[error("Type '{type_string}' must be concrete in this context")]
    ExpectedConcreteType {
        /// The type expression containing unresolved parameters.
        type_string: String,
    },
    /// Unknown type name (not a builtin, missing u! prefix for extension types)
    #[error(
        "Unknown type name: '{}'. Extension types must use the u! prefix (e.g., u!{})",
        name,
        name
    )]
    UnknownTypeName {
        /// The unknown type name
        name: String,
    },
    /// Parameter validation failed
    #[error("Invalid parameter: {0}")]
    InvalidParameter(#[from] TypeParamError),
    /// Field type is invalid
    #[error("Invalid structure field type: {0}")]
    InvalidFieldType(String),
    /// Duplicate struct field name
    #[error("Duplicate struct field '{field_name}'")]
    DuplicateFieldName {
        /// The duplicated field name
        field_name: String,
    },
    /// Type parameter count is invalid for the given type name
    #[error("Type '{type_name}' expects {expected} parameters, got {actual}")]
    InvalidParameterCount {
        /// The type name being validated
        type_name: String,
        /// Expected number of parameters
        expected: usize,
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
    #[error("Type '{type_name}' parameter {index} value {value} is not within {expected}")]
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
    /// Provided parameter value does not fit within the expected bounds
    #[error("Type '{type_name}' parameter {index} value {value} is out of range {expected:?}")]
    InvalidParameterRange {
        /// The type name being validated
        type_name: String,
        /// Zero-based index of the offending parameter
        index: usize,
        /// Provided parameter value
        value: i64,
        /// Description of the expected range or type
        expected: RangeInclusive<i32>,
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
    /// Structure definition, if any
    pub structure: Option<Type>,
    /// Whether this type can have variadic parameters
    pub variadic: Option<bool>,
    /// Human-readable description
    pub description: Option<String>,
}

impl CustomType {
    /// Check if this type name is valid according to Substrait naming rules
    /// (see the `Identifier` rule in `substrait/grammar/SubstraitLexer.g4`).
    /// Identifiers are case-insensitive and must start with a an ASCII letter,
    /// `_`, or `$`, followed by ASCII letters, digits, `_`, or `$`.
    //
    // Note: I'm not sure if `$` is actually something we want to allow, or if
    // `_` is, but it's in the grammar so I'm allowing it here.
    pub fn validate_name(name: &str) -> Result<(), InvalidTypeName> {
        let mut chars = name.chars();
        let first = chars
            .next()
            .ok_or_else(|| InvalidTypeName(name.to_string()))?;
        if !(first.is_ascii_alphabetic() || first == '_' || first == '$') {
            return Err(InvalidTypeName(name.to_string()));
        }

        if !chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$') {
            return Err(InvalidTypeName(name.to_string()));
        }

        Ok(())
    }

    /// Create a new custom type with validation
    pub fn new(
        name: String,
        parameters: Vec<TypeParam>,
        structure: Option<Type>,
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
                            // TODO: add this to TypeParamDefsItem parsing, and
                            // follow it through here. I'm not entirely sure
                            // when/if it is used.
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
            metadata: Default::default(),
            parameters,
            structure,
            variadic: value.variadic,
            // Not tracked by `CustomType`; default to no deprecation.
            deprecated: None,
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
            Some(structure_data) => {
                let parsed = Parse::parse(structure_data, ctx)?;
                // TODO: check that the structure is valid. The `Type::Object`
                // form of `structure_data` is by definition a non-nullable `NSTRUCT`; however,
                // what types allowed under the `Type::String` form is less clear in the spec:
                // See https://github.com/substrait-io/substrait/issues/920.
                Some(parsed)
            }
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
    type Parsed = Type;
    type Error = ExtensionTypeError;

    fn parse(self, ctx: &mut TypeContext) -> Result<Self::Parsed, Self::Error> {
        match self {
            RawType::String(type_string) => {
                let parsed_type = TypeExpr::parse(&type_string)?;
                let mut link = |name: &str| ctx.linked(name);
                parsed_type.visit_references(&mut link);
                expect_concrete_type(parsed_type)
            }
            RawType::Object(field_map) => {
                // Type structure in Substrait must preserve field order (see
                // substrait-io/substrait#915). The typify generation uses
                // IndexMap to retain the YAML order so that the order of the
                // fields in the structure matches that of the extensions file.
                let mut fields = IndexMap::new();

                for (field_name, field_type_value) in field_map {
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
                    let field_type = expect_concrete_type(parsed_field_type)?;

                    if fields.insert(field_name.clone(), field_type).is_some() {
                        return Err(ExtensionTypeError::DuplicateFieldName { field_name });
                    }
                }

                Ok(Type {
                    kind: TypeKind::NamedStruct { fields },
                    nullable: false,
                })
            }
        }
    }
}

/// Invalid type name error
#[derive(Debug, Error, PartialEq)]
#[error("invalid type name `{0}`")]
pub struct InvalidTypeName(String);

/// A type expression with nullability.
#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    /// The type expression
    pub kind: TypeKind,
    /// Whether this type is nullable
    pub nullable: bool,
}

impl Type {
    /// Create a type expression with the given nullability.
    pub fn new(kind: TypeKind, nullable: bool) -> Type {
        Type { kind, nullable }
    }

    /// Create a new extension type reference (without parameters)
    pub fn extension(name: String, nullable: bool) -> Type {
        Type {
            kind: TypeKind::Extension {
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
    ) -> Type {
        Type {
            kind: TypeKind::Extension { name, parameters },
            nullable,
        }
    }

    /// Create a new list type
    pub fn list(element_type: Type, nullable: bool) -> Type {
        Type {
            kind: TypeKind::List(Box::new(element_type)),
            nullable,
        }
    }

    /// Create a new struct type (ordered fields without names)
    pub fn r#struct(field_types: Vec<Type>, nullable: bool) -> Type {
        Type {
            kind: TypeKind::Struct(field_types),
            nullable,
        }
    }

    /// Create a new map type
    pub fn map(key_type: Type, value_type: Type, nullable: bool) -> Type {
        Type {
            kind: TypeKind::Map {
                key: Box::new(key_type),
                value: Box::new(value_type),
            },
            nullable,
        }
    }

    /// Create a new named struct type (nstruct - ordered fields with names)
    pub fn named_struct(fields: IndexMap<String, Type>, nullable: bool) -> Type {
        Type {
            kind: TypeKind::NamedStruct { fields },
            nullable,
        }
    }

    /// Returns whether this type contains no unresolved variables.
    pub fn is_concrete(&self) -> bool {
        match &self.kind {
            TypeKind::Any | TypeKind::Variable { .. } => false,
            TypeKind::FixedChar { length }
            | TypeKind::VarChar { length }
            | TypeKind::FixedBinary { length } => length.is_concrete(),
            TypeKind::Decimal { precision, scale } => {
                precision.is_concrete() && scale.is_concrete()
            }
            TypeKind::PrecisionTime { precision }
            | TypeKind::PrecisionTimestamp { precision }
            | TypeKind::PrecisionTimestampTz { precision }
            | TypeKind::IntervalDay { precision }
            | TypeKind::IntervalCompound { precision } => precision.is_concrete(),
            TypeKind::List(element) => element.is_concrete(),
            TypeKind::Map { key, value } => key.is_concrete() && value.is_concrete(),
            TypeKind::Struct(fields) => fields.iter().all(Type::is_concrete),
            TypeKind::NamedStruct { fields } => fields.values().all(Type::is_concrete),
            TypeKind::Func {
                parameters,
                return_type,
            } => parameters.iter().all(Type::is_concrete) && return_type.is_concrete(),
            TypeKind::Extension { parameters, .. } => {
                parameters.iter().all(TypeParameter::is_concrete)
            }
            _ => true,
        }
    }

    /// Check if this type (as a function argument) is compatible with another
    /// type (as an input).
    ///
    /// Does not bind type variables or symbolic parameters.
    /// Mainly checks nullability:
    ///   - `i64?` is compatible with `i64` and `i64?` - both can be passed as
    ///     arguments
    ///   - `i64` is compatible with `i64` but NOT `i64?` - you can't pass a
    ///     nullable type to a function that only accepts non-nullable arguments
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        // Types must match exactly, but nullable types can accept non-nullable values
        self.kind == other.kind && (self.nullable || !other.nullable)
    }
}

/// Formats full extension-YAML spellings, not function-signature short names.
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt_with_nullability(f, self.nullable)
    }
}

impl From<Type> for RawType {
    fn from(val: Type) -> Self {
        match val.kind {
            TypeKind::NamedStruct { fields } => RawType::Object(
                fields
                    .into_iter()
                    .map(|(name, ty)| (name, serde_json::Value::String(ty.to_string())))
                    .collect(),
            ),
            _ => RawType::String(val.to_string()),
        }
    }
}

/// Parse an integer literal or parameter name for a built-in type.
/// Validate literals against the supplied range.
///
/// - `type_name`: Type being validated (for error messages, e.g., "DECIMAL")
/// - `index`: Parameter position (0-based, e.g., 0 for precision, 1 for scale);
///   needed for error messages
/// - `param`: The parameter to validate
/// - `range`: Optional bounds to enforce (e.g., `Some(1..=38)` for precision)
fn parse_integer_parameter(
    type_name: &str,
    index: usize,
    param: &TypeExprParam<'_>,
    range: Option<RangeInclusive<i32>>,
) -> Result<IntegerParameter, ExtensionTypeError> {
    let value = match param {
        TypeExprParam::Expression(expression) => {
            return Ok(IntegerParameter::Expression(expression.clone()));
        }
        TypeExprParam::Type(TypeExpr::Simple(name, parameters, false))
            if parameters.is_empty() && CustomType::validate_name(name).is_ok() =>
        {
            return Ok(IntegerParameter::Variable(name.to_string()));
        }
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
    }?;

    if let Some(range) = range {
        if range.contains(&value) {
            return Ok(IntegerParameter::Literal(i64::from(value)));
        }
        return Err(ExtensionTypeError::InvalidParameterRange {
            type_name: type_name.to_string(),
            index,
            value: i64::from(value),
            expected: range,
        });
    }

    Ok(IntegerParameter::Literal(i64::from(value)))
}

/// Helper function - checks that param length matches expectations, returns
/// error if not. Assumes a fixed number of expected parameters.
fn expect_param_len(
    type_name: &str,
    params: &[TypeExprParam<'_>],
    expected: usize,
) -> Result<(), ExtensionTypeError> {
    if params.len() != expected {
        return Err(ExtensionTypeError::InvalidParameterCount {
            type_name: type_name.to_string(),
            expected,
            actual: params.len(),
        });
    }
    Ok(())
}

/// Parse a concrete type with all parameters fully defined, without type variables
/// or named numeric parameters.
fn expect_concrete_type(expression: TypeExpr<'_>) -> Result<Type, ExtensionTypeError> {
    match expression {
        TypeExpr::TypeVariable(id, nullability) => {
            Err(ExtensionTypeError::InvalidAnyTypeVariable { id, nullability })
        }
        expression => {
            let parsed = Type::try_from(expression)?;
            if parsed.is_concrete() {
                Ok(parsed)
            } else {
                Err(ExtensionTypeError::ExpectedConcreteType {
                    type_string: parsed.to_string(),
                })
            }
        }
    }
}

/// Helper function - expect a type parameter, and return the [Type] if it is a [TypeExpr]
/// or an error if not.
fn expect_type_argument<'a>(
    type_name: &str,
    index: usize,
    param: TypeExprParam<'a>,
) -> Result<Type, ExtensionTypeError> {
    match param {
        TypeExprParam::Type(t) => Type::try_from(t),
        TypeExprParam::Integer(_) | TypeExprParam::Expression(_) => {
            Err(ExtensionTypeError::InvalidParameterKind {
                type_name: type_name.to_string(),
                index,
                expected: "a type",
            })
        }
    }
}

impl<'a> TryFrom<TypeExprParam<'a>> for TypeParameter {
    type Error = ExtensionTypeError;

    fn try_from(param: TypeExprParam<'a>) -> Result<Self, Self::Error> {
        Ok(match param {
            TypeExprParam::Integer(v) => TypeParameter::Integer(IntegerParameter::Literal(v)),
            TypeExprParam::Expression(expression) => {
                TypeParameter::Integer(IntegerParameter::Expression(expression))
            }
            TypeExprParam::Type(expression) => {
                let is_bare_name = matches!(
                    &expression,
                    TypeExpr::Simple(_, parameters, false) if parameters.is_empty()
                );
                match Type::try_from(expression) {
                    Err(ExtensionTypeError::UnknownTypeName { name }) if is_bare_name => {
                        CustomType::validate_name(&name)?;
                        TypeParameter::Name(name)
                    }
                    result => TypeParameter::Type(result?),
                }
            }
        })
    }
}

/// The tightest provable upper bound for a decimal scale, per the
/// [decimal type constraints](https://substrait.io/types/type_classes/#compound-types).
fn max_literal_scale(precision: &IntegerParameter) -> i32 {
    match precision {
        IntegerParameter::Literal(precision) => *precision as i32,
        // An unresolved precision must eventually satisfy the decimal precision limit.
        IntegerParameter::Variable(_) | IntegerParameter::Expression(_) => 38,
    }
}

/// Parse a builtin type. Returns an `ExtensionTypeError` if the type name is
/// matched, but parameters are incorrect; returns `Some(None)` if the type is
/// not known.
fn parse_builtin<'a>(
    display_name: &str,
    lower_name: &str,
    params: &[TypeExprParam<'a>],
) -> Result<Option<TypeKind>, ExtensionTypeError> {
    if let Some(builtin) = primitive_builtin(lower_name) {
        expect_param_len(display_name, params, 0)?;
        return Ok(Some(builtin));
    }

    match lower_name {
        // Parameterized builtins
        "fixedchar" => {
            expect_param_len(display_name, params, 1)?;
            let length = parse_integer_parameter(display_name, 0, &params[0], None)?;
            Ok(Some(TypeKind::FixedChar { length }))
        }
        "varchar" => {
            expect_param_len(display_name, params, 1)?;
            let length = parse_integer_parameter(display_name, 0, &params[0], None)?;
            Ok(Some(TypeKind::VarChar { length }))
        }
        "fixedbinary" => {
            expect_param_len(display_name, params, 1)?;
            let length = parse_integer_parameter(display_name, 0, &params[0], None)?;
            Ok(Some(TypeKind::FixedBinary { length }))
        }
        "decimal" => {
            expect_param_len(display_name, params, 2)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], Some(1..=38))?;
            let scale = parse_integer_parameter(
                display_name,
                1,
                &params[1],
                Some(0..=max_literal_scale(&precision)),
            )?;
            Ok(Some(TypeKind::Decimal { precision, scale }))
        }
        "precision_time" => {
            expect_param_len(display_name, params, 1)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], Some(0..=12))?;
            Ok(Some(TypeKind::PrecisionTime { precision }))
        }
        "precision_timestamp" => {
            expect_param_len(display_name, params, 1)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], Some(0..=12))?;
            Ok(Some(TypeKind::PrecisionTimestamp { precision }))
        }
        "precision_timestamp_tz" => {
            expect_param_len(display_name, params, 1)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], Some(0..=12))?;
            Ok(Some(TypeKind::PrecisionTimestampTz { precision }))
        }
        "interval_day" => {
            expect_param_len(display_name, params, 1)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], Some(0..=9))?;
            Ok(Some(TypeKind::IntervalDay { precision }))
        }
        "interval_compound" => {
            expect_param_len(display_name, params, 1)?;
            let precision = parse_integer_parameter(display_name, 0, &params[0], None)?;
            Ok(Some(TypeKind::IntervalCompound { precision }))
        }
        _ => Ok(None),
    }
}

impl<'a> TryFrom<TypeExpr<'a>> for Type {
    type Error = ExtensionTypeError;

    fn try_from(parsed_type: TypeExpr<'a>) -> Result<Self, Self::Error> {
        match parsed_type {
            TypeExpr::Func(parameters, return_type, nullable) => {
                let parameters = parameters
                    .into_iter()
                    .enumerate()
                    .map(|(index, parameter)| expect_type_argument("func", index, parameter))
                    .collect::<Result<Vec<_>, _>>()?;
                let return_type = Box::new(Type::try_from(*return_type)?);
                Ok(Type::new(
                    TypeKind::Func {
                        parameters,
                        return_type,
                    },
                    nullable,
                ))
            }
            TypeExpr::Simple(name, params, nullable) => {
                let lower = name.to_ascii_lowercase();

                match lower.as_str() {
                    "any" => {
                        expect_param_len(name, &params, 0)?;
                        return Ok(Type::new(TypeKind::Any, nullable));
                    }
                    "list" => {
                        expect_param_len(name, &params, 1)?;
                        let element =
                            expect_type_argument(name, 0, params.into_iter().next().unwrap())?;
                        return Ok(Type::list(element, nullable));
                    }
                    "map" => {
                        expect_param_len(name, &params, 2)?;
                        let mut iter = params.into_iter();
                        let key = expect_type_argument(name, 0, iter.next().unwrap())?;
                        let value = expect_type_argument(name, 1, iter.next().unwrap())?;
                        return Ok(Type::map(key, value, nullable));
                    }
                    "struct" => {
                        let field_types = params
                            .into_iter()
                            .enumerate()
                            .map(|(idx, param)| expect_type_argument(name, idx, param))
                            .collect::<Result<Vec<_>, _>>()?;
                        return Ok(Type::r#struct(field_types, nullable));
                    }
                    _ => {}
                }

                if let Some(builtin) = parse_builtin(name, lower.as_str(), &params)? {
                    return Ok(Type::new(builtin, nullable));
                }

                // Simple types that aren't builtins are unknown
                // Extension types MUST use the u! prefix
                Err(ExtensionTypeError::UnknownTypeName {
                    name: name.to_string(),
                })
            }
            TypeExpr::UserDefined(name, params, nullable) => {
                let parameters = params
                    .into_iter()
                    .map(TypeParameter::try_from)
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Type::extension_with_params(
                    name.to_string(),
                    parameters,
                    nullable,
                ))
            }
            TypeExpr::TypeVariable(id, nullable) => {
                Ok(Type::new(TypeKind::Variable { id }, nullable))
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

    /// Wrap a type kind without a nullable annotation.
    fn required_type(kind: TypeKind) -> Type {
        Type::new(kind, false)
    }

    /// Parse a string into a [Type]
    fn parse_type(expr: &str) -> Type {
        let parsed = TypeExpr::parse(expr).unwrap();
        Type::try_from(parsed).unwrap()
    }

    /// Parse a string into a [Type], returning the result
    fn parse_type_result(expr: &str) -> Result<Type, ExtensionTypeError> {
        let parsed = TypeExpr::parse(expr).unwrap();
        Type::try_from(parsed)
    }

    /// Parse a string into a builtin [Type], with no unresolved
    /// extension references
    fn parse_simple(s: &str) -> Type {
        let parsed = TypeExpr::parse(s).unwrap();

        let mut refs = Vec::new();
        parsed.visit_references(&mut |name| refs.push(name.to_string()));
        assert!(refs.is_empty(), "{s} should not add an extension reference");

        Type::try_from(parsed).unwrap()
    }

    #[test]
    fn test_numeric_parameter_expressions() {
        let parsed = parse_type("decimal<P1 + P2, S>");
        let TypeKind::Decimal { precision, scale } = &parsed.kind else {
            panic!("expected decimal");
        };
        assert_eq!(
            precision,
            &IntegerParameter::Expression(NumericExpression::parse("P1 + P2").unwrap())
        );
        assert_eq!(scale, &IntegerParameter::Variable("S".into()));
    }

    #[test]
    fn test_numeric_expression_display_and_concreteness() {
        for source in [
            "varchar<L + 1>",
            "decimal<P1 + P2, S>",
            "list<decimal<P + 1, S>>",
            "func<varchar<L + 1> -> decimal<P, S + 1>>",
            "u!custom<P + 1>",
            "varchar<1 + 2>",
        ] {
            let parsed = parse_type(source);
            assert!(!parsed.is_concrete(), "{source}");
            assert_eq!(parse_type(&parsed.to_string()), parsed, "{source}");
        }
    }

    #[test]
    fn test_numeric_expressions_are_not_type_arguments() {
        for source in ["list<P + 1>", "map<i32, P + 1>", "func<P + 1 -> i32>"] {
            assert!(
                matches!(
                    parse_type_result(source),
                    Err(ExtensionTypeError::InvalidParameterKind { .. })
                ),
                "{source}"
            );
        }
    }

    #[test]
    fn test_numeric_expressions_are_not_concrete_structure_fields() {
        let expression = TypeExpr::parse("varchar<L + 1>").unwrap();
        assert!(matches!(
            expect_concrete_type(expression),
            Err(ExtensionTypeError::ExpectedConcreteType { .. })
        ));
    }

    #[test]
    fn test_literal_and_symbolic_lengths_share_a_type_kind() {
        for (length, expected, is_concrete) in [
            (IntegerParameter::Literal(20), "varchar<20>", true),
            (IntegerParameter::Variable("L".into()), "varchar<L>", false),
        ] {
            let type_ = Type::new(
                TypeKind::VarChar {
                    length: length.clone(),
                },
                false,
            );
            assert_eq!(type_.to_string(), expected);
            assert_eq!(type_.is_concrete(), is_concrete);

            let extension = Type::extension_with_params(
                "sized".into(),
                vec![TypeParameter::Integer(length)],
                false,
            );
            assert_eq!(extension.is_concrete(), is_concrete);
        }
    }

    #[test]
    fn test_concreteness_is_recursive() {
        let variable = parse_type("any1?");
        assert_eq!(variable.to_string(), "any1?");
        for field in [
            parse_type("i32"),
            variable,
            Type::new(
                TypeKind::VarChar {
                    length: IntegerParameter::Variable("L".into()),
                },
                false,
            ),
        ] {
            let expected = field.is_concrete();
            for nested in [
                Type::list(field.clone(), false),
                Type::map(parse_type("string"), field.clone(), false),
                Type::r#struct(vec![field.clone()], false),
                Type::named_struct(IndexMap::from([("field".into(), field.clone())]), false),
                Type::extension_with_params(
                    "wrapper".into(),
                    vec![TypeParameter::Type(field)],
                    false,
                ),
            ] {
                assert_eq!(
                    nested.is_concrete(),
                    expected,
                    "unexpected concreteness for {nested}"
                );
            }
        }
    }

    #[test]
    fn test_concreteness_does_not_replace_value_validation() {
        let type_ = Type::new(
            TypeKind::Decimal {
                precision: IntegerParameter::Literal(10),
                scale: IntegerParameter::Literal(20),
            },
            false,
        );
        assert!(type_.is_concrete());
        assert!(matches!(
            parse_type_result(&type_.to_string()),
            Err(ExtensionTypeError::InvalidParameterRange { .. })
        ));
    }

    #[test]
    fn test_structure_parsing_still_requires_concrete_types() {
        for raw in [
            RawType::String("any1?".into()),
            RawType::Object(IndexMap::from([(
                "field".into(),
                Value::String("any1?".into()),
            )])),
        ] {
            assert!(matches!(
                raw.parse(&mut TypeContext::default()),
                Err(ExtensionTypeError::InvalidAnyTypeVariable {
                    id: 1,
                    nullability: true
                })
            ));
        }
    }

    #[test]
    fn test_named_numeric_parameters_remain_unsupported_in_type_structures() {
        for raw in [
            RawType::String("varchar<L>".into()),
            RawType::Object(IndexMap::from([(
                "field".into(),
                Value::String("decimal<P, S>".into()),
            )])),
        ] {
            assert!(matches!(
                raw.parse(&mut TypeContext::default()),
                Err(ExtensionTypeError::ExpectedConcreteType { .. })
            ));
        }
    }

    #[test]
    fn test_extension_integer_parameters_preserve_i64_range() {
        let input = format!("u!wide<{}>", i64::MAX);
        let parsed = parse_type(&input);
        assert!(parsed.is_concrete());
        assert_eq!(parsed.to_string(), input);
        assert!(matches!(
            parse_type_result(&format!("varchar<{}>", i64::MAX)),
            Err(ExtensionTypeError::InvalidParameterValue { .. })
        ));
    }

    /// Create a type parameter from a type expression string
    fn type_param(expr: &str) -> TypeParameter {
        TypeParameter::Type(parse_type(expr))
    }

    /// Create an extension type
    fn extension(name: &str, parameters: Vec<TypeParameter>, nullable: bool) -> Type {
        Type::extension_with_params(name.to_string(), parameters, nullable)
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
        RawType::Object(
            fields
                .iter()
                .map(|(name, ty)| ((*name).into(), serde_json::Value::String((*ty).into())))
                .collect(),
        )
    }

    #[test]
    fn test_builtin_scalar_parsing() {
        let cases = vec![
            ("bool", Some(TypeKind::Boolean)),
            ("i8", Some(TypeKind::I8)),
            ("i16", Some(TypeKind::I16)),
            ("i32", Some(TypeKind::I32)),
            ("i64", Some(TypeKind::I64)),
            ("fp32", Some(TypeKind::Fp32)),
            ("fp64", Some(TypeKind::Fp64)),
            ("STRING", Some(TypeKind::String)),
            ("binary", Some(TypeKind::Binary)),
            ("uuid", Some(TypeKind::Uuid)),
            ("date", Some(TypeKind::Date)),
            ("interval_year", Some(TypeKind::IntervalYear)),
            ("invalid", None),
        ];

        for (input, expected) in cases {
            let result = parse_builtin(input, input.to_ascii_lowercase().as_str(), &[]).unwrap();
            match expected {
                Some(expected_type) => {
                    assert_eq!(
                        result,
                        Some(expected_type),
                        "expected builtin type for {input}"
                    );
                }
                None => {
                    assert!(result.is_none(), "expected parsing {input} to fail");
                }
            }
        }
    }

    #[test]
    fn test_nested_symbolic_types() {
        assert_eq!(
            parse_type("List<any>"),
            Type::list(required_type(TypeKind::Any), false)
        );
        assert_eq!(
            parse_type("Map<string, any?>"),
            Type::map(parse_type("string"), Type::new(TypeKind::Any, true), false)
        );
        assert_eq!(
            parse_type("List?<DECIMAL<P, S>>"),
            Type::list(parse_type("decimal<P, S>"), true)
        );
        assert_eq!(
            parse_type("Map<string, precision_timestamp<P>>"),
            Type::map(
                parse_type("string"),
                parse_type("precision_timestamp<P>"),
                false
            )
        );
        assert_eq!(
            parse_type("Struct?<any1, List<Map<string, VARCHAR<L>>>>"),
            Type::r#struct(
                vec![
                    parse_type("any1"),
                    parse_type("List<Map<string, VARCHAR<L>>>")
                ],
                true
            )
        );
        assert_eq!(parse_type("any").to_string(), "any");
        assert_eq!(parse_type("Any?").to_string(), "any?");
    }

    #[test]
    fn test_nested_type_concreteness_follows_elements() {
        assert!(parse_type("list<decimal<10, 2>>").is_concrete());
        assert!(!parse_type("list<any>").is_concrete());
    }

    #[test]
    fn test_function_types() {
        for (input, expected) in [
            ("func<i32 -> i64>", "func<i32 -> i64>"),
            ("FUNC?<any1 -> any2>", "func?<any1 -> any2>"),
            (
                "func<(i32, list<any1>) -> i64?>",
                "func<(i32, list<any1>) -> i64?>",
            ),
            (
                "func<i32 -> func<i64 -> boolean?>>",
                "func<i32 -> func<i64 -> bool?>>",
            ),
        ] {
            let ty = parse_type(input);
            assert_eq!(ty.to_string(), expected, "unexpected spelling for {input}");
            assert_eq!(
                parse_type(&ty.to_string()),
                ty,
                "round trip failed for {input}"
            );
        }

        assert!(parse_type("func<i32 -> i64>").is_concrete());
        assert!(!parse_type("func<any1 -> i64>").is_concrete());
        assert!(!parse_type("func<i32 -> any1>").is_concrete());
    }

    #[test]
    fn test_function_type_validation() {
        for input in ["func<5 -> i64>", "func<i32 -> missing>"] {
            assert!(
                parse_type_result(input).is_err(),
                "{input} should not be accepted as a signature type"
            );
        }
    }

    #[test]
    fn test_symbolic_user_defined_type_parameters() {
        let ty = parse_type("u!Box?<List<decimal<P, S>>, 5, N>");
        assert_eq!(
            ty,
            Type::extension_with_params(
                "Box".to_string(),
                vec![
                    TypeParameter::Type(parse_type("List<decimal<P, S>>")),
                    TypeParameter::Integer(IntegerParameter::Literal(5)),
                    TypeParameter::Name("N".to_string()),
                ],
                true,
            )
        );
        assert!(!ty.is_concrete());
        assert!(!parse_type("u!Box<5, N>").is_concrete());
        assert_eq!(ty.to_string(), "u!Box?<list<decimal<P, S>>, 5, N>");
    }

    #[test]
    fn test_invalid_nested_signature_types() {
        for input in [
            "List<any1, any2>",
            "Map<any1>",
            "Struct<1, any1>",
            "List<missing_type?>",
            "List<decimal<any1>>",
            "List<i32<P>>",
        ] {
            assert!(
                Type::try_from(TypeExpr::parse(input).unwrap()).is_err(),
                "{input} should not be accepted as a signature type"
            );
        }
    }

    #[test]
    fn test_named_integer_parameters() {
        let parameter = IntegerParameter::Variable("N".to_string());
        for (name, expected) in [
            (
                "fixedchar",
                TypeKind::FixedChar {
                    length: parameter.clone(),
                },
            ),
            (
                "varchar",
                TypeKind::VarChar {
                    length: parameter.clone(),
                },
            ),
            (
                "fixedbinary",
                TypeKind::FixedBinary {
                    length: parameter.clone(),
                },
            ),
            (
                "precision_time",
                TypeKind::PrecisionTime {
                    precision: parameter.clone(),
                },
            ),
            (
                "precision_timestamp",
                TypeKind::PrecisionTimestamp {
                    precision: parameter.clone(),
                },
            ),
            (
                "precision_timestamp_tz",
                TypeKind::PrecisionTimestampTz {
                    precision: parameter.clone(),
                },
            ),
            (
                "interval_day",
                TypeKind::IntervalDay {
                    precision: parameter.clone(),
                },
            ),
            (
                "interval_compound",
                TypeKind::IntervalCompound {
                    precision: parameter,
                },
            ),
        ] {
            let input = format!("{}?<N>", name.to_ascii_uppercase());
            let expression = Type::try_from(TypeExpr::parse(&input).unwrap()).unwrap();
            assert_eq!(
                expression,
                Type::new(expected, true),
                "unexpected type for {input}"
            );
        }

        for (input, precision, scale) in [
            (
                "decimal<P, S>",
                IntegerParameter::Variable("P".into()),
                IntegerParameter::Variable("S".into()),
            ),
            (
                "decimal<10, S>",
                IntegerParameter::Literal(10),
                IntegerParameter::Variable("S".into()),
            ),
            (
                "decimal<P, 2>",
                IntegerParameter::Variable("P".into()),
                IntegerParameter::Literal(2),
            ),
        ] {
            assert_eq!(
                Type::try_from(TypeExpr::parse(input).unwrap()).unwrap(),
                Type::new(TypeKind::Decimal { precision, scale }, false)
            );
        }
    }

    #[test]
    fn test_named_numeric_parameters_preserve_builtin_validation() {
        for input in [
            "i32<P>",
            "varchar<L, 2>",
            "decimal<any1, S>",
            "decimal<P?, S>",
            "decimal<P<1>, S>",
            "decimal<u!P, S>",
            "decimal<39, S>",
            "decimal<P, 39>",
            "precisiontime<P>",
            "missing<P>",
        ] {
            assert!(
                Type::try_from(TypeExpr::parse(input).unwrap()).is_err(),
                "{input} should not be accepted as a signature type"
            );
        }
    }

    #[test]
    fn test_parameterized_builtin_types() {
        let cases = vec![
            (
                "precision_time<2>",
                required_type(TypeKind::PrecisionTime {
                    precision: IntegerParameter::Literal(2),
                }),
            ),
            (
                "precision_timestamp<1>",
                required_type(TypeKind::PrecisionTimestamp {
                    precision: IntegerParameter::Literal(1),
                }),
            ),
            (
                "precision_timestamp_tz<5>",
                required_type(TypeKind::PrecisionTimestampTz {
                    precision: IntegerParameter::Literal(5),
                }),
            ),
            (
                "DECIMAL<10,2>",
                required_type(TypeKind::Decimal {
                    precision: IntegerParameter::Literal(10),
                    scale: IntegerParameter::Literal(2),
                }),
            ),
            (
                "fixedchar<12>",
                required_type(TypeKind::FixedChar {
                    length: IntegerParameter::Literal(12),
                }),
            ),
            (
                "VarChar<42>",
                required_type(TypeKind::VarChar {
                    length: IntegerParameter::Literal(42),
                }),
            ),
            (
                "fixedbinary<8>",
                required_type(TypeKind::FixedBinary {
                    length: IntegerParameter::Literal(8),
                }),
            ),
            (
                "interval_day<7>",
                required_type(TypeKind::IntervalDay {
                    precision: IntegerParameter::Literal(7),
                }),
            ),
            (
                "interval_compound<6>",
                required_type(TypeKind::IntervalCompound {
                    precision: IntegerParameter::Literal(6),
                }),
            ),
        ];

        for (expr, expected) in cases {
            let found = parse_simple(expr);
            assert_eq!(found, expected, "unexpected type for {expr}");
        }
    }

    #[test]
    fn test_parameterized_builtin_range_errors() {
        use ExtensionTypeError::InvalidParameterRange;

        let cases = vec![
            ("precision_time<13>", "precision_time", 0, 13, 0..=12),
            ("precision_time<-1>", "precision_time", 0, -1, 0..=12),
            (
                "precision_timestamp<13>",
                "precision_timestamp",
                0,
                13,
                0..=12,
            ),
            (
                "precision_timestamp<-1>",
                "precision_timestamp",
                0,
                -1,
                0..=12,
            ),
            (
                "precision_timestamp_tz<20>",
                "precision_timestamp_tz",
                0,
                20,
                0..=12,
            ),
            ("interval_day<10>", "interval_day", 0, 10, 0..=9),
            ("DECIMAL<39,0>", "DECIMAL", 0, 39, 1..=38),
            ("DECIMAL<0,0>", "DECIMAL", 0, 0, 1..=38),
            ("DECIMAL<10,-1>", "DECIMAL", 1, -1, 0..=10),
            ("DECIMAL<10,12>", "DECIMAL", 1, 12, 0..=10),
        ];

        for (expr, expected_type, expected_index, expected_value, expected_range) in cases {
            match parse_type_result(expr) {
                Ok(value) => panic!("expected error parsing {expr}, got {value:?}"),
                Err(InvalidParameterRange {
                    type_name,
                    index,
                    value,
                    expected,
                }) => {
                    assert_eq!(type_name, expected_type, "unexpected type for {expr}");
                    assert_eq!(index, expected_index, "unexpected index for {expr}");
                    assert_eq!(
                        value,
                        i64::from(expected_value),
                        "unexpected value for {expr}"
                    );
                    assert_eq!(expected, expected_range, "unexpected range for {expr}");
                }
                Err(other) => panic!("expected InvalidParameterRange for {expr}, got {other:?}"),
            }
        }
    }

    #[test]
    fn test_container_types() {
        let cases = vec![
            (
                "List<i32>",
                Type::list(Type::new(TypeKind::I32, false), false),
            ),
            (
                "List<fp64?>",
                Type::list(Type::new(TypeKind::Fp64, true), false),
            ),
            (
                "Map?<i64, string?>",
                Type::map(
                    Type::new(TypeKind::I64, false),
                    Type::new(TypeKind::String, true),
                    true,
                ),
            ),
            (
                "Struct?<i8, string?>",
                Type::r#struct(
                    vec![
                        Type::new(TypeKind::I8, false),
                        Type::new(TypeKind::String, true),
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
                    vec![
                        type_param("List<i32>"),
                        TypeParameter::Integer(IntegerParameter::Literal(10)),
                    ],
                    false,
                ),
            ),
            (
                "u!Geo?<List<i32?>>",
                extension("Geo", vec![type_param("List<i32?>")], true),
            ),
            (
                "u!Custom<string?, bool>",
                extension(
                    "Custom",
                    vec![
                        type_param("string?"),
                        TypeParameter::Type(Type::new(TypeKind::Boolean, false)),
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
    fn test_builtin_type_display_lowercase() {
        for input in [
            "I32",
            "FIXEDCHAR<5>",
            "VARCHAR<10>",
            "FIXEDBINARY<8>",
            "DeCiMaL<10, 2>",
            "PRECISION_TIME<3>",
            "PRECISION_TIMESTAMP<3>",
            "PRECISION_TIMESTAMP_TZ<3>",
            "INTERVAL_DAY<3>",
            "INTERVAL_COMPOUND<3>",
        ] {
            let concrete = parse_type(input);
            assert_eq!(parse_type(&input.to_ascii_lowercase()), concrete);

            let output = concrete.to_string();
            assert_eq!(output, output.to_ascii_lowercase(), "display for {input}");
        }
    }

    #[test]
    fn test_precision_types_require_underscores() {
        for name in [
            "precisiontime",
            "precisiontimestamp",
            "precisiontimestamptz",
        ] {
            for spelling in [name.to_string(), name.to_ascii_uppercase()] {
                assert!(!TypeKind::is_name(&spelling));
                assert_eq!(
                    parse_type_result(&format!("{spelling}<3>")),
                    Err(ExtensionTypeError::UnknownTypeName { name: spelling })
                );
            }
        }
    }

    #[test]
    fn test_precision_type_round_trip_display() {
        for (input, expected) in [
            ("PRECISION_TIME<3>", "precision_time<3>"),
            ("PRECISION_TIMESTAMP<3>", "precision_timestamp<3>"),
            ("PRECISION_TIMESTAMP_TZ<3>", "precision_timestamp_tz<3>"),
        ] {
            let concrete = parse_type(input);
            let output = concrete.to_string();
            assert_eq!(output, expected);
            assert_eq!(parse_type(&output), concrete);

            let raw: RawType = concrete.clone().into();
            let mut ctx = TypeContext::default();
            assert_eq!(raw.parse(&mut ctx).unwrap(), concrete);
        }
    }

    #[test]
    fn test_type_round_trip_display() {
        // (example, canonical form)
        let cases = vec![
            ("i32", "i32"),
            ("I64?", "i64?"),
            ("list<string>", "list<string>"),
            ("List<STRING?>", "list<string?>"),
            ("map<i32, list<string>>", "map<i32, list<string>>"),
            ("struct<i8, string?>", "struct<i8, string?>"),
            (
                "Struct<List<i32>, Map<string, list<i64?>>>",
                "struct<list<i32>, map<string, list<i64?>>>",
            ),
            (
                "Map<List<I32?>, Struct<string, list<i64?>>>",
                "map<list<i32?>, struct<string, list<i64?>>>",
            ),
            ("List?<i32>", "list?<i32>"),
            ("Map?<string, List?<i32?>>", "map?<string, list?<i32?>>"),
            ("Struct?<i32, string?>", "struct?<i32, string?>"),
            ("struct<>", "struct<>"),
            ("FIXEDCHAR?<5>", "fixedchar?<5>"),
            ("VARCHAR?<10>", "varchar?<10>"),
            ("FIXEDBINARY?<8>", "fixedbinary?<8>"),
            ("DeCiMaL?<10,2>", "decimal?<10, 2>"),
            ("precision_time?<3>", "precision_time?<3>"),
            ("precision_timestamp?<3>", "precision_timestamp?<3>"),
            ("precision_timestamp_tz?<3>", "precision_timestamp_tz?<3>"),
            ("InTeRvAl_DaY?<3>", "interval_day?<3>"),
            ("INTERVAL_COMPOUND?<3>", "interval_compound?<3>"),
            ("u!geometry?", "u!geometry?"),
            ("u!CustomType?<i32>", "u!CustomType?<i32>"),
            ("u!custom<i32>", "u!custom<i32>"),
            ("u!custom?<i32, 5>", "u!custom?<i32, 5>"),
            ("list?<u!geometry?>", "list?<u!geometry?>"),
        ];

        for (input, expected) in cases {
            let parsed = TypeExpr::parse(input).unwrap();
            let concrete = Type::try_from(parsed).unwrap();
            let actual = concrete.to_string();

            assert_eq!(actual, expected, "unexpected display for {input}");
            assert_eq!(
                parse_type(&actual),
                concrete,
                "round trip failed for {input}"
            );

            let raw: RawType = concrete.clone().into();
            let mut ctx = TypeContext::default();
            assert_eq!(
                raw.parse(&mut ctx).unwrap(),
                concrete,
                "raw type round trip failed for {input}"
            );
        }
    }

    /// Test that named struct field order preserves the structure order when
    /// round-tripping through RawType (Substrait #915).
    #[test]
    fn test_named_struct_field_order_stability() -> Result<(), ExtensionTypeError> {
        let raw = RawType::Object(
            [("beta", "i32"), ("alpha", "string?")]
                .into_iter()
                .map(|(name, ty)| (name.to_string(), Value::String(ty.to_string())))
                .collect(),
        );
        let mut ctx = TypeContext::default();
        let concrete = Parse::parse(raw, &mut ctx)?;

        let round_tripped: RawType = concrete.into();
        match round_tripped {
            RawType::Object(result_map) => {
                let keys: Vec<_> = result_map.keys().collect();
                assert_eq!(
                    keys,
                    vec!["beta", "alpha"],
                    "field order should be preserved"
                );
            }
            other => panic!("expected Object, got {other:?}"),
        }

        Ok(())
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
                Ok((Some(1), Some(10))),
            ),
            (
                "fractional_min",
                simple_extensions::TypeParamDefsItem {
                    name: Some("K".to_string()),
                    description: None,
                    type_: simple_extensions::TypeParamDefsItemType::Integer,
                    min: Some(1.5),
                    max: None,
                    options: None,
                    optional: None,
                },
                Err(TypeParamError::InvalidIntegerBounds {
                    min: Some(1.5),
                    max: None,
                }),
            ),
            (
                "fractional_max",
                simple_extensions::TypeParamDefsItem {
                    name: Some("K".to_string()),
                    description: None,
                    type_: simple_extensions::TypeParamDefsItemType::Integer,
                    min: None,
                    max: Some(9.9),
                    options: None,
                    optional: None,
                },
                Err(TypeParamError::InvalidIntegerBounds {
                    min: None,
                    max: Some(9.9),
                }),
            ),
        ];

        for (label, item, expected) in cases {
            match (TypeParam::try_from(item), expected) {
                (Ok(tp), Ok((expected_min, expected_max))) => match tp.param_type {
                    ParameterConstraint::Integer { min, max } => {
                        assert_eq!(min, expected_min, "min mismatch for {label}");
                        assert_eq!(max, expected_max, "max mismatch for {label}");
                    }
                    _ => panic!("expected integer param type for {label}"),
                },
                (Err(actual_err), Err(expected_err)) => {
                    assert_eq!(actual_err, expected_err, "unexpected error for {label}");
                }
                (result, expected) => {
                    panic!("unexpected result for {label}: got {result:?}, expected {expected:?}")
                }
            }
        }
    }

    #[test]
    fn test_custom_type_round_trip() -> Result<(), ExtensionTypeError> {
        let fields = IndexMap::from_iter([
            ("x".to_string(), Type::new(TypeKind::Fp64, false)),
            ("y".to_string(), Type::new(TypeKind::Fp64, false)),
        ]);

        let cases = vec![
            CustomType::new(
                "AliasType".to_string(),
                vec![],
                Some(Type::new(TypeKind::I32, false)),
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
                Some(Type::named_struct(fields, false)),
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
            ("9bad", false),
            ("bad-name", false),
            ("bad.name", false),
            ("GoodName", true),
            ("also_good", true),
            ("_underscore", true),
            ("$dollar", true),
            ("CamelCase123", true),
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
                RawType::String("i32".to_string()),
                Type::new(TypeKind::I32, false),
            ),
            (
                "named_struct",
                raw_named_struct(&[("field1", "fp64"), ("field2", "i32?")]),
                Type::named_struct(
                    IndexMap::from_iter([
                        ("field1".to_string(), Type::new(TypeKind::Fp64, false)),
                        ("field2".to_string(), Type::new(TypeKind::I32, true)),
                    ]),
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
                    deprecated: None,
                    name: "Alias".to_string(),
                    description: Some("Alias type".to_string()),
                    metadata: Default::default(),
                    parameters: None,
                    structure: Some(RawType::String("BINARY".to_string())),
                    variadic: None,
                },
                "Alias",
                Some("Alias type"),
                Some(Type::new(TypeKind::Binary, false)),
            ),
            (
                "named_struct",
                simple_extensions::SimpleExtensionsTypesItem {
                    deprecated: None,
                    name: "Point".to_string(),
                    description: Some("A 2D point".to_string()),
                    metadata: Default::default(),
                    parameters: None,
                    structure: Some(raw_named_struct(&[("x", "fp64"), ("y", "fp64?")])),
                    variadic: None,
                },
                "Point",
                Some("A 2D point"),
                Some(Type::named_struct(
                    IndexMap::from_iter([
                        ("x".to_string(), Type::new(TypeKind::Fp64, false)),
                        ("y".to_string(), Type::new(TypeKind::Fp64, true)),
                    ]),
                    false,
                )),
            ),
            (
                "no_structure",
                simple_extensions::SimpleExtensionsTypesItem {
                    deprecated: None,
                    name: "Opaque".to_string(),
                    description: None,
                    metadata: Default::default(),
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
}
