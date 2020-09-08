//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.

mod bool;

pub use self::bool::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
