// SPDX-License-Identifier: Apache-2.0

//! Owned type derivation expressions and evaluator.
//!
//! This module provides owned representations of type derivation expressions
//! and the evaluator that computes concrete return types from them.
//!
//! For the borrowing AST types and parser, see [`super::type_ast`].

use std::collections::HashMap;

use thiserror::Error;

use super::type_ast::{
    is_valid_identifier, AssignmentExpr, BinaryOp, DerivationExpr, Expr, TypeExpr, TypeExprParam,
};
use super::types::{BasicBuiltinType, ConcreteType, ConcreteTypeKind, ExtensionTypeError};

/// An owned type derivation expression for storage and evaluation.
#[derive(Clone, Debug, PartialEq)]
pub struct TypeDerivation {
    /// Variable assignments in order of execution
    pub assignments: Vec<Assignment>,
    /// Final type expression (e.g., `DECIMAL<prec, scale>`)
    pub result_type: ResultTypeExpr,
}

/// An owned variable assignment
#[derive(Clone, Debug, PartialEq)]
pub struct Assignment {
    /// Variable name
    pub name: String,
    /// Value expression
    pub value: OwnedExpr,
}

/// An owned expression
#[derive(Clone, Debug, PartialEq)]
pub enum OwnedExpr {
    /// Integer literal
    Integer(i64),
    /// Identifier (variable or parameter reference)
    Ident(String),
    /// Binary operation
    BinaryOp {
        /// Left operand
        left: Box<OwnedExpr>,
        /// Operator
        op: BinaryOp,
        /// Right operand
        right: Box<OwnedExpr>,
    },
    /// Function call (e.g., `max(a, b)`)
    FunctionCall {
        /// Function name
        name: String,
        /// Function arguments
        args: Vec<OwnedExpr>,
    },
    /// Ternary conditional (`cond ? then : else`)
    Ternary {
        /// Condition expression
        condition: Box<OwnedExpr>,
        /// Value when condition is true
        then_expr: Box<OwnedExpr>,
        /// Value when condition is false
        else_expr: Box<OwnedExpr>,
    },
    /// Parenthesized expression
    Paren(Box<OwnedExpr>),
    /// Unary NOT
    Not(Box<OwnedExpr>),
}

/// An owned type expression for the result type.
#[derive(Clone, Debug, PartialEq)]
pub struct ResultTypeExpr {
    /// Type name (e.g., `DECIMAL`, `VARCHAR`)
    pub name: String,
    /// Type parameters (e.g., precision and scale for DECIMAL)
    pub params: Vec<TypeParam>,
    /// Whether the type is nullable
    pub nullable: bool,
    /// Whether this is a user-defined type (prefixed with `u!`)
    pub user_defined: bool,
}

/// A type parameter (integer literal or variable reference).
#[derive(Clone, Debug, PartialEq)]
pub enum TypeParam {
    /// Integer literal parameter
    Integer(i64),
    /// Variable reference parameter
    Variable(String),
    /// Nested type parameter
    Type(Box<ResultTypeExpr>),
}

/// Errors that can occur while evaluating a type derivation.
#[derive(Debug, Error, PartialEq)]
pub enum EvalError {
    /// Referenced a variable that was not defined
    #[error("undefined variable: {0}")]
    UndefinedVariable(String),
    /// Type mismatch in an operation
    #[error("type error: expected {expected}, got {got}")]
    TypeError {
        /// The expected type
        expected: &'static str,
        /// The actual type
        got: &'static str,
    },
    /// Division by zero
    #[error("division by zero")]
    DivisionByZero,
    /// Unknown function name
    #[error("unknown function: {0}")]
    UnknownFunction(String),
    /// Wrong number of arguments to a function
    #[error("function {name} expects {expected} arguments, got {got}")]
    WrongArgCount {
        /// Function name
        name: String,
        /// Expected argument count
        expected: usize,
        /// Actual argument count
        got: usize,
    },
    /// Error constructing the result type
    #[error("error constructing result type: {0}")]
    TypeConstruction(#[from] ExtensionTypeError),
}

impl From<DerivationExpr<'_>> for TypeDerivation {
    fn from(expr: DerivationExpr<'_>) -> Self {
        TypeDerivation {
            assignments: expr.assignments.into_iter().map(Assignment::from).collect(),
            result_type: ResultTypeExpr::from_type_expr(&expr.result_type),
        }
    }
}

