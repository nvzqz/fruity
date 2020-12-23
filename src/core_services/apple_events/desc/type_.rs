use crate::core::FourCharCode;
use std::fmt;

/// Specifies the type of the data stored in an [`AEDesc`](super::AEDesc)
/// descriptor.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/desctype?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AEDescType(pub FourCharCode);

impl fmt::Debug for AEDescType {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format as escaped ASCII string.
        self.0.fmt(f)
    }
}

impl AEDescType {
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

/// Preferred numeric event descriptor types.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/1542872-numeric_descriptor_type_constant?language=objc).
impl AEDescType {
    /// Value: `shor`.
    ///
    /// 16-bit signed integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesint16?language=objc).
    #[doc(alias = "typeSInt16")]
    pub const I16: Self = Self::from_chars(*b"shor");

    /// Value: `ushr`.
    ///
    /// 16-bit unsigned integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeuint16?language=objc).
    #[doc(alias = "typeUInt16")]
    pub const U16: Self = Self::from_chars(*b"ushr");

    /// Value: `long`.
    ///
    /// 32-bit signed integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesint32?language=objc).
    #[doc(alias = "typeSInt32")]
    pub const I32: Self = Self::from_chars(*b"long");

    /// Value: `magn`.
    ///
    /// 32-bit unsigned integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeuint32?language=objc).
    #[doc(alias = "typeUInt32")]
    pub const U32: Self = Self::from_chars(*b"magn");

    /// Value: `comp`.
    ///
    /// 64-bit signed integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesint64?language=objc).
    #[doc(alias = "typeSInt64")]
    pub const I64: Self = Self::from_chars(*b"comp");

    /// Value: `ucom`.
    ///
    /// 64-bit unsigned integer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeuint64?language=objc).
    #[doc(alias = "typeUInt64")]
    pub const U64: Self = Self::from_chars(*b"ucom");

    /// Value: `sing`.
    ///
    /// 32-bit floating point value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeieee32bitfloatingpoint?language=objc).
    #[doc(alias = "typeIEEE32BitFloatingPoint")]
    pub const F32: Self = Self::from_chars(*b"sing");

    /// Value: `doub`.
    ///
    /// 64-bit floating point value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeieee64bitfloatingpoint?language=objc).
    #[doc(alias = "typeIEEE64BitFloatingPoint")]
    pub const F64: Self = Self::from_chars(*b"doub");

    /// Value: `ldbl`.
    ///
    /// 128-bit floating point value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/type128bitfloatingpoint?language=objc).
    #[doc(alias = "type128BitFloatingPoint")]
    pub const F128: Self = Self::from_chars(*b"ldbl");

    /// Value: `decm`.
    ///
    /// Decimal.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedecimalstruct?language=objc).
    #[doc(alias = "typeDecimalStruct")]
    pub const DECIMAL: Self = Self::from_chars(*b"decm");
}

impl AEDescType {
    /// Value: `****`.
    ///
    /// Matches any type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typewildcard?language=objc).
    #[doc(alias = "typeWildCard")]
    pub const WILDCARD: Self = Self::from_chars(*b"****");

    /// Value: `bool`.
    ///
    /// Boolean value—single byte with value 0 or 1.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeboolean?language=objc).
    #[doc(alias = "typeBoolean")]
    pub const BOOL: Self = Self::from_chars(*b"bool");

    /// Value: `true`.
    ///
    /// Value: `TRUE` Boolean value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetrue?language=objc).
    #[doc(alias = "typeTrue")]
    pub const TRUE: Self = Self::from_chars(*b"true");

    /// Value: `fals`.
    ///
    /// `FALSE` Boolean value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefalse?language=objc).
    #[doc(alias = "typeFalse")]
    pub const FALSE: Self = Self::from_chars(*b"fals");

    /// Value: `null`.
    ///
    /// A null data storage pointer. When resolving an object specifier, an
    /// object with a null storage pointer specifies the default container at
    /// the top of the container hierarchy.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typenull?language=objc).
    #[doc(alias = "typeNull")]
    pub const NULL: Self = Self::from_chars(*b"null");

