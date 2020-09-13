use super::{Object, SEL};
use std::mem;

mod get_fn;

macro_rules! _msg_send {
    ($obj:expr, $sel:ident $(=> $ret:ty)?) => {
        $obj._msg_send $(::<_, $ret>)? (selector!($sel))
    };
    ($obj:expr, $($arg_name:ident : $arg:expr)+ $(=> $ret:ty)?) => {
        $obj._msg_send_with $(::<_, $ret>)? (
            selector!($($arg_name :)+),
            ($($arg,)+)
        )
    };
}

// This trait is intentionally undocumented to ensure it is not publicly
// exported.
#[deny(missing_docs)]
pub trait MsgArgs: Sized {
    unsafe fn msg_send<Ret: 'static>(obj: &Object, sel: SEL, args: Self) -> Ret;
}

/// Implements `MsgArgs` for tuples of different sizes.
macro_rules! impl_msg_args_base {
    ($($arg:ident),*) => {
        impl<$($arg,)*> MsgArgs for ($($arg,)*) {
            #[inline]
            #[allow(non_snake_case)]
            unsafe fn msg_send<Ret: 'static>(
                obj: &Object,
                sel: SEL,
                ($($arg,)*): Self,
            ) -> Ret {
                let msg_send: unsafe extern "C" fn(&Object, SEL $(, $arg)*) -> Ret
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
