use crate::{core_graphics::CGFloat, foundation::NSEdgeInsets};

/// Edge insets that take language direction into account.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/uikit/nsdirectionaledgeinsets?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/uikit/nsdirectionaledgeinsets?language=objc)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct NSDirectionalEdgeInsets {
    /// The top edge inset value.
    pub top: CGFloat,

    /// The leading edge inset value.
    pub leading: CGFloat,

    /// The bottom edge inset value.
    pub bottom: CGFloat,

    /// The trailing edge inset value.
    pub trailing: CGFloat,
}

impl From<NSEdgeInsets> for NSDirectionalEdgeInsets {
    #[inline]
    fn from(insets: NSEdgeInsets) -> Self {
        Self::from_edge_insets(insets)
    }
}

impl NSDirectionalEdgeInsets {
    /// An instance whose insets are all set to 0.
    pub const ZERO: Self = Self::all(0.0);

    /// Returns an instance with the specified insets.
    ///
    /// This is equivalent to
    /// [`NSDirectionalEdgeInsetsMake`](https://developer.apple.com/documentation/uikit/2867596-nsdirectionaledgeinsetsmake?language=objc).
    #[inline]
    #[doc(alias = "NSDirectionalEdgeInsetsMake")]
    pub const fn new(top: CGFloat, leading: CGFloat, bottom: CGFloat, trailing: CGFloat) -> Self {
        Self {
            top,
            leading,
            bottom,
            trailing,
        }
    }

    /// Returns an instance with all insets set to the same value.
    #[inline]
    pub const fn all(inset: CGFloat) -> Self {
        Self::new(inset, inset, inset, inset)
    }

    /// Returns an instance from normal insets, assuming left-to-right.
    #[inline]
    pub const fn from_edge_insets(insets: NSEdgeInsets) -> Self {
        Self {
            top: insets.top,
            leading: insets.left,
            bottom: insets.bottom,
            trailing: insets.right,
        }
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
            && self.leading.is_finite()
            && self.bottom.is_finite()
            && self.trailing.is_finite()
    }
}