    /// Value: `list`.
    ///
    /// List of descriptors.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeaelist?language=objc).
    #[doc(alias = "typeAEList")]
    pub const AE_LIST: Self = Self::from_chars(*b"list");

    /// Value: `reco`.
    ///
    /// List of keyword-specified descriptors.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeaerecord?language=objc).
    #[doc(alias = "typeAERecord")]
    pub const AE_RECORD: Self = Self::from_chars(*b"reco");

    /// Value: `aevt`.
    ///
    /// An apple event.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeappleevent?language=objc).
    #[doc(alias = "typeAppleEvent")]
    pub const APPLE_EVENT: Self = Self::from_chars(*b"aevt");

    /// Value: `evrc`.
    ///
    /// An event record.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeeventrecord?language=objc).
    #[doc(alias = "typeEventRecord")]
    pub const EVENT_RECORD: Self = Self::from_chars(*b"evrc");

    /// Value: `alis`.
    ///
    /// An AliasPtr, from a valid AliasHandle.
    #[doc(alias = "typeAlias")]
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/files?language=objc).
    #[deprecated = "use `FILE_URL` or `BOOKMARK_DATA` to refer to files"]
    pub const ALIAS: Self = Self::from_chars(*b"alis");

    /// Value: `enum`.
    ///
    /// Enumerated data.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeenumerated?language=objc).
    #[doc(alias = "typeEnumerated")]
    pub const ENUMERATED: Self = Self::from_chars(*b"enum");

    /// Value: `type`.
    ///
    /// OSType.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetype?language=objc).
    #[doc(alias = "typeType")]
    pub const TYPE: Self = Self::from_chars(*b"type");

    /// Value: `appa`.
    ///
    /// Process Manager launch parameters.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeappparameters?language=objc).
    #[doc(alias = "typeAppParameters")]
    pub const APP_PARAMETERS: Self = Self::from_chars(*b"appa");

    /// Value: `prop`.
    ///
    /// Apple event object property.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeproperty?language=objc).
    #[doc(alias = "typeProperty")]
    pub const PROPERTY: Self = Self::from_chars(*b"prop");

    /// Value: `fsrf`.
    ///
    /// File system reference. Used in preference to file system specifications
    /// (`FSS`).
    #[doc(alias = "typeFSRef")]
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/files?language=objc).
    #[deprecated = "use `FILE_URL` or `BOOKMARK` to refer to files"]
    pub const FS_REF: Self = Self::from_chars(*b"fsrf");

    /// Value: `bmrk`.
    ///
    /// The bytes of a `CFURLBookmarkData`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typebookmarkdata?language=objc).
    #[doc(alias = "typeBookmarkData")]
    pub const BOOKMARK_DATA: Self = Self::from_chars(*b"bmrk");

    /// Value: `keyw`.
    ///
    /// An Apple event keyword.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typekeyword?language=objc).
    #[doc(alias = "typeKeyword")]
    pub const KEYWORD: Self = Self::from_chars(*b"keyw"); /* OSType */

    /// Value: `sect`.
    ///
    /// Handle to a section record.
    #[doc(alias = "typeSectionH")]
    ///
    /// See [documentationhttps://developer.apple.com/documentation/coreservices/](#[).?language=objc
    #[deprecated]
    pub const SECTION_HANDLE: Self = Self::from_chars(*b"sect");

    /// Value: `sign`.
    ///
    /// Application signature.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeapplsignature?language=objc).
    #[doc(alias = "typeApplSignature")]
    pub const APPL_SIGNATURE: Self = Self::from_chars(*b"sign");

    /// Value: `qdrt`.
    #[allow(missing_docs)]
    #[doc(alias = "typeQDRectangle")]
    ///
    /// See [documentationhttps://developer.apple.com/documentation/coreservices/](#[).?language=objc
    #[deprecated]
    pub const QD_RECTANGLE: Self = Self::from_chars(*b"qdrt");

    /// Value: `fixd`.
    #[allow(missing_docs)]
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefixed?language=objc).
    #[doc(alias = "typeFixed")]
    pub const FIXED: Self = Self::from_chars(*b"fixd");

    /// Value: `psn `.
    ///
    /// A process serial number. See also [`AEAddressDesc`].
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeprocessserialnumber?language=objc).
    #[doc(alias = "typeProcessSerialNumber")]
    pub const PROCESS_SERIAL_NUMBER: Self = Self::from_chars(*b"psn ");

    /// Value: `furl`.
    ///
    /// A file URL. That is, the associated data consists of the bytes of a
    /// UTF-8 encoded URL with a scheme of "file".
    ///
    /// This type is appropriate for describing a file that may not yet
    /// exist—see [Technical Note 2022][tn2022] for more information.
    ///
    /// [tn2022]: https://developer.apple.com/library/archive/technotes/tn2022
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefileurl?language=objc).
    #[doc(alias = "typeFileURL")]
    pub const FILE_URL: Self = Self::from_chars(*b"furl");

    /// Value: `aprl`.
    ///
    /// An application by URL.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeapplicationurl?language=objc).
    #[doc(alias = "typeApplicationURL")]
    pub const APP_URL: Self = Self::from_chars(*b"aprl");
}

