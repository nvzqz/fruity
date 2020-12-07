use crate::{
    core::Arc,
    core_graphics::{CGAffineTransform, CGPoint, CGRect, CGSize, CGVector},
    foundation::NSValue,
    objc::ClassType,
};

/// Core Graphics geometry values.
///
/// Requires the **`ui_kit`** feature flag.
impl NSValue {
    /// Creates a new value object containing the specified point.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624531-valuewithcgpoint).
    #[inline]
    #[doc(alias = "valueWithCGPoint")]
    #[doc(alias = "valueWithCGPoint:")]
    pub fn from_cg_point(value: CGPoint) -> Arc<Self> {
        unsafe { _msg_send![Self::class(), valueWithCGPoint: value] }
    }

    /// Returns the value as a `CGPoint`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624534-cgpointvalue).
    #[inline]
    #[doc(alias = "CGPointValue")]
    pub fn cg_point_value(&self) -> CGPoint {
        unsafe { _msg_send![self, CGPointValue] }
    }

    /// Creates a new value object containing the specified size.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624511-valuewithcgsize).
    #[inline]
    #[doc(alias = "valueWithCGSize")]
    #[doc(alias = "valueWithCGSize:")]
    pub fn from_cg_size(value: CGSize) -> Arc<Self> {
        unsafe { _msg_send![Self::class(), valueWithCGSize: value] }
    }

    /// Returns the value as a `CGSize`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624489-cgsizevalue).
    #[inline]
    #[doc(alias = "CGSizeValue")]
    pub fn cg_size_value(&self) -> CGSize {
        unsafe { _msg_send![self, CGSizeValue] }
    }

    /// Creates a new value object containing the specified rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624529-valuewithcgrect).
    #[inline]
    #[doc(alias = "valueWithCGRect")]
    #[doc(alias = "valueWithCGRect:")]
    pub fn from_cg_rect(value: CGRect) -> Arc<Self> {
        unsafe { _msg_send![Self::class(), valueWithCGRect: value] }
    }

    /// Returns the value as a `CGRect`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreimage/civector/1438108-cgrectvalue).
    #[inline]
    #[doc(alias = "CGRectValue")]
    pub fn cg_rect_value(&self) -> CGRect {
        unsafe { _msg_send![self, CGRectValue] }
    }

    /// Creates a new value object containing the specified vector.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624493-valuewithcgvector).
    #[inline]
    #[doc(alias = "valueWithCGVector")]
    #[doc(alias = "valueWithCGVector:")]
    pub fn from_cg_vector(value: CGVector) -> Arc<Self> {
        unsafe { _msg_send![Self::class(), valueWithCGVector: value] }
    }

    /// Returns the value as a `CGVector`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624486-cgvectorvalue).
    #[inline]
    #[doc(alias = "CGVectorValue")]
    pub fn cg_vector_value(&self) -> CGVector {
        unsafe { _msg_send![self, CGVectorValue] }
    }

    /// Creates a new value object containing the specified affine
    /// transformation matrix.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624503-valuewithcgaffinetransform).
    #[inline]
    #[doc(alias = "valueWithCGAffineTransform")]
    #[doc(alias = "valueWithCGAffineTransform:")]
    pub fn from_cg_affine_transform(value: CGAffineTransform) -> Arc<Self> {
        unsafe { _msg_send![Self::class(), valueWithCGAffineTransform: value] }
    }

    /// Returns the value as a `CGAffineTransform`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1624512-cgaffinetransformvalue).
    #[inline]
    #[doc(alias = "CGAffineTransformValue")]
    pub fn cg_affine_transform_value(&self) -> CGAffineTransform {
        unsafe { _msg_send![self, CGAffineTransformValue] }
    }
}
