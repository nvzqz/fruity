// This macro is intentionally undocumented to ensure it is not publicly
// exported.

#![feature(log_syntax)]

macro_rules! subclass {
    (
        $(#[$meta:meta])+
        $vis:vis class $a:ident $(<$lifetime:lifetime>)? : $b:ty ;
    ) => {
        $(#[$meta])+
        #[repr(C)]
        $vis struct $a $(<$lifetime>)? ($b);

        impl $(<$lifetime>)? $crate::core::ObjectType for $a $(<$lifetime>)? {
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

        impl $(<$lifetime>)? std::ops::Deref for $a $(<$lifetime>)? {
            type Target = $b;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl $(<$lifetime>)? AsRef<$a $(<$lifetime>)?> for $a $(<$lifetime>)? {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl $(<$lifetime>)? AsMut<$a $(<$lifetime>)?> for $a $(<$lifetime>)? {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        impl<$($lifetime,)? T> AsRef<T> for $a $(<$lifetime>)? where $b: AsRef<T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.0.as_ref()
            }
        }

        impl<$($lifetime,)? T> AsMut<T> for $a $(<$lifetime>)? where $b: AsMut<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut T {
                self.0.as_mut()
            }
        }
    };
    (
        $(#[$meta:meta])+
        $vis:vis class $a:ident <$lifetime:lifetime, $genty:ident> : $b:ty ;
    ) => {
        $(#[$meta])+
        #[repr(C)]
        $vis struct $a <$lifetime, $genty> ($b, std::marker::PhantomData<&$lifetime $genty>);

        impl <$lifetime, $genty> $crate::core::ObjectType for $a <$lifetime, $genty> {
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

        impl <$lifetime, $genty> std::ops::Deref for $a <$lifetime, $genty> {
            type Target = $b;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl <$lifetime, $genty> AsRef<$a <$lifetime, $genty>> for $a <$lifetime, $genty> {
            #[inline]
            fn as_ref(&self) -> &Self {
                self
            }
        }

        impl <$lifetime, $genty> AsMut<$a <$lifetime, $genty>> for $a <$lifetime, $genty> {
            #[inline]
            fn as_mut(&mut self) -> &mut Self {
                self
            }
        }

        impl <$lifetime, $genty, _T> AsRef<_T> for $a <$lifetime, $genty> where $b: AsRef<_T> {
            #[inline]
            fn as_ref(&self) -> &_T {
                self.0.as_ref()
            }
        }

        impl <$lifetime, $genty, _T> AsMut<_T> for $a <$lifetime, $genty> where $b: AsMut<_T> {
            #[inline]
            fn as_mut(&mut self) -> &mut _T {
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
        $vis:vis wrapper $wrapper:ident $(<$lifetime:lifetime>)? : $target:ty ;
    ) => {
        $(#[$meta])+
        #[repr(C)]
        $vis struct $wrapper $(<$lifetime>)? (pub $target);

        impl $(<$lifetime>)? $crate::core::ObjectType for $wrapper $(<$lifetime>)? {
            #[inline]
            fn retain(obj: &Self) -> $crate::core::Arc<Self> {
                <$target>::retain(&obj.0).into()
            }

            #[inline]
            unsafe fn release(obj: std::ptr::NonNull<Self>) {
                <$target>::release(obj.cast::<$target>())
            }
        }

        impl $(<$lifetime>)? From<$crate::core::Arc<$target>> for $crate::core::Arc<$wrapper $(<$lifetime>)?> {
            #[inline]
            fn from(obj: $crate::core::Arc<$target>) -> Self {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { $crate::core::Arc::cast_unchecked(obj) }
            }
        }

        impl $(<$lifetime>)? From<$crate::core::Arc<$wrapper $(<$lifetime>)?>> for $crate::core::Arc<$target> {
            #[inline]
            fn from(obj: $crate::core::Arc<$wrapper>) -> Self {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { $crate::core::Arc::cast_unchecked(obj) }
            }
        }

        impl<$($lifetime,)? T> AsRef<T> for $wrapper $(<$lifetime>)? where $target: AsRef<T> {
            #[inline]
            fn as_ref(&self) -> &T {
                self.0.as_ref()
            }
        }

        impl<$($lifetime,)? T> AsMut<T> for $wrapper $(<$lifetime>)? where $target: AsMut<T> {
            #[inline]
            fn as_mut(&mut self) -> &mut T {
                self.0.as_mut()
            }
        }

        impl $(<$lifetime>)? AsRef<$wrapper $(<$lifetime>)?> for $target {
            #[inline]
            fn as_ref(&self) -> &$wrapper $(<$lifetime>)? {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { &*(self as *const $target as *const $wrapper) }
            }
        }

        impl $(<$lifetime>)? AsMut<$wrapper $(<$lifetime>)?> for $target {
            #[inline]
            fn as_mut(&mut self) -> &mut $wrapper $(<$lifetime>)? {
                // SAFETY: Both types have equivalent memory representations.
                unsafe { &mut *(self as *mut $target as *mut $wrapper) }
            }
        }

        // TODO: PartialOrd<$target> for $wrapper
        // TODO: PartialOrd<$wrapper> for $target

        impl $(<$lifetime>)? PartialEq<$target> for $wrapper $(<$lifetime>)? where $target: PartialEq {
            #[inline]
            fn eq(&self, other: &$target) -> bool {
                self.0 == *other
            }
        }

        impl $(<$lifetime>)? PartialEq<$wrapper $(<$lifetime>)?> for $target where $target: PartialEq {
            #[inline]
            fn eq(&self, other: &$wrapper $(<$lifetime>)?) -> bool {
                *self == other.0
            }
        }
    };
}
