use super::{Class, SEL};
use std::{cell::UnsafeCell, fmt, ops::Deref, ptr::NonNull};

/// An opaque object instance.
///
/// This is designed to be used behind a reference. In the future, this will be
/// defined as an
/// [`extern type`](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md).
///
/// All Objective-C types within this crate ultimately
/// [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) to this type.
#[repr(C)]
pub struct Object {
    // Stores data that may live in the `__DATA` link section, which is mutable.
    // It is normally undefined behavior for references to alias mutable data.
    // We can inform Rust that this data is internally mutable by using
    // `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for Object {}

impl fmt::Debug for Object {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl Object {
    // Do not call these methods directly. Use the `_msg_send!` macro instead.
    #[inline]
    pub(crate) unsafe fn _msg_send<T>(&self, sel: SEL) -> T
    where
        T: 'static,
    {
        self._msg_send_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_with<A, T>(&self, sel: SEL, args: A) -> T
    where
        A: super::msg::MsgArgs,
        T: 'static,
    {
        A::msg_send(self, sel, args)
    }
}

impl Object {
    /// Casts `self` to a raw nullable pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut Object {
        self._data.get().cast()
    }

    /// Casts `self` to a raw non-null pointer.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<Object> {
        NonNull::from(self).cast()
    }

    /// Returns the class that this object is an instance of.
    #[inline]
    pub fn get_class(&self) -> &'static Class {
        unsafe { _msg_send![self, class] }
    }
}

/// A non-null smart pointer to any object instance, including classes.
///
/// This is semantically equivalent to `id _Nonnull` in Objective-C.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/id).
///
/// # Distinction from `NSObject`
///
/// `NSObject` is the root of _almost_ all Objective-C classes. Although very
/// rare, it is possible for other root objects to exist. For example, one you
/// can find
#[repr(transparent)]
#[allow(non_camel_case_types)]
pub struct id(NonNull<Object>);

unsafe impl Send for id {}
unsafe impl Sync for id {}

impl Deref for id {
    type Target = Object;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl Drop for id {
    #[inline]
    fn drop(&mut self) {
        extern "C" {
            fn objc_release(obj: &Object);
        }
        unsafe { objc_release(self) };
    }
}

impl Clone for id {
    #[inline]
    fn clone(&self) -> Self {
        extern "C" {
            fn objc_retain(obj: &Object) -> NonNull<Object>;
        }
        Self(unsafe { objc_retain(self) })
    }
}

impl fmt::Debug for id {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl fmt::Pointer for id {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl id {
    /// Creates an object identifier from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid Objective-C object instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NonNull::new_unchecked(ptr.cast()))
    }

    /// Creates an object identifier from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid Objective-C object instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(ptr.cast())
    }

    /// Casts `self` to a raw nullable pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut Object {
        self.0.as_ptr()
    }

    /// Casts `self` to a raw non-null pointer.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<Object> {
        self.0
    }
}
