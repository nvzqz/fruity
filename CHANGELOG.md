# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][crate]

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added

- Internal macros to simplify creating class types and wrappers:

  - `subclass!` performs all the wrapping and trait impls (e.g. `obj::Object`,
    `Deref`) to make a new subclass easy to declare. This macro is
    runtime-agnostic.

  - `object_wrapper!` for thin wrappers over object types. Unlike `subclass!`,
    this is not meant for creating a new class type.

  - `objc_class_type!` implements `objc::ClassType` using the given class name.

  - `objc_subclass!` calls `subclass!` and `objc_class_type!`.

  - `objc_object_wrapper!` calls `object_wrapper!` and implements the
    `objc::ObjectType` trait.

  - `ns_string_wrapper!` calls `objc_object_wrapper!` with
    `foundation::NSString` and implements the `Debug` and `Display` traits.

- `objc::ClassType` trait for types whose classes are statically available.

- Improvements to `ns_string!` macro:

  - Can take `const X: &str` as input, not just string literals.

  - Allows interior null bytes, transcoding the string to UTF-16.

  - Allows trailing null bytes and uses the constant as-is.

    This makes it possible for input data to not get emitted in the binary twice.
    The compiler currently does not coalesce string prefixes and instead emits the
    same prefix data twice.

- Internal convenience `_msg_send!` macro for dispatching `objc_msgSend` and
  associating arguments with the appropriate selector part.

  - Also `_msg_send_cached!` that internally caches the selector in a global
    atomic pointer.

- Created private `common` module for types exposed in multiple public modules:

  - `NSDirectionalEdgeInsets`

