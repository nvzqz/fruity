/// Creates a [`&'static str`](https://doc.rust-lang.org/std/primitive.str.html)
/// from a selector literal, that may be used as the basis of a
/// [`SEL`](objc/struct.SEL.html).
///
/// # Examples
///
/// This macro accepts anything that can go in Objective-C's `@selector`:
///
/// ```
/// use fruity::selector_str;
///
/// assert_eq!(selector_str!(init), "init\0");
///
/// assert_eq!(selector_str!(initWithArg:),   "initWithArg:\0");
/// assert_eq!(selector_str!(initWithArg : ), "initWithArg:\0");
///
/// assert_eq!(selector_str!(initWithArg:arg2:),     "initWithArg:arg2:\0");
/// assert_eq!(selector_str!(initWithArg : arg2 : ), "initWithArg:arg2:\0");
/// ```
///
/// The result of the macro can even be used in a `const`:
///
/// ```
/// # use fruity::selector_str;
/// const SEL: &str = selector_str!(initWithArg:);
/// ```
///
/// Invalid selectors will fail to compile:
///
/// ```compile_fail
/// # use fruity::selector_str;
/// let sel = selector_str!(initWithArg::);
/// ```
#[macro_export]
macro_rules! selector_str {
    ($sel:ident) => {
        std::concat!(std::stringify!($sel), "\0")
    };
    ($($sel:ident :)*) => {
        std::concat!($(std::stringify!($sel), ":",)* "\0")
    };
    ($sel:expr) => {
        std::compile_error!("Selector literal cannot be an expression")
    };
    ($($sel:tt)+) => {
        std::compile_error!(std::concat!(
            "Invalid selector literal: '",
            std::stringify!($($sel)+),
            "'",
        ))
    };
}

/// Creates a [`SEL`](objc/struct.SEL.html) from a selector literal.
///
/// # Examples
///
/// This macro accepts anything that can go in Objective-C's `@selector`:
///
/// ```
/// use fruity::selector;
///
/// let sel = selector!(init);
/// let sel = selector!(initWithArg:);
/// let sel = selector!(initWithArg:arg2:);
/// ```
///
/// Invalid selectors will fail to compile:
///
/// ```compile_fail
/// # use fruity::selector;
/// let sel = selector!(initWithArg::);
/// ```
#[macro_export]
macro_rules! selector {
    ($($sel:tt)*) => {
        {
            let ptr = $crate::selector_str!($($sel)*).as_ptr();

            // SAFETY: `selector_str!` creates a null-terminated UTF-8 string.
            #[allow(unused_unsafe)]
            let sel = unsafe { $crate::objc::SEL::register(ptr as _) };

            sel
        }
    };
}
