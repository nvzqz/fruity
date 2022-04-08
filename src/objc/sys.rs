//! Raw unsafe C functions exposed by libobjc.

use super::*;
use std::os::raw::{c_char, c_uint, c_void};

#[allow(missing_docs)]
extern "C" {
    pub fn class_getClassMethod(cls: *const Class, name: Sel) -> *const Method;
    pub fn class_getInstanceVariable(cls: *const Class, name: *const c_char) -> *const Ivar;
    pub fn class_getInstanceMethod(cls: *const Class, name: Sel) -> *const Method;
    pub fn class_copyIvarList(cls: *const Class, out_count: *mut c_uint) -> *mut *const Ivar;
    pub fn class_copyMethodList(cls: *const Class, out_count: *mut c_uint) -> *mut *const Method;

    pub fn object_getClass(obj: *const ObjCObject) -> *const Class;

    pub fn object_setInstanceVariable(
        obj: *const ObjCObject,
        name: *const c_char,
        value: *mut c_void,
    ) -> *const Ivar;

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

    pub fn ivar_getName(ivar: *const Ivar) -> *const c_char;
    pub fn ivar_getOffset(ivar: *const Ivar) -> isize;
    pub fn ivar_getTypeEncoding(ivar: *const Ivar) -> *const c_char;

    pub fn NSGetSizeAndAlignment(ty: *const c_char, size: *mut NSUInteger, align: *mut NSUInteger);
}
