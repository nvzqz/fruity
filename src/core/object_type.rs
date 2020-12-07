use super::Arc;
use std::ptr::NonNull;

/// An object from some runtime.
///
/// All objects have a lifetime tied to a retain count.
///
/// This is implemented using different functions for each runtime:
///
/// - Objective-C: `objc_retain` and `objc_release`
///
/// - Core Foundation: `CFRetain` and `CFRelease`
///
/// - libdispatch (a.k.a. Grand Central Dispatch): `dispatch_retain` and
///   `dispatch_release`
///
/// # Related Items
///
/// - [`objc::ObjectType`](crate::objc::ObjectType)
pub trait ObjectType: Sized {
    /// Increments the object's retain count and returns a smart pointer that
    /// automatically calls [`release`](Self::release) on
    /// [`Drop`].
    ///
    /// To avoid releasing on [`Drop`], call [`std::mem::forget`] on the result.
    #[must_use = "The retained object is immediately released if unused"]
    fn retain(obj: &Self) -> Arc<Self>;

    /// Decrements the object's retain count.
    ///
    /// # Safety
    ///
    /// The object must not be released after being deallocated, or else the
    /// program will either abort, read/write unowned memory, or trigger
    /// undefined behavior.
    unsafe fn release(obj: NonNull<Self>);
}
