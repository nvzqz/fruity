use super::NSComparisonResult;
use crate::objc::{id, Class, NSObject, NSUInteger, BOOL, SEL};
use std::{cmp::Ordering, ops::Deref};

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

impl PartialEq for NSString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> BOOL;
        }

        let obj = self.as_id();
        let sel = selector!(isEqualToString:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) != 0 }
    }
}

impl Eq for NSString {}

impl PartialOrd for NSString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NSString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.compare(other).into()
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

    /// Creates an immutable string object from copying a slice.
    pub fn from_str(s: &str) -> NSString {
        let value: Self = Self(Self::class().alloc());

        extern "C" {
            fn objc_msgSend(
                obj: NSString,
                sel: SEL,
                bytes: *const u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
            ) -> NSString;
        }

        let obj = value;
        let sel = selector!(initWithBytes:length:encoding:);
        let bytes = s.as_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;

        unsafe { objc_msgSend(obj, sel, bytes, length, encoding) }
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

    // TODO: Other comparison methods:
    // - compare:options:
    // - compare:options:range:
    // - compare:options:range:locale:

    /// Compares the string and a given string using no options.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414082-compare).
    #[inline]
    pub fn compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> NSComparisonResult;
        }

        let obj = self.as_id();
        let sel = selector!(compare:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) }
    }

    /// Compares the string and a given string using a localized comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416999-localizedcompare).
    #[inline]
    pub fn localized_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> NSComparisonResult;
        }

        let obj = self.as_id();
        let sel = selector!(localizedCompare:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) }
    }

    /// Compares the string with a given string using `NSCaseInsensitiveSearch`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414769-caseinsensitivecompare).
    #[inline]
    pub fn case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> NSComparisonResult;
        }

        let obj = self.as_id();
        let sel = selector!(caseInsensitiveCompare:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) }
    }

    /// Compares the string with a given string using a case-insensitive,
    /// localized, comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1417333-localizedcaseinsensitivecompare).
    #[inline]
    pub fn localized_case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> NSComparisonResult;
        }

        let obj = self.as_id();
        let sel = selector!(localizedCaseInsensitiveCompare:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) }
    }

    /// Compares strings as sorted by the Finder.
    ///
    /// This method should be used whenever file names or other strings are
    /// presented in lists and tables where Finder-like sorting is appropriate.
    /// The exact sorting behavior of this method is different under different
    /// locales and may be changed in future releases. This method uses the
    /// current locale.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1409742-localizedstandardcompare).
    #[inline]
    pub fn localized_standard_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, other: id) -> NSComparisonResult;
        }

        let obj = self.as_id();
        let sel = selector!(localizedStandardCompare:);
        let other = other.as_id();

        unsafe { objc_msgSend(obj, sel, other) }
    }

    /// Returns `true` if the given string matches the beginning characters of
    /// this string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1410309-hasprefix).
    #[inline]
    pub fn has_prefix(&self, prefix: &NSString) -> bool {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, prefix: id) -> BOOL;
        }

        let obj = self.as_id();
        let sel = selector!(hasPrefix:);
        let prefix = prefix.as_id();

        unsafe { objc_msgSend(obj, sel, prefix) != 0 }
    }

    /// Returns `true` if the given string matches the ending characters of this
    /// string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416529-hassuffix).
    #[inline]
    pub fn has_suffix(&self, suffix: &NSString) -> bool {
        extern "C" {
            fn objc_msgSend(obj: id, sel: SEL, suffix: id) -> BOOL;
        }

        let obj = self.as_id();
        let sel = selector!(hasSuffix:);
        let suffix = suffix.as_id();

        unsafe { objc_msgSend(obj, sel, suffix) != 0 }
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

impl PartialEq for NSMutableString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        NSString::eq(self, other)
    }
}

impl PartialEq<NSString> for NSMutableString {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        (self as &NSString).eq(other)
    }
}

impl PartialEq<NSMutableString> for NSString {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        self.eq(other as &NSString)
    }
}

impl Eq for NSMutableString {}

impl PartialOrd for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<NSString> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<Ordering> {
        Some(NSString::cmp(self, other))
    }
}

impl PartialOrd<NSMutableString> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<Ordering> {
        Some(NSString::cmp(self, other))
    }
}

impl Ord for NSMutableString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        NSString::cmp(self, other)
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

    /// Creates a mutable string object from copying a slice.
    pub fn from_str(s: &str) -> NSMutableString {
        let value: Self = Self(NSString(Self::class().alloc()));

        extern "C" {
            fn objc_msgSend(
                obj: NSMutableString,
                sel: SEL,
                bytes: *const u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
            ) -> NSMutableString;
        }

        let obj = value;
        let sel = selector!(initWithBytes:length:encoding:);
        let bytes = s.as_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;

        unsafe { objc_msgSend(obj, sel, bytes, length, encoding) }
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
