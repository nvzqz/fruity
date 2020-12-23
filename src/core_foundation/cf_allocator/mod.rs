use super::CFType;

subclass! {
    /// An allocator object.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/corefoundation/cfallocator?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/corefoundation/cfallocator?language=objc)
    ///
    /// # Using the default allocator
    ///
    /// APIs that use `CFAllocator` will take an `Option<&CFAllocator>`, where
    /// [`None`] is equivalent to
    /// [`kCFAllocatorDefault`](https://developer.apple.com/documentation/corefoundation/kcfallocatordefault?language=objc).
    pub class CFAllocator: CFType<'static>;
}

/// Predefined allocators.
///
/// See [documentation](https://developer.apple.com/documentation/corefoundation/cfallocator/predefined_allocators?language=objc).
impl CFAllocator {
    /// An allocator that does nothing—it allocates no memory.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfallocatornull?language=objc).
    #[doc(alias = "kCFAllocatorNull")]
    pub fn null() -> &'static Self {
        extern "C" {
            static kCFAllocatorNull: &'static CFAllocator;
        }
        unsafe { kCFAllocatorNull }
    }

    /// Default system allocator.
    ///
    /// You rarely need to use this.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfallocatorsystemdefault?language=objc).
    #[doc(alias = "kCFAllocatorSystemDefault")]
    pub fn system_default() -> &'static Self {
        extern "C" {
            static kCFAllocatorSystemDefault: &'static CFAllocator;
        }
        unsafe { kCFAllocatorSystemDefault }
    }

    /// An allocator that uses `malloc()`, `realloc()`, and `free()`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfallocatormalloc?language=objc).
    #[doc(alias = "kCFAllocatorMalloc")]
    pub fn malloc() -> &'static Self {
        extern "C" {
            static kCFAllocatorMalloc: &'static CFAllocator;
        }
        unsafe { kCFAllocatorMalloc }
    }

    /// An allocator that uses the default malloc zone, returned by
    /// `malloc_default_zone()`.
    ///
    /// You should only use this when an object is safe to be allocated in non-scanned memory.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfallocatormalloczone?language=objc).
    #[doc(alias = "kCFAllocatorMallocZone")]
    pub fn malloc_zone() -> &'static Self {
        extern "C" {
            static kCFAllocatorMallocZone: &'static CFAllocator;
        }
        unsafe { kCFAllocatorMallocZone }
    }

    /// Special allocator argument to `CFAllocatorCreate`—it uses the functions
    /// given in the context to allocate the allocator.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/kcfallocatorusecontext?language=objc).
    ///
    /// # Safety
    ///
    /// This isn't really a `CFAllocator` reference, so most operations outside
    /// of the single specific use case are unsafe. They will most likely cause
    /// a [segfault](https://en.wikipedia.org/wiki/Segmentation_fault).
    ///
    /// This is treated as a `CFAllocator` reference in order to make it easy to
    /// fit into `CFAllocator` APIs.
    #[doc(alias = "kCFAllocatorUseContext")]
    pub unsafe fn use_context() -> &'static Self {
        extern "C" {
            static kCFAllocatorUseContext: &'static CFAllocator;
        }
        kCFAllocatorUseContext
    }
}
