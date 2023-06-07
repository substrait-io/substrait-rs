// SPDX-License-Identifier: Apache-2.0

//! Parsing of functions in [text::simple_extensions].

use std::collections::{HashMap, HashSet};

use crate::{
    parse::{Context, Parse},
    text::simple_extensions,
};
use thiserror::Error;

/// A simple extension function definition.
// #[derive(Clone)]
pub enum SimpleExtensionFunction {
    /// An aggregate function
    AggregateFunction(AggregateFunction),

    /// A scalar function
    ScalarFunction(ScalarFunction),

    /// A window function
    WindowFunction(WindowFunction),
}

/// A parsed [simple_extensions::AggregateFunction].
pub struct AggregateFunction {
    /// The name of this function.
    name: String,

    /// An optional description of this function.
    description: Option<String>,

    /// The implementations of this function.
    impls: Vec<AggregateFunctionImplsItem>,
}

/// Parse errors for [simple_extensions::AggregateFunction].
#[derive(Debug, Error)]
pub enum AggregateFunctionError {
    /// Invalid aggregate function impl.
    #[error("invalid aggregate function impl")]
    AggregateFunctionImpl(#[from] AggregateFunctionImplsItemError),
}

impl<C: Context> Parse<C> for simple_extensions::AggregateFunction {
    type Parsed = AggregateFunction;
    type Error = AggregateFunctionError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let simple_extensions::AggregateFunction {
            description,
            impls,
            name,
        } = self;

        // Parse the impls.
        let impls = impls
            .into_iter()
            .enumerate()
            .map(|(index, aggregate_function_impl)| {
                ctx.parse_record_index(aggregate_function_impl, index)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::Parsed {
            name,
            description,
            impls,
        })
    }
}

impl From<AggregateFunction> for simple_extensions::AggregateFunction {
    fn from(value: AggregateFunction) -> Self {
        let AggregateFunction {
            name,
            description,
            impls,
        } = value;

        Self {
            name,
            description,
            impls: impls.into_iter().map(Into::into).collect(),
        }
    }
}

/// Parse errors for [simple_extensions::AggregateFunctionImplsItem].
#[derive(Debug, Error)]
#[error("todo")]
pub enum AggregateFunctionImplsItemError {
    /// Invalid arguments
    #[error("invalid arguments")]
    Arguments(#[from] ArgumentsError),
}

/// A parsed [simple_extensions::AggregateFunctionImplsItem].
pub struct AggregateFunctionImplsItem {
    /// Arguments
    args: Option<Arguments>,
}

impl<C: Context> Parse<C> for simple_extensions::AggregateFunctionImplsItem {
    type Parsed = AggregateFunctionImplsItem;
    type Error = AggregateFunctionImplsItemError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let simple_extensions::AggregateFunctionImplsItem {
            args,
            decomposable,
            deterministic,
            implementation,
            intermediate,
            maxset,
            nullability,
            options,
            ordered,
            return_,
            session_dependent,
            variadic,
        } = self;

        // Parse the arguments (arguments are optional so fine if missing).
        let args = args.map(|arguments| ctx.parse(arguments)).transpose()?;

        todo!("parse aggregate function impl")
    }
}

impl From<AggregateFunctionImplsItem> for simple_extensions::AggregateFunctionImplsItem {
    fn from(value: AggregateFunctionImplsItem) -> Self {
        todo!("convert")
    }
}

/// A parsed [simple_extensions::Arguments].
pub struct Arguments {
    /// The arguments items.
    arguments: Vec<ArgumentsItem>,
}

/// Parse errors for [simple_extensions::Arguments].
#[derive(Debug, Error)]
pub enum ArgumentsError {
    /// Invalid argument item
    #[error("invalid arguments item")]
    ArgumentsItem(#[from] ArgumentsItemError),
}

impl<C: Context> Parse<C> for simple_extensions::Arguments {
    type Parsed = Arguments;
    type Error = ArgumentsError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let simple_extensions::Arguments(items) = self;

