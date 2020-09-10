//! [Foundation](https://developer.apple.com/documentation/foundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`foundation`**
//! [feature flag](../index.html#feature-flags).
//!
//! It also transitively enables [`objc`](../objc/index.html).

mod cmp;
mod nsstring;

pub use cmp::*;
pub use nsstring::*;

#[cfg_attr(feature = "link", link(name = "Foundation", kind = "framework"))]
extern "C" {}
