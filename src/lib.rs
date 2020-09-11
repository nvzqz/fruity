//! Rusty bindings for Apple libraries, brought to you by
//! [@NikolaiVazquez](https://twitter.com/NikolaiVazquez).
//!
//! # Index
//!
//! 1. [Donate](#donate)
//! 2. [Usage](#usage)
//!    1. [Feature Flags](#feature-flags)
//! 3. [Goals](#goals)
//!    1. [Idiomatic Rust](#idiomatic-rust)
//!    2. [Zero Cost](#zero-cost)
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
//! # Usage
//!
//! This library is available [on crates.io][crate] and can be used in your
//! project by adding the following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies.fruity]
//! version = "0.1.0"
//! ```
//!
//! ## Feature Flags
//!
//! Each module for a library or framework has its own
//! [feature flag](https://doc.rust-lang.org/cargo/reference/features.html)
//! with the same name.
//!
//! For example, this is how you enable the
//! [`foundation`](foundation/index.html) module:
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
//! runtime cost as writing the same code directly in Objective-C.
//!
//! This is true for the following:
//!
//! - **Calling object methods.**
//!
//!   Method dispatch is always direct and does not need the error checking
//!   overhead of other wrappers that use the
//!   [`objc::msg_send!`](https://docs.rs/objc/0.2.*/objc/macro.msg_send.html)
//!   macro. This also reduces the size of your program by not emitting panics
//!   that would otherwise never get called.
//!
//!   This library is carefully written to ensure that calls to
//!   [`objc_msgSend`](https://developer.apple.com/documentation/objectivec/1456712-objc_msgsend)
//!   are always done with the correct object type, method selector, and
//!   arguments.
//!
//! - **Getting a static class.**
//!
//!   Getters like
//!   [`NSString::class`](foundation/struct.NSString.html#method.class)
//!   retrieve the class directly through its symbol. This is instantaneous,
//!   especially when compared to calling into the Objective-C runtime via
//!   [`objc_getClass`](https://developer.apple.com/documentation/objectivec/1418952-objc_getclass).
//!
//! - **Creating an `NSString` from a Rust string literal.**
//!
//!   The [`nsstring!`](macro.nsstring.html)
//!   macro creates an `NSString` literal (i.e. `@"string"`) at compile time.
//!   There is no runtime dispatch/allocation/initialization cost.
//!
//! Some parts of this library are still not zero cost. Your help would be much
//! appreciated here!
//!
//! These are:
//!
//! - **The `selector!` macro.** See
//!   [issue #2](https://github.com/nvzqz/fruity/issues/2)
//!   for details.
//!
//! # License
//!
//! This project is released under either the
//! [MIT License](https://github.com/nvzqz/fruity/blob/main/LICENSE-MIT) or
//! [Apache License (Version 2.0)](https://github.com/nvzqz/fruity/blob/main/LICENSE-APACHE),
//! at your choosing.
//!
//! [crate]: https://crates.io/crates/fruity
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
