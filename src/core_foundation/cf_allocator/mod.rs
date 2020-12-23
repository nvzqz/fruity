use super::{CFType, CFTypeID};
use crate::{core::Arc, core_foundation::sys};
use std::{mem::MaybeUninit, ptr};

mod callbacks;
mod context;

pub use callbacks::*;
pub use context::*;

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

impl CFAllocator {
    /// Returns the type identifier for `CFAllocator`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521328-cfallocatorgettypeid?language=objc).
    #[inline]
    #[doc(alias = "CFAllocatorGetTypeID")]
    pub fn type_id() -> CFTypeID {
        unsafe { sys::CFAllocatorGetTypeID() }
    }

    /// Creates an allocator object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521159-cfallocatorcreate?language=objc).
    #[inline]
    #[doc(alias = "CFAllocatorCreate")]
    pub unsafe fn create(
        allocator: Option<&CFAllocator>,
        mut context: CFAllocatorContext,
    ) -> Arc<Self> {
        let allocator: *const CFAllocator = match allocator {
            Some(allocator) => allocator,
            None => ptr::null(),
        };
        Arc::from_raw(sys::CFAllocatorCreate(allocator, &mut context))
    }

    /// Returns the context of the default allocator.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521267-cfallocatorgetcontext?language=objc).
    #[inline]
    #[doc(alias = "CFAllocatorGetContext")]
    pub fn default_context() -> CFAllocatorContext {
        unsafe {
            let mut context = MaybeUninit::uninit();
            sys::CFAllocatorGetContext(ptr::null(), context.as_mut_ptr());
            context.assume_init()
        }
    }

    /// Returns the context of this allocator.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521267-cfallocatorgetcontext?language=objc).
    #[inline]
    #[doc(alias = "CFAllocatorGetContext")]
    pub fn context(&self) -> CFAllocatorContext {
        unsafe {
            let mut context = MaybeUninit::uninit();
            sys::CFAllocatorGetContext(self, context.as_mut_ptr());
            context.assume_init()
        }
    }
}

/// Getting and setting the default allocator.
impl CFAllocator {
    /// Gets the default allocator object for the current thread and retains it.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521325-cfallocatorgetdefault?language=objc).
    #[inline]
    #[doc(alias = "CFAllocatorGetDefault")]
    pub fn default() -> Arc<Self> {
        unsafe { Arc::retain_raw(sys::CFAllocatorGetDefault()) }
    }

    /// Gets the default allocator object for the current thread without
    /// retaining it.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521325-cfallocatorgetdefault?language=objc).
    ///
    /// # Safety
    ///
    /// Ownership of the allocator returned by `CFAllocatorGetDefault` follows
    /// [The Get Rule](https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1),
    /// so you do not own it and cannot be certain of the object's life span.
    #[inline]
    #[doc(alias = "CFAllocatorGetDefault")]
    pub unsafe fn default_unretained<'a>() -> &'a Self {
        &*sys::CFAllocatorGetDefault()
    }

    /// Sets this given allocator as the default for the current thread.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521325-cfallocatorgetdefault?language=objc).
    ///
    /// # Safety
    ///
    /// This allocator must be able to deal with arbitrary allocation requests.
    ///
    /// This allocator must never be released, even if another allocator
    /// replaces it as the default.
    #[inline]
    #[doc(alias = "CFAllocatorSetDefault")]
    pub unsafe fn make_default(&self) {
        sys::CFAllocatorSetDefault(self);
    }
}
