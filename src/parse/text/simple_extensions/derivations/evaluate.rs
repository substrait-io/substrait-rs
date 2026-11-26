// SPDX-License-Identifier: Apache-2.0

//! Evaluation of Substrait type-derivation expressions.
//!
//! A derivation (the string form used in extension function `return` clauses,
//! e.g. `decimal<P1 + 1, S1 + 1>`) is parsed into an [`Expr`] by
//! [`parse_derivation`], then evaluated against a set of bound parameter
//! values into a [`Type`].
//!
//! Supported:
//! - scalar types, parameterized types (`decimal`, `varchar`, precision
//!   types, …), `list` and `map`, with `?` nullability (also on bound type
//!   variables, e.g. `any1?`);
//! - integer arithmetic (`+ - * /`, checked for overflow), comparisons
//!   (`< <= > >=`), equality (`= !=`), boolean `AND`/`OR`/`!`, ternaries
//!   (`c ? a : b`), `IF c THEN a ELSE b`, and `min`/`max`;
//! - multiline definitions (`name = expr` lines followed by a final type),
//!   whose bindings are local to that definition.
//!
//! Like the grammar, names are case-insensitive: `P1`, `p1` and `ANY1` refer
//! to the bindings `P1`, `p1` and `any1` alike.
//!
//! Resulting types go through the same conversion as types parsed from
//! extension YAML ([`Type::try_from`]), so built-in names, parameter counts
//! and bounds (e.g. decimal precision `1..=38`) are checked the same way.
//!
//! `struct`, user-defined and function types, and the `integer_parameter(...)`
//! function are reported as [`DerivationError::NotYetImplemented`].

// TODO(#449): remove once the registry's return-type resolution calls
// `evaluate`. Until then the evaluator is exercised only by its unit tests.
#![allow(dead_code)]

use std::collections::HashMap;

use thiserror::Error;

use super::super::type_ast::{BinaryOp, Expr, TypeExpr, TypeParseError};
use super::super::type_parser::parse_derivation;
use super::super::types::{ExtensionTypeError, Type};

/// Bindings in scope, keyed by lower-cased name.
type Scope = HashMap<String, Value>;

/// A value produced while evaluating a derivation expression.
///
/// Sub-expressions are either integers (type parameters and arithmetic over
/// them), booleans (comparisons feeding conditionals), or fully resolved types.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// An integer (a numeric type parameter or arithmetic result).
    Integer(i64),
    /// A boolean (the result of a comparison).
    Boolean(bool),
    /// A resolved type.
    Type(Type),
}

/// Errors that can occur while evaluating a derivation expression.
#[derive(Debug, Error, PartialEq)]
pub enum DerivationError {
    /// The expression could not be parsed, or is too complex to parse safely.
    #[error("failed to parse derivation expression: {0}")]
    Parse(#[from] TypeParseError),
    /// A referenced parameter was not bound in the values map.
    #[error("unbound parameter: {0}")]
    UnboundParameter(String),
    /// An unknown function was called.
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    /// A known function was called with unsuitable arguments.
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
    /// Integer arithmetic overflowed `i64`.
    #[error("integer overflow in `{0}`")]
    Overflow(String),
    /// Integer division by zero.
    #[error("division by zero in `{0}`")]
    DivisionByZero(String),
    /// A value of the wrong kind was used (e.g. a type where an integer was
    /// expected).
    #[error("expected {expected}, found {found:?}")]
    UnexpectedValue {
        /// What was expected.
        expected: &'static str,
        /// What was found.
        found: Value,
    },
    /// The resulting type is invalid (e.g. a decimal precision above 38).
    #[error(transparent)]
    InvalidType(#[from] ExtensionTypeError),
    /// A construct that is valid in the grammar but not supported here.
    #[error("not yet implemented: {0}")]
    NotYetImplemented(String),
}

/// Parse and evaluate a derivation expression against the given parameter
/// bindings. See [`evaluate_expr`].
pub fn evaluate(
    expression: &str,
    values: &HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    evaluate_expr(&parse_derivation(expression)?, values)
}

/// Evaluate a parsed derivation expression against the given parameter
/// bindings.
///
/// Integer parameters (e.g. `P`, `S`) bind to [`Value::Integer`]; type
/// variables (e.g. `any1`) bind to [`Value::Type`]. Names are matched
/// case-insensitively.
pub fn evaluate_expr(
    expression: &Expr<'_>,
    values: &HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    let scope = values
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), value.clone()))
        .collect();
    eval(expression, &scope)
}