impl From<AssignmentExpr<'_>> for Assignment {
    fn from(expr: AssignmentExpr<'_>) -> Self {
        Assignment {
            name: expr.name.to_string(),
            value: OwnedExpr::from(&expr.value),
        }
    }
}

impl From<&Expr<'_>> for OwnedExpr {
    fn from(expr: &Expr<'_>) -> Self {
        match expr {
            Expr::Integer(n) => OwnedExpr::Integer(*n),
            Expr::Ident(s) => OwnedExpr::Ident(s.to_string()),
            Expr::BinaryOp { left, op, right } => OwnedExpr::BinaryOp {
                left: Box::new(OwnedExpr::from(left.as_ref())),
                op: *op,
                right: Box::new(OwnedExpr::from(right.as_ref())),
            },
            Expr::FunctionCall { name, args } => OwnedExpr::FunctionCall {
                name: name.to_string(),
                args: args.iter().map(OwnedExpr::from).collect(),
            },
            Expr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => OwnedExpr::Ternary {
                condition: Box::new(OwnedExpr::from(condition.as_ref())),
                then_expr: Box::new(OwnedExpr::from(then_expr.as_ref())),
                else_expr: Box::new(OwnedExpr::from(else_expr.as_ref())),
            },
            Expr::Paren(inner) => OwnedExpr::Paren(Box::new(OwnedExpr::from(inner.as_ref()))),
            Expr::Not(inner) => OwnedExpr::Not(Box::new(OwnedExpr::from(inner.as_ref()))),
        }
    }
}

impl ResultTypeExpr {
    fn from_type_expr(expr: &TypeExpr<'_>) -> Self {
        match expr {
            TypeExpr::Simple(name, params, nullable) => ResultTypeExpr {
                name: name.to_string(),
                params: params.iter().map(TypeParam::from_type_expr_param).collect(),
                nullable: *nullable,
                user_defined: false,
            },
            TypeExpr::UserDefined(name, params, nullable) => ResultTypeExpr {
                name: name.to_string(),
                params: params.iter().map(TypeParam::from_type_expr_param).collect(),
                nullable: *nullable,
                user_defined: true,
            },
            TypeExpr::TypeVariable(id, nullable) => ResultTypeExpr {
                name: format!("any{}", id),
                params: vec![],
                nullable: *nullable,
                user_defined: false,
            },
        }
    }
}

