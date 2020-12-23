use std::ffi::c_void;

/// A pointer to an opaque data type that provides storage for an
/// [`AEDesc`](super::AEDesc) descriptor.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/coreservices/aedatastorage?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/coreservices/aedatastorage?language=objc)
pub type AEDataStorage = *mut AEDataStorageType;

/// An opaque data type used to store data in Apple event descriptors.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/coreservices/aedatastoragetype?language=swift) |
/// [Objective-C](https://developer.apple.com/documentation/coreservices/aedatastoragetype?language=objc)
pub type AEDataStorageType = *mut c_void;
