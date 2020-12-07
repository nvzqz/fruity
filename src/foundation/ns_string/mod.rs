use super::{NSComparisonResult, NSRange};
use crate::core::Arc;
use crate::objc::{Class, ClassType, NSObject, NSUInteger, BOOL, SEL};
use std::{cmp::Ordering, ffi::CStr, fmt, os::raw::c_char, ptr, slice, str};

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
        fn NSSelectorFromString(string: &NSString) -> Option<SEL>;
    }
    unsafe { NSSelectorFromString(string) }
}

objc_subclass! {
    /// A static, plain-text Unicode string object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring).
    pub class NSString: NSObject;
}

impl Default for &NSString {
    #[inline]
    fn default() -> Self {
        ns_string!("")
    }
}

impl PartialEq for NSString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { _msg_send_cached![self, isEqualToString: other => BOOL] }.into()
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

impl PartialEq<str> for NSString {
    fn eq(&self, other: &str) -> bool {
        // SAFETY: This instance is not mutated while the UTF-16 slice exists.
        if let Some(this) = unsafe { self.as_utf16() } {
            let mut this_iter = this.iter();
            let mut other_iter = other.encode_utf16();
            loop {
                match (this_iter.next(), other_iter.next()) {
                    (Some(&this), Some(other)) if this == other => continue,
                    (None, None) => return true,
                    (_, _) => return false,
                }
            }
        } else {
            // If the string is not UTF-16, then it is UTF-8 (or some other
            // encoding?).

            // SAFETY: `this` is short-lived.
            let this = unsafe { self.to_str() };

            this == other
        }
    }
}

impl PartialEq<&str> for NSString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        *self == **other
    }
}

impl PartialEq<NSString> for str {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        other == self
    }
}

impl PartialEq<NSString> for &str {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        other == self
    }
}

impl PartialOrd<str> for NSString {
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        // SAFETY: This instance is not mutated while the UTF-16 slice exists.
        if let Some(this) = unsafe { self.as_utf16() } {
            let mut this_iter = this.iter();
            let mut other_iter = other.encode_utf16();
            loop {
                match (this_iter.next(), other_iter.next()) {
                    (Some(&this), Some(other)) => match this.cmp(&other) {
                        Ordering::Equal => continue,
                        ord => return Some(ord),
                    },
                    (Some(_), None) => return Some(Ordering::Greater),
                    (None, Some(_)) => return Some(Ordering::Less),
                    (None, None) => return Some(Ordering::Equal),
                }
            }
        } else {
            // If the string is not UTF-16, then it is UTF-8 (or some other
            // encoding?).

            // SAFETY: `this` is short-lived.
            let this = unsafe { self.to_str() };

            Some(this.cmp(other))
        }
    }
}

impl PartialOrd<&str> for NSString {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        self.partial_cmp(*other)
    }
}

impl PartialOrd<NSString> for str {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<Ordering> {
        Some(other.partial_cmp(self)?.reverse())
    }
}

impl PartialOrd<NSString> for &str {
    #[inline]
    fn partial_cmp(&self, other: &NSString) -> Option<Ordering> {
        Some(other.partial_cmp(self)?.reverse())
    }
}

impl From<&str> for Arc<NSString> {
    #[inline]
    fn from(s: &str) -> Self {
        NSString::from_str(s)
    }
}

impl From<&mut str> for Arc<NSString> {
    #[inline]
    fn from(s: &mut str) -> Self {
        NSString::from_str(s)
    }
}

