use crate::{
    foundation::NSString,
    objc::{Object, ObjectType},
};
use std::{fmt, ptr::NonNull};

/// The name of an [`NSException`](struct.NSException.html).
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsexceptionname).
#[repr(transparent)]
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NSExceptionName(NSString);

unsafe impl ObjectType for NSExceptionName {}

impl From<NSExceptionName> for NSString {
    #[inline]
    fn from(obj: NSExceptionName) -> Self {
        obj.0
    }
}

impl AsRef<NSString> for NSExceptionName {
    #[inline]
    fn as_ref(&self) -> &NSString {
        &self.0
    }
}

impl PartialEq<NSString> for NSExceptionName {
    #[inline]
    fn eq(&self, other: &NSString) -> bool {
        self.0 == *other
    }
}

impl PartialEq<NSExceptionName> for NSString {
    #[inline]
    fn eq(&self, other: &NSExceptionName) -> bool {
        *self == other.0
    }
}

// Use `NSString` formatting.

impl fmt::Debug for NSExceptionName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Pointer for NSExceptionName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl NSExceptionName {
    /// Creates a new instance wrapping `name`.
    #[inline]
    pub const fn new(name: NSString) -> Self {
        Self(name)
    }

    /// Creates an immutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSExceptionName` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSString::from_ptr(ptr))
    }

    /// Creates an immutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSExceptionName` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSString::from_non_null_ptr(ptr))
    }
}

macro_rules! name {
    (
        $(#[$docs:meta])+
        $fn:ident $value:ident
    ) => {
        $(#[$docs])+
        #[inline]
        pub fn $fn() -> &'static NSExceptionName {
            extern "C" {
                static $value: NSExceptionName;
            }
            unsafe { &$value }
        }
    };
}

/// Uncategorized exceptions.
impl NSExceptionName {
    name! {
        /// A generic name for an exception.
        ///
        /// You should typically use a more specific exception name.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsgenericexception).
        generic NSGenericException
    }

    name! {
        /// The exception raised when an internal assertion fails and implies an
        /// unexpected condition within the called code.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinternalinconsistencyexception).
        internal_inconsistency NSInternalInconsistencyException
    }

    name! {
        /// The exception raised when you pass an invalid argument to a method,
        /// such as a `nil` (null/`None`) pointer where a non-`nil` object is
        /// required.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidargumentexception).
        invalid_argument NSInvalidArgumentException
    }
}

/// [`NSString`](struct.NSString.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised when a string cannot be represented in a
        /// file-system or string encoding.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nscharacterconversionexception).
        character_conversion NSCharacterConversionException
    }

    name! {
        /// The exception raised when a string cannot be parsed as a property
        /// list.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsparseerrorexception).
        parse_error NSParseErrorException
    }
}

/// [`NSDecimalNumber`](struct.NSDecimalNumber.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised if there is an exactness error.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberexactnessexception).
        decimal_number_exactness NSDecimalNumberExactnessException
    }

    name! {
        /// The exception raised on overflow.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberoverflowexception).
        decimal_number_overflow NSDecimalNumberOverflowException
    }

    name! {
        /// The exception raised on underflow.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberunderflowexception).
        decimal_number_underflow NSDecimalNumberUnderflowException
    }

    name! {
        /// The exception raised on divide by zero.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdecimalnumberdividebyzeroexception).
        decimal_number_divide_by_zero NSDecimalNumberDivideByZeroException
    }
}

/// [`NSFileHandle`](struct.NSFileHandle.html) exceptions.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsfilehandle/exception_names).
impl NSExceptionName {
    name! {
        /// The exception raised if attempts to determine file-handle type fail
        /// or if attempts to read from a file or channel fail.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsfilehandleoperationexception).
        file_handle_operation NSFileHandleOperationException
    }
}

