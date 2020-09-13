use crate::core_graphics::{CGPoint, CGRect, CGRectEdge, CGSize};

/// A point in a Cartesian coordinate system.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nspoint).
pub type NSPoint = CGPoint;

/// A two-dimensional size.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nssize).
pub type NSSize = CGSize;

/// A rectangle.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsrect).
pub type NSRect = CGRect;

/// Coordinates that establish the edges of a rectangle.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsrectedge).
pub type NSRectEdge = CGRectEdge;
