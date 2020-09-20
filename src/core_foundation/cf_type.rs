use super::{Boolean, CFHashCode, CFIndex};
use std::{cell::UnsafeCell, fmt, hash, mem, ops::Deref, ptr::NonNull};

/// Unique constant integer value that identifies particular Core Foundation
/// opaque types.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/corefoundation/cftypeid?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/corefoundation/cftypeid?language=objc)
pub type CFTypeID = usize;

/// A Core Foundation object instance.
///
/// This is designed to be used behind a reference. In the future, this will be
/// defined as an
/// [`extern type`](https://github.com/rust-lang/rfcs/blob/master/text/1861-extern-types.md).
///
/// All Core Foundation types within this crate ultimately
/// [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html) to this type.
#[repr(C)]
pub struct CFType {
    // This data can be mutably aliased behind a shared reference.
    _data: UnsafeCell<[u8; 0]>,
}

// SAFETY: `CFTypeRef` is bridged to `id`.
#[cfg(feature = "objc")]
unsafe impl crate::objc::ObjectType for &CFType {}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for CFType {}

impl AsRef<CFType> for CFType {
    #[inline]
    fn as_ref(&self) -> &CFType {
        self
    }
}

impl PartialEq for CFType {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        extern "C" {
            fn CFEqual(cf1: &CFType, cf2: &CFType) -> Boolean;
        }
        unsafe { CFEqual(self, other) != 0 }
    }
}

impl hash::Hash for CFType {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash().hash(state)
    }
}

impl fmt::Debug for CFType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl CFType {
    #[inline]
    pub(crate) fn _retain(&self) -> NonNull<CFType> {
        extern "C" {
            fn CFRetain(cf: &CFType) -> NonNull<CFType>;
        }
        unsafe { CFRetain(self) }
    }

    /// Casts `self` to a raw nullable pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut CFType {
        self._data.get().cast()
    }

    /// Casts `self` to a raw non-null pointer.
    #[inline]
    pub fn as_non_null_ptr(&self) -> NonNull<CFType> {
        NonNull::from(self).cast()
    }

    /// Returns this object's reference count.
    ///
    /// This method is only useful for debugging certain objects.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1521288-cfgetretaincount).
    #[inline]
    pub fn retain_count(&self) -> CFIndex {
        extern "C" {
            fn CFGetRetainCount(cf: &CFType) -> CFIndex;
        }
        unsafe { CFGetRetainCount(self) }
    }

    /// Returns a code that can be used to identify `self` in a hashing
    /// structure.
    #[inline]
    pub fn hash(&self) -> CFHashCode {
        extern "C" {
            fn CFHash(cf: &CFType) -> CFHashCode;
        }
        unsafe { CFHash(self) }
    }

    /// Returns the unique identifier of an opaque type to which `self` belongs.
    #[inline]
    pub fn get_type_id(&self) -> CFTypeID {
        extern "C" {
            fn CFGetTypeID(cf: &CFType) -> CFTypeID;
        }
        unsafe { CFGetTypeID(self) }
    }

    // TODO: `CFGetAllocator`

    // TODO: `CFCopyDescription`
}

/// A non-null smart pointer to any Core Foundation object instance.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/corefoundation/cftyperef?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/corefoundation/cftyperef?language=objc)
#[repr(transparent)]
pub struct CFTypeRef(NonNull<CFType>);

// SAFETY: `CFTypeRef` is bridged to `id`.
#[cfg(feature = "objc")]
unsafe impl crate::objc::ObjectType for CFTypeRef {}

unsafe impl Send for CFTypeRef {}
unsafe impl Sync for CFTypeRef {}

impl Deref for CFTypeRef {
    type Target = CFType;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl AsRef<CFType> for CFTypeRef {
    #[inline]
    fn as_ref(&self) -> &CFType {
        self
    }
}

impl Drop for CFTypeRef {
    #[inline]
    fn drop(&mut self) {
        extern "C" {
            fn CFRelease(cf: &CFType);
        }
        unsafe { CFRelease(self) };
    }
}

impl Clone for CFTypeRef {
    #[inline]
    fn clone(&self) -> Self {
        Self(self._retain())
    }
}

#[cfg(feature = "objc")]
impl From<CFTypeRef> for crate::objc::id {
    #[inline]
    fn from(cf: CFTypeRef) -> Self {
        // SAFETY: `CFTypeRef` is bridged to `id`.
        unsafe { mem::transmute(cf) }
    }
}

impl PartialEq for CFTypeRef {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        CFType::eq(self, other)
    }
}

impl PartialEq<CFType> for CFTypeRef {
    #[inline]
    fn eq(&self, other: &CFType) -> bool {
        CFType::eq(self, other)
    }
}

impl PartialEq<CFTypeRef> for CFType {
    #[inline]
    fn eq(&self, other: &CFTypeRef) -> bool {
        self.eq(other as &CFType)
    }
}

impl hash::Hash for CFTypeRef {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        hash::Hash::hash(self as &CFType, state)
    }
}