impl TypeParam {
    fn from_type_expr_param(param: &TypeExprParam<'_>) -> Self {
        match param {
            TypeExprParam::Integer(n) => TypeParam::Integer(*n),
            TypeExprParam::Type(t) => {
                if let TypeExpr::Simple(name, params, false) = t {
                    if params.is_empty() && is_valid_identifier(name) {
                        return TypeParam::Variable(name.to_string());
                    }
                }
                TypeParam::Type(Box::new(ResultTypeExpr::from_type_expr(t)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Value {
    Integer(i64),
    Boolean(bool),
}

impl Value {
    fn as_integer(&self) -> Result<i64, EvalError> {
        match self {
            Value::Integer(n) => Ok(*n),
            Value::Boolean(_) => Err(EvalError::TypeError {
                expected: "integer",
                got: "boolean",
            }),
        }
    }

    fn as_boolean(&self) -> Result<bool, EvalError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            Value::Integer(_) => Err(EvalError::TypeError {
                expected: "boolean",
                got: "integer",
            }),
        }
    }

    fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Boolean(_) => "boolean",
        }
    }
}

struct EvalContext {
    params: HashMap<String, i64>,
    variables: HashMap<String, Value>,
}

impl EvalContext {
    fn new(params: HashMap<String, i64>) -> Self {
        EvalContext {
            params,
            variables: HashMap::new(),
        }
    }

    fn get(&self, name: &str) -> Result<Value, EvalError> {
        if let Some(v) = self.variables.get(name) {
            return Ok(v.clone());
        }
        if let Some(&n) = self.params.get(name) {
            return Ok(Value::Integer(n));
        }
        Err(EvalError::UndefinedVariable(name.to_string()))
    }

    fn set(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    fn eval_expr(&self, expr: &OwnedExpr) -> Result<Value, EvalError> {
        match expr {
            OwnedExpr::Integer(n) => Ok(Value::Integer(*n)),
            OwnedExpr::Ident(name) => self.get(name),
            OwnedExpr::BinaryOp { left, op, right } => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binary_op(&left_val, *op, &right_val)
            }
            OwnedExpr::FunctionCall { name, args } => {
                let arg_values: Result<Vec<_>, _> =
                    args.iter().map(|a| self.eval_expr(a)).collect();
                self.eval_function(name, arg_values?)
            }
            OwnedExpr::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                let cond = self.eval_expr(condition)?.as_boolean()?;
                if cond {
                    self.eval_expr(then_expr)
                } else {
                    self.eval_expr(else_expr)
                }
            }
            OwnedExpr::Paren(inner) => self.eval_expr(inner),
            OwnedExpr::Not(inner) => {
                let val = self.eval_expr(inner)?.as_boolean()?;
                Ok(Value::Boolean(!val))
            }
        }
    }

    fn eval_binary_op(
        &self,
        left: &Value,
        op: BinaryOp,
        right: &Value,
    ) -> Result<Value, EvalError> {
        match op {
            BinaryOp::Add => Ok(Value::Integer(left.as_integer()? + right.as_integer()?)),
            BinaryOp::Sub => Ok(Value::Integer(left.as_integer()? - right.as_integer()?)),
            BinaryOp::Mul => Ok(Value::Integer(left.as_integer()? * right.as_integer()?)),
            BinaryOp::Div => {
                let r = right.as_integer()?;
                if r == 0 {
                    return Err(EvalError::DivisionByZero);
                }
                Ok(Value::Integer(left.as_integer()? / r))
            }
            BinaryOp::Lt => Ok(Value::Boolean(left.as_integer()? < right.as_integer()?)),
            BinaryOp::Gt => Ok(Value::Boolean(left.as_integer()? > right.as_integer()?)),
            BinaryOp::Le => Ok(Value::Boolean(left.as_integer()? <= right.as_integer()?)),
            BinaryOp::Ge => Ok(Value::Boolean(left.as_integer()? >= right.as_integer()?)),
            BinaryOp::Eq => match (left, right) {
                (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l == r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l == r)),
                _ => Err(EvalError::TypeError {
                    expected: left.type_name(),
                    got: right.type_name(),
                }),
            },
            BinaryOp::Ne => match (left, right) {
                (Value::Integer(l), Value::Integer(r)) => Ok(Value::Boolean(l != r)),
                (Value::Boolean(l), Value::Boolean(r)) => Ok(Value::Boolean(l != r)),
                _ => Err(EvalError::TypeError {
                    expected: left.type_name(),
                    got: right.type_name(),
                }),
            },
            BinaryOp::And => Ok(Value::Boolean(left.as_boolean()? && right.as_boolean()?)),
            BinaryOp::Or => Ok(Value::Boolean(left.as_boolean()? || right.as_boolean()?)),
        }
    }

