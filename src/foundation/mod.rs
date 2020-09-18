//! [Foundation](https://developer.apple.com/documentation/foundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`foundation`**
//! [feature flag](../index.html#feature-flags).
//!
//! It also transitively enables [`objc`](../objc/index.html) and
//! [`core_graphics`](../core_graphics/index.html).

#![cfg(feature = "foundation")]

pub mod error_codes;

mod cmp;
mod geometry;
mod ns_error;
mod ns_exception;
mod ns_null;
mod ns_number;
mod ns_range;
mod ns_string;
mod ns_value;

pub use cmp::*;
pub use geometry::*;
pub use ns_error::*;
pub use ns_exception::*;
pub use ns_null::*;
pub use ns_number::*;
pub use ns_range::*;
pub use ns_string::*;
pub use ns_value::*;

/// A value indicating that a requested item couldn't be found or doesn’t exist.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsnotfound).
#[allow(non_upper_case_globals)]
pub const NSNotFound: crate::objc::NSInteger = crate::objc::NSIntegerMax;

#[link(name = "Foundation", kind = "framework")]
extern "C" {}
