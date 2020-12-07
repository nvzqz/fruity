//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`objc`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "objc")]

#[macro_use]
pub(crate) mod sel;

#[macro_use]
mod macros;

#[macro_use]
mod msg;

mod autoreleasepool;
mod bool;
mod class;
mod class_type;
mod int;
mod ns_object;
mod objc_object;
mod object_type;

pub use self::bool::*;
pub use autoreleasepool::*;
pub use class::*;
pub use class_type::*;
pub use int::*;
pub use ns_object::*;
pub use objc_object::*;
pub use object_type::*;
pub use sel::SEL;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
