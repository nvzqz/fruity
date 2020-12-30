//! Raw unsafe C functions exposed by libdispatch.

use super::{
    DispatchObject, DispatchQosClass, DispatchQueue, DispatchSource, DispatchSourceType,
    DispatchTime,
};
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

    #[doc(alias = "DISPATCH_SOURCE_TYPE_ADD")]
    pub static _dispatch_source_type_data_add: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_OR")]
    pub static _dispatch_source_type_data_or: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_REPLACE")]
    pub static _dispatch_source_type_data_replace: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_MACH_SEND")]
    pub static _dispatch_source_type_mach_send: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_MACH_RECV")]
    pub static _dispatch_source_type_mach_recv: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_MEMORYPRESSURE")]
    pub static _dispatch_source_type_memorypressure: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_PROC")]
    pub static _dispatch_source_type_proc: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_READ")]
    pub static _dispatch_source_type_read: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_SIGNAL")]
    pub static _dispatch_source_type_signal: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_TIMER")]
    pub static _dispatch_source_type_timer: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_VNODE")]
    pub static _dispatch_source_type_vnode: DispatchSourceType;

    #[doc(alias = "DISPATCH_SOURCE_TYPE_WRITE")]
    pub static _dispatch_source_type_write: DispatchSourceType;

    pub fn dispatch_source_create(
        type_: *const DispatchSourceType,
        handle: usize,
        mask: usize,
        queue: *const DispatchQueue,
    ) -> *const DispatchSource;

    pub fn dispatch_source_cancel(source: *const DispatchSource);
    pub fn dispatch_source_testcancel(source: *const DispatchSource) -> isize;

    pub fn dispatch_source_get_data(source: *const DispatchSource) -> usize;
    pub fn dispatch_source_get_mask(source: *const DispatchSource) -> usize;
    pub fn dispatch_source_get_handle(source: *const DispatchSource) -> usize;
    pub fn dispatch_source_merge_data(source: *const DispatchSource, value: usize);

    pub fn dispatch_source_set_timer(
        source: *const DispatchSource,
        start: DispatchTime,
        interval: u64,
        leeway: u64,
    );

    // TODO: Create safe wrapper methods for these functions.
    pub fn dispatch_source_set_registration_handler_f(
        source: *const DispatchSource,
        handler: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
    );
    pub fn dispatch_source_set_event_handler_f(
        source: *const DispatchSource,
        handler: unsafe extern "C" fn(ctx: *mut c_void),
    );
    pub fn dispatch_source_set_cancel_handler_f(
        source: *const DispatchSource,
        handler: Option<unsafe extern "C" fn(ctx: *mut c_void)>,
    );
}
