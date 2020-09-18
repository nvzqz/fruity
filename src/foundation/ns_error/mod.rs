use super::NSString;
use crate::objc::{Class, NSInteger, NSObject, Object, ObjectType};
use std::{fmt, ops::Deref, ptr::NonNull};

mod domain;
mod recovery_attempting;
mod user_info_key;

pub use domain::*;
pub use recovery_attempting::*;
pub use user_info_key::*;

// TODO: Add error codes for Cocoa, Mach, and POSIX.

/// Information about an error condition including a domain, a domain-specific
/// error code, and application-specific information.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nserror).
///
/// # Formatting
///
/// The [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
/// implementation writes the result of
/// [`localized_description`](#method.localized_description).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSError(NSObject);

unsafe impl ObjectType for NSError {}

impl From<NSError> for NSObject {
    #[inline]
    fn from(obj: NSError) -> Self {
        obj.0
    }
}

impl Deref for NSError {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        &self.0
    }
}

// TODO: `fmt::Debug`

impl fmt::Display for NSError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.localized_description().fmt(f)
    }
}

impl fmt::Pointer for NSError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl NSError {
    /// Returns the `NSError` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSError"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Creates an immutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSError` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSObject::from_ptr(ptr))
    }

    /// Creates an immutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSError` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSObject::from_non_null_ptr(ptr))
    }

    // TODO: `new(domain: &NSErrorDomain, code: NSInteger, user_info: &NSDictionary<NSErrorUserInfoKey, id>) -> Self`
}

/// Getting error properties.
impl NSError {
    /// Returns the error code.
    ///
    /// Note that errors are domain-specific.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1409165-code).
    #[inline]
    pub fn code(&self) -> NSInteger {
        unsafe { _msg_send![self, code] }
    }

    /// Returns a string containing the error domain.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1413924-domain).
    #[inline]
    pub fn domain(&self) -> NSErrorDomain {
        unsafe { _msg_send![self, domain] }
    }

    // TODO: `userInfo`
}

/// Getting error user info.
impl NSError {
    /// Returns a string containing the localized description of the error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_description`](struct.NSErrorUserInfoKey.html#method.localized_description).
    /// If it doesn't exist, a default string is constructed from the domain and
    /// code.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1413924-domain).
    #[inline]
    pub fn localized_description(&self) -> NSString {
        unsafe { _msg_send![self, localizedDescription] }
    }

    /// Returns a string containing the localized explanation of the reason for
    /// the error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_failure_reason`](struct.NSErrorUserInfoKey.html#method.localized_failure_reason).
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1412752-localizedfailurereason).
    #[inline]
    pub fn localized_failure_reason(&self) -> Option<NSString> {
        unsafe { _msg_send![self, localizedFailureReason] }
    }

    // TODO: `localizedRecoveryOptions`

    /// Returns a string containing the localized recovery suggestion for the
    /// error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_recovery_suggestion`](struct.NSErrorUserInfoKey.html#method.localized_recovery_suggestion).
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1407500-localizedrecoverysuggestion).
    #[inline]
    pub fn localized_recovery_suggestion(&self) -> Option<NSString> {
        unsafe { _msg_send![self, localizedRecoverySuggestion] }
    }

    /// Returns the object in the user info dictionary corresponding to
    /// [`NSErrorUserInfoKey::recovery_attempter`](struct.NSErrorUserInfoKey.html#method.recovery_attempter).
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1408864-recoveryattempter).
    #[inline]
    pub fn recovery_attempter(&self) -> Option<NSErrorRecoveryAttempting> {
        unsafe { _msg_send![self, recoveryAttempter] }
    }

    /// Returns the object in the user info dictionary corresponding to
    /// [`NSErrorUserInfoKey::help_anchor`](struct.NSErrorUserInfoKey.html#method.help_anchor).
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1414718-helpanchor).
    #[inline]
    pub fn help_anchor(&self) -> Option<NSString> {
        unsafe { _msg_send![self, helpAnchor] }
    }
}

/// Providing error user info.
impl NSError {
    // TODO: Methods that use blocks:
    // - `userInfoValueProviderForDomain:`
    // - `setUserInfoValueProviderForDomain:provider:`
}
