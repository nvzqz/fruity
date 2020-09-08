use crate::objc::{Class, NSObject, NSUInteger};
use std::ops::Deref;

/// A static, plain-text Unicode string object.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsstring).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSString(NSObject);

impl From<NSString> for NSObject {
    #[inline]
    fn from(obj: NSString) -> Self {
        obj.0
    }
}

impl Deref for NSString {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        &self.0
    }
}

impl NSString {
    /// Returns the `NSString` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSString"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> NSString {
        Self(NSObject::copy(self))
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> NSMutableString {
        NSMutableString(Self(NSObject::mutable_copy(self)))
    }
}

/// A dynamic plain-text Unicode string object.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsmutablestring).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSMutableString(NSString);

impl From<NSMutableString> for NSObject {
    #[inline]
    fn from(obj: NSMutableString) -> Self {
        (obj.0).0
    }
}

impl From<NSMutableString> for NSString {
    #[inline]
    fn from(obj: NSMutableString) -> Self {
        obj.0
    }
}

impl Deref for NSMutableString {
    type Target = NSString;

    #[inline]
    fn deref(&self) -> &NSString {
        &self.0
    }
}

impl NSMutableString {
    /// Returns the `NSMutableString` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSMutableString"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }
}

/// Possible [`NSString`](struct.NSString.html) encodings.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsstringencoding).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NSStringEncoding(pub NSUInteger);

#[allow(non_upper_case_globals)]
impl NSStringEncoding {
    /// Strict 7-bit ASCII encoding within 8-bit chars; ASCII values 0…127 only.
    pub const ASCII: Self = Self(1);

    /// 8-bit ASCII encoding with NEXTSTEP extensions.
    pub const NEXTSTEP: Self = Self(2);

    /// 8-bit EUC encoding for Japanese text.
    pub const JapaneseEUC: Self = Self(3);

    /// An 8-bit representation of Unicode characters, suitable for transmission
    /// or storage by ASCII-based systems.
    pub const UTF8: Self = Self(4);

    /// 8-bit ISO Latin 1 encoding.
    pub const ISOLatin1: Self = Self(5);

    /// 8-bit Adobe Symbol encoding vector.
    pub const Symbol: Self = Self(6);

    /// 7-bit verbose ASCII to represent all Unicode characters.
    pub const NonLossyASCII: Self = Self(7);

    /// 8-bit Shift-JIS encoding for Japanese text.
    pub const ShiftJIS: Self = Self(8);

    /// 8-bit ISO Latin 2 encoding.
    pub const ISOLatin2: Self = Self(9);

    /// The canonical Unicode encoding for string objects.
    pub const Unicode: Self = Self(10);

    /// Microsoft Windows codepage 1251, encoding Cyrillic characters;
    /// equivalent to AdobeStandardCyrillic font encoding.
    pub const WindowsCP1251: Self = Self(11);

    /// Microsoft Windows codepage 1252; equivalent to WinLatin1.
    pub const WindowsCP1252: Self = Self(12);

    /// Microsoft Windows codepage 1253, encoding Greek characters.
    pub const WindowsCP1253: Self = Self(13);

    /// Microsoft Windows codepage 1254, encoding Turkish characters.
    pub const WindowsCP1254: Self = Self(14);

    /// Microsoft Windows codepage 1250; equivalent to WinLatin2.
    pub const WindowsCP1250: Self = Self(15);

    /// ISO 2022 Japanese encoding for email.
    pub const ISO2022JP: Self = Self(21);

    /// Classic Macintosh Roman encoding.
    pub const MacOSRoman: Self = Self(30);

    /// An alias for [`Unicode`](#associatedconstant.Unicode).
    pub const UTF16: Self = Self::Unicode;

    /// [`UTF16`](#associatedconstant.UTF16) encoding with explicit endianness
    /// specified.
    pub const UTF16BigEndian: Self = Self(0x90000100);

    /// [`UTF16`](#associatedconstant.UTF16) encoding with explicit endianness
    /// specified.
    pub const UTF16LittleEndian: Self = Self(0x94000100);

    /// 32-bit UTF encoding.
    pub const UTF32: Self = Self(0x8c000100);

    /// [`UTF32`](#associatedconstant.UTF32) encoding with explicit endianness
    /// specified.
    pub const UTF32BigEndian: Self = Self(0x98000100);

    /// [`UTF32`](#associatedconstant.UTF32) encoding with explicit endianness
    /// specified.
    pub const UTF32LittleEndian: Self = Self(0x9c000100);
}
