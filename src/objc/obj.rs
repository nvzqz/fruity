use super::{Class, NSUInteger, BOOL, SEL};
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
    // It is normally undefined behavior for shared references to point to
    // mutable data. We can inform Rust that this data is internally mutable by
    // using `UnsafeCell`.
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
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL) -> &'static Class;
        }

        let sel = selector!(class);

        unsafe { objc_msgSend(self, sel) }
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