/// Preferred unicode text types. In both cases, there is no explicit null
/// termination or length byte.
impl AEDescType {
    /// Value: `ut16`.
    ///
    /// Big endian 16-bit Unicode
    /// ([UTF-16](https://en.wikipedia.org/wiki/UTF-16)) with optional
    /// byte-order-mark (BOM), or little endian UTF-16 with required BOM.
    ///
    /// [UTF-16]: https://en.wikipedia.org/wiki/UTF-16
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeutf16externalrepresentation?language=objc).
    #[doc(alias = "typeUTF16ExternalRepresentation")]
    pub const UTF16: Self = Self::from_chars(*b"ut16");

    /// Value: `utf8`.
    ///
    /// 8-bit Unicode ([UTF-8](https://en.wikipedia.org/wiki/UTF-8) encoding).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeutf8text?language=objc).
    #[doc(alias = "typeUTF8Text")]
    pub const UTF8: Self = Self::from_chars(*b"utf8");
}

/// Event descriptor types that are deprecated due to their lack of explicit
/// encoding or byte order definition. Please use [`UTF16`](Self::UTF16) or
/// [`UTF8`](Self::UTF8) instead.
#[deprecated = "use `UTF16` or `UTF8` instead"]
impl AEDescType {
    /// Value: `TEXT`.
    ///
    /// Unterminated string of system script characters.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typechar?language=objc).
    #[doc(alias = "typeChar")]
    pub const CHAR: Self = Self::from_chars(*b"TEXT");

    /// Value: `sutx`.
    ///
    /// Styled Unicode text. Not implemented.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typestyledunicodetext?language=objc).
    #[doc(alias = "typeStyledUnicodeText")]
    pub const STYLED_UNICODE_TEXT: Self = Self::from_chars(*b"sutx");

    /// Value: `encs`.
    ///
    /// Styled Unicode text. Not implemented.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeencodedstring?language=objc).
    #[doc(alias = "typeEncodedString")]
    pub const ENCODED_STRING: Self = Self::from_chars(*b"encs");

    /// Value: `utxt`.
    ///
    /// Unicode text. Native byte ordering, optional BOM.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeunicodetext?language=objc).
    #[doc(alias = "typeUnicodeText")]
    pub const UNICODE_TEXT: Self = Self::from_chars(*b"utxt");

    /// Value: `cstr`.
    ///
    /// C string—Mac OS Roman characters followed by a NULL byte.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecstring?language=objc).
    #[doc(alias = "typeCString")]
    pub const C_STRING: Self = Self::from_chars(*b"cstr");

    /// Value: `pstr`.
    ///
    /// Pascal string—unsigned length byte followed by Mac OS Roman characters.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepstring?language=objc).
    #[doc(alias = "typePString")]
    pub const P_STRING: Self = Self::from_chars(*b"pstr");
}

/// Apple event objects.
impl AEDescType {
    // TODO: Reference `keyAEContainer` doc item

    /// Value: `obj `.
    ///
    /// Specifies a descriptor used with the `keyAEContainer` keyword in a
    /// keyword-specified descriptor. The key data for the descriptor is an
    /// object specifier.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeobjectspecifier?language=objc).
    #[doc(alias = "typeObjectSpecifier")]
    pub const OBJECT_SPECIFIER: Self = Self::from_chars(*b"obj ");

