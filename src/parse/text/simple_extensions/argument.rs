// SPDX-License-Identifier: Apache-2.0

//! Parsing of [simple_extensions::ArgumentsItem].

use thiserror::Error;

use crate::{
    parse::{Context, Parse},
    text::simple_extensions,
};

/// A parsed [simple_extensions::ArgumentsItem].
#[derive(Clone, Debug)]
pub enum ArgumentsItem {
    /// Arguments that support a fixed set of declared values as constant arguments.
    EnumArgument(EnumerationArg),

    /// Arguments that refer to a data value.
    ValueArgument(ValueArg),

    /// Arguments that are used only to inform the evaluation and/or type derivation of the function.
    TypeArgument(TypeArg),
}

impl ArgumentsItem {
    #[inline]
    fn parse_name(name: Option<String>) -> Result<Option<String>, ArgumentsItemError> {
        match name {
            Some(s) if s.is_empty() => {
                Err(ArgumentsItemError::EmptyOptionalField("name".to_string()))
            }
            _ => Ok(name),
        }
    }

    #[inline]
    fn parse_description(
        description: Option<String>,
    ) -> Result<Option<String>, ArgumentsItemError> {
        match description {
            Some(s) if s.is_empty() => Err(ArgumentsItemError::EmptyOptionalField(
                "description".to_string(),
            )),
            _ => Ok(description),
        }
    }
}

impl<C: Context> Parse<C> for simple_extensions::ArgumentsItem {
    type Parsed = ArgumentsItem;
    type Error = ArgumentsItemError;

    fn parse(self, ctx: &mut C) -> Result<Self::Parsed, Self::Error> {
        match self {
            simple_extensions::ArgumentsItem::EnumerationArg(arg) => Ok(ctx.parse(arg)?.into()),
            simple_extensions::ArgumentsItem::ValueArg(arg) => Ok(ctx.parse(arg)?.into()),
            simple_extensions::ArgumentsItem::TypeArg(arg) => Ok(ctx.parse(arg)?.into()),
        }
    }
}

impl From<ArgumentsItem> for simple_extensions::ArgumentsItem {
    fn from(value: ArgumentsItem) -> Self {
        match value {
            ArgumentsItem::EnumArgument(arg) => arg.into(),
            ArgumentsItem::ValueArgument(arg) => arg.into(),
            ArgumentsItem::TypeArgument(arg) => arg.into(),
        }
    }
}

/// Parse errors for [simple_extensions::ArgumentsItem].
#[derive(Debug, Error, PartialEq)]
pub enum ArgumentsItemError {
    /// Invalid enumeration options.
    #[error("invalid enumeration options: {0}")]
    InvalidEnumOptions(#[from] EnumOptionsError),

    /// Empty optional field.
    #[error("the optional field `{0}` is empty and should be removed")]
    EmptyOptionalField(String),
}

/// Arguments that support a fixed set of declared values as constant arguments.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumerationArg {
    /// A human-readable name for this argument to help clarify use.
    name: Option<String>,

    /// Additional description for this argument.
    description: Option<String>,

    /// List of valid string options for this argument.
    options: EnumOptions,
}

impl<C: Context> Parse<C> for simple_extensions::EnumerationArg {
    type Parsed = EnumerationArg;

    type Error = ArgumentsItemError;

    fn parse(self, ctx: &mut C) -> Result<EnumerationArg, ArgumentsItemError> {
        Ok(EnumerationArg {
            name: ArgumentsItem::parse_name(self.name)?,
            description: ArgumentsItem::parse_description(self.description)?,
            options: ctx.parse(self.options)?,
        })
    }
}

impl From<EnumerationArg> for simple_extensions::EnumerationArg {
    fn from(value: EnumerationArg) -> Self {
        simple_extensions::EnumerationArg {
            name: value.name,
            description: value.description,
            options: value.options.into(),
        }
    }
}

impl From<EnumerationArg> for simple_extensions::ArgumentsItem {
    fn from(value: EnumerationArg) -> Self {
        simple_extensions::ArgumentsItem::EnumerationArg(value.into())
    }
}

impl From<EnumerationArg> for ArgumentsItem {
    fn from(value: EnumerationArg) -> Self {
        ArgumentsItem::EnumArgument(value)
    }
}