fn eval(expr: &Expr<'_>, scope: &Scope) -> Result<Value, DerivationError> {
    match expr {
        Expr::Type(type_expr) => eval_type(type_expr, scope),
        Expr::Integer(value) => Ok(Value::Integer(*value)),
        Expr::Binary(op, left, right) => eval_binary(*op, left, right, scope),
        Expr::Not(operand) => Ok(Value::Boolean(!as_bool(eval(operand, scope)?)?)),
        // Only the chosen branch is evaluated.
        Expr::If(condition, then, otherwise) => {
            if as_bool(eval(condition, scope)?)? {
                eval(then, scope)
            } else {
                eval(otherwise, scope)
            }
        }
        Expr::Call(name, arguments) => {
            // Resolve the function before evaluating its arguments, so that
            // e.g. `integer_parameter(precision)` is reported as unsupported
            // rather than as an unbound `precision`.
            let pick = resolve_function(name)?;
            let arguments = arguments
                .iter()
                .map(|argument| eval(argument, scope).and_then(as_int))
                .collect::<Result<Vec<_>, _>>()?;
            pick(arguments.into_iter())
                .map(Value::Integer)
                .ok_or_else(|| {
                    DerivationError::InvalidArguments(format!(
                        "{}() needs an argument",
                        name.to_ascii_lowercase()
                    ))
                })
        }
        Expr::Program(assignments, final_type) => {
            // The assignments are local to this definition.
            let mut scope = scope.clone();
            for (name, value) in assignments {
                let value = eval(value, &scope)?;
                scope.insert(name.to_ascii_lowercase(), value);
            }
            eval_type(final_type, &scope)
        }
    }
}

fn eval_binary(
    op: BinaryOp,
    left: &Expr<'_>,
    right: &Expr<'_>,
    scope: &Scope,
) -> Result<Value, DerivationError> {
    match op {
        // `AND`/`OR` short-circuit, so e.g. `S > 0 AND P / S > 2` is safe.
        BinaryOp::And => Ok(Value::Boolean(
            as_bool(eval(left, scope)?)? && as_bool(eval(right, scope)?)?,
        )),
        BinaryOp::Or => Ok(Value::Boolean(
            as_bool(eval(left, scope)?)? || as_bool(eval(right, scope)?)?,
        )),
        BinaryOp::Equal | BinaryOp::NotEqual => {
            let left = eval(left, scope)?;
            let right = eval(right, scope)?;
            let equal = match (&left, &right) {
                (Value::Integer(_), Value::Integer(_))
                | (Value::Boolean(_), Value::Boolean(_))
                | (Value::Type(_), Value::Type(_)) => left == right,
                _ => {
                    return Err(DerivationError::UnexpectedValue {
                        expected: "operands of the same kind",
                        found: right,
                    });
                }
            };
            Ok(Value::Boolean(equal == (op == BinaryOp::Equal)))
        }
        BinaryOp::Add
        | BinaryOp::Subtract
        | BinaryOp::Multiply
        | BinaryOp::Divide
        | BinaryOp::Less
        | BinaryOp::LessOrEqual
        | BinaryOp::Greater
        | BinaryOp::GreaterOrEqual => {
            let l = as_int(eval(left, scope)?)?;
            let r = as_int(eval(right, scope)?)?;
            let overflow = || DerivationError::Overflow(format!("{l} {op} {r}"));
            Ok(match op {
                BinaryOp::Add => Value::Integer(l.checked_add(r).ok_or_else(overflow)?),
                BinaryOp::Subtract => Value::Integer(l.checked_sub(r).ok_or_else(overflow)?),
                BinaryOp::Multiply => Value::Integer(l.checked_mul(r).ok_or_else(overflow)?),
                BinaryOp::Divide if r == 0 => {
                    return Err(DerivationError::DivisionByZero(format!("{l} / {r}")));
                }
                // Truncating division; `i64::MIN / -1` overflows.
                BinaryOp::Divide => Value::Integer(l.checked_div(r).ok_or_else(overflow)?),
                BinaryOp::Less => Value::Boolean(l < r),
                BinaryOp::LessOrEqual => Value::Boolean(l <= r),
                BinaryOp::Greater => Value::Boolean(l > r),
                BinaryOp::GreaterOrEqual => Value::Boolean(l >= r),
                _ => unreachable!("only arithmetic and comparison operators reach here"),
            })
        }
    }
}

