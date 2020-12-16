use super::BOOL;
use std::{
    ffi::CStr,
    fmt,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

#[macro_use]
mod macros;

pub(crate) mod atomic;
pub(crate) mod cached;

/// A method selector.
///
/// Selectors can be safely created using the
/// [`selector!`](../macro.selector.html) macro.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/sel).
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sel(NonNull<c_void>);

unsafe impl Send for Sel {}
unsafe impl Sync for Sel {}

impl PartialEq for Sel {
    #[inline]
    #[doc(alias = "sel_isEqual")]
    fn eq(&self, other: &Self) -> bool {
        extern "C" {
            fn sel_isEqual(lhs: Sel, rhs: Sel) -> BOOL;
        }
        unsafe { sel_isEqual(*self, *other) }.into()
    }
}

impl Eq for Sel {}

impl fmt::Debug for Sel {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.name().fmt(f)
    }
}

impl Sel {
    /// Registers a method name with the Objective-C runtime and returns the
    /// selector.
    ///
    /// # Safety
    ///
    /// The name must be a non-null UTF-8 C string.
    #[inline]
    #[doc(alias = "sel_registerName")]
    pub unsafe fn register(name: *const c_char) -> Self {
        sel_registerName(name)
    }

    /// Creates a selector from a raw pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to valid selector data.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *const c_void) -> Self {
        Self(NonNull::new_unchecked(ptr as _))
    }

    /// Creates a selector from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to valid selector data.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<c_void>) -> Self {
        Self(ptr)
    }

    /// Returns a raw nullable pointer to this selector's data.
    #[inline]
    pub const fn as_ptr(&self) -> *const c_void {
        self.0.as_ptr()
    }

    /// Returns a raw non-null pointer to this selector's data.
    #[inline]
    pub const fn as_non_null_ptr(&self) -> NonNull<c_void> {
        self.0
    }

    /// Returns the name of the method this selector refers to.
    #[inline]
    pub fn name(self) -> &'static CStr {
        unsafe { CStr::from_ptr(sel_getName(self)) }
    }
}

extern "C" {
    fn sel_registerName(name: *const c_char) -> Sel;
    fn sel_getName(sel: Sel) -> *const c_char;
}