/// List of valid string options
#[derive(Clone, Debug, PartialEq)]
pub struct EnumOptions(Vec<String>);

impl<C: Context> Parse<C> for simple_extensions::EnumOptions {
    type Parsed = EnumOptions;

    type Error = EnumOptionsError;

    fn parse(self, _ctx: &mut C) -> Result<EnumOptions, EnumOptionsError> {
        let options = self.0;
        if options.is_empty() {
            return Err(EnumOptionsError::EmptyList);
        }

        let mut unique_set = std::collections::HashSet::new();
        for option in options.iter() {
            if option.is_empty() {
                return Err(EnumOptionsError::EmptyOption);
            }
            if !unique_set.insert(option) {
                return Err(EnumOptionsError::DuplicatedOption(option.clone()));
            }
        }

        Ok(EnumOptions(options))
    }
}

impl From<EnumOptions> for simple_extensions::EnumOptions {
    fn from(value: EnumOptions) -> Self {
        simple_extensions::EnumOptions(value.0)
    }
}

/// Parse errors for [simple_extensions::EnumOptions].
#[derive(Debug, Error, PartialEq)]
pub enum EnumOptionsError {
    /// Empty list.
    #[error("empty list")]
    EmptyList,

    /// Duplicated option.
    #[error("duplicated option: {0}")]
    DuplicatedOption(String),

    /// Empty option.
    #[error("empty option")]
    EmptyOption,
}

/// Arguments that refer to a data value.
#[derive(Clone, Debug)]
pub struct ValueArg {
    /// A human-readable name for this argument to help clarify use.
    name: Option<String>,

    /// Additional description for this argument.
    description: Option<String>,

    /// A fully defined type or a type expression.
    ///
    /// todo: implement parsed [simple_extensions::Type].
    value: simple_extensions::Type,

    /// Whether this argument is required to be a constant for invocation.
    /// For example, in some system a regular expression pattern would only be accepted as a literal
    /// and not a column value reference.
    constant: Option<bool>,
}

impl<C: Context> Parse<C> for simple_extensions::ValueArg {
    type Parsed = ValueArg;

    type Error = ArgumentsItemError;

    fn parse(self, _ctx: &mut C) -> Result<ValueArg, ArgumentsItemError> {
        Ok(ValueArg {
            name: ArgumentsItem::parse_name(self.name)?,
            description: ArgumentsItem::parse_description(self.description)?,
            value: self.value,
            constant: self.constant,
        })
    }
}

impl From<ValueArg> for simple_extensions::ValueArg {
    fn from(value: ValueArg) -> Self {
        simple_extensions::ValueArg {
            name: value.name,
            description: value.description,
            value: value.value,
            constant: value.constant,
        }
    }
}

impl From<ValueArg> for simple_extensions::ArgumentsItem {
    fn from(value: ValueArg) -> Self {
        simple_extensions::ArgumentsItem::ValueArg(value.into())
    }
}

impl From<ValueArg> for ArgumentsItem {
    fn from(value: ValueArg) -> Self {
        ArgumentsItem::ValueArgument(value)
    }
}

/// Arguments that are used only to inform the evaluation and/or type derivation of the function.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeArg {
    /// A human-readable name for this argument to help clarify use.
    name: Option<String>,

    /// Additional description for this argument.
    description: Option<String>,

    /// A partially or completely parameterized type. E.g. `List<K>` or `K`.
    ///
    /// todo: implement parsed [simple_extensions::Type].
    type_: String,
}

impl<C: Context> Parse<C> for simple_extensions::TypeArg {
    type Parsed = TypeArg;

    type Error = ArgumentsItemError;

    fn parse(self, _ctx: &mut C) -> Result<TypeArg, ArgumentsItemError> {
        Ok(TypeArg {
            name: ArgumentsItem::parse_name(self.name)?,
            description: ArgumentsItem::parse_description(self.description)?,
            type_: self.type_,
        })
    }
}

impl From<TypeArg> for simple_extensions::TypeArg {
    fn from(value: TypeArg) -> Self {
        simple_extensions::TypeArg {
            name: value.name,
            description: value.description,
            type_: value.type_,
        }
    }
}

impl From<TypeArg> for simple_extensions::ArgumentsItem {
    fn from(value: TypeArg) -> Self {
        simple_extensions::ArgumentsItem::TypeArg(value.into())
    }
}

