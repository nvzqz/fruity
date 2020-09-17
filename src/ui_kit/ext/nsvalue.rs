use crate::{
    core_graphics::{CGAffineTransform, CGPoint, CGRect, CGSize, CGVector},
    foundation::NSValue,
};

/// Core Graphics geometry values.
///
/// Requires the **`ui_kit`** feature flag.
impl NSValue {
    /// Creates a new value object containing the specified point.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624531-valuewithcgpoint).
    #[inline]
    pub fn value_with_cg_point(value: CGPoint) -> Self {
        unsafe { _msg_send![Self::class(), valueWithCGPoint: value] }
    }

    /// Returns the value as a `CGPoint`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624534-cgpointvalue).
    #[inline]
    pub fn cg_point_value(&self) -> CGPoint {
        unsafe { _msg_send![self, CGPointValue] }
    }

    /// Creates a new value object containing the specified size.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624511-valuewithcgsize).
    #[inline]
    pub fn value_with_cg_size(value: CGSize) -> Self {
        unsafe { _msg_send![Self::class(), valueWithCGSize: value] }
    }

    /// Returns the value as a `CGSize`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624489-cgsizevalue).
    #[inline]
    pub fn cg_size_value(&self) -> CGSize {
        unsafe { _msg_send![self, CGSizeValue] }
    }

    /// Creates a new value object containing the specified rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624529-valuewithcgrect).
    #[inline]
    pub fn value_with_cg_rect(value: CGRect) -> Self {
        unsafe { _msg_send![Self::class(), valueWithCGRect: value] }
    }

    /// Returns the value as a `CGRect`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreimage/civector/1438108-cgrectvalue).
    #[inline]
    pub fn cg_rect_value(&self) -> CGRect {
        unsafe { _msg_send![self, CGRectValue] }
    }

    /// Creates a new value object containing the specified vector.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624493-valuewithcgvector).
    #[inline]
    pub fn value_with_cg_vector(value: CGVector) -> Self {
        unsafe { _msg_send![Self::class(), valueWithCGVector: value] }
    }

    /// Returns the value as a `CGVector`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624486-cgvectorvalue).
    #[inline]
    pub fn cg_vector_value(&self) -> CGVector {
        unsafe { _msg_send![self, CGVectorValue] }
    }

    /// Creates a new value object containing the specified affine
    /// transformation matrix.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624503-valuewithcgaffinetransform).
    #[inline]
    pub fn value_with_cg_affine_transform(value: CGAffineTransform) -> Self {
        unsafe { _msg_send![Self::class(), valueWithCGAffineTransform: value] }
    }

    /// Returns the value as a `CGAffineTransform`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624512-cgaffinetransformvalue).
    #[inline]
    pub fn cg_affine_transform_value(&self) -> CGAffineTransform {
        unsafe { _msg_send![self, CGAffineTransformValue] }
    }
}
