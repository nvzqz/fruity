use super::{NSComparisonResult, NSRange};
use crate::objc::{Class, NSObject, NSUInteger, Object, ObjectType, BOOL, NO, SEL};
use std::{
    cmp::Ordering,
    ffi::CStr,
    fmt,
    ops::Deref,
    os::raw::c_char,
    ptr::{self, NonNull},
    slice, str,
};

#[macro_use]
mod macros;

mod encoding;

pub use encoding::*;

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

unsafe impl ObjectType for NSString {}

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

// Causes a linker error if not static.
static DEFAULT: NSString = ns_string!("");

impl Default for NSString {
    #[inline]
    fn default() -> Self {
        DEFAULT.clone()
    }
}

impl PartialEq for NSString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { _msg_send_cached![self, isEqualToString: other as &Object => BOOL] != 0 }
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

impl fmt::Debug for NSString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // SAFETY: The lifetime of `str` is very short.
        let str = unsafe { self.to_str() };

        str.fmt(f)
    }
}

impl fmt::Display for NSString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // SAFETY: The lifetime of `str` is very short.
        let str = unsafe { self.to_str() };

        str.fmt(f)
    }
}

impl fmt::Pointer for NSString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
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
}

/// Getting available encodings.
impl NSString {
    /// Returns a slice containing all supported encodings.
    ///
    /// The first time this is called, one pass is done to determine the length
    /// of the slice. The slice is then cached for subsequent calls.
    #[inline]
    pub fn available_encodings_slice() -> &'static [NSStringEncoding] {
        use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

        static CACHED: (AtomicPtr<NSStringEncoding>, AtomicUsize) = (
            AtomicPtr::new(ptr::null_mut()),
            AtomicUsize::new(0), // count
        );

        #[cold]
        fn slow_path() -> &'static [NSStringEncoding] {
            let start = NSString::available_encodings_ptr();

            let mut current = start;
            let mut count = 0;
            unsafe {
                // The end of the buffer is marked by a 0 encoding.
                while (*current).0 != 0 {
                    count += 1;
                    current = current.add(1);
                }
            }

            // The pointer must be stored second so that the fast path does not
            // read a length of 0.
            //
            // This is to prevent:
            //   A: store ptr
            //   B: read  ptr
            //   B: read  count
            //   A: store count
            CACHED.1.store(count, Ordering::Release);
            CACHED.0.store(start as *mut _, Ordering::Release);

            unsafe { slice::from_raw_parts(start, count) }
        }

        let cached_ptr = CACHED.0.load(Ordering::Acquire);
        if !cached_ptr.is_null() {
            let count = CACHED.1.load(Ordering::Acquire);
            return unsafe { slice::from_raw_parts(cached_ptr, count) };
        }

        slow_path()
    }

    /// Returns an iterator over all supported encodings.
    ///
    /// Unlike [`available_encodings_slice`](#method.available_encodings_slice),
    /// this is implemented lazily and does not perform caching.
    #[inline]
    pub fn available_encodings_iter() -> impl Iterator<Item = NSStringEncoding> {
        #[repr(transparent)]
        struct Iter(*const NSStringEncoding);

        unsafe impl Send for Iter {}
        unsafe impl Sync for Iter {}

        impl Iterator for Iter {
            type Item = NSStringEncoding;

            #[inline]
            fn next(&mut self) -> Option<NSStringEncoding> {
                let encoding = unsafe { *self.0 };
                if encoding.0 == 0 {
                    None
                } else {
                    unsafe { self.0 = self.0.add(1) };
                    Some(encoding)
                }
            }
        }

        // No more encodings are emitted after `None`.
        impl std::iter::FusedIterator for Iter {}

        Iter(Self::available_encodings_ptr())
    }

    /// Returns a pointer to a buffer containing all supported encodings.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1417579-availablestringencodings).
    #[inline]
    pub fn available_encodings_ptr() -> *const NSStringEncoding {
        unsafe { _msg_send![Self::class(), availableStringEncodings] }
    }

    /// Returns the number of supported encodings.
    ///
    /// The first time this is called, one pass is done to determine the number
    /// of encodings. The count is then cached for subsequent calls. This shares
    /// the same cache as
    /// [`available_encodings_slice`](#method.available_encodings_slice).
    #[inline]
    pub fn available_encodings_count() -> usize {
        Self::available_encodings_slice().len()
    }
}

