use crate::objc::Sel;
use std::{ffi::CStr, fmt, marker::PhantomData, os::raw::c_char, ptr::NonNull};

/// An Objective-C method definition.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/objc_method_description).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MethodDescription<'a> {
    name: Sel,
    types: NonNull<c_char>,
    _marker: PhantomData<&'a CStr>,
}

unsafe impl Send for MethodDescription<'_> {}
unsafe impl Sync for MethodDescription<'_> {}

impl fmt::Debug for MethodDescription<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MethodDescription")
            .field("name", &self.name())
            .field("types", &self.types())
            .finish()
    }
}

impl<'a> MethodDescription<'a> {
    /// Creates a new instance with the method name and argument types.
    #[inline]
    pub const fn new(name: Sel, types: &'a CStr) -> Self {
        unsafe { Self::from_raw_parts(name, types.as_ptr()) }
    }

    /// An alternative to [`new`](Self::new) that does not incur the cost of
    /// looking up the null byte twice (once to create `&CStr`).
    ///
    /// This will be removed in favor of [`new`](Self::new) when `&CStr` becomes
    /// an FFI-safe thin pointer.
    ///
    /// # Safety
    ///
    /// `types` must point to a valid C string that will not live shorter than
    /// the lifetime `'a`.
    #[inline]
    pub const unsafe fn from_raw_parts(name: Sel, types: *const c_char) -> Self {
        Self {
            name,
            types: NonNull::new_unchecked(types as *mut c_char),
            _marker: PhantomData,
        }
    }

    /// The name of the method at runtime.
    #[inline]
    pub fn name(&self) -> Sel {
        self.name
    }

    /// The types of the method arguments.
    #[inline]
    pub fn types(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.types.as_ptr()) }
    }
}
