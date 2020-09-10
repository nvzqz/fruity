use super::{id, Class, NSUInteger, Object, BOOL, SEL};
use std::{ops::Deref, ptr::NonNull};

/// The root class for most Objective-C objects.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject).
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct NSObject(id);

impl Deref for NSObject {
    type Target = id;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
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

    /// Creates an object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSObject` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(id::from_ptr(ptr))
    }

    /// Creates an object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSObject` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(id::from_non_null_ptr(ptr))
    }

    /// Returns a pointer to this object's data.
    #[inline]
    pub fn as_id(&self) -> &id {
        &self.0
    }

    /// Returns a raw nullable pointer to this object's data.
    #[inline]
    pub fn as_ptr(&self) -> *mut Object {
        self.0.as_ptr()
    }

    /// Returns a raw non-null pointer to this object's data.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<Object> {
        self.0.as_non_null_ptr()
    }

    /// Returns `true` if this object implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    pub fn responds_to_selector(&self, selector: SEL) -> bool {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, selector: SEL) -> BOOL;
        }

        let sel = selector!(respondsToSelector:);

        unsafe { objc_msgSend(self, sel, selector) != 0 }
    }

    /// Returns `true` if this object is an instance or subclass of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass)
    #[inline]
    pub fn is_kind_of_class(&self, class: &Class) -> bool {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, class: &Class) -> BOOL;
        }

        let sel = selector!(isKindOfClass:);

        unsafe { objc_msgSend(self, sel, class) != 0 }
    }

    /// Returns `true` if this object is an instance of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418766-ismemberofclass)
    #[inline]
    pub fn is_member_of_class(&self, class: &Class) -> bool {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, class: &Class) -> BOOL;
        }

        let sel = selector!(isMemberOfClass:);

        unsafe { objc_msgSend(self, sel, class) != 0 }
    }

    /// Returns an integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash).
    #[inline]
    pub fn hash(&self) -> NSUInteger {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL) -> NSUInteger;
        }

        let sel = selector!(hash);

        unsafe { objc_msgSend(self, sel) }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> NSObject {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL) -> NSObject;
        }

        let sel = selector!(copy);

        unsafe { objc_msgSend(self, sel) }
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> NSObject {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL) -> NSObject;
        }

        let sel = selector!(mutableCopy);

        unsafe { objc_msgSend(self, sel) }
    }
}