    fn eval_function(&self, name: &str, args: Vec<Value>) -> Result<Value, EvalError> {
        match name.to_lowercase().as_str() {
            "max" => {
                if args.len() != 2 {
                    return Err(EvalError::WrongArgCount {
                        name: name.to_string(),
                        expected: 2,
                        got: args.len(),
                    });
                }
                Ok(Value::Integer(
                    args[0].as_integer()?.max(args[1].as_integer()?),
                ))
            }
            "min" => {
                if args.len() != 2 {
                    return Err(EvalError::WrongArgCount {
                        name: name.to_string(),
                        expected: 2,
                        got: args.len(),
                    });
                }
                Ok(Value::Integer(
                    args[0].as_integer()?.min(args[1].as_integer()?),
                ))
            }
            _ => Err(EvalError::UnknownFunction(name.to_string())),
        }
    }

    fn resolve_type_param(&self, param: &TypeParam) -> Result<i64, EvalError> {
        match param {
            TypeParam::Integer(n) => Ok(*n),
            TypeParam::Variable(name) => self.get(name)?.as_integer(),
            TypeParam::Type(_) => Err(EvalError::TypeError {
                expected: "integer",
                got: "type",
            }),
        }
    }
}

impl TypeDerivation {
    /// Evaluate the derivation with given parameters, returning a ConcreteType.
    pub fn evaluate(&self, params: HashMap<String, i64>) -> Result<ConcreteType, EvalError> {
        let mut ctx = EvalContext::new(params);

        for assign in &self.assignments {
            let value = ctx.eval_expr(&assign.value)?;
            ctx.set(assign.name.clone(), value);
        }

        resolve_type(&self.result_type, &ctx)
    }
}

