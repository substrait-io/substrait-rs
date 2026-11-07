// SPDX-License-Identifier: Apache-2.0

//! ANTLR-generated parser for the Substrait type-derivation DSL.
//!
//! These submodules are generated from the Substrait `grammar/SubstraitType.g4`
//! (at the spec version pinned by `substrait-prost` in `Cargo.toml`) using the
//! ANTLR Rust target. They
//! are committed to the repository so that an ordinary `cargo build` does not
//! require a Java runtime.
//!
//! Do not edit the generated files by hand; run `pixi run gen-antlr` to
//! regenerate them. See `antlr/README.md` for details.
//!
//! [`evaluate`] consumes the generated parser to evaluate type-derivation
//! expressions into [`Type`](super::types::Type)s.

mod evaluate;
mod substraittypelexer;
mod substraittypelistener;
mod substraittypeparser;
