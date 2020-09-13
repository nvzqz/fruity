//! [Core Graphics](https://developer.apple.com/documentation/coregraphics) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_graphics`**
//! [feature flag](../index.html#feature-flags).

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {}

mod geometry;

pub use geometry::*;