impl From<NSRange> for Arc<NSString> {
    #[inline]
    fn from(range: NSRange) -> Self {
        NSString::from_nsrange(range)
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

/// Getting available encodings.
impl NSString {
    /// Returns a slice containing all supported encodings.
    ///
    /// The first time this is called, one pass is done to determine the length
    /// of the slice. The slice is then cached for subsequent calls.
    #[inline]
    #[doc(alias = "availableStringEncodings")]
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
    #[doc(alias = "availableStringEncodings")]
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
    #[doc(alias = "availableStringEncodings")]
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
    #[doc(alias = "availableStringEncodings")]
    pub fn available_encodings_count() -> usize {
        Self::available_encodings_slice().len()
    }
}

impl NSString {
    // Shared non-inlined `from_str` implementation.
    //
    // This allows for reducing the code size of the final binary.
    unsafe fn _from_str(s: &str, class: &Class) -> Arc<NSString> {
        let value: Arc<Self> = class.alloc();

        #[allow(clashing_extern_declarations)]
        extern "C" {
            fn objc_msgSend(
                obj: Arc<NSString>,
                sel: SEL,
                bytes: *const u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
            ) -> Arc<NSString>;
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
    #[doc(alias = "initWithBytes")]
    #[doc(alias = "initWithBytes:length:encoding:")]
    pub fn from_str(s: &str) -> Arc<NSString> {
        unsafe { Self::_from_str(s, Self::class()) }
    }

    /// Creates an immutable string object without copying a slice.
    ///
    /// # Safety
    ///
    /// The returned string object or its clones must not outlive the referenced
    /// string slice.
    #[doc(alias = "initWithBytesNoCopy")]
    #[doc(alias = "initWithBytesNoCopy:length:encoding:freeWhenDone:")]
    pub unsafe fn from_str_no_copy(s: &str) -> Arc<NSString> {
        let value: Arc<Self> = Self::class().alloc();

        #[allow(clashing_extern_declarations)]
        extern "C" {
            fn objc_msgSend(
                obj: Arc<NSString>,
                sel: SEL,
                bytes: *const u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
                free_when_done: BOOL,
            ) -> Arc<NSString>;
        }

        let obj = value;
        let sel = selector!(initWithBytesNoCopy:length:encoding:freeWhenDone:);
        let bytes = s.as_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;
        let free_when_done = BOOL::NO;

        objc_msgSend(obj, sel, bytes, length, encoding, free_when_done)
    }

    /// Returns a string representation of `range`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/1415155-nsstringfromrange).
    #[inline]
    #[doc(alias = "NSStringFromRange")]
    pub fn from_nsrange(range: NSRange) -> Arc<Self> {
        extern "C" {
            fn NSStringFromRange(range: NSRange) -> Arc<NSString>;
        }
        unsafe { NSStringFromRange(range) }
    }

    /// Returns a copy of this object using
    /// [`NSCopying`](https://developer.apple.com/documentation/foundation/nscopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418807-copy).
    #[inline]
    pub fn copy(&self) -> Arc<NSString> {
        let copy = NSObject::copy(self);
        unsafe { Arc::cast_unchecked(copy) }
    }

    /// Returns a copy of this object using
    /// [`NSMutableCopying`](https://developer.apple.com/documentation/foundation/nsmutablecopying).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418978-mutablecopy).
    #[inline]
    pub fn mutable_copy(&self) -> Arc<NSMutableString> {
        let copy = NSObject::mutable_copy(self);
        unsafe { Arc::cast_unchecked(copy) }
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
    #[doc(alias = "CFStringGetCStringPtr")]
    pub fn as_utf8_ptr(&self) -> *const c_char {
        type CFStringEncoding = u32;

        #[allow(non_upper_case_globals)]
        const kCFStringEncodingUTF8: CFStringEncoding = 0x08000100;

        extern "C" {
            fn CFStringGetCStringPtr(s: &NSString, encoding: CFStringEncoding) -> *const c_char;
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

/// Getting contents as [UTF-16](https://en.wikipedia.org/wiki/UTF-16).
impl NSString {
    /// Returns a pointer to the UTF-16 representation of `self`, or null if the
    /// internal storage of `self` does not allow this to be returned
    /// efficiently.
    ///
    /// This is retrieved using
    /// [`CFStringGetCharactersPtr`](https://developer.apple.com/documentation/corefoundation/1542939-cfstringgetcharactersptr)
    ///
    /// See [`as_utf8_ptr`](#method.as_utf8_ptr) for the UTF-8 equivalent.
    #[inline]
    #[doc(alias = "CFStringGetCharactersPtr")]
    pub fn as_utf16_ptr(&self) -> *const u16 {
        extern "C" {
            fn CFStringGetCharactersPtr(s: &NSString) -> *const u16;
        }
        unsafe { CFStringGetCharactersPtr(self) }
    }

    /// Returns the contents of `self` as a UTF-16 string slice, or `None` if
    /// the internal storage of `self` does not allow this to be returned
    /// efficiently.
    ///
    /// See [`as_str`](#method.as_str) for the UTF-8 equivalent.
    ///
    /// # Safety
    ///
    /// You must ensure that `self` is not mutated during the lifetime of the
    /// returned slice.
    #[inline]
    pub unsafe fn as_utf16(&self) -> Option<&[u16]> {
        let ptr = self.as_utf16_ptr();
        if ptr.is_null() {
            return None;
        }

        Some(slice::from_raw_parts(ptr, self.length()))
    }
}

impl NSString {
    /// Returns the number of UTF-16 code units in `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414212-length).
    #[inline]
    pub fn length(&self) -> NSUInteger {
        unsafe { _msg_send![self, length] }
    }

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
        unsafe { _msg_send![self, compare: other] }
    }

    /// Compares the string and a given string using a localized comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416999-localizedcompare).
    #[inline]
    #[doc(alias = "localizedCompare")]
    pub fn localized_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, localizedCompare: other] }
    }

    /// Compares the string with a given string using `NSCaseInsensitiveSearch`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1414769-caseinsensitivecompare).
    #[inline]
    #[doc(alias = "caseInsensitiveCompare")]
    pub fn case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, caseInsensitiveCompare: other] }
    }

    /// Compares the string with a given string using a case-insensitive,
    /// localized, comparison.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1417333-localizedcaseinsensitivecompare).
    #[inline]
    #[doc(alias = "localizedCaseInsensitiveCompare")]
    pub fn localized_case_insensitive_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, localizedCaseInsensitiveCompare: other] }
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
    #[doc(alias = "localizedStandardCompare")]
    pub fn localized_standard_compare(&self, other: &NSString) -> NSComparisonResult {
        unsafe { _msg_send![self, localizedStandardCompare: other] }
    }

    /// Returns `true` if the given string matches the beginning characters of
    /// `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1410309-hasprefix).
    #[inline]
    #[doc(alias = "hasPrefix")]
    pub fn has_prefix(&self, prefix: &NSString) -> bool {
        unsafe { _msg_send![self, hasPrefix: prefix => BOOL] }.into()
    }

    /// Returns `true` if the given string matches the ending characters of this
    /// string.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsstring/1416529-hassuffix).
    #[inline]
    #[doc(alias = "hasSuffix")]
    pub fn has_suffix(&self, suffix: &NSString) -> bool {
        unsafe { _msg_send![self, hasSuffix: suffix => BOOL] }.into()
    }
}

