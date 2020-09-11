# Changelog [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][crate]

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog] and this project adheres to
[Semantic Versioning].

## [Unreleased]

### Changed

- **\[breaking\]** Increased crate `#[cfg]` strictness from any 32/64 bit to
  only target x86 and ARM.

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

[#1]: https://github.com/nvzqz/fruity/issues/1

[Unreleased]: https://github.com/nvzqz/fruity/compare/v0.2.0...HEAD
[0.2.0]:      https://github.com/nvzqz/fruity/compare/v0.1.0...v0.2.0
