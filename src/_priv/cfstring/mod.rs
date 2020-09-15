use std::ffi::c_void;

pub mod utf16;

// From `CFString.c`:
// > !!! Note: Constant CFStrings use the bit patterns:
// > C8 (11001000 = default allocator, not inline, not freed contents; 8-bit; has NULL byte; doesn't have length; is immutable)
// > D0 (11010000 = default allocator, not inline, not freed contents; Unicode; is immutable)
// > The bit usages should not be modified in a way that would effect these bit patterns.
//
// The 7 byte is the `CFTypeID` of `CFStringRef`.
const FLAGS_ASCII: usize = 0x07_C8;
const FLAGS_UTF16: usize = 0x07_D0;

#[repr(C)]
pub struct CFStringAscii {
    isa: *const c_void,
    flags: usize,
    data: *const u8,
    len: usize,
}

// Required to place in a `static`.
unsafe impl Sync for CFStringAscii {}

impl CFStringAscii {
    pub const fn new(isa: *const c_void, data: *const u8, len: usize) -> Self {
        Self {
            isa,
            data,
            len,
            flags: FLAGS_ASCII,
        }
    }

    pub const fn as_ptr(&self) -> *const c_void {
        self as *const Self as *const c_void
    }
}

#[repr(C)]
pub struct CFStringUtf16 {
    isa: *const c_void,
    flags: usize,
    data: *const u16,
    len: usize,
}

// Required to place in a `static`.
unsafe impl Sync for CFStringUtf16 {}

impl CFStringUtf16 {
    pub const fn new(isa: *const c_void, data: *const u16, len: usize) -> Self {
        Self {
            isa,
            data,
            len,
            flags: FLAGS_UTF16,
        }
    }

    pub const fn as_ptr(&self) -> *const c_void {
        self as *const Self as *const c_void
    }
}

/// Returns `s` with any 0 byte at the end removed.
pub const fn trim_trailing_nul(s: &str) -> &[u8] {
    match s.as_bytes() {
        [b @ .., 0] => b,
        b => b,
    }
}

/// Returns `true` if `bytes` is entirely ASCII with no interior nulls.
pub const fn is_ascii(bytes: &[u8]) -> bool {
    let mut i = 0;
    loop {
        if i == bytes.len() {
            return true;
        }

        let byte = bytes[i];
        if !byte.is_ascii() || byte == 0 {
            return false;
        }

        i += 1;
    }
}