objc_subclass! {
    /// A dynamic plain-text Unicode string object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsmutablestring).
    pub class NSMutableString: NSString;
}

impl Default for Arc<NSMutableString> {
    #[inline]
    fn default() -> Self {
        unsafe { NSMutableString::class().alloc_init() }
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

impl PartialEq<str> for NSMutableString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        NSString::eq(self, other)
    }
}

impl PartialEq<&str> for NSMutableString {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        NSString::eq(self, other)
    }
}

impl PartialEq<NSMutableString> for str {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        other == self
    }
}

impl PartialEq<NSMutableString> for &str {
    #[inline]
    fn eq(&self, other: &NSMutableString) -> bool {
        other == self
    }
}

impl PartialOrd<str> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        NSString::partial_cmp(self, other)
    }
}

impl PartialOrd<&str> for NSMutableString {
    #[inline]
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        NSString::partial_cmp(self, other)
    }
}

impl PartialOrd<NSMutableString> for str {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<Ordering> {
        Some(other.partial_cmp(self)?.reverse())
    }
}

impl PartialOrd<NSMutableString> for &str {
    #[inline]
    fn partial_cmp(&self, other: &NSMutableString) -> Option<Ordering> {
        Some(other.partial_cmp(self)?.reverse())
    }
}

impl From<&str> for Arc<NSMutableString> {
    #[inline]
    fn from(s: &str) -> Self {
        NSMutableString::from_str(s)
    }
}

impl From<&mut str> for Arc<NSMutableString> {
    #[inline]
    fn from(s: &mut str) -> Self {
        NSMutableString::from_str(s)
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

impl NSMutableString {
    /// Creates a mutable string object from copying a slice.
    #[inline]
    pub fn from_str(s: &str) -> Arc<NSMutableString> {
        unsafe { Arc::cast_unchecked(NSString::_from_str(s, Self::class())) }
    }

    /// Creates a mutable string object without copying a slice.
    ///
    /// # Safety
    ///
    /// The returned string object or its clones must not outlive the referenced
    /// string slice.
    pub unsafe fn from_str_no_copy(s: &mut str) -> Arc<NSMutableString> {
        let value: Arc<Self> = Self::class().alloc();

        #[allow(clashing_extern_declarations)]
        extern "C" {
            fn objc_msgSend(
                obj: Arc<NSMutableString>,
                sel: SEL,
                bytes: *mut u8,
                length: NSUInteger,
                encoding: NSStringEncoding,
                free_when_done: BOOL,
            ) -> Arc<NSMutableString>;
        }

        let obj = value;
        let sel = selector!(initWithBytesNoCopy:length:encoding:freeWhenDone:);
        let bytes = s.as_mut_ptr();
        let length = s.len();
        let encoding = NSStringEncoding::UTF8;
        let free_when_done = BOOL::NO;

        objc_msgSend(obj, sel, bytes, length, encoding, free_when_done)
    }
}
