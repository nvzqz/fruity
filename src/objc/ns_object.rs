use super::{Class, ClassType, NSUInteger, ObjCObject, BOOL, SEL};
use crate::core::Arc;

// TODO: Create `NSObjectProtocol` for `@protocol NSObject` and `Deref` to that.
objc_subclass! {
    /// An instance of the root class for most Objective-C objects.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject).
    pub class NSObject: ObjCObject;
}

impl Default for Arc<NSObject> {
    #[inline]
    fn default() -> Self {
        unsafe { NSObject::class().alloc_init() }
    }
}

impl PartialEq for NSObject {
    #[inline]
    fn eq(&self, other: &NSObject) -> bool {
        unsafe { _msg_send_any_cached![self, isEqual: other => BOOL] }.into()
    }
}

impl NSObject {
    /// Returns this object's reference count.
    ///
    /// This method is only useful for debugging certain objects.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1571952-retaincount).
    #[inline]
    pub fn retain_count(&self) -> usize {
        unsafe { _msg_send_any_cached![self, retainCount] }
    }

    /// Returns `true` if this object implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    pub fn responds_to_selector(&self, selector: SEL) -> bool {
        unsafe { _msg_send_any_cached![self, respondsToSelector: selector => BOOL] }.into()
    }

    /// Returns `true` if this object is an instance or subclass of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418511-iskindofclass)
    #[inline]
    pub fn is_kind_of_class(&self, class: &Class) -> bool {
        unsafe { _msg_send_any_cached![self, isKindOfClass: class => BOOL] }.into()
    }

    /// Returns `true` if this object is an instance of `class`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418766-ismemberofclass)
    #[inline]
    pub fn is_member_of_class(&self, class: &Class) -> bool {
        unsafe { _msg_send_any_cached![self, isMemberOfClass: class => BOOL] }.into()
    }

    /// Returns an integer that can be used as a table address in a hash table
    /// structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418859-hash).
    #[inline]
    pub fn hash(&self) -> NSUInteger {
        unsafe { _msg_send_any_cached![self, hash] }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> Arc<NSObject> {
        unsafe { _msg_send_any_cached![self, copy] }
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    #[doc(alias = "mutableCopy")]
    pub fn mutable_copy(&self) -> Arc<NSObject> {
        unsafe { _msg_send_any_cached![self, mutableCopy] }
    }
}
