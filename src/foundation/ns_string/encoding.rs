use super::NSString;
use crate::objc::{NSUInteger, Unretained};
use std::fmt;

/// Possible [`NSString`](struct.NSString.html) encodings.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsstringencoding).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NSStringEncoding(pub NSUInteger);

impl fmt::Debug for NSStringEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // SAFETY: The unretained string does not live long.
        if let Some(name) = unsafe { self.name_unretained() } {
            name.fmt(f)
        } else {
            f.debug_tuple("Unknown").field(&self.0).finish()
        }
    }
}

impl NSStringEncoding {
    /// Returns the canonical name of this string encoding.
    ///
    /// This is retrieved using
    /// [`CFStringGetNameOfEncoding`](https://developer.apple.com/documentation/corefoundation/1543585-cfstringgetnameofencoding).
    #[inline]
    pub fn name(&self) -> Option<NSString> {
        // SAFETY: The string is immediately retained.
        let name = unsafe { self.name_unretained()? };
        Some(NSString::clone(&name))
    }

    // SAFETY: The string is created using "The Get Rule", so it should be
    // retained for long uses.
    #[inline]
    pub(crate) unsafe fn name_unretained<'a>(&self) -> Option<Unretained<'a, NSString>> {
        type CFStringEncoding = u32;

        extern "C" {
            fn CFStringConvertNSStringEncodingToEncoding(enc: NSStringEncoding)
                -> CFStringEncoding;
            fn CFStringGetNameOfEncoding<'a>(
                enc: CFStringEncoding,
            ) -> Option<Unretained<'a, NSString>>;
        }

        CFStringGetNameOfEncoding(CFStringConvertNSStringEncodingToEncoding(*self))
    }
}

impl NSStringEncoding {
    /// Strict 7-bit ASCII encoding within 8-bit chars; ASCII values 0…127 only.
    pub const ASCII: Self = Self(1);

    /// 7-bit verbose ASCII to represent all Unicode characters.
    pub const ASCII_NON_LOSSY: Self = Self(7);

    /// The canonical Unicode encoding for string objects.
    ///
    /// This is an alias for [`UTF16`](#associatedconstant.UTF16).
    pub const UNICODE: Self = Self::UTF16;

    /// An 8-bit representation of Unicode characters, suitable for transmission
    /// or storage by ASCII-based systems.
    pub const UTF8: Self = Self(4);

    /// 16-bit UTF encoding.
    pub const UTF16: Self = Self(10);

    /// [`UTF16`](#associatedconstant.UTF16) encoding with explicit big
    /// endianness specified.
    pub const UTF16_BE: Self = Self(0x90000100);

    /// [`UTF16`](#associatedconstant.UTF16) encoding with explicit little
    /// endianness specified.
    pub const UTF16_LE: Self = Self(0x94000100);

    /// 32-bit UTF encoding.
    pub const UTF32: Self = Self(0x8c000100);

    /// [`UTF32`](#associatedconstant.UTF32) encoding with explicit big
    /// endianness specified.
    pub const UTF32_BE: Self = Self(0x98000100);

    /// [`UTF32`](#associatedconstant.UTF32) encoding with explicit little
    /// endianness specified.
    pub const UTF32_LE: Self = Self(0x9c000100);

    /// 8-bit ASCII encoding with NextStep extensions.
    pub const NEXT_STEP: Self = Self(2);

    /// 8-bit ISO Latin 1 encoding.
    pub const ISO_LATIN1: Self = Self(5);

    /// 8-bit ISO Latin 2 encoding.
    pub const ISO_LATIN2: Self = Self(9);

    /// 8-bit Adobe Symbol encoding vector.
    pub const SYMBOL: Self = Self(6);

    /// 8-bit Shift-JIS encoding for Japanese text.
    pub const SHIFT_JIS: Self = Self(8);

    /// 8-bit EUC encoding for Japanese text.
    pub const JP_EUC: Self = Self(3);

    /// ISO 2022 Japanese encoding for email.
    pub const JP_ISO2022: Self = Self(21);

    /// Classic Macintosh Roman encoding.
    pub const MAC_ROMAN: Self = Self(30);

    /// Microsoft Windows codepage 1251, encoding Cyrillic characters;
    /// equivalent to AdobeStandardCyrillic font encoding.
    pub const WINDOWS_CP1251: Self = Self(11);

    /// Microsoft Windows codepage 1252; equivalent to WinLatin1.
    pub const WINDOWS_CP1252: Self = Self(12);

    /// Microsoft Windows codepage 1253, encoding Greek characters.
    pub const WINDOWS_CP1253: Self = Self(13);

    /// Microsoft Windows codepage 1254, encoding Turkish characters.
    pub const WINDOWS_CP1254: Self = Self(14);

    /// Microsoft Windows codepage 1250; equivalent to WinLatin2.
    pub const WINDOWS_CP1250: Self = Self(15);
}
