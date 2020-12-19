mod description;

pub use description::*;

/// A pointer to the function of a method implementation.
///
/// The first argument is a pointer to `self` (that is, the memory for the
/// particular instance of this class, or, for a class method, a pointer to the
/// metaclass). The second argument is the method selector. The method arguments
/// follow.
///
/// See [documentation](https://developer.apple.com/documentation/objectivec/objective-c_runtime/imp?language=objc).
pub type Imp = unsafe extern "C" fn();
