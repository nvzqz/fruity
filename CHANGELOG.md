# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][crate]

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added

- `ObjectType` trait to generalize over object references.

- `Unretained<T>` wrapper type that's semantically a `&T` but with the memory
  representation of `T` when `T` implements `ObjectType`.

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

- Methods for efficiently getting an `Option<&str>` from `NSString` if it's
  UTF-8.

  These do not allocate and construct a new UTF-8 C string if the string does
  not represent one.

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

- The `ns_string!` macro now transcodes non-ASCII strings to UTF-16, instead of
  allowing UTF-8 data where only ASCII data is expected.

  Transcoding was implemented by [@thomcc]. Iterator technique was provided by
  [@rodrimati1992].

  See issue [#3].

### Changed

- **\[breaking\]** Renamed `nsstring!` macro to `ns_string!`.

- **\[breaking\]** Increased crate `#[cfg]` strictness from any 32/64 bit to
  only target x86 and ARM.

- **\[breaking\]**  The `ns_string!` macro can only take ASCII strings.

  The canonical Unicode representation is UTF-16, so any non-ASCII strings must
  be transcoded to UTF-16. See issue [#3].

- **\[breaking\]** Renamed constants in `NSStringEncoding` to be simpler and use
  UPPER_SNAKE_CASE.

- `NSStringEncoding` is formatted like a Rust `enum`. This improves debugging.

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
