//! [Core Animation](https://developer.apple.com/documentation/quartzcore)
//! framework, also known as QuartzCore.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_animation`**
//! [feature flag](../index.html#feature-flags).
//!
//! # Documentation
//!
//! - [Introduction](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CoreAnimation_guide/Introduction/Introduction.html)
//! - [Basics](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CoreAnimation_guide/CoreAnimationBasics/CoreAnimationBasics.html)
//! - [Improving Animation Performance](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/CoreAnimation_guide/ImprovingAnimationPerformance/ImprovingAnimationPerformance.html)

#![cfg(feature = "core_animation")]
#![doc(alias = "quartz_core")]
#![doc(alias = "quartzcore")]

#[link(name = "QuartzCore", kind = "framework")]
extern "C" {}
