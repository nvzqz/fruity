use super::NSComparisonResult;
use crate::objc::{Class, NSObject, NSUInteger, Object, BOOL, NO, SEL};
use std::{cmp::Ordering, ffi::CStr, ops::Deref, os::raw::c_char, ptr::NonNull, str};

/// Returns the selector with a given name.
///
/// If the string cannot be converted to UTF-8 (this should be only due to
/// insufficient memory), this returns
/// [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
///
/// Use this function if you want your code to look more like Objective-C.
/// Otherwise, the [`to_selector`](struct.NSString.html#method.to_selector)
/// method should be preferred.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/1395294-nsselectorfromstring).
#[inline]
#[allow(non_snake_case)]
pub fn NSSelectorFromString(string: &NSString) -> Option<SEL> {
    extern "C" {
        fn NSSelectorFromString(string: &Object) -> Option<SEL>;
    }
    unsafe { NSSelectorFromString(string) }
}

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
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> BOOL;
        }

        let sel = selector!(isEqualToString:);

        unsafe { objc_msgSend(self, sel, other) != 0 }
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

impl From<&str> for NSString {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<&mut str> for NSString {
    #[inline]
    fn from(s: &mut str) -> Self {
        Self::from_str(s)
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

    /// Creates an immutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSString` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSObject::from_ptr(ptr))
    }

    /// Creates an immutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSString` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSObject::from_non_null_ptr(ptr))
    }

    // Shared non-inlined `from_str` implementation.
    //
    // This allows for reducing the code size of the final binary.
    unsafe fn _from_str(s: &str, class: &Class) -> NSString {
        let value: Self = Self(class.alloc());

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

        objc_msgSend(obj, sel, bytes, length, encoding)
    }

    /// Creates an immutable string object from copying a slice.
    #[inline]
    pub fn from_str(s: &str) -> NSString {
        unsafe { Self::_from_str(s, Self::class()) }
    }

    /// Creates an immutable string object without copying a slice.
    ///
    /// # Safety
    ///
    /// The returned string object or its clones must not outlive the referenced
    /// string slice.
    pub unsafe fn from_str_no_copy(s: &str) -> NSString {
        let value: Self = Self(Self::class().alloc());

        extern "C" {
            fn objc_msgSend(
                obj: NSString,
                sel: SEL,
                bytes: *const u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
                free_when_done: BOOL,
            ) -> NSString;
        }

        let obj = value;
        let sel = selector!(initWithBytesNoCopy:length:encoding:freeWhenDone:);
        let bytes = s.as_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;
        let free_when_done = NO;

        objc_msgSend(obj, sel, bytes, length, encoding, free_when_done)
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

    /// Returns a null-terminated UTF-8 representation of this string.
    ///
    /// This C string is a pointer to a structure inside this string object,
    /// which may have a lifetime shorter than the string object and will
    /// certainly not have a longer lifetime. Therefore, you should copy the C
    /// string if it needs to be stored outside of the memory context in which
    /// you use this property.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string).
    #[inline]
    pub fn to_utf8_ptr(&self) -> *const c_char {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL) -> *const c_char;
        }

        let sel = selector!(UTF8String);

