use crate::dispatch::sys;
use std::{fmt, hash, ptr};

/// An identifier for the type of system object being monitored by a dispatch
/// source.
///
/// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_t).
#[repr(C)]
pub struct DispatchSourceType {
    _data: [u8; 0],
}

impl PartialEq for DispatchSourceType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for DispatchSourceType {}

impl hash::Hash for DispatchSourceType {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (self as *const Self).hash(state);
    }
}

/// Convenience macro to reduce boilerplate for `DispatchSourceType` methods.
macro_rules! dispatch_source_types {
    ($(
        $(#[$meta:meta])*
        $name:ident = $value:ident, $macro:literal;
    )+) => {
        impl fmt::Debug for DispatchSourceType {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Format as macro name if known.
                let name = $(if self == Self::$name() {
                    $macro.fmt(f)
                } else)+ {
                    // Format as pointer by default if unknown.
                    return (self as *const Self).fmt(f);
                };

                name.fmt(f)
            }
        }

        impl DispatchSourceType {
            $(
                $(#[$meta])*
                #[inline]
                #[doc(alias = $macro)]
                pub fn $name() -> &'static Self {
                    unsafe { &sys::$value }
                }
            )+
        }
    };
}

dispatch_source_types! {
    /// Monitoring a timer.
    ///
    /// The handle is unused (pass zero for now). The mask is unused (pass zero
    /// for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_timer).
    timer = _dispatch_source_type_timer, "DISPATCH_SOURCE_TYPE_TIMER";

    /// Monitoring read operations on a file descriptor.
    ///
    /// The handle is a file descriptor (`int`). The mask is unused (pass zero
    /// for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_read).
    read = _dispatch_source_type_read, "DISPATCH_SOURCE_TYPE_READ";

    /// Monitoring write operations on a file descriptor.
    ///
    /// The handle is a file descriptor (`int`). The mask is unused (pass zero
    /// for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_write).
    write = _dispatch_source_type_write, "DISPATCH_SOURCE_TYPE_WRITE";

    /// Monitoring changes to a file system object.
    ///
    /// The handle is a file descriptor (`int`). The mask is a mask of desired
    /// events from
    /// [Dispatch Source Vnode Event Flags](https://developer.apple.com/documentation/dispatch/dispatch_source_vnode_flags_t).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_vnode).
    vnode = _dispatch_source_type_vnode, "DISPATCH_SOURCE_TYPE_VNODE";

    /// Monitoring signals.
    ///
    /// The handle is a signal number (`int`). The mask is unused (pass zero for
    /// now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_signal).
    signal = _dispatch_source_type_signal, "DISPATCH_SOURCE_TYPE_SIGNAL";

    /// Monitoring a process.
    ///
    /// The handle is a process identifier (`pid_t`). The mask is a mask of
    /// desired events from
    /// [Dispatch Source Process Event Flags](https://developer.apple.com/documentation/dispatch/dispatch_source_proc_flags_t).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_proc).
    proc = _dispatch_source_type_proc, "DISPATCH_SOURCE_TYPE_PROC";

    /// Monitoring memory pressure events.
    ///
    /// The handle is unused and you should pass 0 for that parameter. The mask
    /// is a mask of desired events from
    /// [Dispatch Source Memory Pressure Event Flags](https://developer.apple.com/documentation/dispatch/dispatch_source_memorypressure_flags_t).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_memorypressure).
    memory_pressure = _dispatch_source_type_memorypressure, "DISPATCH_SOURCE_TYPE_MEMORYPRESSURE";

    /// Monitoring a mach send port.
    ///
    /// The handle is a Mach port with a send or send-once right
    /// (`mach_port_t`). The mask is a mask of desired events from
    /// [Dispatch Source Mach Send Event Flags](https://developer.apple.com/documentation/dispatch/dispatch_source_mach_send_flags_t).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_mach_send).
    mach_send = _dispatch_source_type_mach_send, "DISPATCH_SOURCE_TYPE_MACH_SEND";

    /// Monitoring a mach receive port.
    ///
    /// The handle is a Mach port with a receive right (`mach_port_t`). The mask
    /// is unused (pass zero for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_mach_recv).
    mach_recv = _dispatch_source_type_mach_recv, "DISPATCH_SOURCE_TYPE_MACH_RECV";

    /// Monitoring custom events involving the coalescing of data with an AND
    /// operator.
    ///
    /// The handle is unused (pass zero for now). The mask is unused (pass zero
    /// for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_data_add).
    data_add = _dispatch_source_type_data_add, "DISPATCH_SOURCE_TYPE_DATA_ADD";

    /// Monitoring custom events involving the coalescing of data with an OR
    /// operator.
    ///
    /// The handle is unused (pass zero for now). The mask is unused (pass zero
    /// for now).
    ///
    /// See [documentation](https://developer.apple.com/documentation/dispatch/dispatch_source_type_data_or).
    data_or = _dispatch_source_type_data_or, "DISPATCH_SOURCE_TYPE_DATA_OR";
}
