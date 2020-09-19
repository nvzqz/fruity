//! [Dispatch](https://developer.apple.com/documentation/dispatch) library.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`dispatch`**
//! [feature flag](../index.html#feature-flags).
//!
//! # Additional Features
//!
//! When the [`objc`](../objc/index.html) module is enabled, object types in
//! this module implement [`ObjectType`](../objc/trait.ObjectType.html). This
//! enables them to be stored in types like
//! [`NSArray`](../foundation/struct.NSArray.html) and
//! [`NSDictionary`](../foundation/struct.NSDictionary.html).

#![cfg(feature = "dispatch")]

// Dispatch is reexported by libSystem on Apple platforms.
#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
extern "C" {}

mod object;
mod qos;
mod time;

pub use object::*;
pub use qos::*;
pub use time::*;
