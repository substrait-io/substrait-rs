// SPDX-License-Identifier: Apache-2.0

//! Concrete type system for function validation in the registry.
//!
//! This module provides a clean, type-safe wrapper around Substrait extension types,
//! separating function signature patterns from concrete argument types.

use crate::parse::Parse;
use crate::registry::context::ExtensionContext;
use crate::text::simple_extensions::SimpleExtensionsTypesItem;
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

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
/// A validated extension type definition
#[derive(Clone, Debug)]
pub struct ExtensionType<'a> {
    /// The URI of the extension defining this type
    pub uri: &'a Url,
    item: &'a SimpleExtensionsTypesItem,
}

impl<'a> ExtensionType<'a> {
    /// Create a new ExtensionType wrapper from already-validated data
    pub(crate) fn new_unchecked(uri: &'a Url, item: &'a SimpleExtensionsTypesItem) -> Self {
        Self { uri, item }
    }

    /// Get the name of this extension type
    pub fn name(&self) -> &str {
        &self.item.name
    }
}

impl<'a> From<ExtensionType<'a>> for &'a SimpleExtensionsTypesItem {
    fn from(ext_type: ExtensionType<'a>) -> Self {
        ext_type.item
    }
}

impl PartialEq for ExtensionType<'_> {
    fn eq(&self, other: &Self) -> bool {
        // There should only be one type of a given name per file
        self.uri == other.uri && self.item.name == other.item.name
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid type name: {0}")]
/// Error for invalid type names in extension definitions
pub struct InvalidTypeName(String);

impl<'a> Parse<ExtensionContext<'a>> for &'a SimpleExtensionsTypesItem {
    type Parsed = ExtensionType<'a>;
    // TODO: Not all names are valid for types, we should validate that
    type Error = InvalidTypeName;

    fn parse(self, ctx: &mut ExtensionContext<'a>) -> Result<Self::Parsed, Self::Error> {
        ctx.add_type(self);
        Ok(ExtensionType {
            uri: ctx.uri,
            item: &self,
        })
    }
}

/// Represents a known, specific type, either builtin or extension
#[derive(Clone, Debug, PartialEq)]
pub enum KnownType<'a> {
    /// Built-in primitive types
    Builtin(BuiltinType),
    /// Custom types defined in extension YAML files
    Extension(ExtensionType<'a>),
}

/// A concrete type, fully specified with nullability and parameters
#[derive(Clone, Debug, PartialEq)]
pub struct ConcreteType<'a> {
    /// Base type, can be builtin or extension
    pub base: KnownType<'a>,
    /// Is the overall type nullable?
    pub nullable: bool,
    // TODO: Add non-type parameters (e.g. integers, enum, etc.)
    /// Parameters for the type, if there are any
    pub parameters: Vec<ConcreteType<'a>>,
}

impl<'a> ConcreteType<'a> {
    /// Create a concrete type from a builtin type
    pub fn builtin(builtin_type: BuiltinType, nullable: bool) -> ConcreteType<'static> {
        ConcreteType {
            base: KnownType::Builtin(builtin_type),
            nullable,
            parameters: Vec::new(),
        }
    }

    /// Create a concrete type from an extension type
    pub fn extension(t: ExtensionType<'a>, nullable: bool) -> Self {
        Self {
            base: KnownType::Extension(t),
            nullable,
            parameters: Vec::new(),
        }
    }

    /// Create a parameterized concrete type
    pub fn parameterized(
        base: KnownType<'a>,
        nullable: bool,
        parameters: Vec<ConcreteType<'a>>,
    ) -> Self {
        Self {
            base,
            nullable,
            parameters,
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

/// A pattern for function arguments that can match concrete types or type variables
#[derive(Clone, Debug, PartialEq)]
pub enum ArgumentPattern<'a> {
    /// Type variable like any1, any2, etc.
    TypeVariable(u32),
    /// Concrete type pattern
    Concrete(ConcreteType<'a>),
}

/// Result of matching an argument pattern against a concrete type
#[derive(Clone, Debug, PartialEq)]
pub enum Match<'a> {
    /// Pattern matched exactly (for concrete patterns)
    Concrete,
    /// Type variable bound to concrete type
    Variable(u32, ConcreteType<'a>),
    /// Match failed
    Fail,
}

impl<'a> ArgumentPattern<'a> {
    /// Check if this pattern matches the given concrete type
    pub fn matches(&self, concrete: &ConcreteType<'a>) -> Match<'a> {
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

/// Type variable bindings from matching function arguments
#[derive(Debug, Clone, PartialEq)]
pub struct TypeBindings<'a> {
    /// Map of type variable IDs (e.g. 1 for 'any1') to their concrete types
    pub vars: HashMap<u32, ConcreteType<'a>>,
}

impl<'a> TypeBindings<'a> {
    /// Create type bindings by matching argument patterns against concrete arguments
    pub fn new(patterns: &[ArgumentPattern<'a>], args: &[ConcreteType<'a>]) -> Option<Self> {
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
    pub fn get(&self, var_id: u32) -> Option<&ConcreteType<'a>> {
        self.vars.get(&var_id)
    }
}
