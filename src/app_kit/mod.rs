//! [AppKit](https://developer.apple.com/documentation/appkit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`app_kit`**
//! [feature flag](../index.html#feature-flags).

#![cfg(all(
    feature = "app_kit",
    any(
        target_os = "macos",
        // Enabled by `build.rs` for `x86_64-apple-ios-macabi`.
        mac_catalyst,
    )
))]

mod version;

pub use version::*;

#[link(name = "AppKit", kind = "framework")]
extern "C" {}
