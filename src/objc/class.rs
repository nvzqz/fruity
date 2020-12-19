use super::{sys, Method, Property, Sel, BOOL};
use crate::core::{Arc, ObjectType};
use std::{
    cell::UnsafeCell,
    cmp,
    ffi::CStr,
    fmt, hash, mem,
    os::raw::{c_char, c_int},
    panic::RefUnwindSafe,
    ptr,
};

#[cfg(feature = "malloced")]
use malloced::Malloced;

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
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: UnsafeCell<[u8; 0]>,
}

// This type is used globally, so we must be able to share it across threads.
unsafe impl Sync for Class {}
unsafe impl Send for Class {}

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
    #[doc(alias = "objc_getClass")]
    pub fn get(name: &CStr) -> Option<&'static Class> {
        unsafe { objc_getClass(name.as_ptr()) }
    }

    /// Returns the number of classes registered with the Objective-C runtime.
    #[inline]
    pub fn count() -> usize {
        unsafe { objc_getClassList(ptr::null_mut(), 0) as usize }
    }

    /// Returns all classes registered with the Objective-C runtime.
    #[doc(alias = "objc_getClassList")]
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
    #[allow(unused)] // Used by `foundation`
    pub(crate) unsafe fn alloc<T: ObjectType>(&self) -> Arc<T> {
        // TODO: Add `cfg` use `objc_msgSend` on older platforms where this
        // symbol does not exist, such as macOS 10.10+.
        //
        // This may require reading the `-mmacosx-version-min` flag somehow.

        extern "C" {
            fn objc_alloc();
        }
        let objc_alloc: unsafe extern "C" fn() = objc_alloc;
        let objc_alloc: unsafe extern "C" fn(&Class) -> Arc<T> = mem::transmute(objc_alloc);

        objc_alloc(self)
    }

    /// Calls `[[self alloc] init]`.
    #[inline]
    pub(crate) unsafe fn alloc_init<T: ObjectType>(&self) -> Arc<T> {
        // TODO: Add `cfg` use `objc_msgSend` on older platforms where this
        // symbol does not exist.
        //
        // This may require reading the `-mmacosx-version-min` flag somehow.

        extern "C" {
            fn objc_alloc_init();
        }
        let objc_alloc_init: unsafe extern "C" fn() = objc_alloc_init;
        let objc_alloc_init: unsafe extern "C" fn(&Class) -> Arc<T> =
            mem::transmute(objc_alloc_init);

        objc_alloc_init(self)
    }

    /// Returns `true` if this class implements or inherits a method that can
    /// respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418956-nsobject/1418583-respondstoselector).
    #[inline]
    #[doc(alias = "respondsToSelector")]
    pub fn responds_to_selector(&self, selector: Sel) -> bool {
        unsafe { _msg_send_any_cached![self, respondsToSelector: selector => BOOL] }.into()
    }

    /// Returns `true` if instances of this class implement or inherit a method
    /// that can respond to a specified message.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1418555-instancesrespondtoselector).
    #[inline]
    #[doc(alias = "instancesRespondToSelector")]
    pub fn instances_respond_to_selector(&self, selector: Sel) -> bool {
        unsafe { _msg_send_any_cached![self, instancesRespondToSelector: selector => BOOL] }.into()
    }

    /// Returns the name of this class.
    #[inline]
    #[doc(alias = "class_getName")]
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(class_getName(self)) }
    }

    /// Returns this class's superclass, or `None` if this is a root class
    /// (e.g. [`NSObject`](struct.NSObject.html)).
    #[inline]
    #[doc(alias = "class_getSuperclass")]
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

    /// Returns a reference to the data for a class method defined by `name`, or
    /// `None` if this class or its superclasses do not implement a class method
    /// with the specified selector.
    ///
    /// Note that this function searches superclasses for implementations,
    /// whereas [`copy_class_method_list`](Self::copy_class_method_list) does
    /// not.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418887-class_getclassmethod?language=objc).
    #[inline]
    #[doc(alias = "class_getClassMethod")]
    pub fn get_class_method(&self, name: Sel) -> Option<&Method> {
        unsafe { sys::class_getClassMethod(self, name).as_ref() }
    }

    /// Returns the class methods implemented by this class, or `None` if this
    /// class implements no instance methods.
    ///
    /// To get the implementations of instance methods that may be implemented
    /// by superclasses, use [`get_class_method`](Self::get_class_method).
    ///
    /// This calls
    /// [`class_copyMethodList`](https://developer.apple.com/documentation/objectivec/1418490-class_copymethodlist?language=objc)
    /// on the metaclass of this class.
    #[cfg(feature = "malloced")]
    #[inline]
    #[doc(alias = "class_copyMethodList")]
    pub fn copy_class_method_list(&self) -> Option<Malloced<[&Method]>> {
        use std::{mem::MaybeUninit, os::raw::c_uint};

        // TODO: Move this function into `objc::sys` module.
        extern "C" {
            fn object_getClass(obj: *const Class) -> *const Class;
        }

        let superclass = unsafe { object_getClass(self) };

        let mut len = MaybeUninit::<c_uint>::uninit();
        unsafe {
            let data = sys::class_copyMethodList(superclass, len.as_mut_ptr());
            if data.is_null() {
                None
            } else {
                Some(Malloced::slice_from_raw_parts(
                    data.cast::<&Method>(),
                    len.assume_init() as usize,
                ))
            }
        }
    }

    /// Returns a reference to the data for an instance method defined by
    /// `name`, or `None` if this class or its superclasses do not implement an
    /// instance method with the specified selector.
    ///
    /// Note that this function searches superclasses for implementations,
    /// whereas [`copy_instance_method_list`](Self::copy_instance_method_list)
    /// does not.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418887-class_getclassmethod?language=objc).
    #[inline]
    #[doc(alias = "class_getInstanceMethod")]
    pub fn get_instance_method(&self, name: Sel) -> Option<&Method> {
        unsafe { sys::class_getInstanceMethod(self, name).as_ref() }
    }

    /// Returns the instance methods implemented by this class, or `None` if
    /// this class implements no instance methods.
    ///
    /// To get the implementations of instance methods that may be implemented
    /// by superclasses, use [`get_instance_method`](Self::get_instance_method).
    ///
    /// See [`documentation`](https://developer.apple.com/documentation/objectivec/1418490-class_copymethodlist?language=objc).
    #[cfg(feature = "malloced")]
    #[inline]
    #[doc(alias = "class_copyMethodList")]
    pub fn copy_instance_method_list(&self) -> Option<Malloced<[&Method]>> {
        use std::{mem::MaybeUninit, os::raw::c_uint};

        let mut len = MaybeUninit::<c_uint>::uninit();
        unsafe {
            let data = sys::class_copyMethodList(self, len.as_mut_ptr());
            if data.is_null() {
                None
            } else {
                Some(Malloced::slice_from_raw_parts(
                    data.cast::<&Method>(),
                    len.assume_init() as usize,
                ))
            }
        }
    }

    /// Returns a property of `self` with `name`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418597-class_getproperty).
    #[inline]
    #[doc(alias = "class_getProperty")]
    pub fn get_property<'a>(&'a self, name: &CStr) -> Option<&'a Property> {
        extern "C" {
            fn class_getProperty(class: &Class, name: *const c_char) -> Option<&Property>;
        }
        unsafe { class_getProperty(self, name.as_ptr()) }
    }

    /// Returns a `malloc`-ed list of properties declared by `self`.
    ///
    /// Any properties declared by superclasses are not included.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418553-class_copypropertylist).
    #[cfg(feature = "malloced")]
    #[inline]
    #[doc(alias = "class_copyPropertyList")]
    pub fn copy_property_list(&self) -> Option<Malloced<[&Property]>> {
        use std::{mem::MaybeUninit, os::raw::c_uint};

        extern "C" {
            fn class_copyPropertyList<'a>(
                class: &'a Class,
                out_count: *mut c_uint,
            ) -> *mut &'a Property;
        }

        let mut len = MaybeUninit::<c_uint>::uninit();
        unsafe {
            let data = class_copyPropertyList(self, len.as_mut_ptr());
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

extern "C" {
    fn objc_getClass(name: *const c_char) -> Option<&'static Class>;
    fn objc_getClassList(buf: *mut &'static Class, buf_len: c_int) -> c_int;

    fn class_getName(class: &Class) -> *const c_char;
    fn class_getSuperclass(class: &Class) -> Option<&Class>;
    fn class_getInstanceSize(class: &Class) -> usize;
}
