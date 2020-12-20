use super::OSErr;
use std::num::NonZeroI32;

/// A non-zero 32-bit error code.
///
/// This is the new counterpart to [`OSErr`](super::OSErr).
///
/// # Usage
///
/// In FFI code, this type is meant to be used as [`Option<OSStatus>`](Option).
/// [`None`] becomes 0 (no error) because this type is
/// [`#[repr(transparent)]`]((https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent))
/// over [`NonZeroI32`].
///
/// Rust bindings that call `Option<OSStatus>`-returning functions should return
/// [`Result<T, OSStatus>`](Result).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OSStatus(NonZeroI32);

impl From<OSErr> for OSStatus {
    #[inline]
    fn from(error: OSErr) -> Self {
        let value = error.value() as i32;

        // SAFETY: `OSErr` can never have a zero value.
        unsafe { Self::new_unchecked(value) }
    }
}

impl From<NonZeroI32> for OSStatus {
    #[inline]
    fn from(value: NonZeroI32) -> Self {
        Self(value)
    }
}

impl OSStatus {
    /// Creates an instance from `value`, returning `None` if it is zero.
    #[inline]
    pub const fn new(value: i32) -> Option<Self> {
        match NonZeroI32::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates an instance from a non-zero `value`.
    #[inline]
    pub const fn new_non_zero(value: NonZeroI32) -> Self {
        Self(value)
    }

    /// Creates an instance from `value`, without checking if it is zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[inline]
    pub const unsafe fn new_unchecked(value: i32) -> Self {
        Self(NonZeroI32::new_unchecked(value))
    }

    /// Returns this error's integer value.
    #[inline]
    pub const fn value(self) -> i32 {
        self.0.get()
    }

    /// Returns this error's non-zero integer value.
    #[inline]
    pub const fn non_zero_value(self) -> NonZeroI32 {
        self.0
    }
}
