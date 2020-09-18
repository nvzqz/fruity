use super::{NSEdgeInsets, NSPoint, NSRange, NSRect, NSSize};
use crate::objc::{Class, NSObject, NSUInteger, Object, ObjectType};
use std::{
    ffi::CStr,
    fmt, mem,
    ops::Deref,
    os::raw::{c_char, c_void},
    ptr::NonNull,
};

// TODO: Implement methods defined in other frameworks.

/// A simple container for a single C or Objective-C data item.
///
/// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue).
#[repr(transparent)]
#[derive(Clone)]
pub struct NSValue(NSObject);

unsafe impl ObjectType for NSValue {}

impl From<NSValue> for NSObject {
    #[inline]
    fn from(obj: NSValue) -> Self {
        obj.0
    }
}

impl Deref for NSValue {
    type Target = NSObject;

    #[inline]
    fn deref(&self) -> &NSObject {
        &self.0
    }
}

impl fmt::Pointer for NSValue {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_ptr().fmt(f)
    }
}

impl NSValue {
    /// Returns the `NSValue` class.
    #[inline]
    pub fn class() -> &'static Class {
        extern "C" {
            #[link_name = "OBJC_CLASS_$_NSValue"]
            static CLASS: Class;
        }
        unsafe { &CLASS }
    }

    /// Creates an immutable string object from a raw nullable pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSValue` instance.
    #[inline]
    pub const unsafe fn from_ptr(ptr: *mut Object) -> Self {
        Self(NSObject::from_ptr(ptr))
    }

    /// Creates an immutable object from a raw non-null pointer.
    ///
    /// # Safety
    ///
    /// The pointer must point to a valid `NSValue` instance.
    #[inline]
    pub const unsafe fn from_non_null_ptr(ptr: NonNull<Object>) -> Self {
        Self(NSObject::from_non_null_ptr(ptr))
    }
}

/// Arbitrary values.
impl NSValue {
    /// Creates a value object containing the specified value, interpreted with
    /// the specified Objective-C type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1551466-valuewithbytes).
    #[inline]
    pub unsafe fn with_bytes(value: *const c_void, objc_type: *const c_char) -> Self {
        _msg_send![
            Self::class(),
            valueWithBytes: value
            withObjCType: objc_type
        ]
    }

    /// Returns a pointer to a C string containing the Objective-C type of this
    /// object's value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1412365-objctype).
    #[inline]
    pub fn objc_type(&self) -> *const c_char {
        unsafe { _msg_send![self, objCType] }
    }

    /// Returns [`objc_type`](#method.objc_type) as a C string reference.
    #[inline]
    pub fn objc_type_cstr(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.objc_type()) }
    }

    /// Writes `size` bytes of the value to a pointer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/2919632-getvalue).
    ///
    /// # Exception Handling
    ///
    /// If `size` is the wrong number of bytes, an `NSInvalidArgumentException`
    /// is thrown.
    #[inline]
    pub unsafe fn write_value(&self, value: *mut c_void, size: NSUInteger) {
        _msg_send![
            self,
            getValue: value
            size: size
        ]
    }

    /// Writes the value to a slice buffer.
    ///
    /// # Exception Handling
    ///
    /// If `value` is the wrong number of bytes, an `NSInvalidArgumentException`
    /// is thrown.
    #[inline]
    pub fn write_value_slice(&self, value: &mut [u8]) {
        unsafe { self.write_value(value.as_mut_ptr().cast(), value.len()) };
    }

    /// Returns the value as some generic type.
    ///
    /// # Exception Handling
    ///
    /// If the type is the wrong size, an `NSInvalidArgumentException` is
    /// thrown.
    ///
    /// # Safety
    ///
    /// The value must have a valid memory representation for the return type.
    #[inline]
    pub unsafe fn transmute_value<T>(&self) -> T {
        let mut value = mem::MaybeUninit::<T>::uninit();
        self.write_value(value.as_mut_ptr().cast(), mem::size_of::<T>());
        value.assume_init()
    }
}

/// Pointer values.
impl NSValue {
    /// Creates a value object containing the specified pointer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1415975-valuewithpointer).
    #[inline]
    pub fn with_ptr(ptr: *const c_void) -> Self {
        unsafe { _msg_send![Self::class(), valueWithPointer: ptr] }
    }

    /// Returns the value as an untyped pointer.
    ///
    /// The value as a pointer to void. If the value object was not created to
    /// hold a pointer-sized data item, the result is undefined.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410668-pointervalue).
    #[inline]
    pub fn ptr_value(&self) -> *const c_void {
        unsafe { _msg_send![self, pointerValue] }
    }

    /// Creates a value object containing the specified pointer.
    ///
    /// This method is useful if you want to add an object to a collection but
    /// don’t want the collection to create a strong reference to it.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1408098-valuewithnonretainedobject).
    #[inline]
    pub fn with_nonretained_object(obj: *mut Object) -> Self {
        unsafe { _msg_send![Self::class(), valueWithNonretainedObject: obj] }
    }

    /// Returns the value as a non-retained pointer to an object.
    ///
    /// If the value was not created to hold a pointer-sized data item, the
    /// result is undefined.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410668-pointervalue).
    #[inline]
    pub fn nonretained_object_value(&self) -> *mut Object {
        unsafe { _msg_send![self, nonretainedObjectValue] }
    }
}

/// Range values.
impl NSValue {
    /// Creates a new value object containing the specified range.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410315-valuewithrange).
    #[inline]
    pub fn with_range(value: NSRange) -> Self {
        unsafe { _msg_send![Self::class(), valueWithRange: value] }
    }

    /// Returns the value as an `NSRange`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1413902-rangevalue).
    #[inline]
    pub fn range_value(&self) -> NSRange {
        unsafe { _msg_send![self, pointValue] }
    }
}

/// Foundation geometry values.
impl NSValue {
    /// Creates a new value object containing the specified point.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391106-valuewithpoint).
    #[inline]
    pub fn with_point(value: NSPoint) -> Self {
        unsafe { _msg_send![Self::class(), valueWithPoint: value] }
    }

    /// Returns the value as an `NSPoint`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391255-pointvalue).
    #[inline]
    pub fn point_value(&self) -> NSPoint {
        unsafe { _msg_send![self, pointValue] }
    }

    /// Creates a new value object containing the specified size.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391199-valuewithsize).
    #[inline]
    pub fn with_size(value: NSSize) -> Self {
        unsafe { _msg_send![Self::class(), valueWithSize: value] }
    }

    /// Returns the value as an `NSSize`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391301-sizevalue).
    #[inline]
    pub fn size_value(&self) -> NSSize {
        unsafe { _msg_send![self, sizeValue] }
    }

    /// Creates a new value object containing the specified rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391281-valuewithrect).
    #[inline]
    pub fn with_rect(value: NSRect) -> Self {
        unsafe { _msg_send![Self::class(), valueWithRect: value] }
    }

    /// Returns the value as an `NSRect`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391171-rectvalue).
    #[inline]
    pub fn rect_value(&self) -> NSRect {
        unsafe { _msg_send![self, rectValue] }
    }

    /// Creates a new value object containing the specified edge insets.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391181-valuewithedgeinsets).
    #[inline]
    pub fn with_edge_insets(value: NSEdgeInsets) -> Self {
        unsafe { _msg_send![Self::class(), valueWithEdgeInsets: value] }
    }

    /// Returns the value as an `NSEdgeInsets`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391123-edgeinsetsvalue).
    #[inline]
    pub fn edge_insets_value(&self) -> NSEdgeInsets {
        unsafe { _msg_send![self, edgeInsetsValue] }
    }
}
