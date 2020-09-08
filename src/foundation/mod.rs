//! [Foundation](https://developer.apple.com/documentation/foundation) framework.

mod nsstring;

pub use nsstring::*;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}
