//! `NSError` codes exported by Foundation.

// TODO: Use intra-doc link for `NSError` above.
// See https://github.com/rust-lang/rust/issues/43466.

// TODO: Document these error codes.
#![allow(missing_docs, non_upper_case_globals)]

use crate::objc::NSInteger;

macro_rules! codes {
    ($($name:ident = $value:expr,)+) => {
        $(
            pub const $name: NSInteger = $value;
        )+
    };
}

// NSCocoaErrorDomain
codes! {
    // File system and file I/O related errors,
    NSFileNoSuchFileError = 4,
    NSFileLockingError = 255,
    NSFileReadUnknownError = 256,
    NSFileReadNoPermissionError = 257,
    NSFileReadInvalidFileNameError = 258,
    NSFileReadCorruptFileError = 259,
    NSFileReadNoSuchFileError = 260,
    NSFileReadInapplicableStringEncodingError = 261,
    NSFileReadUnsupportedSchemeError = 262,
    NSFileReadTooLargeError = 263,
    NSFileReadUnknownStringEncodingError = 264,
    NSFileWriteUnknownError = 512,
    NSFileWriteNoPermissionError = 513,
    NSFileWriteInvalidFileNameError = 514,
    NSFileWriteFileExistsError = 516,
    NSFileWriteInapplicableStringEncodingError = 517,
    NSFileWriteUnsupportedSchemeError = 518,
    NSFileWriteOutOfSpaceError = 640,
    NSFileWriteVolumeReadOnlyError = 642,

    // NSFileManager unmount errors
    NSFileManagerUnmountUnknownError = 768,
    NSFileManagerUnmountBusyError = 769,

    // Other errors
    NSKeyValueValidationError = 1024,
    NSFormattingError = 2048,
    NSUserCancelledError = 3072,
    NSFeatureUnsupportedError = 3328,

    // Executable loading errors
    NSExecutableNotLoadableError = 3584,
    NSExecutableArchitectureMismatchError = 3585,
    NSExecutableRuntimeMismatchError = 3586,
    NSExecutableLoadError = 3587,
    NSExecutableLinkError = 3588,

    // Inclusive error range definitions,
    NSFileErrorMinimum = 0,
    NSFileErrorMaximum = 1023,

    NSValidationErrorMinimum = 1024,
    NSValidationErrorMaximum = 2047,

    NSExecutableErrorMinimum = 3584,
    NSExecutableErrorMaximum = 3839,

    NSFormattingErrorMinimum = 2048,
    NSFormattingErrorMaximum = 2559,

    NSPropertyListReadCorruptError = 3840,
    NSPropertyListReadUnknownVersionError = 3841,
    NSPropertyListReadStreamError = 3842,
    NSPropertyListWriteStreamError = 3851,
    NSPropertyListWriteInvalidError = 3852,

    NSPropertyListErrorMinimum = 3840,
    NSPropertyListErrorMaximum = 4095,

    NSXPCConnectionInterrupted = 4097,
    NSXPCConnectionInvalid = 4099,
    NSXPCConnectionReplyInvalid = 4101,

    NSXPCConnectionErrorMinimum = 4096,
    NSXPCConnectionErrorMaximum = 4224,

    NSUbiquitousFileUnavailableError = 4353,
    NSUbiquitousFileNotUploadedDueToQuotaError = 4354,
    NSUbiquitousFileUbiquityServerNotAvailable = 4355,

    NSUbiquitousFileErrorMinimum = 4352,
    NSUbiquitousFileErrorMaximum = 4607,

    NSUserActivityHandoffFailedError = 4608,
    NSUserActivityConnectionUnavailableError = 4609,
    NSUserActivityRemoteApplicationTimedOutError = 4610,
    NSUserActivityHandoffUserInfoTooLargeError = 4611,

    NSUserActivityErrorMinimum = 4608,
    NSUserActivityErrorMaximum = 4863,

    NSCoderReadCorruptError = 4864,
    NSCoderValueNotFoundError = 4865,
    NSCoderInvalidValueError = 4866,
    NSCoderErrorMinimum = 4864,
    NSCoderErrorMaximum = 4991,

    NSBundleErrorMinimum = 4992,
    NSBundleErrorMaximum = 5119,

    NSBundleOnDemandResourceOutOfSpaceError = 4992,
    NSBundleOnDemandResourceExceededMaximumSizeError = 4993,
    NSBundleOnDemandResourceInvalidTagError = 4994,

    NSCloudSharingNetworkFailureError = 5120,
    NSCloudSharingQuotaExceededError = 5121,
    NSCloudSharingTooManyParticipantsError = 5122,
    NSCloudSharingConflictError = 5123,
    NSCloudSharingNoPermissionError = 5124,
    NSCloudSharingOtherError = 5375,

    NSCloudSharingErrorMinimum = 5120,
    NSCloudSharingErrorMaximum = 5375,

    NSCompressionFailedError = 5376,
    NSDecompressionFailedError = 5377,

    NSCompressionErrorMinimum = 5376,
    NSCompressionErrorMaximum = 5503,
}

