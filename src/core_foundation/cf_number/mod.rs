use crate::{
    core::Arc,
    core_foundation::{sys, CFAllocator, CFComparisonResult, CFType},
};
use std::{
    cmp::Ordering,
    mem::{self, MaybeUninit},
    ptr,
};

mod type_;

pub use type_::*;

use super::{CFIndex, CFTypeID};

subclass! {
    /// A number object.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/corefoundation/cfnumber?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/corefoundation/cfnumber?language=objc)
    #[derive(PartialEq, Hash)]
    pub class CFNumber: CFType<'static>;
}

impl Eq for CFNumber {}

impl PartialOrd for CFNumber {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFNumber {
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other).into()
    }
}

impl From<i8> for Arc<CFNumber> {
    #[inline]
    fn from(value: i8) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::I8, &value) }
    }
}

impl From<i16> for Arc<CFNumber> {
    #[inline]
    fn from(value: i16) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::I16, &value) }
    }
}

impl From<i32> for Arc<CFNumber> {
    #[inline]
    fn from(value: i32) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::I32, &value) }
    }
}

impl From<i64> for Arc<CFNumber> {
    #[inline]
    fn from(value: i64) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::I64, &value) }
    }
}

impl From<isize> for Arc<CFNumber> {
    #[inline]
    fn from(value: isize) -> Self {
        if mem::size_of::<isize>() == 4 {
            (value as i32).into()
        } else {
            (value as i64).into()
        }
    }
}

impl From<f32> for Arc<CFNumber> {
    #[inline]
    fn from(value: f32) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::F32, &value) }
    }
}

impl From<f64> for Arc<CFNumber> {
    #[inline]
    fn from(value: f64) -> Self {
        unsafe { CFNumber::create(None, CFNumberType::F64, &value) }
    }
}

impl CFNumber {
    /// Returns a reference to a
    /// [NaN (Not a Number)](https://en.wikipedia.org/wiki/NaN) value.
    ///
    /// This internally references
    /// [`kCFNumberNaN`](https://developer.apple.com/documentation/corefoundation/kCFNumberNaN).
    #[inline]
    #[doc(alias = "kCFNumberNaN")]
    pub fn nan() -> &'static Self {
        extern "C" {
            static kCFNumberNaN: &'static CFNumber;
        }
        unsafe { kCFNumberNaN }
    }

    /// Returns a reference to the infinity (∞) value.
    ///
    /// This internally references
    /// [`kCFNumberPositiveInfinity`](https://developer.apple.com/documentation/corefoundation/kcfnumberpositiveinfinity).
    #[inline]
    #[doc(alias = "kCFNumberPositiveInfinity")]
    pub fn infinity() -> &'static Self {
        extern "C" {
            static kCFNumberPositiveInfinity: &'static CFNumber;
        }
        unsafe { kCFNumberPositiveInfinity }
    }

    /// Returns a reference to the negative infinity (−∞) value.
    ///
    /// This internally references
    /// [`kCFNumberNegativeInfinity`](https://developer.apple.com/documentation/corefoundation/kcfnumbernegativeinfinity).
    #[inline]
    #[doc(alias = "kCFNumberNegativeInfinity")]
    pub fn neg_infinity() -> &'static Self {
        extern "C" {
            static kCFNumberNegativeInfinity: &'static CFNumber;
        }
        unsafe { kCFNumberNegativeInfinity }
    }
}

impl CFNumber {
    /// Returns the type identifier for `CFNumber`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1541730-cfnumbergettypeid?language=objc).
    #[inline]
    #[doc(alias = "CFNumberGetTypeID")]
    pub fn type_id() -> CFTypeID {
        unsafe { sys::CFNumberGetTypeID() }
    }

    /// Creates a new `CFNumber` object using a specified value's `Into`
    /// implementation.
    #[inline]
    pub fn new<T>(value: T) -> Arc<Self>
    where
        T: Into<Arc<Self>>,
    {
        value.into()
    }

