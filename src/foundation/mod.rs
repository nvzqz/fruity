//! [Foundation](https://developer.apple.com/documentation/foundation) framework.

mod cmp;
mod nsstring;

pub use cmp::*;
pub use nsstring::*;

#[cfg_attr(feature = "link", link(name = "Foundation", kind = "framework"))]
extern "C" {}
