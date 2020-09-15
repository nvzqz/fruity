use super::{CGAffineTransform, CGFloat, CGPoint, CGSize};
use std::mem;

/// The location and dimensions of a rectangle.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/CGRect).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq)]
pub struct CGRect {
    /// The coordinates of the rectangle's origin.
    pub origin: CGPoint,
    /// The height and width of the rectangle.
    pub size: CGSize,
}

/// Rectangle construction.
impl CGRect {
    /// A rectangle with zero origin and size.
    pub const ZERO: Self = Self::new(CGPoint::ZERO, CGSize::ZERO);

    /// The null rectangle, representing an invalid value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/cgrectnull).
    pub const NULL: Self = Self::new(
        CGPoint::new(CGFloat::INFINITY, CGFloat::INFINITY),
        CGSize::ZERO,
    );

    /// A rectangle that has infinite extent.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/cgrectinfinite).
    pub const INFINITE: Self = {
        let max = CGFloat::MAX;
        let min = CGFloat::MIN / 2.0;

        Self::from_floats(min, min, max, max)
    };

    /// Returns a point with the specified coordinates.
    ///
    /// This is equivalent to
    /// [`CGRectMake`](https://developer.apple.com/documentation/coregraphics/1455746-CGRectmake).
    #[inline]
    pub const fn new(origin: CGPoint, size: CGSize) -> Self {
        Self { origin, size }
    }

    /// Returns a rectangle with the given components.
    #[inline]
    pub const fn from_floats(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat) -> Self {
        Self::new(CGPoint::new(x, y), CGSize::new(width, height))
    }

    /// Returns a rectangle with the given components losslessly converted to
    /// [`CGFloat`](type.CGFloat.html)s.
    #[inline]
    pub const fn from_i16s(x: i16, y: i16, width: i16, height: i16) -> Self {
        Self::from_floats(x as _, y as _, width as _, height as _)
    }
}

/// Get and set rectangle properties.
impl CGRect {
    /// Returns the x-coordinate of `self`.
    #[inline]
    pub const fn x(&self) -> CGFloat {
        self.origin.x
    }

    /// Returns the y-coordinate of `self`.
    #[inline]
    pub const fn y(&self) -> CGFloat {
        self.origin.y
    }

    /// Returns the width of `self`.
    #[inline]
    pub const fn width(&self) -> CGFloat {
        self.size.width
    }

    /// Returns the height of `self`.
    #[inline]
    pub const fn height(&self) -> CGFloat {
        self.size.height
    }

    /// Returns `self` with an updated x-coordinate.
    #[inline]
    #[must_use]
    pub const fn with_x(mut self, x: CGFloat) -> Self {
        self.origin.x = x;
        self
    }

    /// Returns `self` with an updated y-coordinate.
    #[inline]
    #[must_use]
    pub const fn with_y(mut self, y: CGFloat) -> Self {
        self.origin.y = y;
        self
    }

    /// Returns `self` with an updated width.
    #[inline]
    #[must_use]
    pub const fn with_width(mut self, width: CGFloat) -> Self {
        self.size.width = width;
        self
    }

    /// Returns `self` with an updated height.
    #[inline]
    #[must_use]
    pub const fn with_height(mut self, height: CGFloat) -> Self {
        self.size.height = height;
        self
    }

    /// Updates the x-coordinate of `self` in-place.
    #[inline]
    pub fn set_x(&mut self, x: CGFloat) -> &mut Self {
        self.origin.x = x;
        self
    }

    /// Updates the y-coordinate of `self` in-place.
    #[inline]
    pub fn set_y(&mut self, y: CGFloat) -> &mut Self {
        self.origin.y = y;
        self
    }

    /// Updates the width of `self` in-place.
    #[inline]
    pub fn set_width(&mut self, width: CGFloat) -> &mut Self {
        self.size.width = width;
        self
    }

    /// Updates the height of `self` in-place.
    #[inline]
    pub fn set_height(&mut self, height: CGFloat) -> &mut Self {
        self.size.height = height;
        self
    }
}

/// Get rectangle bounds.
impl CGRect {
    /// Returns the smallest value for the x-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455948-cgrectgetminx).
    #[inline]
    pub fn min_x(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMinX(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMinX(*self) }
    }

