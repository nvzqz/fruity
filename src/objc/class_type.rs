use super::{Class, ObjectType};

/// A type that represents an instance of a specific Objective-C class.
///
/// # Related Items
///
/// - [`objc::ObjectType`](crate::objc::ObjectType)
pub trait ClassType<'a>: ObjectType<'a> {
    /// Returns the Objective-C class that can be used to instantiate a new
    /// instance of `Self`.
    ///
    /// If the class is not available at runtime, a link error will occur during
    /// program launch.
    fn class() -> &'static Class;
}
