macro_rules! def {
    ($(#[$doc:meta])+) => {
        $(#[$doc])+
        #[cfg(target_pointer_width = "64")]
        pub type CGFloat = f64;

        $(#[$doc])+
        #[cfg(target_pointer_width = "32")]
        pub type CGFloat = f32;
    };
}

def! {
    /// The architecture-dependent floating-point type.
    ///
    /// This is [`f64`](https://doc.rust-lang.org/std/primitive.f64.html) on
    /// 64-bit and [`f32`](https://doc.rust-lang.org/std/primitive.f32.html) on
    /// 32-bit.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coregraphics/cgfloat).
}
