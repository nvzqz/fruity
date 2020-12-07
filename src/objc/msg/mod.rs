use super::{Class, ObjCObject, SEL};
use std::{ffi::c_void, mem};

mod get_fn;

macro_rules! _msg_send {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send $(::<$ret>)? (selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_with $(::<_, $ret>)? (
            selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

macro_rules! _msg_send_cached {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send $(::<$ret>)? (_cached_selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_with $(::<_, $ret>)? (
            _cached_selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

// Do not call these methods directly. Use the `_msg_send!` macro instead.
impl ObjCObject {
    #[inline]
    pub(crate) unsafe fn _msg_send<T>(&self, sel: SEL) -> T
    where
        T: 'static,
    {
        self._msg_send_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_with<A, T>(&self, sel: SEL, args: A) -> T
    where
        A: super::msg::MsgArgs,
        T: 'static,
    {
        A::msg_send(self as *const Self as *const c_void, sel, args)
    }
}

// Do not call these methods directly. Use the `_msg_send!` macro instead.
impl Class {
    #[inline]
    pub(crate) unsafe fn _msg_send<T>(&self, sel: SEL) -> T
    where
        T: 'static,
    {
        self._msg_send_with(sel, ())
    }

    #[inline]
    pub(crate) unsafe fn _msg_send_with<A, T>(&self, sel: SEL, args: A) -> T
    where
        A: super::msg::MsgArgs,
        T: 'static,
    {
        A::msg_send(self as *const Self as *const c_void, sel, args)
    }
}

// This trait is intentionally undocumented to ensure it is not publicly
// exported.
#[deny(missing_docs)]
pub trait MsgArgs: Sized {
    unsafe fn msg_send<Ret: 'static>(obj: *const c_void, sel: SEL, args: Self) -> Ret;
}

/// Implements `MsgArgs` for tuples of different sizes.
macro_rules! impl_msg_args_base {
    ($($arg:ident),*) => {
        impl<$($arg,)*> MsgArgs for ($($arg,)*) {
            #[inline]
            #[allow(non_snake_case)]
            unsafe fn msg_send<Ret: 'static>(
                obj: *const c_void,
                sel: SEL,
                ($($arg,)*): Self,
            ) -> Ret {
                let msg_send: unsafe extern "C" fn(*const c_void, SEL $(, $arg)*) -> Ret
                    = mem::transmute(get_fn::msg_send_fn::<Ret>());

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
