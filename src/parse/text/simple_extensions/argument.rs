// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

use crate::{
    parse::{Context, Parse},
    text::simple_extensions,
};

/// A parsed [simple_extensions::ArgumentsItem].
#[derive(Clone, Debug)]
pub enum ArgumentsItem {
    /// Arguments that support a fixed set of declared values as constant arguments.
    EnumArgument(EnumArgument),

    /// Arguments that refer to a data value.
    ValueArgument(ValueArgument),

    /// Arguments that are used only to inform the evaluation and/or type derivation of the function.
    TypeArgument(TypeArgument),
}

impl ArgumentsItem {
    #[inline]
    fn parse_name(name: Option<String>) -> Result<String, ArgumentsItemError> {
        match name {
            Some(s) if s.is_empty() => {
                Err(ArgumentsItemError::EmptyOptionalField("name".to_string()))
            }
            _ => Ok(name.unwrap_or_default()),
        }
    }

    #[inline]
    fn parse_description(description: Option<String>) -> Result<String, ArgumentsItemError> {
        match description {
            Some(s) if s.is_empty() => Err(ArgumentsItemError::EmptyOptionalField(
                "description".to_string(),
            )),
            _ => Ok(description.unwrap_or_default()),
        }
    }
}

impl<C: Context> Parse<C> for simple_extensions::ArgumentsItem {
    type Parsed = ArgumentsItem;
    type Error = ArgumentsItemError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        match self {
            simple_extensions::ArgumentsItem::Variant0 {
                name,
                description,
                options,
            } => EnumArgument::parse(name, description, options).map(Into::into),

            simple_extensions::ArgumentsItem::Variant1 {
                name,
                description,
                value,
                constant,
            } => ValueArgument::parse(ctx, name, description, value, constant).map(Into::into),

            simple_extensions::ArgumentsItem::Variant2 {
                name,
                description,
                type_,
            } => TypeArgument::parse(ctx, name, description, type_).map(Into::into),
        }
    }
}

impl From<ArgumentsItem> for simple_extensions::ArgumentsItem {
    fn from(value: ArgumentsItem) -> Self {
        match value {
            ArgumentsItem::EnumArgument(EnumArgument {
                name,
                description,
                options,
            }) => simple_extensions::ArgumentsItem::Variant0 {
                name: Some(name),
                description: Some(description),
                options,
            },

            ArgumentsItem::ValueArgument(ValueArgument {
                name,
                description,
                value,
                constant,
            }) => simple_extensions::ArgumentsItem::Variant1 {
                name: Some(name),
                description: Some(description),
                value,
                constant: Some(constant),
            },

            ArgumentsItem::TypeArgument(TypeArgument {
                name,
                description,
                type_,
            }) => simple_extensions::ArgumentsItem::Variant2 {
                name: Some(name),
                description: Some(description),
                type_,
            },
        }
    }
}

/// Parse errors for [simple_extensions::ArgumentsItem].
#[derive(Debug, Error, PartialEq)]
pub enum ArgumentsItemError {
    /// Empty enumeration option.
    #[error("empty enumeration option")]
    EmptyEnumOption,

    /// Empty optional field.
    #[error("the optional field `{0}` is empty and should be removed")]
    EmptyOptionalField(String),
}

/// Arguments that support a fixed set of declared values as constant arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumArgument {
    /// A human-readable name for this argument to help clarify use.
    name: String,

    /// Additional description for this argument.
    description: String,

    /// List of valid string options for this argument.
    options: Vec<String>,
}

impl EnumArgument {
    fn parse(
        name: Option<String>,
        description: Option<String>,
        options: Vec<String>,
    ) -> Result<EnumArgument, ArgumentsItemError> {
        if options.iter().any(String::is_empty) {
            return Err(ArgumentsItemError::EmptyEnumOption);
        }

        Ok(EnumArgument {
            name: ArgumentsItem::parse_name(name)?,
            description: ArgumentsItem::parse_description(description)?,
            options,
        })
    }
}

impl From<EnumArgument> for ArgumentsItem {
    fn from(value: EnumArgument) -> Self {
        ArgumentsItem::EnumArgument(value)
    }
}

/// Arguments that refer to a data value.
#[derive(Clone, Debug)]
pub struct ValueArgument {
    /// A human-readable name for this argument to help clarify use.
    name: String,