    /// Returns the smallest value for the y-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454832-cgrectgetminy).
    #[inline]
    pub fn min_y(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMinY(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMinY(*self) }
    }

    /// Returns the center value for the x-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1456175-cgrectgetmidx).
    #[inline]
    pub fn mid_x(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMidX(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMidX(*self) }
    }

    /// Returns the center value for the y-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1456550-cgrectgetmidy).
    #[inline]
    pub fn mid_y(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMidY(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMidY(*self) }
    }

    /// Returns the largest value for the x-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454334-cgrectgetmaxx).
    #[inline]
    pub fn max_x(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMaxX(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMaxX(*self) }
    }

    /// Returns the largest value for the y-coordinate of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454060-cgrectgetmaxy).
    #[inline]
    pub fn max_y(&self) -> CGFloat {
        extern "C" {
            fn CGRectGetMaxY(rect: CGRect) -> CGFloat;
        }
        unsafe { CGRectGetMaxY(*self) }
    }
}

impl CGRect {
    /// Returns `true` if `self` has zero width or height, or is a null
    /// rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454917-cgrectisempty).
    #[inline]
    pub fn is_empty(&self) -> bool {
        extern "C" {
            fn CGRectIsEmpty(rect: CGRect) -> u8;
        }
        unsafe { CGRectIsEmpty(*self) != 0 }
    }

    /// Returns `true` if `self` is a null rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455471-cgrectisnull).
    #[inline]
    pub fn is_null(&self) -> bool {
        extern "C" {
            fn CGRectIsNull(rect: CGRect) -> u8;
        }
        unsafe { CGRectIsNull(*self) != 0 }
    }

    /// Returns `true` if `self` is infinite.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455008-cgrectisinfinite).
    #[inline]
    pub fn is_infinite(&self) -> bool {
        extern "C" {
            fn CGRectIsInfinite(rect: CGRect) -> u8;
        }
        unsafe { CGRectIsInfinite(*self) != 0 }
    }

    /// Returns `true` if `self` contains `point`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1456316-cgrectcontainspoint).
    #[inline]
    pub fn contains_point(&self, point: CGPoint) -> bool {
        extern "C" {
            fn CGRectContainsPoint(rect: CGRect, point: CGPoint) -> u8;
        }
        unsafe { CGRectContainsPoint(*self, point) != 0 }
    }

    /// Returns `true` if `self` contains `other`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454186-cgrectcontainsrect).
    #[inline]
    pub fn contains_rect(&self, other: &Self) -> bool {
        extern "C" {
            fn CGRectContainsRect(r1: CGRect, r2: CGRect) -> u8;
        }
        unsafe { CGRectContainsRect(*self, *other) != 0 }
    }

