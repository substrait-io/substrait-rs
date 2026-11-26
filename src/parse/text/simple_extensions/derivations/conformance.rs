// SPDX-License-Identifier: Apache-2.0

//! Conformance tests for [`evaluate`] against the Substrait function test
//! cases bundled in `substrait-extensions`.
//!
//! Each test case (e.g. `add(1::dec<38, 0>, 2::dec<38, 0>) = 3::dec<38, 0>`)
//! pins the concrete return type of a function for concrete argument types.
//! For every case we find the matching implementation in the extension YAML the
//! test file includes, bind its signature parameters (`P1`, `S1`, `any1`, …) by
//! unifying it with the argument types, evaluate its `return` derivation, and
//! compare the result with the expected type.
//!
//! The expected nullability follows the function's nullability handling: with
//! `DECLARED_OUTPUT` (and `DISCRETE`) it is exactly the derived type's; with
//! `MIRROR` (the default) the result is also nullable when any argument is.
//! Cases with options (e.g. `[on_domain_error:NULL]`) may additionally be
//! nullable, since those options can produce nulls at runtime.

use std::collections::{BTreeMap, HashMap};

use substrait_extensions::extensions::SIMPLE_EXTENSIONS;
use substrait_extensions::testcases::TESTCASES;

use super::evaluate::{Value, evaluate};
use crate::parse::text::simple_extensions::TypeExpr;
use crate::parse::text::simple_extensions::types::{IntegerParameter, Type, TypeKind};

/// An argument of a test case, as far as signature matching is concerned.
#[derive(Debug)]
enum ArgType {
    /// An enumeration option (`'FLOOR'::enum`).
    Enum,
    /// A value of the given concrete type.
    Value(Type),
}

/// One parsed test case line.
#[derive(Debug)]
struct Case {
    function: String,
    args: Vec<ArgType>,
    expected: Type,
    /// Whether the case sets options (`[on_domain_error:NULL]`, …), which can
    /// make the result null at runtime regardless of the derived type.
    has_options: bool,
}

/// A function implementation from an extension YAML.
#[derive(Debug)]
struct Impl {
    /// `Some(pattern)` for value arguments, `None` for enumeration arguments.
    args: Vec<Option<String>>,
    variadic: bool,
    return_: String,
    /// The `nullability` handling, `MIRROR` when not given.
    nullability: String,
}

impl Impl {
    /// Whether the return clause does more than name a fixed type, i.e.
    /// exercises parameter binding or derivation logic.
    fn is_derived(&self) -> bool {
        self.return_.contains('<')
            || self.return_.contains('\n')
            || self.return_.to_ascii_lowercase().contains("any")
    }
}

/// Why a test case could not be checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Skip {
    /// The line is not a single function call with typed arguments (table
    /// definitions, column references, lambdas, …).
    UnsupportedLine,
    /// An argument or result type is outside what [`parse_concrete`] handles.
    UnsupportedType,
    /// No implementation's signature matches the argument types.
    NoMatchingImpl,
}

