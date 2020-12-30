//! Raw unsafe C functions exposed by libdispatch.

use super::{DispatchObject, DispatchQosClass, DispatchQueue, DispatchTime};
use std::os::raw::{c_char, c_int, c_long, c_ulong, c_void};

// Dispatch is reexported by libSystem on Apple platforms.
#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
#[allow(missing_docs)]
extern "C" {
    pub static _dispatch_main_q: DispatchQueue;

    pub fn dispatch_retain(obj: *const DispatchObject);
    pub fn dispatch_release(obj: *const DispatchObject);

    pub fn dispatch_activate(obj: *const DispatchObject);
    pub fn dispatch_resume(obj: *const DispatchObject);
    pub fn dispatch_suspend(obj: *const DispatchObject);

    pub fn dispatch_get_context(obj: *const DispatchObject) -> *mut c_void;
    pub fn dispatch_set_context(obj: *const DispatchObject, context: *mut c_void);

    pub fn dispatch_set_target_queue(obj: *const DispatchObject, queue: *const DispatchQueue);
    pub fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> *const DispatchQueue;
    pub fn dispatch_queue_get_label(queue: *const DispatchQueue) -> *const c_char;
    pub fn dispatch_queue_get_qos_class(
        queue: *const DispatchQueue,
        relative_priority_ptr: *mut c_int,
    ) -> DispatchQosClass;

    pub fn dispatch_async_f(
        queue: *const DispatchQueue,
        ctx: *mut c_void,
        work: unsafe extern "C" fn(ctx: *mut c_void),
    );
    pub fn dispatch_sync_f(
        queue: *const DispatchQueue,
        ctx: *mut c_void,
        work: unsafe extern "C" fn(ctx: *mut c_void),
    );
    pub fn dispatch_apply_f(
        iterations: usize,
        queue: *const DispatchQueue,
        ctx: *mut c_void,
        work: unsafe extern "C" fn(ctx: *mut c_void, iteration: usize),
    );

    pub fn dispatch_time(when: DispatchTime, delta: i64) -> DispatchTime;
}
