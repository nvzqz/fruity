//! [Foundation](https://developer.apple.com/documentation/foundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`foundation`**
//! [feature flag](../index.html#feature-flags).
//!
//! It also transitively enables [`objc`](../objc/index.html) and
//! [`core_graphics`](../core_graphics/index.html).

#![cfg(feature = "foundation")]

mod cmp;
mod geometry;
mod nsnumber;
mod nsrange;
mod nsstring;
mod nsvalue;

pub use cmp::*;
pub use geometry::*;
pub use nsnumber::*;
pub use nsrange::*;
pub use nsstring::*;
pub use nsvalue::*;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}
