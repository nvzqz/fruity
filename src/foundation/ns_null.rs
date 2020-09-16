use crate::objc::{Class, NSObject, Object, ObjectType};
use std::{fmt, ops::Deref};

/// A singleton object used to represent null values in collection objects that
/// don’t allow `nil` values.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsnull).
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NSNull(&'static Object);

unsafe impl ObjectType for NSNull {
    #[inline]
    fn as_object(&self) -> &'static Object {
        self.0
    }
}

impl From<NSNull> for NSObject {
    #[inline]
    fn from(obj: NSNull) -> Self {
        unsafe { Self::from_ptr(obj.0.as_ptr()) }
    }
}

impl Deref for NSNull {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        // SAFETY: `NSObject` is a transparent wrapper over `NonNull<Object>`.
        unsafe { &*(self as *const Self as *const NSObject) }
    }
}

impl fmt::Pointer for NSNull {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl NSNull {
    /// Returns the `NSNull` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSNull"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Returns the singleton instance.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnull).
    #[inline]
    pub fn null() -> Self {
        extern "C" {
            // `NSNull` is toll-free bridged with `CFNullRef` whose only
            // instance is this.
            static kCFNull: NSNull;
        }
        unsafe { kCFNull }
    }
}
