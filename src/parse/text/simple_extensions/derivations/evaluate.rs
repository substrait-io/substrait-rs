// SPDX-License-Identifier: Apache-2.0

//! Evaluation of Substrait type-derivation expressions.
//!
//! This parses a derivation expression (the string form used in extension
//! function `return` clauses, e.g. `decimal<P1 + 1, S1 + 1>`) with the
//! ANTLR-generated [`SubstraitTypeParser`] and evaluates it against a
//! set of bound parameter values into a [`Type`].
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
//! Resulting types are checked against the same parameter bounds as types
//! parsed from extension YAML (e.g. decimal precision `1..=38`).
//!
//! `struct`/`nStruct`, user-defined and function types, and the
//! `integer_parameter(...)` function are reported as
//! [`DerivationError::NotYetImplemented`]: named structs in particular are a
//! schema concept, not a function return type, so they are out of scope here.

// TODO(#449): remove once the registry's return-type resolution calls
// `evaluate`. Until then the evaluator is exercised only by its unit tests.
#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::rc::Rc;

use antlr4rust::InputStream;
use antlr4rust::TokenSource;
use antlr4rust::common_token_stream::CommonTokenStream;
use antlr4rust::error_listener::ErrorListener;
use antlr4rust::errors::ANTLRError;
use antlr4rust::parser::Parser;
use antlr4rust::recognizer::Recognizer;
use antlr4rust::token::{TOKEN_DEFAULT_CHANNEL, TOKEN_EOF, Token};
use antlr4rust::token_factory::{TokenAware, TokenFactory};
use antlr4rust::tree::ParseTree;
use thiserror::Error;

use super::super::types::{
    DECIMAL_PRECISION, ExtensionTypeError, INTERVAL_DAY_PRECISION, IntegerParameter,
    SUBSECOND_PRECISION, Type, TypeKind, check_integer_literal,
};
use substrait_antlr::substrait_type::substraittypelexer::{
    Identifier as IDENTIFIER_TOKEN_TYPE, Lt, OParen, SubstraitTypeLexer,
};
use substrait_antlr::substrait_type::substraittypeparser::*;

/// The token type produced by the generated parser (private there).
type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;

/// Bindings in scope, keyed by lower-cased name.
type Scope = HashMap<String, Value>;

/// The most (non-whitespace) tokens an expression may have. This bounds how
/// deep chains such as `1 + 1 + …` recurse during evaluation. The longest
/// derivations in the Substrait extensions have about 110 tokens.
pub const MAX_TOKENS: usize = 1024;

/// The most opening `(` and `<` tokens an expression may have, which bounds
/// its nesting depth. The generated parser recurses several frames per nesting
/// level. The longest derivations in the Substrait extensions (decimal
/// `add`, `subtract` and `modulus`) have 6 openings and nest 2 levels deep.
///
/// Unoptimized (debug) builds use much larger stack frames in the generated
/// parser and exhaust a 2 MiB thread stack at 10 levels of nesting, so the
/// limit is kept low enough to hold in debug builds too, not just optimized
/// ones.
pub const MAX_OPENINGS: usize = 8;

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
    /// The expression could not be parsed.
    #[error("failed to parse derivation expression: {0}")]
    Parse(String),
    /// The expression exceeds [`MAX_TOKENS`] or [`MAX_OPENINGS`].
    #[error("derivation expression too complex: {0}")]
    TooComplex(String),
    /// A referenced parameter was not bound in the values map.
    #[error("unbound parameter: {0}")]
    UnboundParameter(String),
    /// An unknown function was called.
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    /// A known function was called with unsuitable arguments.
    #[error("invalid arguments: {0}")]
    InvalidArguments(String),
    /// An unknown binary operator was encountered.
    #[error("unknown binary operator: {0}")]
    UnknownOperator(String),
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
    /// The parse tree had an unexpected shape (missing child, etc.).
    #[error("malformed derivation expression")]
    Malformed,
}

