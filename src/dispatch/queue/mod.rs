use super::{DispatchObject, DispatchQos, DispatchQosClass};
use std::{
    ffi::{CStr, CString},
    fmt,
    os::raw::{c_char, c_int, c_long, c_ulong},
    ptr,
};

mod attr;
mod builder;
mod priority;

pub use attr::*;
pub use builder::*;
pub use priority::*;

subclass! {
    /// An object that manages the execution of tasks serially or concurrently on
    /// your app's main thread or on a background thread.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_queue)
    pub class DispatchQueue: DispatchObject;
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

extern "C" {
    fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> &'static DispatchQueue;
}

/// Getting global queues.
impl DispatchQueue {
    /// The serial dispatch queue associated with the main thread of the current
    /// process.
    #[inline]
    #[doc(alias = "dispatch_get_main_queue")]
    pub fn main() -> &'static Self {
        extern "C" {
            static mut _dispatch_main_q: DispatchQueue;
        }
        unsafe { &_dispatch_main_q }
    }

    /// Returns the global system concurrent queue with the specified
    /// quality-of-service class.
    #[inline]
    #[doc(alias = "dispatch_get_global_queue")]
    pub fn global_with_qos(qos_class: DispatchQosClass) -> &'static Self {
        unsafe { dispatch_get_global_queue(qos_class as _, 0) }
    }

    /// Returns the global system concurrent queue with the specified priority.
    #[inline]
    #[doc(alias = "dispatch_get_global_queue")]
    pub fn global_with_priority(priority: DispatchQueuePriority) -> &'static Self {
        unsafe { dispatch_get_global_queue(priority as _, 0) }
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
    fn dispatch_queue_get_label(queue: *const DispatchQueue) -> *const c_char;
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
    #[doc(alias = "dispatch_queue_get_label")]
    pub unsafe fn current_queue_label<'a>() -> Option<&'a CStr> {
        let label = dispatch_queue_get_label(ptr::null());
        if label.is_null() {
            None
        } else {
            Some(CStr::from_ptr(label))
        }
    }

    /// Returns an owned copy of the label of the current queue.
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn current_queue_label_owned() -> Option<CString> {
        Self::with_current_queue_label(|label| Some(label?.to_owned()))
    }

    /// Returns the result of calling the function with a reference to the label
    /// of the current queue.
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn with_current_queue_label<F, T>(f: F) -> T
    where
        F: FnOnce(Option<&CStr>) -> T,
    {
        // SAFETY: The string cannot be used past the lifetime of the current
        // queue because the reference only lives as long as the scope of `f`.
        f(unsafe { Self::current_queue_label() })
    }

    /// Returns the label assigned to the queue at creation time.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1780825-label) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452939-dispatch_queue_get_label)
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn label(&self) -> Option<&CStr> {
        unsafe {
            let label = dispatch_queue_get_label(self);
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
    #[doc(alias = "dispatch_queue_get_qos_class")]
    pub fn qos(&self) -> DispatchQos {
        extern "C" {
            fn dispatch_queue_get_qos_class(
                queue: *const DispatchQueue,
                relative_priority_ptr: *mut c_int,
            ) -> DispatchQosClass;
        }

        let mut relative_priority = 0;
        let qos_class = unsafe { dispatch_queue_get_qos_class(self, &mut relative_priority) };

        DispatchQos::new(qos_class, relative_priority)
    }
}

/// Queue operations.
impl DispatchQueue {
    // TODO: Implement operations.
}
