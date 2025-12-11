// SPDX-License-Identifier: Apache-2.0

//! Utilities for working with Substrait *text* objects.
//!
//! The generated [`crate::text`] module exposes the raw YAML-derived structs
//! (e.g. [`crate::text::simple_extensions::SimpleExtensions`]).  This module
//! provides parsing helpers that validate those raw values and offer
//! higher-level wrappers for validation, lookups, and combining into protobuf
//! objects.

pub mod simple_extensions;
