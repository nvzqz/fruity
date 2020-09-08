use super::Class;
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
    pub fn as_ptr(self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Returns a raw non-null pointer to this object's data.
    #[inline]
    pub fn as_non_null_ptr(self) -> NonNull<c_void> {
        self.0.as_non_null_ptr()
    }
}

extern "C" {
    fn objc_retain(obj: id) -> id;
    fn objc_release(obj: id);
}