    /// Value: `exmn`.
    ///
    /// Specifies a descriptor that acts as a placeholder for each of the
    /// successive elements in a container when the Apple Event Manager tests
    /// those elements one at a time. The descriptor has a null data storage
    /// pointer. This descriptor type is used only with `formTest`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeobjectbeingexamined?language=objc).
    #[doc(alias = "typeObjectBeingExamined")]
    pub const OBJECT_BEING_EXAMINED: Self = Self::from_chars(*b"exmn");

    /// Value: `ccnt`.
    ///
    /// Specifies a container for an element that demarcates one boundary in a
    /// range. The descriptor has a null data storage pointer. This descriptor
    /// type is used only with formRange.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecurrentcontainer?language=objc).
    #[doc(alias = "typeCurrentContainer")]
    pub const CURRENT_CONTAINER: Self = Self::from_chars(*b"ccnt");

    /// Value: `toke`.
    ///
    /// Specifies a descriptor whose data storage pointer refers to a structure
    /// of type `AEDisposeToken`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetoken?language=objc).
    #[doc(alias = "typeToken")]
    pub const TOKEN: Self = Self::from_chars(*b"toke");

    /// Value: `rel `.
    ///
    /// Specifies a descriptor whose data consists of one of the constants
    /// `kAENext` or kAEPrevious, which are described in `AEDisposeToken`. Used
    /// with `formRelativePosition`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typerelativedescriptor?language=objc).
    #[doc(alias = "typeRelativeDescriptor")]
    pub const RELATIVE_DESCRIPTOR: Self = Self::from_chars(*b"rel ");

    /// Value: `abso`.
    ///
    /// Specifies a descriptor whose data consists of one of the constants
    /// `kAEFirst`, `kAEMiddle`, `kAELast`, `kAEAny`, or `kAEAll`, which are
    /// described in `AEDisposeToken`. Used with `formAbsolutePosition`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeabsoluteordinal?language=objc).
    #[doc(alias = "typeAbsoluteOrdinal")]
    pub const ABSOLUTE_ORDINAL: Self = Self::from_chars(*b"abso");

    /// Value: `inde`.
    ///
    /// Specifies a descriptor whose data indicates an indexed position within a
    /// range of values.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeindexdescriptor?language=objc).
    #[doc(alias = "typeIndexDescriptor")]
    pub const INDEX_DESCRIPTOR: Self = Self::from_chars(*b"inde");

    /// Value: `rang`.
    ///
    /// Specifies a range descriptor that identifies two Apple event objects
    /// marking the beginning and end of a range of elements. The data for a
    /// range descriptor consists of two keyword-specified descriptors with the
    /// keywords `keyAERangeStart` and `keyAERangeStop`, respectively, which
    /// specify the first Apple event object in the desired range and the last
    /// Apple event object in the desired range.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typerangedescriptor?language=objc).
    #[doc(alias = "typeRangeDescriptor")]
    pub const RANGE_DESCRIPTOR: Self = Self::from_chars(*b"rang");

    /// Value: `logi`.
    ///
    /// Specifies a logical descriptor. Data is one of the constants described
    /// in `AEDisposeToken`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelogicaldescriptor?language=objc).
    #[doc(alias = "typeLogicalDescriptor")]
    pub const LOGICAL_DESCRIPTOR: Self = Self::from_chars(*b"logi");

    /// Value: `cmpd`.
    ///
    /// Specifies a comparison descriptor. Data is one of the constants
    /// described in `AEDisposeToken`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecompdescriptor?language=objc).
    #[doc(alias = "typeCompDescriptor")]
    pub const COMP_DESCRIPTOR: Self = Self::from_chars(*b"cmpd");

    /// Value: `ostl`.
    ///
    /// Specifies a descriptor whose data consists of a list of tokens. (Token
    /// is defined in `AEDisposeToken`.)
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeosltokenlist?language=objc).
    #[doc(alias = "typeOSLTokenList")]
    pub const OSL_TOKEN_LIST: Self = Self::from_chars(*b"ostl");
}

