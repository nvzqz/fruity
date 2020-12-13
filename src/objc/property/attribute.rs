use std::{ffi::CStr, fmt, marker::PhantomData, os::raw::c_char, ptr::NonNull};

/// A [`Property`](super::Property) attribute.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/objc_property_attribute_t).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PropertyAttribute<'a> {
    // TODO: Use `&'a CStr` when it becomes a thin pointer.
    name: NonNull<c_char>,
    value: NonNull<c_char>,
    _marker: PhantomData<&'a CStr>,
}

impl fmt::Debug for PropertyAttribute<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PropertyAttribute")
            .field("name", &self.name())
            .field("value", &self.value())
            .finish()
    }
}

impl PartialEq for PropertyAttribute<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name() && self.value() == other.value()
    }
}

impl Eq for PropertyAttribute<'_> {}

impl<'a> PropertyAttribute<'a> {
    /// The name of the attribute.
    #[inline]
    pub fn name(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.name_ptr()) }
    }

    /// A pointer to the name of the attribute.
    ///
    /// This will be removed in favor of [`name`](Self::name) when `&CStr`
    /// becomes an FFI-safe thin pointer.
    #[inline]
    pub fn name_ptr(&self) -> *const c_char {
        self.name.as_ptr()
    }

    /// The value of the attribute (usually empty).
    #[inline]
    pub fn value(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(self.value_ptr()) }
    }

    /// A pointer to the value of the attribute (usually empty).
    ///
    /// This will be removed in favor of [`value`](Self::value) when `&CStr`
    /// becomes an FFI-safe thin pointer.
    #[inline]
    pub fn value_ptr(&self) -> *const c_char {
        self.value.as_ptr()
    }
}
