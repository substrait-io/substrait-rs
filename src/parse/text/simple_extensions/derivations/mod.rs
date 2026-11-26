// SPDX-License-Identifier: Apache-2.0

//! Type-derivation expressions: the Substrait DSL used in extension function
//! `return` clauses, e.g. `decimal<P1 + 1, S1>` or multiline programs.
//!
//! Derivations are parsed into an [`Expr`](super::type_ast::Expr) by the same
//! parser as types ([`parse_derivation`](super::type_parser::parse_derivation)),
//! and [`evaluate`] evaluates them into [`Type`](super::types::Type)s.

#[cfg(test)]
mod conformance;
mod evaluate;
