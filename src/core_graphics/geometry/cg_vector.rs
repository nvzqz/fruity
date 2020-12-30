use super::CGFloat;

/// A two-dimensional vector.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/cgvector).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct CGVector {
    /// The x component.
    pub dx: CGFloat,
    /// The y component.
    pub dy: CGFloat,
}

impl From<(CGFloat, CGFloat)> for CGVector {
    #[inline]
    fn from((dx, dy): (CGFloat, CGFloat)) -> Self {
        Self::new(dx, dy)
    }
}

impl CGVector {
    /// A vector with zero change.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Returns a vector with the specified components.
    ///
    /// This is equivalent to
    /// [`CGPointMake`](https://developer.apple.com/documentation/coregraphics/1454811-cgvectormake).
    #[inline]
    pub const fn new(dx: CGFloat, dy: CGFloat) -> Self {
        Self { dx, dy }
    }

    /// Returns a point with the given components losslessly converted to
    /// [`CGFloat`]s.
    #[inline]
    pub const fn from_i16s(dx: i16, dy: i16) -> Self {
        Self::new(dx as _, dy as _)
    }
}
