//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.

mod bool;
mod class;
mod int;
mod obj;

pub use self::bool::*;
pub use class::*;
pub use int::*;
pub use obj::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
