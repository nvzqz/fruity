use super::{NSObject, Object, BOOL, SEL};
use std::{
    cmp,
    ffi::CStr,
    fmt, hash,
    ops::Deref,
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
pub struct Class(
    // `Object` stores static data in the `__DATA` section, which is needed to
    // store class data. Internally, this is accomplished with `UnsafeCell`.
    Object,
);

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl RefUnwindSafe for Class {}

impl Deref for Class {
    type Target = Object;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Object> for Class {
    #[inline]
    fn as_ref(&self) -> &Object {
        self
    }
}

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

    #[inline]
    pub(crate) fn alloc(&self) -> NSObject {
        unsafe { _msg_send![self, alloc] }
    }

    /// Returns a reference to this class as an Objective-C object.
    #[inline]
    pub const fn as_object(&self) -> &Object {
        &self.0
    }

    /// Returns `true` if this class implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    pub fn responds_to_selector(&self, selector: SEL) -> bool {
        unsafe { _msg_send![self, respondsToSelector:selector => BOOL] != 0 }
    }

    /// Returns `true` if instances of this class implement or inherit a method
    /// that can respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418555-instancesrespondtoselector).
    #[inline]
    pub fn instances_respond_to_selector(&self, selector: SEL) -> bool {
        unsafe { _msg_send![self, instancesRespondToSelector:selector => BOOL] != 0 }
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

    /// Returns an iterator over the superclasses of this class.
    #[inline]
    pub fn superclass_iter(&self) -> impl Iterator<Item = &Class> + Copy {
        #[derive(Copy, Clone)]
        struct Iter<'a>(&'a Class);

        impl<'a> Iterator for Iter<'a> {
            type Item = &'a Class;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let superclass = self.0.superclass()?;
                self.0 = superclass;
                Some(superclass)
            }
        }

        // There are no more superclasses after the root is reached.
        impl std::iter::FusedIterator for Iter<'_> {}

        Iter(self)
    }

    /// Returns the number of superclasses of this class.
    #[inline]
    pub fn superclass_count(&self) -> usize {
        self.superclass_iter().count()
    }

    /// Returns `true` if this class has a superclass.
    #[inline]
    pub fn is_subclass(&self) -> bool {
        self.superclass().is_some()
    }

    /// Returns `true` if this class is a subclass of, or identical to, the
    /// other class.
    pub fn is_subclass_of(&self, other: &Self) -> bool {
        if self == other {
            true
        } else {
            self.superclass_iter().any(|superclass| superclass == other)
        }
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
