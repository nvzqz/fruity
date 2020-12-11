use super::{Class, ObjCObject};

/// An Objective-C object instance.
///
/// # Related Items
///
/// - [`objc::ClassType`](crate::objc::ClassType)
/// - [`core::ObjectType`](crate::core::ObjectType)
pub trait ObjectType<'data>: 'data + crate::core::ObjectType + AsRef<ObjCObject<'data>> {
    /// Casts `self` into a type-erased Objective-C object.
    #[inline]
    fn as_objc_object(&self) -> &ObjCObject<'data> {
        self.as_ref()
    }

    /// Returns the class that this object is an instance of.
    #[inline]
    fn class<'s>(&'s self) -> &'s Class where 'data: 's {
        self.as_objc_object().class()
    }
}
