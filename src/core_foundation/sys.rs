//! Raw unsafe C functions exposed by `CoreFoundation.framework`.

use super::{
    Boolean, CFAllocator, CFAllocatorContext, CFHashCode, CFIndex, CFOptionFlags, CFType, CFTypeID,
};
use std::ffi::c_void;

#[allow(missing_docs)]
#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    pub fn CFRetain(cf: *const CFType) -> *const CFType;
    pub fn CFRelease(cf: *const CFType);
    pub fn CFGetRetainCount(cf: *const CFType) -> CFIndex;

    pub fn CFEqual(cf1: *const CFType, cf2: *const CFType) -> Boolean;

    pub fn CFHash(cf: *const CFType) -> CFHashCode;
    pub fn CFGetTypeID(cf: *const CFType) -> CFTypeID;

    pub fn CFAllocatorGetTypeID() -> CFTypeID;

    pub fn CFAllocatorCreate(
        allocator: *const CFAllocator,
        context: *mut CFAllocatorContext,
    ) -> *mut CFAllocator;

    pub fn CFAllocatorAllocate(
        allocator: *const CFAllocator,
        size: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut c_void;

    pub fn CFAllocatorReallocate(
        allocator: *const CFAllocator,
        ptr: *mut c_void,
        new_size: CFIndex,
        hint: CFOptionFlags,
    ) -> *mut c_void;

    pub fn CFAllocatorDeallocate(allocator: *const CFAllocator, ptr: *mut c_void);

    pub fn CFAllocatorGetPreferredSizeForSize(
        allocator: *const CFAllocator,
        size: CFIndex,
        hint: CFOptionFlags,
    ) -> CFIndex;

    pub fn CFAllocatorGetDefault() -> *const CFAllocator;
    pub fn CFAllocatorSetDefault(allocator: *const CFAllocator);

    pub fn CFAllocatorGetContext(allocator: *const CFAllocator, context: *mut CFAllocatorContext);
}