/// Evaluate a type, or a bare name, which refers to a binding when one is in
/// scope.
fn eval_type(type_expr: &TypeExpr<'_>, scope: &Scope) -> Result<Value, DerivationError> {
    let (name, params, nullable) = match type_expr {
        TypeExpr::TypeVariable(id, nullable) => {
            return with_nullability(lookup(scope, &format!("any{id}"))?, *nullable);
        }
        TypeExpr::Simple(name, params, nullable) => (*name, params, *nullable),
        TypeExpr::UserDefined(..) | TypeExpr::Func(..) => {
            return Err(DerivationError::NotYetImplemented(
                "function/user-defined return types".to_string(),
            ));
        }
    };

    let lower = name.to_ascii_lowercase();
    if params.is_empty() {
        // Parameter names parse as types without parameters. Grammar
        // keywords such as `i32` cannot be assigned, so a binding always
        // means a parameter.
        if let Some(value) = scope.get(&lower) {
            return with_nullability(value.clone(), nullable);
        }
        // A bare `any` is not a binding: it does not determine a type.
        if lower == "any" {
            return Err(DerivationError::UnboundParameter(name.to_string()));
        }
    }

    // Types whose parameters are types are built directly, since evaluated
    // types cannot be turned back into a `TypeExpr`. The grammar fixes their
    // parameter counts.
    match lower.as_str() {
        "list" => {
            let [element] = eval_types::<1>(params, scope)?;
            return Ok(Value::Type(Type::list(element, nullable)));
        }
        "map" => {
            let [key, value] = eval_types::<2>(params, scope)?;
            return Ok(Value::Type(Type::map(key, value, nullable)));
        }
        "struct" => {
            return Err(DerivationError::NotYetImplemented(
                "struct return types".to_string(),
            ));
        }
        _ => {}
    }

    // The other types take integer parameters: evaluate them to literals and
    // convert as for a type written in YAML, which checks names and bounds.
    let literals = params
        .iter()
        .map(|param| eval(param, scope).and_then(as_int).map(Expr::Integer))
        .collect::<Result<Vec<_>, _>>()?;
    match Type::try_from(TypeExpr::Simple(name, literals, nullable)) {
        // An unknown bare name is a parameter that was not bound.
        Err(ExtensionTypeError::UnknownTypeName { .. }) if params.is_empty() => {
            Err(DerivationError::UnboundParameter(name.to_string()))
        }
        result => Ok(Value::Type(result?)),
    }
}

