//! [Core Foundation](https://developer.apple.com/documentation/corefoundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_foundation`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_foundation")]

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}