impl From<TypeArg> for ArgumentsItem {
    fn from(value: TypeArg) -> Self {
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
        let enum_argument =
            simple_extensions::ArgumentsItem::EnumerationArg(simple_extensions::EnumerationArg {
                name: Some("arg".to_string()),
                description: Some("desc".to_string()),
                options: simple_extensions::EnumOptions(vec!["OVERFLOW".to_string()]),
            });
        let item = enum_argument.parse(&mut Context::default())?;
        let enum_argument = match item {
            ArgumentsItem::EnumArgument(enum_argument) => enum_argument,
            _ => unreachable!(),
        };
        assert_eq!(
            enum_argument,
            EnumerationArg {
                name: Some("arg".to_string()),
                description: Some("desc".to_string()),
                options: EnumOptions(vec!["OVERFLOW".to_string()]),
            }
        );
        Ok(())
    }

    #[test]
    fn parse_empty_enum_options() -> Result<(), ArgumentsItemError> {
        let options = simple_extensions::EnumOptions(vec![]);
        let is_err = options
            .parse(&mut Context::default())
            .err()
            .map(|err| matches!(err, EnumOptionsError::EmptyList));
        assert_eq!(is_err, Some(true));
        Ok(())
    }

    #[test]
    fn parse_enum_options_with_empty_value() -> Result<(), ArgumentsItemError> {
        let options = simple_extensions::EnumOptions(vec!["".to_string()]);
        let is_err = options
            .parse(&mut Context::default())
            .err()
            .map(|err| matches!(err, EnumOptionsError::EmptyOption));
        assert_eq!(is_err, Some(true));
        Ok(())
    }

    #[test]
    fn parse_enum_argument_with_duplicated_option() -> Result<(), ArgumentsItemError> {
        let options =
            simple_extensions::EnumOptions(vec!["OVERFLOW".to_string(), "OVERFLOW".to_string()]);
        let is_err = options
            .clone()
            .parse(&mut Context::default())
            .err()
            .map(|err| {
                matches!(
                    err,
                    EnumOptionsError::DuplicatedOption(opt) if opt == "OVERFLOW"
                )
            });
        assert_eq!(is_err, Some(true));
        Ok(())
    }