impl AEDescType {
    /// Value: `tTXT`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeaetext?language=objc).
    #[doc(alias = "typeAEText")]
    pub const AE_TEXT: Self = Self::from_chars(*b"tTXT");

    /// Value: `carc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typearc?language=objc).
    #[doc(alias = "typeArc")]
    pub const ARC: Self = Self::from_chars(*b"carc");

    /// Value: `best`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typebest?language=objc).
    #[doc(alias = "typeBest")]
    pub const BEST: Self = Self::from_chars(*b"best");

    /// Value: `ccel`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecell?language=objc).
    #[doc(alias = "typeCell")]
    pub const CELL: Self = Self::from_chars(*b"ccel");

    /// Value: `gcli`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeclassinfo?language=objc).
    #[doc(alias = "typeClassInfo")]
    pub const CLASS_INFO: Self = Self::from_chars(*b"gcli");

    /// Value: `clrt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecolortable?language=objc).
    #[doc(alias = "typeColorTable")]
    pub const COLOR_TABLE: Self = Self::from_chars(*b"clrt");

    /// Value: `ccol`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecolumn?language=objc).
    #[doc(alias = "typeColumn")]
    pub const COLUMN: Self = Self::from_chars(*b"ccol");

    /// Value: `tdas`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedashstyle?language=objc).
    #[doc(alias = "typeDashStyle")]
    pub const DASH_STYLE: Self = Self::from_chars(*b"tdas");

    /// Value: `tdta`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedata?language=objc).
    #[doc(alias = "typeData")]
    pub const DATA: Self = Self::from_chars(*b"tdta");

    /// Value: `cdrw`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedrawingarea?language=objc).
    #[doc(alias = "typeDrawingArea")]
    pub const DRAWING_AREA: Self = Self::from_chars(*b"cdrw");

    /// Value: `elin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeeleminfo?language=objc).
    #[doc(alias = "typeElemInfo")]
    pub const ELEM_INFO: Self = Self::from_chars(*b"elin");

    /// Value: `enum`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeenumeration?language=objc).
    #[doc(alias = "typeEnumeration")]
    pub const ENUMERATION: Self = Self::from_chars(*b"enum");

    /// Value: `EPS `.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeeps?language=objc).
    #[doc(alias = "typeEPS")]
    pub const EPS: Self = Self::from_chars(*b"EPS ");

    /// Value: `evin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeeventinfo?language=objc).
    #[doc(alias = "typeEventInfo")]
    pub const EVENT_INFO: Self = Self::from_chars(*b"evin");

    /// Value: `fwin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefinderwindow?language=objc).
    #[doc(alias = "typeFinderWindow")]
    pub const FINDER_WINDOW: Self = Self::from_chars(*b"fwin");

    /// Value: `fpnt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefixedpoint?language=objc).
    #[doc(alias = "typeFixedPoint")]
    pub const FIXED_POINT: Self = Self::from_chars(*b"fpnt");

    /// Value: `frct`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefixedrectangle?language=objc).
    #[doc(alias = "typeFixedRectangle")]
    pub const FIXED_RECTANGLE: Self = Self::from_chars(*b"frct");

    /// Value: `glin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegraphicline?language=objc).
    #[doc(alias = "typeGraphicLine")]
    pub const GRAPHIC_LINE: Self = Self::from_chars(*b"glin");

    /// Value: `cgtx`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegraphictext?language=objc).
    #[doc(alias = "typeGraphicText")]
    pub const GRAPHIC_TEXT: Self = Self::from_chars(*b"cgtx");

    /// Value: `cpic`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegroupedgraphic?language=objc).
    #[doc(alias = "typeGroupedGraphic")]
    pub const GROUPED_GRAPHIC: Self = Self::from_chars(*b"cpic");

    /// Value: `insl`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeinsertionloc?language=objc).
    #[doc(alias = "typeInsertionLoc")]
    pub const INSERTION_LOC: Self = Self::from_chars(*b"insl");

    /// Value: `itxt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeintltext?language=objc).
    #[doc(alias = "typeIntlText")]
    pub const INTL_TEXT: Self = Self::from_chars(*b"itxt");

