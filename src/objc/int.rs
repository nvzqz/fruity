#![allow(non_upper_case_globals)]

/// Describes an integer.
///
/// When building 32-bit applications, NSInteger is a 32-bit integer. A 64-bit
/// application treats NSInteger as a 64-bit integer.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsinteger).
pub type NSInteger = isize;

/// Describes an unsigned integer.
///
/// When building 32-bit applications, NSUInteger is a 32-bit unsigned integer.
/// A 64-bit application treats NSUInteger as a 64-bit unsigned integer
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsuinteger).
pub type NSUInteger = usize;

/// The maximum [`NSInteger`](type.NSInteger.html) value.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsintegermax).
pub const NSIntegerMax: NSInteger = NSInteger::MAX;

/// The minimum [`NSInteger`](type.NSInteger.html) value.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsintegermin).
pub const NSIntegerMin: NSInteger = NSInteger::MIN;

/// The maximum [`NSUInteger`](type.NSUInteger.html) value.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsuintegermax).
pub const NSUIntegerMax: NSUInteger = NSUInteger::MAX;

/// The minimum [`NSUInteger`](type.NSUInteger.html) value.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/nsuintegermin).
pub const NSUIntegerMin: NSUInteger = NSUInteger::MIN;