    /// Creates a new `CFNumber` object using a specified value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1542182-cfnumbercreate?language=objc).
    ///
    /// # Safety
    ///
    /// `value` must represent a valid instance for the given `CFNumberType`.
    #[inline]
    #[doc(alias = "CFNumberCreate")]
    pub unsafe fn create<T>(
        allocator: Option<&CFAllocator>,
        number_type: CFNumberType,
        value: &T,
    ) -> Arc<Self> {
        Arc::from_raw(sys::CFNumberCreate(
            match allocator {
                Some(allocator) => allocator,
                None => ptr::null(),
            },
            number_type,
            (value as *const T).cast(),
        ))
    }

    /// Compares `self` to `other` and returns the result.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1542018-cfnumbercompare?language=objc).
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    pub fn compare(&self, other: &Self) -> CFComparisonResult {
        unsafe { sys::CFNumberCompare(self, other, ptr::null_mut()) }
    }

    /// Returns the number of bytes used by this object to store its value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1542080-cfnumbergetbytesize?language=objc).
    #[inline]
    #[doc(alias = "CFNumberGetByteSize")]
    pub fn byte_size(&self) -> CFIndex {
        unsafe { sys::CFNumberGetByteSize(self) }
    }

    /// Returns the type used by this object to store its value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1543388-cfnumbergettype?language=objc).
    #[inline]
    #[doc(alias = "CFNumberGetType")]
    pub fn get_type(&self) -> CFNumberType {
        unsafe { sys::CFNumberGetType(self) }
    }

    /// Returns `true` if this object contains a value stored as one of the
    /// defined floating point types.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1543131-cfnumberisfloattype?language=objc).
    #[inline]
    #[doc(alias = "CFNumberIsFloatType")]
    pub fn is_float_type(&self) -> bool {
        unsafe { sys::CFNumberIsFloatType(self) != 0 }
    }
}

/// Getting number values.
///
/// All of these functions call
/// [`CFNumberGetValue`](https://developer.apple.com/documentation/corefoundation/1543114-cfnumbergetvalue?language=objc).
impl CFNumber {
    /// Returns the value of this object cast to a specified type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1543114-cfnumbergetvalue?language=objc).
    ///
    /// # Safety
    ///
    /// The generic type `T` must have a valid representation for the requested
    /// [`CFNumberType`].
    #[inline]
    #[doc(alias = "CFNumberGetValue")]
    pub unsafe fn get_value<T>(&self, number_type: CFNumberType) -> Option<T> {
        let mut value = MaybeUninit::<T>::uninit();

        if sys::CFNumberGetValue(self, number_type, value.as_mut_ptr().cast()) != 0 {
            Some(value.assume_init())
        } else {
            None
        }
    }

    /// Returns the value of this object cast to `i8`.
    #[inline]
    pub fn i8_value(&self) -> Option<i8> {
        unsafe { self.get_value(CFNumberType::I8) }
    }

    /// Returns the value of this object cast to `i16`.
    #[inline]
    pub fn i16_value(&self) -> Option<i16> {
        unsafe { self.get_value(CFNumberType::I16) }
    }

    /// Returns the value of this object cast to `i32`.
    #[inline]
    pub fn i32_value(&self) -> Option<i32> {
        unsafe { self.get_value(CFNumberType::I32) }
    }

    /// Returns the value of this object cast to `i64`.
    #[inline]
    pub fn i64_value(&self) -> Option<i64> {
        unsafe { self.get_value(CFNumberType::I64) }
    }

    /// Returns the value of this object cast to `isize`.
    #[inline]
    pub fn isize_value(&self) -> Option<isize> {
        if mem::size_of::<isize>() == 4 {
            Some(self.i32_value()? as isize)
        } else {
            Some(self.i64_value()? as isize)
        }
    }

    /// Returns the value of this object cast to `f32`.
    #[inline]
    pub fn f32_value(&self) -> Option<f32> {
        unsafe { self.get_value(CFNumberType::F32) }
    }

    /// Returns the value of this object cast to `f64`.
    #[inline]
    pub fn f64_value(&self) -> Option<f64> {
        unsafe { self.get_value(CFNumberType::F64) }
    }
}