    /// Value: `intl`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeintlwritingcode?language=objc).
    #[doc(alias = "typeIntlWritingCode")]
    pub const INTL_WRITING_CODE: Self = Self::from_chars(*b"intl");

    /// Value: `ldt `.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongdatetime?language=objc).
    #[doc(alias = "typeLongDateTime")]
    pub const LONG_DATE_TIME: Self = Self::from_chars(*b"ldt ");

    /// Value: `cfat`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecfabsolutetime?language=objc).
    #[doc(alias = "typeCFAbsoluteTime")]
    pub const CF_ABSOLUTE_TIME: Self = Self::from_chars(*b"cfat");

    /// Value: `isot`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeiso8601datetime?language=objc).
    #[doc(alias = "typeISO8601DateTime")]
    pub const ISO8601_DATE_TIME: Self = Self::from_chars(*b"isot");

    /// Value: `lfxd`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongfixed?language=objc).
    #[doc(alias = "typeLongFixed")]
    pub const LONG_FIXED: Self = Self::from_chars(*b"lfxd");

    /// Value: `lfpt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongfixedpoint?language=objc).
    #[doc(alias = "typeLongFixedPoint")]
    pub const LONG_FIXED_POINT: Self = Self::from_chars(*b"lfpt");

    /// Value: `lfrc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongfixedrectangle?language=objc).
    #[doc(alias = "typeLongFixedRectangle")]
    pub const LONG_FIXED_RECTANGLE: Self = Self::from_chars(*b"lfrc");

    /// Value: `lpnt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongpoint?language=objc).
    #[doc(alias = "typeLongPoint")]
    pub const LONG_POINT: Self = Self::from_chars(*b"lpnt");

    /// Value: `lrct`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typelongrectangle?language=objc).
    #[doc(alias = "typeLongRectangle")]
    pub const LONG_RECTANGLE: Self = Self::from_chars(*b"lrct");

    /// Value: `mLoc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typemachineloc?language=objc).
    #[doc(alias = "typeMachineLoc")]
    pub const MACHINE_LOC: Self = Self::from_chars(*b"mLoc");

    /// Value: `covl`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeoval?language=objc).
    #[doc(alias = "typeOval")]
    pub const OVAL: Self = Self::from_chars(*b"covl");

    /// Value: `pmin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeparaminfo?language=objc).
    #[doc(alias = "typeParamInfo")]
    pub const PARAM_INFO: Self = Self::from_chars(*b"pmin");

    /// Value: `PICT`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepict?language=objc).
    #[doc(alias = "typePict")]
    pub const PICT: Self = Self::from_chars(*b"PICT");

    /// Value: `cpix`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepixelmap?language=objc).
    #[doc(alias = "typePixelMap")]
    pub const PIXEL_MAP: Self = Self::from_chars(*b"cpix");

    /// Value: `tpmm`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepixmapminus?language=objc).
    #[doc(alias = "typePixMapMinus")]
    pub const PIX_MAP_MINUS: Self = Self::from_chars(*b"tpmm");

    /// Value: `cpgn`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepolygon?language=objc).
    #[doc(alias = "typePolygon")]
    pub const POLYGON: Self = Self::from_chars(*b"cpgn");

    /// Value: `pinf`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepropinfo?language=objc).
    #[doc(alias = "typePropInfo")]
    pub const PROP_INFO: Self = Self::from_chars(*b"pinf");

    /// Value: `ptr `.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeptr?language=objc).
    #[doc(alias = "typePtr")]
    pub const PTR: Self = Self::from_chars(*b"ptr ");

    /// Value: `QDpt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeqdpoint?language=objc).
    #[doc(alias = "typeQDPoint")]
    pub const QD_POINT: Self = Self::from_chars(*b"QDpt");

    /// Value: `Qrgn`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeqdregion?language=objc).
    #[doc(alias = "typeQDRegion")]
    pub const QD_REGION: Self = Self::from_chars(*b"Qrgn");

    /// Value: `crec`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typerectangle?language=objc).
    #[doc(alias = "typeRectangle")]
    pub const RECTANGLE: Self = Self::from_chars(*b"crec");

