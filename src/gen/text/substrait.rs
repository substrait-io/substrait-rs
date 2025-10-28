
#[doc = "Generated types for `http://substrait.io/schemas/dialect`"]
pub mod dialect {
    /// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`AggregateRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "AGGREGATE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AggregateRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&AggregateRelation> for AggregateRelation {
    fn from(value: &AggregateRelation) -> Self {
        value.clone()
    }
}
impl AggregateRelation {
    pub fn builder() -> builder::AggregateRelation {
        Default::default()
    }
}
///`CastExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "CAST"
///    },
///    "failure_options": {
///      "description": "The permissible failure options for cast operations.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "RETURN_NULL",
///          "THROW_EXCEPTION"
///        ]
///      },
///      "uniqueItems": true
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CastExpression {
    pub expression: ::serde_json::Value,
    ///The permissible failure options for cast operations.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub failure_options: ::std::option::Option<Vec<CastExpressionFailureOptionsItem>>,
}
impl ::std::convert::From<&CastExpression> for CastExpression {
    fn from(value: &CastExpression) -> Self {
        value.clone()
    }
}
impl CastExpression {
    pub fn builder() -> builder::CastExpression {
        Default::default()
    }
}
///`CastExpressionFailureOptionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "RETURN_NULL",
///    "THROW_EXCEPTION"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum CastExpressionFailureOptionsItem {
    #[serde(rename = "RETURN_NULL")]
    ReturnNull,
    #[serde(rename = "THROW_EXCEPTION")]
    ThrowException,
}
impl ::std::convert::From<&Self> for CastExpressionFailureOptionsItem {
    fn from(value: &CastExpressionFailureOptionsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for CastExpressionFailureOptionsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ReturnNull => f.write_str("RETURN_NULL"),
            Self::ThrowException => f.write_str("THROW_EXCEPTION"),
        }
    }
}
impl ::std::str::FromStr for CastExpressionFailureOptionsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "RETURN_NULL" => Ok(Self::ReturnNull),
            "THROW_EXCEPTION" => Ok(Self::ThrowException),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for CastExpressionFailureOptionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for CastExpressionFailureOptionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for CastExpressionFailureOptionsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ConsistentPartitionWindowRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "CONSISTENT_PARTITION_WINDOW"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConsistentPartitionWindowRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ConsistentPartitionWindowRelation>
