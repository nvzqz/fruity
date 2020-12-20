//! [Core Image](https://developer.apple.com/documentation/coreimage)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_image`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_image")]

#[link(name = "CoreImage", kind = "framework")]
extern "C" {}