impl NSString {
    // Shared non-inlined `from_str` implementation.
    //
    // This allows for reducing the code size of the final binary.
    unsafe fn _from_str(s: &str, class: &Class) -> NSString {
        let value: Self = class.alloc();

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
        let value: Self = Self::class().alloc();

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

    /// Returns a string representation of `range`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1415155-nsstringfromrange).
    #[inline]
    pub fn from_nsrange(range: NSRange) -> Self {
        extern "C" {
            fn NSStringFromRange(range: NSRange) -> NSString;
        }
        unsafe { NSStringFromRange(range) }
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

/// Getting contents as [UTF-8](https://en.wikipedia.org/wiki/UTF-8).
impl NSString {
    /// Returns a null-terminated UTF-8 representation of `self`, or null
    /// if the internal storage of `self` does not allow this to be returned
    /// efficiently.
    ///
    /// Unlike [`to_utf8_ptr`](#method.to_utf8_ptr.html), this does not allocate
    /// and construct a new UTF-8 C string if `self` does not represent one.
    ///
    /// This is retrieved using
    /// [`CFStringGetCStringPtr`](https://developer.apple.com/documentation/corefoundation/1542133-cfstringgetcstringptr)
    /// and
    /// [`kCFStringEncodingUTF8`](https://developer.apple.com/documentation/corefoundation/cfstringbuiltinencodings/kcfstringencodingutf8).
    #[inline]
    pub fn as_utf8_ptr(&self) -> *const c_char {
        type CFStringEncoding = u32;

        #[allow(non_upper_case_globals)]
        const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

        extern "C" {
            fn CFStringGetCStringPtr(s: &Object, encoding: CFStringEncoding) -> *const c_char;
        }

        unsafe { CFStringGetCStringPtr(self, kCFStringEncodingUTF8) }
    }

    /// Returns a null-terminated UTF-8 representation of `self`.
    ///
    /// This C string is a pointer to a structure inside `self`,
    /// which may have a lifetime shorter than the string object and will
    /// certainly not have a longer lifetime. Therefore, you should copy the C
    /// string if it needs to be stored outside of the memory context in which
    /// you use this property.
    ///
    /// This is retrieved using
    /// [`-[NSString UTF8String]`](https://developer.apple.com/documentation/foundation/nsstring/1411189-utf8string).
    #[inline]
    pub fn to_utf8_ptr(&self) -> *const c_char {
        unsafe { _msg_send![self, UTF8String] }
    }

    /// Returns the contents of `self` as a native UTF-8 string slice, or `None`
    /// if the internal storage of `self` does not allow this to be returned
    /// efficiently.
    ///
    /// Unlike [`to_str`](#method.to_str.html), this does not allocate and
    /// construct a new UTF-8 C string if `self` does not represent one.
    ///
    /// # Safety
    ///
    /// You must ensure that `self` is not mutated during the lifetime of the
    /// returned string slice.
    #[inline]
    pub unsafe fn as_str(&self) -> Option<&str> {
        let s = self.as_str_with_nul()?;

        // `CStr::to_bytes` does a checked slice conversion that emits a length
        // failure panic that'll never get called.
        Some(s.get_unchecked(..s.len() - 1))
    }

    /// Returns the contents of `self` as a native UTF-8 string slice.
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

    /// Returns the contents of `self` as a native UTF-8 string slice ending
    /// with a 0 byte, or `None` if the internal storage of `self` does not
    /// allow this to be returned efficiently.
    ///
    /// Unlike [`to_str_with_nul`](#method.to_str_with_nul.html), this does not
    /// allocate and construct a new UTF-8 C string if `self` does not represent
    /// one.
    ///
    /// # Safety
    ///
    /// You must ensure that `self` is not mutated during the lifetime of the
    /// returned string slice.
    #[inline]
    pub unsafe fn as_str_with_nul(&self) -> Option<&str> {
        let cstr = self.as_utf8_ptr();
        if cstr.is_null() {
            return None;
        }

        let cstr = CStr::from_ptr(cstr);
        Some(str::from_utf8_unchecked(cstr.to_bytes_with_nul()))
    }

    /// Returns the contents of `self` as a native UTF-8 string slice ending
    /// with a 0 byte.
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

    /// Returns the contents of `self` as a native UTF-8 string buffer.
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

    /// Returns the contents of `self` as a native UTF-8 string buffer ending
    /// with a 0 byte.
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
        unsafe { self.to_str_with_nul() }.into()
    }
}

impl NSString {
    /// Returns a selector with `self` as its name.
    ///
    /// If `self` cannot be converted to UTF-8 (this should be only due to
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
        unsafe { _msg_send![self, compare: other as &Object] }
    }

    /// Compares the string and a given string using a localized comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416999-localizedcompare).
    #[inline]
    pub fn localized_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, localizedCompare: other as &Object] }
    }

    /// Compares the string with a given string using `NSCaseInsensitiveSearch`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414769-caseinsensitivecompare).
    #[inline]
    pub fn case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, caseInsensitiveCompare: other as &Object] }
    }

    /// Compares the string with a given string using a case-insensitive,
    /// localized, comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1417333-localizedcaseinsensitivecompare).
    #[inline]
    pub fn localized_case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, localizedCaseInsensitiveCompare: other as &Object] }
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
        unsafe { _msg_send![self, localizedStandardCompare: other as &Object] }
    }

    /// Returns `true` if the given string matches the beginning characters of
    /// `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1410309-hasprefix).
    #[inline]
    pub fn has_prefix(&self, prefix: &NSString) -> bool {
        unsafe { _msg_send![self, hasPrefix: prefix as &Object => BOOL] != 0 }
    }

    /// Returns `true` if the given string matches the ending characters of this
    /// string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416529-hassuffix).
    #[inline]
    pub fn has_suffix(&self, suffix: &NSString) -> bool {
        unsafe { _msg_send![self, hasSuffix: suffix as &Object => BOOL] != 0 }
    }
}

/// A dynamic plain-text Unicode string object.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsmutablestring).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSMutableString(NSString);

unsafe impl ObjectType for NSMutableString {}

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

impl From<NSRange> for NSString {
    fn from(range: NSRange) -> Self {
        Self::from_nsrange(range)
    }
}

impl Deref for NSMutableString {
    type Target = NSString;

    #[inline]
    fn deref(&self) -> &NSString {
        &self.0
    }
}

impl Default for NSMutableString {
    #[inline]
    fn default() -> Self {
        DEFAULT.mutable_copy()
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

impl fmt::Debug for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &NSString).fmt(f)
    }
}

impl fmt::Display for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self as &NSString).fmt(f)
    }
}

impl fmt::Pointer for NSMutableString {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
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
        let value: Self = Self::class().alloc();

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
