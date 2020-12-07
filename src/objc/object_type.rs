use super::{Class, ObjCObject};

/// An Objective-C object instance.
///
/// # Related Items
///
/// - [`objc::ClassType`](crate::objc::ClassType)
/// - [`core::ObjectType`](crate::core::ObjectType)
pub trait ObjectType: crate::core::ObjectType + AsRef<ObjCObject> {
    /// Casts `self` into a type-erased Objective-C object.
    #[inline]
    fn as_objc_object(&self) -> &ObjCObject {
        self.as_ref()
    }

    /// Returns the class that this object is an instance of.
    #[inline]
    fn class(&self) -> &Class {
        ObjCObject::class(self.as_ref())
    }
}
