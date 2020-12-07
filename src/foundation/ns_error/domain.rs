ns_string_wrapper! {
    /// The context in which an [`NSError`](struct.NSError.html) code should be
    /// understood.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nserrordomain).
    #[derive(Ord, PartialOrd, Eq, PartialEq)]
    pub wrapper NSErrorDomain;
}

macro_rules! domain {
    (
        $(#[$docs:meta])+
        $fn:ident $value:literal
    ) => {
        $(#[$docs])+
        #[inline]
        #[doc(alias = $value)]
        pub fn $fn() -> &'static NSErrorDomain {
            extern "C" {
                #[link_name = $value]
                static VALUE: NSErrorDomain;
            }
            unsafe { &VALUE }
        }
    };
}

impl NSErrorDomain {
    domain! {
        /// Cocoa errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nscocoaerrordomain).
        cocoa "NSCocoaErrorDomain"
    }

    domain! {
        /// POSIX/BSD errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsposixerrordomain).
        posix "NSPOSIXErrorDomain"
    }

    domain! {
        /// Mac OS 9/Carbon errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsosstatuserrordomain).
        os_status "NSOSStatusErrorDomain"
    }

    domain! {
        /// Mach errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsmacherrordomain).
        mach "NSMachErrorDomain"
    }

    // NSNetServices.h

    domain! {
        /// The error domain used by [`NSNetService`](struct.NSNetService.html)
        /// or the mach network layer.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsnetserviceserrordomain).
        ns_net_services "NSNetServicesErrorDomain"
    }

    // NSStream.h

    domain! {
        /// The error domain used by [`NSError`](struct.NSError.html) when
        /// reporting SOCKS errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsstreamsockserrordomain).
        stream_socks "NSStreamSOCKSErrorDomain"
    }

    domain! {
        /// The error domain used by [`NSError`](struct.NSError.html) when
        /// reporting SSL errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsstreamsocketsslerrordomain).
        stream_socket_ssl "NSStreamSocketSSLErrorDomain"
    }

    // NSURLError.h

    domain! {
        /// URL loading system errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsurlerrordomain).
        ns_url "NSURLErrorDomain"
    }

    // NSXMLParser.h

    domain! {
        /// An error in XML parsing.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsxmlparsererrordomain).
        ns_xml_parser "NSXMLParserErrorDomain"
    }
}

/// UIKit errors.
///
/// Requires the **`ui_kit`** feature flag.
#[cfg(feature = "ui_kit")]
impl NSErrorDomain {
    // UIDocumentBrowserViewController.h

    domain! {
        /// An error raised by the document browser.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uidocumentbrowsererrordomain).
        ui_document_browser "UIDocumentBrowserErrorDomain"
    }

    // UIGuidedAccess.h

    domain! {
        /// Returns `UIGuidedAccessErrorDomain`, which is currently
        /// undocumented.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiguidedaccesserrordomain).
        ui_guided_access "UIGuidedAccessErrorDomain"
    }

    // UIPrintError.h

    // TODO: Handle this being unavailable on tvOS.
    domain! {
        /// An error in printing via UIKit.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiprinterrordomain).
        ui_print "UIPrintErrorDomain"
    }

    // UIPrintError.h

    // TODO: Handle this being only available on iOS 13+.
    domain! {
        /// Scene-related errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uisceneerrordomain).
        ui_scene "UISceneErrorDomain"
    }
}