- Created `core` module for core types and traits are not specific to any
  wrapped library:

  - `ObjectType` trait to generalize retain/release.

  - `Arc` type to handle automatic reference counting. Built on top of the
    methods in `ObjectType`.

    - Implements `Default`:

      - `Arc<T>` where `&T` implements `Default` by retaining the value.

      - `Arc<NSObject>` from `[[NSObject alloc] init]`.

      - `Arc<NSMutableString>` from `[[NSMutableString alloc] init]`.

  - `OSErr` and `OSStatus` non-zero structs for error codes.

  - `FourCharCode` struct for
    [four-character codes](https://en.wikipedia.org/wiki/FourCC).

  - `OSType` alias for `FourCharCode`.

- Created `core_graphics` module for
  [Core Graphics](https://developer.apple.com/documentation/coregraphics)
  framework:

  - Geometry types: `CGFloat`, `CGPoint`, `CGSize`, `CGRect`, `CGRectEdge`,
    `CGVector`, and `CGAffineTransform`.

    These are aliased in `foundation` as: `NSPoint`, `NSSize`, `NSRect`, and
    `NSRectEdge`.

- Created `app_kit` module for
  [AppKit](https://developer.apple.com/documentation/appkit)
  framework.

  - `NSAppKitVersion` type and version constants through 10.15.

- Created `dispatch` module for
  [Dispatch](https://developer.apple.com/documentation/dispatch)
  library:

  - `sys` module for raw unsafe C functions exposed by libdispatch.

  - Types: `DispatchObject`, `DispatchQueueBuilder`, `DispatchQueuePriority`,
    `DispatchQueueAttributes`, `DispatchTime`, `DispatchQos`,
    `DispatchQosClass`, `DispatchAutoreleaseFrequency`, `DispatchSource`,
    `DispatchSourceType`.

  - `DispatchQueue`

    - `current_queue_label_owned` as safe owned alternative to
      `current_queue_label`.

    - `with_current_queue_label` as safe scoped alternative to
      `current_queue_label`.

- Created `core_services` module for
  [Core Services](https://developer.apple.com/documentation/coreservices)
  framework.

  - `apple_events` module for low level Apple Events API in `AE.framework`:

    - `AEDescType`, `AEEventClass`, and `AEEventID` newtype structs over
      `FourCharCode`.

    - `AEReturnID` newtype struct over `i16`.

    - `AETransactionID` and `AESendMode` newtype structs over `i32`.

    - `AEDataStorage` and `AEDataStorageType` pointer type aliases.

- Created `core_animation` module for
  [Core Animation](https://developer.apple.com/documentation/quartzcore)
  framework.

- Created `core_image` module for
  [Core Image](https://developer.apple.com/documentation/coreimage)
  framework.

- Created `core_video` module for
  [Core Video](https://developer.apple.com/documentation/corevideo)
  framework.

- Created `core_audio` module for
  [Core Audio](https://developer.apple.com/documentation/coreaudio)
  framework.

- Created `core_text` module for
  [Core Text](https://developer.apple.com/documentation/coretext)
  framework.

- Created `system_configuration` module for
  [System Configuration](https://developer.apple.com/documentation/systemconfiguration)
  framework.

- Created `cf_network` module for
  [CFNetwork](https://developer.apple.com/documentation/cfnetwork)
  framework.

- Created `io_kit` module for
  [IOKit](https://developer.apple.com/documentation/iokit)
  framework.

- Added APIs to `foundation` module:

  - Constants: `NSNotFound`.

  - Types: `NSNumber`, `NSEdgeInsets`, `NSRange`, `NSValue`, `NSNull`,
    `NSException`, `NSExceptionName`, `NSError`, `NSErrorDomain`,
    `NSErrorUserInfoKey`, `NSErrorRecoveryAttempting`, `NSTimeInterval`.

  - `NSError` error codes.

  - Methods for `NSString`:

    - `length`

    - Efficiently getting an `Option<&str>` if it's UTF-8 or an `Option<&[u16]>` if it's UTF-16.

      Unlike `to_str` and friends, these do not allocate and transcode to a new
      string if the original string does not represent the encoding.

    - Efficiently comparing against `&str`.

    - Getting all available encodings: `available_encodings_slice`,
      `available_encodings_iter`, `available_encodings_ptr`.

    - `available_encodings_count` for number of available encodings. This calls
      `available_encodings_slice().len()`.

  - Methods for `NSStringEncoding`:

    - `name`.

    - Methods for getting all available `NSStringEncoding`s.

- Added APIs to `core_foundation` module:

  - Constants: `kCFNotFound`.

  - Types: `CFType`, `CFTypeRef`, `CFTypeID`, `CFOptionFlags`, `CFIndex`,
    `CFHashCode`, `CFComparisonResult`, `CFAllocator`, `CFAllocatorContext`,
    `CFNumber`, `CFNumberType`, `CFBoolean`.

  - Callback function type aliases for `CFAllocator`.

- Added APIs to `objc` module:

  - Pointer methods for `Sel`.

  - Implemented `PartialEq` for `NSObject`.

  - `autoreleasepool` function equivalent to `@autoreleasepool` that drains on
    panic.

  - `BOOL::NO` and `BOOL::YES` associated constants as alternatives to the
    freestanding constants. These should be preferred.

### Fixed

- Correct dispatching of the appropriate `objc_msgSend` based on return type.

- **\[breaking\]** The safety of `DispatchQueue::current_queue_label_owned` by
  marking it as `unsafe`. It is unspecified whether the label may outlive the
  current queue.

- The signed-ness of `BOOL` on platforms where it is a `signed char`.

- **\[breaking\]** The memory representation of `BOOL` on platforms where it is
  a C/C++ `Bool`.

- The `ns_string!` macro now transcodes non-ASCII strings to UTF-16, instead of
  allowing UTF-8 data where only ASCII data is expected (see issue [#3]).

  Transcoding was implemented by [@thomcc]. Iterator technique was provided by
  [@rodrimati1992].

### Changed

- **\[breaking\]** Objects now are closer to how they are in Objective-C.
  `&NSString` in Rust is like `NSString *` in Objective-C. The new `Arc<T>` type
  handles releasing the reference on `Drop`.

  As a result, constructors now return `Arc<Self>`.

- **\[breaking\]** Shortened lifetime of `class` on Objective-C objects from
  static to `self`.

- **\[breaking\]** Renamed `SEL` to `Sel`.

- **\[breaking\]** Renamed `get_class` to `class` for Objective-C objects.

- **\[breaking\]** Renamed `nsstring!` macro to `ns_string!`.

- **\[breaking\]** Renamed constants in `NSStringEncoding` to be simpler and use
  UPPER_SNAKE_CASE.

- **\[breaking\]** Changed `BOOL` from a type alias to a newtype.

- **\[breaking\]** Increased crate `#[cfg]` strictness from any 32/64 bit to
  only target x86 and ARM.

### Removed

- Pointer conversion methods on object types. These are now handled through the
  new `Arc<T>` object wrapper type.

## [0.2.0] - 2020-09-11

### Changed

- **\[breaking\]** Placed each library/framework module behind a feature flag.
  - `foundation` transitively enables `objc`.

### Removed

- **\[breaking\]** Removed the `link` cargo feature.

  At this point, it's unclear what benefit this feature provides. Further
  investigation will be done before this crate is 1.0 to determine if disabling
  linking is worthwhile to have. See issue [#1].

## 0.1.0 - 2020-09-10

Initial release.

[crate]:       https://crates.io/crates/fruity
[crate-badge]: https://img.shields.io/crates/v/fruity.svg
[docs]:        https://docs.rs/fruity
[docs-badge]:  https://docs.rs/fruity/badge.svg

[Keep a Changelog]:    http://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

[@thomcc]: https://github.com/thomcc
[@rodrimati1992]: https://github.com/rodrimati1992

[#3]: https://github.com/nvzqz/fruity/issues/3
[#1]: https://github.com/nvzqz/fruity/issues/1

[Unreleased]: https://github.com/nvzqz/fruity/compare/v0.2.0...HEAD
[0.2.0]:      https://github.com/nvzqz/fruity/compare/v0.1.0...v0.2.0
