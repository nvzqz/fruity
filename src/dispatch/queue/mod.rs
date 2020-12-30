use super::{sys, DispatchObject, DispatchQos, DispatchQosClass};
use std::{
    ffi::{c_void, CStr, CString},
    fmt,
    mem::{self, ManuallyDrop, MaybeUninit},
    panic, process, ptr,
};

mod attr;
mod builder;
mod priority;

pub use attr::*;
pub use builder::*;
pub use priority::*;

subclass! {
    /// An object that manages the execution of tasks serially or concurrently on
    /// your app's main thread or on a background thread.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_queue)
    pub class DispatchQueue: DispatchObject;
}

impl fmt::Debug for DispatchQueue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let qos = self.qos();

        f.debug_struct("DispatchQueue")
            .field("label", &self.label().unwrap_or_default())
            .field("qos_class", &qos.qos_class)
            .field("relative_priority", &qos.relative_priority)
            .finish()
    }
}

/// Getting global queues.
impl DispatchQueue {
    /// The serial dispatch queue associated with the main thread of the current
    /// process.
    #[inline]
    #[doc(alias = "dispatch_get_main_queue")]
    pub fn main() -> &'static Self {
        unsafe { &sys::_dispatch_main_q }
    }

    /// Returns the global system concurrent queue with the specified
    /// quality-of-service class.
    #[inline]
    #[doc(alias = "dispatch_get_global_queue")]
    pub fn global_with_qos(qos_class: DispatchQosClass) -> &'static Self {
        unsafe { &*sys::dispatch_get_global_queue(qos_class as _, 0) }
    }

    /// Returns the global system concurrent queue with the specified priority.
    #[inline]
    #[doc(alias = "dispatch_get_global_queue")]
    pub fn global_with_priority(priority: DispatchQueuePriority) -> &'static Self {
        unsafe { &*sys::dispatch_get_global_queue(priority as _, 0) }
    }
}

/// Creating queues.
impl DispatchQueue {
    // This type deliberately does not have a `new` method or implement
    // `Default` because it's very uncommon to create an unlabeled serial queue
    // (the default).

    /// Returns a value that can be used to configure and create a queue.
    #[inline]
    pub fn builder<'a>() -> DispatchQueueBuilder<'a> {
        DispatchQueueBuilder::new()
    }
}

/// Queue properties.
impl DispatchQueue {
    /// Returns a reference to the label of the current queue.
    ///
    /// # Safety
    ///
    /// The returned label must live as long as the current queue.
    ///
    /// Consider instead using
    /// [`current_queue_label_owned`](Self::current_queue_label_owned) or
    /// [`with_current_queue_label`](Self::with_current_queue_label),
    /// depending on how long the string is needed for.
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub unsafe fn current_queue_label<'a>() -> Option<&'a CStr> {
        let label = sys::dispatch_queue_get_label(ptr::null());
        if label.is_null() {
            None
        } else {
            Some(CStr::from_ptr(label))
        }
    }

    /// Returns an owned copy of the label of the current queue.
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn current_queue_label_owned() -> Option<CString> {
        Self::with_current_queue_label(|label| Some(label?.to_owned()))
    }

    /// Returns the result of calling the function with a reference to the label
    /// of the current queue.
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn with_current_queue_label<F, T>(f: F) -> T
    where
        F: FnOnce(Option<&CStr>) -> T,
    {
        // SAFETY: The string cannot be used past the lifetime of the current
        // queue because the reference only lives as long as the scope of `f`.
        f(unsafe { Self::current_queue_label() })
    }

    /// Returns the label assigned to the queue at creation time.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1780825-label) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452939-dispatch_queue_get_label)
    #[inline]
    #[doc(alias = "dispatch_queue_get_label")]
    pub fn label(&self) -> Option<&CStr> {
        unsafe {
            let label = sys::dispatch_queue_get_label(self);
            if label.is_null() {
                None
            } else {
                Some(CStr::from_ptr(label))
            }
        }
    }

    /// Returns the quality-of-service level assigned to the queue.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1781008-qos) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452829-dispatch_queue_get_qos_class)
    #[inline]
    #[doc(alias = "dispatch_queue_get_qos_class")]
    pub fn qos(&self) -> DispatchQos {
        let mut relative_priority = 0;
        let qos_class = unsafe { sys::dispatch_queue_get_qos_class(self, &mut relative_priority) };

        DispatchQos::new(qos_class, relative_priority)
    }
}