    /// Value: `tr16`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typergb16?language=objc).
    #[doc(alias = "typeRGB16")]
    pub const RGB16: Self = Self::from_chars(*b"tr16");

    /// Value: `tr96`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typergb96?language=objc).
    #[doc(alias = "typeRGB96")]
    pub const RGB96: Self = Self::from_chars(*b"tr96");

    /// Value: `cRGB`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typergbcolor?language=objc).
    #[doc(alias = "typeRGBColor")]
    pub const RGB_COLOR: Self = Self::from_chars(*b"cRGB");

    /// Value: `trot`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typerotation?language=objc).
    #[doc(alias = "typeRotation")]
    pub const ROTATION: Self = Self::from_chars(*b"trot");

    /// Value: `crrc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeroundedrectangle?language=objc).
    #[doc(alias = "typeRoundedRectangle")]
    pub const ROUNDED_RECTANGLE: Self = Self::from_chars(*b"crrc");

    /// Value: `crow`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typerow?language=objc).
    #[doc(alias = "typeRow")]
    pub const ROW: Self = Self::from_chars(*b"crow");

    /// Value: `styl`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typescrapstyles?language=objc).
    #[doc(alias = "typeScrapStyles")]
    pub const SCRAP_STYLES: Self = Self::from_chars(*b"styl");

    /// Value: `scpt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typescript?language=objc).
    #[doc(alias = "typeScript")]
    pub const SCRIPT: Self = Self::from_chars(*b"scpt");

    /// Value: `STXT`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typestyledtext?language=objc).
    #[doc(alias = "typeStyledText")]
    pub const STYLED_TEXT: Self = Self::from_chars(*b"STXT");

    /// Value: `suin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesuiteinfo?language=objc).
    #[doc(alias = "typeSuiteInfo")]
    pub const SUITE_INFO: Self = Self::from_chars(*b"suin");

    /// Value: `ctbl`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetable?language=objc).
    #[doc(alias = "typeTable")]
    pub const TABLE: Self = Self::from_chars(*b"ctbl");

    /// Value: `tsty`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetextstyles?language=objc).
    #[doc(alias = "typeTextStyles")]
    pub const TEXT_STYLES: Self = Self::from_chars(*b"tsty");

    /// Value: `TIFF`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typetiff?language=objc).
    #[doc(alias = "typeTIFF")]
    pub const TIFF: Self = Self::from_chars(*b"TIFF");

    /// Value: `JPEG`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typejpeg?language=objc).
    #[doc(alias = "typeJPEG")]
    pub const JPEG: Self = Self::from_chars(*b"JPEG");

    /// Value: `GIFf`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegif?language=objc).
    #[doc(alias = "typeGIF")]
    pub const GIF: Self = Self::from_chars(*b"GIFf");

    /// Value: `vers`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeversion?language=objc).
    #[doc(alias = "typeVersion")]
    pub const VERSION: Self = Self::from_chars(*b"vers");

    /// Value: `mobj`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typehimenu?language=objc).
    #[doc(alias = "typeHIMenu")]
    pub const HI_MENU: Self = Self::from_chars(*b"mobj");

    /// Value: `wobj`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typehiwindow?language=objc).
    #[doc(alias = "typeHIWindow")]
    pub const HI_WINDOW: Self = Self::from_chars(*b"wobj");
}

/// Unit types.
impl AEDescType {
    /// Value: `metr`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typemeters?language=objc).
    #[doc(alias = "typeMeters")]
    pub const METERS: Self = Self::from_chars(*b"metr");

    /// Value: `inch`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeinches?language=objc).
    #[doc(alias = "typeInches")]
    pub const INCHES: Self = Self::from_chars(*b"inch");

    /// Value: `feet`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typefeet?language=objc).
    #[doc(alias = "typeFeet")]
    pub const FEET: Self = Self::from_chars(*b"feet");

    /// Value: `yard`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeyards?language=objc).
    #[doc(alias = "typeYards")]
    pub const YARDS: Self = Self::from_chars(*b"yard");

    /// Value: `mile`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typemiles?language=objc).
    #[doc(alias = "typeMiles")]
    pub const MILES: Self = Self::from_chars(*b"mile");

