# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][crate]

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Added

- The `nsstring!` macro can now take `const X: &str` as input, not just string
  literals.

- The `nsstring!` macro now allows interior null bytes, transcoding the string
  to UTF-16.

- The `nsstring!` macro now allows trailing null bytes and uses the constant
  as-is.

  This makes it possible for input data to not get emitted in the binary twice.
  The compiler currently does not coalesce string prefixes and instead emits the
  same prefix data twice.

- Correct dispatching of the appropriate `objc_msgSend` based on return type.

- Internal convenience `_msg_send!` macro for dispatching `objc_msgSend` and
  associating arguments with the appropriate selector part.

- Foundation types: `NSNumber`, `NSEdgeInsets`, `NSRange`, `NSValue`.

- `core_graphics` module for [Core Graphics](https://developer.apple.com/documentation/coregraphics)
  framework.

  - Geometry types: `CGFloat`, `CGPoint`, `CGSize`, `CGRect`, `CGRectEdge`,
    `CGVector`, and `CGAffineTransform`.

    These are aliased in `foundation` as: `NSPoint`, `NSSize`, `NSRect`, and
    `NSRectEdge`.

### Fixed

- The `nsstring!` macro now transcodes non-ASCII strings to UTF-16, instead of
  allowing UTF-8 data where only ASCII data is expected.

  Transcoding was implemented by [@thomcc]. Iterator technique was provided by
  [@rodrimati1992].

  See issue [#3].

### Changed

- **\[breaking\]** Increased crate `#[cfg]` strictness from any 32/64 bit to
  only target x86 and ARM.

- **\[breaking\]**  The `nsstring!` macro can only take ASCII strings.

  The canonical Unicode representation is UTF-16, so any non-ASCII strings must
  be transcoded to UTF-16. See issue [#3].

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
