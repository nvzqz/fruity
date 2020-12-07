// This macro is intentionally undocumented to ensure it is not publicly
// exported.
macro_rules! ns_string_wrapper {
    (
        $(#[$meta:meta])+
        $vis:vis wrapper $wrapper:ident;
    ) => {
        objc_object_wrapper! {
            $(#[$meta])+
            $vis wrapper $wrapper: $crate::foundation::NSString<'static>;
        }

        // Use `NSString` formatting.

        impl std::fmt::Debug for $wrapper {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::fmt::Display for $wrapper {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
    (
        $(#[$meta:meta])+
        $vis:vis wrapper $wrapper:ident <$lifetime:lifetime>;
    ) => {
        objc_object_wrapper! {
            $(#[$meta])+
            $vis wrapper $wrapper<$lifetime>: $crate::foundation::NSString<$lifetime>;
        }

        // Use `NSString` formatting.

        impl std::fmt::Debug for $wrapper<'_> {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::fmt::Display for $wrapper<'_> {
            #[inline]
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

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
/// let hello = fruity::ns_string!("hello");
/// assert_eq!(hello.to_string(), "hello");
///
/// const WORLD: &str = "world";
/// let world = fruity::ns_string!(WORLD);
/// assert_eq!(world.to_string(), WORLD);
/// ```
///
/// The result of this macro can even be used to create `static` values:
///
/// ```
/// # use fruity::foundation::NSString;
/// static WORLD: &NSString = fruity::ns_string!("world");
///
/// assert_eq!(WORLD.to_string(), "world");
/// ```
///
/// Note that the result cannot be used in a `const` because it refers to
/// static data outside of this library.
///
/// # Unicode Strings
///
/// In Objective-C, non-ASCII strings are UTF-16. However, Rust strings are
/// UTF-8.
///
/// This macro transcodes non-ASCII strings to UTF-16:
///
/// ```
/// # use fruity::foundation::NSString;
/// static HELLO_RU: &NSString = fruity::ns_string!("Привет");
///
/// assert_eq!(HELLO_RU.to_string(), "Привет");
/// ```
///
/// Note that because this is implemented with `const` evaluation, massive
/// strings can increase compile time and even hit the `const` evaluation limit.
///
/// # Null-Terminated Strings
///
/// If the input string already ends with a 0 byte, then this macro does not
/// append one.
///
/// ```
/// let cstr = fruity::ns_string!("example\0");
/// let normal = fruity::ns_string!("example");
///
/// assert_eq!(cstr, normal);
/// ```
///
/// Interior null bytes are allowed and are not stripped:
///
/// ```
/// # // TODO: Add `to_string()` test when a Rust strings with nulls can be retrieved.
/// let example = fruity::ns_string!("exa\0mple");
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
macro_rules! ns_string {
    ($s:expr) => {{
        // Note that this always uses full paths to items from `$crate`. This
        // does not import any items because doing so could cause ambiguity if
        // the same names are exposed at the call site of this macro.
        //
        // The only names directly used are expressions, whose names shadow any
        // other names outside of this macro.

        // This is defined in CoreFoundation, but we don't emit a link attribute
        // here because it is already linked via Foundation.
        //
        // Although this is a "private" (underscored) symbol, it is directly
        // referenced in Objective-C binaries. So it's safe for us to reference.
        extern "C" {
            static __CFConstantStringClassReference: $crate::_priv::c_void;
        }

        let cfstring_ptr: *const $crate::_priv::c_void = {
            // Remove any trailing null early.
            const INPUT: &[u8] = $crate::_priv::cfstring::trim_trailing_nul($s);

            if $crate::_priv::cfstring::is_ascii(INPUT) {
                // The ASCII bytes with a trailing null byte.
                #[repr(C)]
                struct Ascii {
                    data: [u8; INPUT.len()],
                    nul: u8,
                }

                const ASCII: Ascii = Ascii {
                    data: unsafe { *$crate::_priv::std::mem::transmute::<_, &_>(INPUT.as_ptr()) },
                    nul: 0,
                };

                const ASCII_ARRAY: &[u8; INPUT.len() + 1] =
                    unsafe { $crate::_priv::std::mem::transmute(&ASCII) };

                #[link_section = "__DATA,__cfstring,regular"]
                static CFSTRING: $crate::_priv::cfstring::CFStringAscii =
                    $crate::_priv::cfstring::CFStringAscii::new(
                        unsafe { &__CFConstantStringClassReference },
                        ASCII_ARRAY.as_ptr(),
                        // The length does not include the trailing null.
                        INPUT.len(),
                    );

                CFSTRING.as_ptr()
            } else {
                // The full UTF-16 contents along with the written length.
                const UTF16_FULL: (&[u16; INPUT.len()], usize) = {
                    let mut out = [0u16; INPUT.len()];
                    let mut iter = $crate::_priv::cfstring::utf16::EncodeUtf16Iter::new(INPUT);
                    let mut written = 0;

                    while let Some((state, chars)) = iter.next() {
                        iter = state;
                        out[written] = chars.repr[0];
                        written += 1;

                        if chars.len > 1 {
                            out[written] = chars.repr[1];
                            written += 1;
                        }
                    }

                    (&{ out }, written)
                };

                // The written UTF-16 contents with a trailing null code point.
                #[repr(C)]
                struct Utf16 {
                    data: [u16; UTF16_FULL.1],
                    nul: u16,
                }

                const UTF16: Utf16 = Utf16 {
                    data: unsafe {
                        *$crate::_priv::std::mem::transmute::<_, &_>(UTF16_FULL.0.as_ptr())
                    },
                    nul: 0,
                };

                const UTF16_ARRAY: &[u16; UTF16_FULL.1 + 1] =
                    unsafe { $crate::_priv::std::mem::transmute(&UTF16) };

                #[link_section = "__DATA,__cfstring,regular"]
                static CFSTRING: $crate::_priv::cfstring::CFStringUtf16 =
                    $crate::_priv::cfstring::CFStringUtf16::new(
                        unsafe { &__CFConstantStringClassReference },
                        UTF16_ARRAY.as_ptr(),
                        // The length does not include the trailing null.
                        UTF16_FULL.1,
                    );

                CFSTRING.as_ptr()
            }
        };

        union Cast<T: 'static> {
            pointer: *const T,
            reference: &'static T,
        }

        #[allow(unused_unsafe)]
        let ns_string: &$crate::foundation::NSString = unsafe {
            Cast {
                pointer: cfstring_ptr.cast(),
            }
            .reference
        };

        ns_string
    }};
}

#[cfg(test)]
mod tests {
    use super::super::NSString;

    #[test]
    fn ns_string() {
        macro_rules! test {
            ($($s:expr,)+) => {$({
                static STRING: &NSString = ns_string!($s);
                assert_eq!(STRING.to_string(), $s);
            })+};
        }

        test! {
            "asdf",
            "🦀",
            "🏳️‍🌈",
            "𝄞music",
            "abcd【e】fg",
            "abcd⒠fg",
            "ääääh",
            "lööps, bröther?",
            "\u{fffd} \u{fffd} \u{fffd}",
            "讓每個人都能打造出。",
        }
    }
}
