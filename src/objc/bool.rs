use std::{fmt, os::raw::c_schar};

/// The boolean value type.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/bool).
///
/// # Memory Representation
///
/// On Intel OS X and 32-bit iOS, Objective-C's `BOOL` type is a `signed char`.
/// Everywhere else it is C/C++'s `Bool`.
///
/// As a result, this type is a `#[repr(transparent)]` wrapper around
/// [`c_schar`] (a.k.a. [`i8`]) or [`bool`] respectively.
///
/// This crate uses a [newtype] instead of a [type alias] because aliasing
/// [`bool`] would allow for `if x` on some platforms but not others. When using
/// this type in a conditional statement, use [`is_yes`](Self::is_yes).
///
/// [newtype]: https://doc.rust-lang.org/rust-by-example/generics/new_types.html
/// [type alias]: https://doc.rust-lang.org/rust-by-example/types/alias.html
#[repr(transparent)]
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BOOL {
    #[cfg(any(
        all(any(target_os = "macos", mac_catalyst), target_arch = "x86_64"),
        all(target_os = "ios", target_pointer_width = "32"),
    ))]
    value: c_schar,

    #[cfg(not(any(
        all(any(target_os = "macos", mac_catalyst), target_arch = "x86_64"),
        all(target_os = "ios", target_pointer_width = "32"),
    )))]
    value: bool,
}

impl From<bool> for BOOL {
    #[inline]
    fn from(b: bool) -> BOOL {
        BOOL::new(b)
    }
}

impl From<BOOL> for bool {
    #[inline]
    fn from(b: BOOL) -> bool {
        b.is_yes()
    }
}

impl fmt::Debug for BOOL {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(if self.is_yes() { "YES" } else { "NO" })
    }
}

impl BOOL {
    // Keywords `false` and `true` link to `bool` docs instead of keyword docs.
    // So they are explicitly linked.

    /// The [`BOOL`](struct.BOOL.html) equivalent to
    /// [`false`](https://doc.rust-lang.org/std/keyword.false.html).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/no).
    pub const NO: Self = Self::new(false);

    /// The [`BOOL`](struct.BOOL.html) equivalent to
    /// [`true`](https://doc.rust-lang.org/std/keyword.true.html).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/yes).
    pub const YES: Self = Self::new(true);

    /// Creates an Objective-C boolean from a Rust boolean.
    #[inline]
    pub const fn new(value: bool) -> Self {
        Self { value: value as _ }
    }

    /// Returns `true` if `self` should be treated like
    /// [`NO`](Self::NO).
    #[inline]
    pub const fn is_no(self) -> bool {
        self.value as c_schar == 0
    }

    /// Returns `true` if `self` should be treated like
    /// [`YES`](Self::YES).
    #[inline]
    pub const fn is_yes(self) -> bool {
        self.value as c_schar != 0
    }
}

/// The [`BOOL`](struct.BOOL.html) equivalent to
/// [`false`](https://doc.rust-lang.org/std/keyword.false.html).
///
/// Use [`BOOL::NO`] if an associated constant is preferred.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/no).
pub const NO: BOOL = BOOL::NO;

/// The [`BOOL`](struct.BOOL.html) equivalent to
/// [`true`](https://doc.rust-lang.org/std/keyword.true.html).
///
/// Use [`BOOL::YES`] if an associated constant is preferred.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/yes).
pub const YES: BOOL = BOOL::YES;
