// SPDX-License-Identifier: Apache-2.0

//! Table function definitions parsed from extension YAML.

use crate::text::simple_extensions::{
    TableFunction as RawTableFunction, TableFunctionImplsItem as RawImpl,
};

use super::argument::ArgumentsItem;
use super::extensions::TypeContext;
use super::scalar_functions::{
    Options, ScalarFunctionError, VariadicBehavior, parse_arguments, parse_return_type,
};
use super::types::{Type, TypeKind};
use thiserror::Error;

/// Errors that can occur when parsing table functions
#[derive(Debug, Error)]
pub enum TableFunctionError {
    /// Table function has no implementations
    #[error("Table function '{name}' must have at least one implementation")]
    NoImplementations {
        /// The function name
        name: String,
    },
    /// The declared schema is not a non-nullable struct
    #[error("Table function '{name}' schema must be a non-nullable struct, got {schema}")]
    InvalidSchema {
        /// The function name
        name: String,
        /// The declared schema
        schema: Type,
    },
    /// Error in an implementation's arguments, variadic behavior or schema type
    #[error(transparent)]
    Signature(#[from] ScalarFunctionError),
}

/// A validated table function definition with one or more implementations
#[derive(Clone, Debug, PartialEq)]
pub struct TableFunction {
    /// Function name
    pub name: String,
    /// Human-readable description
    pub description: Option<String>,
    /// Function implementations (overloads)
    pub impls: Vec<Impl>,
}

impl TableFunction {
    /// Parse a table function from raw YAML with the provided type context.
    pub(super) fn from_raw(
        raw: RawTableFunction,
        ctx: &mut TypeContext,
    ) -> Result<Self, TableFunctionError> {
        if raw.impls.is_empty() {
            return Err(TableFunctionError::NoImplementations { name: raw.name });
        }

        let impls = raw
            .impls
            .into_iter()
            .map(|impl_| Impl::from_raw(impl_, &raw.name, ctx))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(TableFunction {
            name: raw.name,
            description: raw.description,
            impls,
        })
    }
}

/// A single table function implementation (overload) with parsed signature types.
#[derive(Clone, Debug, PartialEq)]
pub struct Impl {
    /// Function arguments with types and optional names/descriptions
    pub args: Vec<ArgumentsItem>,
    /// Configurable function options
    pub options: Options,
    /// Variadic argument behavior.
    ///
    /// `None` indicates the function is not variadic.
    pub variadic: Option<VariadicBehavior>,
    /// The schema of the rows the function produces, always a non-nullable struct.
    ///
    /// `None` when the schema depends on the call, for example on the value of
    /// a constant argument; the producer then states it in the plan.
    pub schema: Option<Type>,
}

impl Impl {
    /// Parse an implementation from raw YAML with the provided type context.
    pub(super) fn from_raw(
        raw: RawImpl,
        name: &str,
        ctx: &mut TypeContext,
    ) -> Result<Self, TableFunctionError> {
        let schema = raw
            .schema
            .map(|s| parse_return_type(s.0, ctx))
            .transpose()?;
        if let Some(schema) = &schema {
            let is_struct = matches!(
                schema.kind,
                TypeKind::Struct(_) | TypeKind::NamedStruct { .. }
            );
            if !is_struct || schema.nullable {
                return Err(TableFunctionError::InvalidSchema {
                    name: name.to_string(),
                    schema: schema.clone(),
                });
            }
        }

        Ok(Impl {
            args: parse_arguments(raw.args, ctx)?,
            options: raw.options.as_ref().map(Options::from).unwrap_or_default(),
            variadic: raw.variadic.map(VariadicBehavior::try_from).transpose()?,
            schema,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::simple_extensions::{ReturnValue, Type as RawType};

    fn raw_impl(schema: Option<&str>) -> RawImpl {
        RawImpl {
            schema: schema.map(|s| ReturnValue(RawType::String(s.to_string()))),
            ..Default::default()
        }
    }

    fn raw_function(impls: Vec<RawImpl>) -> RawTableFunction {
        RawTableFunction {
            deprecated: None,
            description: None,
            impls,
            metadata: Default::default(),
            name: "series".to_string(),
        }
    }

    #[test]
    fn test_no_implementations_error() {
        let mut ctx = TypeContext::default();
        let result = TableFunction::from_raw(raw_function(vec![]), &mut ctx);

        assert!(matches!(
            result,
            Err(TableFunctionError::NoImplementations { name }) if name == "series"
        ));
    }

    #[test]
    fn test_schema() {
        let mut ctx = TypeContext::default();
        let result = TableFunction::from_raw(
            raw_function(vec![raw_impl(Some("struct<i64, string?>"))]),
            &mut ctx,
        )
        .unwrap();

        assert_eq!(
            result.impls[0].schema,
            Some(Type::new(
                TypeKind::Struct(vec![
                    Type::new(TypeKind::I64, false),
                    Type::new(TypeKind::String, true),
                ]),
                false,
            ))
        );
    }

    #[test]
    fn test_schema_absent() {
        let mut ctx = TypeContext::default();
        let result = TableFunction::from_raw(raw_function(vec![raw_impl(None)]), &mut ctx).unwrap();

        assert_eq!(result.impls[0].schema, None);
    }

    #[test]
    fn test_invalid_schema() {
        for schema in ["i64", "struct?<i64>"] {
            let mut ctx = TypeContext::default();
            let result =
                TableFunction::from_raw(raw_function(vec![raw_impl(Some(schema))]), &mut ctx);

            assert!(
                matches!(result, Err(TableFunctionError::InvalidSchema { .. })),
                "expected an invalid schema error for {schema}, got {result:?}"
            );
        }
    }
}
