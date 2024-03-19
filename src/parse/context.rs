// SPDX-License-Identifier: Apache-2.0

//! A parse context.

use crate::parse::Parse;

/// A parse context.
///
/// Parsing Substrait data is context-sensitive. This trait provides methods
/// that can be used by parser implementations to parse Substrait data.
pub trait Context {
    /// Parse an item with this context.
    ///
    /// See [Parse::parse].
    fn parse<T: Parse<Self>>(&mut self, item: T) -> Result<T::Parsed, T::Error>
    where
        Self: Sized,
    {
        item.parse(self)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    #[derive(Default)]
    pub struct Context {}

    impl super::Context for Context {}
}