    /// Returns `true` if `self` intersects `other`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454747-cgrectintersectsrect).
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        extern "C" {
            fn CGRectIntersectsRect(r1: CGRect, r2: CGRect) -> u8;
        }
        unsafe { CGRectIntersectsRect(*self, *other) != 0 }
    }

    /// Returns `self` with a positive width and height.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1456432-cgrectstandardize).
    #[inline]
    pub fn standardize(self) -> Self {
        extern "C" {
            fn CGRectStandardize(rect: CGRect) -> CGRect;
        }
        unsafe { CGRectStandardize(self) }
    }

    /// Returns the smallest rectangle that results from converting the values
    /// of `self` to integers.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1456348-cgrectintegral).
    #[inline]
    pub fn integral(self) -> Self {
        extern "C" {
            fn CGRectIntegral(rect: CGRect) -> CGRect;
        }
        unsafe { CGRectIntegral(self) }
    }

    /// Returns the result of applying an affine transformation to `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455875-cgrectapplyaffinetransform).
    #[inline]
    pub fn apply(self, transform: CGAffineTransform) -> Self {
        extern "C" {
            fn CGRectApplyAffineTransform(rect: CGRect, transform: CGAffineTransform) -> CGRect;
        }
        unsafe { CGRectApplyAffineTransform(self, transform) }
    }

    /// Returns `self` with its origin offset.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454841-cgrectoffset).
    #[inline]
    pub fn offset(self, dx: CGFloat, dy: CGFloat) -> Self {
        extern "C" {
            fn CGRectOffset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
        }
        unsafe { CGRectOffset(self, dx, dy) }
    }

    /// Returns a rectangle that is smaller or larger than `self`, with the same
    /// center point.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454218-cgrectinset).
    #[inline]
    pub fn inset(self, dx: CGFloat, dy: CGFloat) -> Self {
        extern "C" {
            fn CGRectInset(rect: CGRect, dx: CGFloat, dy: CGFloat) -> CGRect;
        }
        unsafe { CGRectInset(self, dx, dy) }
    }

    /// Returns two rectangles by dividing `self`.
    ///
    /// Together `edge` and `amount` define a line (parallel to the specified
    /// edge of the rectangle and at the specified distance from that edge) that
    /// divides the rectangle into two component rectangles.
    ///
    /// The returned tuple consists of:
    ///
    /// - `slice`: The component rectangle nearest the edge of the original
    ///   rectangle specified by `edge`, with width equal to `amount`.
    ///
    /// - `remainder`: The component rectangle equal to the remaining area of
    ///   the original rectangle not included in the `slice` rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455925-cgrectdivide).
    #[inline]
    pub fn divide(self, amount: CGFloat, edge: CGRectEdge) -> (Self, Self) {
        extern "C" {
            fn CGRectDivide(
                rect: CGRect,
                slice: *mut CGRect,
                remainder: *mut CGRect,
                amount: CGFloat,
                edge: CGRectEdge,
            );
        }

        let mut slice = mem::MaybeUninit::uninit();
        let mut remainder = mem::MaybeUninit::uninit();

        unsafe {
            CGRectDivide(
                self,
                slice.as_mut_ptr(),
                remainder.as_mut_ptr(),
                amount,
                edge,
            );
            (slice.assume_init(), remainder.assume_init())
        }
    }

    /// Returns the smallest rectangle that contains `self` and `other`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455837-cgrectunion).
    #[inline]
    pub fn union(self, other: Self) -> Self {
        extern "C" {
            fn CGRectUnion(r1: CGRect, r2: CGRect) -> CGRect;
        }
        unsafe { CGRectUnion(self, other) }
    }

    /// Returns the intersection of `self` and `other`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455346-cgrectintersection).
    #[inline]
    pub fn intersection(self, other: Self) -> Self {
        extern "C" {
            fn CGRectIntersection(r1: CGRect, r2: CGRect) -> CGRect;
        }
        unsafe { CGRectIntersection(self, other) }
    }
}

/// Coordinates that establish the edges of a rectangle.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/cgrectedge).
#[repr(u32)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum CGRectEdge {
    /// The minimum value for the x-coordinate of the rectangle.
    ///
    /// In macOS and iOS with the default coordinate system this is the left
    /// edge of the rectangle.
    MinX,

    /// The minimum value for the y-coordinate of the rectangle.
    ///
    /// In macOS with the default coordinate system this is the bottom edge of
    /// the rectangle. In iOS with the default coordinate system this is the top
    /// edge of the rectangle.
    MinY,

    /// The maximum value for the x-coordinate of the rectangle.
    ///
    /// In macOS and iOS with the default coordinate system this is the right
    /// edge of the rectangle.
    MaxX,

    /// The maximum value for the y-coordinate of the rectangle.
    ///
    /// In macOS with the default coordinate system this is the top edge of the
    /// rectangle. In iOS with the default coordinate system this is the bottom
    /// edge of the rectangle.
    MaxY,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let expected = unsafe {
            extern "C" {
                static CGRectZero: CGRect;
            }
            CGRectZero
        };
        assert_eq!(CGRect::ZERO, expected);
    }

    #[test]
    fn null() {
        let expected = unsafe {
            extern "C" {
                static CGRectNull: CGRect;
            }
            CGRectNull
        };
        assert_eq!(CGRect::NULL, expected);
    }

    #[test]
    fn infinite() {
        let expected = unsafe {
            extern "C" {
                static CGRectInfinite: CGRect;
            }
            CGRectInfinite
        };
        assert_eq!(CGRect::INFINITE, expected);
    }
}
