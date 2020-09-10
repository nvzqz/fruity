use super::{Class, NSUInteger, BOOL, SEL};
use std::{ffi::c_void, fmt, ptr::NonNull};

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
pub struct id(NonNull<c_void>);

unsafe impl Send for id {}
unsafe impl Sync for id {}

impl Drop for id {
    #[inline]
    fn drop(&mut self) {
        extern "C" {
            fn objc_release(obj: NonNull<c_void>);
        }
        unsafe { objc_release(self.0) };
    }
}

impl Clone for id {
    #[inline]
    fn clone(&self) -> Self {
        extern "C" {
            fn objc_retain(obj: NonNull<c_void>) -> NonNull<c_void>;
        }
        Self(unsafe { objc_retain(self.0) })
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
    pub const unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        Self(NonNull::new_unchecked(ptr))
    }

    /// Creates an object identifier from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid Objective-C object instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<c_void>) -> Self {
        Self(ptr)
    }

    /// Casts `self` to a raw nullable pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    /// Casts `self` to a raw non-null pointer.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<c_void> {
        self.0
    }
}

/// The root class for most Objective-C objects.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject).
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct NSObject(id);

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
    pub fn as_id(&self) -> &id {
        &self.0
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

    /// Returns `true` if this object implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    pub fn responds_to_selector(&self, selector: SEL) -> bool {
        extern "C" {
            fn objc_msgSend(obj: *mut c_void, sel: SEL, selector: SEL) -> BOOL;
        }

        let obj = self.as_ptr();
        let sel = selector!(respondsToSelector:);

        unsafe { objc_msgSend(obj, sel, selector) != 0 }
    }

    /// Returns the class that this object is an instance of.
    #[inline]
    pub fn get_class(&self) -> &'static Class {
        extern "C" {
            fn objc_msgSend(obj: *mut c_void, sel: SEL) -> &'static Class;
        }

        let obj = self.as_ptr();
        let sel = selector!(class);

        unsafe { objc_msgSend(obj, sel) }
    }

    /// Returns `true` if this object is an instance or subclass of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass)
    #[inline]
    pub fn is_kind_of_class(&self, class: &Class) -> bool {
        extern "C" {
            fn objc_msgSend(obj: *mut c_void, sel: SEL, class: &Class) -> BOOL;
        }

        let obj = self.as_ptr();
        let sel = selector!(isKindOfClass:);

        unsafe { objc_msgSend(obj, sel, class) != 0 }
    }

    /// Returns `true` if this object is an instance of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418766-ismemberofclass)
    #[inline]
    pub fn is_member_of_class(&self, class: &Class) -> bool {
        extern "C" {
            fn objc_msgSend(obj: *mut c_void, sel: SEL, class: &Class) -> BOOL;
        }

        let obj = self.as_ptr();
        let sel = selector!(isMemberOfClass:);

        unsafe { objc_msgSend(obj, sel, class) != 0 }
    }

    /// Returns an integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash).
    #[inline]
    pub fn hash(&self) -> NSUInteger {
        extern "C" {
            fn objc_msgSend(obj: *mut c_void, sel: SEL) -> NSUInteger;
        }

        let obj = self.as_ptr();
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
            fn objc_msgSend(obj: *mut c_void, sel: SEL) -> NSObject;
        }

        let obj = self.as_ptr();
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
            fn objc_msgSend(obj: *mut c_void, sel: SEL) -> NSObject;
        }

        let obj = self.as_ptr();
        let sel = selector!(mutableCopy);

        unsafe { objc_msgSend(obj, sel) }
    }
}