/// Evaluate a derivation expression against the given parameter bindings.
///
/// Integer parameters (e.g. `P`, `S`) bind to [`Value::Integer`]; type
/// variables (e.g. `any1`) bind to [`Value::Type`]. Names are matched
/// case-insensitively.
pub fn evaluate(
    expression: &str,
    values: &HashMap<String, Value>,
) -> Result<Value, DerivationError> {
    // YAML block scalars (`return: |`) end with a newline, which the grammar
    // only tolerates after multiline definitions; surrounding whitespace
    // carries no meaning, so drop it.
    let expression = expression.trim();
    check_complexity(expression)?;

    // Both the lexer and the parser report into the same list; ANTLR's default
    // listeners only print to stderr and let invalid input through.
    let errors = Rc::new(RefCell::new(Vec::new()));
    let listener = CollectingErrorListener {
        messages: errors.clone(),
    };
    let mut lexer = SubstraitTypeLexer::new(InputStream::new(expression));
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(listener.clone()));
    let mut parser = SubstraitTypeParser::new(CommonTokenStream::new(lexer));
    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(listener));

    // `startRule` is `expr EOF`, so trailing input is an error rather than
    // silently ignored (which `expr` alone would do).
    let tree = parser
        .startRule()
        .map_err(|e| DerivationError::Parse(e.to_string()))?;
    if let Some(first) = errors.borrow().first() {
        return Err(DerivationError::Parse(first.clone()));
    }
    let tree = tree.expr().ok_or(DerivationError::Malformed)?;

    let scope = values
        .iter()
        .map(|(name, value)| (name.to_ascii_lowercase(), value.clone()))
        .collect();
    eval_expr(&tree, &scope)
}

/// Reject expressions whose size or nesting could exhaust the stack in the
/// (recursive) parser or evaluator. Lexing is iterative, so this is safe to
/// run on any input. Lexer errors are ignored here; parsing reports them.
fn check_complexity(expression: &str) -> Result<(), DerivationError> {
    let mut lexer = SubstraitTypeLexer::new(InputStream::new(expression));
    lexer.remove_error_listeners();
    let (mut tokens, mut openings) = (0, 0);
    loop {
        let token = lexer.next_token();
        let token_type = token.get_token_type();
        if token_type == TOKEN_EOF {
            return Ok(());
        }
        if token.get_channel() != TOKEN_DEFAULT_CHANNEL {
            continue;
        }
        tokens += 1;
        // Counting openings (closers need no tracking) bounds the nesting
        // depth. `<` is also the less-than operator; counting it anyway only
        // makes the bound more conservative.
        if token_type == OParen || token_type == Lt {
            openings += 1;
        }
        if tokens > MAX_TOKENS {
            return Err(DerivationError::TooComplex(format!(
                "more than {MAX_TOKENS} tokens"
            )));
        }
        if openings > MAX_OPENINGS {
            return Err(DerivationError::TooComplex(format!(
                "more than {MAX_OPENINGS} opening brackets"
            )));
        }
    }
}

