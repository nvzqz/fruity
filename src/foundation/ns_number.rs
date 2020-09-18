use super::{NSComparisonResult, NSString, NSValue};
use crate::objc::{Class, NSInteger, NSObject, NSUInteger, Object, ObjectType, BOOL};
use std::{
    cmp::Ordering,
    fmt,
    ops::Deref,
    os::raw::{
        c_char, c_double, c_float, c_int, c_long, c_longlong, c_short, c_uchar, c_uint, c_ulong,
        c_ulonglong, c_ushort,
    },
    ptr::NonNull,
};

/// An object wrapper for primitive scalar numeric values.
///
/// There are [static instances](#static-instances) which make using certain
/// numbers much faster.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSNumber(NSValue);

unsafe impl ObjectType for NSNumber {}

impl From<NSNumber> for NSValue {
    #[inline]
    fn from(obj: NSNumber) -> Self {
        obj.0
    }
}

impl From<NSNumber> for NSObject {
    #[inline]
    fn from(obj: NSNumber) -> Self {
        NSValue::from(obj).into()
    }
}

impl Deref for NSNumber {
    type Target = NSValue;

    #[inline]
    fn deref(&self) -> &NSValue {
        &self.0
    }
}

impl PartialEq for NSNumber {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { _msg_send_cached![self, isEqualToNumber: other as &Object => BOOL] != 0 }
    }
}

impl Eq for NSNumber {}

impl PartialOrd for NSNumber {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NSNumber {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other).into()
    }
}

impl From<bool> for NSNumber {
    #[inline]
    fn from(value: bool) -> Self {
        Self::with_bool(value)
    }
}

impl From<c_double> for NSNumber {
    #[inline]
    fn from(value: c_double) -> Self {
        Self::with_double(value)
    }
}

impl From<c_float> for NSNumber {
    #[inline]
    fn from(value: c_float) -> Self {
        Self::with_float(value)
    }
}

impl From<NSInteger> for NSNumber {
    #[inline]
    fn from(value: NSInteger) -> Self {
        Self::with_integer(value)
    }
}

impl From<NSUInteger> for NSNumber {
    #[inline]
    fn from(value: NSUInteger) -> Self {
        Self::with_unsigned_integer(value)
    }
}

impl From<c_char> for NSNumber {
    #[inline]
    fn from(value: c_char) -> Self {
        Self::with_char(value)
    }
}

impl From<c_uchar> for NSNumber {
    #[inline]
    fn from(value: c_uchar) -> Self {
        Self::with_unsigned_char(value)
    }
}

impl From<c_int> for NSNumber {
    #[inline]
    fn from(value: c_int) -> Self {
        Self::with_int(value)
    }
}

impl From<c_uint> for NSNumber {
    #[inline]
    fn from(value: c_uint) -> Self {
        Self::with_unsigned_int(value)
    }
}

impl From<c_long> for NSNumber {
    #[inline]
    fn from(value: c_long) -> Self {
        Self::with_long(value)
    }
}

impl From<c_ulong> for NSNumber {
    #[inline]
    fn from(value: c_ulong) -> Self {
        Self::with_unsigned_long(value)
    }
}

// TODO: Determine if `c_longlong` and `c_ulonglong` differ from `c_long` and
// `c_ulong` on the targeted platforms. If they do, then conditionally add
// `From` implementations.

impl From<c_short> for NSNumber {
    #[inline]
    fn from(value: c_short) -> Self {
        Self::with_short(value)
    }
}

impl From<c_ushort> for NSNumber {
    #[inline]
    fn from(value: c_ushort) -> Self {
        Self::with_unsigned_short(value)
    }
}

impl From<NSNumber> for NSString {
    #[inline]
    fn from(number: NSNumber) -> Self {
        number.string_value()
    }
}

impl From<&NSNumber> for NSString {
    #[inline]
    fn from(number: &NSNumber) -> Self {
        number.string_value()
    }
}

