//! Raw unsafe C functions exposed by `CoreFoundation.framework`.

use super::{Boolean, CFHashCode, CFIndex, CFType, CFTypeID};

#[allow(missing_docs)]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRetain(cf: *const CFType) -> *const CFType;
    pub fn CFRelease(cf: *const CFType);
    pub fn CFGetRetainCount(cf: *const CFType) -> CFIndex;

    pub fn CFEqual(cf1: *const CFType, cf2: *const CFType) -> Boolean;

    pub fn CFHash(cf: *const CFType) -> CFHashCode;
    pub fn CFGetTypeID(cf: *const CFType) -> CFTypeID;
}
