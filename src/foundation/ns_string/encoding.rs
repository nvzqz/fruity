use crate::objc::NSUInteger;
use std::fmt;

/// Possible [`NSString`](struct.NSString.html) encodings.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsstringencoding).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NSStringEncoding(pub NSUInteger);

impl fmt::Debug for NSStringEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format like an enum.
        let name: &str = match *self {
            Self::ASCII => &"ASCII",
            Self::NEXTSTEP => &"NEXTSTEP",
            Self::JapaneseEUC => &"JapaneseEUC",
            Self::UTF8 => &"UTF8",
            Self::ISOLatin1 => &"ISOLatin1",
            Self::Symbol => &"Symbol",
            Self::NonLossyASCII => &"NonLossyASCII",
            Self::ShiftJIS => &"ShiftJIS",
            Self::ISOLatin2 => &"ISOLatin2",
            Self::WindowsCP1251 => &"WindowsCP1251",
            Self::WindowsCP1252 => &"WindowsCP1252",
            Self::WindowsCP1253 => &"WindowsCP1253",
            Self::WindowsCP1254 => &"WindowsCP1254",
            Self::WindowsCP1250 => &"WindowsCP1250",
            Self::ISO2022JP => &"ISO2022JP",
            Self::MacOSRoman => &"MacOSRoman",
            Self::UTF16 => &"UTF16", // Same as `Unicode`
            Self::UTF16BigEndian => &"UTF16BigEndian",
            Self::UTF16LittleEndian => &"UTF16LittleEndian",
            Self::UTF32 => &"UTF32",
            Self::UTF32BigEndian => &"UTF32BigEndian",
            Self::UTF32LittleEndian => &"UTF32LittleEndian",
            _ => {
                return f.debug_tuple("Other").field(&self.0).finish();
            }
        };
        fmt::Display::fmt(name, f)
    }
}

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