impl fmt::Debug for NSNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for NSNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self._cfboolean_value() {
            Some(false) => "NO".fmt(f),
            Some(true) => "YES".fmt(f),
            None => match unsafe { *self.objc_type() as u8 } {
                // https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
                b'f' => self.float_value().fmt(f),
                b'd' => self.double_value().fmt(f),
                b'c' | b'i' | b's' | b'l' | b'q' => self.longlong_value().fmt(f),
                _ => self.unsigned_longlong_value().fmt(f),
            },
        }
    }
}

impl fmt::Pointer for NSNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl NSNumber {
    /// Returns the `NSNumber` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSNumber"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Creates an immutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSNumber` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSValue::from_ptr(ptr))
    }

    /// Creates an immutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSNumber` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSValue::from_non_null_ptr(ptr))
    }
}

/// Scalar constructors.
impl NSNumber {
    // TODO: Add constructors:
    // - initWithCoder:

    /// Creates a number object containing a boolean.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551475-numberwithbool).
    #[inline]
    pub fn with_bool(value: bool) -> Self {
        unsafe { _msg_send![Self::class(), numberWithBool: value as BOOL] }
    }

    /// Creates a number object from a C `float`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551471-numberwithfloat)
    #[inline]
    pub fn with_float(value: c_float) -> Self {
        unsafe { _msg_send![Self::class(), numberWithFloat: value] }
    }

    /// Creates a number object from a C `double`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551463-numberwithdouble)
    #[inline]
    pub fn with_double(value: c_double) -> Self {
        unsafe { _msg_send![Self::class(), numberWithDouble: value] }
    }

    /// Creates a number object from a C `char`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551464-numberwithchar)
    #[inline]
    pub fn with_char(value: c_char) -> Self {
        unsafe { _msg_send![Self::class(), numberWithChar: value] }
    }

    /// Creates a number object from a C `short`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551476-numberwithshort)
    #[inline]
    pub fn with_short(value: c_short) -> Self {
        unsafe { _msg_send![Self::class(), numberWithShort: value] }
    }

    /// Creates a number object from a C `int`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551470-numberwithint)
    #[inline]
    pub fn with_int(value: c_int) -> Self {
        unsafe { _msg_send![Self::class(), numberWithInt: value] }
    }

    /// Creates a number object from a C `long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551474-numberwithlong)
    #[inline]
    pub fn with_long(value: c_long) -> Self {
        unsafe { _msg_send![Self::class(), numberWithLong: value] }
    }

    /// Creates a number object from a C `long long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551462-numberwithlonglong)
    #[inline]
    pub fn with_longlong(value: c_longlong) -> Self {
        unsafe { _msg_send![Self::class(), numberWithLongLong: value] }
    }

    /// Creates a number object from an Objective-C integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551473-numberwithinteger)
    #[inline]
    pub fn with_integer(value: NSInteger) -> Self {
        unsafe { _msg_send![Self::class(), numberWithInteger: value] }
    }

    /// Creates a number object from a C `unsigned char`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551468-numberwithunsignedchar)
    #[inline]
    pub fn with_unsigned_char(value: c_uchar) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedChar: value] }
    }

    /// Creates a number object from a C `unsigned short`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551467-numberwithunsignedshort)
    #[inline]
    pub fn with_unsigned_short(value: c_ushort) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedShort: value] }
    }

    /// Creates a number object from a C `unsigned int`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551472-numberwithunsignedint)
    #[inline]
    pub fn with_unsigned_int(value: c_uint) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedInt: value] }
    }

    /// Creates a number object from a C `unsigned long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551477-numberwithunsignedlong)
    #[inline]
    pub fn with_unsigned_long(value: c_ulong) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedLong: value] }
    }

    /// Creates a number object from a C `unsigned long long`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551465-numberwithunsignedlonglong)
    #[inline]
    pub fn with_unsigned_longlong(value: c_ulonglong) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedLongLong: value] }
    }

    /// Creates a number object from a Objective-C unsigned integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1551469-numberwithunsignedinteger)
    #[inline]
    pub fn with_unsigned_integer(value: NSUInteger) -> Self {
        unsafe { _msg_send![Self::class(), numberWithUnsignedInteger: value] }
    }
}

