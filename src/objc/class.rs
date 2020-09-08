use std::{
    cell::UnsafeCell,
    cmp,
    ffi::CStr,
    fmt, hash,
    os::raw::{c_char, c_int},
    panic::RefUnwindSafe,
    ptr,
};

/// An Objective-C class.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/class).
///
/// # Usage
///
/// This is an opaque type meant to be used behind a shared reference `&Class`,
/// which is semantically equivalent to `Class _Nonnull`.
///
/// A nullable class is defined as `Option<&Class>`, which is semantically
/// equivalent to `Class _Nullable`.
#[repr(C)]
pub struct Class {
    // Compile-time classes are stored in the `__DATA` link section, which is
    // mutable. It is normally undefined behavior for shared references to point
    // to mutable data. We can inform Rust that this data is internally mutable
    // by using `UnsafeCell`.
    _priv: UnsafeCell<[u8; 0]>,
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for Class {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for Class {}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Class").field(&self.name()).finish()
    }
}

impl PartialEq for Class {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for Class {}

impl PartialOrd for Class {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Class {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self as *const Self).cmp(&(other as *const Self))
    }
}

impl hash::Hash for Class {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        (self as *const Self).hash(state);
    }
}

impl Class {
    /// Returns the class definition of a specified class, or `None` if the
    /// class is not registered with the Objective-C runtime.
    #[inline]
    pub fn get(name: &CStr) -> Option<&'static Class> {
        unsafe { objc_getClass(name.as_ptr()) }
    }

    /// Returns the number of classes registered with the Objective-C runtime.
    #[inline]
    pub fn count() -> usize {
        unsafe { objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// Returns all classes registered with the Objective-C runtime.
    pub fn all() -> Vec<&'static Class> {
        let len = Self::count();

        let mut all = Vec::<&'static Class>::with_capacity(len);
        unsafe {
            objc_getClassList(all.as_mut_ptr(), len as c_int);
            all.set_len(len);
        }

        all
    }

    /// Returns the name of this class.
    #[inline]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(class_getName(self)) }
    }

    /// Returns this class's superclass, or `None` if this is a root class
    /// (e.g. [`NSObject`](struct.NSObject.html)).
    #[inline]
    pub fn superclass(&self) -> Option<&Class> {
        unsafe { class_getSuperclass(self) }
    }

    /// Returns the size of instances of this class.
    #[inline]
    pub fn instance_size(&self) -> usize {
        unsafe { class_getInstanceSize(self) }
    }
}

extern "C" {
    fn objc_getClass(name: *const c_char) -> Option<&'static Class>;
    fn objc_getClassList(buf: *mut &'static Class, buf_len: c_int) -> c_int;

    fn class_getName(class: &Class) -> *const c_char;
    fn class_getSuperclass(class: &Class) -> Option<&Class>;
    fn class_getInstanceSize(class: &Class) -> usize;
}
