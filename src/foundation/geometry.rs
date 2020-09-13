use crate::core_graphics::{CGFloat, CGPoint, CGRect, CGRectEdge, CGSize};

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

/// A description of the distance between the edges of two rectangles.
///
/// Edge insets describe the distance between the edges of one rectangle to a
/// related rectangle that can be described by measuring a constant but
/// edge-specific distance from each edge.
///
/// A common use for this structure is to describe the relationship between a
/// view’s frame and its alignment rectangle.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsedgeinsets).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct NSEdgeInsets {
    /// The distance from the top of the source rectangle to the top of the
    /// result rectangle.
    pub top: CGFloat,
    /// The distance from the left side of the source rectangle to the left side
    /// of the result rectangle.
    pub left: CGFloat,
    /// The distance from the bottom of the source rectangle to the bottom of
    /// the result rectangle.
    pub bottom: CGFloat,
    /// The distance from the right side of the source rectangle to the right
    /// side of the result rectangle.
    pub right: CGFloat,
}

impl From<(CGFloat, CGFloat, CGFloat, CGFloat)> for NSEdgeInsets {
    #[inline]
    fn from((top, left, bottom, right): (CGFloat, CGFloat, CGFloat, CGFloat)) -> Self {
        Self::new(top, left, bottom, right)
    }
}

impl NSEdgeInsets {
    /// No insets; the rectangles are the same.
    pub const ZERO: Self = Self::all(0.0);

    /// Returns an instance with the specified insets.
    ///
    /// This is equivalent to
    /// [`NSEdgeInsetsMake`](https://developer.apple.com/documentation/foundation/1391130-nsedgeinsetsmake).
    #[inline]
    pub const fn new(top: CGFloat, left: CGFloat, bottom: CGFloat, right: CGFloat) -> Self {
        Self {
            top,
            left,
            bottom,
            right,
        }
    }

    /// Returns an instance with all insets set to the same value.
    #[inline]
    pub const fn all(inset: CGFloat) -> Self {
        Self::new(inset, inset, inset, inset)
    }

    /// Returns `true` if `self` has no insets.
    #[inline]
    pub fn is_zero(&self) -> bool {
        *self == Self::ZERO
    }

    /// Returns `true` if the insets of `self` are neither infinite nor NaN.
    #[inline]
    pub fn is_finite(&self) -> bool {
        self.top.is_finite()
            && self.left.is_finite()
            && self.bottom.is_finite()
            && self.right.is_finite()
    }
}
