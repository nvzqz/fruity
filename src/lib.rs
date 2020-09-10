//! Rusty bindings for Apple libraries, brought to you by
//! [@NikolaiVazquez](https://twitter.com/NikolaiVazquez).

// This crate is only available for 32 and 64 bit systems.
#![cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#![deny(improper_ctypes)]
#![warn(missing_docs)]

#[macro_use]
pub mod objc;

pub mod core_foundation;
pub mod foundation;

// This module is not for public use. It is an implementation detail of macros
// exposed by this crate.
#[doc(hidden)]
pub mod _priv {
    pub use std;

    #[repr(C)]
    pub struct __CFString {
        pub isa: *const std::ffi::c_void,
        pub flags: usize,
        pub data: *const u8,
        pub len: usize,
    }

    unsafe impl Send for __CFString {}
    unsafe impl Sync for __CFString {}

    /// Returns `true` if the string is a valid C string.
    pub const fn is_cstr(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }

        let bytes = s.as_bytes();
        let mut i = 0;
        let last_i = bytes.len() - 1;
        loop {
            if bytes[i] == 0 {
                return i == last_i;
            }

            i += 1;

            if i > last_i {
                return false;
            }
        }
    }

    #[cfg(test)]
    mod tests {

        #[test]
        fn is_cstr() {
            let cases: &[_] = &[
                ("\0", true),
                ("\0\0", false),
                ("\0\0\0", false),
                ("", false),
                ("a", false),
                ("a\0", true),
                ("\0a\0", false),
                ("a\0\0", false),
            ];

            for &(s, is_cstr) in cases {
                assert_eq!(super::is_cstr(s), is_cstr, "invalid result for {:?}", s);
            }
        }
    }
}
