use super::NSString;
use crate::core::Arc;
use crate::objc::NSUInteger;
use std::ops::Range;

/// Describe a portion of a series, such as characters in a string or objects in
/// an array.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NSRange {
    /// The start index (0 is the first, as in C arrays).
    ///
    /// For type compatibility with the rest of the system, `LONG_MAX` is the
    /// maximum value you should use for location.
    pub location: NSUInteger,
    /// The number of items in the range (can be 0).
    ///
    /// For type compatibility with the rest of the system, `LONG_MAX` is the
    /// maximum value you should use for length.
    pub length: NSUInteger,
}

impl From<Range<NSUInteger>> for NSRange {
    #[inline]
    fn from(range: Range<NSUInteger>) -> Self {
        Self::from_range(range)
    }
}

impl NSRange {
    /// Returns a range with the specified values.
    ///
    /// This is equivalent to
    /// [`NSMakeRange`](https://developer.apple.com/documentation/foundation/1417188-nsmakerange).
    #[inline]
    pub const fn new(location: NSUInteger, length: NSUInteger) -> Self {
        Self { location, length }
    }

    /// Converts from a native Rust range.
    ///
    /// This takes into account empty ranges with an `end` less than `start`.
    #[inline]
    pub fn from_range(range: Range<NSUInteger>) -> Self {
        let location = range.start;
        let length = range.end.saturating_sub(location);
        Self::new(location, length)
    }

    /// Returns a range from a textual representation.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1408420-nsrangefromstring).
    #[inline]
    #[doc(alias = "NSRangeFromString")]
    pub fn from_nsstring(string: &NSString) -> Self {
        extern "C" {
            fn NSRangeFromString(string: &NSString) -> NSRange;
        }
        unsafe { NSRangeFromString(string) }
    }

    /// Returns `true` if `self` has zero length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns `true` if `location` is in `self`.
    ///
    /// This is equivalent to
    /// [`NSLocationInRange`](https://developer.apple.com/documentation/foundation/1416238-nslocationinrange).
    #[inline]
    pub fn contains(&self, location: NSUInteger) -> bool {
        if location < self.location {
            false
        } else {
            location - self.location < self.length
        }
    }

    /// Returns the sum of the location and length of the range.
    ///
    /// The result wraps on overflow.
    ///
    /// This is equivalent to
    /// [`NSMaxRange`](https://developer.apple.com/documentation/foundation/1407494-nsmaxrange).
    #[inline]
    pub fn end(&self) -> NSUInteger {
        self.location.wrapping_add(self.length)
    }

    /// Returns a string representation of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1415155-nsstringfromrange).
    #[inline]
    pub fn to_nsstring(self) -> Arc<NSString<'static>> {
        self.into()
    }

    /// Returns a range covering all indices in and between `self` and `other`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1412317-nsunionrange).
    #[inline]
    #[doc(alias = "NSUnionRange")]
    pub fn union(self, other: Self) -> Self {
        extern "C" {
            fn NSUnionRange(r1: NSRange, r2: NSRange) -> NSRange;
        }
        unsafe { NSUnionRange(self, other) }
    }

    /// Returns a range covering all indices in and between `self` and `other`.
    ///
    /// If the returned range’s `length` is 0, then the two ranges don’t
    /// intersect, and the value of `location` is undefined.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1413065-nsintersectionrange).
    #[inline]
    #[doc(alias = "NSIntersectionRange")]
    pub fn intersection(self, other: Self) -> Self {
        extern "C" {
            fn NSIntersectionRange(r1: NSRange, r2: NSRange) -> NSRange;
        }
        unsafe { NSIntersectionRange(self, other) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_range() {
        let cases: &[(Range<NSUInteger>, NSRange)] = &[
            (0..0, NSRange::new(0, 0)),
            (0..10, NSRange::new(0, 10)),
            (10..0, NSRange::new(10, 0)),
            (10..10, NSRange::new(10, 0)),
            (10..20, NSRange::new(10, 10)),
        ];

        for (range, expected) in cases {
            assert_eq!(NSRange::from(range.clone()), *expected);
        }
    }
}
