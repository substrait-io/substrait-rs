// SPDX-License-Identifier: Apache-2.0

//! Evaluation of Substrait type-derivation expressions.
//!
//! This parses a derivation expression (the string form used in extension
//! function `return` clauses, e.g. `decimal<P1 + 1, S1 + 1>`) with the
//! ANTLR-generated [`super::substraittypeparser`] and evaluates it against a
//! set of bound parameter values into a [`ConcreteType`].
//!
//! Scalars, parameterized types (`decimal`, `varchar`, precision types, …),
//! `list` and `map` are supported, along with arithmetic, `min`/`max`,
//! ternaries and multiline definitions. `struct`/`nStruct` and
//! user-defined/function types are intentionally left as
//! [`DerivationError::NotYetImplemented`]: named structs in particular are a
//! schema concept, not a function return type, so they are out of scope here.

// TODO(#449): remove once the registry's return-type resolution calls
// `evaluate`. Until then the evaluator is exercised only by its unit tests.
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use antlr4rust::InputStream;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::Token;
use antlr4rust::token_factory::{TokenAware, TokenFactory};
use antlr4rust::tree::ParseTree;
use thiserror::Error;

use super::super::types::{BasicBuiltinType, ConcreteType};
use super::substraittypelexer::SubstraitTypeLexer;
use super::substraittypeparser::*;

/// Token type for `Identifier` in the generated lexer
/// (`SubstraitTypeLexer::Identifier`). Used to filter terminal children in
/// multiline definitions; a mismatch after a grammar change would be caught by
/// the `multiline`/`decimal_example` unit tests below.
const IDENTIFIER_TOKEN_TYPE: i32 = 83;

/// A value produced while evaluating a derivation expression.
///
/// Sub-expressions are either integers (type parameters and arithmetic over
/// them), booleans (comparisons feeding ternaries), or fully resolved types.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// An integer (a numeric type parameter or arithmetic result).
    Integer(i64),
    /// A boolean (the result of a comparison).
    Boolean(bool),
    /// A resolved type.
    Type(ConcreteType),
}

/// Errors that can occur while evaluating a derivation expression.
#[derive(Debug, Error, PartialEq)]
pub enum DerivationError {
    /// The expression could not be parsed.
    #[error("failed to parse derivation expression: {0}")]
    Parse(String),
    /// A referenced parameter was not bound in the values map.
    #[error("unbound parameter: {0}")]
    UnboundParameter(String),
    /// An unknown function was called (only `min`/`max` are supported).
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    /// An unknown binary operator was encountered.
    #[error("unknown binary operator: {0}")]
    UnknownOperator(String),
    /// A value of the wrong kind was used (e.g. a type where an integer was
    /// expected).
    #[error("expected {expected}, found {found:?}")]
    UnexpectedValue {
        /// What was expected.
        expected: &'static str,
        /// What was found.
        found: Value,
    },
    /// A construct that is valid in the grammar but not supported here.
    #[error("not yet implemented: {0}")]
    NotYetImplemented(String),
    /// The parse tree had an unexpected shape (missing child, etc.).
    #[error("malformed derivation expression")]
    Malformed,
}

/// Evaluate a derivation expression against the given parameter bindings.
///
/// Integer parameters (e.g. `P`, `S`) bind to [`Value::Integer`]; type
/// variables (e.g. `any1`) bind to [`Value::Type`].
pub fn evaluate(
    expression: &str,
    values: &HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    let lexer = SubstraitTypeLexer::new(InputStream::new(expression));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = SubstraitTypeParser::new(token_source);

    let errors = Rc::new(RefCell::new(Vec::new()));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(CollectingErrorListener {
        messages: errors.clone(),
    }));

    let tree = parser
        .expr()
        .map_err(|e| DerivationError::Parse(e.to_string()))?;
    if let Some(first) = errors.borrow().first() {
        return Err(DerivationError::Parse(first.clone()));
    }

    // `multilineDefinition` introduces local bindings, so work on a copy.
    let mut values = values.clone();
    eval_expr(&tree, &mut values)
}

