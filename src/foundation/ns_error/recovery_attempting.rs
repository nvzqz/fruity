use super::NSError;
use crate::objc::{NSObject, NSUInteger, ObjCObject, BOOL, SEL};
use std::ffi::c_void;

// TODO: Create `NSObjectProtocol` and wrap that.
objc_object_wrapper! {
    /// A set of methods that provide options to recover from an error.
    ///
    /// This type is returned by
    /// [`NSError::recovery_attempter`](struct.NSError.html#method.recovery_attempter).
    ///
    /// This informal protocol provides methods that allow your application to
    /// attempt to recover from an error. These methods are invoked when an
    /// [`NSError`](struct.NSError.html) is returned that specifies the
    /// implementing object as the error recoveryAttempter and the user has
    /// selected one of the error’s localized recovery options.
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
    pub wrapper NSErrorRecoveryAttempting: NSObject;
}

impl NSErrorRecoveryAttempting {
    /// Attempts a recovery from an error noted in an application-modal dialog.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1416402-attemptrecovery).
    #[doc(alias = "attemptRecoveryFromError")]
    #[doc(alias = "attemptRecoveryFromError:optionIndex:")]
    pub fn attempt_recovery(&self, error: &NSError, recovery_option_index: NSUInteger) -> bool {
        let sel = selector!(attemptRecoveryFromError:optionIndex:);

        let this: &NSObject = &self.0;
        if !this.responds_to_selector(sel) {
            return false;
        }

        // - (BOOL)attemptRecoveryFromError:(NSError *)error
        //                      optionIndex:(NSUInteger)recoveryOptionIndex;
        unsafe {
            self.0
                ._msg_send_any_with::<_, BOOL>(sel, (error, recovery_option_index))
                .into()
        }
    }

    /// Attempts a recovery from an error noted in a document-modal sheet.
    ///
    /// See [documentation](https://developer.apple.com/documentation/objectivec/nsobject/1411071-attemptrecovery).
    #[doc(alias = "attemptRecoveryFromError")]
    #[doc(alias = "attemptRecoveryFromError:optionIndex:delegate:didRecoverSelector:contextInfo:")]
    pub unsafe fn attempt_recovery_with(
        &self,
        error: &NSError,
        recovery_option_index: NSUInteger,
        delegate: Option<&ObjCObject>,
        did_recover_selector: Option<SEL>,
        context_info: *mut c_void,
    ) {
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
        self.0._msg_send_any_with(
            sel,
            (
                error,
                recovery_option_index,
                delegate,
                did_recover_selector,
                context_info,
            ),
        )
    }
}
