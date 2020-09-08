use std::os::raw::c_char;

/// The boolean value type.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/bool).
pub type BOOL = c_char;

/// The [`BOOL`](type.BOOL.html) equivalent to
/// [`false`](https://doc.rust-lang.org/std/keyword.false.html).
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/no).
pub const NO: BOOL = 0;

/// The [`BOOL`](type.BOOL.html) equivalent to
/// [`true`](https://doc.rust-lang.org/std/keyword.true.html).
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/yes).
pub const YES: BOOL = 1;
