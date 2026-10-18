//! A wrapper to hold Rune values in places which require [`Send`].
#![allow(unsafe_code)]

/// Holds a value which is not [`Send`] or [`Sync`], i.e. Rune functions, so it can be captured by script functions.
///
/// SAFETY: Rune values are reference counted without atomics. BMS only runs a script context from one place at a time,
/// every call into a script goes through the exclusive access to its context and the world, so the counts are never touched concurrently.
pub struct SendWrapper<T>(T);

impl<T> SendWrapper<T> {
    /// Wraps the value
    pub fn new(value: T) -> Self {
        Self(value)
    }

    /// The wrapped value, always use this and not the field, so closures capture the whole wrapper.
    pub fn get(&self) -> &T {
        &self.0
    }
}

// SAFETY: see the type's documentation
unsafe impl<T> Send for SendWrapper<T> {}
// SAFETY: see the type's documentation
unsafe impl<T> Sync for SendWrapper<T> {}
