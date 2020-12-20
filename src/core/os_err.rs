use std::num::NonZeroI16;

/// A non-zero 16-bit error code.
///
/// This is the old counterpart to [`OSStatus`](super::OSStatus).
///
/// # Usage
///
/// In FFI code, this type is meant to be used as [`Option<OSErr>`](Option).
/// [`None`] becomes 0 (no error) because this type is
/// [`#[repr(transparent)]`]((https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent))
/// over [`NonZeroI16`].
///
/// Rust bindings that call `Option<OSErr>`-returning functions should return
/// [`Result<T, OSErr>`](Result).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OSErr(NonZeroI16);

impl From<NonZeroI16> for OSErr {
    #[inline]
    fn from(value: NonZeroI16) -> Self {
        Self(value)
    }
}

impl OSErr {
    /// Creates an instance from `value`, returning `None` if it is zero.
    #[inline]
    pub const fn new(value: i16) -> Option<Self> {
        match NonZeroI16::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates an instance from a non-zero `value`.
    #[inline]
    pub const fn new_non_zero(value: NonZeroI16) -> Self {
        Self(value)
    }

    /// Creates an instance from `value`, without checking if it is zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[inline]
    pub const unsafe fn new_unchecked(value: i16) -> Self {
        Self(NonZeroI16::new_unchecked(value))
    }

    /// Returns this error's integer value.
    #[inline]
    pub const fn value(self) -> i16 {
        self.0.get()
    }

    /// Returns this error's integer value.
    #[inline]
    pub const fn non_zero_value(self) -> NonZeroI16 {
        self.0
    }
}
