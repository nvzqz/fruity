use std::{
    ffi::CStr,
    fmt,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

#[macro_use]
mod macros;

/// A method selector.
///
/// Selectors can be safely created using the
/// [`selector!`](../macro.selector.html) macro.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/sel).
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SEL(NonNull<c_void>);

unsafe impl Send for SEL {}
unsafe impl Sync for SEL {}

impl fmt::Debug for SEL {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name().fmt(f)
    }
}

impl SEL {
    /// Registers a method name with the Objective-C runtime and returns the
    /// selector.
    ///
    /// # Safety
    ///
    /// The name must be a non-null UTF-8 C string.
    #[inline]
    pub unsafe fn register(name: *const c_char) -> Self {
        sel_registerName(name)
    }

    /// Returns the name of the method this selector refers to.
    #[inline]
    pub fn name(self) -> &'static CStr {
        unsafe { CStr::from_ptr(sel_getName(self)) }
    }
}

extern "C" {
    fn sel_registerName(name: *const c_char) -> SEL;
    fn sel_getName(sel: SEL) -> *const c_char;
}