    #[test]
    fn parse_value_argument() -> Result<(), ArgumentsItemError> {
        let item = simple_extensions::ArgumentsItem::ValueArg(simple_extensions::ValueArg {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            value: text::simple_extensions::Type::Variant0("i32".to_string()),
            constant: Some(true),
        });
        let item = item.parse(&mut Context::default())?;
        match item {
            ArgumentsItem::ValueArgument(ValueArg {
                name,
                description,
                value,
                constant,
            }) => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert!(
                    matches!(value, text::simple_extensions::Type::Variant0(type_) if type_ == "i32")
                );
                assert_eq!(constant, Some(true));
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[test]
    fn parse_type_argument() -> Result<(), ArgumentsItemError> {
        let type_argument = simple_extensions::ArgumentsItem::TypeArg(simple_extensions::TypeArg {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            type_: "".to_string(),
        });
        let item = type_argument.parse(&mut Context::default())?;
        match item {
            ArgumentsItem::TypeArgument(TypeArg {
                name,
                description,
                type_,
            }) => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert_eq!(type_, "");
            }
            _ => unreachable!(),
        };
        Ok(())
    }

    #[test]
    fn parse_argument_with_nones() -> Result<(), ArgumentsItemError> {
        let items = vec![
            simple_extensions::ArgumentsItem::EnumerationArg(simple_extensions::EnumerationArg {
                name: None,
                description: None,
                options: simple_extensions::EnumOptions(vec!["OVERFLOW".to_string()]),
            }),
            simple_extensions::ArgumentsItem::ValueArg(simple_extensions::ValueArg {
                name: None,
                description: None,
                value: text::simple_extensions::Type::Variant0("i32".to_string()),
                constant: None,
            }),
            simple_extensions::ArgumentsItem::TypeArg(simple_extensions::TypeArg {
                name: None,
                description: None,
                type_: "".to_string(),
            }),
        ];

        for item in items {
            let item = item.parse(&mut Context::default())?;
            let (name, description) = match item {
                ArgumentsItem::EnumArgument(EnumerationArg {
                    name, description, ..
                }) => (name, description),

                ArgumentsItem::ValueArgument(ValueArg {
                    name,
                    description,
                    constant,
                    ..
                }) => {
                    assert!(constant.is_none());
                    (name, description)
                }

                ArgumentsItem::TypeArgument(TypeArg {
                    name, description, ..
                }) => (name, description),
            };
            assert!(name.is_none());
            assert!(description.is_none());
        }

        Ok(())
    }

    #[test]
    fn parse_argument_with_empty_fields() -> Result<(), ArgumentsItemError> {
        let items = vec![
            simple_extensions::ArgumentsItem::EnumerationArg(simple_extensions::EnumerationArg {
                name: Some("".to_string()),
                description: None,
                options: simple_extensions::EnumOptions(vec!["OVERFLOW".to_string()]),
            }),
            simple_extensions::ArgumentsItem::ValueArg(simple_extensions::ValueArg {
                name: Some("".to_string()),
                description: None,
                value: text::simple_extensions::Type::Variant0("i32".to_string()),
                constant: None,
            }),
            simple_extensions::ArgumentsItem::TypeArg(simple_extensions::TypeArg {
                name: Some("".to_string()),
                description: None,
                type_: "".to_string(),
            }),
        ];
        for item in items {
            assert_eq!(
                item.parse(&mut Context::default()).err(),
                Some(ArgumentsItemError::EmptyOptionalField("name".to_string()))
            );
        }

        let items = vec![
            simple_extensions::ArgumentsItem::EnumerationArg(simple_extensions::EnumerationArg {
                name: None,
                description: Some("".to_string()),
                options: simple_extensions::EnumOptions(vec!["OVERFLOW".to_string()]),
            }),
            simple_extensions::ArgumentsItem::ValueArg(simple_extensions::ValueArg {
                name: None,
                description: Some("".to_string()),
                value: text::simple_extensions::Type::Variant0("i32".to_string()),
                constant: None,
            }),
            simple_extensions::ArgumentsItem::TypeArg(simple_extensions::TypeArg {
                name: None,
                description: Some("".to_string()),
                type_: "".to_string(),
            }),
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
        let item: ArgumentsItem = EnumerationArg {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            options: EnumOptions(vec!["OVERFLOW".to_string()]),
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::EnumerationArg(
                simple_extensions::EnumerationArg {
                    name,
                    description,
                    options,
                },
            ) => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert_eq!(options.0, vec!["OVERFLOW".to_string()]);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn from_value_argument() {
        let item: ArgumentsItem = ValueArg {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            value: text::simple_extensions::Type::Variant0("".to_string()),
            constant: Some(true),
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::ValueArg(simple_extensions::ValueArg {
                name,
                description,
                value,
                constant,
            }) => {
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
        let item: ArgumentsItem = TypeArg {
            name: Some("arg".to_string()),
            description: Some("desc".to_string()),
            type_: "".to_string(),
        }
        .into();

        let item: text::simple_extensions::ArgumentsItem = item.into();
        match item {
            text::simple_extensions::ArgumentsItem::TypeArg(simple_extensions::TypeArg {
                name,
                description,
                type_,
            }) => {
                assert_eq!(name, Some("arg".to_string()));
                assert_eq!(description, Some("desc".to_string()));
                assert_eq!(type_, "");
            }
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "extensions")]
    #[test]
    fn parse_extensions() {
        use crate::extensions::EXTENSIONS;
        use crate::parse::context::tests::Context;

        macro_rules! parse_arguments {
            ($url:expr, $fns:expr) => {
                $fns.iter().for_each(|f| {
                    f.impls
                        .iter()
                        .filter_map(|i| i.args.as_ref())
                        .flat_map(|a| &a.0)
                        .for_each(|item| {
                            item.clone()
                                .parse(&mut Context::default())
                                .unwrap_or_else(|err| {
                                    panic!(
                                        "found an invalid argument: {}, (url: {}, function: {}, arg: {:?})",
                                        err,
                                        $url.to_string(),
                                        f.name,
                                        item
                                    );
                                });
                        })
                });
            };
        }

        EXTENSIONS.iter().for_each(|(url, ext)| {
            parse_arguments!(url, ext.scalar_functions);
            parse_arguments!(url, ext.aggregate_functions);
            parse_arguments!(url, ext.window_functions);
        });
    }
}
