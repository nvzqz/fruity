//! [Core Foundation](https://developer.apple.com/documentation/corefoundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_foundation`**
//! [feature flag](../index.html#feature-flags).
//!
//! It also transitively enables [`objc`](../objc/index.html).

#![cfg(feature = "core_foundation")]

pub mod sys;

mod cf_allocator;
mod cf_number;
mod cf_type;
mod cmp;

pub use cf_allocator::*;
pub use cf_number::*;
pub use cf_type::*;
pub use cmp::*;

/// A constant that indicates that a search operation did not succeed in
/// locating the target value.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfnotfound).
#[allow(non_upper_case_globals)]
pub const kCFNotFound: CFIndex = -1;

/// A bitfield used for passing special allocation and other requests into Core
/// Foundation functions.
///
/// The flag bits are specific to particular opaque types and functions in Core
/// Foundation.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfoptionflags).
pub type CFOptionFlags = usize;

/// Type for hash codes returned by
/// [`CFType::hash`](struct.CFType.html#method.hash).
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfhashcode).
pub type CFHashCode = usize;

/// An integer type used as an array index, count, size, or length.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfindex).
pub type CFIndex = isize;

pub(crate) type Boolean = std::os::raw::c_uchar;