for ConsistentPartitionWindowRelation {
    fn from(value: &ConsistentPartitionWindowRelation) -> Self {
        value.clone()
    }
}
impl ConsistentPartitionWindowRelation {
    pub fn builder() -> builder::ConsistentPartitionWindowRelation {
        Default::default()
    }
}
///`CrossRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "CROSS"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CrossRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&CrossRelation> for CrossRelation {
    fn from(value: &CrossRelation) -> Self {
        value.clone()
    }
}
impl CrossRelation {
    pub fn builder() -> builder::CrossRelation {
        Default::default()
    }
}
///`DdlRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "DDL"
///    },
///    "write_types": {
///      "description": "The list of valid types a ddl relation can operate on.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "NAMED_OBJECT",
///          "EXTENSION_OBJECT"
///        ]
///      },
///      "uniqueItems": true
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DdlRelation {
    pub relation: ::serde_json::Value,
    ///The list of valid types a ddl relation can operate on.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_types: ::std::option::Option<Vec<DdlRelationWriteTypesItem>>,
}
impl ::std::convert::From<&DdlRelation> for DdlRelation {
    fn from(value: &DdlRelation) -> Self {
        value.clone()
    }
}
impl DdlRelation {
    pub fn builder() -> builder::DdlRelation {
        Default::default()
    }
}
///`DdlRelationWriteTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "NAMED_OBJECT",
///    "EXTENSION_OBJECT"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DdlRelationWriteTypesItem {
    #[serde(rename = "NAMED_OBJECT")]
    NamedObject,
    #[serde(rename = "EXTENSION_OBJECT")]
    ExtensionObject,
}
impl ::std::convert::From<&Self> for DdlRelationWriteTypesItem {
    fn from(value: &DdlRelationWriteTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DdlRelationWriteTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NamedObject => f.write_str("NAMED_OBJECT"),
            Self::ExtensionObject => f.write_str("EXTENSION_OBJECT"),
        }
    }
}
impl ::std::str::FromStr for DdlRelationWriteTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NAMED_OBJECT" => Ok(Self::NamedObject),
            "EXTENSION_OBJECT" => Ok(Self::ExtensionObject),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DdlRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for DdlRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DdlRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Dialect`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "http://substrait.io/schemas/dialect",
///  "title": "Dialect",
///  "type": "object",
///  "properties": {
///    "dependencies": {
///      "description": "A map of extension files depended on by this dialect. Keys are aliases for extensions, which are used within the dialect. Values are URNs.\n",
///      "type": "object",
///      "minProperties": 1,
///      "additionalProperties": {
///        "type": "string"
///      }
///    },
///    "name": {
///      "description": "Optional name of the dialect (e.g., \"MySQL\", \"PostgreSQL\", \"Spark\").",
///      "type": "string"
///    },
///    "supported_aggregate_functions": {
///      "description": "Functions that aggregate multiple rows into a single result.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/dialect_function"
///      }
///    },
///    "supported_expressions": {
///      "description": "Expressions that are supported by this dialect.  If not present or empty, all expressions are supported.\n",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "enum": [
///              "LITERAL",
///              "SELECTION",
///              "SCALAR_FUNCTION",
///              "WINDOW_FUNCTION",
///              "IF_THEN",
///              "SWITCH",
///              "SINGULAR_OR_LIST",
///              "MULTI_OR_LIST",
///              "CAST",
///              "SUBQUERY",
///              "NESTED",
///              "DYNAMIC_PARAMETER"
///            ]
///          },
///          {
///            "$ref": "#/definitions/literal_expression"
///          },
///          {
///            "$ref": "#/definitions/selection_expression"
///          },
///          {
///            "$ref": "#/definitions/scalar_function_expression"
///          },
///          {
///            "$ref": "#/definitions/window_function_expression"
///          },
///          {
///            "$ref": "#/definitions/if_then_expression"
///          },
///          {
///            "$ref": "#/definitions/switch_expression"
///          },
///          {
///            "$ref": "#/definitions/singular_or_list_expression"
///          },
///          {
///            "$ref": "#/definitions/multi_or_list_expression"
///          },
///          {
///            "$ref": "#/definitions/cast_expression"
///          },
///          {
///            "$ref": "#/definitions/subquery_expression"
///          },
///          {
///            "$ref": "#/definitions/nested_expression"
///          },
///          {
///            "$ref": "#/definitions/dynamic_parameter_expression"
///          }
///        ]
///      },
///      "uniqueItems": true
///    },
///    "supported_relations": {
///      "description": "Relations that are supported by this dialect.  If not present or empty, all relations are supported.\n",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "enum": [
///              "READ",
///              "FILTER",
///              "FETCH",
///              "AGGREGATE",
///              "SORT",
///              "JOIN",
///              "PROJECT",
///              "SET",
///              "CROSS",
///              "REFERENCE",
///              "WRITE",
///              "DDL",
///              "UPDATE",
///              "HASH_JOIN",
///              "MERGE_JOIN",
///              "NESTED_LOOP_JOIN",
///              "CONSISTENT_PARTITION_WINDOW",
///              "EXCHANGE",
///              "EXPAND"
///            ]
///          },
///          {
///            "$ref": "#/definitions/read_relation"
///          },
///          {
///            "$ref": "#/definitions/filter_relation"
///          },
///          {
///            "$ref": "#/definitions/fetch_relation"
///          },
///          {
///            "$ref": "#/definitions/aggregate_relation"
///          },
///          {
///            "$ref": "#/definitions/sort_relation"
///          },
///          {
///            "$ref": "#/definitions/join_relation"
///          },
///          {
///            "$ref": "#/definitions/project_relation"
///          },
///          {
///            "$ref": "#/definitions/set_relation"
///          },
///          {
///            "$ref": "#/definitions/extension_single_relation"
///          },
///          {
///            "$ref": "#/definitions/extension_multi_relation"
///          },
///          {
///            "$ref": "#/definitions/extension_leaf_relation"
///          },
///          {
///            "$ref": "#/definitions/cross_relation"
///          },
///          {
///            "$ref": "#/definitions/reference_relation"
///          },
///          {
///            "$ref": "#/definitions/write_relation"
///          },
///          {
///            "$ref": "#/definitions/ddl_relation"
///          },
///          {
///            "$ref": "#/definitions/update_relation"
///          },
///          {
///            "$ref": "#/definitions/hash_join_relation"
///          },
///          {
///            "$ref": "#/definitions/merge_join_relation"
///          },
///          {
///            "$ref": "#/definitions/nested_loop_join_relation"
///          },
///          {
///            "$ref": "#/definitions/consistent_partition_window_relation"
///          },
///          {
///            "$ref": "#/definitions/exchange_relation"
///          },
///          {
///            "$ref": "#/definitions/expand_relation"
///          }
///        ]
///      },
///      "uniqueItems": true
///    },
///    "supported_scalar_functions": {
///      "description": "Regular functions that operate on scalar values.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/dialect_function"
///      }
///    },
///    "supported_types": {
///      "description": "Types that are supported by this dialect. If not present or empty, all standard types are supported.\n",
///      "type": "array",
///      "items": {
///        "oneOf": [
///          {
///            "enum": [
///              "BOOL",
///              "I8",
///              "I16",
///              "I32",
///              "I64",
///              "FP32",
///              "FP64",
///              "BINARY",
///              "FIXED_BINARY",
///              "STRING",
///              "VARCHAR",
///              "FIXED_CHAR",
///              "PRECISION_TIME",
///              "PRECISION_TIMESTAMP",
///              "PRECISION_TIMESTAMP_TZ",
///              "DATE",
///              "TIME",
///              "INTERVAL_COMPOUND",
///              "INTERVAL_DAY",
///              "INTERVAL_YEAR",
///              "UUID",
///              "DECIMAL",
///              "STRUCT",
///              "LIST",
///              "MAP"
///            ]
///          },
///          {
///            "$ref": "#/definitions/type_bool"
///          },
///          {
///            "$ref": "#/definitions/type_i8"
///          },
///          {
///            "$ref": "#/definitions/type_i16"
///          },
///          {
///            "$ref": "#/definitions/type_i32"
///          },
///          {
///            "$ref": "#/definitions/type_i64"
///          },
///          {
///            "$ref": "#/definitions/type_fp32"
///          },
///          {
///            "$ref": "#/definitions/type_fp64"
///          },
///          {
///            "$ref": "#/definitions/type_binary"
///          },
///          {
///            "$ref": "#/definitions/type_fixed_binary"
///          },
///          {
///            "$ref": "#/definitions/type_string"
///          },
///          {
///            "$ref": "#/definitions/type_varchar"
///          },
///          {
///            "$ref": "#/definitions/type_fixed_char"
///          },
///          {
///            "$ref": "#/definitions/type_precision_time"
///          },
///          {
///            "$ref": "#/definitions/type_precision_timestamp"
///          },
///          {
///            "$ref": "#/definitions/type_precision_timestamp_tz"
///          },
///          {
///            "$ref": "#/definitions/type_date"
///          },
///          {
///            "$ref": "#/definitions/type_time"
///          },
///          {
///            "$ref": "#/definitions/type_interval_compound"
///          },
///          {
///            "$ref": "#/definitions/type_interval_day"
///          },
///          {
///            "$ref": "#/definitions/type_interval_year"
///          },
///          {
///            "$ref": "#/definitions/type_uuid"
///          },
///          {
///            "$ref": "#/definitions/type_decimal"
///          },
///          {
///            "$ref": "#/definitions/type_struct"
///          },
///          {
///            "$ref": "#/definitions/type_list"
///          },
///          {
///            "$ref": "#/definitions/type_map"
///          },
///          {
///            "$ref": "#/definitions/type_user_defined"
///          }
///        ]
///      },
///      "uniqueItems": true
///    },
///    "supported_window_functions": {
///      "description": "Functions that operate over a window of rows.",
///      "type": "array",
///      "items": {
///        "$ref": "#/definitions/dialect_function"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dialect {
    /**A map of extension files depended on by this dialect. Keys are aliases for extensions, which are used within the dialect. Values are URNs.
*/
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub dependencies: ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >,
    ///Optional name of the dialect (e.g., "MySQL", "PostgreSQL", "Spark").
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    ///Functions that aggregate multiple rows into a single result.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub supported_aggregate_functions: ::std::vec::Vec<DialectFunction>,
    /**Expressions that are supported by this dialect.  If not present or empty, all expressions are supported.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supported_expressions: ::std::option::Option<
        Vec<DialectSupportedExpressionsItem>,
    >,
    /**Relations that are supported by this dialect.  If not present or empty, all relations are supported.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supported_relations: ::std::option::Option<Vec<DialectSupportedRelationsItem>>,
    ///Regular functions that operate on scalar values.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub supported_scalar_functions: ::std::vec::Vec<DialectFunction>,
    /**Types that are supported by this dialect. If not present or empty, all standard types are supported.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supported_types: ::std::option::Option<Vec<DialectSupportedTypesItem>>,
    ///Functions that operate over a window of rows.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub supported_window_functions: ::std::vec::Vec<DialectFunction>,
}
impl ::std::convert::From<&Dialect> for Dialect {
    fn from(value: &Dialect) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for Dialect {
    fn default() -> Self {
        Self {
            dependencies: Default::default(),
            name: Default::default(),
            supported_aggregate_functions: Default::default(),
            supported_expressions: Default::default(),
            supported_relations: Default::default(),
            supported_scalar_functions: Default::default(),
            supported_types: Default::default(),
            supported_window_functions: Default::default(),
        }
    }
}
impl Dialect {
    pub fn builder() -> builder::Dialect {
        Default::default()
    }
}
///`DialectFunction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "source",
///    "supported_impls"
///  ],
///  "properties": {
///    "name": {
///      "description": "The name of the function as declared in the extension it is defined in.\n",
///      "type": "string"
///    },
///    "required_options": {
///      "description": "Function-specific options that must be supported by the dialect.  These are the exact list of options that must be used.  If no options are listed here, no options must be present.\n",
///      "type": "object"
///    },
///    "source": {
///      "description": "Dependency (alias) in which the function is declared.",
///      "type": "string"
///    },
///    "supported_impls": {
///      "description": "List of one or more implementations supported by this function. Use argument signatures to identify the implementations.\n",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "minItems": 1
///    },
///    "system_metadata": {
///      "description": "system specific metadata for the function",
///      "type": "object",
///      "$ref": "#/definitions/system_function_metadata"
///    },
///    "variadic": {
///      "description": "If present, indicates the function accepts a variable number of arguments. Specifies the minimum and/or maximum number of arguments allowed.\n",
///      "type": "object",
///      "properties": {
///        "max": {
///          "description": "Maximum number of arguments allowed",
///          "type": "integer",
///          "minimum": 1.0
///        },
///        "min": {
///          "description": "Minimum number of arguments required",
///          "type": "integer",
///          "minimum": 1.0
///        }
///      },
///      "additionalProperties": false
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DialectFunction {
    /**The name of the function as declared in the extension it is defined in.
*/
    pub name: ::std::string::String,
    /**Function-specific options that must be supported by the dialect.  These are the exact list of options that must be used.  If no options are listed here, no options must be present.
*/
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub required_options: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ///Dependency (alias) in which the function is declared.
    pub source: ::std::string::String,
    /**List of one or more implementations supported by this function. Use argument signatures to identify the implementations.
*/
    pub supported_impls: ::std::vec::Vec<::std::string::String>,
    ///system specific metadata for the function
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemFunctionMetadata>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub variadic: ::std::option::Option<DialectFunctionVariadic>,
}
impl ::std::convert::From<&DialectFunction> for DialectFunction {
    fn from(value: &DialectFunction) -> Self {
        value.clone()
    }
}
impl DialectFunction {
    pub fn builder() -> builder::DialectFunction {
        Default::default()
    }
}
/**If present, indicates the function accepts a variable number of arguments. Specifies the minimum and/or maximum number of arguments allowed.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "If present, indicates the function accepts a variable number of arguments. Specifies the minimum and/or maximum number of arguments allowed.\n",
///  "type": "object",
///  "properties": {
///    "max": {
///      "description": "Maximum number of arguments allowed",
///      "type": "integer",
///      "minimum": 1.0
///    },
///    "min": {
///      "description": "Minimum number of arguments required",
///      "type": "integer",
///      "minimum": 1.0
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DialectFunctionVariadic {
    ///Maximum number of arguments allowed
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<::std::num::NonZeroU64>,
    ///Minimum number of arguments required
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<::std::num::NonZeroU64>,
}
impl ::std::convert::From<&DialectFunctionVariadic> for DialectFunctionVariadic {
    fn from(value: &DialectFunctionVariadic) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for DialectFunctionVariadic {
    fn default() -> Self {
        Self {
            max: Default::default(),
            min: Default::default(),
        }
    }
}
impl DialectFunctionVariadic {
    pub fn builder() -> builder::DialectFunctionVariadic {
        Default::default()
    }
}
///`DialectSupportedExpressionsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "enum": [
///        "LITERAL",
///        "SELECTION",
///        "SCALAR_FUNCTION",
///        "WINDOW_FUNCTION",
///        "IF_THEN",
///        "SWITCH",
///        "SINGULAR_OR_LIST",
///        "MULTI_OR_LIST",
///        "CAST",
///        "SUBQUERY",
///        "NESTED",
///        "DYNAMIC_PARAMETER"
///      ]
///    },
///    {
///      "$ref": "#/definitions/literal_expression"
///    },
///    {
///      "$ref": "#/definitions/selection_expression"
///    },
///    {
///      "$ref": "#/definitions/scalar_function_expression"
///    },
///    {
///      "$ref": "#/definitions/window_function_expression"
///    },
///    {
///      "$ref": "#/definitions/if_then_expression"
///    },
///    {
///      "$ref": "#/definitions/switch_expression"
///    },
///    {
///      "$ref": "#/definitions/singular_or_list_expression"
///    },
///    {
///      "$ref": "#/definitions/multi_or_list_expression"
///    },
///    {
///      "$ref": "#/definitions/cast_expression"
///    },
///    {
///      "$ref": "#/definitions/subquery_expression"
///    },
///    {
///      "$ref": "#/definitions/nested_expression"
///    },
///    {
///      "$ref": "#/definitions/dynamic_parameter_expression"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DialectSupportedExpressionsItem {
    Variant0(DialectSupportedExpressionsItemVariant0),
    Variant1(LiteralExpression),
    Variant2(SelectionExpression),
    Variant3(ScalarFunctionExpression),
    Variant4(WindowFunctionExpression),
    Variant5(IfThenExpression),
    Variant6(SwitchExpression),
    Variant7(SingularOrListExpression),
    Variant8(MultiOrListExpression),
    Variant9(CastExpression),
    Variant10(SubqueryExpression),
    Variant11(NestedExpression),
    Variant12(DynamicParameterExpression),
}
impl ::std::convert::From<&Self> for DialectSupportedExpressionsItem {
    fn from(value: &DialectSupportedExpressionsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<DialectSupportedExpressionsItemVariant0>
for DialectSupportedExpressionsItem {
    fn from(value: DialectSupportedExpressionsItemVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<LiteralExpression> for DialectSupportedExpressionsItem {
    fn from(value: LiteralExpression) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<SelectionExpression> for DialectSupportedExpressionsItem {
    fn from(value: SelectionExpression) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<ScalarFunctionExpression> for DialectSupportedExpressionsItem {
    fn from(value: ScalarFunctionExpression) -> Self {
        Self::Variant3(value)
    }
}
impl ::std::convert::From<WindowFunctionExpression> for DialectSupportedExpressionsItem {
    fn from(value: WindowFunctionExpression) -> Self {
        Self::Variant4(value)
    }
}
impl ::std::convert::From<IfThenExpression> for DialectSupportedExpressionsItem {
    fn from(value: IfThenExpression) -> Self {
        Self::Variant5(value)
    }
}
impl ::std::convert::From<SwitchExpression> for DialectSupportedExpressionsItem {
    fn from(value: SwitchExpression) -> Self {
        Self::Variant6(value)
    }
}
impl ::std::convert::From<SingularOrListExpression> for DialectSupportedExpressionsItem {
    fn from(value: SingularOrListExpression) -> Self {
        Self::Variant7(value)
    }
}
impl ::std::convert::From<MultiOrListExpression> for DialectSupportedExpressionsItem {
    fn from(value: MultiOrListExpression) -> Self {
        Self::Variant8(value)
    }
}
impl ::std::convert::From<CastExpression> for DialectSupportedExpressionsItem {
    fn from(value: CastExpression) -> Self {
        Self::Variant9(value)
    }
}
impl ::std::convert::From<SubqueryExpression> for DialectSupportedExpressionsItem {
    fn from(value: SubqueryExpression) -> Self {
        Self::Variant10(value)
    }
}
impl ::std::convert::From<NestedExpression> for DialectSupportedExpressionsItem {
    fn from(value: NestedExpression) -> Self {
        Self::Variant11(value)
    }
}
impl ::std::convert::From<DynamicParameterExpression>
for DialectSupportedExpressionsItem {
    fn from(value: DynamicParameterExpression) -> Self {
        Self::Variant12(value)
    }
}
///`DialectSupportedExpressionsItemVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "LITERAL",
///    "SELECTION",
///    "SCALAR_FUNCTION",
///    "WINDOW_FUNCTION",
///    "IF_THEN",
///    "SWITCH",
///    "SINGULAR_OR_LIST",
///    "MULTI_OR_LIST",
///    "CAST",
///    "SUBQUERY",
///    "NESTED",
///    "DYNAMIC_PARAMETER"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DialectSupportedExpressionsItemVariant0 {
    #[serde(rename = "LITERAL")]
    Literal,
    #[serde(rename = "SELECTION")]
    Selection,
    #[serde(rename = "SCALAR_FUNCTION")]
    ScalarFunction,
    #[serde(rename = "WINDOW_FUNCTION")]
    WindowFunction,
    #[serde(rename = "IF_THEN")]
    IfThen,
    #[serde(rename = "SWITCH")]
    Switch,
    #[serde(rename = "SINGULAR_OR_LIST")]
    SingularOrList,
    #[serde(rename = "MULTI_OR_LIST")]
    MultiOrList,
    #[serde(rename = "CAST")]
    Cast,
    #[serde(rename = "SUBQUERY")]
    Subquery,
    #[serde(rename = "NESTED")]
    Nested,
    #[serde(rename = "DYNAMIC_PARAMETER")]
    DynamicParameter,
}
impl ::std::convert::From<&Self> for DialectSupportedExpressionsItemVariant0 {
    fn from(value: &DialectSupportedExpressionsItemVariant0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DialectSupportedExpressionsItemVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Literal => f.write_str("LITERAL"),
            Self::Selection => f.write_str("SELECTION"),
            Self::ScalarFunction => f.write_str("SCALAR_FUNCTION"),
            Self::WindowFunction => f.write_str("WINDOW_FUNCTION"),
            Self::IfThen => f.write_str("IF_THEN"),
            Self::Switch => f.write_str("SWITCH"),
            Self::SingularOrList => f.write_str("SINGULAR_OR_LIST"),
            Self::MultiOrList => f.write_str("MULTI_OR_LIST"),
            Self::Cast => f.write_str("CAST"),
            Self::Subquery => f.write_str("SUBQUERY"),
            Self::Nested => f.write_str("NESTED"),
            Self::DynamicParameter => f.write_str("DYNAMIC_PARAMETER"),
        }
    }
}
impl ::std::str::FromStr for DialectSupportedExpressionsItemVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "LITERAL" => Ok(Self::Literal),
            "SELECTION" => Ok(Self::Selection),
            "SCALAR_FUNCTION" => Ok(Self::ScalarFunction),
            "WINDOW_FUNCTION" => Ok(Self::WindowFunction),
            "IF_THEN" => Ok(Self::IfThen),
            "SWITCH" => Ok(Self::Switch),
            "SINGULAR_OR_LIST" => Ok(Self::SingularOrList),
            "MULTI_OR_LIST" => Ok(Self::MultiOrList),
            "CAST" => Ok(Self::Cast),
            "SUBQUERY" => Ok(Self::Subquery),
            "NESTED" => Ok(Self::Nested),
            "DYNAMIC_PARAMETER" => Ok(Self::DynamicParameter),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DialectSupportedExpressionsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for DialectSupportedExpressionsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DialectSupportedExpressionsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DialectSupportedRelationsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "enum": [
///        "READ",
///        "FILTER",
///        "FETCH",
///        "AGGREGATE",
///        "SORT",
///        "JOIN",
///        "PROJECT",
///        "SET",
///        "CROSS",
///        "REFERENCE",
///        "WRITE",
///        "DDL",
///        "UPDATE",
///        "HASH_JOIN",
///        "MERGE_JOIN",
///        "NESTED_LOOP_JOIN",
///        "CONSISTENT_PARTITION_WINDOW",
///        "EXCHANGE",
///        "EXPAND"
///      ]
///    },
///    {
///      "$ref": "#/definitions/read_relation"
///    },
///    {
///      "$ref": "#/definitions/filter_relation"
///    },
///    {
///      "$ref": "#/definitions/fetch_relation"
///    },
///    {
///      "$ref": "#/definitions/aggregate_relation"
///    },
///    {
///      "$ref": "#/definitions/sort_relation"
///    },
///    {
///      "$ref": "#/definitions/join_relation"
///    },
///    {
///      "$ref": "#/definitions/project_relation"
///    },
///    {
///      "$ref": "#/definitions/set_relation"
///    },
///    {
///      "$ref": "#/definitions/extension_single_relation"
///    },
///    {
///      "$ref": "#/definitions/extension_multi_relation"
///    },
///    {
///      "$ref": "#/definitions/extension_leaf_relation"
///    },
///    {
///      "$ref": "#/definitions/cross_relation"
///    },
///    {
///      "$ref": "#/definitions/reference_relation"
///    },
///    {
///      "$ref": "#/definitions/write_relation"
///    },
///    {
///      "$ref": "#/definitions/ddl_relation"
///    },
///    {
///      "$ref": "#/definitions/update_relation"
///    },
///    {
///      "$ref": "#/definitions/hash_join_relation"
///    },
///    {
///      "$ref": "#/definitions/merge_join_relation"
///    },
///    {
///      "$ref": "#/definitions/nested_loop_join_relation"
///    },
///    {
///      "$ref": "#/definitions/consistent_partition_window_relation"
///    },
///    {
///      "$ref": "#/definitions/exchange_relation"
///    },
///    {
///      "$ref": "#/definitions/expand_relation"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DialectSupportedRelationsItem {
    Variant0(DialectSupportedRelationsItemVariant0),
    Variant1(ReadRelation),
    Variant2(FilterRelation),
    Variant3(FetchRelation),
    Variant4(AggregateRelation),
    Variant5(SortRelation),
    Variant6(JoinRelation),
    Variant7(ProjectRelation),
    Variant8(SetRelation),
    Variant9(ExtensionSingleRelation),
    Variant10(ExtensionMultiRelation),
    Variant11(ExtensionLeafRelation),
    Variant12(CrossRelation),
    Variant13(ReferenceRelation),
    Variant14(WriteRelation),
    Variant15(DdlRelation),
    Variant16(UpdateRelation),
    Variant17(HashJoinRelation),
    Variant18(MergeJoinRelation),
    Variant19(NestedLoopJoinRelation),
    Variant20(ConsistentPartitionWindowRelation),
    Variant21(ExchangeRelation),
    Variant22(ExpandRelation),
}
impl ::std::convert::From<&Self> for DialectSupportedRelationsItem {
    fn from(value: &DialectSupportedRelationsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<DialectSupportedRelationsItemVariant0>
for DialectSupportedRelationsItem {
    fn from(value: DialectSupportedRelationsItemVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<ReadRelation> for DialectSupportedRelationsItem {
    fn from(value: ReadRelation) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<FilterRelation> for DialectSupportedRelationsItem {
    fn from(value: FilterRelation) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<FetchRelation> for DialectSupportedRelationsItem {
    fn from(value: FetchRelation) -> Self {
        Self::Variant3(value)
    }
}
impl ::std::convert::From<AggregateRelation> for DialectSupportedRelationsItem {
    fn from(value: AggregateRelation) -> Self {
        Self::Variant4(value)
    }
}
impl ::std::convert::From<SortRelation> for DialectSupportedRelationsItem {
    fn from(value: SortRelation) -> Self {
        Self::Variant5(value)
    }
}
impl ::std::convert::From<JoinRelation> for DialectSupportedRelationsItem {
    fn from(value: JoinRelation) -> Self {
        Self::Variant6(value)
    }
}
impl ::std::convert::From<ProjectRelation> for DialectSupportedRelationsItem {
    fn from(value: ProjectRelation) -> Self {
        Self::Variant7(value)
    }
}
impl ::std::convert::From<SetRelation> for DialectSupportedRelationsItem {
    fn from(value: SetRelation) -> Self {
        Self::Variant8(value)
    }
}
impl ::std::convert::From<ExtensionSingleRelation> for DialectSupportedRelationsItem {
    fn from(value: ExtensionSingleRelation) -> Self {
        Self::Variant9(value)
    }
}
impl ::std::convert::From<ExtensionMultiRelation> for DialectSupportedRelationsItem {
    fn from(value: ExtensionMultiRelation) -> Self {
        Self::Variant10(value)
    }
}
impl ::std::convert::From<ExtensionLeafRelation> for DialectSupportedRelationsItem {
    fn from(value: ExtensionLeafRelation) -> Self {
        Self::Variant11(value)
    }
}
impl ::std::convert::From<CrossRelation> for DialectSupportedRelationsItem {
    fn from(value: CrossRelation) -> Self {
        Self::Variant12(value)
    }
}
impl ::std::convert::From<ReferenceRelation> for DialectSupportedRelationsItem {
    fn from(value: ReferenceRelation) -> Self {
        Self::Variant13(value)
    }
}
impl ::std::convert::From<WriteRelation> for DialectSupportedRelationsItem {
    fn from(value: WriteRelation) -> Self {
        Self::Variant14(value)
    }
}
impl ::std::convert::From<DdlRelation> for DialectSupportedRelationsItem {
    fn from(value: DdlRelation) -> Self {
        Self::Variant15(value)
    }
}
impl ::std::convert::From<UpdateRelation> for DialectSupportedRelationsItem {
    fn from(value: UpdateRelation) -> Self {
        Self::Variant16(value)
    }
}
impl ::std::convert::From<HashJoinRelation> for DialectSupportedRelationsItem {
    fn from(value: HashJoinRelation) -> Self {
        Self::Variant17(value)
    }
}
impl ::std::convert::From<MergeJoinRelation> for DialectSupportedRelationsItem {
    fn from(value: MergeJoinRelation) -> Self {
        Self::Variant18(value)
    }
}
impl ::std::convert::From<NestedLoopJoinRelation> for DialectSupportedRelationsItem {
    fn from(value: NestedLoopJoinRelation) -> Self {
        Self::Variant19(value)
    }
}
impl ::std::convert::From<ConsistentPartitionWindowRelation>
for DialectSupportedRelationsItem {
    fn from(value: ConsistentPartitionWindowRelation) -> Self {
        Self::Variant20(value)
    }
}
impl ::std::convert::From<ExchangeRelation> for DialectSupportedRelationsItem {
    fn from(value: ExchangeRelation) -> Self {
        Self::Variant21(value)
    }
}
impl ::std::convert::From<ExpandRelation> for DialectSupportedRelationsItem {
    fn from(value: ExpandRelation) -> Self {
        Self::Variant22(value)
    }
}
///`DialectSupportedRelationsItemVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "READ",
///    "FILTER",
///    "FETCH",
///    "AGGREGATE",
///    "SORT",
///    "JOIN",
///    "PROJECT",
///    "SET",
///    "CROSS",
///    "REFERENCE",
///    "WRITE",
///    "DDL",
///    "UPDATE",
///    "HASH_JOIN",
///    "MERGE_JOIN",
///    "NESTED_LOOP_JOIN",
///    "CONSISTENT_PARTITION_WINDOW",
///    "EXCHANGE",
///    "EXPAND"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DialectSupportedRelationsItemVariant0 {
    #[serde(rename = "READ")]
    Read,
    #[serde(rename = "FILTER")]
    Filter,
    #[serde(rename = "FETCH")]
    Fetch,
    #[serde(rename = "AGGREGATE")]
    Aggregate,
    #[serde(rename = "SORT")]
    Sort,
    #[serde(rename = "JOIN")]
    Join,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "SET")]
    Set,
    #[serde(rename = "CROSS")]
    Cross,
    #[serde(rename = "REFERENCE")]
    Reference,
    #[serde(rename = "WRITE")]
    Write,
    #[serde(rename = "DDL")]
    Ddl,
    #[serde(rename = "UPDATE")]
    Update,
    #[serde(rename = "HASH_JOIN")]
    HashJoin,
    #[serde(rename = "MERGE_JOIN")]
    MergeJoin,
    #[serde(rename = "NESTED_LOOP_JOIN")]
    NestedLoopJoin,
    #[serde(rename = "CONSISTENT_PARTITION_WINDOW")]
    ConsistentPartitionWindow,
    #[serde(rename = "EXCHANGE")]
    Exchange,
    #[serde(rename = "EXPAND")]
    Expand,
}
impl ::std::convert::From<&Self> for DialectSupportedRelationsItemVariant0 {
    fn from(value: &DialectSupportedRelationsItemVariant0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DialectSupportedRelationsItemVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Read => f.write_str("READ"),
            Self::Filter => f.write_str("FILTER"),
            Self::Fetch => f.write_str("FETCH"),
            Self::Aggregate => f.write_str("AGGREGATE"),
            Self::Sort => f.write_str("SORT"),
            Self::Join => f.write_str("JOIN"),
            Self::Project => f.write_str("PROJECT"),
            Self::Set => f.write_str("SET"),
            Self::Cross => f.write_str("CROSS"),
            Self::Reference => f.write_str("REFERENCE"),
            Self::Write => f.write_str("WRITE"),
            Self::Ddl => f.write_str("DDL"),
            Self::Update => f.write_str("UPDATE"),
            Self::HashJoin => f.write_str("HASH_JOIN"),
            Self::MergeJoin => f.write_str("MERGE_JOIN"),
            Self::NestedLoopJoin => f.write_str("NESTED_LOOP_JOIN"),
            Self::ConsistentPartitionWindow => f.write_str("CONSISTENT_PARTITION_WINDOW"),
            Self::Exchange => f.write_str("EXCHANGE"),
            Self::Expand => f.write_str("EXPAND"),
        }
    }
}
impl ::std::str::FromStr for DialectSupportedRelationsItemVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "READ" => Ok(Self::Read),
            "FILTER" => Ok(Self::Filter),
            "FETCH" => Ok(Self::Fetch),
            "AGGREGATE" => Ok(Self::Aggregate),
            "SORT" => Ok(Self::Sort),
            "JOIN" => Ok(Self::Join),
            "PROJECT" => Ok(Self::Project),
            "SET" => Ok(Self::Set),
            "CROSS" => Ok(Self::Cross),
            "REFERENCE" => Ok(Self::Reference),
            "WRITE" => Ok(Self::Write),
            "DDL" => Ok(Self::Ddl),
            "UPDATE" => Ok(Self::Update),
            "HASH_JOIN" => Ok(Self::HashJoin),
            "MERGE_JOIN" => Ok(Self::MergeJoin),
            "NESTED_LOOP_JOIN" => Ok(Self::NestedLoopJoin),
            "CONSISTENT_PARTITION_WINDOW" => Ok(Self::ConsistentPartitionWindow),
            "EXCHANGE" => Ok(Self::Exchange),
            "EXPAND" => Ok(Self::Expand),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DialectSupportedRelationsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for DialectSupportedRelationsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DialectSupportedRelationsItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DialectSupportedTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "enum": [
///        "BOOL",
///        "I8",
///        "I16",
///        "I32",
///        "I64",
///        "FP32",
///        "FP64",
///        "BINARY",
///        "FIXED_BINARY",
///        "STRING",
///        "VARCHAR",
///        "FIXED_CHAR",
///        "PRECISION_TIME",
///        "PRECISION_TIMESTAMP",
///        "PRECISION_TIMESTAMP_TZ",
///        "DATE",
///        "TIME",
///        "INTERVAL_COMPOUND",
///        "INTERVAL_DAY",
///        "INTERVAL_YEAR",
///        "UUID",
///        "DECIMAL",
///        "STRUCT",
///        "LIST",
///        "MAP"
///      ]
///    },
///    {
///      "$ref": "#/definitions/type_bool"
///    },
///    {
///      "$ref": "#/definitions/type_i8"
///    },
///    {
///      "$ref": "#/definitions/type_i16"
///    },
///    {
///      "$ref": "#/definitions/type_i32"
///    },
///    {
///      "$ref": "#/definitions/type_i64"
///    },
///    {
///      "$ref": "#/definitions/type_fp32"
///    },
///    {
///      "$ref": "#/definitions/type_fp64"
///    },
///    {
///      "$ref": "#/definitions/type_binary"
///    },
///    {
///      "$ref": "#/definitions/type_fixed_binary"
///    },
///    {
///      "$ref": "#/definitions/type_string"
///    },
///    {
///      "$ref": "#/definitions/type_varchar"
///    },
///    {
///      "$ref": "#/definitions/type_fixed_char"
///    },
///    {
///      "$ref": "#/definitions/type_precision_time"
///    },
///    {
///      "$ref": "#/definitions/type_precision_timestamp"
///    },
///    {
///      "$ref": "#/definitions/type_precision_timestamp_tz"
///    },
///    {
///      "$ref": "#/definitions/type_date"
///    },
///    {
///      "$ref": "#/definitions/type_time"
///    },
///    {
///      "$ref": "#/definitions/type_interval_compound"
///    },
///    {
///      "$ref": "#/definitions/type_interval_day"
///    },
///    {
///      "$ref": "#/definitions/type_interval_year"
///    },
///    {
///      "$ref": "#/definitions/type_uuid"
///    },
///    {
///      "$ref": "#/definitions/type_decimal"
///    },
///    {
///      "$ref": "#/definitions/type_struct"
///    },
///    {
///      "$ref": "#/definitions/type_list"
///    },
///    {
///      "$ref": "#/definitions/type_map"
///    },
///    {
///      "$ref": "#/definitions/type_user_defined"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum DialectSupportedTypesItem {
    Variant0(DialectSupportedTypesItemVariant0),
    Variant1(TypeBool),
    Variant2(TypeI8),
    Variant3(TypeI16),
    Variant4(TypeI32),
    Variant5(TypeI64),
    Variant6(TypeFp32),
    Variant7(TypeFp64),
    Variant8(TypeBinary),
    Variant9(TypeFixedBinary),
    Variant10(TypeString),
    Variant11(TypeVarchar),
    Variant12(TypeFixedChar),
    Variant13(TypePrecisionTime),
    Variant14(TypePrecisionTimestamp),
    Variant15(TypePrecisionTimestampTz),
    Variant16(TypeDate),
    Variant17(TypeTime),
    Variant18(TypeIntervalCompound),
    Variant19(TypeIntervalDay),
    Variant20(TypeIntervalYear),
    Variant21(TypeUuid),
    Variant22(TypeDecimal),
    Variant23(TypeStruct),
    Variant24(TypeList),
    Variant25(TypeMap),
    Variant26(TypeUserDefined),
}
impl ::std::convert::From<&Self> for DialectSupportedTypesItem {
    fn from(value: &DialectSupportedTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<DialectSupportedTypesItemVariant0>
for DialectSupportedTypesItem {
    fn from(value: DialectSupportedTypesItemVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<TypeBool> for DialectSupportedTypesItem {
    fn from(value: TypeBool) -> Self {
        Self::Variant1(value)
    }
}
impl ::std::convert::From<TypeI8> for DialectSupportedTypesItem {
    fn from(value: TypeI8) -> Self {
        Self::Variant2(value)
    }
}
impl ::std::convert::From<TypeI16> for DialectSupportedTypesItem {
    fn from(value: TypeI16) -> Self {
        Self::Variant3(value)
    }
}
impl ::std::convert::From<TypeI32> for DialectSupportedTypesItem {
    fn from(value: TypeI32) -> Self {
        Self::Variant4(value)
    }
}
impl ::std::convert::From<TypeI64> for DialectSupportedTypesItem {
    fn from(value: TypeI64) -> Self {
        Self::Variant5(value)
    }
}
impl ::std::convert::From<TypeFp32> for DialectSupportedTypesItem {
    fn from(value: TypeFp32) -> Self {
        Self::Variant6(value)
    }
}
impl ::std::convert::From<TypeFp64> for DialectSupportedTypesItem {
    fn from(value: TypeFp64) -> Self {
        Self::Variant7(value)
    }
}
impl ::std::convert::From<TypeBinary> for DialectSupportedTypesItem {
    fn from(value: TypeBinary) -> Self {
        Self::Variant8(value)
    }
}
impl ::std::convert::From<TypeFixedBinary> for DialectSupportedTypesItem {
    fn from(value: TypeFixedBinary) -> Self {
        Self::Variant9(value)
    }
}
impl ::std::convert::From<TypeString> for DialectSupportedTypesItem {
    fn from(value: TypeString) -> Self {
        Self::Variant10(value)
    }
}
impl ::std::convert::From<TypeVarchar> for DialectSupportedTypesItem {
    fn from(value: TypeVarchar) -> Self {
        Self::Variant11(value)
    }
}
impl ::std::convert::From<TypeFixedChar> for DialectSupportedTypesItem {
    fn from(value: TypeFixedChar) -> Self {
        Self::Variant12(value)
    }
}
impl ::std::convert::From<TypePrecisionTime> for DialectSupportedTypesItem {
    fn from(value: TypePrecisionTime) -> Self {
        Self::Variant13(value)
    }
}
impl ::std::convert::From<TypePrecisionTimestamp> for DialectSupportedTypesItem {
    fn from(value: TypePrecisionTimestamp) -> Self {
        Self::Variant14(value)
    }
}
impl ::std::convert::From<TypePrecisionTimestampTz> for DialectSupportedTypesItem {
    fn from(value: TypePrecisionTimestampTz) -> Self {
        Self::Variant15(value)
    }
}
impl ::std::convert::From<TypeDate> for DialectSupportedTypesItem {
    fn from(value: TypeDate) -> Self {
        Self::Variant16(value)
    }
}
impl ::std::convert::From<TypeTime> for DialectSupportedTypesItem {
    fn from(value: TypeTime) -> Self {
        Self::Variant17(value)
    }
}
impl ::std::convert::From<TypeIntervalCompound> for DialectSupportedTypesItem {
    fn from(value: TypeIntervalCompound) -> Self {
        Self::Variant18(value)
    }
}
impl ::std::convert::From<TypeIntervalDay> for DialectSupportedTypesItem {
    fn from(value: TypeIntervalDay) -> Self {
        Self::Variant19(value)
    }
}
impl ::std::convert::From<TypeIntervalYear> for DialectSupportedTypesItem {
    fn from(value: TypeIntervalYear) -> Self {
        Self::Variant20(value)
    }
}
impl ::std::convert::From<TypeUuid> for DialectSupportedTypesItem {
    fn from(value: TypeUuid) -> Self {
        Self::Variant21(value)
    }
}
impl ::std::convert::From<TypeDecimal> for DialectSupportedTypesItem {
    fn from(value: TypeDecimal) -> Self {
        Self::Variant22(value)
    }
}
impl ::std::convert::From<TypeStruct> for DialectSupportedTypesItem {
    fn from(value: TypeStruct) -> Self {
        Self::Variant23(value)
    }
}
impl ::std::convert::From<TypeList> for DialectSupportedTypesItem {
    fn from(value: TypeList) -> Self {
        Self::Variant24(value)
    }
}
impl ::std::convert::From<TypeMap> for DialectSupportedTypesItem {
    fn from(value: TypeMap) -> Self {
        Self::Variant25(value)
    }
}
impl ::std::convert::From<TypeUserDefined> for DialectSupportedTypesItem {
    fn from(value: TypeUserDefined) -> Self {
        Self::Variant26(value)
    }
}
///`DialectSupportedTypesItemVariant0`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "BOOL",
///    "I8",
///    "I16",
///    "I32",
///    "I64",
///    "FP32",
///    "FP64",
///    "BINARY",
///    "FIXED_BINARY",
///    "STRING",
///    "VARCHAR",
///    "FIXED_CHAR",
///    "PRECISION_TIME",
///    "PRECISION_TIMESTAMP",
///    "PRECISION_TIMESTAMP_TZ",
///    "DATE",
///    "TIME",
///    "INTERVAL_COMPOUND",
///    "INTERVAL_DAY",
///    "INTERVAL_YEAR",
///    "UUID",
///    "DECIMAL",
///    "STRUCT",
///    "LIST",
///    "MAP"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum DialectSupportedTypesItemVariant0 {
    #[serde(rename = "BOOL")]
    Bool,
    I8,
    I16,
    I32,
    I64,
    #[serde(rename = "FP32")]
    Fp32,
    #[serde(rename = "FP64")]
    Fp64,
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "FIXED_BINARY")]
    FixedBinary,
    #[serde(rename = "STRING")]
    String,
    #[serde(rename = "VARCHAR")]
    Varchar,
    #[serde(rename = "FIXED_CHAR")]
    FixedChar,
    #[serde(rename = "PRECISION_TIME")]
    PrecisionTime,
    #[serde(rename = "PRECISION_TIMESTAMP")]
    PrecisionTimestamp,
    #[serde(rename = "PRECISION_TIMESTAMP_TZ")]
    PrecisionTimestampTz,
    #[serde(rename = "DATE")]
    Date,
    #[serde(rename = "TIME")]
    Time,
    #[serde(rename = "INTERVAL_COMPOUND")]
    IntervalCompound,
    #[serde(rename = "INTERVAL_DAY")]
    IntervalDay,
    #[serde(rename = "INTERVAL_YEAR")]
    IntervalYear,
    #[serde(rename = "UUID")]
    Uuid,
    #[serde(rename = "DECIMAL")]
    Decimal,
    #[serde(rename = "STRUCT")]
    Struct,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "MAP")]
    Map,
}
impl ::std::convert::From<&Self> for DialectSupportedTypesItemVariant0 {
    fn from(value: &DialectSupportedTypesItemVariant0) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for DialectSupportedTypesItemVariant0 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Bool => f.write_str("BOOL"),
            Self::I8 => f.write_str("I8"),
            Self::I16 => f.write_str("I16"),
            Self::I32 => f.write_str("I32"),
            Self::I64 => f.write_str("I64"),
            Self::Fp32 => f.write_str("FP32"),
            Self::Fp64 => f.write_str("FP64"),
            Self::Binary => f.write_str("BINARY"),
            Self::FixedBinary => f.write_str("FIXED_BINARY"),
            Self::String => f.write_str("STRING"),
            Self::Varchar => f.write_str("VARCHAR"),
            Self::FixedChar => f.write_str("FIXED_CHAR"),
            Self::PrecisionTime => f.write_str("PRECISION_TIME"),
            Self::PrecisionTimestamp => f.write_str("PRECISION_TIMESTAMP"),
            Self::PrecisionTimestampTz => f.write_str("PRECISION_TIMESTAMP_TZ"),
            Self::Date => f.write_str("DATE"),
            Self::Time => f.write_str("TIME"),
            Self::IntervalCompound => f.write_str("INTERVAL_COMPOUND"),
            Self::IntervalDay => f.write_str("INTERVAL_DAY"),
            Self::IntervalYear => f.write_str("INTERVAL_YEAR"),
            Self::Uuid => f.write_str("UUID"),
            Self::Decimal => f.write_str("DECIMAL"),
            Self::Struct => f.write_str("STRUCT"),
            Self::List => f.write_str("LIST"),
            Self::Map => f.write_str("MAP"),
        }
    }
}
impl ::std::str::FromStr for DialectSupportedTypesItemVariant0 {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "BOOL" => Ok(Self::Bool),
            "I8" => Ok(Self::I8),
            "I16" => Ok(Self::I16),
            "I32" => Ok(Self::I32),
            "I64" => Ok(Self::I64),
            "FP32" => Ok(Self::Fp32),
            "FP64" => Ok(Self::Fp64),
            "BINARY" => Ok(Self::Binary),
            "FIXED_BINARY" => Ok(Self::FixedBinary),
            "STRING" => Ok(Self::String),
            "VARCHAR" => Ok(Self::Varchar),
            "FIXED_CHAR" => Ok(Self::FixedChar),
            "PRECISION_TIME" => Ok(Self::PrecisionTime),
            "PRECISION_TIMESTAMP" => Ok(Self::PrecisionTimestamp),
            "PRECISION_TIMESTAMP_TZ" => Ok(Self::PrecisionTimestampTz),
            "DATE" => Ok(Self::Date),
            "TIME" => Ok(Self::Time),
            "INTERVAL_COMPOUND" => Ok(Self::IntervalCompound),
            "INTERVAL_DAY" => Ok(Self::IntervalDay),
            "INTERVAL_YEAR" => Ok(Self::IntervalYear),
            "UUID" => Ok(Self::Uuid),
            "DECIMAL" => Ok(Self::Decimal),
            "STRUCT" => Ok(Self::Struct),
            "LIST" => Ok(Self::List),
            "MAP" => Ok(Self::Map),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DialectSupportedTypesItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for DialectSupportedTypesItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for DialectSupportedTypesItemVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`DynamicParameterExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "DYNAMIC_PARAMETER"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct DynamicParameterExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&DynamicParameterExpression> for DynamicParameterExpression {
    fn from(value: &DynamicParameterExpression) -> Self {
        value.clone()
    }
}
impl DynamicParameterExpression {
    pub fn builder() -> builder::DynamicParameterExpression {
        Default::default()
    }
}
///`ExchangeRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "kinds": {
///      "description": "The list of valid exchanges an exchange relation can perform.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "SCATTER_BY_FIELDS",
///          "SINGLE_TARGET",
///          "MULTI_TARGET",
///          "ROUND_ROBIN",
///          "BROADCAST"
///        ]
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "EXCHANGE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExchangeRelation {
    ///The list of valid exchanges an exchange relation can perform.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub kinds: ::std::option::Option<Vec<ExchangeRelationKindsItem>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ExchangeRelation> for ExchangeRelation {
    fn from(value: &ExchangeRelation) -> Self {
        value.clone()
    }
}
impl ExchangeRelation {
    pub fn builder() -> builder::ExchangeRelation {
        Default::default()
    }
}
///`ExchangeRelationKindsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "SCATTER_BY_FIELDS",
///    "SINGLE_TARGET",
///    "MULTI_TARGET",
///    "ROUND_ROBIN",
///    "BROADCAST"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ExchangeRelationKindsItem {
    #[serde(rename = "SCATTER_BY_FIELDS")]
    ScatterByFields,
    #[serde(rename = "SINGLE_TARGET")]
    SingleTarget,
    #[serde(rename = "MULTI_TARGET")]
    MultiTarget,
    #[serde(rename = "ROUND_ROBIN")]
    RoundRobin,
    #[serde(rename = "BROADCAST")]
    Broadcast,
}
impl ::std::convert::From<&Self> for ExchangeRelationKindsItem {
    fn from(value: &ExchangeRelationKindsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExchangeRelationKindsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ScatterByFields => f.write_str("SCATTER_BY_FIELDS"),
            Self::SingleTarget => f.write_str("SINGLE_TARGET"),
            Self::MultiTarget => f.write_str("MULTI_TARGET"),
            Self::RoundRobin => f.write_str("ROUND_ROBIN"),
            Self::Broadcast => f.write_str("BROADCAST"),
        }
    }
}
impl ::std::str::FromStr for ExchangeRelationKindsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SCATTER_BY_FIELDS" => Ok(Self::ScatterByFields),
            "SINGLE_TARGET" => Ok(Self::SingleTarget),
            "MULTI_TARGET" => Ok(Self::MultiTarget),
            "ROUND_ROBIN" => Ok(Self::RoundRobin),
            "BROADCAST" => Ok(Self::Broadcast),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExchangeRelationKindsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExchangeRelationKindsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExchangeRelationKindsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ExpandRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "field_types": {
///      "description": "The valid methods that control how an expand relation behaves.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "SWITCHING_FIELD",
///          "CONSTANT_FIELD"
///        ]
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "EXPAND"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExpandRelation {
    ///The valid methods that control how an expand relation behaves.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub field_types: ::std::option::Option<Vec<ExpandRelationFieldTypesItem>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ExpandRelation> for ExpandRelation {
    fn from(value: &ExpandRelation) -> Self {
        value.clone()
    }
}
impl ExpandRelation {
    pub fn builder() -> builder::ExpandRelation {
        Default::default()
    }
}
///`ExpandRelationFieldTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "SWITCHING_FIELD",
///    "CONSTANT_FIELD"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ExpandRelationFieldTypesItem {
    #[serde(rename = "SWITCHING_FIELD")]
    SwitchingField,
    #[serde(rename = "CONSTANT_FIELD")]
    ConstantField,
}
impl ::std::convert::From<&Self> for ExpandRelationFieldTypesItem {
    fn from(value: &ExpandRelationFieldTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ExpandRelationFieldTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::SwitchingField => f.write_str("SWITCHING_FIELD"),
            Self::ConstantField => f.write_str("CONSTANT_FIELD"),
        }
    }
}
impl ::std::str::FromStr for ExpandRelationFieldTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SWITCHING_FIELD" => Ok(Self::SwitchingField),
            "CONSTANT_FIELD" => Ok(Self::ConstantField),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ExpandRelationFieldTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ExpandRelationFieldTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ExpandRelationFieldTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ExtensionLeafRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "message_types": {
///      "description": "The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.\n",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "EXTENSION_LEAF"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExtensionLeafRelation {
    /**The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message_types: ::std::option::Option<Vec<::std::string::String>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ExtensionLeafRelation> for ExtensionLeafRelation {
    fn from(value: &ExtensionLeafRelation) -> Self {
        value.clone()
    }
}
impl ExtensionLeafRelation {
    pub fn builder() -> builder::ExtensionLeafRelation {
        Default::default()
    }
}
///`ExtensionMultiRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "message_types": {
///      "description": "The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.\n",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "EXTENSION_MULTI"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExtensionMultiRelation {
    /**The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message_types: ::std::option::Option<Vec<::std::string::String>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ExtensionMultiRelation> for ExtensionMultiRelation {
    fn from(value: &ExtensionMultiRelation) -> Self {
        value.clone()
    }
}
impl ExtensionMultiRelation {
    pub fn builder() -> builder::ExtensionMultiRelation {
        Default::default()
    }
}
///`ExtensionSingleRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "message_types": {
///      "description": "The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.\n",
///      "type": "array",
///      "items": {
///        "type": "string"
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "EXTENSION_SINGLE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ExtensionSingleRelation {
    /**The list of supported message types.  Use the URI (e.g. type.googleapis.com/google.profile.Person) that is related to the detail field.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub message_types: ::std::option::Option<Vec<::std::string::String>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ExtensionSingleRelation> for ExtensionSingleRelation {
    fn from(value: &ExtensionSingleRelation) -> Self {
        value.clone()
    }
}
impl ExtensionSingleRelation {
    pub fn builder() -> builder::ExtensionSingleRelation {
        Default::default()
    }
}
///`FetchRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "FETCH"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FetchRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&FetchRelation> for FetchRelation {
    fn from(value: &FetchRelation) -> Self {
        value.clone()
    }
}
impl FetchRelation {
    pub fn builder() -> builder::FetchRelation {
        Default::default()
    }
}
///`FilterRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "FILTER"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct FilterRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&FilterRelation> for FilterRelation {
    fn from(value: &FilterRelation) -> Self {
        value.clone()
    }
}
impl FilterRelation {
    pub fn builder() -> builder::FilterRelation {
        Default::default()
    }
}
///`HashJoinRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "join_types": {
///      "$ref": "#/definitions/join_types"
///    },
///    "relation": {
///      "const": "HASH_JOIN"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct HashJoinRelation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub join_types: ::std::option::Option<JoinTypes>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&HashJoinRelation> for HashJoinRelation {
    fn from(value: &HashJoinRelation) -> Self {
        value.clone()
    }
}
impl HashJoinRelation {
    pub fn builder() -> builder::HashJoinRelation {
        Default::default()
    }
}
///`IfThenExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "IF_THEN"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IfThenExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&IfThenExpression> for IfThenExpression {
    fn from(value: &IfThenExpression) -> Self {
        value.clone()
    }
}
impl IfThenExpression {
    pub fn builder() -> builder::IfThenExpression {
        Default::default()
    }
}
///`JoinRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "join_types": {
///      "$ref": "#/definitions/join_types"
///    },
///    "relation": {
///      "const": "JOIN"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct JoinRelation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub join_types: ::std::option::Option<JoinTypes>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&JoinRelation> for JoinRelation {
    fn from(value: &JoinRelation) -> Self {
        value.clone()
    }
}
impl JoinRelation {
    pub fn builder() -> builder::JoinRelation {
        Default::default()
    }
}
///The list of possible join types used by join relations.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The list of possible join types used by join relations.",
///  "type": "array",
///  "items": {
///    "enum": [
///      "INNER",
///      "OUTER",
///      "LEFT",
///      "RIGHT",
///      "LEFT_SEMI",
///      "RIGHT_SEMI",
///      "LEFT_ANTI",
///      "RIGHT_ANTI",
///      "LEFT_SINGLE",
///      "RIGHT_SINGLE",
///      "LEFT_MARK",
///      "RIGHT_MARK"
///    ]
///  },
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct JoinTypes(pub Vec<JoinTypesItem>);
impl ::std::ops::Deref for JoinTypes {
    type Target = Vec<JoinTypesItem>;
    fn deref(&self) -> &Vec<JoinTypesItem> {
        &self.0
    }
}
impl ::std::convert::From<JoinTypes> for Vec<JoinTypesItem> {
    fn from(value: JoinTypes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&JoinTypes> for JoinTypes {
    fn from(value: &JoinTypes) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<JoinTypesItem>> for JoinTypes {
    fn from(value: Vec<JoinTypesItem>) -> Self {
        Self(value)
    }
}
///`JoinTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "INNER",
///    "OUTER",
///    "LEFT",
///    "RIGHT",
///    "LEFT_SEMI",
///    "RIGHT_SEMI",
///    "LEFT_ANTI",
///    "RIGHT_ANTI",
///    "LEFT_SINGLE",
///    "RIGHT_SINGLE",
///    "LEFT_MARK",
///    "RIGHT_MARK"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum JoinTypesItem {
    #[serde(rename = "INNER")]
    Inner,
    #[serde(rename = "OUTER")]
    Outer,
    #[serde(rename = "LEFT")]
    Left,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "LEFT_SEMI")]
    LeftSemi,
    #[serde(rename = "RIGHT_SEMI")]
    RightSemi,
    #[serde(rename = "LEFT_ANTI")]
    LeftAnti,
    #[serde(rename = "RIGHT_ANTI")]
    RightAnti,
    #[serde(rename = "LEFT_SINGLE")]
    LeftSingle,
    #[serde(rename = "RIGHT_SINGLE")]
    RightSingle,
    #[serde(rename = "LEFT_MARK")]
    LeftMark,
    #[serde(rename = "RIGHT_MARK")]
    RightMark,
}
impl ::std::convert::From<&Self> for JoinTypesItem {
    fn from(value: &JoinTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for JoinTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inner => f.write_str("INNER"),
            Self::Outer => f.write_str("OUTER"),
            Self::Left => f.write_str("LEFT"),
            Self::Right => f.write_str("RIGHT"),
            Self::LeftSemi => f.write_str("LEFT_SEMI"),
            Self::RightSemi => f.write_str("RIGHT_SEMI"),
            Self::LeftAnti => f.write_str("LEFT_ANTI"),
            Self::RightAnti => f.write_str("RIGHT_ANTI"),
            Self::LeftSingle => f.write_str("LEFT_SINGLE"),
            Self::RightSingle => f.write_str("RIGHT_SINGLE"),
            Self::LeftMark => f.write_str("LEFT_MARK"),
            Self::RightMark => f.write_str("RIGHT_MARK"),
        }
    }
}
impl ::std::str::FromStr for JoinTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "INNER" => Ok(Self::Inner),
            "OUTER" => Ok(Self::Outer),
            "LEFT" => Ok(Self::Left),
            "RIGHT" => Ok(Self::Right),
            "LEFT_SEMI" => Ok(Self::LeftSemi),
            "RIGHT_SEMI" => Ok(Self::RightSemi),
            "LEFT_ANTI" => Ok(Self::LeftAnti),
            "RIGHT_ANTI" => Ok(Self::RightAnti),
            "LEFT_SINGLE" => Ok(Self::LeftSingle),
            "RIGHT_SINGLE" => Ok(Self::RightSingle),
            "LEFT_MARK" => Ok(Self::LeftMark),
            "RIGHT_MARK" => Ok(Self::RightMark),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for JoinTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for JoinTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for JoinTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`LiteralExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "LITERAL"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LiteralExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&LiteralExpression> for LiteralExpression {
    fn from(value: &LiteralExpression) -> Self {
        value.clone()
    }
}
impl LiteralExpression {
    pub fn builder() -> builder::LiteralExpression {
        Default::default()
    }
}
///`MergeJoinRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "join_types": {
///      "$ref": "#/definitions/join_types"
///    },
///    "relation": {
///      "const": "MERGE_JOIN"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MergeJoinRelation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub join_types: ::std::option::Option<JoinTypes>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&MergeJoinRelation> for MergeJoinRelation {
    fn from(value: &MergeJoinRelation) -> Self {
        value.clone()
    }
}
impl MergeJoinRelation {
    pub fn builder() -> builder::MergeJoinRelation {
        Default::default()
    }
}
///`MultiOrListExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "MULTI_OR_LIST"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct MultiOrListExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&MultiOrListExpression> for MultiOrListExpression {
    fn from(value: &MultiOrListExpression) -> Self {
        value.clone()
    }
}
impl MultiOrListExpression {
    pub fn builder() -> builder::MultiOrListExpression {
        Default::default()
    }
}
///`NestedExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "NESTED"
///    },
///    "nested_types": {
///      "description": "The valid nested types.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "STRUCT",
///          "LIST",
///          "MAP"
///        ]
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NestedExpression {
    pub expression: ::serde_json::Value,
    ///The valid nested types.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub nested_types: ::std::vec::Vec<NestedExpressionNestedTypesItem>,
}
impl ::std::convert::From<&NestedExpression> for NestedExpression {
    fn from(value: &NestedExpression) -> Self {
        value.clone()
    }
}
impl NestedExpression {
    pub fn builder() -> builder::NestedExpression {
        Default::default()
    }
}
///`NestedExpressionNestedTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "STRUCT",
///    "LIST",
///    "MAP"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum NestedExpressionNestedTypesItem {
    #[serde(rename = "STRUCT")]
    Struct,
    #[serde(rename = "LIST")]
    List,
    #[serde(rename = "MAP")]
    Map,
}
impl ::std::convert::From<&Self> for NestedExpressionNestedTypesItem {
    fn from(value: &NestedExpressionNestedTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NestedExpressionNestedTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Struct => f.write_str("STRUCT"),
            Self::List => f.write_str("LIST"),
            Self::Map => f.write_str("MAP"),
        }
    }
}
impl ::std::str::FromStr for NestedExpressionNestedTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "STRUCT" => Ok(Self::Struct),
            "LIST" => Ok(Self::List),
            "MAP" => Ok(Self::Map),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NestedExpressionNestedTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for NestedExpressionNestedTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NestedExpressionNestedTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`NestedLoopJoinRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "join_types": {
///      "$ref": "#/definitions/join_types"
///    },
///    "relation": {
///      "const": "NESTED_LOOP_JOIN"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct NestedLoopJoinRelation {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub join_types: ::std::option::Option<JoinTypes>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&NestedLoopJoinRelation> for NestedLoopJoinRelation {
    fn from(value: &NestedLoopJoinRelation) -> Self {
        value.clone()
    }
}
impl NestedLoopJoinRelation {
    pub fn builder() -> builder::NestedLoopJoinRelation {
        Default::default()
    }
}
///`ProjectRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "PROJECT"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ProjectRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ProjectRelation> for ProjectRelation {
    fn from(value: &ProjectRelation) -> Self {
        value.clone()
    }
}
impl ProjectRelation {
    pub fn builder() -> builder::ProjectRelation {
        Default::default()
    }
}
///`ReadRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "read_types": {
///      "description": "The list of possible read types used by the read relation.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "VIRTUAL_TABLE",
///          "LOCAL_FILES",
///          "NAMED_TABLE",
///          "EXTENSION_TABLE",
///          "ICEBERG_TABLE"
///        ]
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "READ"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReadRelation {
    ///The list of possible read types used by the read relation.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub read_types: ::std::option::Option<Vec<ReadRelationReadTypesItem>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ReadRelation> for ReadRelation {
    fn from(value: &ReadRelation) -> Self {
        value.clone()
    }
}
impl ReadRelation {
    pub fn builder() -> builder::ReadRelation {
        Default::default()
    }
}
///`ReadRelationReadTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "VIRTUAL_TABLE",
///    "LOCAL_FILES",
///    "NAMED_TABLE",
///    "EXTENSION_TABLE",
///    "ICEBERG_TABLE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum ReadRelationReadTypesItem {
    #[serde(rename = "VIRTUAL_TABLE")]
    VirtualTable,
    #[serde(rename = "LOCAL_FILES")]
    LocalFiles,
    #[serde(rename = "NAMED_TABLE")]
    NamedTable,
    #[serde(rename = "EXTENSION_TABLE")]
    ExtensionTable,
    #[serde(rename = "ICEBERG_TABLE")]
    IcebergTable,
}
impl ::std::convert::From<&Self> for ReadRelationReadTypesItem {
    fn from(value: &ReadRelationReadTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for ReadRelationReadTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::VirtualTable => f.write_str("VIRTUAL_TABLE"),
            Self::LocalFiles => f.write_str("LOCAL_FILES"),
            Self::NamedTable => f.write_str("NAMED_TABLE"),
            Self::ExtensionTable => f.write_str("EXTENSION_TABLE"),
            Self::IcebergTable => f.write_str("ICEBERG_TABLE"),
        }
    }
}
impl ::std::str::FromStr for ReadRelationReadTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "VIRTUAL_TABLE" => Ok(Self::VirtualTable),
            "LOCAL_FILES" => Ok(Self::LocalFiles),
            "NAMED_TABLE" => Ok(Self::NamedTable),
            "EXTENSION_TABLE" => Ok(Self::ExtensionTable),
            "ICEBERG_TABLE" => Ok(Self::IcebergTable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ReadRelationReadTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ReadRelationReadTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ReadRelationReadTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ReferenceRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "REFERENCE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ReferenceRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&ReferenceRelation> for ReferenceRelation {
    fn from(value: &ReferenceRelation) -> Self {
        value.clone()
    }
}
impl ReferenceRelation {
    pub fn builder() -> builder::ReferenceRelation {
        Default::default()
    }
}
///`ScalarFunctionExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "SCALAR_FUNCTION"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScalarFunctionExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&ScalarFunctionExpression> for ScalarFunctionExpression {
    fn from(value: &ScalarFunctionExpression) -> Self {
        value.clone()
    }
}
impl ScalarFunctionExpression {
    pub fn builder() -> builder::ScalarFunctionExpression {
        Default::default()
    }
}
///`SelectionExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "SELECTION"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SelectionExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&SelectionExpression> for SelectionExpression {
    fn from(value: &SelectionExpression) -> Self {
        value.clone()
    }
}
impl SelectionExpression {
    pub fn builder() -> builder::SelectionExpression {
        Default::default()
    }
}
///`SetRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "operations": {
///      "description": "The list of valid set operations used by a set relation.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "MINUS_PRIMARY",
///          "MINUS_PRIMARY_ALL",
///          "MINUS_MULTISET",
///          "INTERSECTION_PRIMARY",
///          "INTERSECTION_MULTISET",
///          "INTERSECTION_MULTISET_ALL",
///          "UNION_DISTINCT",
///          "UNION_ALL"
///        ]
///      },
///      "uniqueItems": true
///    },
///    "relation": {
///      "const": "SET"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SetRelation {
    ///The list of valid set operations used by a set relation.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub operations: ::std::option::Option<Vec<SetRelationOperationsItem>>,
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&SetRelation> for SetRelation {
    fn from(value: &SetRelation) -> Self {
        value.clone()
    }
}
impl SetRelation {
    pub fn builder() -> builder::SetRelation {
        Default::default()
    }
}
///`SetRelationOperationsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "MINUS_PRIMARY",
///    "MINUS_PRIMARY_ALL",
///    "MINUS_MULTISET",
///    "INTERSECTION_PRIMARY",
///    "INTERSECTION_MULTISET",
///    "INTERSECTION_MULTISET_ALL",
///    "UNION_DISTINCT",
///    "UNION_ALL"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SetRelationOperationsItem {
    #[serde(rename = "MINUS_PRIMARY")]
    MinusPrimary,
    #[serde(rename = "MINUS_PRIMARY_ALL")]
    MinusPrimaryAll,
    #[serde(rename = "MINUS_MULTISET")]
    MinusMultiset,
    #[serde(rename = "INTERSECTION_PRIMARY")]
    IntersectionPrimary,
    #[serde(rename = "INTERSECTION_MULTISET")]
    IntersectionMultiset,
    #[serde(rename = "INTERSECTION_MULTISET_ALL")]
    IntersectionMultisetAll,
    #[serde(rename = "UNION_DISTINCT")]
    UnionDistinct,
    #[serde(rename = "UNION_ALL")]
    UnionAll,
}
impl ::std::convert::From<&Self> for SetRelationOperationsItem {
    fn from(value: &SetRelationOperationsItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SetRelationOperationsItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::MinusPrimary => f.write_str("MINUS_PRIMARY"),
            Self::MinusPrimaryAll => f.write_str("MINUS_PRIMARY_ALL"),
            Self::MinusMultiset => f.write_str("MINUS_MULTISET"),
            Self::IntersectionPrimary => f.write_str("INTERSECTION_PRIMARY"),
            Self::IntersectionMultiset => f.write_str("INTERSECTION_MULTISET"),
            Self::IntersectionMultisetAll => f.write_str("INTERSECTION_MULTISET_ALL"),
            Self::UnionDistinct => f.write_str("UNION_DISTINCT"),
            Self::UnionAll => f.write_str("UNION_ALL"),
        }
    }
}
impl ::std::str::FromStr for SetRelationOperationsItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MINUS_PRIMARY" => Ok(Self::MinusPrimary),
            "MINUS_PRIMARY_ALL" => Ok(Self::MinusPrimaryAll),
            "MINUS_MULTISET" => Ok(Self::MinusMultiset),
            "INTERSECTION_PRIMARY" => Ok(Self::IntersectionPrimary),
            "INTERSECTION_MULTISET" => Ok(Self::IntersectionMultiset),
            "INTERSECTION_MULTISET_ALL" => Ok(Self::IntersectionMultisetAll),
            "UNION_DISTINCT" => Ok(Self::UnionDistinct),
            "UNION_ALL" => Ok(Self::UnionAll),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SetRelationOperationsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SetRelationOperationsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SetRelationOperationsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SingularOrListExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "SINGULAR_OR_LIST"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SingularOrListExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&SingularOrListExpression> for SingularOrListExpression {
    fn from(value: &SingularOrListExpression) -> Self {
        value.clone()
    }
}
impl SingularOrListExpression {
    pub fn builder() -> builder::SingularOrListExpression {
        Default::default()
    }
}
///`SortRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "SORT"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SortRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&SortRelation> for SortRelation {
    fn from(value: &SortRelation) -> Self {
        value.clone()
    }
}
impl SortRelation {
    pub fn builder() -> builder::SortRelation {
        Default::default()
    }
}
///`SubqueryExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "SUBQUERY"
///    },
///    "subquery_types": {
///      "description": "The possible subquery types for a subquery relation.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "SCALAR",
///          "IN_PREDICATE",
///          "SET_PREDICATE",
///          "SET_COMPARISON"
///        ]
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SubqueryExpression {
    pub expression: ::serde_json::Value,
    ///The possible subquery types for a subquery relation.
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub subquery_types: ::std::vec::Vec<SubqueryExpressionSubqueryTypesItem>,
}
impl ::std::convert::From<&SubqueryExpression> for SubqueryExpression {
    fn from(value: &SubqueryExpression) -> Self {
        value.clone()
    }
}
impl SubqueryExpression {
    pub fn builder() -> builder::SubqueryExpression {
        Default::default()
    }
}
///`SubqueryExpressionSubqueryTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "SCALAR",
///    "IN_PREDICATE",
///    "SET_PREDICATE",
///    "SET_COMPARISON"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SubqueryExpressionSubqueryTypesItem {
    #[serde(rename = "SCALAR")]
    Scalar,
    #[serde(rename = "IN_PREDICATE")]
    InPredicate,
    #[serde(rename = "SET_PREDICATE")]
    SetPredicate,
    #[serde(rename = "SET_COMPARISON")]
    SetComparison,
}
impl ::std::convert::From<&Self> for SubqueryExpressionSubqueryTypesItem {
    fn from(value: &SubqueryExpressionSubqueryTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SubqueryExpressionSubqueryTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Scalar => f.write_str("SCALAR"),
            Self::InPredicate => f.write_str("IN_PREDICATE"),
            Self::SetPredicate => f.write_str("SET_PREDICATE"),
            Self::SetComparison => f.write_str("SET_COMPARISON"),
        }
    }
}
impl ::std::str::FromStr for SubqueryExpressionSubqueryTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "SCALAR" => Ok(Self::Scalar),
            "IN_PREDICATE" => Ok(Self::InPredicate),
            "SET_PREDICATE" => Ok(Self::SetPredicate),
            "SET_COMPARISON" => Ok(Self::SetComparison),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SubqueryExpressionSubqueryTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SubqueryExpressionSubqueryTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SubqueryExpressionSubqueryTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SwitchExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "SWITCH"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SwitchExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&SwitchExpression> for SwitchExpression {
    fn from(value: &SwitchExpression) -> Self {
        value.clone()
    }
}
impl SwitchExpression {
    pub fn builder() -> builder::SwitchExpression {
        Default::default()
    }
}
/**System specific metadata for use in mapping Substrait to and from Common metadata fields are defined for use across dialects. Individual dialects may include additional metadata to serve their purposes.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "System specific metadata for use in mapping Substrait to and from Common metadata fields are defined for use across dialects. Individual dialects may include additional metadata to serve their purposes.\n",
///  "type": "object",
///  "properties": {
///    "name": {
///      "description": "The name for this function in the system modelled by the dialect. For example, \"+\" might be the system name for \"add\".\n",
///      "type": "string"
///    },
///    "notation": {
///      "description": "The notation used to invoke this function in the system modelled by the dialect. For example, a function with system_name \"+\" could be invoked as follow: * INFIX:    \"a + b\" * POSTFIX:  \"a b +\", * PREFIX:   \"+ a b\" * FUNCTION: \"func(a, b)\"\n",
///      "default": "FUNCTION",
///      "type": "string",
///      "enum": [
///        "INFIX",
///        "POSTFIX",
///        "PREFIX",
///        "FUNCTION"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SystemFunctionMetadata {
    /**The name for this function in the system modelled by the dialect. For example, "+" might be the system name for "add".
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    /**The notation used to invoke this function in the system modelled by the dialect. For example, a function with system_name "+" could be invoked as follow: * INFIX:    "a + b" * POSTFIX:  "a b +", * PREFIX:   "+ a b" * FUNCTION: "func(a, b)"
*/
    #[serde(default = "defaults::system_function_metadata_notation")]
    pub notation: SystemFunctionMetadataNotation,
}
impl ::std::convert::From<&SystemFunctionMetadata> for SystemFunctionMetadata {
    fn from(value: &SystemFunctionMetadata) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SystemFunctionMetadata {
    fn default() -> Self {
        Self {
            name: Default::default(),
            notation: defaults::system_function_metadata_notation(),
        }
    }
}
impl SystemFunctionMetadata {
    pub fn builder() -> builder::SystemFunctionMetadata {
        Default::default()
    }
}
/**The notation used to invoke this function in the system modelled by the dialect. For example, a function with system_name "+" could be invoked as follow: * INFIX:    "a + b" * POSTFIX:  "a b +", * PREFIX:   "+ a b" * FUNCTION: "func(a, b)"
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "The notation used to invoke this function in the system modelled by the dialect. For example, a function with system_name \"+\" could be invoked as follow: * INFIX:    \"a + b\" * POSTFIX:  \"a b +\", * PREFIX:   \"+ a b\" * FUNCTION: \"func(a, b)\"\n",
///  "default": "FUNCTION",
///  "type": "string",
///  "enum": [
///    "INFIX",
///    "POSTFIX",
///    "PREFIX",
///    "FUNCTION"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SystemFunctionMetadataNotation {
    #[serde(rename = "INFIX")]
    Infix,
    #[serde(rename = "POSTFIX")]
    Postfix,
    #[serde(rename = "PREFIX")]
    Prefix,
    #[serde(rename = "FUNCTION")]
    Function,
}
impl ::std::convert::From<&Self> for SystemFunctionMetadataNotation {
    fn from(value: &SystemFunctionMetadataNotation) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SystemFunctionMetadataNotation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Infix => f.write_str("INFIX"),
            Self::Postfix => f.write_str("POSTFIX"),
            Self::Prefix => f.write_str("PREFIX"),
            Self::Function => f.write_str("FUNCTION"),
        }
    }
}
impl ::std::str::FromStr for SystemFunctionMetadataNotation {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "INFIX" => Ok(Self::Infix),
            "POSTFIX" => Ok(Self::Postfix),
            "PREFIX" => Ok(Self::Prefix),
            "FUNCTION" => Ok(Self::Function),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SystemFunctionMetadataNotation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SystemFunctionMetadataNotation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SystemFunctionMetadataNotation {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for SystemFunctionMetadataNotation {
    fn default() -> Self {
        SystemFunctionMetadataNotation::Function
    }
}
/**System specific metadata for use in mapping Substrait to and from the system representation. Common metadata fields are defined for use across dialects. Individual dialects may include additional metadata to serve their purposes.
*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "System specific metadata for use in mapping Substrait to and from the system representation. Common metadata fields are defined for use across dialects. Individual dialects may include additional metadata to serve their purposes.\n",
///  "type": "object",
///  "properties": {
///    "name": {
///      "description": "The name for this type in the system modelled by the dialect.\n",
///      "type": "string"
///    },
///    "supported_as_column": {
///      "description": "Whether this type can be used as a column in a table.  Types listed here are always supported as literals.\n",
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SystemTypeMetadata {
    /**The name for this type in the system modelled by the dialect.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    /**Whether this type can be used as a column in a table.  Types listed here are always supported as literals.
*/
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub supported_as_column: ::std::option::Option<bool>,
}
impl ::std::convert::From<&SystemTypeMetadata> for SystemTypeMetadata {
    fn from(value: &SystemTypeMetadata) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for SystemTypeMetadata {
    fn default() -> Self {
        Self {
            name: Default::default(),
            supported_as_column: Default::default(),
        }
    }
}
impl SystemTypeMetadata {
    pub fn builder() -> builder::SystemTypeMetadata {
        Default::default()
    }
}
///Dialect properties shared by all types
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "description": "Dialect properties shared by all types",
///  "type": "object",
///  "properties": {
///    "system_metadata": {
///      "description": "system specific metadata for the type",
///      "type": "object",
///      "$ref": "#/definitions/system_type_metadata"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TypeBase {
    ///system specific metadata for the type
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
}
impl ::std::convert::From<&TypeBase> for TypeBase {
    fn from(value: &TypeBase) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for TypeBase {
    fn default() -> Self {
        Self {
            system_metadata: Default::default(),
        }
    }
}
impl TypeBase {
    pub fn builder() -> builder::TypeBase {
        Default::default()
    }
}
///`TypeBinary`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "BINARY"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeBinary {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeBinary> for TypeBinary {
    fn from(value: &TypeBinary) -> Self {
        value.clone()
    }
}
impl TypeBinary {
    pub fn builder() -> builder::TypeBinary {
        Default::default()
    }
}
///`TypeBool`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "BOOL"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeBool {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeBool> for TypeBool {
    fn from(value: &TypeBool) -> Self {
        value.clone()
    }
}
impl TypeBool {
    pub fn builder() -> builder::TypeBool {
        Default::default()
    }
}
///`TypeDate`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "DATE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeDate {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeDate> for TypeDate {
    fn from(value: &TypeDate) -> Self {
        value.clone()
    }
}
impl TypeDate {
    pub fn builder() -> builder::TypeDate {
        Default::default()
    }
}
///`TypeDecimal`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "DECIMAL"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeDecimal {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeDecimal> for TypeDecimal {
    fn from(value: &TypeDecimal) -> Self {
        value.clone()
    }
}
impl TypeDecimal {
    pub fn builder() -> builder::TypeDecimal {
        Default::default()
    }
}
///`TypeFixedBinary`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "FIXED_BINARY"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeFixedBinary {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeFixedBinary> for TypeFixedBinary {
    fn from(value: &TypeFixedBinary) -> Self {
        value.clone()
    }
}
impl TypeFixedBinary {
    pub fn builder() -> builder::TypeFixedBinary {
        Default::default()
    }
}
///`TypeFixedChar`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "FIXED_CHAR"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeFixedChar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeFixedChar> for TypeFixedChar {
    fn from(value: &TypeFixedChar) -> Self {
        value.clone()
    }
}
impl TypeFixedChar {
    pub fn builder() -> builder::TypeFixedChar {
        Default::default()
    }
}
///`TypeFp32`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "FP32"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeFp32 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeFp32> for TypeFp32 {
    fn from(value: &TypeFp32) -> Self {
        value.clone()
    }
}
impl TypeFp32 {
    pub fn builder() -> builder::TypeFp32 {
        Default::default()
    }
}
///`TypeFp64`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "FP64"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeFp64 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeFp64> for TypeFp64 {
    fn from(value: &TypeFp64) -> Self {
        value.clone()
    }
}
impl TypeFp64 {
    pub fn builder() -> builder::TypeFp64 {
        Default::default()
    }
}
///`TypeI16`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "I16"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeI16 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeI16> for TypeI16 {
    fn from(value: &TypeI16) -> Self {
        value.clone()
    }
}
impl TypeI16 {
    pub fn builder() -> builder::TypeI16 {
        Default::default()
    }
}
///`TypeI32`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "I32"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeI32 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeI32> for TypeI32 {
    fn from(value: &TypeI32) -> Self {
        value.clone()
    }
}
impl TypeI32 {
    pub fn builder() -> builder::TypeI32 {
        Default::default()
    }
}
///`TypeI64`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "I64"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeI64 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeI64> for TypeI64 {
    fn from(value: &TypeI64) -> Self {
        value.clone()
    }
}
impl TypeI64 {
    pub fn builder() -> builder::TypeI64 {
        Default::default()
    }
}
///`TypeI8`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "I8"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeI8 {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeI8> for TypeI8 {
    fn from(value: &TypeI8) -> Self {
        value.clone()
    }
}
impl TypeI8 {
    pub fn builder() -> builder::TypeI8 {
        Default::default()
    }
}
///`TypeIntervalCompound`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "INTERVAL_COMPOUND"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeIntervalCompound {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeIntervalCompound> for TypeIntervalCompound {
    fn from(value: &TypeIntervalCompound) -> Self {
        value.clone()
    }
}
impl TypeIntervalCompound {
    pub fn builder() -> builder::TypeIntervalCompound {
        Default::default()
    }
}
///`TypeIntervalDay`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "INTERVAL_DAY"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeIntervalDay {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeIntervalDay> for TypeIntervalDay {
    fn from(value: &TypeIntervalDay) -> Self {
        value.clone()
    }
}
impl TypeIntervalDay {
    pub fn builder() -> builder::TypeIntervalDay {
        Default::default()
    }
}
///`TypeIntervalYear`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "INTERVAL_YEAR"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeIntervalYear {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeIntervalYear> for TypeIntervalYear {
    fn from(value: &TypeIntervalYear) -> Self {
        value.clone()
    }
}
impl TypeIntervalYear {
    pub fn builder() -> builder::TypeIntervalYear {
        Default::default()
    }
}
///`TypeList`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "LIST"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeList {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeList> for TypeList {
    fn from(value: &TypeList) -> Self {
        value.clone()
    }
}
impl TypeList {
    pub fn builder() -> builder::TypeList {
        Default::default()
    }
}
///`TypeMap`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "MAP"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeMap {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeMap> for TypeMap {
    fn from(value: &TypeMap) -> Self {
        value.clone()
    }
}
impl TypeMap {
    pub fn builder() -> builder::TypeMap {
        Default::default()
    }
}
///`TypePrecisionTime`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "PRECISION_TIME"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypePrecisionTime {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypePrecisionTime> for TypePrecisionTime {
    fn from(value: &TypePrecisionTime) -> Self {
        value.clone()
    }
}
impl TypePrecisionTime {
    pub fn builder() -> builder::TypePrecisionTime {
        Default::default()
    }
}
///`TypePrecisionTimestamp`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "PRECISION_TIMESTAMP"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypePrecisionTimestamp {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypePrecisionTimestamp> for TypePrecisionTimestamp {
    fn from(value: &TypePrecisionTimestamp) -> Self {
        value.clone()
    }
}
impl TypePrecisionTimestamp {
    pub fn builder() -> builder::TypePrecisionTimestamp {
        Default::default()
    }
}
///`TypePrecisionTimestampTz`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "PRECISION_TIMESTAMP_TZ"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypePrecisionTimestampTz {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypePrecisionTimestampTz> for TypePrecisionTimestampTz {
    fn from(value: &TypePrecisionTimestampTz) -> Self {
        value.clone()
    }
}
impl TypePrecisionTimestampTz {
    pub fn builder() -> builder::TypePrecisionTimestampTz {
        Default::default()
    }
}
///`TypeString`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "STRING"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeString {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeString> for TypeString {
    fn from(value: &TypeString) -> Self {
        value.clone()
    }
}
impl TypeString {
    pub fn builder() -> builder::TypeString {
        Default::default()
    }
}
///`TypeStruct`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "STRUCT"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeStruct {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeStruct> for TypeStruct {
    fn from(value: &TypeStruct) -> Self {
        value.clone()
    }
}
impl TypeStruct {
    pub fn builder() -> builder::TypeStruct {
        Default::default()
    }
}
///`TypeTime`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "TIME"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeTime {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeTime> for TypeTime {
    fn from(value: &TypeTime) -> Self {
        value.clone()
    }
}
impl TypeTime {
    pub fn builder() -> builder::TypeTime {
        Default::default()
    }
}
///`TypeUserDefined`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "source",
///    "type"
///  ],
///  "properties": {
///    "name": {
///      "description": "The name of the type as declared in the extension it is defined in.\n",
///      "type": "string"
///    },
///    "source": {
///      "description": "Dependency (alias) where this type is declared",
///      "type": "string"
///    },
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "USER_DEFINED"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeUserDefined {
    /**The name of the type as declared in the extension it is defined in.
*/
    pub name: ::std::string::String,
    ///Dependency (alias) where this type is declared
    pub source: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeUserDefined> for TypeUserDefined {
    fn from(value: &TypeUserDefined) -> Self {
        value.clone()
    }
}
impl TypeUserDefined {
    pub fn builder() -> builder::TypeUserDefined {
        Default::default()
    }
}
///`TypeUuid`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "UUID"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeUuid {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeUuid> for TypeUuid {
    fn from(value: &TypeUuid) -> Self {
        value.clone()
    }
}
impl TypeUuid {
    pub fn builder() -> builder::TypeUuid {
        Default::default()
    }
}
///`TypeVarchar`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "system_metadata": {
///      "$ref": "#/definitions/system_type_metadata"
///    },
///    "type": {
///      "const": "VARCHAR"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct TypeVarchar {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub system_metadata: ::std::option::Option<SystemTypeMetadata>,
    #[serde(rename = "type")]
    pub type_: ::serde_json::Value,
}
impl ::std::convert::From<&TypeVarchar> for TypeVarchar {
    fn from(value: &TypeVarchar) -> Self {
        value.clone()
    }
}
impl TypeVarchar {
    pub fn builder() -> builder::TypeVarchar {
        Default::default()
    }
}
///`UpdateRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "UPDATE"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpdateRelation {
    pub relation: ::serde_json::Value,
}
impl ::std::convert::From<&UpdateRelation> for UpdateRelation {
    fn from(value: &UpdateRelation) -> Self {
        value.clone()
    }
}
impl UpdateRelation {
    pub fn builder() -> builder::UpdateRelation {
        Default::default()
    }
}
///`WindowFunctionExpression`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "expression"
///  ],
///  "properties": {
///    "expression": {
///      "const": "WINDOW_FUNCTION"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WindowFunctionExpression {
    pub expression: ::serde_json::Value,
}
impl ::std::convert::From<&WindowFunctionExpression> for WindowFunctionExpression {
    fn from(value: &WindowFunctionExpression) -> Self {
        value.clone()
    }
}
impl WindowFunctionExpression {
    pub fn builder() -> builder::WindowFunctionExpression {
        Default::default()
    }
}
///`WriteRelation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "relation"
///  ],
///  "properties": {
///    "relation": {
///      "const": "WRITE"
///    },
///    "write_types": {
///      "description": "The list of valid write types a write relation can perform.",
///      "type": "array",
///      "items": {
///        "enum": [
///          "NAMED_TABLE",
///          "EXTENSION_TABLE"
///        ]
///      },
///      "uniqueItems": true
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WriteRelation {
    pub relation: ::serde_json::Value,
    ///The list of valid write types a write relation can perform.
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub write_types: ::std::option::Option<Vec<WriteRelationWriteTypesItem>>,
}
impl ::std::convert::From<&WriteRelation> for WriteRelation {
    fn from(value: &WriteRelation) -> Self {
        value.clone()
    }
}
impl WriteRelation {
    pub fn builder() -> builder::WriteRelation {
        Default::default()
    }
}
///`WriteRelationWriteTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "enum": [
///    "NAMED_TABLE",
///    "EXTENSION_TABLE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WriteRelationWriteTypesItem {
    #[serde(rename = "NAMED_TABLE")]
    NamedTable,
    #[serde(rename = "EXTENSION_TABLE")]
    ExtensionTable,
}
impl ::std::convert::From<&Self> for WriteRelationWriteTypesItem {
    fn from(value: &WriteRelationWriteTypesItem) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WriteRelationWriteTypesItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::NamedTable => f.write_str("NAMED_TABLE"),
            Self::ExtensionTable => f.write_str("EXTENSION_TABLE"),
        }
    }
}
impl ::std::str::FromStr for WriteRelationWriteTypesItem {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NAMED_TABLE" => Ok(Self::NamedTable),
            "EXTENSION_TABLE" => Ok(Self::ExtensionTable),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WriteRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WriteRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WriteRelationWriteTypesItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AggregateRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for AggregateRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl AggregateRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AggregateRelation> for super::AggregateRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AggregateRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::AggregateRelation> for AggregateRelation {
        fn from(value: super::AggregateRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CastExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        failure_options: ::std::result::Result<
            ::std::option::Option<Vec<super::CastExpressionFailureOptionsItem>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CastExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
                failure_options: Ok(Default::default()),
            }
        }
    }
    impl CastExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
        pub fn failure_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::CastExpressionFailureOptionsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.failure_options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for failure_options: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CastExpression> for super::CastExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CastExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
                failure_options: value.failure_options?,
            })
        }
    }
    impl ::std::convert::From<super::CastExpression> for CastExpression {
        fn from(value: super::CastExpression) -> Self {
            Self {
                expression: Ok(value.expression),
                failure_options: Ok(value.failure_options),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ConsistentPartitionWindowRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ConsistentPartitionWindowRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ConsistentPartitionWindowRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ConsistentPartitionWindowRelation>
    for super::ConsistentPartitionWindowRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ConsistentPartitionWindowRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::ConsistentPartitionWindowRelation>
    for ConsistentPartitionWindowRelation {
        fn from(value: super::ConsistentPartitionWindowRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CrossRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for CrossRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl CrossRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<CrossRelation> for super::CrossRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CrossRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::CrossRelation> for CrossRelation {
        fn from(value: super::CrossRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DdlRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        write_types: ::std::result::Result<
            ::std::option::Option<Vec<super::DdlRelationWriteTypesItem>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DdlRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
                write_types: Ok(Default::default()),
            }
        }
    }
    impl DdlRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
        pub fn write_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::DdlRelationWriteTypesItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.write_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for write_types: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DdlRelation> for super::DdlRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DdlRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                relation: value.relation?,
                write_types: value.write_types?,
            })
        }
    }
    impl ::std::convert::From<super::DdlRelation> for DdlRelation {
        fn from(value: super::DdlRelation) -> Self {
            Self {
                relation: Ok(value.relation),
                write_types: Ok(value.write_types),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Dialect {
        dependencies: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        supported_aggregate_functions: ::std::result::Result<
            ::std::vec::Vec<super::DialectFunction>,
            ::std::string::String,
        >,
        supported_expressions: ::std::result::Result<
            ::std::option::Option<Vec<super::DialectSupportedExpressionsItem>>,
            ::std::string::String,
        >,
        supported_relations: ::std::result::Result<
            ::std::option::Option<Vec<super::DialectSupportedRelationsItem>>,
            ::std::string::String,
        >,
        supported_scalar_functions: ::std::result::Result<
            ::std::vec::Vec<super::DialectFunction>,
            ::std::string::String,
        >,
        supported_types: ::std::result::Result<
            ::std::option::Option<Vec<super::DialectSupportedTypesItem>>,
            ::std::string::String,
        >,
        supported_window_functions: ::std::result::Result<
            ::std::vec::Vec<super::DialectFunction>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Dialect {
        fn default() -> Self {
            Self {
                dependencies: Ok(Default::default()),
                name: Ok(Default::default()),
                supported_aggregate_functions: Ok(Default::default()),
                supported_expressions: Ok(Default::default()),
                supported_relations: Ok(Default::default()),
                supported_scalar_functions: Ok(Default::default()),
                supported_types: Ok(Default::default()),
                supported_window_functions: Ok(Default::default()),
            }
        }
    }
    impl Dialect {
        pub fn dependencies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.dependencies = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dependencies: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn supported_aggregate_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DialectFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_aggregate_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_aggregate_functions: {}",
                        e
                    )
                });
            self
        }
        pub fn supported_expressions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::DialectSupportedExpressionsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.supported_expressions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_expressions: {}",
                        e
                    )
                });
            self
        }
        pub fn supported_relations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::DialectSupportedRelationsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.supported_relations = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_relations: {}", e
                    )
                });
            self
        }
        pub fn supported_scalar_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DialectFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_scalar_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_scalar_functions: {}",
                        e
                    )
                });
            self
        }
        pub fn supported_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::DialectSupportedTypesItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.supported_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for supported_types: {}", e)
                });
            self
        }
        pub fn supported_window_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::DialectFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_window_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_window_functions: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<Dialect> for super::Dialect {
        type Error = super::error::ConversionError;
        fn try_from(
            value: Dialect,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                dependencies: value.dependencies?,
                name: value.name?,
                supported_aggregate_functions: value.supported_aggregate_functions?,
                supported_expressions: value.supported_expressions?,
                supported_relations: value.supported_relations?,
                supported_scalar_functions: value.supported_scalar_functions?,
                supported_types: value.supported_types?,
                supported_window_functions: value.supported_window_functions?,
            })
        }
    }
    impl ::std::convert::From<super::Dialect> for Dialect {
        fn from(value: super::Dialect) -> Self {
            Self {
                dependencies: Ok(value.dependencies),
                name: Ok(value.name),
                supported_aggregate_functions: Ok(value.supported_aggregate_functions),
                supported_expressions: Ok(value.supported_expressions),
                supported_relations: Ok(value.supported_relations),
                supported_scalar_functions: Ok(value.supported_scalar_functions),
                supported_types: Ok(value.supported_types),
                supported_window_functions: Ok(value.supported_window_functions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DialectFunction {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        required_options: ::std::result::Result<
            ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            ::std::string::String,
        >,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        supported_impls: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemFunctionMetadata>,
            ::std::string::String,
        >,
        variadic: ::std::result::Result<
            ::std::option::Option<super::DialectFunctionVariadic>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DialectFunction {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                required_options: Ok(Default::default()),
                source: Err("no value supplied for source".to_string()),
                supported_impls: Err(
                    "no value supplied for supported_impls".to_string(),
                ),
                system_metadata: Ok(Default::default()),
                variadic: Ok(Default::default()),
            }
        }
    }
    impl DialectFunction {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn required_options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.required_options = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for required_options: {}", e
                    )
                });
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
        pub fn supported_impls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_impls = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for supported_impls: {}", e)
                });
            self
        }
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SystemFunctionMetadata>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn variadic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::DialectFunctionVariadic>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.variadic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for variadic: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DialectFunction> for super::DialectFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DialectFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                required_options: value.required_options?,
                source: value.source?,
                supported_impls: value.supported_impls?,
                system_metadata: value.system_metadata?,
                variadic: value.variadic?,
            })
        }
    }
    impl ::std::convert::From<super::DialectFunction> for DialectFunction {
        fn from(value: super::DialectFunction) -> Self {
            Self {
                name: Ok(value.name),
                required_options: Ok(value.required_options),
                source: Ok(value.source),
                supported_impls: Ok(value.supported_impls),
                system_metadata: Ok(value.system_metadata),
                variadic: Ok(value.variadic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DialectFunctionVariadic {
        max: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
        min: ::std::result::Result<
            ::std::option::Option<::std::num::NonZeroU64>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for DialectFunctionVariadic {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                min: Ok(Default::default()),
            }
        }
    }
    impl DialectFunctionVariadic {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::num::NonZeroU64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<DialectFunctionVariadic>
    for super::DialectFunctionVariadic {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DialectFunctionVariadic,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                min: value.min?,
            })
        }
    }
    impl ::std::convert::From<super::DialectFunctionVariadic>
    for DialectFunctionVariadic {
        fn from(value: super::DialectFunctionVariadic) -> Self {
            Self {
                max: Ok(value.max),
                min: Ok(value.min),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DynamicParameterExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for DynamicParameterExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl DynamicParameterExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<DynamicParameterExpression>
    for super::DynamicParameterExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: DynamicParameterExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::DynamicParameterExpression>
    for DynamicParameterExpression {
        fn from(value: super::DynamicParameterExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExchangeRelation {
        kinds: ::std::result::Result<
            ::std::option::Option<Vec<super::ExchangeRelationKindsItem>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ExchangeRelation {
        fn default() -> Self {
            Self {
                kinds: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ExchangeRelation {
        pub fn kinds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::ExchangeRelationKindsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.kinds = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for kinds: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ExchangeRelation> for super::ExchangeRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExchangeRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                kinds: value.kinds?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ExchangeRelation> for ExchangeRelation {
        fn from(value: super::ExchangeRelation) -> Self {
            Self {
                kinds: Ok(value.kinds),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExpandRelation {
        field_types: ::std::result::Result<
            ::std::option::Option<Vec<super::ExpandRelationFieldTypesItem>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ExpandRelation {
        fn default() -> Self {
            Self {
                field_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ExpandRelation {
        pub fn field_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::ExpandRelationFieldTypesItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.field_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for field_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ExpandRelation> for super::ExpandRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExpandRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                field_types: value.field_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ExpandRelation> for ExpandRelation {
        fn from(value: super::ExpandRelation) -> Self {
            Self {
                field_types: Ok(value.field_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExtensionLeafRelation {
        message_types: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ExtensionLeafRelation {
        fn default() -> Self {
            Self {
                message_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ExtensionLeafRelation {
        pub fn message_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ExtensionLeafRelation>
    for super::ExtensionLeafRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExtensionLeafRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_types: value.message_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ExtensionLeafRelation> for ExtensionLeafRelation {
        fn from(value: super::ExtensionLeafRelation) -> Self {
            Self {
                message_types: Ok(value.message_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExtensionMultiRelation {
        message_types: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ExtensionMultiRelation {
        fn default() -> Self {
            Self {
                message_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ExtensionMultiRelation {
        pub fn message_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ExtensionMultiRelation>
    for super::ExtensionMultiRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExtensionMultiRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_types: value.message_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ExtensionMultiRelation> for ExtensionMultiRelation {
        fn from(value: super::ExtensionMultiRelation) -> Self {
            Self {
                message_types: Ok(value.message_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ExtensionSingleRelation {
        message_types: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ExtensionSingleRelation {
        fn default() -> Self {
            Self {
                message_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ExtensionSingleRelation {
        pub fn message_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<::std::string::String>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.message_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for message_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ExtensionSingleRelation>
    for super::ExtensionSingleRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ExtensionSingleRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message_types: value.message_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ExtensionSingleRelation>
    for ExtensionSingleRelation {
        fn from(value: super::ExtensionSingleRelation) -> Self {
            Self {
                message_types: Ok(value.message_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FetchRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for FetchRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl FetchRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FetchRelation> for super::FetchRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FetchRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::FetchRelation> for FetchRelation {
        fn from(value: super::FetchRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct FilterRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for FilterRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl FilterRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<FilterRelation> for super::FilterRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: FilterRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::FilterRelation> for FilterRelation {
        fn from(value: super::FilterRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct HashJoinRelation {
        join_types: ::std::result::Result<
            ::std::option::Option<super::JoinTypes>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for HashJoinRelation {
        fn default() -> Self {
            Self {
                join_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl HashJoinRelation {
        pub fn join_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JoinTypes>>,
            T::Error: ::std::fmt::Display,
        {
            self.join_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for join_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<HashJoinRelation> for super::HashJoinRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HashJoinRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                join_types: value.join_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::HashJoinRelation> for HashJoinRelation {
        fn from(value: super::HashJoinRelation) -> Self {
            Self {
                join_types: Ok(value.join_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IfThenExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for IfThenExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl IfThenExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<IfThenExpression> for super::IfThenExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IfThenExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::IfThenExpression> for IfThenExpression {
        fn from(value: super::IfThenExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct JoinRelation {
        join_types: ::std::result::Result<
            ::std::option::Option<super::JoinTypes>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for JoinRelation {
        fn default() -> Self {
            Self {
                join_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl JoinRelation {
        pub fn join_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JoinTypes>>,
            T::Error: ::std::fmt::Display,
        {
            self.join_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for join_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<JoinRelation> for super::JoinRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: JoinRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                join_types: value.join_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::JoinRelation> for JoinRelation {
        fn from(value: super::JoinRelation) -> Self {
            Self {
                join_types: Ok(value.join_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct LiteralExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for LiteralExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl LiteralExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<LiteralExpression> for super::LiteralExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: LiteralExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::LiteralExpression> for LiteralExpression {
        fn from(value: super::LiteralExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MergeJoinRelation {
        join_types: ::std::result::Result<
            ::std::option::Option<super::JoinTypes>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MergeJoinRelation {
        fn default() -> Self {
            Self {
                join_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl MergeJoinRelation {
        pub fn join_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JoinTypes>>,
            T::Error: ::std::fmt::Display,
        {
            self.join_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for join_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<MergeJoinRelation> for super::MergeJoinRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MergeJoinRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                join_types: value.join_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::MergeJoinRelation> for MergeJoinRelation {
        fn from(value: super::MergeJoinRelation) -> Self {
            Self {
                join_types: Ok(value.join_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MultiOrListExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for MultiOrListExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl MultiOrListExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<MultiOrListExpression>
    for super::MultiOrListExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: MultiOrListExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::MultiOrListExpression> for MultiOrListExpression {
        fn from(value: super::MultiOrListExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NestedExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        nested_types: ::std::result::Result<
            ::std::vec::Vec<super::NestedExpressionNestedTypesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for NestedExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
                nested_types: Ok(Default::default()),
            }
        }
    }
    impl NestedExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
        pub fn nested_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::NestedExpressionNestedTypesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.nested_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nested_types: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NestedExpression> for super::NestedExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NestedExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
                nested_types: value.nested_types?,
            })
        }
    }
    impl ::std::convert::From<super::NestedExpression> for NestedExpression {
        fn from(value: super::NestedExpression) -> Self {
            Self {
                expression: Ok(value.expression),
                nested_types: Ok(value.nested_types),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NestedLoopJoinRelation {
        join_types: ::std::result::Result<
            ::std::option::Option<super::JoinTypes>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for NestedLoopJoinRelation {
        fn default() -> Self {
            Self {
                join_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl NestedLoopJoinRelation {
        pub fn join_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::JoinTypes>>,
            T::Error: ::std::fmt::Display,
        {
            self.join_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for join_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<NestedLoopJoinRelation>
    for super::NestedLoopJoinRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: NestedLoopJoinRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                join_types: value.join_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::NestedLoopJoinRelation> for NestedLoopJoinRelation {
        fn from(value: super::NestedLoopJoinRelation) -> Self {
            Self {
                join_types: Ok(value.join_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ProjectRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ProjectRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ProjectRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ProjectRelation> for super::ProjectRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ProjectRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::ProjectRelation> for ProjectRelation {
        fn from(value: super::ProjectRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReadRelation {
        read_types: ::std::result::Result<
            ::std::option::Option<Vec<super::ReadRelationReadTypesItem>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ReadRelation {
        fn default() -> Self {
            Self {
                read_types: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ReadRelation {
        pub fn read_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::ReadRelationReadTypesItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.read_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for read_types: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ReadRelation> for super::ReadRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReadRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                read_types: value.read_types?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::ReadRelation> for ReadRelation {
        fn from(value: super::ReadRelation) -> Self {
            Self {
                read_types: Ok(value.read_types),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ReferenceRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ReferenceRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl ReferenceRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ReferenceRelation> for super::ReferenceRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ReferenceRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::ReferenceRelation> for ReferenceRelation {
        fn from(value: super::ReferenceRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScalarFunctionExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for ScalarFunctionExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl ScalarFunctionExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ScalarFunctionExpression>
    for super::ScalarFunctionExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScalarFunctionExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::ScalarFunctionExpression>
    for ScalarFunctionExpression {
        fn from(value: super::ScalarFunctionExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SelectionExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for SelectionExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl SelectionExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SelectionExpression> for super::SelectionExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SelectionExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::SelectionExpression> for SelectionExpression {
        fn from(value: super::SelectionExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SetRelation {
        operations: ::std::result::Result<
            ::std::option::Option<Vec<super::SetRelationOperationsItem>>,
            ::std::string::String,
        >,
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for SetRelation {
        fn default() -> Self {
            Self {
                operations: Ok(Default::default()),
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl SetRelation {
        pub fn operations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::SetRelationOperationsItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.operations = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for operations: {}", e)
                });
            self
        }
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SetRelation> for super::SetRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SetRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                operations: value.operations?,
                relation: value.relation?,
            })
        }
    }
    impl ::std::convert::From<super::SetRelation> for SetRelation {
        fn from(value: super::SetRelation) -> Self {
            Self {
                operations: Ok(value.operations),
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SingularOrListExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for SingularOrListExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl SingularOrListExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SingularOrListExpression>
    for super::SingularOrListExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SingularOrListExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::SingularOrListExpression>
    for SingularOrListExpression {
        fn from(value: super::SingularOrListExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SortRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for SortRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl SortRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SortRelation> for super::SortRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SortRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::SortRelation> for SortRelation {
        fn from(value: super::SortRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SubqueryExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        subquery_types: ::std::result::Result<
            ::std::vec::Vec<super::SubqueryExpressionSubqueryTypesItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SubqueryExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
                subquery_types: Ok(Default::default()),
            }
        }
    }
    impl SubqueryExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
        pub fn subquery_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::SubqueryExpressionSubqueryTypesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.subquery_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for subquery_types: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SubqueryExpression> for super::SubqueryExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SubqueryExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
                subquery_types: value.subquery_types?,
            })
        }
    }
    impl ::std::convert::From<super::SubqueryExpression> for SubqueryExpression {
        fn from(value: super::SubqueryExpression) -> Self {
            Self {
                expression: Ok(value.expression),
                subquery_types: Ok(value.subquery_types),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SwitchExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for SwitchExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl SwitchExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SwitchExpression> for super::SwitchExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SwitchExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::SwitchExpression> for SwitchExpression {
        fn from(value: super::SwitchExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemFunctionMetadata {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        notation: ::std::result::Result<
            super::SystemFunctionMetadataNotation,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SystemFunctionMetadata {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                notation: Ok(super::defaults::system_function_metadata_notation()),
            }
        }
    }
    impl SystemFunctionMetadata {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn notation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SystemFunctionMetadataNotation>,
            T::Error: ::std::fmt::Display,
        {
            self.notation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for notation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SystemFunctionMetadata>
    for super::SystemFunctionMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemFunctionMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                notation: value.notation?,
            })
        }
    }
    impl ::std::convert::From<super::SystemFunctionMetadata> for SystemFunctionMetadata {
        fn from(value: super::SystemFunctionMetadata) -> Self {
            Self {
                name: Ok(value.name),
                notation: Ok(value.notation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SystemTypeMetadata {
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        supported_as_column: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SystemTypeMetadata {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                supported_as_column: Ok(Default::default()),
            }
        }
    }
    impl SystemTypeMetadata {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn supported_as_column<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.supported_as_column = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for supported_as_column: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SystemTypeMetadata> for super::SystemTypeMetadata {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SystemTypeMetadata,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                supported_as_column: value.supported_as_column?,
            })
        }
    }
    impl ::std::convert::From<super::SystemTypeMetadata> for SystemTypeMetadata {
        fn from(value: super::SystemTypeMetadata) -> Self {
            Self {
                name: Ok(value.name),
                supported_as_column: Ok(value.supported_as_column),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeBase {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TypeBase {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
            }
        }
    }
    impl TypeBase {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeBase> for super::TypeBase {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeBase,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
            })
        }
    }
    impl ::std::convert::From<super::TypeBase> for TypeBase {
        fn from(value: super::TypeBase) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeBinary {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeBinary {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeBinary {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeBinary> for super::TypeBinary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeBinary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeBinary> for TypeBinary {
        fn from(value: super::TypeBinary) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeBool {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeBool {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeBool {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeBool> for super::TypeBool {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeBool,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeBool> for TypeBool {
        fn from(value: super::TypeBool) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeDate {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeDate {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeDate {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeDate> for super::TypeDate {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeDate,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeDate> for TypeDate {
        fn from(value: super::TypeDate) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeDecimal {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeDecimal {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeDecimal {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeDecimal> for super::TypeDecimal {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeDecimal,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeDecimal> for TypeDecimal {
        fn from(value: super::TypeDecimal) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeFixedBinary {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeFixedBinary {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeFixedBinary {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeFixedBinary> for super::TypeFixedBinary {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeFixedBinary,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeFixedBinary> for TypeFixedBinary {
        fn from(value: super::TypeFixedBinary) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeFixedChar {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeFixedChar {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeFixedChar {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeFixedChar> for super::TypeFixedChar {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeFixedChar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeFixedChar> for TypeFixedChar {
        fn from(value: super::TypeFixedChar) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeFp32 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeFp32 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeFp32 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeFp32> for super::TypeFp32 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeFp32,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeFp32> for TypeFp32 {
        fn from(value: super::TypeFp32) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeFp64 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeFp64 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeFp64 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeFp64> for super::TypeFp64 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeFp64,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeFp64> for TypeFp64 {
        fn from(value: super::TypeFp64) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeI16 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeI16 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeI16 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeI16> for super::TypeI16 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeI16,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeI16> for TypeI16 {
        fn from(value: super::TypeI16) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeI32 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeI32 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeI32 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeI32> for super::TypeI32 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeI32,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeI32> for TypeI32 {
        fn from(value: super::TypeI32) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeI64 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeI64 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeI64 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeI64> for super::TypeI64 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeI64,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeI64> for TypeI64 {
        fn from(value: super::TypeI64) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeI8 {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeI8 {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeI8 {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeI8> for super::TypeI8 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeI8,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeI8> for TypeI8 {
        fn from(value: super::TypeI8) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeIntervalCompound {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeIntervalCompound {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeIntervalCompound {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeIntervalCompound> for super::TypeIntervalCompound {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeIntervalCompound,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeIntervalCompound> for TypeIntervalCompound {
        fn from(value: super::TypeIntervalCompound) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeIntervalDay {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeIntervalDay {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeIntervalDay {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeIntervalDay> for super::TypeIntervalDay {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeIntervalDay,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeIntervalDay> for TypeIntervalDay {
        fn from(value: super::TypeIntervalDay) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeIntervalYear {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeIntervalYear {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeIntervalYear {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeIntervalYear> for super::TypeIntervalYear {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeIntervalYear,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeIntervalYear> for TypeIntervalYear {
        fn from(value: super::TypeIntervalYear) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeList {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeList {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeList {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeList> for super::TypeList {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeList,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeList> for TypeList {
        fn from(value: super::TypeList) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeMap {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeMap {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeMap {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeMap> for super::TypeMap {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeMap,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeMap> for TypeMap {
        fn from(value: super::TypeMap) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypePrecisionTime {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypePrecisionTime {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypePrecisionTime {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypePrecisionTime> for super::TypePrecisionTime {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypePrecisionTime,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypePrecisionTime> for TypePrecisionTime {
        fn from(value: super::TypePrecisionTime) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypePrecisionTimestamp {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypePrecisionTimestamp {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypePrecisionTimestamp {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypePrecisionTimestamp>
    for super::TypePrecisionTimestamp {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypePrecisionTimestamp,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypePrecisionTimestamp> for TypePrecisionTimestamp {
        fn from(value: super::TypePrecisionTimestamp) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypePrecisionTimestampTz {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypePrecisionTimestampTz {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypePrecisionTimestampTz {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypePrecisionTimestampTz>
    for super::TypePrecisionTimestampTz {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypePrecisionTimestampTz,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypePrecisionTimestampTz>
    for TypePrecisionTimestampTz {
        fn from(value: super::TypePrecisionTimestampTz) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeString {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeString {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeString {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeString> for super::TypeString {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeString,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeString> for TypeString {
        fn from(value: super::TypeString) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeStruct {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeStruct {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeStruct {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeStruct> for super::TypeStruct {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeStruct,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeStruct> for TypeStruct {
        fn from(value: super::TypeStruct) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeTime {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeTime {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeTime {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeTime> for super::TypeTime {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeTime,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeTime> for TypeTime {
        fn from(value: super::TypeTime) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeUserDefined {
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        source: ::std::result::Result<::std::string::String, ::std::string::String>,
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeUserDefined {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                source: Err("no value supplied for source".to_string()),
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeUserDefined {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for source: {}", e)
                });
            self
        }
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeUserDefined> for super::TypeUserDefined {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeUserDefined,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                name: value.name?,
                source: value.source?,
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeUserDefined> for TypeUserDefined {
        fn from(value: super::TypeUserDefined) -> Self {
            Self {
                name: Ok(value.name),
                source: Ok(value.source),
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeUuid {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeUuid {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeUuid {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeUuid> for super::TypeUuid {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeUuid,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeUuid> for TypeUuid {
        fn from(value: super::TypeUuid) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeVarchar {
        system_metadata: ::std::result::Result<
            ::std::option::Option<super::SystemTypeMetadata>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for TypeVarchar {
        fn default() -> Self {
            Self {
                system_metadata: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeVarchar {
        pub fn system_metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SystemTypeMetadata>>,
            T::Error: ::std::fmt::Display,
        {
            self.system_metadata = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for system_metadata: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeVarchar> for super::TypeVarchar {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeVarchar,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                system_metadata: value.system_metadata?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeVarchar> for TypeVarchar {
        fn from(value: super::TypeVarchar) -> Self {
            Self {
                system_metadata: Ok(value.system_metadata),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct UpdateRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for UpdateRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
            }
        }
    }
    impl UpdateRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<UpdateRelation> for super::UpdateRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: UpdateRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { relation: value.relation? })
        }
    }
    impl ::std::convert::From<super::UpdateRelation> for UpdateRelation {
        fn from(value: super::UpdateRelation) -> Self {
            Self {
                relation: Ok(value.relation),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WindowFunctionExpression {
        expression: ::std::result::Result<::serde_json::Value, ::std::string::String>,
    }
    impl ::std::default::Default for WindowFunctionExpression {
        fn default() -> Self {
            Self {
                expression: Err("no value supplied for expression".to_string()),
            }
        }
    }
    impl WindowFunctionExpression {
        pub fn expression<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.expression = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for expression: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WindowFunctionExpression>
    for super::WindowFunctionExpression {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WindowFunctionExpression,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expression: value.expression?,
            })
        }
    }
    impl ::std::convert::From<super::WindowFunctionExpression>
    for WindowFunctionExpression {
        fn from(value: super::WindowFunctionExpression) -> Self {
            Self {
                expression: Ok(value.expression),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WriteRelation {
        relation: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        write_types: ::std::result::Result<
            ::std::option::Option<Vec<super::WriteRelationWriteTypesItem>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WriteRelation {
        fn default() -> Self {
            Self {
                relation: Err("no value supplied for relation".to_string()),
                write_types: Ok(Default::default()),
            }
        }
    }
    impl WriteRelation {
        pub fn relation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.relation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for relation: {}", e)
                });
            self
        }
        pub fn write_types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<Vec<super::WriteRelationWriteTypesItem>>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.write_types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for write_types: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WriteRelation> for super::WriteRelation {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WriteRelation,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                relation: value.relation?,
                write_types: value.write_types?,
            })
        }
    }
    impl ::std::convert::From<super::WriteRelation> for WriteRelation {
        fn from(value: super::WriteRelation) -> Self {
            Self {
                relation: Ok(value.relation),
                write_types: Ok(value.write_types),
            }
        }
    }
}
/// Generation of default values for serde.
pub mod defaults {
    pub(super) fn system_function_metadata_notation() -> super::SystemFunctionMetadataNotation {
        super::SystemFunctionMetadataNotation::Function
    }
}

}
#[doc = "Generated types for `http://substrait.io/schemas/simple_extensions`"]
pub mod simple_extensions {
    /// Error types.
pub mod error {
    /// Error from a `TryFrom` or `FromStr` implementation.
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(
            &self,
            f: &mut ::std::fmt::Formatter<'_>,
        ) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
///`AggregateFunction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "impls",
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "impls": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "return"
///        ],
///        "properties": {
///          "args": {
///            "$ref": "#/$defs/arguments"
///          },
///          "decomposable": {
///            "$ref": "#/$defs/decomposable"
///          },
///          "deterministic": {
///            "$ref": "#/$defs/deterministic"
///          },
///          "implementation": {
///            "$ref": "#/$defs/implementation"
///          },
///          "intermediate": {
///            "$ref": "#/$defs/intermediate"
///          },
///          "maxset": {
///            "$ref": "#/$defs/maxset"
///          },
///          "nullability": {
///            "$ref": "#/$defs/nullabilityHandling"
///          },
///          "options": {
///            "$ref": "#/$defs/options"
///          },
///          "ordered": {
///            "$ref": "#/$defs/ordered"
///          },
///          "return": {
///            "$ref": "#/$defs/returnValue"
///          },
///          "sessionDependent": {
///            "$ref": "#/$defs/sessionDependent"
///          },
///          "variadic": {
///            "$ref": "#/$defs/variadicBehavior"
///          }
///        },
///        "additionalProperties": false
///      },
///      "minItems": 1
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AggregateFunction {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub impls: ::std::vec::Vec<AggregateFunctionImplsItem>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&AggregateFunction> for AggregateFunction {
    fn from(value: &AggregateFunction) -> Self {
        value.clone()
    }
}
impl AggregateFunction {
    pub fn builder() -> builder::AggregateFunction {
        Default::default()
    }
}
///`AggregateFunctionImplsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "return"
///  ],
///  "properties": {
///    "args": {
///      "$ref": "#/$defs/arguments"
///    },
///    "decomposable": {
///      "$ref": "#/$defs/decomposable"
///    },
///    "deterministic": {
///      "$ref": "#/$defs/deterministic"
///    },
///    "implementation": {
///      "$ref": "#/$defs/implementation"
///    },
///    "intermediate": {
///      "$ref": "#/$defs/intermediate"
///    },
///    "maxset": {
///      "$ref": "#/$defs/maxset"
///    },
///    "nullability": {
///      "$ref": "#/$defs/nullabilityHandling"
///    },
///    "options": {
///      "$ref": "#/$defs/options"
///    },
///    "ordered": {
///      "$ref": "#/$defs/ordered"
///    },
///    "return": {
///      "$ref": "#/$defs/returnValue"
///    },
///    "sessionDependent": {
///      "$ref": "#/$defs/sessionDependent"
///    },
///    "variadic": {
///      "$ref": "#/$defs/variadicBehavior"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct AggregateFunctionImplsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub args: ::std::option::Option<Arguments>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub decomposable: ::std::option::Option<Decomposable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deterministic: ::std::option::Option<Deterministic>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub implementation: ::std::option::Option<Implementation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intermediate: ::std::option::Option<Intermediate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maxset: ::std::option::Option<Maxset>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nullability: ::std::option::Option<NullabilityHandling>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub options: ::std::option::Option<Options>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ordered: ::std::option::Option<Ordered>,
    #[serde(rename = "return")]
    pub return_: ReturnValue,
    #[serde(
        rename = "sessionDependent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_dependent: ::std::option::Option<SessionDependent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub variadic: ::std::option::Option<VariadicBehavior>,
}
impl ::std::convert::From<&AggregateFunctionImplsItem> for AggregateFunctionImplsItem {
    fn from(value: &AggregateFunctionImplsItem) -> Self {
        value.clone()
    }
}
impl AggregateFunctionImplsItem {
    pub fn builder() -> builder::AggregateFunctionImplsItem {
        Default::default()
    }
}
///`Arguments`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "oneOf": [
///      {
///        "$ref": "#/$defs/enumeration_arg"
///      },
///      {
///        "$ref": "#/$defs/value_arg"
///      },
///      {
///        "$ref": "#/$defs/type_arg"
///      }
///    ]
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Arguments(pub ::std::vec::Vec<ArgumentsItem>);
impl ::std::ops::Deref for Arguments {
    type Target = ::std::vec::Vec<ArgumentsItem>;
    fn deref(&self) -> &::std::vec::Vec<ArgumentsItem> {
        &self.0
    }
}
impl ::std::convert::From<Arguments> for ::std::vec::Vec<ArgumentsItem> {
    fn from(value: Arguments) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Arguments> for Arguments {
    fn from(value: &Arguments) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<ArgumentsItem>> for Arguments {
    fn from(value: ::std::vec::Vec<ArgumentsItem>) -> Self {
        Self(value)
    }
}
///`ArgumentsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "$ref": "#/$defs/enumeration_arg"
///    },
///    {
///      "$ref": "#/$defs/value_arg"
///    },
///    {
///      "$ref": "#/$defs/type_arg"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ArgumentsItem {
    EnumerationArg(EnumerationArg),
    ValueArg(ValueArg),
    TypeArg(TypeArg),
}
impl ::std::convert::From<&Self> for ArgumentsItem {
    fn from(value: &ArgumentsItem) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<EnumerationArg> for ArgumentsItem {
    fn from(value: EnumerationArg) -> Self {
        Self::EnumerationArg(value)
    }
}
impl ::std::convert::From<ValueArg> for ArgumentsItem {
    fn from(value: ValueArg) -> Self {
        Self::ValueArg(value)
    }
}
impl ::std::convert::From<TypeArg> for ArgumentsItem {
    fn from(value: TypeArg) -> Self {
        Self::TypeArg(value)
    }
}
///`Decomposable`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "NONE",
///    "ONE",
///    "MANY"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum Decomposable {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ONE")]
    One,
    #[serde(rename = "MANY")]
    Many,
}
impl ::std::convert::From<&Self> for Decomposable {
    fn from(value: &Decomposable) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for Decomposable {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::None => f.write_str("NONE"),
            Self::One => f.write_str("ONE"),
            Self::Many => f.write_str("MANY"),
        }
    }
}
impl ::std::str::FromStr for Decomposable {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "NONE" => Ok(Self::None),
            "ONE" => Ok(Self::One),
            "MANY" => Ok(Self::Many),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Decomposable {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Decomposable {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Decomposable {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Deterministic`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "boolean"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Deterministic(pub bool);
impl ::std::ops::Deref for Deterministic {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<Deterministic> for bool {
    fn from(value: Deterministic) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Deterministic> for Deterministic {
    fn from(value: &Deterministic) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for Deterministic {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Deterministic {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Deterministic {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Deterministic {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Deterministic {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Deterministic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`EnumOptions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "type": "string"
///  },
///  "minItems": 1,
///  "uniqueItems": true
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EnumOptions(pub Vec<::std::string::String>);
impl ::std::ops::Deref for EnumOptions {
    type Target = Vec<::std::string::String>;
    fn deref(&self) -> &Vec<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<EnumOptions> for Vec<::std::string::String> {
    fn from(value: EnumOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&EnumOptions> for EnumOptions {
    fn from(value: &EnumOptions) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Vec<::std::string::String>> for EnumOptions {
    fn from(value: Vec<::std::string::String>) -> Self {
        Self(value)
    }
}
///`EnumerationArg`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "options"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "options": {
///      "$ref": "#/$defs/enum_options"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct EnumerationArg {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub options: EnumOptions,
}
impl ::std::convert::From<&EnumerationArg> for EnumerationArg {
    fn from(value: &EnumerationArg) -> Self {
        value.clone()
    }
}
impl EnumerationArg {
    pub fn builder() -> builder::EnumerationArg {
        Default::default()
    }
}
///`Implementation`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": {
///    "type": "string"
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Implementation(
    pub ::std::collections::HashMap<::std::string::String, ::std::string::String>,
);
impl ::std::ops::Deref for Implementation {
    type Target = ::std::collections::HashMap<
        ::std::string::String,
        ::std::string::String,
    >;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<Implementation>
for ::std::collections::HashMap<::std::string::String, ::std::string::String> {
    fn from(value: Implementation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Implementation> for Implementation {
    fn from(value: &Implementation) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, ::std::string::String>,
> for Implementation {
    fn from(
        value: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self(value)
    }
}
///`Intermediate`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/$defs/type"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Intermediate(pub Type);
impl ::std::ops::Deref for Intermediate {
    type Target = Type;
    fn deref(&self) -> &Type {
        &self.0
    }
}
impl ::std::convert::From<Intermediate> for Type {
    fn from(value: Intermediate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Intermediate> for Intermediate {
    fn from(value: &Intermediate) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Type> for Intermediate {
    fn from(value: Type) -> Self {
        Self(value)
    }
}
///`Maxset`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "number"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Maxset(pub f64);
impl ::std::ops::Deref for Maxset {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<Maxset> for f64 {
    fn from(value: Maxset) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Maxset> for Maxset {
    fn from(value: &Maxset) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<f64> for Maxset {
    fn from(value: f64) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Maxset {
    type Err = <f64 as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Maxset {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Maxset {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Maxset {
    type Error = <f64 as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Maxset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`NullabilityHandling`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "MIRROR",
///    "DECLARED_OUTPUT",
///    "DISCRETE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum NullabilityHandling {
    #[serde(rename = "MIRROR")]
    Mirror,
    #[serde(rename = "DECLARED_OUTPUT")]
    DeclaredOutput,
    #[serde(rename = "DISCRETE")]
    Discrete,
}
impl ::std::convert::From<&Self> for NullabilityHandling {
    fn from(value: &NullabilityHandling) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for NullabilityHandling {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Mirror => f.write_str("MIRROR"),
            Self::DeclaredOutput => f.write_str("DECLARED_OUTPUT"),
            Self::Discrete => f.write_str("DISCRETE"),
        }
    }
}
impl ::std::str::FromStr for NullabilityHandling {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "MIRROR" => Ok(Self::Mirror),
            "DECLARED_OUTPUT" => Ok(Self::DeclaredOutput),
            "DISCRETE" => Ok(Self::Discrete),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for NullabilityHandling {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for NullabilityHandling {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for NullabilityHandling {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`Options`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "additionalProperties": {
///    "type": "object",
///    "required": [
///      "values"
///    ],
///    "properties": {
///      "description": {
///        "type": "string"
///      },
///      "values": {
///        "type": "array",
///        "items": {
///          "type": "string"
///        }
///      }
///    },
///    "additionalProperties": false
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Options(pub ::std::collections::HashMap<::std::string::String, OptionsValue>);
impl ::std::ops::Deref for Options {
    type Target = ::std::collections::HashMap<::std::string::String, OptionsValue>;
    fn deref(
        &self,
    ) -> &::std::collections::HashMap<::std::string::String, OptionsValue> {
        &self.0
    }
}
impl ::std::convert::From<Options>
for ::std::collections::HashMap<::std::string::String, OptionsValue> {
    fn from(value: Options) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Options> for Options {
    fn from(value: &Options) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<
    ::std::collections::HashMap<::std::string::String, OptionsValue>,
> for Options {
    fn from(
        value: ::std::collections::HashMap<::std::string::String, OptionsValue>,
    ) -> Self {
        Self(value)
    }
}
///`OptionsValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "values"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "values": {
///      "type": "array",
///      "items": {
///        "type": "string"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct OptionsValue {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub values: ::std::vec::Vec<::std::string::String>,
}
impl ::std::convert::From<&OptionsValue> for OptionsValue {
    fn from(value: &OptionsValue) -> Self {
        value.clone()
    }
}
impl OptionsValue {
    pub fn builder() -> builder::OptionsValue {
        Default::default()
    }
}
///`Ordered`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "boolean"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Ordered(pub bool);
impl ::std::ops::Deref for Ordered {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<Ordered> for bool {
    fn from(value: Ordered) -> Self {
        value.0
    }
}
impl ::std::convert::From<&Ordered> for Ordered {
    fn from(value: &Ordered) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for Ordered {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for Ordered {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for Ordered {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for Ordered {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for Ordered {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for Ordered {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`ReturnValue`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$ref": "#/$defs/type"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct ReturnValue(pub Type);
impl ::std::ops::Deref for ReturnValue {
    type Target = Type;
    fn deref(&self) -> &Type {
        &self.0
    }
}
impl ::std::convert::From<ReturnValue> for Type {
    fn from(value: ReturnValue) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ReturnValue> for ReturnValue {
    fn from(value: &ReturnValue) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<Type> for ReturnValue {
    fn from(value: Type) -> Self {
        Self(value)
    }
}
///`ScalarFunction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "impls",
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "impls": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "return"
///        ],
///        "properties": {
///          "args": {
///            "$ref": "#/$defs/arguments"
///          },
///          "deterministic": {
///            "$ref": "#/$defs/deterministic"
///          },
///          "implementation": {
///            "$ref": "#/$defs/implementation"
///          },
///          "nullability": {
///            "$ref": "#/$defs/nullabilityHandling"
///          },
///          "options": {
///            "$ref": "#/$defs/options"
///          },
///          "return": {
///            "$ref": "#/$defs/returnValue"
///          },
///          "sessionDependent": {
///            "$ref": "#/$defs/sessionDependent"
///          },
///          "variadic": {
///            "$ref": "#/$defs/variadicBehavior"
///          }
///        },
///        "additionalProperties": false
///      },
///      "minItems": 1
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScalarFunction {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub impls: ::std::vec::Vec<ScalarFunctionImplsItem>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&ScalarFunction> for ScalarFunction {
    fn from(value: &ScalarFunction) -> Self {
        value.clone()
    }
}
impl ScalarFunction {
    pub fn builder() -> builder::ScalarFunction {
        Default::default()
    }
}
///`ScalarFunctionImplsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "return"
///  ],
///  "properties": {
///    "args": {
///      "$ref": "#/$defs/arguments"
///    },
///    "deterministic": {
///      "$ref": "#/$defs/deterministic"
///    },
///    "implementation": {
///      "$ref": "#/$defs/implementation"
///    },
///    "nullability": {
///      "$ref": "#/$defs/nullabilityHandling"
///    },
///    "options": {
///      "$ref": "#/$defs/options"
///    },
///    "return": {
///      "$ref": "#/$defs/returnValue"
///    },
///    "sessionDependent": {
///      "$ref": "#/$defs/sessionDependent"
///    },
///    "variadic": {
///      "$ref": "#/$defs/variadicBehavior"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ScalarFunctionImplsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub args: ::std::option::Option<Arguments>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deterministic: ::std::option::Option<Deterministic>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub implementation: ::std::option::Option<Implementation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nullability: ::std::option::Option<NullabilityHandling>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub options: ::std::option::Option<Options>,
    #[serde(rename = "return")]
    pub return_: ReturnValue,
    #[serde(
        rename = "sessionDependent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_dependent: ::std::option::Option<SessionDependent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub variadic: ::std::option::Option<VariadicBehavior>,
}
impl ::std::convert::From<&ScalarFunctionImplsItem> for ScalarFunctionImplsItem {
    fn from(value: &ScalarFunctionImplsItem) -> Self {
        value.clone()
    }
}
impl ScalarFunctionImplsItem {
    pub fn builder() -> builder::ScalarFunctionImplsItem {
        Default::default()
    }
}
///`SessionDependent`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "boolean"
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct SessionDependent(pub bool);
impl ::std::ops::Deref for SessionDependent {
    type Target = bool;
    fn deref(&self) -> &bool {
        &self.0
    }
}
impl ::std::convert::From<SessionDependent> for bool {
    fn from(value: SessionDependent) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SessionDependent> for SessionDependent {
    fn from(value: &SessionDependent) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<bool> for SessionDependent {
    fn from(value: bool) -> Self {
        Self(value)
    }
}
impl ::std::str::FromStr for SessionDependent {
    type Err = <bool as ::std::str::FromStr>::Err;
    fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl ::std::convert::TryFrom<&str> for SessionDependent {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&String> for SessionDependent {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<String> for SessionDependent {
    type Error = <bool as ::std::str::FromStr>::Err;
    fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
        value.parse()
    }
}
impl ::std::fmt::Display for SessionDependent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
///`SimpleExtensions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "$id": "http://substrait.io/schemas/simple_extensions",
///  "title": "Simple Extensions",
///  "type": "object",
///  "required": [
///    "urn"
///  ],
///  "properties": {
///    "aggregate_functions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/aggregateFunction"
///      }
///    },
///    "dependencies": {
///      "type": "object",
///      "patternProperties": {
///        "^[a-zA-Z_\\$][a-zA-Z0-9_\\$]*$": {
///          "type": "string"
///        }
///      }
///    },
///    "scalar_functions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/scalarFunction"
///      }
///    },
///    "type_variations": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name",
///          "parent"
///        ],
///        "properties": {
///          "description": {
///            "type": "string"
///          },
///          "functions": {
///            "type": "string",
///            "enum": [
///              "INHERITS",
///              "SEPARATE"
///            ]
///          },
///          "name": {
///            "type": "string"
///          },
///          "parent": {
///            "$ref": "#/$defs/type"
///          }
///        },
///        "additionalProperties": false
///      },
///      "minItems": 1
///    },
///    "types": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "name"
///        ],
///        "properties": {
///          "description": {
///            "type": "string"
///          },
///          "name": {
///            "type": "string"
///          },
///          "parameters": {
///            "$ref": "#/$defs/type_param_defs"
///          },
///          "structure": {
///            "$ref": "#/$defs/type"
///          },
///          "variadic": {
///            "type": "boolean"
///          }
///        },
///        "additionalProperties": false
///      },
///      "minItems": 1
///    },
///    "urn": {
///      "type": "string"
///    },
///    "window_functions": {
///      "type": "array",
///      "items": {
///        "$ref": "#/$defs/windowFunction"
///      }
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimpleExtensions {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub aggregate_functions: ::std::vec::Vec<AggregateFunction>,
    #[serde(default, skip_serializing_if = ":: std :: collections :: HashMap::is_empty")]
    pub dependencies: ::std::collections::HashMap<
        SimpleExtensionsDependenciesKey,
        ::std::string::String,
    >,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub scalar_functions: ::std::vec::Vec<ScalarFunction>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub type_variations: ::std::vec::Vec<SimpleExtensionsTypeVariationsItem>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub types: ::std::vec::Vec<SimpleExtensionsTypesItem>,
    pub urn: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub window_functions: ::std::vec::Vec<WindowFunction>,
}
impl ::std::convert::From<&SimpleExtensions> for SimpleExtensions {
    fn from(value: &SimpleExtensions) -> Self {
        value.clone()
    }
}
impl SimpleExtensions {
    pub fn builder() -> builder::SimpleExtensions {
        Default::default()
    }
}
///`SimpleExtensionsDependenciesKey`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "pattern": "^[a-zA-Z_\\$][a-zA-Z0-9_\\$]*$"
///}
/// ```
/// </details>
#[derive(::serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SimpleExtensionsDependenciesKey(::std::string::String);
impl ::std::ops::Deref for SimpleExtensionsDependenciesKey {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SimpleExtensionsDependenciesKey> for ::std::string::String {
    fn from(value: SimpleExtensionsDependenciesKey) -> Self {
        value.0
    }
}
impl ::std::convert::From<&SimpleExtensionsDependenciesKey>
for SimpleExtensionsDependenciesKey {
    fn from(value: &SimpleExtensionsDependenciesKey) -> Self {
        value.clone()
    }
}
impl ::std::str::FromStr for SimpleExtensionsDependenciesKey {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(||
        { ::regress::Regex::new("^[a-zA-Z_\\$][a-zA-Z0-9_\\$]*$").unwrap() });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^[a-zA-Z_\\$][a-zA-Z0-9_\\$]*$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SimpleExtensionsDependenciesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SimpleExtensionsDependenciesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SimpleExtensionsDependenciesKey {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SimpleExtensionsDependenciesKey {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
///`SimpleExtensionsTypeVariationsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name",
///    "parent"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "functions": {
///      "type": "string",
///      "enum": [
///        "INHERITS",
///        "SEPARATE"
///      ]
///    },
///    "name": {
///      "type": "string"
///    },
///    "parent": {
///      "$ref": "#/$defs/type"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimpleExtensionsTypeVariationsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub functions: ::std::option::Option<SimpleExtensionsTypeVariationsItemFunctions>,
    pub name: ::std::string::String,
    pub parent: Type,
}
impl ::std::convert::From<&SimpleExtensionsTypeVariationsItem>
for SimpleExtensionsTypeVariationsItem {
    fn from(value: &SimpleExtensionsTypeVariationsItem) -> Self {
        value.clone()
    }
}
impl SimpleExtensionsTypeVariationsItem {
    pub fn builder() -> builder::SimpleExtensionsTypeVariationsItem {
        Default::default()
    }
}
///`SimpleExtensionsTypeVariationsItemFunctions`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "INHERITS",
///    "SEPARATE"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum SimpleExtensionsTypeVariationsItemFunctions {
    #[serde(rename = "INHERITS")]
    Inherits,
    #[serde(rename = "SEPARATE")]
    Separate,
}
impl ::std::convert::From<&Self> for SimpleExtensionsTypeVariationsItemFunctions {
    fn from(value: &SimpleExtensionsTypeVariationsItemFunctions) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for SimpleExtensionsTypeVariationsItemFunctions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Inherits => f.write_str("INHERITS"),
            Self::Separate => f.write_str("SEPARATE"),
        }
    }
}
impl ::std::str::FromStr for SimpleExtensionsTypeVariationsItemFunctions {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "INHERITS" => Ok(Self::Inherits),
            "SEPARATE" => Ok(Self::Separate),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for SimpleExtensionsTypeVariationsItemFunctions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for SimpleExtensionsTypeVariationsItemFunctions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for SimpleExtensionsTypeVariationsItemFunctions {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`SimpleExtensionsTypesItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "parameters": {
///      "$ref": "#/$defs/type_param_defs"
///    },
///    "structure": {
///      "$ref": "#/$defs/type"
///    },
///    "variadic": {
///      "type": "boolean"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SimpleExtensionsTypesItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub parameters: ::std::option::Option<TypeParamDefs>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub structure: ::std::option::Option<Type>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub variadic: ::std::option::Option<bool>,
}
impl ::std::convert::From<&SimpleExtensionsTypesItem> for SimpleExtensionsTypesItem {
    fn from(value: &SimpleExtensionsTypesItem) -> Self {
        value.clone()
    }
}
impl SimpleExtensionsTypesItem {
    pub fn builder() -> builder::SimpleExtensionsTypesItem {
        Default::default()
    }
}
///`Type`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "oneOf": [
///    {
///      "type": "string"
///    },
///    {
///      "type": "object"
///    }
///  ]
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Type {
    String(::std::string::String),
    Object(::serde_json::Map<::std::string::String, ::serde_json::Value>),
}
impl ::std::convert::From<&Self> for Type {
    fn from(value: &Type) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::serde_json::Map<::std::string::String, ::serde_json::Value>>
for Type {
    fn from(
        value: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    ) -> Self {
        Self::Object(value)
    }
}
///`TypeArg`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "type": {
///      "type": "string"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TypeArg {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: ::std::string::String,
}
impl ::std::convert::From<&TypeArg> for TypeArg {
    fn from(value: &TypeArg) -> Self {
        value.clone()
    }
}
impl TypeArg {
    pub fn builder() -> builder::TypeArg {
        Default::default()
    }
}
///`TypeParamDefs`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "array",
///  "items": {
///    "type": "object",
///    "required": [
///      "type"
///    ],
///    "properties": {
///      "description": {
///        "type": "string"
///      },
///      "max": {
///        "type": "number"
///      },
///      "min": {
///        "type": "number"
///      },
///      "name": {
///        "type": "string"
///      },
///      "optional": {
///        "type": "boolean"
///      },
///      "options": {
///        "$ref": "#/$defs/enum_options"
///      },
///      "type": {
///        "type": "string",
///        "enum": [
///          "dataType",
///          "boolean",
///          "integer",
///          "enumeration",
///          "string"
///        ]
///      }
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct TypeParamDefs(pub ::std::vec::Vec<TypeParamDefsItem>);
impl ::std::ops::Deref for TypeParamDefs {
    type Target = ::std::vec::Vec<TypeParamDefsItem>;
    fn deref(&self) -> &::std::vec::Vec<TypeParamDefsItem> {
        &self.0
    }
}
impl ::std::convert::From<TypeParamDefs> for ::std::vec::Vec<TypeParamDefsItem> {
    fn from(value: TypeParamDefs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&TypeParamDefs> for TypeParamDefs {
    fn from(value: &TypeParamDefs) -> Self {
        value.clone()
    }
}
impl ::std::convert::From<::std::vec::Vec<TypeParamDefsItem>> for TypeParamDefs {
    fn from(value: ::std::vec::Vec<TypeParamDefsItem>) -> Self {
        Self(value)
    }
}
///`TypeParamDefsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "type"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "max": {
///      "type": "number"
///    },
///    "min": {
///      "type": "number"
///    },
///    "name": {
///      "type": "string"
///    },
///    "optional": {
///      "type": "boolean"
///    },
///    "options": {
///      "$ref": "#/$defs/enum_options"
///    },
///    "type": {
///      "type": "string",
///      "enum": [
///        "dataType",
///        "boolean",
///        "integer",
///        "enumeration",
///        "string"
///      ]
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct TypeParamDefsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub optional: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub options: ::std::option::Option<EnumOptions>,
    #[serde(rename = "type")]
    pub type_: TypeParamDefsItemType,
}
impl ::std::convert::From<&TypeParamDefsItem> for TypeParamDefsItem {
    fn from(value: &TypeParamDefsItem) -> Self {
        value.clone()
    }
}
impl TypeParamDefsItem {
    pub fn builder() -> builder::TypeParamDefsItem {
        Default::default()
    }
}
///`TypeParamDefsItemType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "dataType",
///    "boolean",
///    "integer",
///    "enumeration",
///    "string"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum TypeParamDefsItemType {
    #[serde(rename = "dataType")]
    DataType,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "enumeration")]
    Enumeration,
    #[serde(rename = "string")]
    String,
}
impl ::std::convert::From<&Self> for TypeParamDefsItemType {
    fn from(value: &TypeParamDefsItemType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for TypeParamDefsItemType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DataType => f.write_str("dataType"),
            Self::Boolean => f.write_str("boolean"),
            Self::Integer => f.write_str("integer"),
            Self::Enumeration => f.write_str("enumeration"),
            Self::String => f.write_str("string"),
        }
    }
}
impl ::std::str::FromStr for TypeParamDefsItemType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dataType" => Ok(Self::DataType),
            "boolean" => Ok(Self::Boolean),
            "integer" => Ok(Self::Integer),
            "enumeration" => Ok(Self::Enumeration),
            "string" => Ok(Self::String),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TypeParamDefsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TypeParamDefsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TypeParamDefsItemType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`ValueArg`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "value"
///  ],
///  "properties": {
///    "constant": {
///      "type": "boolean"
///    },
///    "description": {
///      "type": "string"
///    },
///    "name": {
///      "type": "string"
///    },
///    "value": {
///      "$ref": "#/$defs/type"
///    }
///  }
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
pub struct ValueArg {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub constant: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub name: ::std::option::Option<::std::string::String>,
    pub value: Type,
}
impl ::std::convert::From<&ValueArg> for ValueArg {
    fn from(value: &ValueArg) -> Self {
        value.clone()
    }
}
impl ValueArg {
    pub fn builder() -> builder::ValueArg {
        Default::default()
    }
}
///`VariadicBehavior`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "properties": {
///    "max": {
///      "type": "number"
///    },
///    "min": {
///      "type": "number"
///    },
///    "parameterConsistency": {
///      "type": "string",
///      "enum": [
///        "CONSISTENT",
///        "INCONSISTENT"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VariadicBehavior {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub max: ::std::option::Option<f64>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub min: ::std::option::Option<f64>,
    #[serde(
        rename = "parameterConsistency",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub parameter_consistency: ::std::option::Option<
        VariadicBehaviorParameterConsistency,
    >,
}
impl ::std::convert::From<&VariadicBehavior> for VariadicBehavior {
    fn from(value: &VariadicBehavior) -> Self {
        value.clone()
    }
}
impl ::std::default::Default for VariadicBehavior {
    fn default() -> Self {
        Self {
            max: Default::default(),
            min: Default::default(),
            parameter_consistency: Default::default(),
        }
    }
}
impl VariadicBehavior {
    pub fn builder() -> builder::VariadicBehavior {
        Default::default()
    }
}
///`VariadicBehaviorParameterConsistency`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "CONSISTENT",
///    "INCONSISTENT"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum VariadicBehaviorParameterConsistency {
    #[serde(rename = "CONSISTENT")]
    Consistent,
    #[serde(rename = "INCONSISTENT")]
    Inconsistent,
}
impl ::std::convert::From<&Self> for VariadicBehaviorParameterConsistency {
    fn from(value: &VariadicBehaviorParameterConsistency) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for VariadicBehaviorParameterConsistency {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Consistent => f.write_str("CONSISTENT"),
            Self::Inconsistent => f.write_str("INCONSISTENT"),
        }
    }
}
impl ::std::str::FromStr for VariadicBehaviorParameterConsistency {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "CONSISTENT" => Ok(Self::Consistent),
            "INCONSISTENT" => Ok(Self::Inconsistent),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for VariadicBehaviorParameterConsistency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for VariadicBehaviorParameterConsistency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for VariadicBehaviorParameterConsistency {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
///`WindowFunction`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "impls",
///    "name"
///  ],
///  "properties": {
///    "description": {
///      "type": "string"
///    },
///    "impls": {
///      "type": "array",
///      "items": {
///        "type": "object",
///        "required": [
///          "return"
///        ],
///        "properties": {
///          "args": {
///            "$ref": "#/$defs/arguments"
///          },
///          "decomposable": {
///            "$ref": "#/$defs/decomposable"
///          },
///          "deterministic": {
///            "$ref": "#/$defs/deterministic"
///          },
///          "implementation": {
///            "$ref": "#/$defs/implementation"
///          },
///          "intermediate": {
///            "$ref": "#/$defs/intermediate"
///          },
///          "maxset": {
///            "$ref": "#/$defs/maxset"
///          },
///          "nullability": {
///            "$ref": "#/$defs/nullabilityHandling"
///          },
///          "options": {
///            "$ref": "#/$defs/options"
///          },
///          "ordered": {
///            "$ref": "#/$defs/ordered"
///          },
///          "return": {
///            "$ref": "#/$defs/returnValue"
///          },
///          "sessionDependent": {
///            "$ref": "#/$defs/sessionDependent"
///          },
///          "variadic": {
///            "$ref": "#/$defs/variadicBehavior"
///          },
///          "window_type": {
///            "type": "string",
///            "enum": [
///              "STREAMING",
///              "PARTITION"
///            ]
///          }
///        },
///        "additionalProperties": false
///      },
///      "minItems": 1
///    },
///    "name": {
///      "type": "string"
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WindowFunction {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub description: ::std::option::Option<::std::string::String>,
    pub impls: ::std::vec::Vec<WindowFunctionImplsItem>,
    pub name: ::std::string::String,
}
impl ::std::convert::From<&WindowFunction> for WindowFunction {
    fn from(value: &WindowFunction) -> Self {
        value.clone()
    }
}
impl WindowFunction {
    pub fn builder() -> builder::WindowFunction {
        Default::default()
    }
}
///`WindowFunctionImplsItem`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "object",
///  "required": [
///    "return"
///  ],
///  "properties": {
///    "args": {
///      "$ref": "#/$defs/arguments"
///    },
///    "decomposable": {
///      "$ref": "#/$defs/decomposable"
///    },
///    "deterministic": {
///      "$ref": "#/$defs/deterministic"
///    },
///    "implementation": {
///      "$ref": "#/$defs/implementation"
///    },
///    "intermediate": {
///      "$ref": "#/$defs/intermediate"
///    },
///    "maxset": {
///      "$ref": "#/$defs/maxset"
///    },
///    "nullability": {
///      "$ref": "#/$defs/nullabilityHandling"
///    },
///    "options": {
///      "$ref": "#/$defs/options"
///    },
///    "ordered": {
///      "$ref": "#/$defs/ordered"
///    },
///    "return": {
///      "$ref": "#/$defs/returnValue"
///    },
///    "sessionDependent": {
///      "$ref": "#/$defs/sessionDependent"
///    },
///    "variadic": {
///      "$ref": "#/$defs/variadicBehavior"
///    },
///    "window_type": {
///      "type": "string",
///      "enum": [
///        "STREAMING",
///        "PARTITION"
///      ]
///    }
///  },
///  "additionalProperties": false
///}
/// ```
/// </details>
#[derive(::serde::Deserialize, ::serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct WindowFunctionImplsItem {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub args: ::std::option::Option<Arguments>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub decomposable: ::std::option::Option<Decomposable>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deterministic: ::std::option::Option<Deterministic>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub implementation: ::std::option::Option<Implementation>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub intermediate: ::std::option::Option<Intermediate>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub maxset: ::std::option::Option<Maxset>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub nullability: ::std::option::Option<NullabilityHandling>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub options: ::std::option::Option<Options>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ordered: ::std::option::Option<Ordered>,
    #[serde(rename = "return")]
    pub return_: ReturnValue,
    #[serde(
        rename = "sessionDependent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub session_dependent: ::std::option::Option<SessionDependent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub variadic: ::std::option::Option<VariadicBehavior>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub window_type: ::std::option::Option<WindowFunctionImplsItemWindowType>,
}
impl ::std::convert::From<&WindowFunctionImplsItem> for WindowFunctionImplsItem {
    fn from(value: &WindowFunctionImplsItem) -> Self {
        value.clone()
    }
}
impl WindowFunctionImplsItem {
    pub fn builder() -> builder::WindowFunctionImplsItem {
        Default::default()
    }
}
///`WindowFunctionImplsItemWindowType`
///
/// <details><summary>JSON schema</summary>
///
/// ```json
///{
///  "type": "string",
///  "enum": [
///    "STREAMING",
///    "PARTITION"
///  ]
///}
/// ```
/// </details>
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd
)]
pub enum WindowFunctionImplsItemWindowType {
    #[serde(rename = "STREAMING")]
    Streaming,
    #[serde(rename = "PARTITION")]
    Partition,
}
impl ::std::convert::From<&Self> for WindowFunctionImplsItemWindowType {
    fn from(value: &WindowFunctionImplsItemWindowType) -> Self {
        value.clone()
    }
}
impl ::std::fmt::Display for WindowFunctionImplsItemWindowType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Streaming => f.write_str("STREAMING"),
            Self::Partition => f.write_str("PARTITION"),
        }
    }
}
impl ::std::str::FromStr for WindowFunctionImplsItemWindowType {
    type Err = self::error::ConversionError;
    fn from_str(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "STREAMING" => Ok(Self::Streaming),
            "PARTITION" => Ok(Self::Partition),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WindowFunctionImplsItemWindowType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &str,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String>
for WindowFunctionImplsItemWindowType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String>
for WindowFunctionImplsItemWindowType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
/// Types for composing complex structures.
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AggregateFunction {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        impls: ::std::result::Result<
            ::std::vec::Vec<super::AggregateFunctionImplsItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for AggregateFunction {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                impls: Err("no value supplied for impls".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl AggregateFunction {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn impls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::AggregateFunctionImplsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.impls = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for impls: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<AggregateFunction> for super::AggregateFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AggregateFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                impls: value.impls?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::AggregateFunction> for AggregateFunction {
        fn from(value: super::AggregateFunction) -> Self {
            Self {
                description: Ok(value.description),
                impls: Ok(value.impls),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct AggregateFunctionImplsItem {
        args: ::std::result::Result<
            ::std::option::Option<super::Arguments>,
            ::std::string::String,
        >,
        decomposable: ::std::result::Result<
            ::std::option::Option<super::Decomposable>,
            ::std::string::String,
        >,
        deterministic: ::std::result::Result<
            ::std::option::Option<super::Deterministic>,
            ::std::string::String,
        >,
        implementation: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
        intermediate: ::std::result::Result<
            ::std::option::Option<super::Intermediate>,
            ::std::string::String,
        >,
        maxset: ::std::result::Result<
            ::std::option::Option<super::Maxset>,
            ::std::string::String,
        >,
        nullability: ::std::result::Result<
            ::std::option::Option<super::NullabilityHandling>,
            ::std::string::String,
        >,
        options: ::std::result::Result<
            ::std::option::Option<super::Options>,
            ::std::string::String,
        >,
        ordered: ::std::result::Result<
            ::std::option::Option<super::Ordered>,
            ::std::string::String,
        >,
        return_: ::std::result::Result<super::ReturnValue, ::std::string::String>,
        session_dependent: ::std::result::Result<
            ::std::option::Option<super::SessionDependent>,
            ::std::string::String,
        >,
        variadic: ::std::result::Result<
            ::std::option::Option<super::VariadicBehavior>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for AggregateFunctionImplsItem {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                decomposable: Ok(Default::default()),
                deterministic: Ok(Default::default()),
                implementation: Ok(Default::default()),
                intermediate: Ok(Default::default()),
                maxset: Ok(Default::default()),
                nullability: Ok(Default::default()),
                options: Ok(Default::default()),
                ordered: Ok(Default::default()),
                return_: Err("no value supplied for return_".to_string()),
                session_dependent: Ok(Default::default()),
                variadic: Ok(Default::default()),
            }
        }
    }
    impl AggregateFunctionImplsItem {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Arguments>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn decomposable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Decomposable>>,
            T::Error: ::std::fmt::Display,
        {
            self.decomposable = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decomposable: {}", e)
                });
            self
        }
        pub fn deterministic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Deterministic>>,
            T::Error: ::std::fmt::Display,
        {
            self.deterministic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for deterministic: {}", e)
                });
            self
        }
        pub fn implementation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.implementation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for implementation: {}", e)
                });
            self
        }
        pub fn intermediate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Intermediate>>,
            T::Error: ::std::fmt::Display,
        {
            self.intermediate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intermediate: {}", e)
                });
            self
        }
        pub fn maxset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Maxset>>,
            T::Error: ::std::fmt::Display,
        {
            self.maxset = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for maxset: {}", e)
                });
            self
        }
        pub fn nullability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NullabilityHandling>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.nullability = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nullability: {}", e)
                });
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Options>>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
        pub fn ordered<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Ordered>>,
            T::Error: ::std::fmt::Display,
        {
            self.ordered = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ordered: {}", e)
                });
            self
        }
        pub fn return_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReturnValue>,
            T::Error: ::std::fmt::Display,
        {
            self.return_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for return_: {}", e)
                });
            self
        }
        pub fn session_dependent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SessionDependent>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_dependent = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for session_dependent: {}", e
                    )
                });
            self
        }
        pub fn variadic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VariadicBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.variadic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for variadic: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<AggregateFunctionImplsItem>
    for super::AggregateFunctionImplsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: AggregateFunctionImplsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                decomposable: value.decomposable?,
                deterministic: value.deterministic?,
                implementation: value.implementation?,
                intermediate: value.intermediate?,
                maxset: value.maxset?,
                nullability: value.nullability?,
                options: value.options?,
                ordered: value.ordered?,
                return_: value.return_?,
                session_dependent: value.session_dependent?,
                variadic: value.variadic?,
            })
        }
    }
    impl ::std::convert::From<super::AggregateFunctionImplsItem>
    for AggregateFunctionImplsItem {
        fn from(value: super::AggregateFunctionImplsItem) -> Self {
            Self {
                args: Ok(value.args),
                decomposable: Ok(value.decomposable),
                deterministic: Ok(value.deterministic),
                implementation: Ok(value.implementation),
                intermediate: Ok(value.intermediate),
                maxset: Ok(value.maxset),
                nullability: Ok(value.nullability),
                options: Ok(value.options),
                ordered: Ok(value.ordered),
                return_: Ok(value.return_),
                session_dependent: Ok(value.session_dependent),
                variadic: Ok(value.variadic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct EnumerationArg {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        options: ::std::result::Result<super::EnumOptions, ::std::string::String>,
    }
    impl ::std::default::Default for EnumerationArg {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Ok(Default::default()),
                options: Err("no value supplied for options".to_string()),
            }
        }
    }
    impl EnumerationArg {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EnumOptions>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<EnumerationArg> for super::EnumerationArg {
        type Error = super::error::ConversionError;
        fn try_from(
            value: EnumerationArg,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                options: value.options?,
            })
        }
    }
    impl ::std::convert::From<super::EnumerationArg> for EnumerationArg {
        fn from(value: super::EnumerationArg) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                options: Ok(value.options),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct OptionsValue {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        values: ::std::result::Result<
            ::std::vec::Vec<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for OptionsValue {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                values: Err("no value supplied for values".to_string()),
            }
        }
    }
    impl OptionsValue {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for values: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<OptionsValue> for super::OptionsValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: OptionsValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::OptionsValue> for OptionsValue {
        fn from(value: super::OptionsValue) -> Self {
            Self {
                description: Ok(value.description),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScalarFunction {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        impls: ::std::result::Result<
            ::std::vec::Vec<super::ScalarFunctionImplsItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ScalarFunction {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                impls: Err("no value supplied for impls".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl ScalarFunction {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn impls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ScalarFunctionImplsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.impls = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for impls: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<ScalarFunction> for super::ScalarFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScalarFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                impls: value.impls?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::ScalarFunction> for ScalarFunction {
        fn from(value: super::ScalarFunction) -> Self {
            Self {
                description: Ok(value.description),
                impls: Ok(value.impls),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ScalarFunctionImplsItem {
        args: ::std::result::Result<
            ::std::option::Option<super::Arguments>,
            ::std::string::String,
        >,
        deterministic: ::std::result::Result<
            ::std::option::Option<super::Deterministic>,
            ::std::string::String,
        >,
        implementation: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
        nullability: ::std::result::Result<
            ::std::option::Option<super::NullabilityHandling>,
            ::std::string::String,
        >,
        options: ::std::result::Result<
            ::std::option::Option<super::Options>,
            ::std::string::String,
        >,
        return_: ::std::result::Result<super::ReturnValue, ::std::string::String>,
        session_dependent: ::std::result::Result<
            ::std::option::Option<super::SessionDependent>,
            ::std::string::String,
        >,
        variadic: ::std::result::Result<
            ::std::option::Option<super::VariadicBehavior>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ScalarFunctionImplsItem {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                deterministic: Ok(Default::default()),
                implementation: Ok(Default::default()),
                nullability: Ok(Default::default()),
                options: Ok(Default::default()),
                return_: Err("no value supplied for return_".to_string()),
                session_dependent: Ok(Default::default()),
                variadic: Ok(Default::default()),
            }
        }
    }
    impl ScalarFunctionImplsItem {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Arguments>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn deterministic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Deterministic>>,
            T::Error: ::std::fmt::Display,
        {
            self.deterministic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for deterministic: {}", e)
                });
            self
        }
        pub fn implementation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.implementation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for implementation: {}", e)
                });
            self
        }
        pub fn nullability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NullabilityHandling>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.nullability = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nullability: {}", e)
                });
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Options>>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
        pub fn return_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReturnValue>,
            T::Error: ::std::fmt::Display,
        {
            self.return_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for return_: {}", e)
                });
            self
        }
        pub fn session_dependent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SessionDependent>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_dependent = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for session_dependent: {}", e
                    )
                });
            self
        }
        pub fn variadic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VariadicBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.variadic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for variadic: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ScalarFunctionImplsItem>
    for super::ScalarFunctionImplsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ScalarFunctionImplsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                deterministic: value.deterministic?,
                implementation: value.implementation?,
                nullability: value.nullability?,
                options: value.options?,
                return_: value.return_?,
                session_dependent: value.session_dependent?,
                variadic: value.variadic?,
            })
        }
    }
    impl ::std::convert::From<super::ScalarFunctionImplsItem>
    for ScalarFunctionImplsItem {
        fn from(value: super::ScalarFunctionImplsItem) -> Self {
            Self {
                args: Ok(value.args),
                deterministic: Ok(value.deterministic),
                implementation: Ok(value.implementation),
                nullability: Ok(value.nullability),
                options: Ok(value.options),
                return_: Ok(value.return_),
                session_dependent: Ok(value.session_dependent),
                variadic: Ok(value.variadic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SimpleExtensions {
        aggregate_functions: ::std::result::Result<
            ::std::vec::Vec<super::AggregateFunction>,
            ::std::string::String,
        >,
        dependencies: ::std::result::Result<
            ::std::collections::HashMap<
                super::SimpleExtensionsDependenciesKey,
                ::std::string::String,
            >,
            ::std::string::String,
        >,
        scalar_functions: ::std::result::Result<
            ::std::vec::Vec<super::ScalarFunction>,
            ::std::string::String,
        >,
        type_variations: ::std::result::Result<
            ::std::vec::Vec<super::SimpleExtensionsTypeVariationsItem>,
            ::std::string::String,
        >,
        types: ::std::result::Result<
            ::std::vec::Vec<super::SimpleExtensionsTypesItem>,
            ::std::string::String,
        >,
        urn: ::std::result::Result<::std::string::String, ::std::string::String>,
        window_functions: ::std::result::Result<
            ::std::vec::Vec<super::WindowFunction>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SimpleExtensions {
        fn default() -> Self {
            Self {
                aggregate_functions: Ok(Default::default()),
                dependencies: Ok(Default::default()),
                scalar_functions: Ok(Default::default()),
                type_variations: Ok(Default::default()),
                types: Ok(Default::default()),
                urn: Err("no value supplied for urn".to_string()),
                window_functions: Ok(Default::default()),
            }
        }
    }
    impl SimpleExtensions {
        pub fn aggregate_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AggregateFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.aggregate_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for aggregate_functions: {}", e
                    )
                });
            self
        }
        pub fn dependencies<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::collections::HashMap<
                    super::SimpleExtensionsDependenciesKey,
                    ::std::string::String,
                >,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.dependencies = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for dependencies: {}", e)
                });
            self
        }
        pub fn scalar_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ScalarFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.scalar_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for scalar_functions: {}", e
                    )
                });
            self
        }
        pub fn type_variations<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::SimpleExtensionsTypeVariationsItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.type_variations = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_variations: {}", e)
                });
            self
        }
        pub fn types<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::vec::Vec<super::SimpleExtensionsTypesItem>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.types = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for types: {}", e)
                });
            self
        }
        pub fn urn<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.urn = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urn: {}", e));
            self
        }
        pub fn window_functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WindowFunction>>,
            T::Error: ::std::fmt::Display,
        {
            self.window_functions = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for window_functions: {}", e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SimpleExtensions> for super::SimpleExtensions {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SimpleExtensions,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                aggregate_functions: value.aggregate_functions?,
                dependencies: value.dependencies?,
                scalar_functions: value.scalar_functions?,
                type_variations: value.type_variations?,
                types: value.types?,
                urn: value.urn?,
                window_functions: value.window_functions?,
            })
        }
    }
    impl ::std::convert::From<super::SimpleExtensions> for SimpleExtensions {
        fn from(value: super::SimpleExtensions) -> Self {
            Self {
                aggregate_functions: Ok(value.aggregate_functions),
                dependencies: Ok(value.dependencies),
                scalar_functions: Ok(value.scalar_functions),
                type_variations: Ok(value.type_variations),
                types: Ok(value.types),
                urn: Ok(value.urn),
                window_functions: Ok(value.window_functions),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SimpleExtensionsTypeVariationsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        functions: ::std::result::Result<
            ::std::option::Option<super::SimpleExtensionsTypeVariationsItemFunctions>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parent: ::std::result::Result<super::Type, ::std::string::String>,
    }
    impl ::std::default::Default for SimpleExtensionsTypeVariationsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                functions: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                parent: Err("no value supplied for parent".to_string()),
            }
        }
    }
    impl SimpleExtensionsTypeVariationsItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn functions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::SimpleExtensionsTypeVariationsItemFunctions>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.functions = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for functions: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Type>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for parent: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SimpleExtensionsTypeVariationsItem>
    for super::SimpleExtensionsTypeVariationsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SimpleExtensionsTypeVariationsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                functions: value.functions?,
                name: value.name?,
                parent: value.parent?,
            })
        }
    }
    impl ::std::convert::From<super::SimpleExtensionsTypeVariationsItem>
    for SimpleExtensionsTypeVariationsItem {
        fn from(value: super::SimpleExtensionsTypeVariationsItem) -> Self {
            Self {
                description: Ok(value.description),
                functions: Ok(value.functions),
                name: Ok(value.name),
                parent: Ok(value.parent),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SimpleExtensionsTypesItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        parameters: ::std::result::Result<
            ::std::option::Option<super::TypeParamDefs>,
            ::std::string::String,
        >,
        structure: ::std::result::Result<
            ::std::option::Option<super::Type>,
            ::std::string::String,
        >,
        variadic: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for SimpleExtensionsTypesItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                parameters: Ok(Default::default()),
                structure: Ok(Default::default()),
                variadic: Ok(Default::default()),
            }
        }
    }
    impl SimpleExtensionsTypesItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn parameters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::TypeParamDefs>>,
            T::Error: ::std::fmt::Display,
        {
            self.parameters = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for parameters: {}", e)
                });
            self
        }
        pub fn structure<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Type>>,
            T::Error: ::std::fmt::Display,
        {
            self.structure = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for structure: {}", e)
                });
            self
        }
        pub fn variadic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.variadic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for variadic: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<SimpleExtensionsTypesItem>
    for super::SimpleExtensionsTypesItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SimpleExtensionsTypesItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                parameters: value.parameters?,
                structure: value.structure?,
                variadic: value.variadic?,
            })
        }
    }
    impl ::std::convert::From<super::SimpleExtensionsTypesItem>
    for SimpleExtensionsTypesItem {
        fn from(value: super::SimpleExtensionsTypesItem) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                parameters: Ok(value.parameters),
                structure: Ok(value.structure),
                variadic: Ok(value.variadic),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeArg {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for TypeArg {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                name: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeArg {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeArg> for super::TypeArg {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeArg,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                name: value.name?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeArg> for TypeArg {
        fn from(value: super::TypeArg) -> Self {
            Self {
                description: Ok(value.description),
                name: Ok(value.name),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TypeParamDefsItem {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        optional: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        options: ::std::result::Result<
            ::std::option::Option<super::EnumOptions>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<
            super::TypeParamDefsItemType,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for TypeParamDefsItem {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                max: Ok(Default::default()),
                min: Ok(Default::default()),
                name: Ok(Default::default()),
                optional: Ok(Default::default()),
                options: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TypeParamDefsItem {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn optional<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.optional = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for optional: {}", e)
                });
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::EnumOptions>>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TypeParamDefsItemType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for type_: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<TypeParamDefsItem> for super::TypeParamDefsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: TypeParamDefsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                max: value.max?,
                min: value.min?,
                name: value.name?,
                optional: value.optional?,
                options: value.options?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::TypeParamDefsItem> for TypeParamDefsItem {
        fn from(value: super::TypeParamDefsItem) -> Self {
            Self {
                description: Ok(value.description),
                max: Ok(value.max),
                min: Ok(value.min),
                name: Ok(value.name),
                optional: Ok(value.optional),
                options: Ok(value.options),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ValueArg {
        constant: ::std::result::Result<
            ::std::option::Option<bool>,
            ::std::string::String,
        >,
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        name: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        value: ::std::result::Result<super::Type, ::std::string::String>,
    }
    impl ::std::default::Default for ValueArg {
        fn default() -> Self {
            Self {
                constant: Ok(Default::default()),
                description: Ok(Default::default()),
                name: Ok(Default::default()),
                value: Err("no value supplied for value".to_string()),
            }
        }
    }
    impl ValueArg {
        pub fn constant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<bool>>,
            T::Error: ::std::fmt::Display,
        {
            self.constant = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for constant: {}", e)
                });
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn value<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Type>,
            T::Error: ::std::fmt::Display,
        {
            self.value = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for value: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<ValueArg> for super::ValueArg {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ValueArg,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                constant: value.constant?,
                description: value.description?,
                name: value.name?,
                value: value.value?,
            })
        }
    }
    impl ::std::convert::From<super::ValueArg> for ValueArg {
        fn from(value: super::ValueArg) -> Self {
            Self {
                constant: Ok(value.constant),
                description: Ok(value.description),
                name: Ok(value.name),
                value: Ok(value.value),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VariadicBehavior {
        max: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        min: ::std::result::Result<::std::option::Option<f64>, ::std::string::String>,
        parameter_consistency: ::std::result::Result<
            ::std::option::Option<super::VariadicBehaviorParameterConsistency>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VariadicBehavior {
        fn default() -> Self {
            Self {
                max: Ok(Default::default()),
                min: Ok(Default::default()),
                parameter_consistency: Ok(Default::default()),
            }
        }
    }
    impl VariadicBehavior {
        pub fn max<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.max = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max: {}", e));
            self
        }
        pub fn min<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<f64>>,
            T::Error: ::std::fmt::Display,
        {
            self.min = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min: {}", e));
            self
        }
        pub fn parameter_consistency<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::VariadicBehaviorParameterConsistency>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.parameter_consistency = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for parameter_consistency: {}",
                        e
                    )
                });
            self
        }
    }
    impl ::std::convert::TryFrom<VariadicBehavior> for super::VariadicBehavior {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VariadicBehavior,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max: value.max?,
                min: value.min?,
                parameter_consistency: value.parameter_consistency?,
            })
        }
    }
    impl ::std::convert::From<super::VariadicBehavior> for VariadicBehavior {
        fn from(value: super::VariadicBehavior) -> Self {
            Self {
                max: Ok(value.max),
                min: Ok(value.min),
                parameter_consistency: Ok(value.parameter_consistency),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WindowFunction {
        description: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        impls: ::std::result::Result<
            ::std::vec::Vec<super::WindowFunctionImplsItem>,
            ::std::string::String,
        >,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for WindowFunction {
        fn default() -> Self {
            Self {
                description: Ok(Default::default()),
                impls: Err("no value supplied for impls".to_string()),
                name: Err("no value supplied for name".to_string()),
            }
        }
    }
    impl WindowFunction {
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for description: {}", e)
                });
            self
        }
        pub fn impls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::WindowFunctionImplsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.impls = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for impls: {}", e)
                });
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
    }
    impl ::std::convert::TryFrom<WindowFunction> for super::WindowFunction {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WindowFunction,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                description: value.description?,
                impls: value.impls?,
                name: value.name?,
            })
        }
    }
    impl ::std::convert::From<super::WindowFunction> for WindowFunction {
        fn from(value: super::WindowFunction) -> Self {
            Self {
                description: Ok(value.description),
                impls: Ok(value.impls),
                name: Ok(value.name),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct WindowFunctionImplsItem {
        args: ::std::result::Result<
            ::std::option::Option<super::Arguments>,
            ::std::string::String,
        >,
        decomposable: ::std::result::Result<
            ::std::option::Option<super::Decomposable>,
            ::std::string::String,
        >,
        deterministic: ::std::result::Result<
            ::std::option::Option<super::Deterministic>,
            ::std::string::String,
        >,
        implementation: ::std::result::Result<
            ::std::option::Option<super::Implementation>,
            ::std::string::String,
        >,
        intermediate: ::std::result::Result<
            ::std::option::Option<super::Intermediate>,
            ::std::string::String,
        >,
        maxset: ::std::result::Result<
            ::std::option::Option<super::Maxset>,
            ::std::string::String,
        >,
        nullability: ::std::result::Result<
            ::std::option::Option<super::NullabilityHandling>,
            ::std::string::String,
        >,
        options: ::std::result::Result<
            ::std::option::Option<super::Options>,
            ::std::string::String,
        >,
        ordered: ::std::result::Result<
            ::std::option::Option<super::Ordered>,
            ::std::string::String,
        >,
        return_: ::std::result::Result<super::ReturnValue, ::std::string::String>,
        session_dependent: ::std::result::Result<
            ::std::option::Option<super::SessionDependent>,
            ::std::string::String,
        >,
        variadic: ::std::result::Result<
            ::std::option::Option<super::VariadicBehavior>,
            ::std::string::String,
        >,
        window_type: ::std::result::Result<
            ::std::option::Option<super::WindowFunctionImplsItemWindowType>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for WindowFunctionImplsItem {
        fn default() -> Self {
            Self {
                args: Ok(Default::default()),
                decomposable: Ok(Default::default()),
                deterministic: Ok(Default::default()),
                implementation: Ok(Default::default()),
                intermediate: Ok(Default::default()),
                maxset: Ok(Default::default()),
                nullability: Ok(Default::default()),
                options: Ok(Default::default()),
                ordered: Ok(Default::default()),
                return_: Err("no value supplied for return_".to_string()),
                session_dependent: Ok(Default::default()),
                variadic: Ok(Default::default()),
                window_type: Ok(Default::default()),
            }
        }
    }
    impl WindowFunctionImplsItem {
        pub fn args<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Arguments>>,
            T::Error: ::std::fmt::Display,
        {
            self.args = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for args: {}", e));
            self
        }
        pub fn decomposable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Decomposable>>,
            T::Error: ::std::fmt::Display,
        {
            self.decomposable = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for decomposable: {}", e)
                });
            self
        }
        pub fn deterministic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Deterministic>>,
            T::Error: ::std::fmt::Display,
        {
            self.deterministic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for deterministic: {}", e)
                });
            self
        }
        pub fn implementation<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Implementation>>,
            T::Error: ::std::fmt::Display,
        {
            self.implementation = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for implementation: {}", e)
                });
            self
        }
        pub fn intermediate<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Intermediate>>,
            T::Error: ::std::fmt::Display,
        {
            self.intermediate = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for intermediate: {}", e)
                });
            self
        }
        pub fn maxset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Maxset>>,
            T::Error: ::std::fmt::Display,
        {
            self.maxset = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for maxset: {}", e)
                });
            self
        }
        pub fn nullability<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::NullabilityHandling>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.nullability = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for nullability: {}", e)
                });
            self
        }
        pub fn options<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Options>>,
            T::Error: ::std::fmt::Display,
        {
            self.options = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for options: {}", e)
                });
            self
        }
        pub fn ordered<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Ordered>>,
            T::Error: ::std::fmt::Display,
        {
            self.ordered = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for ordered: {}", e)
                });
            self
        }
        pub fn return_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReturnValue>,
            T::Error: ::std::fmt::Display,
        {
            self.return_ = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for return_: {}", e)
                });
            self
        }
        pub fn session_dependent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SessionDependent>>,
            T::Error: ::std::fmt::Display,
        {
            self.session_dependent = value
                .try_into()
                .map_err(|e| {
                    format!(
                        "error converting supplied value for session_dependent: {}", e
                    )
                });
            self
        }
        pub fn variadic<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VariadicBehavior>>,
            T::Error: ::std::fmt::Display,
        {
            self.variadic = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for variadic: {}", e)
                });
            self
        }
        pub fn window_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                ::std::option::Option<super::WindowFunctionImplsItemWindowType>,
            >,
            T::Error: ::std::fmt::Display,
        {
            self.window_type = value
                .try_into()
                .map_err(|e| {
                    format!("error converting supplied value for window_type: {}", e)
                });
            self
        }
    }
    impl ::std::convert::TryFrom<WindowFunctionImplsItem>
    for super::WindowFunctionImplsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: WindowFunctionImplsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                args: value.args?,
                decomposable: value.decomposable?,
                deterministic: value.deterministic?,
                implementation: value.implementation?,
                intermediate: value.intermediate?,
                maxset: value.maxset?,
                nullability: value.nullability?,
                options: value.options?,
                ordered: value.ordered?,
                return_: value.return_?,
                session_dependent: value.session_dependent?,
                variadic: value.variadic?,
                window_type: value.window_type?,
            })
        }
    }
    impl ::std::convert::From<super::WindowFunctionImplsItem>
    for WindowFunctionImplsItem {
        fn from(value: super::WindowFunctionImplsItem) -> Self {
            Self {
                args: Ok(value.args),
                decomposable: Ok(value.decomposable),
                deterministic: Ok(value.deterministic),
                implementation: Ok(value.implementation),
                intermediate: Ok(value.intermediate),
                maxset: Ok(value.maxset),
                nullability: Ok(value.nullability),
                options: Ok(value.options),
                ordered: Ok(value.ordered),
                return_: Ok(value.return_),
                session_dependent: Ok(value.session_dependent),
                variadic: Ok(value.variadic),
                window_type: Ok(value.window_type),
            }
        }
    }
}

}