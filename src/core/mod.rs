//! Core types and traits of this crate.
//!
//! Items within this module are not specific to any wrapped library.

#[macro_use]
mod macros;

mod arc;
mod object_type;

pub use arc::*;
pub use object_type::*;
