use super::{CGFloat, CGPoint, CGRect, CGSize};

/// An affine transformation matrix for use in drawing 2D graphics.
///
/// See [documentation](https://developer.apple.com/documentation/coregraphics/cgaffinetransform).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialOrd)]
#[cfg_attr(not(test), derive(PartialEq))]
pub struct CGAffineTransform {
    /// The entry at position [1,1] in the matrix.
    pub a: CGFloat,
    /// The entry at position [1,2] in the matrix.
    pub b: CGFloat,
    /// The entry at position [2,1] in the matrix.
    pub c: CGFloat,
    /// The entry at position [2,2] in the matrix.
    pub d: CGFloat,
    /// The entry at position [3,1] in the matrix.
    pub tx: CGFloat,
    /// The entry at position [3,2] in the matrix.
    pub ty: CGFloat,
}

// For the sake of testing, NaN is considered equal to itself.
#[cfg(test)]
impl PartialEq for CGAffineTransform {
    fn eq(&self, other: &Self) -> bool {
        fn special_eq(a: CGFloat, b: CGFloat) -> bool {
            a == b || (a.is_nan() && b.is_nan())
        }

        fn as_array(t: &CGAffineTransform) -> &[CGFloat; 6] {
            unsafe { &*(t as *const _ as *const _) }
        }

        let this = as_array(self);
        let other = as_array(other);

        for i in 0..6 {
            let a = this[i];
            let b = other[i];

            if !special_eq(a, b) {
                return false;
            }
        }

        true
    }
}

impl CGAffineTransform {
    /// An affine transformation of all zeroes.
    pub const ZERO: Self = Self::new_scale(0.0, 0.0);

    /// An affine transformation that changes nothing.
    pub const IDENTITY: Self = Self::new_scale(1.0, 1.0);

    /// Returns an affine transformation matrix with the provided values.
    ///
    /// This is equivalent to
    /// [`CGAffineTransformMake`](https://developer.apple.com/documentation/coregraphics/1455865-cgaffinetransformmake).
    #[inline]
    pub const fn new(
        a: CGFloat,
        b: CGFloat,
        c: CGFloat,
        d: CGFloat,
        tx: CGFloat,
        ty: CGFloat,
    ) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    /// Returns a transformation for rotating by `angle`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455666-cgaffinetransformmakerotation).
    #[inline]
    pub fn new_rotation(angle: CGFloat) -> Self {
        extern "C" {
            fn CGAffineTransformMakeRotation(angle: CGFloat) -> CGAffineTransform;
        }
        unsafe { CGAffineTransformMakeRotation(angle) }
    }

    /// Returns a transformation for scaling by the provided values.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455016-cgaffinetransformmakescale).
    #[inline]
    pub const fn new_scale(sx: CGFloat, sy: CGFloat) -> Self {
        Self::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
    }

