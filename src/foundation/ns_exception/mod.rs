use super::NSString;
use crate::core::Arc;
use crate::objc::NSObject;

mod name;

pub use name::NSExceptionName;

/// A function pointer that can be used to perform last-minute logging before
/// the program terminates.
///
/// Used by
/// [`NSSetUncaughtExceptionHandler`](fn.NSGetUncaughtExceptionHandler.html) and
/// [`NSSetUncaughtExceptionHandler`](fn.NSSetUncaughtExceptionHandler.html).
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsuncaughtexceptionhandler).
pub type NSUncaughtExceptionHandler = unsafe extern "C" fn(NSException);

/// Returns the top-level error handler.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/1416853-nsgetuncaughtexceptionhandler).
#[inline]
#[allow(non_snake_case)]
pub fn NSGetUncaughtExceptionHandler() -> Option<NSUncaughtExceptionHandler> {
    extern "C" {
        fn NSGetUncaughtExceptionHandler() -> Option<NSUncaughtExceptionHandler>;
    }
    unsafe { NSGetUncaughtExceptionHandler() }
}

/// Changes the top-level error handler.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/1409609-nssetuncaughtexceptionhandler).
#[inline]
#[allow(non_snake_case)]
pub fn NSSetUncaughtExceptionHandler(handler: Option<NSUncaughtExceptionHandler>) {
    extern "C" {
        fn NSSetUncaughtExceptionHandler(handler: Option<NSUncaughtExceptionHandler>);
    }
    unsafe { NSSetUncaughtExceptionHandler(handler) }
}

objc_subclass! {
    /// A special condition that interrupts the normal flow of program
    /// execution.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsexception).
    pub class NSException: NSObject;
}

/// Creating and rasing exceptions.
impl NSException {
    /// Raises the receiver, causing program flow to jump to the local exception
    /// handler.
    ///
    /// When there are no exception handlers in the exception handler stack,
    /// unless the exception is raised during the posting of a notification,
    /// this method calls the uncaught exception handler, in which last-minute
    /// logging can be performed. The program then terminates, regardless of the
    /// actions taken by the uncaught exception handler.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsexception/1416135-raise).
    #[inline]
    #[doc(alias = "objc_exception_throw")]
    pub fn raise(&self) -> ! {
        // TODO(#7): Use "C-unwind" ABI when stable.
        extern "C" {
            fn objc_exception_throw(exception: &NSException) -> !;
        }
        unsafe { objc_exception_throw(self) }
    }
}

/// Querying an `NSException` object.
impl NSException {
    /// Returns a string used to uniquely identify `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsexception/1410925-name).
    #[inline]
    pub fn name(&self) -> Arc<NSExceptionName> {
        unsafe { _msg_send_any![self, name] }
    }

    /// Returns a containing a "human-readable" reason for `self`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsexception/1415537-reason).
    #[inline]
    pub fn reason(&self) -> Option<Arc<NSString>> {
        unsafe { _msg_send_any![self, reason] }
    }

    // TODO: `userInfo`
}

/// Getting exception stack frames.
impl NSException {
    // TODO: `callStackReturnAddresses`

    // TODO: `callStackSymbols`
}