    /// Value: `kmtr`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typekilometers?language=objc).
    #[doc(alias = "typeKilometers")]
    pub const KILOMETERS: Self = Self::from_chars(*b"kmtr");

    /// Value: `cmtr`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecentimeters?language=objc).
    #[doc(alias = "typeCentimeters")]
    pub const CENTIMETERS: Self = Self::from_chars(*b"cmtr");

    /// Value: `sqrm`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesquaremeters?language=objc).
    #[doc(alias = "typeSquareMeters")]
    pub const SQUARE_METERS: Self = Self::from_chars(*b"sqrm");

    /// Value: `sqft`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesquarefeet?language=objc).
    #[doc(alias = "typeSquareFeet")]
    pub const SQUARE_FEET: Self = Self::from_chars(*b"sqft");

    /// Value: `sqyd`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesquareyards?language=objc).
    #[doc(alias = "typeSquareYards")]
    pub const SQUARE_YARDS: Self = Self::from_chars(*b"sqyd");

    /// Value: `sqmi`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesquaremiles?language=objc).
    #[doc(alias = "typeSquareMiles")]
    pub const SQUARE_MILES: Self = Self::from_chars(*b"sqmi");

    /// Value: `sqkm`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typesquarekilometers?language=objc).
    #[doc(alias = "typeSquareKilometers")]
    pub const SQUARE_KILOMETERS: Self = Self::from_chars(*b"sqkm");

    /// Value: `litr`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeliters?language=objc).
    #[doc(alias = "typeLiters")]
    pub const LITERS: Self = Self::from_chars(*b"litr");

    /// Value: `qrts`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typequarts?language=objc).
    #[doc(alias = "typeQuarts")]
    pub const QUARTS: Self = Self::from_chars(*b"qrts");

    /// Value: `galn`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegallons?language=objc).
    #[doc(alias = "typeGallons")]
    pub const GALLONS: Self = Self::from_chars(*b"galn");

    /// Value: `cmet`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecubicmeters?language=objc).
    #[doc(alias = "typeCubicMeters")]
    pub const CUBIC_METERS: Self = Self::from_chars(*b"cmet");

    /// Value: `cfet`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecubicfeet?language=objc).
    #[doc(alias = "typeCubicFeet")]
    pub const CUBIC_FEET: Self = Self::from_chars(*b"cfet");

    /// Value: `cuin`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecubicinches?language=objc).
    #[doc(alias = "typeCubicInches")]
    pub const CUBIC_INCHES: Self = Self::from_chars(*b"cuin");

    /// Value: `ccmt`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecubiccentimeter?language=objc).
    #[doc(alias = "typeCubicCentimeter")]
    pub const CUBIC_CENTIMETER: Self = Self::from_chars(*b"ccmt");

    /// Value: `cyrd`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typecubicyards?language=objc).
    #[doc(alias = "typeCubicYards")]
    pub const CUBIC_YARDS: Self = Self::from_chars(*b"cyrd");

    /// Value: `kgrm`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typekilograms?language=objc).
    #[doc(alias = "typeKilograms")]
    pub const KILOGRAMS: Self = Self::from_chars(*b"kgrm");

    /// Value: `gram`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typegrams?language=objc).
    #[doc(alias = "typeGrams")]
    pub const GRAMS: Self = Self::from_chars(*b"gram");

    /// Value: `ozs `.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typeounces?language=objc).
    #[doc(alias = "typeOunces")]
    pub const OUNCES: Self = Self::from_chars(*b"ozs ");

    /// Value: `lbs `.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typepounds?language=objc).
    #[doc(alias = "typePounds")]
    pub const POUNDS: Self = Self::from_chars(*b"lbs ");

    /// Value: `degc`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedegreesc?language=objc).
    #[doc(alias = "typeDegreesC")]
    pub const DEGREES_C: Self = Self::from_chars(*b"degc");

    /// Value: `degf`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedegreesf?language=objc).
    #[doc(alias = "typeDegreesF")]
    pub const DEGREES_F: Self = Self::from_chars(*b"degf");

    /// Value: `degk`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/typedegreesk?language=objc).
    #[doc(alias = "typeDegreesK")]
    pub const DEGREES_K: Self = Self::from_chars(*b"degk");
}
