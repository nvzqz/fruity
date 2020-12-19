//! Raw unsafe C functions exposed by libobjc.

use super::*;
use std::os::raw::{c_char, c_uint};

#[allow(missing_docs)]
extern "C" {
    pub fn class_getClassMethod(cls: *const Class, name: Sel) -> *const Method;
    pub fn class_getInstanceMethod(cls: *const Class, name: Sel) -> *const Method;
    pub fn class_copyMethodList(cls: *const Class, out_count: *mut c_uint) -> *mut *const Method;

    pub fn method_getNumberOfArguments(m: *const Method) -> u32;

    pub fn method_getName(m: *const Method) -> Sel;

    pub fn method_getDescription<'a>(m: *const Method) -> *mut MethodDescription<'a>;

    pub fn method_getImplementation(m: *const Method) -> Imp;
    pub fn method_setImplementation(m: *const Method, imp: Imp) -> Imp;
    pub fn method_exchangeImplementations(m1: *const Method, m2: *const Method);

    pub fn method_getTypeEncoding(m: *const Method) -> *const c_char;

    // TODO: Create safe wrapper when `Malloced<CStr>` is possible.
    pub fn method_copyReturnType(m: *const Method) -> *mut c_char;
    pub fn method_getReturnType(m: *const Method, dst: *mut c_char, dst_len: usize);

    // TODO: Create safe wrapper when `Malloced<CStr>` is possible.
    pub fn method_copyArgumentType(m: *const Method, index: c_uint) -> *mut c_char;
    pub fn method_getArgumentType(
        m: *const Method,
        index: c_uint,
        dst: *mut c_char,
        dst_len: usize,
    );

    pub fn method_invoke();
    #[cfg(not(target_arch = "aarch64"))]
    pub fn method_invoke_stret();
}
