//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.

mod bool;
mod class;
mod int;

pub use self::bool::*;
pub use class::*;
pub use int::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
