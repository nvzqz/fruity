#[allow(unused)]
macro_rules! cf_bridge {
    ($a:ty, $b:ty) => {
        cf_bridge!(@ $a => $b);
        cf_bridge!(@ $b => $a);
    };
    (@ $a:ty => $b:ty) => {
        impl From<crate::core::Arc<$a>> for crate::core::Arc<$b> {
            #[inline]
            fn from(bridged: crate::core::Arc<$a>) -> Self {
                unsafe { crate::core::Arc::cast_unchecked(bridged) }
            }
        }

        impl AsRef<$b> for $a {
            #[inline]
            fn as_ref(&self) -> &$b {
                unsafe { &*(self as *const $a as *const $b) }
            }
        }

        impl AsMut<$b> for $a {
            #[inline]
            fn as_mut(&mut self) -> &mut $b {
                unsafe { &mut *(self as *mut $a as *mut $b) }
            }
        }
    };
}
