use super::OSStatus;
use std::num::NonZeroI16;

/// A non-zero 16-bit error code.
///
/// This is the old counterpart to [`OSStatus`](super::OSStatus).
///
/// # Usage
///
/// In FFI code, this type is meant to be used as [`Option<OSErr>`](Option).
/// [`None`] becomes 0 (no error) because this type is
/// [`#[repr(transparent)]`]((https://doc.rust-lang.org/nomicon/other-reprs.html#reprtransparent))
/// over [`NonZeroI16`].
///
/// Rust bindings that call `Option<OSErr>`-returning functions should return
/// [`Result<T, OSErr>`](Result).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct OSErr(NonZeroI16);

impl From<NonZeroI16> for OSErr {
    #[inline]
    fn from(value: NonZeroI16) -> Self {
        Self(value)
    }
}

impl OSErr {
    /// Creates an instance from `value`, returning `None` if it is zero.
    #[inline]
    pub const fn new(value: i16) -> Option<Self> {
        match NonZeroI16::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Creates an instance from a non-zero `value`.
    #[inline]
    pub const fn new_non_zero(value: NonZeroI16) -> Self {
        Self(value)
    }

    /// Creates an instance from `value`, without checking if it is zero.
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[inline]
    pub const unsafe fn new_unchecked(value: i16) -> Self {
        Self(NonZeroI16::new_unchecked(value))
    }

    /// Converts an `OSStatus` instance to an `OSErr` if it's within the 16-bit
    /// range.
    #[inline]
    pub const fn from_os_status(status: OSStatus) -> Option<Self> {
        let value = status.value();

        if (value as i16 as i32) == value {
            // SAFETY: `OSStatus` can never have a zero value.
            Some(unsafe { Self::new_unchecked(value as i16) })
        } else {
            None
        }
    }

    /// Returns this error's integer value.
    #[inline]
    pub const fn value(self) -> i16 {
        self.0.get()
    }

    /// Returns this error's integer value.
    #[inline]
    pub const fn non_zero_value(self) -> NonZeroI16 {
        self.0
    }
}

/// Returns a constant `OSErr` from `$value`, or causes a compile error if zero.
macro_rules! os_err {
    ($value:expr) => {{
        const VALUE: (OSErr, /* array size */ usize) = match OSErr::new($value) {
            Some(error) => (error, 0),
            None => (unsafe { OSErr::new_unchecked(1) }, 1),
        };

        // Trick to cause a compile error if the array size is not zero.
        const _: [(); VALUE.1] = [];

        VALUE.0
    }};
}

/// Apple event manager errors.
impl OSErr {
    /// Data could not be coerced to the requested descriptor type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraecoercionfail?language=objc).
    #[doc(alias = "errAECoercionFail")]
    pub const AE_COERCION_FAIL: Self = os_err!(-1700);

    /// Descriptor was not found.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraedescnotfound?language=objc).
    #[doc(alias = "errAEDescNotFound")]
    pub const AE_DESC_NOT_FOUND: Self = os_err!(-1701);

    /// Data in an Apple event could not be read.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraecorruptdata?language=objc).
    #[doc(alias = "errAECorruptData")]
    pub const AE_CORRUPT_DATA: Self = os_err!(-1702);

    /// Wrong descriptor type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraewrongdatatype?language=objc).
    #[doc(alias = "errAEWrongDataType")]
    pub const AE_WRONG_DATA_TYPE: Self = os_err!(-1703);

    /// Not a valid descriptor.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenotaedesc?language=objc).
    #[doc(alias = "errAENotAEDesc")]
    pub const AE_NOT_AE_DESC: Self = os_err!(-1704);

    /// Operation involving a list item failed.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraebadlistitem?language=objc).
    #[doc(alias = "errAEBadListItem")]
    pub const AE_BAD_LIST_ITEM: Self = os_err!(-1705);

    /// Need a newer version of the Apple Event Manager.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenewerversion?language=objc).
    #[doc(alias = "errAENewerVersion")]
    pub const AE_NEWER_VERSION: Self = os_err!(-1706);

