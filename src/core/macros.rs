// This macro is intentionally undocumented to ensure it is not publicly
// exported.
macro_rules! subclass {
    (
        $(#[$meta:meta])+
        $vis:vis class $a:ident : $b:ty ;
    ) => {
        $(#[$meta])+
        #[repr(C)]
        $vis struct $a($b);

        impl $crate::core::ObjectType for $a {
            #[inline]
            fn retain(obj: &Self) -> $crate::core::Arc<Self> {
                let obj = $crate::core::Arc::retain(&obj.0);
                unsafe { $crate::core::Arc::cast_unchecked(obj) }
            }

            #[inline]
            unsafe fn release(obj: std::ptr::NonNull<Self>) {
                <$b>::release(obj.cast());
            }
        }

        impl std::ops::Deref for $a {
            type Target = $b;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl AsRef<$a> for $a {
            #[inline]
            fn as_ref(&self) -> &$a {
                self
            }
        }

        impl AsMut<$a> for $a {
            #[inline]
            fn as_mut(&mut self) -> &mut $a {
                self
            }
        }

        impl<T> AsRef<T> for $a where $b: AsRef<T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.0.as_ref()
            }
        }

        impl<T> AsMut<T> for $a where $b: AsMut<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut T {
                self.0.as_mut()
            }
        }
    };
}

// This macro is intentionally undocumented to ensure it is not publicly
// exported.
macro_rules! object_wrapper {
    (
        $(#[$meta:meta])+
        $vis:vis wrapper $wrapper:ident : $target:ty ;
    ) => {
        $(#[$meta])+
        #[repr(C)]
        $vis struct $wrapper(pub $target);

        impl $crate::core::ObjectType for $wrapper {
            #[inline]
            fn retain(obj: &Self) -> $crate::core::Arc<Self> {
                <$target>::retain(&obj.0).into()
            }

            #[inline]
            unsafe fn release(obj: std::ptr::NonNull<Self>) {
                <$target>::release(obj.cast::<$target>())
            }
        }

        impl From<$crate::core::Arc<$target>> for $crate::core::Arc<$wrapper> {
            #[inline]
            fn from(obj: $crate::core::Arc<$target>) -> Self {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { $crate::core::Arc::cast_unchecked(obj) }
            }
        }

        impl From<$crate::core::Arc<$wrapper>> for $crate::core::Arc<$target> {
            #[inline]
            fn from(obj: $crate::core::Arc<$wrapper>) -> Self {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { $crate::core::Arc::cast_unchecked(obj) }
            }
        }

        impl<T> AsRef<T> for $wrapper where $target: AsRef<T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.0.as_ref()
            }
        }

        impl<T> AsMut<T> for $wrapper where $target: AsMut<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut T {
                self.0.as_mut()
            }
        }

        impl AsRef<$wrapper> for $target {
            #[inline]
            fn as_ref(&self) -> &$wrapper {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { &*(self as *const $target as *const $wrapper) }
            }
        }

        impl AsMut<$wrapper> for $target {
            #[inline]
            fn as_mut(&mut self) -> &mut $wrapper {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { &mut *(self as *mut $target as *mut $wrapper) }
            }
        }

        // TODO: PartialOrd<$target> for $wrapper
        // TODO: PartialOrd<$wrapper> for $target

        impl PartialEq<$target> for $wrapper where $target: PartialEq {
            #[inline]
            fn eq(&self, other: &$target) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$wrapper> for $target where $target: PartialEq {
            #[inline]
            fn eq(&self, other: &$wrapper) -> bool {
                *self == other.0
            }
        }
    };
}