type DispatchFn = unsafe extern "C" fn(ctx: *mut c_void);
type DispatchApplyFn = unsafe extern "C" fn(ctx: *mut c_void, iteration: usize);

/// Queue operations.
impl DispatchQueue {
    /// Submits a function to execute the specified number of times.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// The function run on a queue whose quality-of-service class is most
    /// appropriate for the current execution context. Use
    /// [`apply`](Self::apply) to perform this operation on a specific queue.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/2016088-concurrentperform) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    ///
    /// # Availability
    ///
    /// - **macOS:** 10.9
    /// - **iOS:** 7.0
    /// - **tvOS:** 9.0
    /// - **watchOS:** 2.0
    ///
    /// # Safety
    ///
    /// It is safe to panic within the `work` function. Panics will abort the
    /// process.
    ///
    /// If the overhead of the extra setup is undesirable or you would like to
    /// handle panics yourself, use
    /// [`apply_auto_no_panic`](Self::apply_auto_no_panic) or
    /// [`apply_auto_raw`](Self::apply_auto_raw) instead.
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub fn apply_auto<F>(iterations: usize, work: F)
    where
        F: Sync + Fn(usize),
    {
        // Wrap `work` to abort on panic.
        let work =
            |iteration| match panic::catch_unwind(panic::AssertUnwindSafe(|| work(iteration))) {
                Ok(()) => {}
                Err(_error) => process::abort(),
            };

        // SAFETY: Any panics within `work` are caught.
        unsafe { Self::apply_auto_no_panic(iterations, work) };
    }

    /// Submits a function to execute the specified number of times, without
    /// catching panics.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// The function run on a queue whose quality-of-service class is most
    /// appropriate for the current execution context. Use
    /// [`apply_no_panic`](Self::apply_no_panic) to perform this operation on a
    /// specific queue.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/2016088-concurrentperform) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    ///
    /// # Availability
    ///
    /// - **macOS:** 10.9
    /// - **iOS:** 7.0
    /// - **tvOS:** 9.0
    /// - **watchOS:** 2.0
    ///
    /// # Safety
    ///
    /// It is undefined behavior to panic within the `work` function because it
    /// is called from an `extern "C" fn`. Catch the panic yourself or call
    /// [`apply_auto`](Self::apply_auto) instead.
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub unsafe fn apply_auto_no_panic<F>(iterations: usize, work: F)
    where
        F: Sync + Fn(usize),
    {
        Self::_apply_auto_no_panic(None, iterations, work)
    }

    // This ensures that the wrapper function is never emitted twice for the
    // same generic function input, if used for both an auto and specific queue.
    #[inline]
    unsafe fn _apply_auto_no_panic<F>(queue: Option<&Self>, iterations: usize, work: F)
    where
        F: Sync + Fn(usize),
    {
        extern "C" fn wrapped_work<F>(ctx: *mut F, iteration: usize)
        where
            F: Sync + Fn(usize),
        {
            // SAFETY: `work` is never read mutably. A mutable pointer is
            // required by `apply_raw`.
            let work = unsafe { &*ctx };

            work(iteration);
        }

        let ctx = &work as *const F as *mut F;

        // SAFETY: Both functions have the same ABI.
        let work: extern "C" fn(_, _) = wrapped_work::<F>;
        let work: DispatchApplyFn = mem::transmute(work);

        let queue = match queue {
            Some(queue) => queue,
            None => ptr::null(),
        };

        // SAFETY: `work` is non-null, which is required by this function.
        //
        // And `work` is not an `unsafe fn`, so it needs to handle safety
        // internally.
        sys::dispatch_apply_f(iterations, queue, ctx.cast(), work);
    }

    /// Submits a C function with a context pointer to execute the specified
    /// number of times.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// The function run on a queue whose quality-of-service class is most
    /// appropriate for the current execution context. Use
    /// [`apply_raw`](Self::apply_raw) to perform this operation on a specific
    /// queue.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    ///
    /// # Availability
    ///
    /// - **macOS:** 10.9
    /// - **iOS:** 7.0
    /// - **tvOS:** 9.0
    /// - **watchOS:** 2.0
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub fn apply_auto_raw<Ctx>(
        iterations: usize,
        ctx: *mut Ctx,
        work: extern "C" fn(ctx: *mut Ctx, iteration: usize),
    ) {
        unsafe {
            // SAFETY: Both functions have the same ABI.
            let work: DispatchApplyFn = mem::transmute(work);

            // SAFETY: `work` is non-null, which is required by this function.
            //
            // And `work` is not an `unsafe fn`, so it needs to handle safety
            // internally.
            sys::dispatch_apply_f(iterations, ptr::null(), ctx.cast(), work);
        }
    }