fn eval_expr(x: &ExprContextAll, scope: &Scope) -> Result<Value, DerivationError> {
    match x {
        ExprContextAll::MulDivContext(c) => eval_arithmetic(&c.left, &c.op, &c.right, scope),
        ExprContextAll::AddSubContext(c) => eval_arithmetic(&c.left, &c.op, &c.right, scope),
        ExprContextAll::ComparisonContext(c) => eval_arithmetic(&c.left, &c.op, &c.right, scope),
        ExprContextAll::EqualityContext(c) => {
            let left = eval_child(&c.left, scope)?;
            let right = eval_child(&c.right, scope)?;
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
            match op_text(&c.op)?.as_str() {
                "=" => Ok(Value::Boolean(equal)),
                "!=" => Ok(Value::Boolean(!equal)),
                other => Err(DerivationError::UnknownOperator(other.to_string())),
            }
        }
        // `AND`/`OR` short-circuit, so e.g. `S > 0 AND P / S > 2` is safe.
        ExprContextAll::AndContext(c) => {
            if !as_bool(eval_child(&c.left, scope)?)? {
                return Ok(Value::Boolean(false));
            }
            Ok(Value::Boolean(as_bool(eval_child(&c.right, scope)?)?))
        }
        ExprContextAll::OrContext(c) => {
            if as_bool(eval_child(&c.left, scope)?)? {
                return Ok(Value::Boolean(true));
            }
            Ok(Value::Boolean(as_bool(eval_child(&c.right, scope)?)?))
        }
        ExprContextAll::NotExprContext(c) => {
            Ok(Value::Boolean(!as_bool(eval_child(&c.expr(), scope)?)?))
        }
        ExprContextAll::TernaryContext(c) => {
            eval_conditional(&c.ifExpr, &c.thenExpr, &c.elseExpr, scope)
        }
        ExprContextAll::IfExprContext(c) => {
            eval_conditional(&c.ifExpr, &c.thenExpr, &c.elseExpr, scope)
        }
        ExprContextAll::ParenExpressionContext(c) => eval_child(&c.expr(), scope),
        ExprContextAll::FunctionCallContext(c) => {
            let func = c.Identifier().ok_or(DerivationError::Malformed)?.get_text();
            // Resolve the function before evaluating its arguments, so that
            // e.g. `integer_parameter(precision)` is reported as unsupported
            // rather than as an unbound `precision`.
            let pick = resolve_function(&func)?;
            let args = c
                .expr_all()
                .iter()
                .map(|e| eval_expr(e, scope).and_then(as_int))
                .collect::<Result<Vec<_>, _>>()?;
            pick(args.into_iter()).map(Value::Integer).ok_or_else(|| {
                DerivationError::InvalidArguments(format!(
                    "{}() needs an argument",
                    func.to_ascii_lowercase()
                ))
            })
        }
        ExprContextAll::MultilineDefinitionContext(c) => {
            // The assignments are local to this definition.
            let mut scope = scope.clone();
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
                let value = eval_expr(e, &scope)?;
                scope.insert(name.to_ascii_lowercase(), value);
            }
            let final_type = c.finalType.as_deref().ok_or(DerivationError::Malformed)?;
            eval_type_def(final_type, &scope)
        }
        ExprContextAll::ParameterNameContext(c) => {
            let name = c.Identifier().ok_or(DerivationError::Malformed)?.get_text();
            with_nullability(lookup(scope, &name)?, c.isnull.is_some())
        }
        ExprContextAll::LiteralNumberContext(c) => {
            let text = c.Number().ok_or(DerivationError::Malformed)?.get_text();
            Ok(Value::Integer(parse_int(&text)?))
        }
        ExprContextAll::TypeLiteralContext(c) => {
            let type_def = c.typeDef();
            eval_type_def(
                type_def.as_deref().ok_or(DerivationError::Malformed)?,
                scope,
            )
        }
        ExprContextAll::Error(_) => Err(DerivationError::Malformed),
    }
}

fn eval_child(child: &Option<Rc<ExprContextAll>>, scope: &Scope) -> Result<Value, DerivationError> {
    eval_expr(child.as_deref().ok_or(DerivationError::Malformed)?, scope)
}

fn op_text(op: &Option<TokenType>) -> Result<String, DerivationError> {
    Ok(op
        .as_ref()
        .ok_or(DerivationError::Malformed)?
        .get_text()
        .to_string())
}

/// `c ? a : b` and `IF c THEN a ELSE b`; only the chosen branch is evaluated.
fn eval_conditional(
    condition: &Option<Rc<ExprContextAll>>,
    then: &Option<Rc<ExprContextAll>>,
    otherwise: &Option<Rc<ExprContextAll>>,
    scope: &Scope,
) -> Result<Value, DerivationError> {
    if as_bool(eval_child(condition, scope)?)? {
        eval_child(then, scope)
    } else {
        eval_child(otherwise, scope)
    }
}

