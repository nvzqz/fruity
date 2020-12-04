use super::NSError;
use crate::objc::{NSObject, NSUInteger, Object, ObjectType, BOOL, SEL};
use std::{ffi::c_void, fmt, ops::Deref};

/// A set of methods that provide options to recover from an error.
///
/// This type is returned by
/// [`NSError::recovery_attempter`](struct.NSError.html#method.recovery_attempter).
///
/// This informal protocol provides methods that allow your application to
/// attempt to recover from an error. These methods are invoked when an
/// [`NSError`](struct.NSError.html) is returned that specifies the implementing
/// object as the error recoveryAttempter and the user has selected one of the
/// error’s localized recovery options.
///
/// The method invoked depends on how the error is presented to the user:
///
/// - If the error is presented in a document-modal sheet,
///   [`attempt_recovery_with`](#method.attempt_recovery_with) is invoked.
///
/// - If the error is presented in an application-modal dialog,
///   [`attempt_recovery`](#method.attempt_recovery) is invoked.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nserror/nserrorrecoveryattempting).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSErrorRecoveryAttempting(NSObject);

unsafe impl ObjectType for NSErrorRecoveryAttempting {}

impl From<NSErrorRecoveryAttempting> for NSObject {
    #[inline]
    fn from(object: NSErrorRecoveryAttempting) -> Self {
        object.0
    }
}

impl AsRef<NSObject> for NSErrorRecoveryAttempting {
    #[inline]
    fn as_ref(&self) -> &NSObject {
        self
    }
}

impl Deref for NSErrorRecoveryAttempting {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        &self.0
    }
}

impl fmt::Pointer for NSErrorRecoveryAttempting {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl NSErrorRecoveryAttempting {
    /// Attempts a recovery from an error noted in an application-modal dialog.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1416402-attemptrecovery).
    pub fn attempt_recovery(&self, error: &NSError, recovery_option_index: NSUInteger) -> bool {
        #![allow(unused)]

        let sel = selector!(attemptRecoveryFromError:optionIndex:);

        let this: &NSObject = &self.0;
        if !this.responds_to_selector(sel) {
            return false;
        }

        // - (BOOL)attemptRecoveryFromError:(NSError *)error
        //                      optionIndex:(NSUInteger)recoveryOptionIndex;
        unsafe {
            self._msg_send_with::<_, BOOL>(sel, (error.as_object(), recovery_option_index))
                .into()
        }
    }

    /// Attempts a recovery from an error noted in a document-modal sheet.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1411071-attemptrecovery).
    pub unsafe fn attempt_recovery_with(
        &self,
        error: &NSError,
        recovery_option_index: NSUInteger,
        delegate: Option<&Object>,
        did_recover_selector: Option<SEL>,
        context_info: *mut c_void,
    ) {
        #![allow(unused)]

        let sel = selector!(
            attemptRecoveryFromError:
            optionIndex:
            delegate:
            didRecoverSelector:
            contextInfo:
        );

        let this: &NSObject = &self.0;
        if !this.responds_to_selector(sel) {
            return;
        }

        // - (void)attemptRecoveryFromError:(NSError *)error
        //                      optionIndex:(NSUInteger)recoveryOptionIndex
        //                         delegate:(id)delegate
        //               didRecoverSelector:(SEL)didRecoverSelector
        //                      contextInfo:(void *)contextInfo;
        self._msg_send_with(
            sel,
            (
                error.as_object(),
                recovery_option_index,
                delegate,
                did_recover_selector,
                context_info,
            ),
        )
    }
}