    /// Submits a function for asynchronous execution.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/2016098-async) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452834-dispatch_async_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is safe to panic within the `work` function. Panics will abort the
    /// process.
    ///
    /// If the overhead of the extra setup is undesirable or you would like to
    /// handle panics yourself, use
    /// [`spawn_async_no_panic`](Self::spawn_async_no_panic) or
    /// [`spawn_async_raw`](Self::spawn_async_raw) instead.
    #[inline]
    #[doc(alias = "dispatch_async")]
    #[doc(alias = "dispatch_async_f")]
    pub fn spawn_async<F>(&self, work: F)
    where
        F: Send + FnOnce() + 'static,
    {
        // Wrap `work` to abort on panic.
        let work = || match panic::catch_unwind(panic::AssertUnwindSafe(work)) {
            Ok(()) => {}
            Err(_error) => process::abort(),
        };

        // SAFETY: Any panics within `work` are caught.
        unsafe { self.spawn_async_no_panic(work) };
    }

    /// Submits a function for asynchronous execution, without catching panics.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/2016098-async) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452834-dispatch_async_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is undefined behavior to panic within the `work` function because it
    /// is called from an `extern "C" fn`. Catch the panic yourself or call
    /// [`spawn_async`](Self::spawn_async) instead.
    #[inline]
    #[doc(alias = "dispatch_async")]
    #[doc(alias = "dispatch_async_f")]
    pub unsafe fn spawn_async_no_panic<F>(&self, work: F)
    where
        F: Send + FnOnce() + 'static,
    {
        extern "C" fn wrapped_work<F>(ctx: *mut F)
        where
            F: Send + FnOnce() + 'static,
        {
            // SAFETY: `work` is only used from within this function.
            let work = unsafe { Box::from_raw(ctx) };

            work();
        }

        self.spawn_async_raw(Box::into_raw(Box::new(work)), wrapped_work);
    }

    /// Submits a C function with a context pointer for asynchronous execution.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452834-dispatch_async_f?language=objc)
    #[inline]
    #[doc(alias = "dispatch_async")]
    #[doc(alias = "dispatch_async_f")]
    pub fn spawn_async_raw<Ctx>(&self, ctx: *mut Ctx, work: extern "C" fn(*mut Ctx)) {
        unsafe {
            // SAFETY: Both functions have the same ABI.
            let work: DispatchFn = mem::transmute(work);

            // SAFETY: The queue and `work` are non-null, which is required by
            // this function.
            //
            // And `work` is not an `unsafe fn`, so it needs to handle safety
            // internally.
            sys::dispatch_async_f(self, ctx.cast(), work);
        }
    }

    /// Submits a function for synchronous execution and returns the function's
    /// result after it finishes executing.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1452870-sync) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1453123-dispatch_sync_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is safe to panic within the `work` function. Panics are propagated
    /// back to the caller context.
    ///
    /// If the overhead of the extra setup is undesirable or you would like to
    /// handle panics yourself, use
    /// [`spawn_sync_no_panic`](Self::spawn_sync_no_panic) or
    /// [`spawn_sync_raw`](Self::spawn_sync_raw) instead.
    #[inline]
    #[doc(alias = "dispatch_sync")]
    #[doc(alias = "dispatch_sync_f")]
    pub fn spawn_sync<F, R>(&self, work: F) -> R
    where
        F: Send + FnOnce() -> R,
        R: Send,
    {
        // SAFETY: Any panics within `work` are caught.
        let result = unsafe {
            self.spawn_sync_no_panic(|| panic::catch_unwind(panic::AssertUnwindSafe(work)))
        };

        match result {
            Ok(result) => result,
            Err(error) => panic::resume_unwind(error),
        }
    }

