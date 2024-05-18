// SPDX-License-Identifier: Apache-2.0

#![allow(
    clippy::clone_on_copy,
    clippy::derivable_impls,
    clippy::to_string_trait_impl,
    clippy::uninlined_format_args,
    unused_variables
)]

//! Generated types for text-based definitions.

include!(concat!(env!("OUT_DIR"), "/substrait_text.rs"));