#[test]
fn spec_function_test_cases() {
    let functions = load_functions();

    let mut checked = 0;
    let mut derived = 0;
    let mut skipped: BTreeMap<Skip, usize> = BTreeMap::new();
    let mut failures = Vec::new();

    for file in TESTCASES.find("**/*.test").expect("valid glob") {
        let Some(file) = file.as_file() else { continue };
        let source = file.contents_utf8().expect("test files are UTF-8");
        let urn = source
            .lines()
            .find_map(|l| l.strip_prefix("### SUBSTRAIT_INCLUDE:"))
            .map(|urn| urn.trim().trim_matches('\'').to_string())
            .expect("test file declares its extension");
        let path = file.path().display();

        for (number, line) in source.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with("DEFINE") {
                continue;
            }
            let case = match parse_case(line) {
                Ok(case) => case,
                Err(skip) => {
                    *skipped.entry(skip).or_default() += 1;
                    continue;
                }
            };
            let impls = functions
                .get(&(urn.clone(), case.function.clone()))
                .map(Vec::as_slice)
                .unwrap_or_default();
            let Some((imp, bindings)) = impls
                .iter()
                .find_map(|i| bind(i, &case.args).map(|b| (i, b)))
            else {
                *skipped.entry(Skip::NoMatchingImpl).or_default() += 1;
                continue;
            };

            checked += 1;
            derived += usize::from(imp.is_derived());
            let location = format!("{path}:{}: {line}", number + 1);
            let any_nullable_arg = case
                .args
                .iter()
                .any(|a| matches!(a, ArgType::Value(t) if t.nullable));
            match evaluate(&imp.return_, &bindings) {
                Ok(Value::Type(mut actual)) => {
                    if imp.nullability == "MIRROR" {
                        actual.nullable |= any_nullable_arg;
                    }
                    // Options such as `on_domain_error:NULL` can make the
                    // result null; that is not part of the derivation.
                    if case.has_options && case.expected.nullable {
                        actual.nullable = true;
                    }
                    if actual != case.expected {
                        failures.push(format!(
                            "{location}\n    return {:?} ({}) evaluated to {actual}, expected {}",
                            imp.return_, imp.nullability, case.expected
                        ));
                    }
                }
                Ok(other) => failures.push(format!(
                    "{location}\n    return {:?} evaluated to {other:?}, expected {}",
                    imp.return_, case.expected
                )),
                Err(err) => failures.push(format!(
                    "{location}\n    return {:?} failed: {err}",
                    imp.return_
                )),
            }
        }
    }

    println!(
        "checked {checked} test cases ({derived} with derived return types); skipped {skipped:?}"
    );
    assert!(
        failures.is_empty(),
        "{} of {checked} test cases failed:\n{}",
        failures.len(),
        failures.join("\n")
    );
    // Guard against the test silently checking little (e.g. if the test case
    // format changes and lines end up skipped). Most cases return a fixed type,
    // so guard the derived ones separately.
    assert!(checked > 1000, "only {checked} test cases were checked");
    assert!(
        derived > 150,
        "only {derived} test cases with derived return types were checked"
    );
    // Every test case with supported types must match an implementation; a
    // regression in signature parsing would otherwise just skip cases.
    assert_eq!(
        skipped.get(&Skip::NoMatchingImpl),
        None,
        "unmatched test cases"
    );
}

/// The spec ships no test cases for the decimal arithmetic functions, whose
/// return types are the longest derivation programs in the extensions. Check
/// their actual YAML derivations against results worked out by hand from the
/// formulas, covering both the plain branch and the precision > 38 "borrow"
/// branch (which caps precision at 38 and gives up scale, down to 6).
#[test]
fn decimal_arithmetic_derivations() {
    let functions = load_functions();
    let urn = "extension:io.substrait:functions_arithmetic_decimal";
    // (function, (P1, S1), (P2, S2), expected (precision, scale))
    let cases = [
        ("add", (10, 2), (5, 3), (12, 3)),
        ("add", (38, 10), (38, 10), (38, 9)),
        ("subtract", (10, 2), (5, 3), (12, 3)),
        ("subtract", (38, 10), (38, 10), (38, 9)),
        ("multiply", (10, 2), (5, 3), (16, 5)),
        ("multiply", (38, 10), (38, 10), (38, 6)),
        ("divide", (10, 2), (5, 1), (21, 8)),
        ("divide", (38, 10), (38, 10), (38, 6)),
        ("modulus", (10, 2), (5, 3), (5, 3)),
        ("modulus", (38, 0), (38, 30), (38, 30)),
    ];
    for (function, (p1, s1), (p2, s2), (precision, scale)) in cases {
        let decimal = |p, s| {
            ArgType::Value(Type::new(
                TypeKind::Decimal {
                    precision: IntegerParameter::Literal(p),
                    scale: IntegerParameter::Literal(s),
                },
                false,
            ))
        };
        let args = [decimal(p1, s1), decimal(p2, s2)];
        let impls = &functions[&(urn.to_string(), function.to_string())];
        let (imp, bindings) = impls
            .iter()
            .find_map(|i| bind(i, &args).map(|b| (i, b)))
            .unwrap_or_else(|| panic!("no decimal impl of {function}"));
        assert!(
            imp.return_.contains('\n'),
            "{function} has a derivation program"
        );
        let expected = Type::new(
            TypeKind::Decimal {
                precision: IntegerParameter::Literal(precision),
                scale: IntegerParameter::Literal(scale),
            },
            false,
        );
        assert_eq!(
            evaluate(&imp.return_, &bindings),
            Ok(Value::Type(expected)),
            "{function}(decimal<{p1}, {s1}>, decimal<{p2}, {s2}>)"
        );
    }
}

