//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`objc`**
//! [feature flag](../index.html#feature-flags).

#[macro_use]
mod sel;

mod bool;
mod class;
mod int;
mod nsobject;
mod obj;

pub use self::bool::*;
pub use class::*;
pub use int::*;
pub use nsobject::*;
pub use obj::*;
pub use sel::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
