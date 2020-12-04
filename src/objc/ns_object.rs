use super::{id, Class, NSUInteger, Object, ObjectType, BOOL, SEL};
use std::{fmt, ops::Deref, ptr::NonNull};

/// The root class for most Objective-C objects.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject).
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct NSObject(id);

unsafe impl ObjectType for NSObject {}

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

impl<T: ObjectType> PartialEq<T> for NSObject {
    #[inline]
    fn eq(&self, other: &T) -> bool {
        unsafe { _msg_send_cached![self, isEqual: other.as_object() => BOOL] }.into()
    }
}

impl fmt::Pointer for NSObject {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
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

    /// Returns this object's reference count.
    ///
    /// This method is only useful for debugging certain objects.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1571952-retaincount).
    #[inline]
    pub fn retain_count(&self) -> usize {
        unsafe { _msg_send_cached![self, retainCount] }
    }

    /// Returns `true` if this object implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    pub fn responds_to_selector(&self, selector: SEL) -> bool {
        unsafe { _msg_send_cached![self, respondsToSelector: selector => BOOL] }.into()
    }

    /// Returns `true` if this object is an instance or subclass of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass)
    #[inline]
    pub fn is_kind_of_class(&self, class: &Class) -> bool {
        unsafe { _msg_send_cached![self, isKindOfClass: class => BOOL] }.into()
    }

    /// Returns `true` if this object is an instance of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418766-ismemberofclass)
    #[inline]
    pub fn is_member_of_class(&self, class: &Class) -> bool {
        unsafe { _msg_send_cached![self, isMemberOfClass: class => BOOL] }.into()
    }

    /// Returns an integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash).
    #[inline]
    pub fn hash(&self) -> NSUInteger {
        unsafe { _msg_send_cached![self, hash] }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> NSObject {
        unsafe { _msg_send_cached![self, copy] }
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> NSObject {
        unsafe { _msg_send_cached![self, mutableCopy] }
    }
}