/// Integer operators: `* /`, `+ -` and `< <= > >=`.
fn eval_arithmetic(
    left: &Option<Rc<ExprContextAll>>,
    op: &Option<TokenType>,
    right: &Option<Rc<ExprContextAll>>,
    scope: &Scope,
) -> Result<Value, DerivationError> {
    let l = as_int(eval_child(left, scope)?)?;
    let r = as_int(eval_child(right, scope)?)?;
    let op = op_text(op)?;
    let overflow = || DerivationError::Overflow(format!("{l} {op} {r}"));
    Ok(match op.as_str() {
        "+" => Value::Integer(l.checked_add(r).ok_or_else(overflow)?),
        "-" => Value::Integer(l.checked_sub(r).ok_or_else(overflow)?),
        "*" => Value::Integer(l.checked_mul(r).ok_or_else(overflow)?),
        "/" if r == 0 => return Err(DerivationError::DivisionByZero(format!("{l} / {r}"))),
        // Truncating division; `i64::MIN / -1` overflows.
        "/" => Value::Integer(l.checked_div(r).ok_or_else(overflow)?),
        ">" => Value::Boolean(l > r),
        ">=" => Value::Boolean(l >= r),
        "<" => Value::Boolean(l < r),
        "<=" => Value::Boolean(l <= r),
        other => return Err(DerivationError::UnknownOperator(other.to_string())),
    })
}

fn eval_type_def(x: &TypeDefContext, scope: &Scope) -> Result<Value, DerivationError> {
    if let Some(scalar) = x.scalarType() {
        eval_scalar(&scalar, x.isnull.is_some())
    } else if let Some(parameterized) = x.parameterizedType() {
        eval_parameterized(&parameterized, scope)
    } else if let Some(any) = x.anyType() {
        // A bare `any` is not a binding: it does not determine a type.
        let name = any
            .AnyVar()
            .ok_or_else(|| DerivationError::UnboundParameter("any".to_string()))?
            .get_text();
        with_nullability(lookup(scope, &name)?, any.isnull.is_some())
    } else {
        Err(DerivationError::Malformed)
    }
}

fn eval_scalar(x: &ScalarTypeContextAll, nullable: bool) -> Result<Value, DerivationError> {
    let builtin = match x {
        ScalarTypeContextAll::BooleanContext(_) => TypeKind::Boolean,
        ScalarTypeContextAll::I8Context(_) => TypeKind::I8,
        ScalarTypeContextAll::I16Context(_) => TypeKind::I16,
        ScalarTypeContextAll::I32Context(_) => TypeKind::I32,
        ScalarTypeContextAll::I64Context(_) => TypeKind::I64,
        ScalarTypeContextAll::Fp32Context(_) => TypeKind::Fp32,
        ScalarTypeContextAll::Fp64Context(_) => TypeKind::Fp64,
        ScalarTypeContextAll::StringContext(_) => TypeKind::String,
        ScalarTypeContextAll::BinaryContext(_) => TypeKind::Binary,
        ScalarTypeContextAll::DateContext(_) => TypeKind::Date,
        ScalarTypeContextAll::IntervalYearContext(_) => TypeKind::IntervalYear,
        ScalarTypeContextAll::UuidContext(_) => TypeKind::Uuid,
        ScalarTypeContextAll::Error(_) => return Err(DerivationError::Malformed),
    };
    Ok(Value::Type(Type::new(builtin, nullable)))
}

