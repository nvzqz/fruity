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
//! version = "0.2.0"
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
//! version = "0.2.0"
//! features = ["foundation"]
//! ```
//!
//! This feature transitively enables the [`objc`](objc/index.html) and
//! [`core_graphics`](core_graphics/index.html) features/modules.
//!
//! # Goals
//!
//! ## Idiomatic Rust
//!
//! Fruity makes interfacing with these C and Objective-C APIs feel natural in
//! Rust.
//!
//! - **Automatic Reference Counting.**
//!
//!   Fruity takes advantage of Rust's
//!   [ownership model](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
//!   to handle object reference counting for you.
//!
//!   [`NSObject`](objc/struct.NSObject.html)
//!   is a smart pointer that calls
//!   [`retain`](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1571946-retain)
//!   on [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) and
//!   [`release`](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1571957-release)
//!   on [`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html). This is
//!   exactly how Rust's
//!   [`Arc<T>`](https://doc.rust-lang.org/std/sync/struct.Arc.html) works.
//!
//! - **`Option<NSObject>`.**
//!
//!   In Objective-C, all objects are nullable unless marked with `_Nonnull`.
//!   This often leads to either very defensive checks or careless ignoring of
//!   null objects.
//!
//!   Fruity reverses that and instead makes all objects (such as
//!   [`NSObject`](objc/struct.NSObject.html))
//!   non-null by default. An object can be made nullable by wrapping it with
//!   [`Option<T>`](https://doc.rust-lang.org/std/option/enum.Option.html).
//!
//!   To make FFI safe and easy, the following Objective-C and Rust types are
//!   ABI-compatible:
//!
//!   - `NSObject * _Nonnull` and `NSObject`
//!
//!   - `NSObject * _Nullable` and `Option<NSObject>`
//!
//!   This is because
//!   [`NSObject`](objc/struct.NSObject.html)
//!   is a
//!   [`#[repr(transparent)]`](https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent)
//!   wrapper around a
//!   [`NonNull<T>`](https://doc.rust-lang.org/std/ptr/struct.NonNull.html)
//!   pointer.
//!
//! - **`Result<T, NSError>`.**
//!
//!   In Objective-C, methods take a pointer to where an
//!   [`NSError`](https://developer.apple.com/documentation/foundation/nserror)
//!   is placed upon failure. This makes it easy to avoid error handling and
//!   assume the happy path, which can lead to bugs when errors occur.
//!
//!   Fruity instead returns a
//!   [`Result`](https://doc.rust-lang.org/std/result/enum.Result.html), which
//!   is the canonical way to handle errors in Rust. This ensures that errors
//!   must be acknowledged in some way.
//!
//! - **Natural inheritance.**
//!
//!   Most of these types are classes that inherit from each other. Because true
//!   inheritance is not possible in Rust, Fruity uses
//!   [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
//!   to model Objective-C subclassing.
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
//!   The [`ns_string!`](macro.ns_string.html)
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

// This crate is only available for 32/64 bit x86 and ARM systems.
#![cfg(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "arm",
    target_arch = "aarch64",
))]
#![deny(improper_ctypes)]
#![warn(missing_docs)]

pub(crate) mod common;

#[macro_use]
pub mod core;

// This module is not for public use. It is an implementation detail of macros
// exposed by this crate.
#[doc(hidden)]
pub mod _priv;

#[macro_use]
pub mod objc;

pub mod app_kit;
pub mod core_animation;
pub mod core_foundation;
pub mod core_graphics;
pub mod dispatch;
pub mod foundation;
pub mod ui_kit;
