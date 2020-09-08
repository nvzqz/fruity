use crate::objc::{Class, NSObject};
use std::ops::Deref;

/// A static, plain-text Unicode string object.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsstring).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSString(NSObject);

impl From<NSString> for NSObject {
    #[inline]
    fn from(obj: NSString) -> Self {
        obj.0
    }
}

impl Deref for NSString {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        &self.0
    }
}

impl NSString {
    /// Returns the `NSString` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSString"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> NSString {
        Self(NSObject::copy(self))
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> NSMutableString {
        NSMutableString(Self(NSObject::mutable_copy(self)))
    }
}

/// A dynamic plain-text Unicode string object.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsmutablestring).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSMutableString(NSString);

impl From<NSMutableString> for NSObject {
    #[inline]
    fn from(obj: NSMutableString) -> Self {
        (obj.0).0
    }
}

impl From<NSMutableString> for NSString {
    #[inline]
    fn from(obj: NSMutableString) -> Self {
        obj.0
    }
}

impl Deref for NSMutableString {
    type Target = NSString;

    #[inline]
    fn deref(&self) -> &NSString {
        &self.0
    }
}

impl NSMutableString {
    /// Returns the `NSMutableString` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSMutableString"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }
}