    /// The event is not in AppleEvent format.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenotappleevent?language=objc).
    #[doc(alias = "errAENotAppleEvent")]
    pub const AE_NOT_APPLE_EVENT: Self = os_err!(-1707);

    /// Event wasn’t handled by an Apple event handler.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeeventnothandled?language=objc).
    #[doc(alias = "errAEEventNotHandled")]
    pub const AE_EVENT_NOT_HANDLED: Self = os_err!(-1708);
    /// `AEResetTimer` was passed an invalid reply.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraereplynotvalid?language=objc).
    #[doc(alias = "errAEReplyNotValid")]
    pub const AE_REPLY_NOT_VALID: Self = os_err!(-1709);

    /// Invalid sending mode was passed.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeunknownsendmode?language=objc).
    #[doc(alias = "errAEUnknownSendMode")]
    pub const AE_UNKNOWN_SEND_MODE: Self = os_err!(-1710);

    /// User canceled out of wait loop for reply or receipt.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraewaitcanceled?language=objc).
    #[doc(alias = "errAEWaitCanceled")]
    pub const AE_WAIT_CANCELED: Self = os_err!(-1711);

    /// Apple event timed out.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraetimeout?language=objc).
    #[doc(alias = "errAETimeout")]
    pub const AE_TIMEOUT: Self = os_err!(-1712);

    /// No user interaction allowed.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenouserinteraction?language=objc).
    #[doc(alias = "errAENoUserInteraction")]
    pub const AE_NO_USER_INTERACTION: Self = os_err!(-1713);

    /// Wrong keyword for a special function.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenotaspecialfunction?language=objc).
    #[doc(alias = "errAENotASpecialFunction")]
    pub const AE_NOT_A_SPECIAL_FUNCTION: Self = os_err!(-1714);

    /// A required parameter was not accessed.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeparammissed?language=objc).
    #[doc(alias = "errAEParamMissed")]
    pub const AE_PARAM_MISSED: Self = os_err!(-1715);

    /// Unknown Apple event address type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeunknownaddresstype?language=objc).
    #[doc(alias = "errAEUnknownAddressType")]
    pub const AE_UNKNOWN_ADDRESS_TYPE: Self = os_err!(-1716);

    /// No handler found for an Apple event.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraehandlernotfound?language=objc).
    #[doc(alias = "errAEHandlerNotFound")]
    pub const AE_HANDLER_NOT_FOUND: Self = os_err!(-1717);

    /// Reply has not yet arrived.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraereplynotarrived?language=objc).
    #[doc(alias = "errAEReplyNotArrived")]
    pub const AE_REPLY_NOT_ARRIVED: Self = os_err!(-1718);

    /// Not a valid list index.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeillegalindex?language=objc).
    #[doc(alias = "errAEIllegalIndex")]
    pub const AE_ILLEGAL_INDEX: Self = os_err!(-1719);

    /// The range is not valid because it is impossible for a range to include
    /// the first and last objects that were specified;an example is a range in
    /// which the offset of the first object is greater than the offset of the
    /// last object.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeimpossiblerange?language=objc).
    #[doc(alias = "errAEImpossibleRange")]
    pub const AE_IMPOSSIBLE_RANGE: Self = os_err!(-1720);

    /// The number of operands provided for the `kAENOT` logical operator is not
    /// 1.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraewrongnumberargs?language=objc).
    #[doc(alias = "errAEWrongNumberArgs")]
    pub const AE_WRONG_NUMBER_ARGS: Self = os_err!(-1721);

    /// There is no object accessor function for the specified object class and container type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeaccessornotfound?language=objc).
    #[doc(alias = "errAEAccessorNotFound")]
    pub const AE_ACCESSOR_NOT_FOUND: Self = os_err!(-1723);

    /// The logical operator in a logical descriptor is not `kAEAND`, `kAEOR`,or
    /// `kAENOT`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenosuchlogical?language=objc).
    #[doc(alias = "errAENoSuchLogical")]
    pub const AE_NO_SUCH_LOGICAL: Self = os_err!(-1725);

