use super::{DispatchObject, DispatchQos, DispatchQosClass};
use std::{
    ffi::{CStr, CString},
    fmt,
    ops::Deref,
    os::raw::{c_char, c_int, c_long, c_ulong, c_void},
    ptr::{self, NonNull},
};

mod attr;
mod builder;
mod priority;

pub use attr::*;
pub use builder::*;
pub use priority::*;

#[allow(non_camel_case_types)]
pub(crate) type dispatch_queue_t = *mut dispatch_queue_s;

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) struct dispatch_queue_s {
    _priv: [u8; 0],
}

/// An object that manages the execution of tasks serially or concurrently on
/// your app's main thread or on a background thread.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_queue)
#[repr(transparent)]
#[derive(Clone)]
pub struct DispatchQueue(DispatchObject);

#[cfg(feature = "objc")]
unsafe impl crate::objc::ObjectType for DispatchQueue {}

impl Deref for DispatchQueue {
    type Target = DispatchObject;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<DispatchObject> for DispatchQueue {
    #[inline]
    fn as_ref(&self) -> &DispatchObject {
        self
    }
}

impl AsRef<DispatchQueue> for DispatchQueue {
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Debug for DispatchQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let qos = self.qos();

        f.debug_struct("DispatchQueue")
            .field("label", &self.label().unwrap_or_default())
            .field("qos_class", &qos.qos_class)
            .field("relative_priority", &qos.relative_priority)
            .finish()
    }
}

impl fmt::Pointer for DispatchQueue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl DispatchQueue {
    #[inline]
    pub(crate) const unsafe fn _from_queue(queue: dispatch_queue_t) -> Self {
        Self::from_ptr(queue.cast())
    }

    /// Creates an object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `DispatchQueue` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut c_void) -> Self {
        Self(DispatchObject::from_ptr(ptr))
    }

    /// Creates an object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `DispatchQueue` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<c_void>) -> Self {
        Self(DispatchObject::from_non_null_ptr(ptr))
    }
}

extern "C" {
    fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> dispatch_queue_t;
}

/// Getting global queues.
impl DispatchQueue {
    /// The serial dispatch queue associated with the main thread of the current
    /// process.
    #[inline]
    pub fn main() -> Self {
        extern "C" {
            static mut _dispatch_main_q: dispatch_queue_s;
        }
        unsafe { Self::_from_queue(&mut _dispatch_main_q) }
    }

    /// Returns the global system concurrent queue with the specified
    /// quality-of-service class.
    #[inline]
    pub fn global_with_qos(qos_class: DispatchQosClass) -> Self {
        unsafe { Self::_from_queue(dispatch_get_global_queue(qos_class as _, 0)) }
    }

    /// Returns the global system concurrent queue with the specified priority.
    #[inline]
    pub fn global_with_priority(priority: DispatchQueuePriority) -> Self {
        unsafe { Self::_from_queue(dispatch_get_global_queue(priority as _, 0)) }
    }
}

/// Creating queues.
impl DispatchQueue {
    // This type deliberately does not have a `new` method or implement
    // `Default` because it's very uncommon to create an unlabeled serial queue
    // (the default).

    /// Returns a value that can be used to configure and create a queue.
    #[inline]
    pub fn builder<'a>() -> DispatchQueueBuilder<'a> {
        DispatchQueueBuilder::new()
    }
}

extern "C" {
    fn dispatch_queue_get_label(queue: dispatch_queue_t) -> *const c_char;
}

/// Queue properties.
impl DispatchQueue {
    /// Returns a reference to the label of the current queue.
    ///
    /// # Safety
    ///
    /// The returned label must live as long as the current queue.
    ///
    /// Consider instead using
    /// [`current_queue_label_owned`](Self::current_queue_label_owned) or
    /// [`with_current_queue_label`](Self::with_current_queue_label),
    /// depending on how long the string is needed for.
    #[inline]
    pub unsafe fn current_queue_label<'a>() -> Option<&'a CStr> {
        let label = dispatch_queue_get_label(ptr::null_mut());
        if label.is_null() {
            None
        } else {
            Some(CStr::from_ptr(label))
        }
    }

    /// Returns an owned copy of the label of the current queue.
    #[inline]
    pub fn current_queue_label_owned() -> Option<CString> {
        Self::with_current_queue_label(|label| Some(label?.to_owned()))
    }

    /// Returns the result of calling the function with a reference to the label
    /// of the current queue.
    #[inline]
    pub fn with_current_queue_label<F, T>(f: F) -> T
    where
        F: FnOnce(Option<&CStr>) -> T,
    {
        // SAFETY: The string cannot be used past the lifetime of the current
        // queue because the reference only lives as long as the scope of `f`.
        f(unsafe { Self::current_queue_label() })
    }

    #[inline]
    pub(crate) fn _as_queue(&self) -> dispatch_queue_t {
        self.as_ptr().cast()
    }

    /// Returns the label assigned to the queue at creation time.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1780825-label) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452939-dispatch_queue_get_label)
    #[inline]
    pub fn label(&self) -> Option<&CStr> {
        unsafe {
            let label = dispatch_queue_get_label(self._as_queue());
            if label.is_null() {
                None
            } else {
                Some(CStr::from_ptr(label))
            }
        }
    }

    /// Returns the quality-of-service level assigned to the queue.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1781008-qos) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452829-dispatch_queue_get_qos_class)
    #[inline]
    pub fn qos(&self) -> DispatchQos {
        extern "C" {
            fn dispatch_queue_get_qos_class(
                queue: dispatch_queue_t,
                relative_priority_ptr: *mut c_int,
            ) -> DispatchQosClass;
        }

        let mut relative_priority = 0;
        let qos_class =
            unsafe { dispatch_queue_get_qos_class(self._as_queue(), &mut relative_priority) };

        DispatchQos::new(qos_class, relative_priority)
    }
}

/// Queue operations.
impl DispatchQueue {
    // TODO: Implement operations.
}
