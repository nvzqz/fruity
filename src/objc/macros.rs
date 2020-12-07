macro_rules! objc_class_type {
    ($obj:ident) => {
        objc_class_type!($obj, stringify!($obj));
    };
    ($obj:ty, $class:expr) => {
        objc_class_type!(@processed $obj, $class, concat!("OBJC_CLASS_$_", $class));
    };
    (@processed $obj:ty, $class:expr, $class_symbol:expr) => {
        impl $crate::objc::ClassType for $obj {
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

        impl $crate::objc::ObjectType for $a {}

        objc_class_type!($a);
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

        impl $crate::objc::ObjectType for $wrapper {}
    };
}
