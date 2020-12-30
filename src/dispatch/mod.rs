//! [Dispatch](https://developer.apple.com/documentation/dispatch) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`dispatch`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "dispatch")]

pub mod sys;

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
