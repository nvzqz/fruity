//! [Core Audio](https://developer.apple.com/documentation/coreaudio)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_audio`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_audio")]

#[link(name = "CoreAudio", kind = "framework")]
extern "C" {}
