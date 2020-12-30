use std::ffi::c_void;

/// Calls a function in the context of a new autorelease pool, like
/// `@autoreleasepool`.
///
/// See [documentation](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmAutoreleasePools.html).
///
/// # Panic Handling
///
/// If a panic occurs within the function, the autorelease pool is drained as
/// the stack unwinds.
///
/// This is implemented with [`Drop`].
#[inline]
pub fn autoreleasepool<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let _pool = AutoreleasePool(unsafe { objc_autoreleasePoolPush() });
    f()
}

// For macOS 10.6 and lower, Clang emits `[[NSAutoreleasePool alloc] init]` and
// `-[NSAutoreleasePool drain]`. However, 10.7 is the minimum supported version
// for Rust.
extern "C" {
    fn objc_autoreleasePoolPush() -> *mut c_void;
    fn objc_autoreleasePoolPop(pool: *mut c_void);
}

struct AutoreleasePool(*mut c_void);

impl Drop for AutoreleasePool {
    #[inline]
    fn drop(&mut self) {
        unsafe { objc_autoreleasePoolPop(self.0) };
    }
}
