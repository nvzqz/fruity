use super::{sys, CFType, CFTypeID};
use std::{cmp::Ordering, fmt};

subclass! {
    /// A boolean object.
    ///
    /// Documentation:
    /// [Swift](https://developer.apple.com/documentation/corefoundation/cfboolean?language=swift) |
    /// [Objective-C](https://developer.apple.com/documentation/corefoundation/cfboolean?language=objc)
    #[derive(PartialEq, Hash)]
    pub class CFBoolean: CFType<'static>;
}

// `CFBoolean` is bridged to `NSNumber` but not the other way around.
#[cfg(feature = "foundation")]
mod foundation_casts {
    use super::*;
    use crate::{core::Arc, foundation::NSNumber};

    impl From<Arc<CFBoolean>> for Arc<NSNumber> {
        #[inline]
        fn from(boolean: Arc<CFBoolean>) -> Self {
            unsafe { Arc::cast_unchecked(boolean) }
        }
    }

    impl AsRef<NSNumber> for CFBoolean {
        #[inline]
        fn as_ref(&self) -> &NSNumber {
            unsafe { &*(self as *const Self as *const NSNumber) }
        }
    }
}

impl fmt::Debug for CFBoolean {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_bool().fmt(f)
    }
}

impl fmt::Display for CFBoolean {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_bool().fmt(f)
    }
}

impl Eq for CFBoolean {}

impl PartialOrd for CFBoolean {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFBoolean {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bool().cmp(&other.as_bool())
    }
}

impl Default for &CFBoolean {
    #[inline]
    fn default() -> Self {
        CFBoolean::false_value()
    }
}

impl CFBoolean {
    /// Returns a reference to a `false` value.
    ///
    /// This internally references
    /// [`kCFBooleanFalse`](https://developer.apple.com/documentation/corefoundation/kCFBooleanFalse).
    #[inline]
    #[doc(alias = "kCFBooleanFalse")]
    pub fn false_value() -> &'static Self {
        extern "C" {
            static kCFBooleanFalse: &'static CFBoolean;
        }
        unsafe { kCFBooleanFalse }
    }

    /// Returns a reference to a `true` value.
    ///
    /// This internally references
    /// [`kCFBooleanTrue`](https://developer.apple.com/documentation/corefoundation/kCFBooleanTrue).
    #[inline]
    #[doc(alias = "kCFBooleanTrue")]
    pub fn true_value() -> &'static Self {
        extern "C" {
            static kCFBooleanTrue: &'static CFBoolean;
        }
        unsafe { kCFBooleanTrue }
    }
}

impl CFBoolean {
    /// Returns the type identifier for `CFBoolean`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1541762-cfbooleangettypeid?language=objc).
    #[inline]
    #[doc(alias = "CFBooleanGetTypeID")]
    pub fn type_id() -> CFTypeID {
        unsafe { sys::CFBooleanGetTypeID() }
    }

    /// Returns `kCFBooleanFalse` if `value` is `false`, or `kCFBooleanTrue`
    /// if `value` is `true`.
    #[inline]
    pub fn new(value: bool) -> &'static CFBoolean {
        if value {
            Self::true_value()
        } else {
            Self::false_value()
        }
    }

    /// Returns that native `bool` value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/corefoundation/1541447-cfbooleangetvalue?language=objc).
    #[inline]
    #[doc(alias = "CFBooleanGetValue")]
    pub fn as_bool(&self) -> bool {
        unsafe { sys::CFBooleanGetValue(self) != 0 }
    }
}
