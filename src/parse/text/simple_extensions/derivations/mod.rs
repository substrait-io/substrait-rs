// SPDX-License-Identifier: Apache-2.0

//! ANTLR-generated parser for the Substrait type-derivation DSL.
//!
//! These submodules are generated from `substrait/grammar/SubstraitType.g4`
//! (tracked via the `substrait` submodule) using the ANTLR Rust target. They
//! are committed to the repository so that an ordinary `cargo build` does not
//! require a Java runtime.
//!
//! Do not edit the generated files by hand; run `pixi run gen-antlr` to
//! regenerate them. See `antlr/README.md` for details.
//!
//! [`evaluate`] consumes the generated parser to evaluate type-derivation
//! expressions into [`ConcreteType`](super::types::ConcreteType)s.

mod evaluate;
mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;