// NSURLErrorDomain
codes! {
    NSURLErrorUnknown = -1,
    NSURLErrorCancelled = -999,
    NSURLErrorBadURL = -1000,
    NSURLErrorTimedOut = -1001,
    NSURLErrorUnsupportedURL = -1002,
    NSURLErrorCannotFindHost = -1003,
    NSURLErrorCannotConnectToHost = -1004,
    NSURLErrorNetworkConnectionLost = -1005,
    NSURLErrorDNSLookupFailed = -1006,
    NSURLErrorHTTPTooManyRedirects = -1007,
    NSURLErrorResourceUnavailable = -1008,
    NSURLErrorNotConnectedToInternet = -1009,
    NSURLErrorRedirectToNonExistentLocation = -1010,
    NSURLErrorBadServerResponse = -1011,
    NSURLErrorUserCancelledAuthentication = -1012,
    NSURLErrorUserAuthenticationRequired = -1013,
    NSURLErrorZeroByteResource = -1014,
    NSURLErrorCannotDecodeRawData = -1015,
    NSURLErrorCannotDecodeContentData = -1016,
    NSURLErrorCannotParseResponse = -1017,
    NSURLErrorAppTransportSecurityRequiresSecureConnection = -1022,
    NSURLErrorFileDoesNotExist = -1100,
    NSURLErrorFileIsDirectory = -1101,
    NSURLErrorNoPermissionsToReadFile = -1102,
    NSURLErrorDataLengthExceedsMaximum = -1103,
    NSURLErrorFileOutsideSafeArea = -1104,

    // SSL errors
    NSURLErrorSecureConnectionFailed = -1200,
    NSURLErrorServerCertificateHasBadDate = -1201,
    NSURLErrorServerCertificateUntrusted = -1202,
    NSURLErrorServerCertificateHasUnknownRoot = -1203,
    NSURLErrorServerCertificateNotYetValid = -1204,
    NSURLErrorClientCertificateRejected = -1205,
    NSURLErrorClientCertificateRequired = -1206,
    NSURLErrorCannotLoadFromNetwork = -2000,

    // Download and file I/O errors
    NSURLErrorCannotCreateFile = -3000,
    NSURLErrorCannotOpenFile = -3001,
    NSURLErrorCannotCloseFile = -3002,
    NSURLErrorCannotWriteToFile = -3003,
    NSURLErrorCannotRemoveFile = -3004,
    NSURLErrorCannotMoveFile = -3005,
    NSURLErrorDownloadDecodingFailedMidStream = -3006,
    NSURLErrorDownloadDecodingFailedToComplete =-3007,

    NSURLErrorInternationalRoamingOff = -1018,
    NSURLErrorCallIsActive = -1019,
    NSURLErrorDataNotAllowed = -1020,
    NSURLErrorRequestBodyStreamExhausted = -1021,

    NSURLErrorBackgroundSessionRequiresSharedContainer = -995,
    NSURLErrorBackgroundSessionInUseByAnotherProcess = -996,
    NSURLErrorBackgroundSessionWasDisconnected = -997,
}