fn eval_expr(
    x: &ExprContextAll,
    values: &mut HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    match x {
        ExprContextAll::BinaryExprContext(c) => {
            let left = eval_expr(c.left.as_deref().ok_or(DerivationError::Malformed)?, values)?;
            let right = eval_expr(
                c.right.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            let op =
                c.op.as_ref()
                    .ok_or(DerivationError::Malformed)?
                    .get_text()
                    .to_string();
            apply_binary(&op, left, right)
        }
        ExprContextAll::ParenExpressionContext(c) => {
            let inner = c.expr();
            eval_expr(inner.as_deref().ok_or(DerivationError::Malformed)?, values)
        }
        ExprContextAll::FunctionCallContext(c) => {
            let func = c.Identifier().ok_or(DerivationError::Malformed)?.get_text();
            let args = c
                .expr_all()
                .iter()
                .map(|e| eval_expr(e, values))
                .collect::<Result<Vec<_>, _>>()?;
            apply_function(&func, args)
        }
        ExprContextAll::TernaryContext(c) => {
            let cond = eval_expr(
                c.ifExpr.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            let branch = if as_bool(cond)? {
                c.thenExpr.as_deref()
            } else {
                c.elseExpr.as_deref()
            };
            eval_expr(branch.ok_or(DerivationError::Malformed)?, values)
        }
        ExprContextAll::MultilineDefinitionContext(c) => {
            // `Identifier_all()` returns *every* terminal child (antlr4rust does
            // not distinguish token types at the Rust-type level), so we filter
            // to the assignment left-hand sides. The right-hand sides come back
            // correctly from `expr_all()` (those are distinct context types).
            let names = c
                .Identifier_all()
                .into_iter()
                .filter(|t| t.symbol.get_token_type() == IDENTIFIER_TOKEN_TYPE)
                .map(|t| t.get_text());
            for (name, e) in names.zip(c.expr_all().iter()) {
                let value = eval_expr(e, values)?;
                values.insert(name, value);
            }
            let final_type = c.finalType.as_deref().ok_or(DerivationError::Malformed)?;
            eval_type_def(final_type, values)
        }
        ExprContextAll::ParameterNameContext(c) => {
            let name = c.Identifier().ok_or(DerivationError::Malformed)?.get_text();
            lookup(values, &name)
        }
        ExprContextAll::LiteralNumberContext(c) => {
            let text = c.Number().ok_or(DerivationError::Malformed)?.get_text();
            Ok(Value::Integer(parse_int(&text)?))
        }
        ExprContextAll::TypeLiteralContext(c) => {
            let type_def = c.typeDef();
            eval_type_def(
                type_def.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )
        }
        ExprContextAll::IfExprContext(_) | ExprContextAll::NotExprContext(_) => Err(
            DerivationError::NotYetImplemented("standalone if/not expression".to_string()),
        ),
        ExprContextAll::Error(_) => Err(DerivationError::Malformed),
    }
}

fn eval_type_def(
    x: &TypeDefContext,
    values: &mut HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    if let Some(scalar) = x.scalarType() {
        let nullable = x.isnull.is_some();
        eval_scalar(&scalar, nullable)
    } else if let Some(parameterized) = x.parameterizedType() {
        eval_parameterized(&parameterized, values)
    } else if let Some(any) = x.anyType() {
        let name = any.AnyVar().ok_or(DerivationError::Malformed)?.get_text();
        lookup(values, &name)
    } else {
        Err(DerivationError::Malformed)
    }
}

fn eval_scalar(x: &ScalarTypeContextAll, nullable: bool) -> Result<Value, DerivationError> {
    let builtin = match x {
        ScalarTypeContextAll::BooleanContext(_) => BasicBuiltinType::Boolean,
        ScalarTypeContextAll::I8Context(_) => BasicBuiltinType::I8,
        ScalarTypeContextAll::I16Context(_) => BasicBuiltinType::I16,
        ScalarTypeContextAll::I32Context(_) => BasicBuiltinType::I32,
        ScalarTypeContextAll::I64Context(_) => BasicBuiltinType::I64,
        ScalarTypeContextAll::Fp32Context(_) => BasicBuiltinType::Fp32,
        ScalarTypeContextAll::Fp64Context(_) => BasicBuiltinType::Fp64,
        ScalarTypeContextAll::StringContext(_) => BasicBuiltinType::String,
        ScalarTypeContextAll::BinaryContext(_) => BasicBuiltinType::Binary,
        ScalarTypeContextAll::TimestampContext(_) => BasicBuiltinType::Timestamp,
        ScalarTypeContextAll::TimestampTzContext(_) => BasicBuiltinType::TimestampTz,
        ScalarTypeContextAll::DateContext(_) => BasicBuiltinType::Date,
        ScalarTypeContextAll::TimeContext(_) => BasicBuiltinType::Time,
        ScalarTypeContextAll::IntervalYearContext(_) => BasicBuiltinType::IntervalYear,
        ScalarTypeContextAll::UuidContext(_) => BasicBuiltinType::Uuid,
        ScalarTypeContextAll::Error(_) => return Err(DerivationError::Malformed),
    };
    Ok(Value::Type(ConcreteType::builtin(builtin, nullable)))
}

fn eval_parameterized(
    x: &ParameterizedTypeContextAll,
    values: &mut HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    // Each arm yields the builtin together with its own nullability flag; the
    // `isnull` token lives on every parameterized-type alternative.
    let (builtin, nullable) = match x {
        ParameterizedTypeContextAll::DecimalContext(c) => {
            let precision = eval_numeric(
                c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            let scale = eval_numeric(
                c.scale.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            return Ok(Value::Type(ConcreteType::builtin(
                BasicBuiltinType::Decimal { precision, scale },
                c.isnull.is_some(),
            )));
        }
        ParameterizedTypeContextAll::VarCharContext(c) => (
            BasicBuiltinType::VarChar {
                length: eval_numeric(
                    c.length.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::FixedCharContext(c) => (
            BasicBuiltinType::FixedChar {
                length: eval_numeric(
                    c.length.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::FixedBinaryContext(c) => (
            BasicBuiltinType::FixedBinary {
                length: eval_numeric(
                    c.length.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimeContext(c) => (
            BasicBuiltinType::PrecisionTime {
                precision: eval_numeric(
                    c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimestampContext(c) => (
            BasicBuiltinType::PrecisionTimestamp {
                precision: eval_numeric(
                    c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimestampTZContext(c) => (
            BasicBuiltinType::PrecisionTimestampTz {
                precision: eval_numeric(
                    c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionIntervalDayContext(c) => (
            BasicBuiltinType::IntervalDay {
                precision: eval_numeric(
                    c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(c) => (
            BasicBuiltinType::IntervalCompound {
                precision: eval_numeric(
                    c.precision.as_deref().ok_or(DerivationError::Malformed)?,
                    values,
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::ListContext(c) => {
            let element = c.expr();
            let element = eval_expr(
                element.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            return Ok(Value::Type(ConcreteType::list(
                as_type(element)?,
                c.isnull.is_some(),
            )));
        }
        ParameterizedTypeContextAll::MapContext(c) => {
            let key = eval_expr(c.key.as_deref().ok_or(DerivationError::Malformed)?, values)?;
            let value = eval_expr(
                c.value.as_deref().ok_or(DerivationError::Malformed)?,
                values,
            )?;
            return Ok(Value::Type(ConcreteType::map(
                as_type(key)?,
                as_type(value)?,
                c.isnull.is_some(),
            )));
        }
        ParameterizedTypeContextAll::StructContext(_)
        | ParameterizedTypeContextAll::NStructContext(_) => {
            return Err(DerivationError::NotYetImplemented(
                "struct/nStruct return types".to_string(),
            ));
        }
        ParameterizedTypeContextAll::FuncContext(_)
        | ParameterizedTypeContextAll::UserDefinedContext(_) => {
            return Err(DerivationError::NotYetImplemented(
                "function/user-defined return types".to_string(),
            ));
        }
        ParameterizedTypeContextAll::Error(_) => return Err(DerivationError::Malformed),
    };
    Ok(Value::Type(ConcreteType::builtin(builtin, nullable)))
}

fn eval_numeric(
    x: &NumericParameterContextAll,
    values: &mut HashMap<String, Value>,
) -> Result<i32, DerivationError> {
    let value = match x {
        NumericParameterContextAll::NumericLiteralContext(c) => Value::Integer(parse_int(
            &c.Number().ok_or(DerivationError::Malformed)?.get_text(),
        )?),
        NumericParameterContextAll::NumericParameterNameContext(c) => lookup(
            values,
            &c.Identifier().ok_or(DerivationError::Malformed)?.get_text(),
        )?,
        NumericParameterContextAll::NumericExpressionContext(c) => {
            let inner = c.expr();
            eval_expr(inner.as_deref().ok_or(DerivationError::Malformed)?, values)?
        }
        NumericParameterContextAll::Error(_) => return Err(DerivationError::Malformed),
    };
    let int = as_int(value)?;
    i32::try_from(int).map_err(|_| {
        DerivationError::NotYetImplemented(format!("numeric parameter {int} does not fit in i32"))
    })
}

fn apply_binary(op: &str, left: Value, right: Value) -> Result<Value, DerivationError> {
    let (l, r) = (as_int(left)?, as_int(right)?);
    Ok(match op {
        "+" => Value::Integer(l + r),
        "-" => Value::Integer(l - r),
        "*" => Value::Integer(l * r),
        ">" => Value::Boolean(l > r),
        ">=" => Value::Boolean(l >= r),
        "<" => Value::Boolean(l < r),
        "<=" => Value::Boolean(l <= r),
        other => return Err(DerivationError::UnknownOperator(other.to_string())),
    })
}

fn apply_function(func: &str, args: Vec<Value>) -> Result<Value, DerivationError> {
    let ints = args
        .into_iter()
        .map(as_int)
        .collect::<Result<Vec<_>, _>>()?;
    let result = match func {
        "min" => ints.into_iter().min(),
        "max" => ints.into_iter().max(),
        other => return Err(DerivationError::UnknownFunction(other.to_string())),
    };
    result
        .map(Value::Integer)
        .ok_or_else(|| DerivationError::Malformed)
}

fn lookup(values: &HashMap<String, Value>, name: &str) -> Result<Value, DerivationError> {
    values
        .get(name)
        .cloned()
        .ok_or_else(|| DerivationError::UnboundParameter(name.to_string()))
}

fn as_int(value: Value) -> Result<i64, DerivationError> {
    match value {
        Value::Integer(n) => Ok(n),
        other => Err(DerivationError::UnexpectedValue {
            expected: "integer",
            found: other,
        }),
    }
}

fn as_bool(value: Value) -> Result<bool, DerivationError> {
    match value {
        Value::Boolean(b) => Ok(b),
        Value::Integer(n) => Ok(n != 0),
        other => Err(DerivationError::UnexpectedValue {
            expected: "boolean",
            found: other,
        }),
    }
}

fn as_type(value: Value) -> Result<ConcreteType, DerivationError> {
    match value {
        Value::Type(t) => Ok(t),
        other => Err(DerivationError::UnexpectedValue {
            expected: "type",
            found: other,
        }),
    }
}

fn parse_int(text: &str) -> Result<i64, DerivationError> {
    text.trim()
        .parse()
        .map_err(|_| DerivationError::Parse(format!("invalid integer literal `{text}`")))
}

/// Collects ANTLR syntax errors so they can be surfaced as [`DerivationError`].
#[derive(Clone)]
struct CollectingErrorListener {
    messages: Rc<RefCell<Vec<String>>>,
}

impl<'a, T: Recognizer<'a>> ErrorListener<'a, T> for CollectingErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<&<<T as TokenAware<'a>>::TF as TokenFactory<'a>>::Inner>,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.messages
            .borrow_mut()
            .push(format!("at {line}:{column}: {msg}"));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::text::simple_extensions::types::ConcreteTypeKind;

    fn eval(expr: &str) -> Value {
        evaluate(expr, &HashMap::new()).expect("evaluation succeeds")
    }

    fn eval_with(expr: &str, values: &[(&str, Value)]) -> Value {
        let map = values
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        evaluate(expr, &map).expect("evaluation succeeds")
    }

    fn int(n: i64) -> Value {
        Value::Integer(n)
    }

    fn builtin(b: BasicBuiltinType, nullable: bool) -> Value {
        Value::Type(ConcreteType::builtin(b, nullable))
    }

    #[test]
    fn simple_arithmetic() {
        assert_eq!(eval("1 + 1"), int(2));
    }

    #[test]
    fn arithmetic_with_variables() {
        assert_eq!(eval_with("1 + var", &[("var", int(2))]), int(3));
    }

    #[test]
    fn arithmetic_parenthesis() {
        assert_eq!(eval_with("(1 + var) * 3", &[("var", int(2))]), int(9));
    }

    #[test]
    fn min_max() {
        assert_eq!(
            eval_with("min(var, 7) + max(var, 7)", &[("var", int(5))]),
            int(12)
        );
    }

    #[test]
    fn ternary() {
        assert_eq!(eval_with("var > 3 ? 1 : 0", &[("var", int(5))]), int(1));
        assert_eq!(eval_with("var > 3 ? 1 : 0", &[("var", int(2))]), int(0));
    }

    #[test]
    fn simple_data_types() {
        assert_eq!(eval("i8"), builtin(BasicBuiltinType::I8, false));
        assert_eq!(eval("i16"), builtin(BasicBuiltinType::I16, false));
        assert_eq!(eval("i32"), builtin(BasicBuiltinType::I32, false));
        assert_eq!(eval("i64"), builtin(BasicBuiltinType::I64, false));
        assert_eq!(eval("fp32"), builtin(BasicBuiltinType::Fp32, false));
        assert_eq!(eval("fp64"), builtin(BasicBuiltinType::Fp64, false));
        assert_eq!(eval("boolean"), builtin(BasicBuiltinType::Boolean, false));
        assert_eq!(eval("i8?"), builtin(BasicBuiltinType::I8, true));
        assert_eq!(eval("boolean?"), builtin(BasicBuiltinType::Boolean, true));
    }

    #[test]
    fn decimal_with_parameters() {
        assert_eq!(
            eval_with("decimal<P + 1, S + 1>", &[("P", int(20)), ("S", int(10))]),
            builtin(
                BasicBuiltinType::Decimal {
                    precision: 21,
                    scale: 11
                },
                false
            )
        );
    }

    #[test]
    fn decimal_nullable() {
        assert_eq!(
            eval_with("decimal?<P + 1, S + 1>", &[("P", int(20)), ("S", int(10))]),
            builtin(
                BasicBuiltinType::Decimal {
                    precision: 21,
                    scale: 11
                },
                true
            )
        );
    }

    #[test]
    fn parameterized_nullability_is_preserved() {
        // Regression: the fall-through arms (varchar, precision types, ...) must
        // carry their own `?` nullability, not silently drop it.
        assert_eq!(
            eval("varchar?<10>"),
            builtin(BasicBuiltinType::VarChar { length: 10 }, true)
        );
        assert_eq!(
            eval("varchar<10>"),
            builtin(BasicBuiltinType::VarChar { length: 10 }, false)
        );
        assert_eq!(
            eval_with("precision_timestamp?<P>", &[("P", int(6))]),
            builtin(BasicBuiltinType::PrecisionTimestamp { precision: 6 }, true)
        );
    }

    #[test]
    fn multiline() {
        let result = eval_with(
            "temp = min(var, 7) + max(var, 7)\ndecimal<temp + 1, temp - 1>",
            &[("var", int(5))],
        );
        assert_eq!(
            result,
            builtin(
                BasicBuiltinType::Decimal {
                    precision: 13,
                    scale: 11
                },
                false
            )
        );
    }

    #[test]
    fn decimal_example() {
        // The canonical decimal multiplication output-type derivation.
        let expr = "init_scale = max(S1,S2)\n\
init_prec = init_scale + max(P1 - S1, P2 - S2) + 1\n\
min_scale = min(init_scale, 6)\n\
delta = init_prec - 38\n\
prec = min(init_prec, 38)\n\
scale_after_borrow = max(init_scale - delta, min_scale)\n\
scale = init_prec > 38 ? scale_after_borrow : init_scale\n\
DECIMAL<prec, scale>";
        let result = eval_with(
            expr,
            &[
                ("P1", int(10)),
                ("S1", int(8)),
                ("P2", int(14)),
                ("S2", int(2)),
            ],
        );
        // init_scale=8, init_prec=8+max(2,12)+1=21, prec=min(21,38)=21, scale=8.
        assert_eq!(
            result,
            builtin(
                BasicBuiltinType::Decimal {
                    precision: 21,
                    scale: 8
                },
                false
            )
        );
    }

    #[test]
    fn list_type() {
        let result = eval("list<i32>");
        assert_eq!(
            result,
            Value::Type(ConcreteType::list(
                ConcreteType::builtin(BasicBuiltinType::I32, false),
                false
            ))
        );
    }

    #[test]
    fn any_variable_binds_type() {
        let bound = ConcreteType::builtin(BasicBuiltinType::I64, false);
        assert_eq!(
            eval_with("any1", &[("any1", Value::Type(bound.clone()))]),
            Value::Type(bound)
        );
    }

    #[test]
    fn unbound_parameter_errors() {
        let err = evaluate("P + 1", &HashMap::new()).unwrap_err();
        assert_eq!(err, DerivationError::UnboundParameter("P".to_string()));
    }

    #[test]
    fn struct_not_yet_implemented() {
        let err = evaluate("struct<i32, i32>", &HashMap::new()).unwrap_err();
        assert!(matches!(err, DerivationError::NotYetImplemented(_)));
        // Sanity: ConcreteTypeKind is in scope (named-struct path is deliberately absent).
        let _ = ConcreteTypeKind::Struct(vec![]);
    }
}
