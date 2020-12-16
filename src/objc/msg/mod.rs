use super::{Class, ObjCObject, Sel};
use std::{ffi::c_void, mem};

mod get_fn;

macro_rules! _msg_send_any {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send_any $(::<$ret>)? (selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_any_with $(::<_, $ret>)? (
            selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

macro_rules! _msg_send_strict {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send_strict $(::<$ret>)? (selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_strict_with $(::<_, $ret>)? (
            selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

macro_rules! _msg_send_any_cached {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send_any $(::<$ret>)? (_cached_selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_any_with $(::<_, $ret>)? (
            _cached_selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}
macro_rules! _msg_send_strict_cached {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send_any $(::<$ret>)? (_cached_selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_strict_with $(::<_, $ret>)? (
            _cached_selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

// Do not call these methods directly. Use the `_msg_send!` macro instead.
impl ObjCObject<'_> {
    #[inline]
    pub(crate) unsafe fn _msg_send_any<T>(&self, sel: Sel) -> T
    where
        T: 'static,
    {
        self._msg_send_any_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_any_with<A, T>(&self, sel: Sel, args: A) -> T
    where
        A: super::msg::MsgArgs,
        T: 'static,
    {
        A::msg_send_any(self as *const Self as *const c_void, sel, args)
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_strict<T>(&self, sel: Sel) -> T
where {
        self._msg_send_strict_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_strict_with<A, T>(&self, sel: Sel, args: A) -> T
    where
        A: super::msg::MsgArgs,
    {
        A::msg_send_strict(self as *const Self as *const c_void, sel, args)
    }
}

// Do not call these methods directly. Use the `_msg_send_any!` macro instead.
impl Class {
    #[inline]
    pub(crate) unsafe fn _msg_send_any<T>(&self, sel: Sel) -> T
    where
        T: 'static,
    {
        self._msg_send_any_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_any_with<A, T>(&self, sel: Sel, args: A) -> T
    where
        A: super::msg::MsgArgs,
        T: 'static,
    {
        A::msg_send_any(self as *const Self as *const c_void, sel, args)
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_strict<T>(&self, sel: Sel) -> T
where {
        self._msg_send_strict_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_strict_with<A, T>(&self, sel: Sel, args: A) -> T
    where
        A: super::msg::MsgArgs,
    {
        A::msg_send_strict(self as *const Self as *const c_void, sel, args)
    }
}

// This trait is intentionally undocumented to ensure it is not publicly
// exported.
#[deny(missing_docs)]
pub trait MsgArgs: Sized {
    /// Dispatches the appropriate version of `objc_msgSend` based on the return
    /// type.
    unsafe fn msg_send_any<Ret: 'static>(obj: *const c_void, sel: Sel, args: Self) -> Ret;

    /// Dispatches only to `objc_msgSend`.
    unsafe fn msg_send_strict<Ret>(obj: *const c_void, sel: Sel, args: Self) -> Ret;
}

/// Implements `MsgArgs` for tuples of different sizes.
macro_rules! impl_msg_args_base {
    ($($arg:ident),*) => {
        impl<$($arg,)*> MsgArgs for ($($arg,)*) {
            #[inline]
            #[allow(non_snake_case)]
            unsafe fn msg_send_any<Ret: 'static>(
                obj: *const c_void,
                sel: Sel,
                ($($arg,)*): Self,
            ) -> Ret {
                // TODO(#7): Use "C-unwind" ABI when stable.
                let msg_send: unsafe extern "C" fn(*const c_void, Sel $(, $arg)*) -> Ret
                    = mem::transmute(get_fn::msg_send_fn::<Ret>());

                msg_send(obj, sel $(, $arg)*)
            }

            #[inline]
            #[allow(non_snake_case)]
            unsafe fn msg_send_strict<Ret>(
                obj: *const c_void,
                sel: Sel,
                ($($arg,)*): Self,
            ) -> Ret {
                // TODO(#7): Use "C-unwind" ABI when stable.
                let msg_send: unsafe extern "C" fn() = get_fn::objc_msgSend;
                let msg_send: unsafe extern "C" fn(*const c_void, Sel $(, $arg)*) -> Ret
                    = mem::transmute(msg_send);

                msg_send(obj, sel $(, $arg)*)
            }
        }
    };
}

/// Calls `impl_msg_args_base` on N..0 args.
macro_rules! impl_msg_args {
    () => {
        impl_msg_args_base!();
    };
    ($arg_1:ident $(, $arg_n:ident)*) => {
        impl_msg_args_base!($arg_1 $(, $arg_n)*);
        impl_msg_args!($($arg_n),*);
    };
}

impl_msg_args!(A, B, C, D, E, F, G, H, I, J, K, L);
