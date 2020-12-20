//! [Core Text](https://developer.apple.com/documentation/coretext)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_text`**
//! [feature flag](../index.html#feature-flags).
//!
//! # Documentation
//!
//! - [Introduction](https://developer.apple.com/library/archive/documentation/StringsTextFonts/Conceptual/CoreText_Programming/Introduction/Introduction.html)
//! - [Overview](https://developer.apple.com/library/archive/documentation/StringsTextFonts/Conceptual/CoreText_Programming/Overview/Overview.html)
//! - [Common Text Layout Operations](https://developer.apple.com/library/archive/documentation/StringsTextFonts/Conceptual/CoreText_Programming/LayoutOperations/LayoutOperations.html)
//! - [Common Font Operations](https://developer.apple.com/library/archive/documentation/StringsTextFonts/Conceptual/CoreText_Programming/FontOperations/FontOperations.html)

#![cfg(feature = "core_text")]

#[link(name = "CoreText", kind = "framework")]
extern "C" {}
