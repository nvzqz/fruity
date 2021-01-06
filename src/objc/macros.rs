macro_rules! objc_class_type {
    ($obj:ident <$lifetime:lifetime, $genty:ident>) => {
        objc_class_type!($obj <$lifetime, $genty>, stringify!($obj));
    };
    ($obj:ident <$lifetime:lifetime, $genty:ident>, $class:expr) => {
        objc_class_type!(@processed_generic
                         $obj <$lifetime, $genty>,
                         $class,
                         concat!("OBJC_CLASS_$_", $class)
        );
    };
    (@processed_generic $obj:ident <$lifetime:lifetime, $genty:ident>, $class:expr, $class_symbol:expr) => {
        impl<$lifetime, $genty> $crate::objc::ClassType<$lifetime> for $obj<$lifetime, $genty>
        where $genty: $crate::objc::ObjectType<$lifetime> {
            #[inline]
            fn class() -> &'static $crate::objc::Class {
                $crate::_objc_class!(@ $class_symbol)
            }
        }
    };
    ($obj:ident $(<$lifetime:lifetime>)?) => {
        objc_class_type!($obj $(<$lifetime>)?, stringify!($obj));
    };
    ($obj:ident $(<$lifetime:lifetime>)?, $class:expr) => {
        objc_class_type!(@processed
            $obj $(<$lifetime>)?,
            $class,
            concat!("OBJC_CLASS_$_", $class)
        );
    };
    (@processed $obj:ident <$lifetime:lifetime>, $class:expr, $class_symbol:expr) => {
        impl<$lifetime> $crate::objc::ClassType<$lifetime> for $obj<$lifetime> {
            #[inline]
            fn class() -> &'static $crate::objc::Class {
                $crate::_objc_class!(@ $class_symbol)
            }
        }
    };
    (@processed $obj:ident, $class:expr, $class_symbol:expr) => {
        impl $crate::objc::ClassType<'static> for $obj {
            #[inline]
            fn class() -> &'static $crate::objc::Class {
                extern "C" {
                    #[link_name = $class_symbol]
                    static CLASS: $crate::objc::Class;
                }
                unsafe { &CLASS }
            }
        }
    };
}

macro_rules! objc_subclass {
    (
        $(#[$meta:meta])+
        $vis:vis class $a:ident : $b:ty ;
    ) => {
        subclass! {
            $(#[$meta])+
            $vis class $a : $b ;
        }

        impl $crate::objc::ObjectType<'static> for $a {}

        objc_class_type!($a);
    };
    (
        $(#[$meta:meta])+
        $vis:vis class $a:ident <$lifetime:lifetime> : $b:ty ;
    ) => {
        subclass! {
            $(#[$meta])+
            $vis class $a <$lifetime> : $b ;
        }

        impl<$lifetime> $crate::objc::ObjectType<$lifetime> for $a<$lifetime> {}

        objc_class_type!($a <$lifetime>);
    };
    (
        $(#[$meta:meta])+
            $vis:vis class $a:ident <$lifetime:lifetime, $genty:ident> : $b:ty ;
    ) => {
        subclass! {
            $(#[$meta])+
                $vis class $a <$lifetime, $genty> : $b ;
        }

        impl<$lifetime, $genty> $crate::objc::ObjectType<$lifetime> for $a<$lifetime, $genty>
        where $genty: $crate::objc::ObjectType<$lifetime> {}

        objc_class_type!($a <$lifetime, $genty>);
    };
}

// This macro is intentionally undocumented to ensure it is not publicly
// exported.
macro_rules! objc_object_wrapper {
    (
        $(#[$meta:meta])+
        $vis:vis wrapper $wrapper:ident : $target:ty ;
    ) => {
        object_wrapper! {
            $(#[$meta])+
            $vis wrapper $wrapper: $target;
        }

        impl $crate::objc::ObjectType<'static> for $wrapper {}
    };
    (
        $(#[$meta:meta])+
        $vis:vis wrapper $wrapper:ident <$lifetime:lifetime> : $target:ty ;
    ) => {
        object_wrapper! {
            $(#[$meta])+
            $vis wrapper $wrapper <$lifetime>: $target;
        }

        impl<$lifetime> $crate::objc::ObjectType<$lifetime> for $wrapper<$lifetime> {}
    };
}
