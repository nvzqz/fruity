//! Safe memory management utilities.

use std::{marker::PhantomData, ops::Deref};

/// Wraps a value that internally references immutable data.
///
/// This is used to safely expose immutable objects that were initialized by
/// referencing a buffer instead of copying it.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NoCopy<'a, T> {
    value: T,
    reference: PhantomData<&'a ()>,
}

impl<T> Deref for NoCopy<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> NoCopy<'a, T> {
    /// Creates a new wrapper with an implicit reference lifetime.
    #[inline]
    pub unsafe fn new(value: T) -> Self {
        Self {
            value,
            reference: PhantomData,
        }
    }
}

/// Wraps a value that internally references mutable data.
///
/// This is used to safely expose mutable objects that were initialized by
/// referencing a buffer instead of copying it.
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NoCopyMut<'a, T> {
    value: T,
    reference: PhantomData<&'a mut ()>,
}

impl<T> Deref for NoCopyMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a, T> NoCopyMut<'a, T> {
    /// Creates a new wrapper with an implicit reference lifetime.
    #[inline]
    pub unsafe fn new(value: T) -> Self {
        Self {
            value,
            reference: PhantomData,
        }
    }
}
