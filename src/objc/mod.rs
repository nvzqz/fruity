//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`objc`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "objc")]

pub mod sys;

#[macro_use]
pub(crate) mod sel;

#[macro_use]
mod macros;

#[macro_use]
mod macros_pub;

#[macro_use]
mod msg;

mod autoreleasepool;
mod bool;
mod class;
mod class_type;
mod image_info;
mod int;
mod ivar;
mod method;
mod ns_object;
mod objc_object;
mod object_type;
mod property;
mod type_encoding;

pub use self::bool::*;
pub use autoreleasepool::*;
pub use class::*;
pub use class_type::*;
pub use image_info::*;
pub use int::*;
pub use ivar::*;
pub use method::*;
pub use ns_object::*;
pub use objc_object::*;
pub use object_type::*;
pub use property::*;
pub use sel::Sel;
pub use type_encoding::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
