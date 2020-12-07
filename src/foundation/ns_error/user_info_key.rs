ns_string_wrapper! {
    /// Keys that may exist in an [`NSError`](struct.NSError.html) user info
    /// dictionary.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserroruserinfokey).
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub wrapper NSErrorUserInfoKey;
}

macro_rules! key {
    (
        $(#[$docs:meta])+
        $fn:ident $value:ident
    ) => {
        $(#[$docs])+
        #[inline]
        pub fn $fn() -> &'static NSErrorUserInfoKey {
            extern "C" {
                static $value: NSErrorUserInfoKey;
            }
            unsafe { &$value }
        }
    };
}

impl NSErrorUserInfoKey {
    // TODO: For newer keys, consider detecting the minimum version.

    key! {
        /// The corresponding value is an [`NSURL`](struct.NSURL.html).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsurlerrorkey).
        ns_url NSURLErrorKey
    }

    key! {
        /// Contains the file path of the error.
        ///
        /// The corresponding value is an [`NSString`](struct.NSString.html).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsfilepatherrorkey).
        file_path NSFilePathErrorKey
    }

    key! {
        /// The corresponding value is an [`NSString`](struct.NSString.html)
        /// containing the localized help corresponding to the help button.
        ///
        /// If present, its value is returned by
        /// [`NSError::help_anchor`](struct.NSError.html#method.help_anchor).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nshelpanchorerrorkey).
        help_anchor NSHelpAnchorErrorKey
    }

    key! {
        /// The corresponding value is an [`NSString`](struct.NSString.html)
        /// containing the localized error description.
        ///
        /// If present, its value is returned by
        /// [`NSError::localized_description`](struct.NSError.html#method.localized_description).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nslocalizeddescriptionkey).
        localized_description NSLocalizedDescriptionKey
    }

    key! {
        /// Returns `NSDebugDescriptionErrorKey`, which is currently
        /// undocumented.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdebugdescriptionerrorkey).
        debug_description NSDebugDescriptionErrorKey
    }

    key! {
        /// Returns `NSLocalizedFailureErrorKey`, which is currently
        /// undocumented.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nslocalizedfailureerrorkey).
        localized_failure NSLocalizedFailureErrorKey
    }

    key! {
        /// The corresponding value is an [`NSString`](struct.NSString.html)
        /// containing the localized reason for the failure.
        ///
        /// If present, its value is returned by
        /// [`NSError::localized_failure_reason`](struct.NSError.html#method.localized_failure_reason).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nslocalizedfailurereasonerrorkey).
        localized_failure_reason NSLocalizedFailureReasonErrorKey
    }

    key! {
        /// The corresponding value is an
        /// [`NSArray`](struct.NSArray.html)`<`[`NSString`](struct.NSString.html)`>`
        /// containing the localized titles of buttons appropriate for displaying in
        /// an alert panel.
        ///
        /// If present, its value is returned by
        /// [`NSError::localized_recovery_options`](struct.NSError.html#method.localized_recovery_options).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nslocalizedrecoveryoptionserrorkey).
        localized_recovery_options NSLocalizedRecoveryOptionsErrorKey
    }

    key! {
        /// The corresponding value is an [`NSString`](struct.NSString.html)
        /// containing the localized recovery suggestion for the error.
        ///
        /// If present, its value is returned by
        /// [`NSError::localized_recovery_suggestion`](struct.NSError.html#method.localized_recovery_suggestion).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nslocalizedrecoverysuggestionerrorkey).
        localized_recovery_suggestion NSLocalizedRecoverySuggestionErrorKey
    }

    key! {
        /// The corresponding value is an object that conforms to the
        /// [`NSErrorRecoveryAttempting`](struct.NSErrorRecoveryAttempting.html)
        /// informal protocol.
        ///
        /// If present, its value is returned by
        /// [`NSError::recovery_attempter`](struct.NSError.html#method.recovery_attempter).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsrecoveryattemptererrorkey).
        recovery_attempter NSRecoveryAttempterErrorKey
    }

    key! {
        /// The corresponding value is an [`NSNumber`](struct.NSNumber.html)
        /// containing the [`NSStringEncoding`](struct.NSStringEncoding.html)
        /// value.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsrecoveryattemptererrorkey).
        string_encoding NSStringEncodingErrorKey
    }

    key! {
        /// The corresponding value is an error that was encountered in an
        /// underlying implementation and caused the error that the receiver
        /// represents to occur.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsrecoveryattemptererrorkey).
        underlying_error NSUnderlyingErrorKey
    }
}
