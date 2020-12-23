use crate::core::FourCharCode;
use std::fmt;

/// Specifies the event ID of an Apple event.
///
/// Apple events are identified by their event class and eventID attributes. The
/// event ID is the attribute that identifies a particularApple event within its
/// event class. In conjunction with the event class, the event ID uniquely
/// identifies the Apple event and communicates what action the Apple event
/// should perform.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/aeeventid?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AEEventID(pub FourCharCode);

impl fmt::Debug for AEEventID {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as escaped ASCII string.
        self.0.fmt(f)
    }
}

impl AEEventID {
    /// Returns an instance from the integer value.
    #[inline]
    pub const fn from_int(int: u32) -> Self {
        Self(FourCharCode::from_int(int))
    }

    /// Returns an instance from the 4-character code.
    #[inline]
    pub const fn from_chars(chars: [u8; 4]) -> Self {
        Self(FourCharCode::from_chars(chars))
    }

    /// Returns this descriptor's integer value.
    #[inline]
    pub const fn into_int(self) -> u32 {
        self.0.into_int()
    }

    /// Returns this descriptor's 4-character code.
    #[inline]
    pub const fn into_chars(self) -> [u8; 4] {
        self.0.into_chars()
    }
}

/// <span id="kCoreEventClass"></span>
/// Event IDs for [`AEEventClass::CORE`](super::AEEventClass::CORE).
impl AEEventID {
    /// Event that causes the application to quit.
    ///
    /// Value: `oapp`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeopenapplication?language=objc).
    #[doc(alias = "kAEOpenApplication")]
    pub const OPEN_APPLICATION: Self = Self::from_chars(*b"oapp");

    /// Event that provides an application with a list of documents to open.
    /// Sent, for example, when a selects one or more documents for your
    /// application in the Finder and double-clicks them.
    ///
    /// Value: `odoc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeopendocuments?language=objc).
    #[doc(alias = "kAEOpenDocuments")]
    pub const OPEN_DOCUMENTS: Self = Self::from_chars(*b"odoc");

    /// Event that provides an application with a list of documents to print.
    ///
    /// Value: `pdoc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeprintdocuments?language=objc).
    #[doc(alias = "kAEPrintDocuments")]
    pub const PRINT_DOCUMENTS: Self = Self::from_chars(*b"pdoc");

    /// Event that provides an application with dragged content, such as text or
    /// an image. Sent, for example, when a user drags an image file onto your
    /// application’s icon in the Dock. The application can use the content as
    /// desired—for example, if no document is currently open, it might open a
    /// new document and insert the provided text or image.
    ///
    /// Value: `ocon`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeopencontents?language=objc).
    #[doc(alias = "kAEOpenContents")]
    pub const OPEN_CONTENTS: Self = Self::from_chars(*b"ocon");

    /// Event that causes the application to quit.
    ///
    /// Value: `quit`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaequitapplication?language=objc).
    #[doc(alias = "kAEQuitApplication")]
    pub const QUIT_APPLICATION: Self = Self::from_chars(*b"quit");

    /// Event that is a reply Apple event.
    ///
    /// Value: `ansr`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeanswer?language=objc).
    #[doc(alias = "kAEAnswer")]
    pub const ANSWER: Self = Self::from_chars(*b"ansr");

    /// Event sent by the Process Manager to an application that launched
    /// another application when the launched application quits or terminates.
    ///
    /// Value: `obit`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeapplicationdied?language=objc).
    #[doc(alias = "kAEApplicationDied")]
    pub const APPLICATION_DIED: Self = Self::from_chars(*b"obit");

    /// Event sent by the macOS to a process when the user chooses the
    /// Preferences item for that process.
    ///
    /// Value: `pref`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeshowpreferences?language=objc).
    #[doc(alias = "kAEShowPreferences")]
    pub const SHOW_PREFERENCES: Self = Self::from_chars(*b"pref");
}
