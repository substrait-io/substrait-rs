// SPDX-License-Identifier: Apache-2.0

//! Type-derivation expressions: the Substrait DSL used in extension function
//! `return` clauses, e.g. `decimal<P1 + 1, S1>` or multiline programs.
//!
//! Parsing uses the ANTLR parser generated from the Substrait
//! `grammar/SubstraitType.g4`, packaged in the [`substrait_antlr`] crate at the
//! same spec version as the other Substrait dependencies. [`evaluate`]
//! evaluates the parsed expressions into [`Type`](super::types::Type)s.

#[cfg(test)]
mod conformance;
mod evaluate;