    /// Submits a function for synchronous execution and returns the function's
    /// result after it finishes executing, without catching panics.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/1452870-sync) |
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1453123-dispatch_sync_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is undefined behavior to panic within the `work` function because it
    /// is called from an `extern "C" fn`. Catch the panic yourself or call
    /// [`spawn_sync`](Self::spawn_sync) instead.
    #[inline]
    #[doc(alias = "dispatch_sync")]
    #[doc(alias = "dispatch_sync_f")]
    pub unsafe fn spawn_sync_no_panic<F, R>(&self, work: F) -> R
    where
        F: Send + FnOnce() -> R,
        R: Send,
    {
        struct StackCtx<F, R> {
            work: ManuallyDrop<F>,
            result: MaybeUninit<R>,
        }

        let mut ctx = StackCtx::<F, R> {
            work: ManuallyDrop::new(work),
            result: MaybeUninit::uninit(),
        };

        extern "C" fn wrapped_work<F, R>(ctx: *mut StackCtx<F, R>)
        where
            F: Send + FnOnce() -> R,
            R: Send,
        {
            // SAFETY: `ctx` is exclusively owned by this function.
            let ctx = unsafe { &mut *ctx };

            // SAFETY: `work` is only used from within this function.
            let work = unsafe { ManuallyDrop::take(&mut ctx.work) };

            let result = work();

            // SAFETY: The pointer is valid to write to.
            //
            // TODO: Use `MaybeUninit::write` when it's stabilized.
            unsafe { ctx.result.as_mut_ptr().write(result) };
        }

        self.spawn_sync_raw(&mut ctx, wrapped_work);

        // SAFETY: This is assigned within `wrapped_work`.
        ctx.result.assume_init()
    }

    /// Submits a C function with a context pointer for synchronous execution
    /// and returns the function's result after it finishes executing.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1453123-dispatch_sync_f?language=objc)
    #[inline]
    #[doc(alias = "dispatch_sync")]
    #[doc(alias = "dispatch_sync_f")]
    pub fn spawn_sync_raw<Ctx>(&self, ctx: *mut Ctx, work: extern "C" fn(*mut Ctx)) {
        unsafe {
            // SAFETY: Both functions have the same ABI.
            let work: DispatchFn = mem::transmute(work);

            // SAFETY: The queue and `work` are non-null, which is required by
            // this function.
            //
            // And `work` is not an `unsafe fn`, so it needs to handle safety
            // internally.
            sys::dispatch_sync_f(self, ctx.cast(), work);
        }
    }

    /// Submits a function to execute the specified number of times.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is safe to panic within the `work` function. Panics will abort the
    /// process.
    ///
    /// If the overhead of the extra setup is undesirable or you would like to
    /// handle panics yourself, use [`apply_no_panic`](Self::apply_no_panic) or
    /// [`apply_raw`](Self::apply_raw) instead.
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub fn apply<F>(&self, iterations: usize, work: F)
    where
        F: Sync + Fn(usize),
    {
        // Wrap `work` to abort on panic.
        let work =
            |iteration| match panic::catch_unwind(panic::AssertUnwindSafe(|| work(iteration))) {
                Ok(()) => {}
                Err(_error) => process::abort(),
            };

        // SAFETY: Any panics within `work` are caught.
        unsafe { self.apply_no_panic(iterations, work) };
    }

    /// Submits a function to execute the specified number of times, without
    /// catching panics.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    ///
    /// # Safety
    ///
    /// It is undefined behavior to panic within the `work` function because it
    /// is called from an `extern "C" fn`. Catch the panic yourself or call
    /// [`apply`](Self::apply) instead.
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub unsafe fn apply_no_panic<F>(&self, iterations: usize, work: F)
    where
        F: Sync + Fn(usize),
    {
        Self::_apply_auto_no_panic(Some(self), iterations, work)
    }

    /// Submits a C function with a context pointer to execute the specified
    /// number of times.
    ///
    /// The current index of iteration is passed to each invocation of the
    /// function.
    ///
    /// Documentation:
    /// [Objective-C](https://developer.apple.com/documentation/dispatch/1452846-dispatch_apply_f?language=objc)
    #[inline]
    #[doc(alias = "dispatch_apply")]
    #[doc(alias = "dispatch_apply_f")]
    pub fn apply_raw<Ctx>(
        &self,
        iterations: usize,
        ctx: *mut Ctx,
        work: extern "C" fn(ctx: *mut Ctx, iteration: usize),
    ) {
        unsafe {
            // SAFETY: Both functions have the same ABI.
            let work: DispatchApplyFn = mem::transmute(work);

            // SAFETY: `work` is non-null, which is required by this function.
            //
            // And `work` is not an `unsafe fn`, so it needs to handle safety
            // internally.
            sys::dispatch_apply_f(iterations, self, ctx.cast(), work);
        }
    }
}
