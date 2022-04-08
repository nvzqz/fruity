use crate::objc::{sys, Sel};
use std::{ffi::CStr, fmt};

mod description;

pub use description::*;

use super::TypeEncoding;

/// A pointer to the function of a method implementation.
///
/// The first argument is a pointer to `self` (that is, the memory for the
/// particular instance of this class, or, for a class method, a pointer to the
/// metaclass). The second argument is the method selector. The method arguments
/// follow.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/objective-c_runtime/imp?language=objc).
pub type Imp = unsafe extern "C" fn();

/// A method in a [`Class`](super::Class) definition.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/method?language=objc).
///
/// # Usage
///
/// This is an opaque type meant to be used behind a shared reference
/// `&Method`, which is semantically equivalent to `Method _Nonnull`.
///
/// A nullable method is defined as `Option<&Method>`, which is semantically
/// equivalent to `Method _Nullable`.
#[repr(C)]
pub struct Method {
    // Stores data that may be mutated behind a shared reference. Internal
    // mutability triggers undefined behavior without `UnsafeCell`.
    _data: std::cell::UnsafeCell<[u8; 0]>,
}

unsafe impl Send for Method {}
unsafe impl Sync for Method {}

// Although this uses `UnsafeCell`, it does not point to any Rust types.
impl std::panic::RefUnwindSafe for Method {}

impl fmt::Debug for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MethodDescription")
            .field("name", &self.name())
            .field("type_encoding", &self.type_encoding())
            .field("implementation", &self.implementation())
            .finish()
    }
}

impl Method {
    /// Returns the number of arguments accepted by this method.
    ///
    /// Methods that semantically take 0 arguments actually take 2: the receiver
    /// (`self`) and the selector.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418968-method_getnumberofarguments?language=objc).
    #[inline]
    #[doc(alias = "method_getNumberOfArguments")]
    pub fn num_args(&self) -> u32 {
        unsafe { sys::method_getNumberOfArguments(self) }
    }

    /// Returns the name of this method as a selector.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418758-method_getname?language=objc).
    #[inline]
    #[doc(alias = "method_getName")]
    pub fn name(&self) -> Sel {
        unsafe { sys::method_getName(self) }
    }

    /// Returns [`name`](Self::name) as a C string.
    #[inline]
    pub fn name_c_str(&self) -> &'static CStr {
        self.name().name()
    }

    /// Returns a C string describing this method's parameter and return types.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418488-method_gettypeencoding?language=objc).
    #[inline]
    #[doc(alias = "method_getTypeEncoding")]
    pub fn type_encoding(&self) -> Option<&TypeEncoding> {
        unsafe {
            let encoding = sys::method_getTypeEncoding(self);
            if encoding.is_null() {
                None
            } else {
                Some(TypeEncoding::from_ptr(encoding))
            }
        }
    }

    /// Returns a shared reference to this method's description structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418545-method_getdescription?language=objc).
    #[inline]
    #[doc(alias = "method_getDescription")]
    pub fn description(&self) -> &MethodDescription {
        unsafe { self.description_mut() }
    }

    /// Returns a mutable reference to this method's description structure.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418545-method_getdescription?language=objc).
    ///
    /// # Safety
    ///
    /// The structure must not be mutated such that it causes unsafety elsewhere
    /// in the Objective-C runtime or functions calling into the runtime.
    #[inline]
    #[doc(alias = "method_getDescription")]
    pub unsafe fn description_mut(&self) -> &mut MethodDescription<'_> {
        &mut *sys::method_getDescription(self)
    }

    /// Returns the implementation of this method.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418551-method_getimplementation?language=objc).
    #[inline]
    #[doc(alias = "method_getImplementation")]
    pub fn implementation(&self) -> Imp {
        unsafe { sys::method_getImplementation(self) }
    }

    /// Sets the implementation of this method, returning the previous
    /// implementation.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418707-method_setimplementation?language=objc).
    ///
    /// # Safety
    ///
    /// The implementation function must:
    ///
    /// - Be safe in the operations that it performs.
    ///
    /// - Have the correct the correct signature expected by the Objective-C
    ///   runtime and callers of this method:
    ///
    ///   - Take `self` (the method receiver) as the first argument.
    ///   - Take the method selector as the second argument.
    ///   - Take the method's arguments as subsequent arguments.
    ///   - Return a value of the correct type.
    #[inline]
    #[doc(alias = "method_setImplementation")]
    pub unsafe fn set_implementation(&self, imp: Imp) -> Imp {
        sys::method_setImplementation(self, imp)
    }

    /// Exchanges the implementations of two methods.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/1418769-method_exchangeimplementations?language=objc).
    ///
    /// # Safety
    ///
    /// This is unsafe for the same reasons as
    /// [`set_implementation`](Self::set_implementation).
    ///
    /// # Examples
    ///
    /// This is an atomic version of the following:
    ///
    /// ```rust
    /// # use fruity::objc::{Method, Imp};
    /// let m1: &Method = // ...
    /// # return;
    /// let m2: &Method = // ...
    /// # return;
    ///
    /// let imp1: Imp = m1.implementation();
    /// let imp2: Imp = m2.implementation();
    ///
    /// unsafe {
    ///     m1.set_implementation(imp2);
    ///     m2.set_implementation(imp1);
    /// }
    /// ```
    #[inline]
    #[doc(alias = "method_exchangeImplementations")]
    pub unsafe fn exchange_implementation(&self, other: &Self) {
        sys::method_exchangeImplementations(self, other);
    }
}
