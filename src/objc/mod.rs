//! [Objective-C](https://developer.apple.com/documentation/objectivec) library.

#[macro_use]
mod sel;

mod bool;
mod class;
mod int;
mod obj;

pub use self::bool::*;
pub use class::*;
pub use int::*;
pub use obj::*;
pub use sel::*;

#[link(name = "objc", kind = "dylib")]
extern "C" {}