/// [`NSInvocation`](struct.NSInvocation.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised if the `result` method is called after the
        /// operation was cancelled.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidarchiveoperationexception).
        invocation_operation_cancelled NSInvocationOperationCancelledException
    }

    name! {
        /// The exception raised if the `result` method is called for an
        /// invocation method with a `void` return type.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvocationoperationvoidresultexception).
        invocation_operation_void_result NSInvocationOperationVoidResultException
    }
}

/// [`NSArchiver`](struct.NSArchiver.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised if there are problems initializing or encoding.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidarchiveoperationexception).
        inconsistent_archive NSInconsistentArchiveException
    }
}

/// [`NSKeyedArchiver`](struct.NSKeyedArchiver.html) exceptions.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nskeyedarchiver/keyed_archiving_exception_names).
impl NSExceptionName {
    name! {
        /// The exception raised if there is a problem creating an archive.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidarchiveoperationexception).
        invalid_archive_operation NSInvalidArchiveOperationException
    }

    name! {
        /// The exception raised if there is a problem extracting an archive.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidunarchiveoperationexception).
        invalid_unarchive_operation NSInvalidUnarchiveOperationException
    }
}

/// [`NSPort`](struct.NSPort.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised when a generic error occurred on receive.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsportreceiveexception).
        port_receive NSPortReceiveException
    }

    name! {
        /// The exception raised when a generic error occurred on send.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsportsendexception).
        port_send NSPortSendException
    }

    name! {
        /// The exception raised when a timeout set on a port expires during a
        /// send or receive operation.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsporttimeoutexception).
        port_timeout NSPortTimeoutException
    }
}

/// [`NSRange`](struct.NSRange.html) exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised when attempting to access outside the bounds of
        /// some data, such as beyond the end of a string.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsrangeexception).
        range NSRangeException
    }
}

/// Key value coding exceptions.
impl NSExceptionName {
    name! {
        /// The exception raised when a key value coding operation fails.
        ///
        /// `userInfo` keys are described in
        /// [NSUndefinedKeyException userInfo Keys](https://developer.apple.com/documentation/foundation/nsundefinedkeyexception).
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsundefinedkeyexception).
        undefined_key NSUndefinedKeyException
    }
}

/// Distributed object exceptions.
impl NSExceptionName {
    name! {
        /// The exception that occurs when an internal assertion fails and
        /// implies an unexpected condition within the distributed objects.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsdestinationinvalidexception).
        destination_invalid NSDestinationInvalidException
    }

    name! {
        /// The exception raised when the receive port of an
        /// [`NSConnection`](struct.NSConnection.html) has become invalid.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidreceiveportexception).
        invalid_receive_port NSInvalidReceivePortException
    }

    name! {
        /// The exception raised when the send port of an
        /// [`NSConnection`](struct.NSConnection.html) has become invalid.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidsendportexception).
        invalid_send_port NSInvalidSendPortException
    }

    name! {
        /// The exception raised when a remote object is accessed from a thread
        /// that should not access it.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsinvalidsendportexception).
        object_inaccessible NSObjectInaccessibleException
    }

    name! {
        /// The exception raised when the remote side of the
        /// [`NSConnection`](struct.NSConnection.html) refused to send the
        /// message to the object because the object has never been vended.
        ///
        /// See [documentation](https://developer.apple.com/documentation/foundation/nsobjectnotavailableexception).
        object_not_available NSObjectNotAvailableException
    }
}

// Defined after everything else in order to appear at the end of documentation.

