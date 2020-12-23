use crate::core_foundation::CFIndex;
use std::fmt;

/// Flags to indicate the data type of a [`CFNumber`](super::CFNumber) value.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CFNumberType(pub CFIndex);

impl fmt::Debug for CFNumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match *self {
            Self::I8 => "I8",
            Self::I16 => "I16",
            Self::I32 => "I32",
            Self::I64 => "I64",
            Self::F32 => "F32",
            Self::F64 => "F64",
            Self::C_CHAR => "C_CHAR",
            Self::C_SHORT => "C_SHORT",
            Self::C_INT => "C_INT",
            Self::C_LONG => "C_LONG",
            Self::C_LONGLONG => "C_LONGLONG",
            Self::C_FLOAT => "C_FLOAT",
            Self::C_DOUBLE => "C_DOUBLE",
            Self::CF_INDEX => "CF_INDEX",
            Self::NS_INTEGER => "NS_INTEGER",
            Self::CG_FLOAT => "CG_FLOAT",

            // If the value is outside of the known range, format as an integer.
            _ => return self.0.fmt(f),
        };
        as_str.fmt(f)
    }
}

impl CFNumberType {
    /// An [`i8`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberSInt8Type?language=objc).
    #[doc(alias = "kCFNumberSInt8Type")]
    pub const I8: Self = Self(1);

    /// An [`i16`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberSInt16Type?language=objc).
    #[doc(alias = "kCFNumberSInt16Type")]
    pub const I16: Self = Self(2);

    /// An [`i32`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberSInt32Type?language=objc).
    #[doc(alias = "kCFNumberSInt32Type")]
    pub const I32: Self = Self(3);

    /// An [`i64`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberSInt64Type?language=objc).
    #[doc(alias = "kCFNumberSInt64Type")]
    pub const I64: Self = Self(4);

    /// An [`f32`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberFloat32Type?language=objc).
    #[doc(alias = "kCFNumberFloat32Type")]
    pub const F32: Self = Self(5);

    /// An [`f64`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberFloat64Type?language=objc).
    #[doc(alias = "kCFNumberFloat64Type")]
    pub const F64: Self = Self(6);

    /// A C `char`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberCharType?language=objc).
    #[doc(alias = "kCFNumberCharType")]
    pub const C_CHAR: Self = Self(7);

    /// A C `short`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberShortType?language=objc).
    #[doc(alias = "kCFNumberShortType")]
    pub const C_SHORT: Self = Self(8);

    /// A C `int`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberIntType?language=objc).
    #[doc(alias = "kCFNumberIntType")]
    pub const C_INT: Self = Self(9);

    /// A C `long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberLongType?language=objc).
    #[doc(alias = "kCFNumberLongType")]
    pub const C_LONG: Self = Self(10);

    /// A C `long long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberLongLongType?language=objc).
    #[doc(alias = "kCFNumberLongLongType")]
    pub const C_LONGLONG: Self = Self(11);

    /// A C `float`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberFloatType?language=objc).
    #[doc(alias = "kCFNumberFloatType")]
    pub const C_FLOAT: Self = Self(12);

    /// A C `double`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberDoubleType?language=objc).
    #[doc(alias = "kCFNumberDoubleType")]
    pub const C_DOUBLE: Self = Self(13);

    /// A [`CFIndex`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberCFIndexType?language=objc).
    #[doc(alias = "kCFNumberCFIndexType")]
    pub const CF_INDEX: Self = Self(14);

    /// An [`NSInteger`](crate::objc::NSInteger).
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberNSIntegerType?language=objc).
    #[doc(alias = "kCFNumberNSIntegerType")]
    pub const NS_INTEGER: Self = Self(15);

    /// A [`CGFloat`](crate::core_graphics::CGFloat).
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/cfnumbertype/kCFNumberCGFloatType?language=objc).
    #[doc(alias = "kCFNumberCGFloatType")]
    pub const CG_FLOAT: Self = Self(16);
}
