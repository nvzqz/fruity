//! [Dispatch](https://developer.apple.com/documentation/dispatch) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`dispatch`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "dispatch")]

// Dispatch is reexported by libSystem on Apple platforms.
#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
extern "C" {}

mod autorelease_frequency;
mod object;
mod qos;
mod queue;
mod time;

pub use autorelease_frequency::*;
pub use object::*;
pub use qos::*;
pub use queue::*;
pub use time::*;