fn resolve_type(type_expr: &ResultTypeExpr, ctx: &EvalContext) -> Result<ConcreteType, EvalError> {
    let name_lower = type_expr.name.to_ascii_lowercase();

    match name_lower.as_str() {
        "decimal" => {
            if type_expr.params.len() != 2 {
                return Err(EvalError::TypeConstruction(
                    ExtensionTypeError::InvalidParameterCount {
                        type_name: "DECIMAL".to_string(),
                        expected: 2,
                        actual: type_expr.params.len(),
                    },
                ));
            }
            let precision = ctx.resolve_type_param(&type_expr.params[0])? as i32;
            let scale = ctx.resolve_type_param(&type_expr.params[1])? as i32;
            Ok(ConcreteType {
                kind: ConcreteTypeKind::Builtin(BasicBuiltinType::Decimal { precision, scale }),
                nullable: type_expr.nullable,
            })
        }
        "varchar" => {
            if type_expr.params.len() != 1 {
                return Err(EvalError::TypeConstruction(
                    ExtensionTypeError::InvalidParameterCount {
                        type_name: "VARCHAR".to_string(),
                        expected: 1,
                        actual: type_expr.params.len(),
                    },
                ));
            }
            let length = ctx.resolve_type_param(&type_expr.params[0])? as i32;
            Ok(ConcreteType {
                kind: ConcreteTypeKind::Builtin(BasicBuiltinType::VarChar { length }),
                nullable: type_expr.nullable,
            })
        }
        "fixedchar" => {
            if type_expr.params.len() != 1 {
                return Err(EvalError::TypeConstruction(
                    ExtensionTypeError::InvalidParameterCount {
                        type_name: "FIXEDCHAR".to_string(),
                        expected: 1,
                        actual: type_expr.params.len(),
                    },
                ));
            }
            let length = ctx.resolve_type_param(&type_expr.params[0])? as i32;
            Ok(ConcreteType {
                kind: ConcreteTypeKind::Builtin(BasicBuiltinType::FixedChar { length }),
                nullable: type_expr.nullable,
            })
        }
        "fixedbinary" => {
            if type_expr.params.len() != 1 {
                return Err(EvalError::TypeConstruction(
                    ExtensionTypeError::InvalidParameterCount {
                        type_name: "FIXEDBINARY".to_string(),
                        expected: 1,
                        actual: type_expr.params.len(),
                    },
                ));
            }
            let length = ctx.resolve_type_param(&type_expr.params[0])? as i32;
            Ok(ConcreteType {
                kind: ConcreteTypeKind::Builtin(BasicBuiltinType::FixedBinary { length }),
                nullable: type_expr.nullable,
            })
        }
        "bool" | "boolean" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Boolean,
            type_expr.nullable,
        )),
        "i8" => Ok(ConcreteType::builtin(
            BasicBuiltinType::I8,
            type_expr.nullable,
        )),
        "i16" => Ok(ConcreteType::builtin(
            BasicBuiltinType::I16,
            type_expr.nullable,
        )),
        "i32" => Ok(ConcreteType::builtin(
            BasicBuiltinType::I32,
            type_expr.nullable,
        )),
        "i64" => Ok(ConcreteType::builtin(
            BasicBuiltinType::I64,
            type_expr.nullable,
        )),
        "fp32" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Fp32,
            type_expr.nullable,
        )),
        "fp64" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Fp64,
            type_expr.nullable,
        )),
        "string" => Ok(ConcreteType::builtin(
            BasicBuiltinType::String,
            type_expr.nullable,
        )),
        "binary" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Binary,
            type_expr.nullable,
        )),
        "timestamp" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Timestamp,
            type_expr.nullable,
        )),
        "timestamp_tz" => Ok(ConcreteType::builtin(
            BasicBuiltinType::TimestampTz,
            type_expr.nullable,
        )),
        "date" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Date,
            type_expr.nullable,
        )),
        "time" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Time,
            type_expr.nullable,
        )),
        "interval_year" => Ok(ConcreteType::builtin(
            BasicBuiltinType::IntervalYear,
            type_expr.nullable,
        )),
        "uuid" => Ok(ConcreteType::builtin(
            BasicBuiltinType::Uuid,
            type_expr.nullable,
        )),
        _ => {
            if type_expr.user_defined {
                let params = type_expr
                    .params
                    .iter()
                    .map(|p| match p {
                        TypeParam::Integer(n) => Ok(super::types::TypeParameter::Integer(*n)),
                        TypeParam::Variable(name) => {
                            let val = ctx.get(name)?.as_integer()?;
                            Ok(super::types::TypeParameter::Integer(val))
                        }
                        TypeParam::Type(t) => {
                            let resolved = resolve_type(t, ctx)?;
                            Ok(super::types::TypeParameter::Type(resolved))
                        }
                    })
                    .collect::<Result<Vec<_>, EvalError>>()?;

                Ok(ConcreteType::extension_with_params(
                    type_expr.name.clone(),
                    params,
                    type_expr.nullable,
                ))
            } else {
                Err(EvalError::TypeConstruction(
                    ExtensionTypeError::UnknownTypeName {
                        name: type_expr.name.clone(),
                    },
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::text::simple_extensions::type_ast::DerivationExpr;

    #[test]
    fn test_eval_decimal_add() {
        let input = r#"init_scale = max(S1,S2)
init_prec = init_scale + max(P1 - S1, P2 - S2) + 1
min_scale = min(init_scale, 6)
delta = init_prec - 38
prec = min(init_prec, 38)
scale_after_borrow = max(init_scale - delta, min_scale)
scale = init_prec > 38 ? scale_after_borrow : init_scale
DECIMAL<prec, scale>"#;

        let expr = DerivationExpr::parse(input).unwrap();
        let derivation = TypeDerivation::from(expr);

        // DECIMAL<10,2> + DECIMAL<5,3> should produce DECIMAL<12,3>
        let params = HashMap::from([
            ("P1".to_string(), 10_i64),
            ("S1".to_string(), 2),
            ("P2".to_string(), 5),
            ("S2".to_string(), 3),
        ]);

        let result = derivation.evaluate(params).unwrap();
        match result.kind {
            ConcreteTypeKind::Builtin(BasicBuiltinType::Decimal { precision, scale }) => {
                assert_eq!((precision, scale), (12, 3));
            }
            _ => panic!("expected Decimal type"),
        }
    }
}