    /// Additional description for this argument.
    description: String,

    /// A fully defined type or a type expression.
    ///
    /// todo: implement parsed [simple_extensions::Type].
    value: simple_extensions::Type,

    /// Whether this argument is required to be a constant for invocation.
    /// For example, in some system a regular expression pattern would only be accepted as a literal
    /// and not a column value reference.
    constant: bool,
}

impl ValueArgument {
    fn parse<C: Context>(
        // Retain this parameter for future value parsing
        _ctx: &mut C,
        name: Option<String>,
        description: Option<String>,
        value: simple_extensions::Type,
        constant: Option<bool>,
    ) -> Result<ValueArgument, ArgumentsItemError> {
        Ok(ValueArgument {
            name: ArgumentsItem::parse_name(name)?,
            description: ArgumentsItem::parse_description(description)?,
            value,
            constant: constant.unwrap_or_default(),
        })
    }
}

impl From<ValueArgument> for ArgumentsItem {
    fn from(value: ValueArgument) -> Self {
        ArgumentsItem::ValueArgument(value)
    }
}

/// Arguments that are used only to inform the evaluation and/or type derivation of the function.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeArgument {
    /// A human-readable name for this argument to help clarify use.
    name: String,

    /// Additional description for this argument.
    description: String,

    /// A partially or completely parameterized type. E.g. `List<K>` or `K`.
    ///
    /// todo: implement parsed [simple_extensions::Type].
    type_: String,
}

impl TypeArgument {
    fn parse<C: Context>(
        // Retain this parameter for future type parsing
        _ctx: &mut C,
        name: Option<String>,
        description: Option<String>,
        type_: String,
    ) -> Result<TypeArgument, ArgumentsItemError> {
        Ok(TypeArgument {
            name: ArgumentsItem::parse_name(name)?,
            description: ArgumentsItem::parse_description(description)?,
            type_,
        })
    }
}

