// SPDX-License-Identifier: Apache-2.0

//! A parse context.

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {}

#[cfg(test)]
pub(crate) mod tests {
    #[derive(Default)]
    pub struct Context {}

    impl super::Context for Context {}
}
