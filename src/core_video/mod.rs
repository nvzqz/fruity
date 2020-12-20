//! [Core Video](https://developer.apple.com/documentation/corevideo)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_video`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_video")]

#[link(name = "CoreImage", kind = "framework")]
extern "C" {}