/// Load all function implementations, keyed by (extension URN, function name).
fn load_functions() -> HashMap<(String, String), Vec<Impl>> {
    let mut functions: HashMap<(String, String), Vec<Impl>> = HashMap::new();
    for (urn, source) in SIMPLE_EXTENSIONS {
        let yaml: serde_yaml::Value = serde_yaml::from_str(source).expect("valid extension YAML");
        for kind in [
            "scalar_functions",
            "aggregate_functions",
            "window_functions",
        ] {
            let Some(list) = yaml.get(kind).and_then(|v| v.as_sequence()) else {
                continue;
            };
            for function in list {
                let name = function["name"]
                    .as_str()
                    .expect("function name")
                    .to_string();
                let entry = functions.entry((urn.to_string(), name)).or_default();
                for imp in function["impls"].as_sequence().expect("impls") {
                    let Some(return_) = imp.get("return").and_then(|r| r.as_str()) else {
                        continue;
                    };
                    let args = imp
                        .get("args")
                        .and_then(|a| a.as_sequence())
                        .map(|args| {
                            args.iter()
                                .map(|a| a.get("value").and_then(|v| v.as_str()).map(String::from))
                                .collect()
                        })
                        .unwrap_or_default();
                    let nullability = imp
                        .get("nullability")
                        .and_then(|n| n.as_str())
                        .unwrap_or("MIRROR")
                        .to_string();
                    entry.push(Impl {
                        args,
                        variadic: imp.get("variadic").is_some(),
                        return_: return_.to_string(),
                        nullability,
                    });
                }
            }
        }
    }
    functions
}

/// Try to match an implementation against the argument types, returning the
/// parameter bindings on success.
fn bind(imp: &Impl, args: &[ArgType]) -> Option<HashMap<String, Value>> {
    let fits = args.len() == imp.args.len()
        || (imp.variadic && !imp.args.is_empty() && args.len() >= imp.args.len());
    if !fits {
        return None;
    }
    let mut bindings = HashMap::new();
    for (index, arg) in args.iter().enumerate() {
        // Variadic implementations repeat their last argument.
        let pattern = &imp.args[index.min(imp.args.len() - 1)];
        match (pattern, arg) {
            (None, ArgType::Enum) => {}
            (Some(pattern), ArgType::Value(actual)) => {
                let pattern = TypeExpr::parse(pattern).ok()?;
                let pattern = Type::try_from(pattern).ok()?;
                // Top-level nullability is governed by the function's
                // nullability handling, not by the signature; nested
                // nullability (e.g. list elements) is part of the type.
                let actual = Type::new(actual.kind.clone(), false);
                if !unify(&pattern, &actual, &mut bindings) {
                    return None;
                }
            }
            _ => return None,
        }
    }
    Some(bindings)
}

/// Structurally match a signature type against a concrete type, binding
/// integer parameters and type variables.
fn unify(pattern: &Type, actual: &Type, bindings: &mut HashMap<String, Value>) -> bool {
    use TypeKind::*;
    match (&pattern.kind, &actual.kind) {
        (Any, _) => true,
        (Variable { id }, _) => {
            bind_value(bindings, format!("any{id}"), Value::Type(actual.clone()))
        }
        (FixedChar { length: p }, FixedChar { length: a })
        | (VarChar { length: p }, VarChar { length: a })
        | (FixedBinary { length: p }, FixedBinary { length: a })
        | (PrecisionTime { precision: p }, PrecisionTime { precision: a })
        | (PrecisionTimestamp { precision: p }, PrecisionTimestamp { precision: a })
        | (PrecisionTimestampTz { precision: p }, PrecisionTimestampTz { precision: a })
        | (IntervalDay { precision: p }, IntervalDay { precision: a })
        | (IntervalCompound { precision: p }, IntervalCompound { precision: a }) => {
            unify_int(p, a, bindings)
        }
        (
            Decimal {
                precision: pp,
                scale: ps,
            },
            Decimal {
                precision: ap,
                scale: as_,
            },
        ) => unify_int(pp, ap, bindings) && unify_int(ps, as_, bindings),
        (List(p), List(a)) => unify(p, a, bindings),
        (Map { key: pk, value: pv }, Map { key: ak, value: av }) => {
            unify(pk, ak, bindings) && unify(pv, av, bindings)
        }
        (p, a) => p == a,
    }
}

