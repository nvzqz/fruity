//! Core types and traits of this crate.
//!
//! Items within this module are not specific to any wrapped library.

#[macro_use]
mod macros;

mod arc;
mod four_char_code;
mod object_type;
mod os_err;
mod os_status;

pub use arc::*;
pub use four_char_code::*;
pub use object_type::*;
pub use os_err::*;
pub use os_status::*;
