# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][crate]

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added

- `core` module for core types and traits are not specific to any wrapped
  library.

  - `ObjectType` trait to generalize retain/release.

  - `Arc` type to handle automatic reference counting. Built on top of the
    methods in `ObjectType`.

    - Implements `Default`:

      - `Arc<NSObject>` from `[[NSObject alloc] init]`.

      - `Arc<NSMutableString>` from `[[NSMutableString alloc] init]`.

      - `Arc<NSNull>` from retaining `kCFNull`.

- Internal macros to simplify creating class types and wrappers:

  - `subclass!` performs all the wrapping and trait impls (e.g. `obj::Object`,
    `Deref`) to make a new subclass easy to declare. This macro is
    runtime-agnostic.

  - `class_wrapper!` for thin wrappers over classes. Unlike `subclass!`, this is
    not meant for creating a new class type.

  - `objc_class_type!` implements `objc::ClassType` using the given class name.

  - `objc_subclass!` calls `subclass!` and `objc_class_type!`.

  - `objc_class_wrapper!` calls `class_wrapper!` and implements the
    `objc::ObjectType` trait.

  - `ns_string_wrapper!` calls `objc_class_wrapper!` with `foundation::NSString`
    and implements the `Debug` and `Display` traits.

- `objc::ClassType` trait for types whose classes are statically available.

- `BOOL::NO` and `BOOL::YES` associated constants as alternatives to the
  constants in in the `objc` module. These should be preferred.

- `DispatchQueue::current_queue_label_owned` as safe owned alternative to
  `current_queue_label`.

- `DispatchQueue::with_current_queue_label` as safe scoped alternative to
  `current_queue_label`.

- The `ns_string!` macro can now take `const X: &str` as input, not just string
  literals.

- The `ns_string!` macro now allows interior null bytes, transcoding the string
  to UTF-16.

- The `ns_string!` macro now allows trailing null bytes and uses the constant
  as-is.

  This makes it possible for input data to not get emitted in the binary twice.
  The compiler currently does not coalesce string prefixes and instead emits the
  same prefix data twice.

- Equivalent to `@autoreleasepool` that drains on panic.

- Correct dispatching of the appropriate `objc_msgSend` based on return type.

- Internal convenience `_msg_send!` macro for dispatching `objc_msgSend` and
  associating arguments with the appropriate selector part.

  - Also `_msg_send_cached!` that internally caches the selector in a global
    atomic pointer.

- Foundation types: `NSNumber`, `NSEdgeInsets`, `NSRange`, `NSValue`, `NSNull`,
  `NSException`, `NSExceptionName`, `NSError`, `NSErrorDomain`, `NSErrorUserInfoKey`, `NSErrorRecoveryAttempting`.

- Core Foundation types: `CFType`, `CFTypeRef`, `CFTypeID`, `CFOptionFlags`,
  `CFIndex`, `CFHashCode`, `CFComparisonResult`.

- Foundation constants: `NSNotFound`.

- Core Foundation constants: `kCFNotFound`.

- Methods for `NSStringEncoding`: `name`.

- Methods for `NSString`:

  - `length`

  - Efficiently getting an `Option<&str>` if it's UTF-8 or an `Option<&[u16]>` if it's UTF-16.

    Unlike `to_str` and friends, these do not allocate and transcode to a new
    string if the original string does not represent the encoding.

  - Efficiently comparing against `&str`.

- Methods for getting all available `NSStringEncoding`s.

- Foundation error codes.

- `core_graphics` module for [Core Graphics](https://developer.apple.com/documentation/coregraphics)
  framework.

  - Geometry types: `CGFloat`, `CGPoint`, `CGSize`, `CGRect`, `CGRectEdge`,
    `CGVector`, and `CGAffineTransform`.

    These are aliased in `foundation` as: `NSPoint`, `NSSize`, `NSRect`, and
    `NSRectEdge`.

- `app_kit` module for [AppKit](https://developer.apple.com/documentation/appkit)
  framework.

  - `NSAppKitVersion` type and version constants through 10.15.

- `dispatch` module for [Dispatch](https://developer.apple.com/documentation/dispatch)
  library.

  - Types: `DispatchObject`, `DispatchQueue`, `DispatchQueueBuilder`,
    `DispatchQueuePriority`, `DispatchQueueAttributes`, `DispatchTime`,
    `DispatchQos`, `DispatchQosClass`, `DispatchAutoreleaseFrequency`.

- Pointer methods for `SEL`.

- Implemented `PartialEq` for `NSObject`.

- Implemented `Default` for `NSString` and `NSMutableString`.

### Fixed

- **\[breaking\]** The safety of `DispatchQueue::current_queue_label_owned` by
  marking it as `unsafe`. It is unspecified whether the label may outlive the
  current queue.

- The signed-ness of `BOOL` on platforms where it is a `signed char`.

- **\[breaking\]** The memory representation of `BOOL` on platforms where it is
  a C/C++ `Bool`.

- The `ns_string!` macro now transcodes non-ASCII strings to UTF-16, instead of
  allowing UTF-8 data where only ASCII data is expected.

  Transcoding was implemented by [@thomcc]. Iterator technique was provided by
  [@rodrimati1992].

  See issue [#3].

### Changed

- **\[breaking\]** Objects now are closer to how they are in Objective-C.
  `&NSString` in Rust is like `NSString *` in Objective-C. The new `Arc<T>` type
  handles releasing the reference on `Drop`.

  As a result, constructors now

- **\[breaking\]** Renamed `get_class` to `class` for Objective-C objects.

- **\[breaking\]** Shortened lifetime of `class` on Objective-C objects from
  static to `self`.

- **\[breaking\]** Changed `BOOL` from a type alias to a newtype.

- **\[breaking\]** Renamed `nsstring!` macro to `ns_string!`.

- **\[breaking\]** Increased crate `#[cfg]` strictness from any 32/64 bit to
  only target x86 and ARM.

- **\[breaking\]**  The `ns_string!` macro can only take ASCII strings.

  The canonical Unicode representation is UTF-16, so any non-ASCII strings must
  be transcoded to UTF-16. See issue [#3].

- **\[breaking\]** Renamed constants in `NSStringEncoding` to be simpler and use
  UPPER_SNAKE_CASE.

- `NSStringEncoding` is formatted like a Rust `enum`. This improves debugging.

### Removed

- Pointer conversion methods on object types. These are now handled through
  `Arc`.

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
