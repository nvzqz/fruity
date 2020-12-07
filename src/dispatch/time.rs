/// A point in time relative to the default clock, with nanosecond precision.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchtime) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_time_t)
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DispatchTime(pub u64);

// TODO: `Add<Duration>` and `Sub<Duration>`

const NANOS_PER_SEC: u64 = 1_000_000_000;
const NANOS_PER_MILLI: u64 = 1_000_000;
const NANOS_PER_MICRO: u64 = 1_000;

impl DispatchTime {
    /// A time that occurs immediately.
    ///
    /// This is equivalent to
    /// [`DISPATCH_TIME_NOW`](https://developer.apple.com/documentation/dispatch/dispatch_time_now).
    ///
    /// Use this value to schedule work as soon as possible.
    pub const IMMEDIATE: DispatchTime = DispatchTime(0);

    /// A time in the distant future.
    ///
    /// This is equivalent to
    /// [`DISPATCH_TIME_FOREVER`](https://developer.apple.com/documentation/dispatch/dispatch_time_now)
    /// and
    /// [`DispatchTime.distantFuture`](https://developer.apple.com/documentation/dispatch/dispatchtime/1780795-distantfuture).
    ///
    /// You can pass this value to methods that schedule work to have the system
    /// wait indefinitely for a particular event to occur or condition to be
    /// met.
    pub const DISTANT_FUTURE: DispatchTime = DispatchTime(!0);

    /// Returns the current time.
    ///
    /// This is equivalent to
    /// [`DispatchTime.now()`](https://developer.apple.com/documentation/dispatch/dispatchtime/1780853-now).
    #[inline]
    pub fn now() -> DispatchTime {
        DispatchTime::IMMEDIATE.offset_nanos(0)
    }

    /// Returns a time that's a specified number of seconds from the current
    /// time.
    #[inline]
    pub fn from_secs_from_now(secs: u64) -> Self {
        Self::from_nanos_from_now(secs.saturating_mul(NANOS_PER_SEC))
    }

    /// Returns a time that's a specified number of milliseconds from the
    /// current time.
    #[inline]
    pub fn from_millis_from_now(millis: u64) -> Self {
        Self::from_nanos_from_now(millis.saturating_mul(NANOS_PER_MILLI))
    }

    /// Returns a time that's a specified number of microseconds from the
    /// current time.
    #[inline]
    pub fn from_micros_from_now(micros: u64) -> Self {
        Self::from_nanos_from_now(micros.saturating_mul(NANOS_PER_MICRO))
    }

    /// Returns a time that's a specified number of nanoseconds from the current
    /// time.
    #[inline]
    pub fn from_nanos_from_now(nanos: u64) -> Self {
        let (delta, rem) = match nanos.checked_sub(i64::MAX as u64) {
            Some(rem) => (i64::MAX, rem as i64),
            None => (nanos as i64, 0),
        };

        let value = Self::IMMEDIATE.offset_nanos(delta);
        if rem == 0 {
            value
        } else {
            // TODO: Use info based on `mach_timebase_info` to do arithmetic.
            value.offset_nanos(rem)
        }
    }

    // TODO: dispatch_walltime

    /// Returns `self` offset by `delta` nanoseconds.
    ///
    /// This is equivalent to
    /// [`dispatch_time`](https://developer.apple.com/documentation/dispatch/1420519-dispatch_time).
    #[inline]
    #[doc(alias = "dispatch_time")]
    pub fn offset_nanos(self, delta: i64) -> Self {
        extern "C" {
            fn dispatch_time(when: DispatchTime, delta: i64) -> DispatchTime;
        }
        unsafe { dispatch_time(self, delta) }
    }
}
