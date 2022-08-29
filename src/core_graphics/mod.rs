//! [Core Graphics](https://developer.apple.com/documentation/coregraphics) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_graphics`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_graphics")]

#[cfg_attr(target_vendor = "apple", link(name = "CoreGraphics", kind = "framework"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "CoreGraphics"))]
extern "C" {}

mod geometry;

pub use geometry::*;
