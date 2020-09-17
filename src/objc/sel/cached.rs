#![allow(non_upper_case_globals, non_snake_case)]

// This module maps selector literals used in `_cached_selector` to globally
// cached selectors.
//
// Ideally in the future we'd be able to have static selectors like those in
// Objective-C binaries. See https://github.com/nvzqz/fruity/issues/2.

use super::atomic::AtomicSEL;

pub(crate) static class: AtomicSEL = AtomicSEL::null();
pub(crate) static copy: AtomicSEL = AtomicSEL::null();
pub(crate) static mutableCopy: AtomicSEL = AtomicSEL::null();
pub(crate) static hash: AtomicSEL = AtomicSEL::null();
pub(crate) static retainCount: AtomicSEL = AtomicSEL::null();

pub mod isEqual {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod isEqualToNumber {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod isEqualToString {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod isKindOfClass {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod isMemberOfClass {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod respondsToSelector {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}

pub mod instancesRespondToSelector {
    use super::*;
    pub(crate) static SELECTOR: AtomicSEL = AtomicSEL::null();
}
