use super::{Class, NSUInteger, SEL};
use std::{ffi::c_void, fmt, ptr::NonNull};

/// A non-null pointer to a class instance.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/id).
#[repr(transparent)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct id(NonNull<c_void>);

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
    /// Casts `self` to a raw nullable pointer.
    #[inline]
    pub fn as_ptr(self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Casts `self` to a raw non-null pointer.
    #[inline]
    pub fn as_non_null_ptr(self) -> NonNull<c_void> {
        self.0
    }
}

/// The root class for most Objective-C objects.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject).
#[repr(transparent)]
#[derive(Debug)]
pub struct NSObject(id);

unsafe impl Send for NSObject {}
unsafe impl Sync for NSObject {}

impl Drop for NSObject {
    #[inline]
    fn drop(&mut self) {
        unsafe { objc_release(self.as_id()) };
    }
}

impl Clone for NSObject {
    #[inline]
    fn clone(&self) -> Self {
        Self(unsafe { objc_retain(self.as_id()) })
    }
}

impl AsRef<NSObject> for NSObject {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl NSObject {
    /// Returns the `NSObject` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSObject"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Returns a pointer to this object's data.
    #[inline]
    pub fn as_id(&self) -> id {
        self.0
    }

    /// Returns a raw nullable pointer to this object's data.
    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Returns a raw non-null pointer to this object's data.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<c_void> {
        self.0.as_non_null_ptr()
    }

    /// Returns an integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash).
    #[inline]
    pub fn hash(&self) -> NSUInteger {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL) -> NSUInteger;
        }

        let obj = self.as_id();
        let sel = selector!(hash);

        unsafe { objc_msgSend(obj, sel) }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> NSObject {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL) -> NSObject;
        }

        let obj = self.as_id();
        let sel = selector!(copy);

        unsafe { objc_msgSend(obj, sel) }
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> NSObject {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL) -> NSObject;
        }

        let obj = self.as_id();
        let sel = selector!(copy);

        unsafe { objc_msgSend(obj, sel) }
    }
}

extern "C" {
    fn objc_retain(obj: id) -> id;
    fn objc_release(obj: id);
}
