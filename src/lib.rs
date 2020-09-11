//! Rusty bindings for Apple libraries, brought to you by
//! [@NikolaiVazquez](https://twitter.com/NikolaiVazquez).
//!
//! # Index
//!
//! 1. [Donate](#donate)
//! 2. [Goals](#goals)
//!    1. [Idiomatic Rust](#idiomatic-rust)
//!    2. [Zero Cost](#zero-cost)
//! 3. [Feature Flags](#feature-flags)
//! 4. [License](#license)
//! 5. [Modules](#modules)
//! 6. [Macros](#macros)
//!
//! # Donate
//!
//! If this project is useful to you, consider
//! [sponsoring me](https://github.com/sponsors/nvzqz) or
//! [donating directly](https://www.paypal.me/nvzqz)!
//!
//! Doing so enables me to create high-quality open source software like this.
//! ❤️
//!
//! # Goals
//!
//! ## Idiomatic Rust
//!
//! Fruity makes interfacing with these C and Objective-C APIs feel natural in
//! Rust.
//!
//! Most of these types are classes that inherit from each other. Because Rust
//! does not have inheritance and instead prefers composition, this crate uses
//! [`Deref`] to fake inheritance.
//!
//! [`Deref`]: https://doc.rust-lang.org/std/ops/trait.Deref.html
//!
//! ## Zero Cost
//!
//! Using Fruity to interface with Objective-C libraries should have as little
//! runtime cost as writing the equivalent code directly in Objective-C.
//!
//! # Feature Flags
//!
//! Each module for a library or framework has its own
//! [feature flag](https://doc.rust-lang.org/cargo/reference/features.html)
//! with the same name.
//!
//! For example, this is how you enable the
//! [`foundation`](foundation/index.html) module via [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies.fruity]
//! version = "0.1.0"
//! features = ["foundation"]
//! ```
//!
//! This feature transitively enables the [`objc`](objc/index.html)
//! feature/module.
//!
//! # License
//!
//! This project is released under either:
//!
//! - [MIT License](https://github.com/nvzqz/fruity/blob/main/LICENSE-MIT)
//! - [Apache License (Version 2.0)](https://github.com/nvzqz/fruity/blob/main/LICENSE-APACHE)
//!
//! at your choosing.
//!
//! [`Cargo.toml`]: https://doc.rust-lang.org/cargo/reference/manifest.html

// This crate is only available for 32 and 64 bit systems.
#![cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#![deny(improper_ctypes)]
#![warn(missing_docs)]

#[macro_use]
#[cfg(feature = "objc")]
pub mod objc;

#[cfg(feature = "core_foundation")]
pub mod core_foundation;
#[cfg(feature = "foundation")]
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
