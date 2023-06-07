// SPDX-License-Identifier: Apache-2.0

//! Parsing of [text::simple_extensions].

use std::collections::HashSet;

use super::{Context, Parse};
use crate::text::{
    self,
    simple_extensions::{AggregateFunction, ArgumentsItem, ScalarFunction, Type, WindowFunction},
};
use thiserror::Error;

/// A parsed [text::simple_extensions::SimpleExtensions].
pub struct SimpleExtensions {
    /// The deserialized simple extensions
    pub(crate) simple_extensions: text::simple_extensions::SimpleExtensions,
}

impl SimpleExtensions {
    /// Returns the aggregate functions of this simple extensions.
    pub fn aggregate_functions(&self) -> &[AggregateFunction] {
        &self.simple_extensions.aggregate_functions
    }

    /// Returns the scalar functions of this simple extensions.
    pub fn scalar_functions(&self) -> &[ScalarFunction] {
        &self.simple_extensions.scalar_functions
    }

    /// Returns the window functions of this simple extensions.
    pub fn window_functions(&self) -> &[WindowFunction] {
        &self.simple_extensions.window_functions
    }

    /// Returns an iterator over all the functions in this simple extensions.
    pub fn functions(&self) -> impl Iterator<Item = SimpleExtensionFunction> + '_ {
        self.aggregate_functions()
            .iter()
            .map(SimpleExtensionFunction::from)
            .chain(
                self.scalar_functions()
                    .iter()
                    .map(SimpleExtensionFunction::from),
            )
            .chain(
                self.window_functions()
                    .iter()
                    .map(SimpleExtensionFunction::from),
            )
    }
}

/// Parse errors for [text::simple_extensions::SimpleExtensions].
#[derive(Debug, Error)]
pub enum SimpleExtensionsError {
    /// Duplicate type name.
    #[error("duplicate type name `{0}`")]
    DuplicateTypeName(String),
}

impl<C: Context> Parse<C> for text::simple_extensions::SimpleExtensions {
    type Parsed = SimpleExtensions;
    type Error = SimpleExtensionsError;

    fn parse(self, _ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        let simple_extensions = Self::Parsed {
            simple_extensions: self,
        };

        // Make sure the type names are unique
        let mut type_names = HashSet::new();
        if let Some(item) = simple_extensions
            .simple_extensions
            .types
            .iter()
            .find(|item| !type_names.insert(item.name.as_str()))
        {
            Err(Self::Error::DuplicateTypeName(item.name.clone()))?;
        }

        // Make sure the type variation names are unique
        let mut type_variation_names = HashSet::new();
        if let Some(item) = simple_extensions
            .simple_extensions
            .type_variations
            .iter()
            .find(|item| !type_variation_names.insert(&item.name))
        {
            Err(Self::Error::DuplicateTypeName(item.name.clone()))?;
        }

        // Make sure the function compound names are unique

        // Build the function map

        // todo(mbrobbel): parse

        Ok(simple_extensions)
    }
}

impl From<SimpleExtensions> for text::simple_extensions::SimpleExtensions {
    fn from(value: SimpleExtensions) -> Self {
        let SimpleExtensions {
            simple_extensions, ..
        } = value;

        simple_extensions
    }
}

/// A simple extension function definition.
#[derive(Clone)]
pub enum SimpleExtensionFunction {
    /// An aggregate function
    AggregateFunction(AggregateFunction),
    /// A scalar function
    ScalarFunction(ScalarFunction),
    /// A window function
    WindowFunction(WindowFunction),
}

/// Rename type string to compound signature
fn rename_type_for_compound_signature(type_: &str) -> &str {
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

impl SimpleExtensionFunction {
    /// Returns the name of this simple extension function.
    pub fn name(&self) -> &str {
        match self {
            Self::AggregateFunction(aggregate_function) => &aggregate_function.name,
            Self::ScalarFunction(scalar_function) => &scalar_function.name,
            Self::WindowFunction(window_function) => &window_function.name,
        }
    }

    /// Return all the names of this simple extension function (name and compound names).
    pub fn names(&self) -> impl Iterator<Item = String> + '_ {
        std::iter::once(self.name().to_string()).chain(self.compound_names())
    }

    /// Returns the compound names of this simple extensions function.
    pub fn compound_names(&self) -> Vec<String> {
        // todo(mbrobbel): parse functions
        // notes :
        // for enums resolve the type via name and check if optional (skip optional in name)
        match self {
            Self::AggregateFunction(aggregate_function) => aggregate_function
                .impls
                .iter()
                .map(|item| {
                    format!(
                        "{}:{}",
                        aggregate_function.name,
                        item.args
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter_map(|arg| match arg {
                                ArgumentsItem::Variant1 {
                                    value: Type::Variant0(item),
                                    ..
                                } => Some(rename_type_for_compound_signature(item).to_string()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("_")
                    )
                })
                .collect(),
            Self::ScalarFunction(scalar_function) => scalar_function
                .impls
                .iter()
                .map(|item| {
                    format!(
                        "{}:{}",
                        scalar_function.name,
                        item.args
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter_map(|arg| match arg {
                                ArgumentsItem::Variant1 {
                                    value: Type::Variant0(item),
                                    ..
                                } => Some(rename_type_for_compound_signature(item).to_string()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("_")
                    )
                })
                .collect(),
            Self::WindowFunction(window_function) => window_function
                .impls
                .iter()
                .map(|item| {
                    format!(
                        "{}:{}",
                        window_function.name,
                        item.args
                            .as_ref()
                            .unwrap()
                            .iter()
                            .filter_map(|arg| match arg {
                                ArgumentsItem::Variant1 {
                                    value: Type::Variant0(item),
                                    ..
                                } => Some(rename_type_for_compound_signature(item).to_string()),
                                _ => None,
                            })
                            .collect::<Vec<_>>()
                            .join("_")
                    )
                })
                .collect(),
        }
    }
}

impl From<&AggregateFunction> for SimpleExtensionFunction {
    fn from(value: &AggregateFunction) -> Self {
        Self::AggregateFunction(value.clone())
    }
}

impl From<&ScalarFunction> for SimpleExtensionFunction {
    fn from(value: &ScalarFunction) -> Self {
        Self::ScalarFunction(value.clone())
    }
}

impl From<&WindowFunction> for SimpleExtensionFunction {
    fn from(value: &WindowFunction) -> Self {
        Self::WindowFunction(value.clone())
    }
}
