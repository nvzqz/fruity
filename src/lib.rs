//! [WIP] Rusty bindings for Apple libraries, brought to you by
//! [@NikolaiVazquez](https://twitter.com/NikolaiVazquez).

// This crate is only available for 32 and 64 bit systems.
#![cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
#![deny(improper_ctypes)]
#![warn(missing_docs)]

pub mod mem;

#[macro_use]
pub mod objc;

pub mod core_foundation;
pub mod foundation;
