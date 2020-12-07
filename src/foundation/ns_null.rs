use crate::objc::NSObject;

objc_subclass! {
    /// A singleton object used to represent null values in collection objects that
    /// don’t allow `nil` values.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnull).
    pub class NSNull: NSObject<'static>;
}

impl Default for &NSNull {
    #[inline]
    fn default() -> Self {
        NSNull::null()
    }
}

impl NSNull {
    /// Returns the singleton instance.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsnull).
    #[inline]
    #[doc(alias = "kCFNull")]
    pub fn null() -> &'static Self {
        extern "C" {
            // `NSNull` is toll-free bridged with `CFNullRef` whose only
            // instance is this.
            static kCFNull: &'static NSNull;
        }
        unsafe { kCFNull }
    }
}
