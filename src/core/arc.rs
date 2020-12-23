use super::ObjectType;
use std::{
    fmt,
    hash::{Hash, Hasher},
    mem::{self, ManuallyDrop},
    ops::Deref,
    ptr::NonNull,
};

// TODO: Implement infallible `Arc` casting for class chain.

/// A thread-safe automatically-reference-counted pointer to an object.
#[repr(transparent)]
pub struct Arc<T: ObjectType> {
    obj: NonNull<T>,
}

impl<T: ObjectType> Clone for Arc<T> {
    #[inline]
    fn clone(&self) -> Self {
        Arc::retain(self)
    }
}

impl<T: ObjectType> Drop for Arc<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { T::release(self.obj.cast()) };
    }
}

impl<T: ObjectType> Deref for Arc<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.obj.as_ref() }
    }
}

impl<T: ObjectType> AsRef<T> for Arc<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

unsafe impl<T: ObjectType + Send + Sync> Send for Arc<T> {}
unsafe impl<T: ObjectType + Send + Sync> Sync for Arc<T> {}

impl<'a, T: ObjectType + 'a> Default for Arc<T>
where
    &'a T: Default,
{
    #[inline]
    fn default() -> Self {
        Arc::retain(<&T>::default())
    }
}

impl<T: ObjectType + fmt::Display> fmt::Display for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ObjectType + fmt::Debug> fmt::Debug for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

impl<T: ObjectType> fmt::Pointer for Arc<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.obj.fmt(f)
    }
}

impl<T: ObjectType + Hash> Hash for Arc<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl<T: ObjectType> Arc<T> {
    /// An alias for [`ObjectType::retain`].
    #[inline]
    #[must_use = "The retained object is immediately released if unused"]
    pub fn retain(obj: &T) -> Self {
        ObjectType::retain(obj)
    }

    /// Constructs an `Arc<T>` from a raw pointer.
    ///
    /// # Safety
    ///
    /// The value at `obj` must be a valid instance of `T`.
    ///
    /// After calling this method, there should not be more `Arc`s to `obj` than
    /// the internal reference count, or else the object could be over-released
    /// and the program will either abort, read/write unowned memory, or trigger
    /// undefined behavior.
    #[inline]
    pub unsafe fn from_raw(obj: *const T) -> Self {
        Self {
            obj: NonNull::new_unchecked(obj as *mut T),
        }
    }

    /// Constructs an `Arc<T>` from a raw pointer and retains it.
    ///
    /// # Safety
    ///
    /// The value at `obj` must be a valid instance of `T`.
    #[inline]
    pub unsafe fn retain_raw(obj: *const T) -> Self {
        Self::retain(&ManuallyDrop::new(Self::from_raw(obj)))
    }

    /// Consumes the `Arc`, returning the wrapped pointer.
    ///
    /// To avoid a memory leak, the pointer must be converted back to an `Arc`
    /// using [`Arc::from_raw`].
    #[inline]
    pub fn into_raw(this: Self) -> *const T {
        let obj = this.obj;
        mem::forget(this);
        obj.as_ptr()
    }

    ///
    ///
    /// # Safety
    ///
    /// `U` must represent a subtype or equivalent type as `this` at runtime.
    /// For Objective-C, that means `U` must be the same class or a subclass of
    /// `this`.
    #[inline]
    pub unsafe fn cast_unchecked<U: ObjectType>(this: Self) -> Arc<U> {
        Arc::from_raw(Self::into_raw(this).cast())
    }
}