/// <span id="static-instances">Static instances</span>.
///
/// Use these number references over corresponding constructors for better
/// performance.
impl NSNumber {
    // SAFETY: NSNumber is toll-free bridged to CFNumber and CFBoolean, the
    // underlying types of these statics.

    /// Returns a reference to the equivalent of `@NO`.
    ///
    /// This internally references
    /// [`kCFBooleanFalse`](https://developer.apple.com/documentation/corefoundation/kCFBooleanFalse).
    #[inline]
    pub fn no() -> &'static NSNumber {
        extern "C" {
            static kCFBooleanFalse: NSNumber;
        }
        unsafe { &kCFBooleanFalse }
    }

    /// Returns a reference to the equivalent of `@YES`.
    ///
    /// This internally references
    /// [`kCFBooleanTrue`](https://developer.apple.com/documentation/corefoundation/kCFBooleanTrue).
    #[inline]
    pub fn yes() -> &'static NSNumber {
        extern "C" {
            static kCFBooleanTrue: NSNumber;
        }
        unsafe { &kCFBooleanTrue }
    }

    /// Returns a reference to a
    /// [NaN (Not a Number)](https://en.wikipedia.org/wiki/NaN) value.
    ///
    /// This internally references
    /// [`kCFNumberNaN`](https://developer.apple.com/documentation/corefoundation/kCFNumberNaN).
    #[inline]
    pub fn nan() -> &'static NSNumber {
        extern "C" {
            static kCFNumberNaN: NSNumber;
        }
        unsafe { &kCFNumberNaN }
    }

    /// Returns a reference to the infinity (∞) value.
    ///
    /// This internally references
    /// [`kCFNumberPositiveInfinity`](https://developer.apple.com/documentation/corefoundation/kcfnumberpositiveinfinity).
    #[inline]
    pub fn infinity() -> &'static NSNumber {
        extern "C" {
            static kCFNumberPositiveInfinity: NSNumber;
        }
        unsafe { &kCFNumberPositiveInfinity }
    }

    /// Returns a reference to the negative infinity (−∞) value.
    ///
    /// This internally references
    /// [`kCFNumberNegativeInfinity`](https://developer.apple.com/documentation/corefoundation/kcfnumbernegativeinfinity).
    #[inline]
    pub fn neg_infinity() -> &'static NSNumber {
        extern "C" {
            static kCFNumberNegativeInfinity: NSNumber;
        }
        unsafe { &kCFNumberNegativeInfinity }
    }
}

/// Instance operations.
impl NSNumber {
    #[inline]
    pub(crate) fn _cfboolean_value(&self) -> Option<bool> {
        let this = self.as_ptr();
        if this == Self::no().as_ptr() {
            Some(false)
        } else if this == Self::yes().as_ptr() {
            Some(true)
        } else {
            None
        }
    }

    /// Returns an `NSComparisonResult` value that indicates whether the number
    /// object’s value is greater than, equal to, or less than a given number.
    ///
    /// This method follows the standard C rules for type conversion. For
    /// example, if you compare an NSNumber object that has an integer value
    /// with an NSNumber object that has a floating point value, the integer
    /// value is converted to a floating-point value for comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1413562-compare)
    #[inline]
    pub fn compare(&self, other: &NSNumber) -> NSComparisonResult {
        unsafe { _msg_send![self, compare: other as &Object] }
    }

    /// Returns the number object's value expressed as a human-readable string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1415802-stringvalue)
    #[inline]
    pub fn string_value(&self) -> NSString {
        unsafe { _msg_send![self, stringValue] }
    }

