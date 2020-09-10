/// Creates an [`NSString`](foundation/struct.NSString.html) from a static
/// string.
///
/// # Feature Flag
///
/// This macro is defined in [`foundation`](foundation/index.html),
/// which requires the **`foundation`**
/// [feature flag](index.html#feature-flags).
///
/// # Examples
///
/// This macro takes a `"string"` literal as the argument:
///
/// ```
/// let hello = fruity::nsstring!("hello");
///
/// assert_eq!(hello.to_string(), "hello");
/// ```
///
/// The result of this macro can even be used to create `static` values:
///
/// ```
/// # use fruity::foundation::NSString;
/// static WORLD: NSString = fruity::nsstring!("world");
///
/// assert_eq!(WORLD.to_string(), "world");
/// ```
///
/// Note that the result cannot be used in a `const` because it refers to
/// static data outside of this library.
///
/// # Validity Checking
///
/// Because the string data must be a valid C string, having null bytes inside
/// it causes a compile error:
///
/// ```compile_fail
/// let s = fruity::nsstring!("ab\0cd");
/// ```
///
/// # Runtime Cost
///
/// None.
///
/// The result is equivalent to `@"string"` syntax in Objective-C.
///
/// Because of that, this should be preferred over
/// [`NSString::from_str`](foundation/struct.NSString.html#method.from_str)
/// where possible.
///
/// # Compile-time Cost
///
/// Minimal.
///
/// This is implemented entirely with `const` evaluation. It is not a procedural
/// macro that requires dependencies for parsing.
#[macro_export]
macro_rules! nsstring {
    ($s:expr) => {{
        // Note that this always uses full paths to items from `$crate`. This
        // does not import any items because doing so could cause ambiguity if
        // the same names are exposed at the call site of this macro.
        //
        // The only names directly used are expressions, whose names shadow any
        // other names outside of this macro.

        // This can be in the `__cstring` link section, but it doesn't need to.
        const STR: &str = $crate::_priv::std::concat!($s, "\0");

        // Assert that `STR` is a valid C string:
        // 1. It ends with null.
        // 2. It has no interior nulls.
        const _: [(); 1] = [(); $crate::_priv::is_cstr(STR) as usize];

        #[link_section = "__DATA,__cfstring,regular"]
        static DATA: $crate::_priv::__CFString = $crate::_priv::__CFString {
            isa: unsafe {
                extern "C" {
                    static __CFConstantStringClassReference: $crate::_priv::std::ffi::c_void;
                }
                &__CFConstantStringClassReference
            },

            // This is a magic constant I came across when inspecting
            // Objective-C binaries. This is probably the CFTypeID?
            flags: 0x07c8,

            data: STR.as_ptr(),

            // The length does not include the null byte.
            len: STR.len() - 1,
        };

        #[allow(unused_unsafe)]
        let nsstring =
            unsafe { $crate::foundation::NSString::from_ptr(&DATA as *const _ as *mut _) };

        nsstring
    }};
}
