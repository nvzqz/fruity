//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.

mod bool;
mod int;

pub use self::bool::*;
pub use int::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
