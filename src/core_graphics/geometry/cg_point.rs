use super::{CGAffineTransform, CGFloat};

/// A point in a two-dimensional coordinate system.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/cgpoint).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct CGPoint {
    /// The x-coordinate of the point.
    pub x: CGFloat,
    /// The y-coordinate of the point.
    pub y: CGFloat,
}

impl From<(CGFloat, CGFloat)> for CGPoint {
    #[inline]
    fn from((x, y): (CGFloat, CGFloat)) -> Self {
        Self::new(x, y)
    }
}

impl CGPoint {
    /// A point at location `(0, 0)`.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Returns a point with the specified coordinates.
    ///
    /// This is equivalent to
    /// [`CGPointMake`](https://developer.apple.com/documentation/coregraphics/1455746-cgpointmake).
    #[inline]
    #[doc(alias = "CGPointMake")]
    pub const fn new(x: CGFloat, y: CGFloat) -> Self {
        Self { x, y }
    }

    /// Returns a point with the given components losslessly converted to
    /// [`CGFloat`](type.CGFloat.html)s.
    #[inline]
    pub const fn from_i16s(x: i16, y: i16) -> Self {
        Self::new(x as _, y as _)
    }

    /// Returns the result of applying an affine transformation to `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454251-cgpointapplyaffinetransform).
    #[inline]
    #[doc(alias = "CGPointApplyAffineTransform")]
    pub fn apply(self, transform: CGAffineTransform) -> Self {
        extern "C" {
            // Looking at the disassembly, it appears that this operation is
            // simple enough to implement inline.
            fn CGPointApplyAffineTransform(point: CGPoint, transform: CGAffineTransform)
                -> CGPoint;
        }
        unsafe { CGPointApplyAffineTransform(self, transform) }
    }
}
