use super::NSString;
use crate::core::Arc;
use crate::objc::{NSInteger, NSObject};
use std::fmt;

mod domain;
mod recovery_attempting;
mod user_info_key;

pub use domain::*;
pub use recovery_attempting::*;
pub use user_info_key::*;

// TODO: Add error codes for Cocoa, Mach, and POSIX.

objc_subclass! {
    /// Information about an error condition including a domain, a domain-specific
    /// error code, and application-specific information.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror).
    ///
    /// # Formatting
    ///
    /// The [`Display`](std::fmt::Display) implementation writes the result of
    /// [`localized_description`](Self::localized_description).
    pub class NSError<'data>: NSObject<'data>;
}

// TODO: `fmt::Debug`

impl fmt::Display for NSError<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.localized_description().fmt(f)
    }
}

impl NSError<'_> {
    // TODO: `new(domain: &NSErrorDomain, code: NSInteger, user_info: &NSDictionary<NSErrorUserInfoKey, id>) -> Arc<Self>`
}

/// Getting error properties.
impl NSError<'_> {
    /// Returns the error code.
    ///
    /// Note that errors are domain-specific.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1409165-code).
    #[inline]
    pub fn code(&self) -> NSInteger {
        unsafe { _msg_send_any![self, code] }
    }

    /// Returns a string containing the error domain.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1413924-domain).
    #[inline]
    pub fn domain(&self) -> Arc<NSErrorDomain> {
        unsafe { _msg_send_any![self, domain] }
    }

    // TODO: `userInfo`
}

/// Getting error user info.
impl NSError<'_> {
    /// Returns a string containing the localized description of the error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_description`]. If it doesn't exist, a
    /// default string is constructed from the domain and code.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1413924-domain).
    #[inline]
    #[doc(alias = "localizedDescription")]
    pub fn localized_description(&self) -> Arc<NSString> {
        unsafe { _msg_send_any![self, localizedDescription] }
    }

    /// Returns a string containing the localized explanation of the reason for
    /// the error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_failure_reason`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1412752-localizedfailurereason).
    #[inline]
    #[doc(alias = "localizedFailureReason")]
    pub fn localized_failure_reason(&self) -> Option<Arc<NSString>> {
        unsafe { _msg_send_any![self, localizedFailureReason] }
    }

    // TODO: `localizedRecoveryOptions`

    /// Returns a string containing the localized recovery suggestion for the
    /// error.
    ///
    /// This is the object in the user info dictionary for
    /// [`NSErrorUserInfoKey::localized_recovery_suggestion`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1407500-localizedrecoverysuggestion).
    #[inline]
    #[doc(alias = "localizedRecoverySuggestion")]
    pub fn localized_recovery_suggestion(&self) -> Option<Arc<NSString>> {
        unsafe { _msg_send_any![self, localizedRecoverySuggestion] }
    }

    /// Returns the object in the user info dictionary corresponding to
    /// [`NSErrorUserInfoKey::recovery_attempter`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1408864-recoveryattempter).
    #[inline]
    #[doc(alias = "recoveryAttempter")]
    pub fn recovery_attempter(&self) -> Option<Arc<NSErrorRecoveryAttempting>> {
        unsafe { _msg_send_any![self, recoveryAttempter] }
    }

    /// Returns the object in the user info dictionary corresponding to
    /// [`NSErrorUserInfoKey::help_anchor`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserror/1414718-helpanchor).
    #[inline]
    #[doc(alias = "helpAnchor")]
    pub fn help_anchor(&self) -> Option<Arc<NSString>> {
        unsafe { _msg_send_any![self, helpAnchor] }
    }
}

/// Providing error user info.
impl NSError<'_> {
    // TODO: Methods that use blocks:
    // - `userInfoValueProviderForDomain:`
    // - `setUserInfoValueProviderForDomain:provider:`
}
