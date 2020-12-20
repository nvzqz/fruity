//! Common types across disparate modules.
//!
//! By defining a type in a single location and re-exporting it in the
//! "official" modules, the same type is compatible across modules.

#![allow(dead_code)]

#[cfg(any(feature = "app_kit", feature = "ui_kit"))]
mod ns_directional_insets;

#[cfg(any(feature = "app_kit", feature = "ui_kit"))]
pub use ns_directional_insets::*;