fn unify_int(
    pattern: &IntegerParameter,
    actual: &IntegerParameter,
    bindings: &mut HashMap<String, Value>,
) -> bool {
    match (pattern, actual) {
        (IntegerParameter::Literal(p), IntegerParameter::Literal(a)) => p == a,
        (IntegerParameter::Variable(name), IntegerParameter::Literal(a)) => {
            bind_value(bindings, name.clone(), Value::Integer(*a))
        }
        _ => false,
    }
}

/// Bind `name`, or check consistency with an earlier binding of it.
fn bind_value(bindings: &mut HashMap<String, Value>, name: String, value: Value) -> bool {
    match bindings.get(&name) {
        Some(existing) => *existing == value,
        None => {
            bindings.insert(name, value);
            true
        }
    }
}

/// Parse a test case line of the form `name(arg::type, ...) [options] = result::type`.
///
/// Aggregate cases may be prefixed with an inline table, as in
/// `((1.1), (2.2)) variance(SAMPLE::enum, col0::fp32) = 1.21::fp32`; the table
/// only carries values, so it is dropped.
fn parse_case(line: &str) -> Result<Case, Skip> {
    let line = if line.starts_with('(') {
        line[matching_close(line).ok_or(Skip::UnsupportedLine)? + 1..].trim_start()
    } else {
        line
    };
    let open = line.find('(').ok_or(Skip::UnsupportedLine)?;
    let function = line[..open].trim();
    if function.is_empty() || !function.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(Skip::UnsupportedLine);
    }
    let close = open + matching_close(&line[open..]).ok_or(Skip::UnsupportedLine)?;
    let rest = line[close + 1..].trim_start();
    let has_options = rest.starts_with('[');
    let rest = match rest.strip_prefix('[') {
        Some(options) => &options[options.find(']').ok_or(Skip::UnsupportedLine)? + 1..],
        None => rest,
    };
    let result = rest
        .trim_start()
        .strip_prefix('=')
        .ok_or(Skip::UnsupportedLine)?;

    let args = split_top_level(&line[open + 1..close], ',')
        .into_iter()
        .filter(|a| !a.trim().is_empty())
        .map(|arg| {
            let ty = type_suffix(arg).ok_or(Skip::UnsupportedLine)?;
            if ty.eq_ignore_ascii_case("enum") {
                Ok(ArgType::Enum)
            } else {
                parse_concrete(ty)
                    .map(ArgType::Value)
                    .ok_or(Skip::UnsupportedType)
            }
        })
        .collect::<Result<Vec<_>, _>>()?;
    let expected = type_suffix(result).ok_or(Skip::UnsupportedLine)?;
    let expected = parse_concrete(expected).ok_or(Skip::UnsupportedType)?;

    Ok(Case {
        function: function.to_string(),
        args,
        expected,
        has_options,
    })
}

/// Visit the characters of `s` that are outside quotes, with their nesting
/// depth (parentheses, brackets, braces and angle brackets).
fn top_level(s: &str) -> Option<Vec<(usize, char, i32)>> {
    let mut out = Vec::new();
    let mut depth = 0;
    let mut quoted = false;
    let mut previous = ' ';
    for (i, c) in s.char_indices() {
        if c == '\'' {
            quoted = !quoted;
        } else if !quoted {
            match c {
                '(' | '[' | '{' | '<' => depth += 1,
                // `->` in lambdas is not a closing bracket.
                '>' if previous == '-' => {}
                ')' | ']' | '}' | '>' => depth -= 1,
                _ => {}
            }
            if depth < 0 {
                return None;
            }
            out.push((i, c, depth));
        }
        previous = c;
    }
    Some(out)
}

