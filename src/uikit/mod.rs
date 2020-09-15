//! [UIKit](https://developer.apple.com/documentation/uikit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`uikit`**
//! [feature flag](../index.html#feature-flags).

#![cfg(all(
    feature = "uikit",
    not(target_os = "macos"),
))]

mod nsvalue;

#[link(name = "UIKit", kind = "framework")]
extern "C" {}