        let arguments = items
            .into_iter()
            .enumerate()
            .map(|(index, arguments_item)| ctx.parse_record_value_with_index(arguments_item, index))
            .collect::<Result<_, _>>()?;

        Ok(Self::Parsed { arguments })
    }
}

impl From<Arguments> for simple_extensions::Arguments {
    fn from(value: Arguments) -> Self {
        let Arguments { arguments } = value;

        simple_extensions::Arguments(arguments.into_iter().map(Into::into).collect())
    }
}

/// A parsed [simple_extensions::ArgumentsItem].
pub enum ArgumentsItem {
    /// Enumeration argument.
    Enumeration(EnumerationArgument),

    /// Value argument.
    Value(ValueArgument),

    /// Type argument.
    Type(TypeArgument),
}

/// Parse errors for [simple_extensions::ArgumentsItem].
#[derive(Debug, Error)]
pub enum ArgumentsItemError {
    /// Invalid enumeration argument
    #[error("invalid enumeration argument")]
    EnumerationArgument(#[from] EnumerationArgumentError),

    /// Invalid value argument
    #[error("invalid value argument")]
    ValueArgument(#[from] ValueArgumentError),
}

impl<C: Context> Parse<C> for simple_extensions::ArgumentsItem {
    type Parsed = ArgumentsItem;
    type Error = ArgumentsItemError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        match self {
            // Enumeration
            simple_extensions::ArgumentsItem::Variant0 {
                description,
                name,
                options,
            } => {
                // Make sure there is at least one option
                if options.is_empty() {
                    Err(EnumerationArgumentError::MissingOptions)?;
                }

                // Make sure options are unique
                let mut set = HashSet::with_capacity(options.len());
                if let Some(option) = options.iter().find(|&option| !set.insert(option)) {
                    Err(EnumerationArgumentError::DuplicateOption(option.clone()))?;
                }

                Ok(Self::Parsed::Enumeration(EnumerationArgument {
                    name,
                    description,
                    options,
                }))
            }

            // Value
            simple_extensions::ArgumentsItem::Variant1 {
                constant,
                description,
                name,
                value,
            } => {
                // Parse type
                let value = ctx
                    .parse_record_value(value)
                    .map_err(ValueArgumentError::Type)?;

                Ok(Self::Parsed::Value(ValueArgument {
                    constant,
                    description,
                    name,
                    value,
                }))
            }

            // Type
            simple_extensions::ArgumentsItem::Variant2 {
                description,
                name,
                type_,
            } => todo!()
            // Ok(Self::Parsed::Type(TypeArgument {
            //     description,
            //     name,
            //     r#type: type_,
            // })),
        }
    }
}

impl From<ArgumentsItem> for simple_extensions::ArgumentsItem {
    fn from(value: ArgumentsItem) -> Self {
        match value {
            ArgumentsItem::Enumeration(enumeration_argument) => enumeration_argument.into(),
            ArgumentsItem::Value(value_argument) => todo!(),
            ArgumentsItem::Type(type_argument) => todo!(),
        }
    }
}

/// A parsed [simple_extensions::ArgumentsItem::Variant0].
pub struct EnumerationArgument {
    /// The optional name of this argument
    name: Option<String>,

    /// The optional description of this argument.
    description: Option<String>,

    /// The list of options for this argument.
    options: Vec<String>,
}

/// Parse errors for [simple_extensions::ArgumentsItem::Variant0].
#[derive(Debug, Error)]
pub enum EnumerationArgumentError {
    /// Enumeration argument options are required
    #[error("enumeration argument options are required")]
    MissingOptions,

