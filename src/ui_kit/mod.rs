//! [UIKit](https://developer.apple.com/documentation/uikit) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`ui_kit`**
//! [feature flag](../index.html#feature-flags).

#![cfg(all(feature = "ui_kit", any(mac_catalyst, not(target_os = "macos"))))]

mod ext;

#[doc(inline)]
pub use crate::common::NSDirectionalEdgeInsets;

#[link(name = "UIKit", kind = "framework")]
extern "C" {}
