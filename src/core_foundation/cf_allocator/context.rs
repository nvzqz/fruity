use crate::core_foundation::CFIndex;
use std::ffi::c_void;

/// The context or operating environment for a
/// [`CFAllocator`](super::CFAllocator) object.
///
/// see [documentation](https://developer.apple.com/documentation/corefoundation/cfallocatorcontext?language=objc).
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CFAllocatorContext {
    /// The version number of the allocator.
    ///
    /// Currently the only valid value is 0.
    pub version: CFIndex,

    /// An untyped pointer to program-defined data.
    ///
    /// Allocate memory for this data and assign a pointer to it. This data is
    /// often control information for the allocator.
    ///
    /// You may assign `NULL`.
    pub info: *mut c_void,

    /// A function callback that retains the data pointed to by the info field.
    ///
    /// In implementing this function, retain the data you have defined for the
    /// allocator context in this field. (This might make sense only if the data
    /// is a Core Foundation object.)
    ///
    /// You may set this function pointer to [`None`].
    pub retain: Option<super::CFAllocatorRetainCallBack>,

    /// A function callback that releases the data pointed to by the info field.
    ///
    /// In implementing this function, release (or free) the data you have
    /// defined for the allocator context.
    ///
    /// You may set this function pointer to [`None`], but doing so might result
    /// in memory leaks.
    pub release: Option<super::CFAllocatorReleaseCallBack>,

    /// A function callback that provides a description of the data pointed to
    /// by the info field.
    ///
    /// In implementing this function, return a reference to a `CFString` object
    /// that describes your allocator, particularly some characteristics of your
    /// program-defined data.
    ///
    /// You may set this function pointer to [`None`], in which case Core
    /// Foundation will provide a rudimentary description.
    pub copy_description: Option<super::CFAllocatorCopyDescriptionCallBack>,

    /// A function callback that allocates memory of a requested size.
    ///
    /// In implementing this function, allocate a block of memory of at least
    /// size bytes and return a pointer to the start of the block.
    ///
    /// The `hint` argument is a bitfield that you should currently not use
    /// (that is, assign 0).
    ///
    /// The `size` parameter should always be greater than 0. If it is not, or
    /// if problems in allocation occur, return `NULL`.
    pub allocate: super::CFAllocatorAllocateCallBack,

    /// A function callback that reallocates memory of a requested size for an
    /// existing block of memory.
    pub reallocate: Option<super::CFAllocatorReallocateCallBack>,

    /// A function callback that deallocates a given block of memory.
    ///
    /// In implementing this function, make the block of memory pointed to by
    /// ptr available for subsequent reuse by the allocator but unavailable for
    /// continued use by the program. The `ptr` parameter cannot be `NULL` and
    /// if the ptr parameter is not a block of memory that has been previously
    /// allocated by the allocator, the results are undefined; abnormal program
    /// termination can occur.
    ///
    /// You can set this callback to [`None`], in which case the
    /// `CFAllocatorDeallocate` function has no effect.
    pub deallocate: Option<super::CFAllocatorDeallocateCallBack>,

    /// A function callback that determines whether there is enough free memory
    /// to satisfy a request.
    ///
    /// In implementing this function, return the actual size the allocator is
    /// likely to allocate given a request for a block of memory of size size.
    ///
    /// The `hint` argument is a bitfield that you should currently not use.
    pub preferred_size: Option<super::CFAllocatorPreferredSizeCallBack>,
}