impl From<TypeArgument> for ArgumentsItem {
    fn from(value: TypeArgument) -> Self {
        ArgumentsItem::TypeArgument(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::text::simple_extensions;
    use crate::{parse::context::tests::Context, text};

    #[test]
    fn parse_enum_argument() -> Result<(), ArgumentsItemError> {
        let enum_argument = simple_extensions::ArgumentsItem::Variant0 {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            options: vec!["OVERFLOW".to_string()],
        };
        let item = enum_argument.parse(&mut Context::default())?;
        let enum_argument = match item {
            ArgumentsItem::EnumArgument(enum_argument) => enum_argument,
            _ => unreachable!(),
        };
        assert_eq!(
            enum_argument,
            EnumArgument {
                name: "arg".to_string(),
                description: "desc".to_string(),
                options: vec!["OVERFLOW".to_string()],
            }
        );
        Ok(())
    }

    #[test]
    fn parse_enum_argument_with_empty_option() -> Result<(), ArgumentsItemError> {
        let enum_argument = simple_extensions::ArgumentsItem::Variant0 {
            name: None,
            description: Some("desc".to_string()),
            options: vec!["".to_string()],
        };
        let is_err = enum_argument
            .parse(&mut Context::default())
            .err()
            .map(|err| matches!(err, ArgumentsItemError::EmptyEnumOption));
        assert_eq!(is_err, Some(true));
        Ok(())
    }

    #[test]
    fn parse_value_argument() -> Result<(), ArgumentsItemError> {
        let item = simple_extensions::ArgumentsItem::Variant1 {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            value: text::simple_extensions::Type::Variant0("".to_string()),
            constant: Some(true),
        };
        let item = item.parse(&mut Context::default())?;
        match item {
            ArgumentsItem::ValueArgument(ValueArgument {
                name,
                description,
                value,
                constant,
            }) => {
                assert_eq!(name, "arg");
                assert_eq!(description, "desc");
                assert!(
                    matches!(value, text::simple_extensions::Type::Variant0(type_) if type_.is_empty())
                );
                assert!(constant);
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[test]
    fn parse_type_argument() -> Result<(), ArgumentsItemError> {
        let type_argument = simple_extensions::ArgumentsItem::Variant2 {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            type_: "".to_string(),
        };
        let item = type_argument.parse(&mut Context::default())?;
        match item {
            ArgumentsItem::TypeArgument(TypeArgument {
                name,
                description,
                type_,
            }) => {
                assert_eq!(name, "arg");
                assert_eq!(description, "desc");
                assert_eq!(type_, "");
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[test]
    fn parse_argument_with_nones() -> Result<(), ArgumentsItemError> {
        let items = vec![
            simple_extensions::ArgumentsItem::Variant0 {
                name: None,
                description: None,
                options: vec!["OVERFLOW".to_string()],
            },
            simple_extensions::ArgumentsItem::Variant1 {
                name: None,
                description: None,
                value: text::simple_extensions::Type::Variant0("".to_string()),
                constant: Some(true),
            },
            simple_extensions::ArgumentsItem::Variant2 {
                name: None,
                description: None,
                type_: "".to_string(),
            },
        ];

        for item in items {
            let item = item.parse(&mut Context::default())?;
            let (name, description) = match item {
                ArgumentsItem::EnumArgument(EnumArgument {
                    name, description, ..
                }) => (name, description),

                ArgumentsItem::ValueArgument(ValueArgument {
                    name, description, ..
                }) => (name, description),

                ArgumentsItem::TypeArgument(TypeArgument {
                    name, description, ..
                }) => (name, description),
            };
            assert_eq!(name, "");
            assert_eq!(description, "");
        }

        Ok(())
    }

    #[test]
    fn parse_argument_with_empty_fields() -> Result<(), ArgumentsItemError> {
        let items = vec![
            simple_extensions::ArgumentsItem::Variant0 {
                name: Some("".to_string()),
                description: Some("desc".to_string()),
                options: vec!["OVERFLOW".to_string()],
            },
            simple_extensions::ArgumentsItem::Variant1 {
                name: Some("".to_string()),
                description: Some("desc".to_string()),
                value: text::simple_extensions::Type::Variant0("".to_string()),
                constant: Some(true),
            },
            simple_extensions::ArgumentsItem::Variant2 {
                name: Some("".to_string()),
                description: Some("desc".to_string()),
                type_: "".to_string(),
            },
        ];
        for item in items {
            assert_eq!(
                item.parse(&mut Context::default()).err(),
                Some(ArgumentsItemError::EmptyOptionalField("name".to_string()))
            );
        }

        let items = vec![
            simple_extensions::ArgumentsItem::Variant0 {
                name: Some("arg".to_string()),
                description: Some("".to_string()),
                options: vec!["OVERFLOW".to_string()],
            },
            simple_extensions::ArgumentsItem::Variant1 {
                name: Some("arg".to_string()),
                description: Some("".to_string()),
                value: text::simple_extensions::Type::Variant0("".to_string()),
                constant: Some(true),
            },
            simple_extensions::ArgumentsItem::Variant2 {
                name: Some("arg".to_string()),
                description: Some("".to_string()),
                type_: "".to_string(),
            },
        ];
        for item in items {
            assert_eq!(
                item.parse(&mut Context::default()).err(),
                Some(ArgumentsItemError::EmptyOptionalField(
                    "description".to_string()
                ))
            );
        }

        Ok(())
    }

    #[test]
    fn from_enum_argument() {
        let item: ArgumentsItem = EnumArgument {
            name: "arg".to_string(),
            description: "desc".to_string(),
            options: vec!["OVERFLOW".to_string()],
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::Variant0 {
                name,
                description,
                options,
            } => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert_eq!(options, vec!["OVERFLOW".to_string()]);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn from_value_argument() {
        let item: ArgumentsItem = ValueArgument {
            name: "arg".to_string(),
            description: "desc".to_string(),
            value: text::simple_extensions::Type::Variant0("".to_string()),
            constant: true,
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::Variant1 {
                name,
                description,
                value,
                constant,
            } => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert!(
                    matches!(value, text::simple_extensions::Type::Variant0(type_) if type_.is_empty())
                );
                assert_eq!(constant, Some(true));
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn from_type_argument() {
        let item: ArgumentsItem = TypeArgument {
            name: "arg".to_string(),
            description: "desc".to_string(),
            type_: "".to_string(),
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::Variant2 {
                name,
                description,
                type_,
            } => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert_eq!(type_, "");
            }
            _ => unreachable!(),
        }
    }
}