fn eval_parameterized(
    x: &ParameterizedTypeContextAll,
    scope: &Scope,
) -> Result<Value, DerivationError> {
    // Evaluate integer parameter `index` of `type_name` and check its bounds.
    let int = |type_name: &str,
               index: usize,
               param: &Option<Rc<NumericParameterContextAll>>,
               range: Option<RangeInclusive<i32>>| {
        let value = eval_numeric(param.as_deref().ok_or(DerivationError::Malformed)?, scope)?;
        Ok::<_, DerivationError>(IntegerParameter::Literal(check_integer_literal(
            type_name, index, value, range,
        )?))
    };

    // Each arm yields the kind together with its own nullability flag; the
    // `isnull` token lives on every parameterized-type alternative.
    let (kind, nullable) = match x {
        ParameterizedTypeContextAll::DecimalContext(c) => {
            let precision = int("decimal", 0, &c.precision, Some(DECIMAL_PRECISION))?;
            let IntegerParameter::Literal(max_scale) = precision else {
                unreachable!("evaluated parameters are literals")
            };
            // `max_scale` passed the precision check, so it fits in an i32.
            let scale = int("decimal", 1, &c.scale, Some(0..=max_scale as i32))?;
            (TypeKind::Decimal { precision, scale }, c.isnull.is_some())
        }
        ParameterizedTypeContextAll::VarCharContext(c) => (
            TypeKind::VarChar {
                length: int("varchar", 0, &c.length, None)?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::FixedCharContext(c) => (
            TypeKind::FixedChar {
                length: int("fixedchar", 0, &c.length, None)?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::FixedBinaryContext(c) => (
            TypeKind::FixedBinary {
                length: int("fixedbinary", 0, &c.length, None)?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimeContext(c) => (
            TypeKind::PrecisionTime {
                precision: int("precision_time", 0, &c.precision, Some(SUBSECOND_PRECISION))?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimestampContext(c) => (
            TypeKind::PrecisionTimestamp {
                precision: int(
                    "precision_timestamp",
                    0,
                    &c.precision,
                    Some(SUBSECOND_PRECISION),
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionTimestampTZContext(c) => (
            TypeKind::PrecisionTimestampTz {
                precision: int(
                    "precision_timestamp_tz",
                    0,
                    &c.precision,
                    Some(SUBSECOND_PRECISION),
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionIntervalDayContext(c) => (
            TypeKind::IntervalDay {
                precision: int(
                    "interval_day",
                    0,
                    &c.precision,
                    Some(INTERVAL_DAY_PRECISION),
                )?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::PrecisionIntervalCompoundContext(c) => (
            TypeKind::IntervalCompound {
                precision: int("interval_compound", 0, &c.precision, None)?,
            },
            c.isnull.is_some(),
        ),
        ParameterizedTypeContextAll::ListContext(c) => {
            let element = as_type(eval_child(&c.expr(), scope)?)?;
            return Ok(Value::Type(Type::list(element, c.isnull.is_some())));
        }
        ParameterizedTypeContextAll::MapContext(c) => {
            let key = as_type(eval_child(&c.key, scope)?)?;
            let value = as_type(eval_child(&c.value, scope)?)?;
            return Ok(Value::Type(Type::map(key, value, c.isnull.is_some())));
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
    Ok(Value::Type(Type::new(kind, nullable)))
}

fn eval_numeric(x: &NumericParameterContextAll, scope: &Scope) -> Result<i64, DerivationError> {
    let value = match x {
        NumericParameterContextAll::NumericLiteralContext(c) => Value::Integer(parse_int(
            &c.Number().ok_or(DerivationError::Malformed)?.get_text(),
        )?),
        NumericParameterContextAll::NumericParameterNameContext(c) => lookup(
            scope,
            &c.Identifier().ok_or(DerivationError::Malformed)?.get_text(),
        )?,
        NumericParameterContextAll::NumericExpressionContext(c) => eval_child(&c.expr(), scope)?,
        NumericParameterContextAll::Error(_) => return Err(DerivationError::Malformed),
    };
    as_int(value)
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
            DerivationError::TooComplex(_)
        ));
        let nested = |depth: usize| format!("{}i32{}", "list<".repeat(depth), ">".repeat(depth));
        assert!(matches!(
            err(&nested(MAX_OPENINGS + 1), &[]),
            DerivationError::TooComplex(_)
        ));
        let parens = |depth: usize| format!("{}1{}", "(".repeat(depth), ")".repeat(depth));
        assert!(matches!(
            err(&parens(MAX_OPENINGS + 1), &[]),
            DerivationError::TooComplex(_)
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
