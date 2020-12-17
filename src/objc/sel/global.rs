use super::Sel;
use std::cell::Cell;

/// Stores a static selector that is registered at program start by the
/// Objective-C runtime.
///
/// This always stores a valid `Sel` instance.
#[repr(transparent)]
pub(crate) struct GlobalSel {
    /// The selector stored by this instance.
    ///
    /// `Cell` is used here for three reasons:
    ///
    /// 1. We want to inform the compiler that this value is internally mutable
    ///    and thus should always be initially read from memory.
    ///
    ///    This is also achievable with `AtomicPtr`, however we don't use it
    ///    because of the second reason:
    ///
    /// 2. We want the compiler to assume that multiple reads within the same
    ///    scope will produce the same value. This optimization helps in
    ///    situations like calling `objc_msgSend` in a loop, where the selector
    ///    read can be hoisted out of the loop.
    ///
    ///    This does not currently happen for `AtomicPtr` with `Relaxed` loads,
    ///    even though it should be possible.
    ///
    /// 3. "Cell Sel" is funny to say.
    inner: Cell<Sel>,
}

// SAFETY: The value is re-assigned before it is read in Rust, and it is never
// written to by Rust code.
unsafe impl Send for GlobalSel {}
unsafe impl Sync for GlobalSel {}

impl GlobalSel {
    #[inline]
    pub const fn new(sel: Sel) -> Self {
        Self {
            inner: Cell::new(sel),
        }
    }

    #[inline]
    pub fn get(&self) -> Sel {
        self.inner.get()
    }
}

// TODO: Make private symbols work across compilation units.
// See https://github.com/rust-lang/rust/issues/53929.
//
// TODO: Once the above TODO is done, make all selectors use `GlobalSel`.
#[used]
#[link_section = "__DATA,__objc_selrefs,literal_pointers,no_dead_strip"]
#[export_name = "\x01L_OBJC_SELECTOR_REFERENCES_.fruity(class)"]
#[allow(non_upper_case_globals)]
pub(crate) static class: GlobalSel = {
    #[link_section = "__TEXT,__objc_methname,cstring_literals"]
    #[export_name = "\x01L_OBJC_METH_VAR_NAME_.fruity(class)"]
    static SELECTOR: [u8; 6] = *b"class\0";

    GlobalSel::new(unsafe { Sel::from_ptr(SELECTOR.as_ptr().cast()) })
};
