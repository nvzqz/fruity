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
mod msg;

mod autoreleasepool;
mod bool;
mod class;
mod int;
mod ns_object;
mod obj;
mod unretained;

pub use self::bool::*;
pub use autoreleasepool::*;
pub use class::*;
pub use int::*;
pub use ns_object::*;
pub use obj::*;
pub use sel::SEL;
pub use unretained::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
