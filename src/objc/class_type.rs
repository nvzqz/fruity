use super::{Class, ObjectType};

/// A type that represents an instance of a specific Objective-C class.
///
/// # Related Items
///
/// - [`objc::ObjectType`](crate::objc::ObjectType)
pub trait ClassType<'data>: ObjectType<'data> {
    /// Returns the Objective-C class that can be used to instantiate a new
    /// instance of `Self`.
    ///
    /// If the class is not available at runtime, a link error will occur during
    /// program launch since this uses the class symbol.
    ///
    /// # Safety
    ///
    /// Some functions like [`Class::get_ivar`] may segfault on classes that
    /// have not been registered with the ObjC runtime. If this is an issue, use
    /// [`ClassType::class`] instead.
    unsafe fn direct_class() -> &'static Class;

    /// Returns this type's class guaranteed to have been registered with the
    /// ObjC runtime.
    ///
    /// This is equivalent to `[MyClass class]`.
    fn class() -> &'static Class {
        // SAFETY: We're immediately registering this class with the runtime.
        let class = unsafe { <Self as ClassType>::direct_class() };
        class.as_object().class()
    }
}
