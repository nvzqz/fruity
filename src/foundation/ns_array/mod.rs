use crate::core::Arc;
use crate::core::ObjectType;
use crate::objc::{NSObject, NSUInteger};

objc_subclass! {
    /// A static ordered collection of objects.
    ///
    // See [documentation](https://developer.apple.com/documentation/foundation/nsarray?language=objc)
    pub class NSArray<'data, T>: NSObject<'data>;
}

impl<'a, T: ObjectType> NSArray<'a, T> {
    /// The number of objects in the array.
    pub fn count(&self) -> NSUInteger {
        unsafe { _msg_send_any![self, count] }
    }

    /// Returns the object located at the specified index.
    pub fn object_at_index(&self, index: NSUInteger) -> Arc<T> {
        unsafe { _msg_send_any![self, object_at_index: index] }
    }
}