/// AppKit exceptions.
///
/// Requires the **`app_kit`** feature flag.
#[cfg(feature = "app_kit")]
impl NSExceptionName {
    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstextlinetoolongexception).
        text_line_too_long NSTextLineTooLongException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstextnoselectionexception).
        text_no_selection NSTextNoSelectionException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nswordtableswriteexception).
        word_tables_write NSWordTablesWriteException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nswordtablesreadexception).
        word_tables_read NSWordTablesReadException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstextreadexception).
        text_read NSTextReadException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstextwriteexception).
        text_write NSTextWriteException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nspasteboardcommunicationexception).
        pasteboard_communication NSPasteboardCommunicationException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsprintingcommunicationexception).
        printing_communication NSPrintingCommunicationException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsabortmodalexception).
        abort_modal NSAbortModalException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsabortprintingexception).
        abort_printing NSAbortPrintingException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsillegalselectorexception).
        illegal_selector NSIllegalSelectorException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsappkitvirtualmemoryexception).
        app_kit_virtual_memory NSAppKitVirtualMemoryException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadrtfdirectiveexception).
        bad_rtf_directive NSBadRTFDirectiveException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadrtffonttableexception).
        bad_rtf_font_table NSBadRTFFontTableException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadrtfstylesheetexception).
        bad_rtf_style_sheet NSBadRTFStyleSheetException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstypedstreamversionexception).
        typed_stream_version NSTypedStreamVersionException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nstiffexception).
        tiff NSTIFFException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsprintpackageexception).
        print_package NSPrintPackageException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadrtfcolortableexception).
        bad_rtf_color_table NSBadRTFColorTableException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsdraggingexception).
        dragging NSDraggingException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nscolorlistioexception).
        color_list_io NSColorListIOException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nscolorlistnoteditableexception).
        color_list_not_editable NSColorListNotEditableException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadbitmapparametersexception).
        bad_bitmap_parameters NSBadBitmapParametersException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nswindowservercommunicationexception).
        window_server_communication NSWindowServerCommunicationException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsfontunavailableexception).
        font_unavailable NSFontUnavailableException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsppdincludenotfoundexception).
        ppd_include_not_found NSPPDIncludeNotFoundException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsppdparseexception).
        ppd_parse NSPPDParseException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsppdincludestackoverflowexception).
        ppd_include_stack_overflow NSPPDIncludeStackOverflowException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsppdincludestackunderflowexception).
        ppd_include_stack_underflow NSPPDIncludeStackUnderflowException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsrtfpropertystackoverflowexception).
        rtf_property_stack_overflow NSRTFPropertyStackOverflowException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsappkitignoredexception).
        app_kit_ignored NSAppKitIgnoredException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbadcomparisonexception).
        bad_comparison NSBadComparisonException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsimagecacheexception).
        image_cache NSImageCacheException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsnibloadingexception).
        nib_loading NSNibLoadingException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsbrowserillegaldelegateexception).
        browser_illegal_delegate NSBrowserIllegalDelegateException
    }

    name! {
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsaccessibilityexception).
        accessibility NSAccessibilityException
    }

    name! {
        /// An exception raised when there is already an
        /// [`NSPrintOperation`](../app_kit/struct.NSPrintOperation.html) in
        /// process.
        ///
        /// See [documentation](https://developer.apple.com/documentation/appkit/nsprintoperationexistsexception).
        print_operation_exists NSPrintOperationExistsException
    }
}

/// UIKit exceptions.
///
/// Requires the **`ui_kit`** feature flag.
#[cfg(feature = "ui_kit")]
impl NSExceptionName {
    name! {
        /// The exception raised when a view controller or the app returns `0`
        /// instead of a valid set of supported interface orientation values.
        ///
        /// It is also thrown if the orientation returned by a view controller's
        /// [`preferredInterfaceOrientationForPresentation`](https://developer.apple.com/documentation/uikit/uiviewcontroller/1621438-preferredinterfaceorientationfor)
        /// method does not match one of the view controller’s supported
        /// orientations.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiapplicationinvalidinterfaceorientationexception).
        invalid_interface_orientation UIApplicationInvalidInterfaceOrientationException
    }

    name! {
        /// The exception raised when a view controller hierarchy is
        /// inconsistent with the view hierarchy.
        ///
        /// See [documentation](https://developer.apple.com/documentation/uikit/uiviewcontrollerhierarchyinconsistencyexception).
        view_controller_hierarchy_inconsistency UIViewControllerHierarchyInconsistencyException
    }
}
