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
/// This macro takes a either a `"string"` literal or `const` string slice as
/// the argument:
///
/// ```
/// let hello = fruity::nsstring!("hello");
/// assert_eq!(hello.to_string(), "hello");
///
/// const WORLD: &str = "world";
/// let world = fruity::nsstring!(WORLD);
/// assert_eq!(world.to_string(), WORLD);
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
/// # Unicode Strings
///
/// In Objective-C, non-ASCII strings are represented as UTF-16. However, `&str`
/// is encoded as UTF-8.
///
/// Because of this, using a non-ASCII string is a compile error:
///
/// ```compile_fail
/// let non_ascii = fruity::nsstring!("straße");
/// ```
///
/// This is tracked in [issue #3](https://github.com/nvzqz/fruity/issues/3).
///
/// # Null-Terminated Strings
///
/// If the input string already ends with a 0 byte, then this macro does not
/// append one.
///
/// ```
/// let cstr = fruity::nsstring!("example\0");
/// let normal = fruity::nsstring!("example");
///
/// assert_eq!(cstr, normal);
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

        // TODO(#3): Convert `INPUT` to UTF-16 if it contains non-ASCII bytes.
        const INPUT: &str = $s;

        // Assert that `INPUT` is ASCII. Unicode strings are not currently
        // supported.
        const _: [(); 1] = [(); $crate::_priv::str::is_cf_ascii(INPUT) as usize];

        // This is defined in CoreFoundation, but we don't emit a link attribute
        // here because it is already linked via Foundation.
        //
        // Although this is a "private" (underscored) symbol, it is directly
        // referenced in Objective-C binaries. So it's safe for us to reference.
        extern "C" {
            static __CFConstantStringClassReference: $crate::_priv::std::ffi::c_void;
        }

        // This is composed of:
        // - 07: The `CFTypeID` for `CFString`
        // - C8: Flags for a constant ASCII string with a trailing null byte.
        const FLAGS: usize = 0x07c8;

        // If input already ends with a 0 byte, then we don't need to add it.
        let data = if $crate::_priv::str::is_nul_terminated(INPUT) {
            #[link_section = "__DATA,__cfstring,regular"]
            static DATA: $crate::_priv::__CFString = $crate::_priv::__CFString {
                isa: unsafe { &__CFConstantStringClassReference },
                flags: FLAGS,
                data: INPUT.as_ptr(),
                // The length does not include the null byte.
                len: INPUT.len() - 1,
            };

            &DATA
        } else {
            // Create a new constant with 0 appended to INPUT.

            #[repr(C)]
            struct Bytes {
                input: [u8; INPUT.len()],
                nul: u8,
            }

            const BYTES: Bytes = Bytes {
                input: unsafe { *$crate::_priv::std::mem::transmute::<_, &_>(INPUT.as_ptr()) },
                nul: 0,
            };

            const INPUT_WITH_NUL: &[u8; INPUT.len() + 1] =
                unsafe { $crate::_priv::std::mem::transmute(&BYTES) };

            #[link_section = "__DATA,__cfstring,regular"]
            static DATA: $crate::_priv::__CFString = $crate::_priv::__CFString {
                isa: unsafe { &__CFConstantStringClassReference },
                flags: FLAGS,
                data: INPUT_WITH_NUL.as_ptr(),
                // The length does not include the null byte.
                len: INPUT.len(),
            };

            &DATA
        };

        #[allow(unused_unsafe)]
        let nsstring =
            unsafe { $crate::foundation::NSString::from_ptr(data as *const _ as *mut _) };

        nsstring
    }};
}
