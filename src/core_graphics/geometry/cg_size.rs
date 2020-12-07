use super::{CGAffineTransform, CGFloat};

/// Width and height values.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/cgsize).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct CGSize {
    /// A height value.
    pub height: CGFloat,
    /// A width value.
    pub width: CGFloat,
}

impl From<(CGFloat, CGFloat)> for CGSize {
    #[inline]
    fn from((height, width): (CGFloat, CGFloat)) -> Self {
        CGSize::new(height, width)
    }
}

impl CGSize {
    /// A size with a width and height of 0.
    pub const ZERO: Self = Self::new(0.0, 0.0);

    /// Returns a size with the specified dimension values.
    ///
    /// This is equivalent to
    /// [`CGSizeMake`](https://developer.apple.com/documentation/coregraphics/1455082-cgsizemake).
    #[inline]
    pub const fn new(height: CGFloat, width: CGFloat) -> Self {
        Self { height, width }
    }

    /// Returns a size with the given components losslessly converted to
    /// [`CGFloat`](type.CGFloat.html)s.
    #[inline]
    pub const fn from_i16s(height: i16, width: i16) -> Self {
        Self::new(height as _, width as _)
    }

    /// Returns the result of applying an affine transformation to `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454806-cgsizeapplyaffinetransform).
    #[inline]
    #[doc(alias = "CGSizeApplyAffineTransform")]
    pub fn apply(self, transform: CGAffineTransform) -> Self {
        extern "C" {
            fn CGSizeApplyAffineTransform(size: CGSize, transform: CGAffineTransform) -> CGSize;
        }
        unsafe { CGSizeApplyAffineTransform(self, transform) }
    }
}