        unsafe { objc_msgSend(self, sel) }
    }

    /// Returns the contents of this string object as a native UTF-8 string
    /// slice.
    ///
    /// This internally uses [`to_utf8_ptr`](#method.to_utf8_ptr). See its
    /// documentation for details.
    ///
    /// # Safety
    ///
    /// The lifetime of the returned string slice may be shorter than this
    /// object. Therefore, long use cases should copy the bytes of the returned
    /// string slice or use [`to_string`](#method.to_string).
    #[inline]
    pub unsafe fn to_str(&self) -> &str {
        let s = self.to_str_with_nul();

        // `CStr::to_bytes` does a checked slice conversion that emits a length
        // failure panic that'll never get called.
        s.get_unchecked(..s.len() - 1)
    }

    /// Returns the contents of this string object as a native UTF-8 string
    /// slice, containing a trailing 0 byte.
    ///
    /// This internally uses [`to_utf8_ptr`](#method.to_utf8_ptr). See its
    /// documentation for details.
    ///
    /// # Safety
    ///
    /// The lifetime of the returned string slice may be shorter than this
    /// object. Therefore, long use cases should copy the bytes of the returned
    /// string slice or use [`to_string_with_nul`](#method.to_string_with_nul).
    pub unsafe fn to_str_with_nul(&self) -> &str {
        let cstr = CStr::from_ptr(self.to_utf8_ptr());
        str::from_utf8_unchecked(cstr.to_bytes_with_nul())
    }

    /// Returns the contents of this string object as a native UTF-8 string
    /// buffer.
    ///
    /// This internally uses [`to_utf8_ptr`](#method.to_utf8_ptr). See its
    /// documentation for details.
    ///
    /// # Performance Considerations
    ///
    /// Because of how
    /// [`-[NSString UTF8String]`](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string)
    /// works, this method will likely allocate twice as much memory needed for
    /// the length of the resulting buffer. If your use case is short-lived
    /// enough, consider using [`to_str`](#method.to_str) to save memory and
    /// time.
    #[inline]
    pub fn to_string(&self) -> String {
        // This method relies on `to_string_with_nul` because that method
        // generates a lot of code that is best to only exist once.

        let mut string = self.to_string_with_nul();
        let len = string.len() - 1;

        // This approach is slightly faster than `String::pop`.
        //
        // SAFETY: The null character takes 1 byte in UTF-8.
        unsafe { string.as_mut_vec().set_len(len) };

        string
    }

    /// Returns the contents of this string object as a native UTF-8 string
    /// buffer, containing a trailing 0 byte.
    ///
    /// This internally uses [`to_utf8_ptr`](#method.to_utf8_ptr). See its
    /// documentation for details.
    ///
    /// # Performance Considerations
    ///
    /// Because of how
    /// [`-[NSString UTF8String]`](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string)
    /// works, this method will likely allocate twice as much memory needed for
    /// the length of the resulting buffer. If your use case is short-lived
    /// enough, consider using [`to_str_with_nul`](#method.to_str_with_nul) to
    /// save memory and time.
    pub fn to_string_with_nul(&self) -> String {
        // SAFETY: This use of the string is reasonably short-lived enough for
        // its lifetime to be long enough.
        unsafe { self.to_str() }.into()
    }

    /// Returns a selector with this string as its name.
    ///
    /// If this string cannot be converted to UTF-8 (this should be only due to
    /// insufficient memory), this returns
    /// [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1395294-nsselectorfromstring).
    #[inline]
    pub fn to_selector(&self) -> Option<SEL> {
        NSSelectorFromString(self)
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
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> NSComparisonResult;
        }

        let sel = selector!(compare:);

        unsafe { objc_msgSend(self, sel, other) }
    }

    /// Compares the string and a given string using a localized comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416999-localizedcompare).
    #[inline]
    pub fn localized_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> NSComparisonResult;
        }

        let sel = selector!(localizedCompare:);

        unsafe { objc_msgSend(self, sel, other) }
    }

    /// Compares the string with a given string using `NSCaseInsensitiveSearch`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414769-caseinsensitivecompare).
    #[inline]
    pub fn case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> NSComparisonResult;
        }

        let sel = selector!(caseInsensitiveCompare:);

        unsafe { objc_msgSend(self, sel, other) }
    }

    /// Compares the string with a given string using a case-insensitive,
    /// localized, comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1417333-localizedcaseinsensitivecompare).
    #[inline]
    pub fn localized_case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> NSComparisonResult;
        }

        let sel = selector!(localizedCaseInsensitiveCompare:);

        unsafe { objc_msgSend(self, sel, other) }
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
            fn objc_msgSend(obj: &Object, sel: SEL, other: &Object) -> NSComparisonResult;
        }

        let sel = selector!(localizedStandardCompare:);

        unsafe { objc_msgSend(self, sel, other) }
    }

    /// Returns `true` if the given string matches the beginning characters of
    /// this string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1410309-hasprefix).
    #[inline]
    pub fn has_prefix(&self, prefix: &NSString) -> bool {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, prefix: &Object) -> BOOL;
        }

        let sel = selector!(hasPrefix:);

        unsafe { objc_msgSend(self, sel, prefix) != 0 }
    }

    /// Returns `true` if the given string matches the ending characters of this
    /// string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416529-hassuffix).
    #[inline]
    pub fn has_suffix(&self, suffix: &NSString) -> bool {
        extern "C" {
            fn objc_msgSend(obj: &Object, sel: SEL, suffix: &Object) -> BOOL;
        }

        let sel = selector!(hasSuffix:);

        unsafe { objc_msgSend(self, sel, suffix) != 0 }
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

impl From<&str> for NSMutableString {
    #[inline]
    fn from(s: &str) -> Self {
        Self::from_str(s)
    }
}

impl From<&mut str> for NSMutableString {
    #[inline]
    fn from(s: &mut str) -> Self {
        Self::from_str(s)
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

    /// Creates a mutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSMutableString` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSString::from_ptr(ptr))
    }

    /// Creates a mutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSMutableString` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSString::from_non_null_ptr(ptr))
    }

    /// Creates a mutable string object from copying a slice.
    #[inline]
    pub fn from_str(s: &str) -> NSMutableString {
        unsafe { Self(NSString::_from_str(s, Self::class())) }
    }

    /// Creates a mutable string object without copying a slice.
    ///
    /// # Safety
    ///
    /// The returned string object or its clones must not outlive the referenced
    /// string slice.
    pub unsafe fn from_str_no_copy(s: &mut str) -> NSMutableString {
        let value: Self = Self(NSString(Self::class().alloc()));

        extern "C" {
            fn objc_msgSend(
                obj: NSMutableString,
                sel: SEL,
                bytes: *mut u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
                free_when_done: BOOL,
            ) -> NSMutableString;
        }

        let obj = value;
        let sel = selector!(initWithBytesNoCopy:length:encoding:freeWhenDone:);
        let bytes = s.as_mut_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;
        let free_when_done = NO;

        objc_msgSend(obj, sel, bytes, length, encoding, free_when_done)
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
