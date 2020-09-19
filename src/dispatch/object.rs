use super::{dispatch_queue_t, DispatchQueue};
use std::{
    ffi::c_void,
    fmt,
    ptr::{self, NonNull},
};

#[allow(non_camel_case_types)]
type dispatch_object_t = NonNull<c_void>;

/// The base type for dispatch objects.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchobject) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_object_t)
#[repr(transparent)]
pub struct DispatchObject(NonNull<c_void>);

#[cfg(feature = "objc")]
unsafe impl crate::objc::ObjectType for DispatchObject {}

unsafe impl Send for DispatchObject {}
unsafe impl Sync for DispatchObject {}

impl Drop for DispatchObject {
    #[inline]
    fn drop(&mut self) {
        extern "C" {
            fn dispatch_release(obj: dispatch_object_t);
        }
        unsafe { dispatch_release(self.0) };
    }
}

impl Clone for DispatchObject {
    #[inline]
    fn clone(&self) -> Self {
        self._retain();
        Self(self.0)
    }
}

impl AsRef<DispatchObject> for DispatchObject {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Pointer for DispatchObject {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl DispatchObject {
    /// Creates an object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid Dispatch object instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        Self(NonNull::new_unchecked(ptr.cast()))
    }

    /// Creates an object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid Dispatch object instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<c_void>) -> Self {
        Self(ptr.cast())
    }

    #[inline]
    pub(crate) fn _retain(&self) {
        extern "C" {
            fn dispatch_retain(obj: dispatch_object_t);
        }
        unsafe { dispatch_retain(self.0) };
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

/// Dispatch operations.
impl DispatchObject {
    /// Activates `self`.
    ///
    /// Once a dispatch object has been activated, it cannot change its target
    /// queue.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1641002-dispatch_activate).
    #[inline]
    pub fn activate(&self) {
        extern "C" {
            fn dispatch_activate(obj: dispatch_object_t);
        }
        unsafe { dispatch_activate(self.0) }
    }

    /// Resumes the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452929-dispatch_resume).
    #[inline]
    pub fn resume(&self) {
        extern "C" {
            fn dispatch_resume(obj: dispatch_object_t);
        }
        unsafe { dispatch_resume(self.0) }
    }

    /// Suspends the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452801-dispatch_suspend).
    #[inline]
    pub fn suspend(&self) {
        extern "C" {
            fn dispatch_suspend(obj: dispatch_object_t);
        }
        unsafe { dispatch_suspend(self.0) }
    }

    /// Specifies the dispatch queue on which to perform work associated with
    /// `self`.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchobject/1452989-settarget) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452989-dispatch_set_target_queue)
    #[inline]
    pub fn set_target<Q>(&self, queue: Q)
    where
        for<'q> Q: Into<Option<&'q DispatchQueue>>,
    {
        extern "C" {
            fn dispatch_set_target_queue(object: dispatch_object_t, queue: dispatch_queue_t);
        }

        let target = match queue.into() {
            Some(queue) => queue._as_queue(),
            None => ptr::null_mut(),
        };

        unsafe { dispatch_set_target_queue(self.0, target) };
    }

    /// Returns the application-defined context of an object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1453005-dispatch_get_context).
    #[inline]
    pub fn context(&self) -> *mut c_void {
        extern "C" {
            fn dispatch_get_context(obj: dispatch_object_t) -> *mut c_void;
        }
        unsafe { dispatch_get_context(self.0) }
    }

    /// Associates an application-defined context with the object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452807-dispatch_set_context).
    #[inline]
    pub fn set_context(&self, context: *mut c_void) {
        extern "C" {
            fn dispatch_set_context(obj: dispatch_object_t, context: *mut c_void);
        }
        unsafe { dispatch_set_context(self.0, context) }
    }
}