    /// The descriptor in a test key is neither a comparison descriptor nor a
    /// logical descriptor.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraebadtestkey?language=objc).
    #[doc(alias = "errAEBadTestKey")]
    pub const AE_BAD_TEST_KEY: Self = os_err!(-1726);

    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenotanobjspec?language=objc).
    #[doc(alias = "errAENotAnObjSpec")]
    pub const AE_NOT_AN_OBJ_SPEC: Self = os_err!(-1727);

    /// Runtime resolution of an object failed.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenosuchobject?language=objc).
    #[doc(alias = "errAENoSuchObject")]
    pub const AE_NO_SUCH_OBJECT: Self = os_err!(-1728);

    /// An object-counting function returned a negative result.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraenegativecount?language=objc).
    #[doc(alias = "errAENegativeCount")]
    pub const AE_NEGATIVE_COUNT: Self = os_err!(-1729);

    /// The container for an Apple event object is specified by an empty list.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeemptylistcontainer?language=objc).
    #[doc(alias = "errAEEmptyListContainer")]
    pub const AE_EMPTY_LIST_CONTAINER: Self = os_err!(-1730);

    /// The object type isn’t recognized.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeunknownobjecttype?language=objc).
    #[doc(alias = "errAEUnknownObjectType")]
    pub const AE_UNKNOWN_OBJECT_TYPE: Self = os_err!(-1731);

    /// Recording is already on.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraerecordingisalreadyon?language=objc).
    #[doc(alias = "errAERecordingIsAlreadyOn")]
    pub const AE_RECORDING_IS_ALREADY_ON: Self = os_err!(-1732);

    /// Break out of all levels of `AEReceive` to the topmost (1.1 or greater).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraereceiveterminate?language=objc).
    #[doc(alias = "errAEReceiveTerminate")]
    pub const AE_RECEIVE_TERMINATE: Self = os_err!(-1733);

    /// Break out of lowest level only of `AEReceive` (1.1 or greater).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraereceiveescapecurrent?language=objc).
    #[doc(alias = "errAEReceiveEscapeCurrent")]
    pub const AE_RECEIVE_ESCAPE_CURRENT: Self = os_err!(-1734);

    /// Event has been filtered and should not be propagated (1.1 or greater).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeeventfiltered?language=objc).
    #[doc(alias = "errAEEventFiltered")]
    pub const AE_EVENT_FILTERED: Self = os_err!(-1735);

    /// Attempt to install handler in table for identical class and ID (1.1 or
    /// greater).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraeduplicatehandler?language=objc).
    #[doc(alias = "errAEDuplicateHandler")]
    pub const AE_DUPLICATE_HANDLER: Self = os_err!(-1736);

    /// Nesting violation while streaming.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraestreambadnesting?language=objc).
    #[doc(alias = "errAEStreamBadNesting")]
    pub const AE_STREAM_BAD_NESTING: Self = os_err!(-1737);

    /// Attempt to convert a stream that has already been converted.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraestreamalreadyconverted?language=objc).
    #[doc(alias = "errAEStreamAlreadyConverted")]
    pub const AE_STREAM_ALREADY_CONVERTED: Self = os_err!(-1738);

    /// Attempt to perform an invalid operation on a null descriptor.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraedescisnull?language=objc).
    #[doc(alias = "errAEDescIsNull")]
    pub const AE_DESC_IS_NULL: Self = os_err!(-1739);

    /// `AEBuildDesc` and related functions detected a syntax error.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraebuildsyntaxerror?language=objc).
    #[doc(alias = "errAEBuildSyntaxError")]
    pub const AE_BUILD_SYNTAX_ERROR: Self = os_err!(-1740);

    /// Buffer for `AEFlattenDesc` too small.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/erraebuffertoosmall?language=objc).
    #[doc(alias = "errAEBufferTooSmall")]
    pub const AE_BUFFER_TOO_SMALL: Self = os_err!(-1741);
}
