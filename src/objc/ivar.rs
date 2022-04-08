use super::{sys, TypeEncoding};
use core::fmt;
use std::ffi::CStr;

/// An opaque type that represents an instance variable.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/objectivec/ivar?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/objectivec/ivar?language=objc)
#[repr(C)]
pub struct Ivar {
    _priv: [u8; 0],
}

impl fmt::Debug for Ivar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Ivar")
            .field("name", &self.name())
            .field("offset", &self.offset())
            .field("type_encoding", &self.type_encoding())
            .finish()
    }
}

impl Ivar {
    /// Returns the instance variable's name.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/objectivec/1418922-ivar_getname?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/objectivec/1418922-ivar_getname?language=objc)
    #[inline]
    #[doc(alias = "ivar_getName")]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(sys::ivar_getName(self)) }
    }

    /// Returns the instance variable's offset from the object base.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/objectivec/1418976-ivar_getoffset?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/objectivec/1418976-ivar_getoffset?language=objc)
    #[inline]
    #[doc(alias = "ivar_getOffset")]
    pub fn offset(&self) -> isize {
        unsafe { sys::ivar_getOffset(self) }
    }

    /// Returns the instance variable's `@encode(type)` string.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/objectivec/1418569-ivar_gettypeencoding?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/objectivec/1418569-ivar_gettypeencoding?language=objc)
    #[inline]
    #[doc(alias = "ivar_getTypeEncoding")]
    pub fn type_encoding(&self) -> &TypeEncoding {
        unsafe { TypeEncoding::from_ptr(sys::ivar_getTypeEncoding(self)) }
    }
}
