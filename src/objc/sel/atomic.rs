use super::SEL;
use std::{
    os::raw::c_void,
    ptr::{self, NonNull},
    sync::atomic::{AtomicPtr, Ordering},
};

/// Allows for globally caching reused selectors safely.
#[repr(transparent)]
pub(crate) struct AtomicSEL(AtomicPtr<c_void>);

impl AtomicSEL {
    #[inline]
    pub const fn null() -> Self {
        Self(AtomicPtr::new(ptr::null_mut()))
    }

    /// Loads the selector in `self`, or calls `make_sel` to create a new one
    /// that is stored in `self` and returned.
    #[inline]
    pub fn load_or_store_with<F>(&self, make_sel: F) -> SEL
    where
        F: FnOnce() -> SEL,
    {
        if let Some(ptr) = NonNull::new(self.0.load(Ordering::Relaxed)) {
            unsafe { SEL::from_non_null_ptr(ptr) }
        } else {
            let selector = make_sel();
            self.0.store(selector.as_ptr() as _, Ordering::SeqCst);
            selector
        }
    }
}
