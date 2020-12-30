use std::fmt;

// TODO: Expose raw type publicly and move relevant C functions to `sys` module.
#[allow(non_camel_case_types)]
pub(crate) type dispatch_queue_attr_t = *mut dispatch_queue_attr_s;

#[repr(C)]
#[allow(non_camel_case_types)]
pub(crate) struct dispatch_queue_attr_s {
    _priv: [u8; 0],
}

/// Attributes that define the behavior of a
/// [`DispatchQueue`](crate::dispatch::DispatchQueue).
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/attributes) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_queue_attr_t)
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DispatchQueueAttributes(
    // This type is meant to be compatible with Swift's
    // `DispatchQueue.Attributes`.
    u64,
);

impl fmt::Debug for DispatchQueueAttributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DispatchQueueAttributes")
            .field("is_concurrent", &self.is_concurrent())
            .field("is_initially_inactive", &self.is_initially_inactive())
            .finish()
    }
}

const SHIFT_CONCURRENT: u64 = 1;
const SHIFT_INACTIVE: u64 = 2;

const FLAG_CONCURRENT: u64 = 1 << SHIFT_CONCURRENT;
const FLAG_INACTIVE: u64 = 1 << SHIFT_INACTIVE;

impl DispatchQueueAttributes {
    /// Create a dispatch queue that invokes blocks serially in FIFO order.
    pub const SERIAL: Self = Self(0);

    /// Create a dispatch queue that invokes blocks serially in FIFO order, and
    /// that is initially inactive.
    pub const SERIAL_INACTIVE: Self = Self::SERIAL.with_initially_inactive(true);

    /// Create a dispatch queue that may invoke blocks concurrently and supports
    /// barrier blocks submitted with the dispatch barrier API.
    pub const CONCURRENT: Self = Self(FLAG_CONCURRENT);

    /// Create a dispatch queue that may invoke blocks concurrently and supports
    /// barrier blocks submitted with the dispatch barrier API, and that is
    /// initially inactive.
    pub const CONCURRENT_INACTIVE: Self = Self::CONCURRENT.with_initially_inactive(true);

    /// Returns `true` if `self` creates a dispatch queue that may invoke blocks
    /// concurrently and supports barrier blocks submitted with the dispatch
    /// barrier API.
    #[inline]
    pub const fn is_concurrent(&self) -> bool {
        self.0 & FLAG_CONCURRENT != 0
    }

    /// Reassigns the value of [`is_concurrent`](Self::is_concurrent).
    #[inline]
    pub const fn with_concurrent(self, yes: bool) -> Self {
        Self((self.0 & !FLAG_CONCURRENT) | ((yes as u64) << SHIFT_CONCURRENT))
    }

    /// Returns `true` if `self` creates a dispatch queue that is initially
    /// inactive.
    #[inline]
    pub const fn is_initially_inactive(&self) -> bool {
        self.0 & FLAG_INACTIVE != 0
    }

    /// Reassigns the value of
    /// [`is_initially_inactive`](Self::is_initially_inactive).
    #[inline]
    pub const fn with_initially_inactive(self, yes: bool) -> Self {
        Self((self.0 & !FLAG_INACTIVE) | ((yes as u64) << SHIFT_INACTIVE))
    }

    #[inline]
    pub(crate) fn _to_raw(&self) -> dispatch_queue_attr_t {
        const DISPATCH_QUEUE_SERIAL: dispatch_queue_attr_t = 0 as _;

        extern "C" {
            static mut _dispatch_queue_attr_concurrent: dispatch_queue_attr_s;

            fn dispatch_queue_attr_make_initially_inactive(
                attr: dispatch_queue_attr_t,
            ) -> dispatch_queue_attr_t;
        }

        let mut attr = DISPATCH_QUEUE_SERIAL;

        if self.is_concurrent() {
            attr = unsafe { &mut _dispatch_queue_attr_concurrent };
        }

        // available(macOS 10.12, iOS 10.0, tvOS 10.0, watchOS 3.0)
        //
        // TODO: Handle availability.
        if self.is_initially_inactive() {
            attr = unsafe { dispatch_queue_attr_make_initially_inactive(attr) };
        }

        attr
    }
}
