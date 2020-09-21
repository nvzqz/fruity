use super::{Object, ObjectType};
use std::{fmt, marker::PhantomData, ops::Deref, ptr::NonNull};

/// A non-retaining object pointer.
///
/// This is semantically a `&T`, but with the memory representation of `T` when
/// `T` implements [`ObjectType`](trait.ObjectType.html). Unlike `T`, this does
/// not decrement the retain count on
/// [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html).
///
/// This type can be safely used in a callback function that takes an object
/// instance without an incremented retain count for the consumer to decrement.
#[repr(transparent)]
pub struct Unretained<'a, T>(NonNull<Object>, PhantomData<&'a T>);

unsafe impl<T: ObjectType> ObjectType for Unretained<'_, T> {}

unsafe impl<T: Sync> Send for Unretained<'_, T> {}
unsafe impl<T: Sync> Sync for Unretained<'_, T> {}

impl<T> Clone for Unretained<'_, T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Unretained<'_, T> {}

impl<T: ObjectType> Deref for Unretained<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `ObjectType` requires that `T` has the same memory
        // representation as `NonNull<Object>`.
        unsafe { &*(self as *const _ as *const _) }
    }
}

impl<T: ObjectType> AsRef<T> for Unretained<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

impl<'a, T: ObjectType> From<&'a T> for Unretained<'a, T> {
    #[inline]
    fn from(obj: &'a T) -> Self {
        Self::from_obj(obj)
    }
}

impl<T> fmt::Pointer for Unretained<'_, T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, T> Unretained<'a, T> {
    /// Creates an unretained pointer to `obj`.
    #[inline]
    pub fn from_obj(obj: &'a T) -> Self
    where
        T: ObjectType,
    {
        // SAFETY: The instance must live as long as `obj`.
        unsafe { Self::from_ptr(obj.as_object().as_ptr()) }
    }

    /// Creates an unretained object pointer from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid retained Objective-C object
    /// instance that will not outlive the created pointer.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self::from_non_null_ptr(NonNull::new_unchecked(ptr))
    }

    /// Creates an unretained object pointer from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid retained Objective-C object
    /// instance that will not outlive the created pointer.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(ptr, PhantomData)
    }

    /// Gets the underlying object value by retaining it in the appropriate way.
    #[inline]
    pub fn get(&self) -> T
    where
        T: ObjectType + Clone,
    {
        T::clone(self)
    }
}
