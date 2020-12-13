use std::{ffi::CStr, fmt, os::raw::c_char};

mod attribute;

pub use attribute::*;

#[cfg(feature = "malloced")]
use malloced::Malloced;

/// An Objective-C property declaration.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/objc_property_t).
///
/// # Usage
///
/// This is an opaque type meant to be used behind a shared reference
/// `&Property`, which is semantically equivalent to `objc_property_t _Nonnull`.
///
/// A nullable property is defined as `Option<&Property>`, which is semantically
/// equivalent to `objc_property_t _Nullable`.
#[repr(C)]
pub struct Property {
    data: [u8; 0],
}

impl fmt::Debug for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Property")
            .field("name", &self.name())
            .field("attributes", &self.attributes())
            .finish()
    }
}

impl Property {
    /// Returns the name of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418903-property_getname).
    #[inline]
    #[doc(alias = "property_getName")]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.name_ptr()) }
    }

    /// Returns a pointer to the name of `self`.
    ///
    /// This will be removed in favor of [`name`](Self::name) when `&CStr`
    /// becomes an FFI-safe thin pointer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418903-property_getname).
    #[inline]
    #[doc(alias = "property_getName")]
    pub fn name_ptr(&self) -> *const c_char {
        extern "C" {
            fn property_getName(property: &Property) -> *const c_char;
        }
        unsafe { property_getName(self) }
    }

    /// Returns the attributes of `self`.
    ///
    /// The format of the attribute string is described in
    /// [Declared Properties](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtPropertyIntrospection.html#//apple_ref/doc/uid/TP40008048-CH101)
    /// in
    /// [Objective-C Runtime Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40008048).
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418909-property_getattributes).
    #[inline]
    #[doc(alias = "property_getAttributes")]
    pub fn attributes(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.attributes_ptr()) }
    }

    /// Returns a pointer to the attributes of `self`.
    ///
    /// The format of the attribute string is described in
    /// [Declared Properties](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtPropertyIntrospection.html#//apple_ref/doc/uid/TP40008048-CH101)
    /// in
    /// [Objective-C Runtime Programming Guide](https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Introduction/Introduction.html#//apple_ref/doc/uid/TP40008048).
    ///
    /// This will be removed in favor of [`attributes`](Self::attributes) when
    /// `&CStr` becomes an FFI-safe thin pointer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418909-property_getattributes).
    #[inline]
    #[doc(alias = "property_getAttributes")]
    pub fn attributes_ptr(&self) -> *const c_char {
        extern "C" {
            fn property_getAttributes(property: &Property) -> *const c_char;
        }
        unsafe { property_getAttributes(self) }
    }

    // TODO: property_copyAttributeValue
    //
    // This requires `Malloced<CStr>`, but that's not possible until we get
    // `CStr::from_mut_ptr(*mut c_char) -> &mut CStr`.

    /// Returns a `malloc`-ed list of attributes of `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418675-property_copyattributelist).
    #[cfg(feature = "malloced")]
    #[inline]
    #[doc(alias = "property_copyAttributeList")]
    pub fn copy_attributes_list<'a>(&'a self) -> Option<Malloced<[PropertyAttribute<'a>]>> {
        use std::{mem::MaybeUninit, os::raw::c_uint};

        extern "C" {
            fn property_copyAttributeList<'a>(
                property: &'a Property,
                out_count: *mut c_uint,
            ) -> *mut PropertyAttribute<'a>;
        }

        let mut len = MaybeUninit::<c_uint>::uninit();
        unsafe {
            let data = property_copyAttributeList(self, len.as_mut_ptr());
            if data.is_null() {
                None
            } else {
                Some(Malloced::slice_from_raw_parts(
                    data,
                    len.assume_init() as usize,
                ))
            }
        }
    }
}
