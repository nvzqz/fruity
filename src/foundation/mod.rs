//! [Foundation](https://developer.apple.com/documentation/foundation) framework.

mod cmp;
mod nsstring;

pub use cmp::*;
pub use nsstring::*;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}
