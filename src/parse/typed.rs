// SPDX-License-Identifier: Apache-2.0

//! A generic new type wrapper.

use std::{any, fmt, hash, marker::PhantomData};

/// A generic new type wrapper for values of type `T` that belong to items of type `U`.
pub struct Typed<T, U> {
    /// The wrapped value.
    value: T,
    /// The attached type.
    _type: PhantomData<U>,
}

impl<T, U> Typed<T, U> {
    /// Internal method to construct a new wrapper from a value.
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            _type: PhantomData,
        }
    }

    /// Returns a reference to the wrapped value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: AsRef<V>, U, V: ?Sized> AsRef<V> for Typed<T, U> {
    fn as_ref(&self) -> &V {
        self.value.as_ref()
    }
}

impl<T: Clone, U> Clone for Typed<T, U> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _type: self._type,
        }
    }
}

impl<T: Copy, U> Copy for Typed<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for Typed<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(any::type_name::<U>())
            .field(&self.value)
            .finish()
    }
}

impl<T: fmt::Display, U> fmt::Display for Typed<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<T: PartialEq, U> PartialEq for Typed<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, U> Eq for Typed<T, U> {}

impl<T: hash::Hash, U> hash::Hash for Typed<T, U> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// A generic anchor new type for the anchor mechanism used in Substrait data.
pub type Anchor<T> = Typed<u32, T>;

/// A generic Name type for the named things inside of Substrait.
pub type Name<T> = Typed<String, T>;
