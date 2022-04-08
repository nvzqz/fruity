use super::sys;
use std::{alloc::Layout, ffi::CStr, fmt, mem::MaybeUninit, os::raw::c_char};

/// An [ObjC `@encode(type)`][encodings] thin C string.
///
/// [encodings]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html#//apple_ref/doc/uid/TP40008048-CH100
pub struct TypeEncoding {
    data: [u8; 0],
}

impl fmt::Debug for TypeEncoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("TypeEncoding")
            .field("type", &self.as_cstr())
            .field("layout", &self.layout())
            .finish()
    }
}

impl TypeEncoding {
    /// Creates an instance from a raw C string pointer.
    #[inline]
    pub const unsafe fn from_ptr<'a>(encoding: *const c_char) -> &'a TypeEncoding {
        &*encoding.cast()
    }

    /// Returns the raw C string pointer.
    #[inline]
    pub const fn as_ptr(&self) -> *const c_char {
        self.data.as_ptr().cast()
    }

    /// Returns the raw C string.
    #[inline]
    pub fn as_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.as_ptr()) }
    }

    /// Returns a tuple of the type's encoded size and alignment.
    #[inline]
    #[doc(alias = "NSGetSizeAndAlignment")]
    pub fn size_and_alignment(&self) -> (usize, usize) {
        unsafe {
            let mut size = MaybeUninit::uninit();
            let mut align = MaybeUninit::uninit();

            sys::NSGetSizeAndAlignment(self.as_ptr(), size.as_mut_ptr(), align.as_mut_ptr());

            (size.assume_init(), align.assume_init())
        }
    }

    /// Returns the type's encoded size and alignment as an allocation
    /// [`Layout`].
    // #[inline]
    pub fn layout(&self) -> Layout {
        let (size, align) = self.size_and_alignment();
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }
}
