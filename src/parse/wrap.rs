use std::{fmt, hash, marker::PhantomData};

/// A generic new type wrapper for values of type `T` that belong to items of type `U`.
pub struct Wrapper<T, U> {
    value: T,
    _type: PhantomData<U>,
}

impl<T, U> Wrapper<T, U> {
    /// Internal method to construct a new wrapper from a value.
    pub(super) fn new(value: T) -> Self {
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

impl<T: AsRef<V>, U, V: ?Sized> AsRef<V> for Wrapper<T, U> {
    fn as_ref(&self) -> &V {
        self.value.as_ref()
    }
}

impl<T: Clone, U> Clone for Wrapper<T, U> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _type: self._type,
        }
    }
}

impl<T: Copy, U> Copy for Wrapper<T, U> {}

impl<T: fmt::Debug, U> fmt::Debug for Wrapper<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

impl<T: fmt::Display, U> fmt::Display for Wrapper<T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<T: PartialEq, U> PartialEq for Wrapper<T, U> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: Eq, U> Eq for Wrapper<T, U> {}

impl<T: hash::Hash, U> hash::Hash for Wrapper<T, U> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

/// A generic anchor new type for the anchor mechanism used in Substrait data.
pub type Anchor<T> = Wrapper<u32, T>;

/// A generic new type around a string representing a name.
pub type Name<T> = Wrapper<String, T>;
