use super::DispatchQueue;
use crate::core::{Arc, ObjectType};
use std::{cell::UnsafeCell, ffi::c_void, panic::RefUnwindSafe, ptr::NonNull};

/// The base type for dispatch objects.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchobject) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/&DispatchObject)
#[repr(C)]
pub struct DispatchObject {
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

impl ObjectType for DispatchObject {
    #[inline]
    fn retain(obj: &Self) -> Arc<Self> {
        extern "C" {
            fn dispatch_retain(obj: &DispatchObject);
        }
        unsafe {
            dispatch_retain(obj);
            Arc::from_raw(obj)
        }
    }

    #[inline]
    unsafe fn release(obj: NonNull<Self>) {
        extern "C" {
            fn dispatch_release(obj: NonNull<DispatchObject>);
        }
        dispatch_release(obj);
    }
}

unsafe impl Send for DispatchObject {}
unsafe impl Sync for DispatchObject {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for DispatchObject {}

impl AsRef<DispatchObject> for DispatchObject {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
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
            fn dispatch_activate(obj: &DispatchObject);
        }
        unsafe { dispatch_activate(self) }
    }

    /// Resumes the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452929-dispatch_resume).
    #[inline]
    pub fn resume(&self) {
        extern "C" {
            fn dispatch_resume(obj: &DispatchObject);
        }
        unsafe { dispatch_resume(self) }
    }

    /// Suspends the invocation of block objects on `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452801-dispatch_suspend).
    #[inline]
    pub fn suspend(&self) {
        extern "C" {
            fn dispatch_suspend(obj: &DispatchObject);
        }
        unsafe { dispatch_suspend(self) }
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
            fn dispatch_set_target_queue(obj: &DispatchObject, queue: Option<&DispatchQueue>);
        }

        unsafe { dispatch_set_target_queue(self, queue.into()) };
    }

    /// Returns the application-defined context of an object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1453005-dispatch_get_context).
    #[inline]
    pub fn context(&self) -> *mut c_void {
        extern "C" {
            fn dispatch_get_context(obj: &DispatchObject) -> *mut c_void;
        }
        unsafe { dispatch_get_context(self) }
    }

    /// Associates an application-defined context with the object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/1452807-dispatch_set_context).
    #[inline]
    pub fn set_context(&self, context: *mut c_void) {
        extern "C" {
            fn dispatch_set_context(obj: &DispatchObject, context: *mut c_void);
        }
        unsafe { dispatch_set_context(self, context) }
    }
}
