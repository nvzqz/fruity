//! [Core Foundation](https://developer.apple.com/documentation/corefoundation) framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`core_foundation`**
//! [feature flag](../index.html#feature-flags).

#![cfg(feature = "core_foundation")]

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {}

mod cf_type;

pub use cf_type::*;

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
