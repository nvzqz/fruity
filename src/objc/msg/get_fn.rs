#[allow(unused)]
use std::{any::TypeId, mem};

// TODO(#7): Use "C-unwind" ABI when stable.
#[allow(unused)]
extern "C" {
    fn objc_msgSend();
    fn objc_msgSend_fpret();
    fn objc_msgSend_stret();
}

#[inline]
pub fn msg_send_fn<Ret: 'static>() -> unsafe extern "C" fn() {
    #[cfg(target_arch = "x86")]
    {
        let ret_type = TypeId::of::<Ret>();

        if ret_type == TypeId::of::<f32>() || ret_type == TypeId::of::<f64>() {
            objc_msgSend_fpret
        } else {
            match mem::size_of::<Ret>() {
                0 | 1 | 2 | 4 | 8 => objc_msgSend,
                _ => objc_msgSend_stret,
            }
        }
    }

    #[cfg(target_arch = "x86_64")]
    {
        if mem::size_of::<Ret>() <= 16 {
            objc_msgSend
        } else {
            objc_msgSend_stret
        }
    }

    #[cfg(target_arch = "arm")]
    {
        let ret_type = TypeId::of::<Ret>();

        if mem::size_of::<Ret>() <= 4
            || ret_type == TypeId::of::<i64>()
            || ret_type == TypeId::of::<u64>()
            || ret_type == TypeId::of::<f64>()
        {
            objc_msgSend
        } else {
            objc_msgSend_stret
        }
    }

    #[cfg(target_arch = "aarch64")]
    {
        objc_msgSend
    }
}