    /// Returns a transformation for translating across the x and y axes.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454909-cgaffinetransformmaketranslation).
    #[inline]
    pub const fn new_translation(tx: CGFloat, ty: CGFloat) -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0, tx, ty)
    }

    /// Returns `true` if `self` does nothing when applied.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455754-cgaffinetransformisidentity).
    pub fn is_identity(&self) -> bool {
        // This implementation is optimized for speed and code size.

        // The sign bit.
        #[cfg(target_pointer_width = "64")]
        const SIGN: u64 = 0x8000_0000_0000_0000;
        #[cfg(target_pointer_width = "32")]
        const SIGN: u32 = 0x8000_0000;

        // The bits for 1.0.
        #[cfg(target_pointer_width = "64")]
        const ONE: u64 = 0x3FF0000000000000;
        #[cfg(target_pointer_width = "32")]
        const ONE: u32 = 0x3F800000;

        if self.a.to_bits() != ONE || self.d.to_bits() != ONE {
            false
        } else {
            // 0.0 and -0.0 without the sign bit have all bits set to 0.
            (self.b.to_bits() & !SIGN)
                | (self.c.to_bits() & !SIGN)
                | (self.tx.to_bits() & !SIGN)
                | (self.ty.to_bits() & !SIGN)
                == 0
        }
    }

    /// Returns the result of applying this affine transformation to `point`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454251-cgpointapplyaffinetransform).
    #[inline]
    pub fn apply_to_point(self, point: CGPoint) -> CGPoint {
        point.apply(self)
    }

    /// Returns the result of applying this affine transformation to `size`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1454806-cgsizeapplyaffinetransform).
    #[inline]
    pub fn apply_to_size(self, size: CGSize) -> CGSize {
        size.apply(self)
    }

    /// Returns the result of applying this affine transformation to `rect`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455875-cgrectapplyaffinetransform).
    #[inline]
    pub fn apply_to_rect(self, rect: CGRect) -> CGRect {
        rect.apply(self)
    }

    /// Returns the result of inverse of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455264-cgaffinetransforminvert).
    #[inline]
    pub fn invert(self) -> Self {
        extern "C" {
            fn CGAffineTransformInvert(transform: CGAffineTransform) -> CGAffineTransform;
        }
        unsafe { CGAffineTransformInvert(self) }
    }

    /// Returns the result of rotating `self` by `angle`.
    ///
    /// The actual direction of rotation is dependent on the coordinate system
    /// orientation of the target platform, which is different in iOS and macOS.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455962-cgaffinetransformrotate).
    #[inline]
    pub fn rotate(self, angle: CGFloat) -> Self {
        extern "C" {
            fn CGAffineTransformRotate(
                transform: CGAffineTransform,
                angle: CGFloat,
            ) -> CGAffineTransform;
        }
        unsafe { CGAffineTransformRotate(self, angle) }
    }

    /// Returns the result of scaling the x and y values of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455882-cgaffinetransformscale).
    #[inline]
    pub fn scale(self, sx: CGFloat, sy: CGFloat) -> Self {
        Self {
            a: self.a * sx,
            b: self.b * sx,
            c: self.c * sy,
            d: self.d * sy,
            ..self
        }
    }

    /// Returns the result of moving the x and y values of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455822-cgaffinetransformtranslate).
    #[inline]
    pub fn translate(self, tx: CGFloat, ty: CGFloat) -> Self {
        extern "C" {
            fn CGAffineTransformTranslate(
                transform: CGAffineTransform,
                tx: CGFloat,
                ty: CGFloat,
            ) -> CGAffineTransform;
        }
        unsafe { CGAffineTransformTranslate(self, tx, ty) }
    }

    /// Returns an affine transformation matrix constructed by multiplying
    /// `self` with `other`.
    ///
    /// You might perform several concatenations in order to create a single
    /// affine transform that contains the cumulative effects of several
    /// transformations.
    ///
    /// Note that matrix operations are not commutative—the order in which you
    /// concatenate matrices is important. That is, the result of multiplying
    /// `self` by `other` does not necessarily equal the result of multiplying
    /// `other` by `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/1455996-cgaffinetransformconcat).
    #[inline]
    pub fn concat(self, other: Self) -> Self {
        extern "C" {
            fn CGAffineTransformConcat(
                t1: CGAffineTransform,
                t2: CGAffineTransform,
            ) -> CGAffineTransform;
        }
        unsafe { CGAffineTransformConcat(self, other) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    fn rand_transform<R: Rng>(rng: &mut R) -> CGAffineTransform {
        CGAffineTransform::new(
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
            rng.gen(),
        )
    }

    fn combinations(scalars: &[CGFloat]) -> impl Iterator<Item = CGAffineTransform> + '_ {
        scalars
            .iter()
            .flat_map(move |&a| scalars.iter().map(move |&b| (a, b)))
            .flat_map(move |(a, b)| scalars.iter().map(move |&c| (a, b, c)))
            .flat_map(move |(a, b, c)| scalars.iter().map(move |&d| (a, b, c, d)))
            .flat_map(move |(a, b, c, d)| scalars.iter().map(move |&tx| (a, b, c, d, tx)))
            .flat_map(move |(a, b, c, d, tx)| {
                scalars
                    .iter()
                    .map(move |&ty| CGAffineTransform::new(a, b, c, d, tx, ty))
            })
    }

    fn special_scalar_combinations() -> impl Iterator<Item = CGAffineTransform> {
        combinations(SPECIAL_SCALARS)
    }

    const SPECIAL_SCALARS: &[CGFloat] = &[
        0.0,
        -0.0,
        1.0,
        -1.0,
        CGFloat::INFINITY,
        CGFloat::NEG_INFINITY,
        CGFloat::NAN,
    ];

    #[test]
    fn identity() {
        extern "C" {
            static CGAffineTransformIdentity: CGAffineTransform;
        }
        let expected = unsafe { CGAffineTransformIdentity };

        assert_eq!(CGAffineTransform::IDENTITY, expected);
    }

    #[test]
    fn is_identity() {
        fn reference_impl(transform: &CGAffineTransform) -> bool {
            extern "C" {
                fn CGAffineTransformIsIdentity(transform: CGAffineTransform) -> u8;
            }
            unsafe { CGAffineTransformIsIdentity(*transform) != 0 }
        }

        fn test(transform: &CGAffineTransform) {
            let result = transform.is_identity();
            let expected = reference_impl(&transform);

            assert_eq!(
                result, expected,
                "Incorrect `is_identity` for {:?}",
                transform,
            );
        }

        // Test special scalars that include identity transforms.
        for transform in special_scalar_combinations() {
            test(&transform);
        }

        // Test random transforms.
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            test(&rand_transform(&mut rng));
        }
    }

    #[test]
    fn new_scale() {
        fn reference_impl(sx: CGFloat, sy: CGFloat) -> CGAffineTransform {
            extern "C" {
                fn CGAffineTransformMakeScale(sx: CGFloat, sy: CGFloat) -> CGAffineTransform;
            }
            unsafe { CGAffineTransformMakeScale(sx, sy) }
        }

        fn test(sx: CGFloat, sy: CGFloat) {
            let result = CGAffineTransform::new_scale(sx, sy);
            let expected = reference_impl(sx, sy);

            assert_eq!(
                result, expected,
                "Incorrect scale transform for sx={:?}, sy={:?}",
                sx, sy,
            );
        }

        // Test special scalars.
        for &sx in SPECIAL_SCALARS {
            for &sy in SPECIAL_SCALARS {
                test(sx, sy);
            }
        }

        // Test random scalars.
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            test(rng.gen(), rng.gen());
        }
    }

    #[test]
    fn scale() {
        fn reference_impl(t: CGAffineTransform, sx: CGFloat, sy: CGFloat) -> CGAffineTransform {
            extern "C" {
                fn CGAffineTransformScale(
                    t: CGAffineTransform,
                    sx: CGFloat,
                    sy: CGFloat,
                ) -> CGAffineTransform;
            }
            unsafe { CGAffineTransformScale(t, sx, sy) }
        }

        fn test(transform: CGAffineTransform, sx: CGFloat, sy: CGFloat) {
            let result = transform.scale(sx, sy);
            let expected = reference_impl(transform, sx, sy);

            assert_eq!(
                result, expected,
                "Incorrect scale for sx={:?}, sy={:?}\n    on: `{:?}`",
                sx, sy, transform
            );
        }

        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let transform = rand_transform(&mut rng);

            // Test special scalars.
            for &sx in SPECIAL_SCALARS {
                for &sy in SPECIAL_SCALARS {
                    test(transform, sx, sy);
                }
            }

            // Test random scalars.
            for _ in 0..100 {
                test(transform, rng.gen(), rng.gen());
            }
        }
    }

    #[test]
    fn new_translation() {
        fn reference_impl(tx: CGFloat, ty: CGFloat) -> CGAffineTransform {
            extern "C" {
                fn CGAffineTransformMakeTranslation(tx: CGFloat, ty: CGFloat) -> CGAffineTransform;
            }
            unsafe { CGAffineTransformMakeTranslation(tx, ty) }
        }

        fn test(tx: CGFloat, ty: CGFloat) {
            let result = CGAffineTransform::new_translation(tx, ty);
            let expected = reference_impl(tx, ty);

            assert_eq!(
                result, expected,
                "Incorrect scale transform for tx={:?}, ty={:?}",
                tx, ty,
            );
        }

        // Test special scalars.
        for &tx in SPECIAL_SCALARS {
            for &ty in SPECIAL_SCALARS {
                test(tx, ty);
            }
        }

        // Test random scalars.
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            test(rng.gen(), rng.gen());
        }
    }
}
