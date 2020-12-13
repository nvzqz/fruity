/// Returns a [`Class`](crate::objc::Class) reference with the given name.
///
/// The symbol of the class is referenced by adding a `OBJC_CLASS_$_` prefix to
/// `$name`.
/// This will not demangle This does not currently support mangled symbol names.
///
/// # Feature Flag
///
/// This macro is defined in [`objc`](objc/index.html), which requires the
/// **`objc`** [feature flag](index.html#feature-flags).
///
/// # Examples
///
/// The class name can be passed as an identifier or string literal:
///
/// ```rust
/// use fruity::objc::Class;
///
/// let class_a: &Class = fruity::objc_class!(NSObject);
/// let class_b: &Class = fruity::objc_class!("NSObject");
///
/// assert_eq!(class_a, class_b);
/// ```
///
/// If the symbol cannot be be found at link time, an error will occur:
///
/// ```compile_fail
/// # use fruity::objc::Class;
/// let class: &Class = fruity::objc_class!(NSFruity);
/// # println!("{:?}", class.name()); // must be used
/// ```
#[macro_export]
macro_rules! objc_class {
    ($name:ident) => {
        $crate::objc_class!(stringify!($name))
    };
    ($name:expr) => {
        $crate::_objc_class!(@ concat!("OBJC_CLASS_$_", $name))
    };
}

// A separate macro is used so that only the public argument patterns are showed
// in docs.
#[doc(hidden)]
#[macro_export]
macro_rules! _objc_class {
    (@ $link_name:expr) => {{
        extern "C" {
            #[link_name = $link_name]
            static CLASS: $crate::objc::Class;
        }

        #[allow(unused_unsafe)]
        let class = unsafe { &CLASS };

        class
    }};
}