    /// Enumeration argument options contain duplicates
    #[error("duplicate enumeration argument option: `{0}`")]
    DuplicateOption(String),
}

impl From<EnumerationArgument> for simple_extensions::ArgumentsItem {
    fn from(value: EnumerationArgument) -> Self {
        let EnumerationArgument {
            name,
            description,
            options,
        } = value;

        simple_extensions::ArgumentsItem::Variant0 {
            description,
            name,
            options,
        }
    }
}

/// A parsed [simple_extensions::ArgumentsItem::Variant1].
pub struct ValueArgument {
    /// Whether this argument is required to be a constant for invocation.
    constant: Option<bool>,

    /// The optional description of this argument.
    description: Option<String>,

    /// The optional name of this argument.
    name: Option<String>,

    /// The type of the value.
    value: Type,
}

/// Parse errors for [simple_extensions::ArgumentsItem::Variant1].
#[derive(Debug, Error)]
pub enum ValueArgumentError {
    /// Invalid type
    #[error("invalid type")]
    Type(#[from] TypeError),
}

impl From<ValueArgument> for simple_extensions::ArgumentsItem {
    fn from(value: ValueArgument) -> Self {
        let ValueArgument {
            constant,
            description,
            name,
            value,
        } = value;

        simple_extensions::ArgumentsItem::Variant1 {
            constant,
            description,
            name,
            value: value.into(),
        }
    }
}

/// A parsed [simple_extensions::Type].
pub enum Type {
    /// Simple type.
    Simple(String), // todo parse

    /// A compound type.
    Compound(HashMap<String, serde_json::Value>), // todo parse
}

/// Parse errors for [simple_extensions::Type].
#[derive(Debug, Error)]
pub enum TypeError {}

impl<C: Context> Parse<C> for simple_extensions::Type {
    type Parsed = Type;
    type Error = TypeError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        todo!("parse type")
    }
}

impl From<Type> for simple_extensions::Type {
    fn from(value: Type) -> Self {
        match value {
            Type::Simple(simple_type) => simple_extensions::Type::Variant0(simple_type),
            Type::Compound(compound_type) => simple_extensions::Type::Variant1(compound_type),
        }
    }
}

/// A parsed [simple_extensions::ArgumentsItem::Variant2].
pub struct TypeArgument {
    /// An optional description for this type argument.
    description: Option<String>,

    /// An optional name for this type argument.
    name: Option<String>,

    /// The type,
    r#type: TypeDeclaration,
}

/// A type declaration. Based on type syntax.
///
/// Based on https://substrait.io/types/type_parsing/.
pub struct TypeDeclaration {
    /// Case-insensitive name for this type declaration.
    name: String,
    nullability: Option<bool>,
    variation: Option<usize>,
    parameters: Vec<TypeDeclaration>,
}

/// Parse errors for [simple_extensions::ArgumentsItem::Variant2].
#[derive(Debug, Error)]
pub enum TypeArgumentError {}

/// A parsed [simple_extensions::ScalarFunction].
pub struct ScalarFunction {}

/// A parsed [simple_extensions::WindowFunction].
pub struct WindowFunction {}

/// Rename type string to compound signature
fn _rename_type_for_compound_signature(type_: &str) -> &str {
    match type_ {
        // Required Enumeration 	req
        // i8 	i8
        // i16 	i16
        // i32 	i32
        // i64 	i64
        // fp32 	fp32
        // fp64 	fp64
        "string" => "str",
        "binary" => "vbin",
        "timestamp" => "ts",
        "timestamp_tz" => "tstz",
        "date" => "date",
        "time" => "time",
        "interval_year" => "iyear",
        "interval_day" => "iday",
        "uuid" => "uuid",
        "fixedchar<N>" => "fchar",
        "varchar<N>" => "vchar",
        "fixedbinary<N>" => "fbin",
        "decimal<P,S>" => "dec",
        "struct<T1,T2,…,TN>" => "struct",
        "list<T>" => "list",
        "map<K,V>" => "map",
        r#"any[\d]?"# => "any",
        "user defined type" => "u!name",
        _ => panic!("unknown type"),
    }
}
