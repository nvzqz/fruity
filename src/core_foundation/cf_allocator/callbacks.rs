use crate::core_foundation::{CFIndex, CFOptionFlags};
use std::{ffi::c_void, ptr::NonNull};

/// A [`CFAllocator`](super::CFAllocator) function callback that retains the
/// given data.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorretaincallback?language=objc).
pub type CFAllocatorRetainCallBack = unsafe extern "C" fn(info: *const c_void) -> *const c_void;
// typedef const void *(*CFAllocatorRetainCallBack)(const void *info)

/// A [`CFAllocator`](super::CFAllocator) function callback that releases the
/// given data.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorreleasecallback?language=objc).
pub type CFAllocatorReleaseCallBack = unsafe extern "C" fn(info: *const c_void);

/// A [`CFAllocator`](super::CFAllocator) function callback that provides a
/// description of the specified data.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorcopydescriptioncallback?language=objc).
pub type CFAllocatorCopyDescriptionCallBack =
    unsafe extern "C" fn(info: *const c_void) -> *mut c_void; // TODO: Return type `Arc<CFString>`.

/// A [`CFAllocator`](super::CFAllocator) function callback that allocates
/// memory of a requested size.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorallocatecallback?language=objc).
pub type CFAllocatorAllocateCallBack = unsafe extern "C" fn(
    alloc_size: CFIndex,
    hint: CFOptionFlags,
    info: *mut c_void,
) -> *mut c_void;

/// A [`CFAllocator`](super::CFAllocator) function callback that reallocates
/// memory of a requested size for an existing block of memory.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorreallocatecallback?language=objc).
pub type CFAllocatorReallocateCallBack = unsafe extern "C" fn(
    ptr: *mut c_void,
    new_size: CFIndex,
    hint: CFOptionFlags,
    info: *mut c_void,
) -> *mut c_void;

/// A [`CFAllocator`](super::CFAllocator) function callback that deallocates a
/// block of memory.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatordeallocatecallback?language=objc).
pub type CFAllocatorDeallocateCallBack =
    unsafe extern "C" fn(ptr: NonNull<c_void>, info: *mut c_void);

/// A [`CFAllocator`](super::CFAllocator) function callback that gives the size
/// of memory likely to be allocated, given a certain request.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorpreferredsizecallback?language=objc).
pub type CFAllocatorPreferredSizeCallBack =
    unsafe extern "C" fn(size: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> CFIndex;