    /// Returns a string that represents the contents of the number object for a
    /// given locale.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1409984-descriptionwithlocale)
    #[inline]
    pub fn description_with_locale<L>(&self, locale: Option<L>) -> NSString
    where
        L: AsRef<Object>,
    {
        let locale: Option<&Object> = match &locale {
            Some(locale) => Some(locale.as_ref()),
            None => None,
        };
        unsafe { _msg_send![self, descriptionWithLocale: locale] }
    }
}

/// Accessing numeric values.
impl NSNumber {
    // TODO: Implement methods:
    // - decimalValue

    /// Returns the number object's value expressed as boolean, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1410865-boolvalue).
    #[inline]
    pub fn bool_value(&self) -> bool {
        unsafe { _msg_send![self, boolValue => BOOL] != 0 }
    }

    /// Returns the number object's value expressed as a C `float`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1418317-floatvalue).
    #[inline]
    pub fn float_value(&self) -> c_float {
        unsafe { _msg_send![self, floatValue] }
    }

    /// Returns the number object's value expressed as a C `double`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1414104-doublevalue).
    #[inline]
    pub fn double_value(&self) -> c_double {
        unsafe { _msg_send![self, doubleValue] }
    }

    /// Returns the number object's value expressed as a C `char`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1407838-charvalue).
    #[inline]
    pub fn char_value(&self) -> c_char {
        unsafe { _msg_send![self, charValue] }
    }

    /// Returns the number object's value expressed as a C `short`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1407601-shortvalue).
    #[inline]
    pub fn short_value(&self) -> c_short {
        unsafe { _msg_send![self, shortValue] }
    }

    /// Returns the number object's value expressed as a C `int`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1407153-intvalue).
    #[inline]
    pub fn int_value(&self) -> c_int {
        unsafe { _msg_send![self, intValue] }
    }

    /// Returns the number object's value expressed as a C `long`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1412566-longvalue).
    #[inline]
    pub fn long_value(&self) -> c_long {
        unsafe { _msg_send![self, longValue] }
    }

    /// Returns the number object's value expressed as a C `long long`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1416870-longlongvalue).
    #[inline]
    pub fn longlong_value(&self) -> c_longlong {
        unsafe { _msg_send![self, longLongValue] }
    }

    /// Returns the number object's value expressed as an Objective-C integer, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1412554-integervalue).
    #[inline]
    pub fn integer_value(&self) -> NSInteger {
        unsafe { _msg_send![self, integerValue] }
    }

    /// Returns the number object's value expressed as a C `unsigned char`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1409016-unsignedcharvalue).
    #[inline]
    pub fn unsigned_char_value(&self) -> c_uchar {
        unsafe { _msg_send![self, unsignedCharValue] }
    }

    /// Returns the number object's value expressed as a C `unsigned short`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1410604-unsignedshortvalue).
    #[inline]
    pub fn unsigned_short_value(&self) -> c_ushort {
        unsafe { _msg_send![self, unsignedShortValue] }
    }

    /// Returns the number object's value expressed as a C `unsigned int`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1417875-unsignedintvalue).
    #[inline]
    pub fn unsigned_int_value(&self) -> c_uint {
        unsafe { _msg_send![self, unsignedIntValue] }
    }

    /// Returns the number object's value expressed as a C `unsigned long`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1415252-unsignedlongvalue).
    #[inline]
    pub fn unsigned_long_value(&self) -> c_ulong {
        unsafe { _msg_send![self, unsignedLongValue] }
    }

    /// Returns the number object's value expressed as a C `unsigned long long`, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1414524-unsignedlonglongvalue).
    #[inline]
    pub fn unsigned_longlong_value(&self) -> c_ulonglong {
        unsafe { _msg_send![self, unsignedLongLongValue] }
    }

    /// Returns the number object's value expressed as an Objective-C unsigned integer, converted as necessary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnumber/1413324-unsignedintegervalue).
    #[inline]
    pub fn unsigned_integer_value(&self) -> NSUInteger {
        unsafe { _msg_send![self, unsignedIntegerValue] }
    }
}
