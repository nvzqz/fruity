use super::{sys, Class, Ivar, ObjectType, Sel, BOOL};
use crate::core::Arc;
use std::{
    alloc,
    cell::UnsafeCell,
    ffi::{c_void, CStr},
    fmt,
    marker::PhantomData,
    mem::ManuallyDrop,
    panic::RefUnwindSafe,
    ptr::NonNull,
};

/// An automatically-reference-counted pointer to a type-erased Objective-C
/// object.
///
/// This is semantically equivalent to `id _Nonnull` in ARC-ed Objective-C.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/id).
#[allow(non_camel_case_types)]
pub type id<'data> = Arc<ObjCObject<'data>>;

/// A type-erased instance of any Objective-C class.
///
/// This is designed to be used behind a reference or smart pointer like
/// [`Arc`](../obj/struct.Arc.html). In the future, this will be defined as an
/// [`extern type`](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md).
///
/// All Objective-C class types within this crate ultimately
/// [`Deref`](std::ops::Deref) to this type.
///
/// This is equivalent to [`objc_object`](https://developer.apple.com/documentation/objectivec/objc_object?language=objc).
///
/// # Distinction from `NSObject`
///
/// `NSObject` is the root of _almost_ all Objective-C classes. Although very
/// rare, it is possible for other root classes to exist, such as `NSProxy`.
#[repr(C)]
pub struct ObjCObject<'data> {
    // TODO: Figure out the correct lifetime variance for `'data`.
    _marker: PhantomData<&'data ()>,
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

impl crate::core::ObjectType for ObjCObject<'_> {
    #[inline]
    #[doc(alias = "objc_retain")]
    fn retain(obj: &Self) -> Arc<Self> {
        extern "C" {
            fn objc_retain<'data>(obj: &ObjCObject<'data>) -> Arc<ObjCObject<'data>>;
        }
        unsafe { objc_retain(obj) }
    }

    #[inline]
    #[doc(alias = "objc_release")]
    unsafe fn release(obj: NonNull<Self>) {
        extern "C" {
            fn objc_release(obj: NonNull<ObjCObject>);
        }
        objc_release(obj);
    }
}

impl<'data> super::ObjectType<'data> for ObjCObject<'data> {
    #[inline]
    fn class<'s>(&'s self) -> &'s Class
    where
        'data: 's,
    {
        // TODO: Call `_objc_opt_class` on:
        // - macOS 10.15+
        // - iOS (unknown)
        // - tvOS (unknown)
        // - watchOS (unknown)
        unsafe { _msg_send_strict_cached![self, class] }
    }
}

impl<'data> AsRef<ObjCObject<'data>> for ObjCObject<'data> {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for ObjCObject<'_> {}
unsafe impl Send for ObjCObject<'_> {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for ObjCObject<'_> {}

impl fmt::Debug for ObjCObject<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as *const Self).fmt(f)
    }
}

impl ObjCObject<'_> {
    /// Returns `true` if this class implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    #[doc(alias = "respondsToSelector")]
    pub fn responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { _msg_send_any_cached![self, respondsToSelector: selector => BOOL] }.into()
    }

    /// Changes the value of an instance variable of a class instance and
    /// returns its [`Ivar`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1441498-object_setinstancevariable?language=objc).
    ///
    /// # Safety
    ///
    /// `value` must be the expected memory layout and representation for the
    /// object's instance variable. This is checked in debug builds.
    #[inline]
    #[doc(alias = "object_setInstanceVariable")]
    pub unsafe fn set_ivar<'a, T>(&'a mut self, name: &CStr, value: T) -> &'a Ivar {
        let mut value = ManuallyDrop::new(value);
        let value = &mut *value as *mut T as *mut c_void;

        // Ensure in debug builds that this is kosher.
        if cfg!(debug_assertions) {
            let class = self.class();
            let ty = std::any::type_name::<T>();

            let ivar = class.get_ivar(name).unwrap_or_else(|| {
                let class = class.name();
                panic!("No instance variable {name:?} on {class:?}");
            });

            assert_eq!(
                ivar.type_encoding().layout(),
                alloc::Layout::new::<T>(),
                "Incorrect layout for writing `{ty}` to {name:?} on {class:?}",
            );
        }

        &*sys::object_setInstanceVariable(self, name.as_ptr(), value)
    }
}