/// Byte offset of the parenthesis closing the one `s` starts with.
fn matching_close(s: &str) -> Option<usize> {
    top_level(s)?
        .into_iter()
        .find(|&(_, c, depth)| c == ')' && depth == 0)
        .map(|(i, _, _)| i)
}

fn split_top_level(s: &str, separator: char) -> Vec<&str> {
    let Some(chars) = top_level(s) else {
        return vec![s];
    };
    let mut parts = Vec::new();
    let mut start = 0;
    for (i, c, depth) in chars {
        if c == separator && depth == 0 {
            parts.push(&s[start..i]);
            start = i + 1;
        }
    }
    parts.push(&s[start..]);
    parts
}

/// The type after the last top-level `::` of a typed literal.
fn type_suffix(s: &str) -> Option<&str> {
    let chars = top_level(s)?;
    let position = chars
        .windows(2)
        .rev()
        .find(|w| w[0].1 == ':' && w[1].1 == ':' && w[0].2 == 0)
        .map(|w| w[1].0 + 1)?;
    Some(s[position..].trim())
}

/// Parse a concrete type as spelled in test cases (short names such as `dec`,
/// `str` or `pts` included). Returns `None` for types outside the scope of
/// these tests (user-defined, functions, structs, …).
fn parse_concrete(s: &str) -> Option<Type> {
    let s = s.trim();
    let (head, params) = match s.find('<') {
        Some(i) => (&s[..i], Some(s[i + 1..].strip_suffix('>')?)),
        None => (s, None),
    };
    let (name, nullable) = match head.strip_suffix('?') {
        Some(name) => (name, true),
        None => (head, false),
    };
    let params = params.map(|p| split_top_level(p, ',')).unwrap_or_default();
    let int = |i: usize| -> Option<IntegerParameter> {
        Some(IntegerParameter::Literal(
            params.get(i)?.trim().parse().ok()?,
        ))
    };
    let only = |n: usize| (params.len() == n).then_some(());

    let kind = match name.to_ascii_lowercase().as_str() {
        "i8" => TypeKind::I8,
        "i16" => TypeKind::I16,
        "i32" => TypeKind::I32,
        "i64" => TypeKind::I64,
        "fp32" => TypeKind::Fp32,
        "fp64" => TypeKind::Fp64,
        "bool" | "boolean" => TypeKind::Boolean,
        "str" | "string" => TypeKind::String,
        "vbin" | "binary" => TypeKind::Binary,
        "date" => TypeKind::Date,
        "iyear" | "interval_year" => TypeKind::IntervalYear,
        "uuid" => TypeKind::Uuid,
        _ => {
            let kind = match name.to_ascii_lowercase().as_str() {
                "dec" | "decimal" => {
                    only(2)?;
                    TypeKind::Decimal {
                        precision: int(0)?,
                        scale: int(1)?,
                    }
                }
                "vchar" | "varchar" => TypeKind::VarChar { length: int(0)? },
                "fchar" | "fixedchar" => TypeKind::FixedChar { length: int(0)? },
                "fbin" | "fixedbinary" => TypeKind::FixedBinary { length: int(0)? },
                "pt" | "precision_time" => TypeKind::PrecisionTime { precision: int(0)? },
                "pts" | "precision_timestamp" => {
                    TypeKind::PrecisionTimestamp { precision: int(0)? }
                }
                "ptstz" | "precision_timestamp_tz" => {
                    TypeKind::PrecisionTimestampTz { precision: int(0)? }
                }
                "iday" | "interval_day" => TypeKind::IntervalDay { precision: int(0)? },
                "icompound" | "interval_compound" => {
                    TypeKind::IntervalCompound { precision: int(0)? }
                }
                "list" => {
                    only(1)?;
                    TypeKind::List(Box::new(parse_concrete(params[0])?))
                }
                "map" => {
                    only(2)?;
                    TypeKind::Map {
                        key: Box::new(parse_concrete(params[0])?),
                        value: Box::new(parse_concrete(params[1])?),
                    }
                }
                _ => return None,
            };
            if !matches!(
                kind,
                TypeKind::Decimal { .. } | TypeKind::List(_) | TypeKind::Map { .. }
            ) {
                only(1)?;
            }
            return Some(Type::new(kind, nullable));
        }
    };
    only(0)?;
    Some(Type::new(kind, nullable))
}
