use super::{sys, DispatchObject, DispatchQueue, DispatchTime};
use crate::core::Arc;
use std::ptr;

mod type_;

pub use type_::*;

// TODO: Create wrapper types for specific dispatch source types.

// TODO: Create types for the flags of different dispatch source types.

subclass! {
    /// An object that coordinates the processing of specific low-level system
    /// events, such as file-system events, timers, and UNIX signals.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsource) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_source)
    pub class DispatchSource: DispatchObject;
}

impl DispatchSource {
    /// Creates a new dispatch source to monitor low-level system events.
    ///
    /// Dispatch sources are created in a suspended state. After creating the
    /// source and setting any desired attributes (for example, the handler or
    /// the context), your application must call
    /// [`activate`](DispatchObject::activate) to begin event delivery.
    ///
    /// **Important:** Event source creation is asynchronous, so be aware of any
    /// race conditions with monitored system handles. For example, if a
    /// dispatch source is created for a process and that process exits before
    /// the source is created, any specified cancellation handler may not be
    /// called.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385630-dispatch_source_create)
    ///
    /// # Safety
    ///
    /// The expectations for the given source type must be upheld.
    #[inline]
    #[doc(alias = "dispatch_source_create")]
    pub unsafe fn create(
        source_type: &DispatchSourceType,
        handle: usize,
        mask: usize,
        queue: Option<&DispatchQueue>,
    ) -> Arc<Self> {
        let queue = match queue {
            Some(queue) => queue,
            None => ptr::null(),
        };
        Arc::from_raw(sys::dispatch_source_create(
            source_type,
            handle,
            mask,
            queue,
        ))
    }

    /// Returns pending data for this dispatch source.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsourceprotocol/1781051-data) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385658-dispatch_source_get_data)
    #[inline]
    #[doc(alias = "dispatch_source_get_data")]
    pub fn data(&self) -> usize {
        unsafe { sys::dispatch_source_get_data(self) }
    }

    /// Merges data into a dispatch source and submits its event handler block
    /// to its target queue.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385658-dispatch_source_get_data)
    ///
    /// # Safety
    ///
    /// This dispatch source must be of type [`DispatchSourceType::data_add`] or
    /// [`DispatchSourceType::data_or`].
    #[inline]
    #[doc(alias = "dispatch_source_get_data")]
    pub unsafe fn merge_data(&self, value: usize) {
        sys::dispatch_source_merge_data(self, value);
    }

    /// Returns the mask of events monitored by this dispatch source.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsourceprotocol/1780529-mask) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385612-dispatch_source_get_mask)
    #[inline]
    #[doc(alias = "dispatch_source_get_mask")]
    pub fn mask(&self) -> usize {
        unsafe { sys::dispatch_source_get_mask(self) }
    }

    /// Returns the underlying system handle associated with this dispatch
    /// source.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsourceprotocol/1781068-handle) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385670-dispatch_source_get_handle)
    #[inline]
    #[doc(alias = "dispatch_source_get_handle")]
    pub fn handle(&self) -> usize {
        unsafe { sys::dispatch_source_get_handle(self) }
    }

    /// Sets a start time, interval, and leeway value for this dispatch timer
    /// source.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385606-dispatch_source_set_timer)
    ///
    /// # Safety
    ///
    /// This dispatch source must be of type [`DispatchSourceType::timer`].
    #[inline]
    #[doc(alias = "dispatch_source_set_timer")]
    pub unsafe fn set_timer(&self, start: DispatchTime, interval: u64, leeway: u64) {
        sys::dispatch_source_set_timer(self, start, interval, leeway);
    }

    /// Asynchronously cancels this dispatch source, preventing any further
    /// invocation of its event handler block.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsourceprotocol/1780783-cancel) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385604-dispatch_source_cancel)
    #[inline]
    #[doc(alias = "dispatch_source_cancel")]
    pub fn cancel(&self) {
        unsafe { sys::dispatch_source_cancel(self) };
    }

    /// Returns `true` if this dispatch source has been cancelled.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchsourceprotocol/1780754-iscancelled) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1385616-dispatch_source_testcancel)
    #[inline]
    #[doc(alias = "dispatch_source_testcancel")]
    pub fn is_cancelled(&self) -> bool {
        unsafe { sys::dispatch_source_testcancel(self) != 0 }
    }
}