/// Evaluate `N` type parameters.
fn eval_types<const N: usize>(
    params: &[Expr<'_>],
    scope: &Scope,
) -> Result<[Type; N], DerivationError> {
    let types = params
        .iter()
        .map(|param| eval(param, scope).and_then(as_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(types
        .try_into()
        .unwrap_or_else(|_| unreachable!("the grammar fixes the parameter count")))
}

/// The integer functions callable in derivations.
type IntFunction = fn(std::vec::IntoIter<i64>) -> Option<i64>;

fn resolve_function(func: &str) -> Result<IntFunction, DerivationError> {
    match func.to_ascii_lowercase().as_str() {
        "min" => Ok(Iterator::min),
        "max" => Ok(Iterator::max),
        // Refers to the value of a function argument, which is not a binding
        // the caller can provide yet.
        "integer_parameter" => Err(DerivationError::NotYetImplemented(
            "integer_parameter()".to_string(),
        )),
        _ => Err(DerivationError::UnknownFunction(func.to_string())),
    }
}

fn lookup(scope: &Scope, name: &str) -> Result<Value, DerivationError> {
    scope
        .get(&name.to_ascii_lowercase())
        .cloned()
        .ok_or_else(|| DerivationError::UnboundParameter(name.to_string()))
}

/// Apply a `?` suffix: `nullable` makes the (type) value nullable.
fn with_nullability(value: Value, nullable: bool) -> Result<Value, DerivationError> {
    if !nullable {
        return Ok(value);
    }
    let mut t = as_type(value)?;
    t.nullable = true;
    Ok(Value::Type(t))
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
        other => Err(DerivationError::UnexpectedValue {
            expected: "boolean",
            found: other,
        }),
    }
}

fn as_type(value: Value) -> Result<Type, DerivationError> {
    match value {
        Value::Type(t) => Ok(t),
        other => Err(DerivationError::UnexpectedValue {
            expected: "type",
            found: other,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::type_parser::{MAX_OPENINGS, MAX_TOKENS};
    use super::super::super::types::{IntegerParameter, TypeKind};
    use super::*;

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

    fn lit(n: i64) -> IntegerParameter {
        IntegerParameter::Literal(n)
    }

    fn builtin(kind: TypeKind, nullable: bool) -> Value {
        Value::Type(Type::new(kind, nullable))
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
    fn operator_precedence() {
        assert_eq!(eval("1 + 2 * 3"), int(7));
        assert_eq!(eval("2 * 3 - 1"), int(5));
        assert_eq!(eval_with("var + 1 > 3 ? 1 : 0", &[("var", int(3))]), int(1));
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
        assert_eq!(eval("i8"), builtin(TypeKind::I8, false));
        assert_eq!(eval("i16"), builtin(TypeKind::I16, false));
        assert_eq!(eval("i32"), builtin(TypeKind::I32, false));
        assert_eq!(eval("i64"), builtin(TypeKind::I64, false));
        assert_eq!(eval("fp32"), builtin(TypeKind::Fp32, false));
        assert_eq!(eval("fp64"), builtin(TypeKind::Fp64, false));
        assert_eq!(eval("boolean"), builtin(TypeKind::Boolean, false));
        assert_eq!(eval("i8?"), builtin(TypeKind::I8, true));
        assert_eq!(eval("boolean?"), builtin(TypeKind::Boolean, true));
    }

    #[test]
    fn decimal_with_parameters() {
        assert_eq!(
            eval_with("decimal<P + 1, S + 1>", &[("P", int(20)), ("S", int(10))]),
            builtin(
                TypeKind::Decimal {
                    precision: lit(21),
                    scale: lit(11)
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
                TypeKind::Decimal {
                    precision: lit(21),
                    scale: lit(11)
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
            builtin(TypeKind::VarChar { length: lit(10) }, true)
        );
        assert_eq!(
            eval("varchar<10>"),
            builtin(TypeKind::VarChar { length: lit(10) }, false)
        );
        assert_eq!(
            eval_with("precision_timestamp?<P>", &[("P", int(6))]),
            builtin(TypeKind::PrecisionTimestamp { precision: lit(6) }, true)
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
                TypeKind::Decimal {
                    precision: lit(13),
                    scale: lit(11)
                },
                false
            )
        );
    }

    #[test]
    fn decimal_example() {
        // The decimal add/subtract output-type derivation from
        // `functions_arithmetic_decimal.yaml`.
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
                TypeKind::Decimal {
                    precision: lit(21),
                    scale: lit(8)
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
            Value::Type(Type::list(Type::new(TypeKind::I32, false), false))
        );
    }

    #[test]
    fn any_variable_binds_type() {
        let bound = Type::new(TypeKind::I64, false);
        assert_eq!(
            eval_with("any1", &[("any1", Value::Type(bound.clone()))]),
            Value::Type(bound)
        );
    }

    #[test]
    fn trailing_input_errors() {
        for expr in ["i32 i64", "decimal<1, 2> i32", "1 + 1 2"] {
            let err = evaluate(expr, &HashMap::new()).unwrap_err();
            assert!(
                matches!(err, DerivationError::Parse(_)),
                "{expr:?}: {err:?}"
            );
        }
    }

    #[test]
    fn invalid_characters_error() {
        for expr in ["i32}", "{name}", "i32 $"] {
            let err = evaluate(expr, &HashMap::new()).unwrap_err();
            assert!(
                matches!(err, DerivationError::Parse(_)),
                "{expr:?}: {err:?}"
            );
        }
    }

    #[test]
    fn trailing_newline_is_accepted() {
        assert_eq!(eval("i32\n"), builtin(TypeKind::I32, false));
        assert_eq!(
            eval_with("p = P + 1\ndecimal<p, 0>\n", &[("P", int(9))]),
            builtin(
                TypeKind::Decimal {
                    precision: lit(10),
                    scale: lit(0)
                },
                false
            )
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
    }

    fn err(expr: &str, values: &[(&str, Value)]) -> DerivationError {
        let map = values
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect();
        evaluate(expr, &map).expect_err("evaluation fails")
    }

    fn i32_type(nullable: bool) -> Type {
        Type::new(TypeKind::I32, nullable)
    }

    #[test]
    fn left_associativity() {
        assert_eq!(eval("10 - 2 - 3"), int(5));
        assert_eq!(eval("8 / 2 / 2"), int(2));
    }

    #[test]
    fn division_truncates() {
        assert_eq!(eval("10 / 3"), int(3));
        assert_eq!(eval("-7 / 2"), int(-3));
    }

    #[test]
    fn overflow_is_an_error() {
        let max = ("P", int(i64::MAX));
        for (expr, values) in [
            ("9223372036854775807 + 1", vec![]),
            ("-9223372036854775808 - 1", vec![]),
            ("9223372036854775807 * 2", vec![]),
            ("-9223372036854775808 / -1", vec![]),
            ("P + 1", vec![max.clone()]),
            ("decimal<P * P, 0>", vec![max]),
        ] {
            assert!(
                matches!(err(expr, &values), DerivationError::Overflow(_)),
                "{expr}"
            );
        }
    }

    #[test]
    fn division_by_zero_is_an_error() {
        assert!(matches!(
            err("P / S", &[("P", int(1)), ("S", int(0))]),
            DerivationError::DivisionByZero(_)
        ));
    }

    #[test]
    fn equality() {
        assert_eq!(eval("1 = 1 ? 1 : 0"), int(1));
        assert_eq!(eval("1 != 1 ? 1 : 0"), int(0));
        let any1 = ("any1", Value::Type(i32_type(false)));
        assert_eq!(
            eval_with("any1 = i32 ? 1 : 0", std::slice::from_ref(&any1)),
            int(1)
        );
        assert_eq!(eval_with("any1 != i64 ? 1 : 0", &[any1]), int(1));
        assert!(matches!(
            err("1 = i32", &[]),
            DerivationError::UnexpectedValue { .. }
        ));
    }

    #[test]
    fn boolean_operators() {
        assert_eq!(eval("1 > 0 AND 2 > 0 ? 1 : 0"), int(1));
        assert_eq!(eval("1 > 0 and 2 < 0 ? 1 : 0"), int(0));
        assert_eq!(eval("1 < 0 OR 2 > 0 ? 1 : 0"), int(1));
        assert_eq!(eval("1 < 0 or 2 < 0 ? 1 : 0"), int(0));
        assert_eq!(eval("!(1 > 0) ? 1 : 0"), int(0));
        assert_eq!(eval("!(1 > 0 AND 1 < 0) ? 1 : 0"), int(1));
    }

    #[test]
    fn boolean_operators_short_circuit() {
        // The right-hand side would divide by zero.
        let values = [("P", int(5)), ("S", int(0))];
        assert_eq!(eval_with("S > 0 AND P / S > 2 ? 1 : 0", &values), int(0));
        assert_eq!(eval_with("S = 0 OR P / S > 2 ? 1 : 0", &values), int(1));
    }

    #[test]
    fn conditions_must_be_boolean() {
        assert!(matches!(
            err("1 ? 2 : 3", &[]),
            DerivationError::UnexpectedValue {
                expected: "boolean",
                ..
            }
        ));
        assert!(matches!(
            err("1 AND 1 ? 2 : 3", &[]),
            DerivationError::UnexpectedValue {
                expected: "boolean",
                ..
            }
        ));
    }

    #[test]
    fn if_then_else() {
        let values = [("P", int(11))];
        assert_eq!(eval_with("IF P > 10 THEN 1 ELSE 2", &values), int(1));
        assert_eq!(eval_with("if P > 20 then 1 else 2", &values), int(2));
    }

    #[test]
    fn type_valued_conditionals() {
        let values = [("P", int(11))];
        assert_eq!(
            eval_with("P > 10 ? i64 : i32", &values),
            builtin(TypeKind::I64, false)
        );
        assert_eq!(
            eval_with("IF P > 20 THEN i64 ELSE i32?", &values),
            builtin(TypeKind::I32, true)
        );
    }

    #[test]
    fn map_type() {
        assert_eq!(
            eval_with("map?<i32, varchar<L>>", &[("L", int(5))]),
            Value::Type(Type::map(
                i32_type(false),
                Type::new(TypeKind::VarChar { length: lit(5) }, false),
                true
            ))
        );
    }

    #[test]
    fn nullable_type_variable() {
        let any1 = ("any1", Value::Type(i32_type(false)));
        assert_eq!(
            eval_with("any1?", std::slice::from_ref(&any1)),
            Value::Type(i32_type(true))
        );
        assert_eq!(
            eval_with("list<any1?>", std::slice::from_ref(&any1)),
            Value::Type(Type::list(i32_type(true), false))
        );
        // `?` on an already nullable binding keeps it nullable; without `?` the
        // binding is returned unchanged.
        let nullable = ("any1", Value::Type(i32_type(true)));
        assert_eq!(
            eval_with("any1?", std::slice::from_ref(&nullable)),
            Value::Type(i32_type(true))
        );
        assert_eq!(eval_with("any1", &[nullable]), Value::Type(i32_type(true)));
        // Named type parameters take `?` too, but integers cannot be nullable.
        assert_eq!(
            eval_with("T?", &[("T", Value::Type(i32_type(false)))]),
            Value::Type(i32_type(true))
        );
        assert!(matches!(
            err("P?", &[("P", int(5))]),
            DerivationError::UnexpectedValue {
                expected: "type",
                ..
            }
        ));
    }

    #[test]
    fn bare_any_is_unbound() {
        assert_eq!(
            err("list?<any>", &[]),
            DerivationError::UnboundParameter("any".to_string())
        );
    }

    #[test]
    fn names_are_case_insensitive() {
        assert_eq!(
            eval("A = 1\ndecimal<a, 0>"),
            builtin(
                TypeKind::Decimal {
                    precision: lit(1),
                    scale: lit(0)
                },
                false
            )
        );
        assert_eq!(eval_with("p + 1", &[("P", int(5))]), int(6));
        assert_eq!(
            eval_with("ANY1", &[("any1", Value::Type(i32_type(false)))]),
            Value::Type(i32_type(false))
        );
        assert_eq!(eval("MIN(1, 2) + Max(1, 2)"), int(3));
    }

    #[test]
    fn multiline_bindings_are_local() {
        // `x` is bound inside the map key only; the value cannot see it.
        assert_eq!(
            err("map<(x = 1\ni32), varchar<x>>", &[]),
            DerivationError::UnboundParameter("x".to_string())
        );
        // Bindings may shadow parameters without affecting the caller's map.
        assert_eq!(eval_with("P = P + 1\ndecimal<P, 0>", &[("P", int(9))]), {
            builtin(
                TypeKind::Decimal {
                    precision: lit(10),
                    scale: lit(0),
                },
                false,
            )
        });
    }

    #[test]
    fn functions() {
        assert!(matches!(
            err("min()", &[]),
            DerivationError::InvalidArguments(_)
        ));
        assert_eq!(
            err("avg(1, 2)", &[]),
            DerivationError::UnknownFunction("avg".to_string())
        );
        // Used by `functions_datetime.yaml`; refers to an argument's value.
        assert!(matches!(
            err(
                "precision = integer_parameter(precision)\nprecision_time<precision>",
                &[]
            ),
            DerivationError::NotYetImplemented(_)
        ));
    }

    #[test]
    fn parameter_bounds_are_checked() {
        use ExtensionTypeError::{InvalidParameterRange, InvalidParameterValue};
        for expr in [
            "decimal<0, 0>",
            "decimal<39, 0>",
            "decimal<10, 11>",
            "decimal<10, -1>",
            "precision_time<13>",
            "precision_timestamp<-1>",
            "precision_timestamp_tz<13>",
            "interval_day<10>",
        ] {
            assert!(
                matches!(
                    err(expr, &[]),
                    DerivationError::InvalidType(InvalidParameterRange { .. })
                ),
                "{expr}"
            );
        }
        assert!(matches!(
            err("varchar<3000000000>", &[]),
            DerivationError::InvalidType(InvalidParameterValue { .. })
        ));
        // Derived values are checked too, e.g. a precision past 38.
        assert!(matches!(
            err("decimal<P + 1, 0>", &[("P", int(38))]),
            DerivationError::InvalidType(InvalidParameterRange { .. })
        ));
        // The bounds are inclusive.
        assert_eq!(
            eval("decimal<38, 38>"),
            builtin(
                TypeKind::Decimal {
                    precision: lit(38),
                    scale: lit(38)
                },
                false
            )
        );
    }

    #[test]
    fn complexity_limits() {
        let sum = |terms: usize| vec!["1"; terms].join(" + ");
        // 100 terms is 199 tokens: fine.
        assert_eq!(eval(&sum(100)), int(100));
        assert!(matches!(
            err(&sum(MAX_TOKENS / 2 + 1), &[]),
            DerivationError::Parse(TypeParseError::TooComplex(_))
        ));
        let nested = |depth: usize| format!("{}i32{}", "list<".repeat(depth), ">".repeat(depth));
        assert!(matches!(
            err(&nested(MAX_OPENINGS + 1), &[]),
            DerivationError::Parse(TypeParseError::TooComplex(_))
        ));
        let parens = |depth: usize| format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
        assert!(matches!(
            err(&parens(MAX_OPENINGS + 1), &[]),
            DerivationError::Parse(TypeParseError::TooComplex(_))
        ));
        // The deepest nesting the limits allow fits in a 2 MiB thread stack,
        // even in debug builds (which is how this test usually runs).
        let deepest = [
            nested(MAX_OPENINGS),
            parens(MAX_OPENINGS),
            format!(
                "{}1{}",
                "max(1, ".repeat(MAX_OPENINGS),
                ")".repeat(MAX_OPENINGS)
            ),
        ];
        std::thread::Builder::new()
            .stack_size(2 << 20)
            .spawn(move || {
                for expr in deepest {
                    evaluate(&expr, &HashMap::new()).expect("within limits");
                }
            })
            .expect("spawn thread")
            .join()
            .expect("no stack overflow");
        // Whitespace and comments do not count.
        assert_eq!(eval(&format!("{}1", " ".repeat(5000))), int(1));
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert_eq!(eval("\n  i32 \n"), builtin(TypeKind::I32, false));
    }
}
