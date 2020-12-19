use super::{NSEdgeInsets, NSPoint, NSRange, NSRect, NSSize};
use crate::core::Arc;
use crate::objc::{ClassType, NSObject, NSUInteger, ObjCObject};
use std::{
    ffi::CStr,
    mem,
    os::raw::{c_char, c_void},
};

// TODO: Implement methods defined in other frameworks.

objc_subclass! {
    /// A simple container for a single C or Objective-C data item.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue).
    pub class NSValue: NSObject<'static>;
}

/// Arbitrary values.
impl NSValue {
    /// Creates a value object containing the specified value, interpreted with
    /// the specified Objective-C type.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1551466-valuewithbytes).
    #[inline]
    #[doc(alias = "valueWithBytes")]
    pub unsafe fn from_bytes(value: *const c_void, objc_type: *const c_char) -> Arc<Self> {
        _msg_send_any![
            Self::class(),
            valueWithBytes: value
            withObjCType: objc_type
        ]
    }

    pub(crate) fn objc_type_single(&self) -> c_char {
        let objc_type = self.objc_type();
        let single = unsafe { *objc_type };
        if single == 0 || unsafe { *objc_type.add(1) != 0 } {
            0
        } else {
            single
        }
    }

    /// Returns a pointer to a C string containing the Objective-C type of this
    /// object's value.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1412365-objctype).
    #[inline]
    #[doc(alias = "objCType")]
    pub fn objc_type(&self) -> *const c_char {
        unsafe { _msg_send_any![self, objCType] }
    }

    /// Returns [`objc_type`](#method.objc_type) as a C string reference.
    #[inline]
    #[doc(alias = "objCType")]
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
    #[doc(alias = "getValue")]
    #[doc(alias = "getValue:size:")]
    pub unsafe fn write_value(&self, value: *mut c_void, size: NSUInteger) {
        _msg_send_any![
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
    #[doc(alias = "getValue")]
    #[doc(alias = "getValue:size:")]
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
    #[doc(alias = "getValue")]
    #[doc(alias = "getValue:size:")]
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
    #[doc(alias = "valueWithPointer")]
    #[doc(alias = "valueWithPointer:")]
    pub fn from_ptr(ptr: *const c_void) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithPointer: ptr] }
    }

    /// Returns the value as an untyped pointer.
    ///
    /// The value as a pointer to void. If the value object was not created to
    /// hold a pointer-sized data item, the result is undefined.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410668-pointervalue).
    #[inline]
    pub fn ptr_value(&self) -> *const c_void {
        unsafe { _msg_send_any![self, pointerValue] }
    }

    /// Creates a value object containing the specified pointer.
    ///
    /// This method is useful if you want to add an object to a collection but
    /// don’t want the collection to create a strong reference to it.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1408098-valuewithnonretainedobject).
    #[inline]
    #[doc(alias = "valueWithNonretainedObject")]
    #[doc(alias = "valueWithNonretainedObject:")]
    pub fn from_nonretained_object(obj: *mut ObjCObject) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithNonretainedObject: obj] }
    }

    /// Returns the value as a non-retained pointer to an object.
    ///
    /// If the value was not created to hold a pointer-sized data item, the
    /// result is undefined.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410668-pointervalue).
    #[inline]
    pub fn nonretained_object_value(&self) -> *mut ObjCObject {
        unsafe { _msg_send_strict![self, nonretainedObjectValue] }
    }
}

/// Range values.
impl NSValue {
    /// Creates a new value object containing the specified range.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1410315-valuewithrange).
    #[inline]
    #[doc(alias = "valueWithRange")]
    #[doc(alias = "valueWithRange:")]
    pub fn from_range(value: NSRange) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithRange: value] }
    }

    /// Returns the value as an `NSRange`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1413902-rangevalue).
    #[inline]
    pub fn range_value(&self) -> NSRange {
        unsafe { _msg_send_any![self, pointValue] }
    }
}

/// Foundation geometry values.
impl NSValue {
    /// Creates a new value object containing the specified point.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391106-valuewithpoint).
    #[inline]
    #[doc(alias = "valueWithPoint")]
    #[doc(alias = "valueWithPoint:")]
    pub fn from_point(value: NSPoint) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithPoint: value] }
    }

    /// Returns the value as an `NSPoint`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391255-pointvalue).
    #[inline]
    #[doc(alias = "pointValue")]
    pub fn point_value(&self) -> NSPoint {
        unsafe { _msg_send_any![self, pointValue] }
    }

    /// Creates a new value object containing the specified size.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391199-valuewithsize).
    #[inline]
    #[doc(alias = "valueWithSize")]
    #[doc(alias = "valueWithSize:")]
    pub fn from_size(value: NSSize) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithSize: value] }
    }

    /// Returns the value as an `NSSize`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391301-sizevalue).
    #[inline]
    #[doc(alias = "sizeValue")]
    pub fn size_value(&self) -> NSSize {
        unsafe { _msg_send_any![self, sizeValue] }
    }

    /// Creates a new value object containing the specified rectangle.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391281-valuewithrect).
    #[inline]
    #[doc(alias = "valueWithRect")]
    #[doc(alias = "valueWithRect:")]
    pub fn from_rect(value: NSRect) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithRect: value] }
    }

    /// Returns the value as an `NSRect`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391171-rectvalue).
    #[inline]
    #[doc(alias = "rectValue")]
    pub fn rect_value(&self) -> NSRect {
        unsafe { _msg_send_any![self, rectValue] }
    }

    /// Creates a new value object containing the specified edge insets.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391181-valuewithedgeinsets).
    #[inline]
    #[doc(alias = "valueWithEdgeInsets")]
    #[doc(alias = "valueWithEdgeInsets:")]
    pub fn from_edge_insets(value: NSEdgeInsets) -> Arc<Self> {
        unsafe { _msg_send_any![Self::class(), valueWithEdgeInsets: value] }
    }

    /// Returns the value as an `NSEdgeInsets`.
    ///
    /// See [documentation](https://developer.apple.com/documentation/foundation/nsvalue/1391123-edgeinsetsvalue).
    #[inline]
    #[doc(alias = "edgeInsetsValue")]
    pub fn edge_insets_value(&self) -> NSEdgeInsets {
        unsafe { _msg_send_any![self, edgeInsetsValue] }
    }
}
