//! [Foundation](https://developer.apple.com/documentation/foundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`foundation`**
//! [feature flag](../index.html#feature-flags).
//!
//! It also transitively enables [`objc`](crate::objc) and
//! [`core_graphics`](crate::core_graphics).

#![cfg(feature = "foundation")]

#[macro_use]
mod ns_string;

pub mod error_codes;

mod cmp;
mod geometry;
mod ns_error;
mod ns_exception;
mod ns_null;
mod ns_number;
mod ns_range;
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

/// A number of seconds.
///
/// A `NSTimeInterval` value is always specified in seconds; it yields
/// sub-millisecond precision over a range of 10,000 years.
///
/// On its own, a time interval does not specify a unique point in time, or even
/// a span between specific times. Combining a time interval with one or more
/// known reference points yields a `NSDate` or `NSDateInterval` value.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nstimeinterval?language=objc).
pub type NSTimeInterval = f64;

/// A value indicating that a requested item couldn't be found or doesn’t exist.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsnotfound).
#[allow(non_upper_case_globals)]
pub const NSNotFound: crate::objc::NSInteger = crate::objc::NSIntegerMax;

#[cfg_attr(target_vendor = "apple", link(name = "Foundation", kind = "framework"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "Foundation"))]
extern "C" {}
