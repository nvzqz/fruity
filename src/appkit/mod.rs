//! [AppKit](https://developer.apple.com/documentation/appkit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`appkit`**
//! [feature flag](../index.html#feature-flags).

#![cfg(all(
    feature = "appkit",
    target_os = "macos",
))]

#[link(name = "AppKit", kind = "framework")]
extern "C" {}
