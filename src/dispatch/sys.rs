//! Raw unsafe C functions exposed by libdispatch.

// Dispatch is reexported by libSystem on Apple platforms.
#[cfg_attr(target_vendor = "apple", link(name = "System", kind = "dylib"))]
#[cfg_attr(not(target_vendor = "apple"), link(name = "dispatch", kind = "dylib"))]
#[allow(missing_docs)]
extern "C" {}
