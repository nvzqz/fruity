//! [UIKit](https://developer.apple.com/documentation/uikit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`ui_kit`**
//! [feature flag](../index.html#feature-flags).

#![cfg(all(
    feature = "ui_kit",
    not(target_os = "macos"),
))]

mod nsvalue;

#[link(name = "UIKit", kind = "framework")]
extern "C" {}
