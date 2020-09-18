use crate::{foundation::NSString, objc::ObjectType};
use std::fmt;

/// The context in which an [`NSError`](struct.NSError.html) code should be
/// understood.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nserrordomain).
#[repr(transparent)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NSErrorDomain(pub NSString);

unsafe impl ObjectType for NSErrorDomain {}

impl From<NSString> for NSErrorDomain {
    #[inline]
    fn from(string: NSString) -> Self {
        Self(string)
    }
}

impl From<NSErrorDomain> for NSString {
    #[inline]
    fn from(key: NSErrorDomain) -> Self {
        key.0
    }
}

impl AsRef<NSString> for NSErrorDomain {
    #[inline]
    fn as_ref(&self) -> &NSString {
        &self.0
    }
}

impl AsRef<NSErrorDomain> for NSErrorDomain {
    #[inline]
    fn as_ref(&self) -> &NSErrorDomain {
        self
    }
}

impl AsRef<NSErrorDomain> for NSString {
    #[inline]
    fn as_ref(&self) -> &NSErrorDomain {
        // SAFETY: Both types have equivalent memory representations.
        unsafe { &*(self as *const _ as *const _) }
    }
}

// Use `NSString` formatting.

impl fmt::Debug for NSErrorDomain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Display for NSErrorDomain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for NSErrorDomain {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! domain {
    (
        $(#[$docs:meta])+
        $fn:ident $value:ident
    ) => {
        $(#[$docs])+
        #[inline]
        pub fn $fn() -> &'static NSErrorDomain {
            extern "C" {
                static $value: NSErrorDomain;
            }
            unsafe { &$value }
        }
    };
}

impl NSErrorDomain {
    domain! {
        /// Cocoa errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nscocoaerrordomain).
        cocoa NSCocoaErrorDomain
    }

    domain! {
        /// POSIX/BSD errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsposixerrordomain).
        posix NSPOSIXErrorDomain
    }

    domain! {
        /// Mac OS 9/Carbon errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsosstatuserrordomain).
        os_status NSOSStatusErrorDomain
    }

    domain! {
        /// Mach errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsmacherrordomain).
        mach NSMachErrorDomain
    }

    // NSNetServices.h

    domain! {
        /// The error domain used by [`NSNetService`](struct.NSNetService.html)
        /// or the mach network layer.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsnetserviceserrordomain).
        ns_net_services NSNetServicesErrorDomain
    }

    // NSStream.h

    domain! {
        /// The error domain used by [`NSError`](struct.NSError.html) when
        /// reporting SOCKS errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsstreamsockserrordomain).
        stream_socks NSStreamSOCKSErrorDomain
    }

    domain! {
        /// The error domain used by [`NSError`](struct.NSError.html) when
        /// reporting SSL errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsstreamsocketsslerrordomain).
        stream_socket_ssl NSStreamSocketSSLErrorDomain
    }

    // NSURLError.h

    domain! {
        /// URL loading system errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsurlerrordomain).
        ns_url NSURLErrorDomain
    }

    // NSXMLParser.h

    domain! {
        /// An error in XML parsing.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsxmlparsererrordomain).
        ns_xml_parser NSXMLParserErrorDomain
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
        ui_document_browser UIDocumentBrowserErrorDomain
    }

    // UIGuidedAccess.h

    domain! {
        /// Returns `UIGuidedAccessErrorDomain`, which is currently
        /// undocumented.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiguidedaccesserrordomain).
        ui_guided_access UIGuidedAccessErrorDomain
    }

    // UIPrintError.h

    // TODO: Handle this being unavailable on tvOS.
    domain! {
        /// An error in printing via UIKit.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiprinterrordomain).
        ui_print UIPrintErrorDomain
    }

    // UIPrintError.h

    // TODO: Handle this being only available on iOS 13+.
    domain! {
        /// Scene-related errors.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uisceneerrordomain).
        ui_scene UISceneErrorDomain
    }
}
