/// The frequency with which a [`DispatchQueue`](struct.DispatchQueue.html)
/// creates autorelease pools for its tasks.
///
/// Documentation:
/// [Swift](https://developer.apple.com/documentation/dispatch/dispatchqueue/autoreleasefrequency) |
/// [Objective-C](https://developer.apple.com/documentation/dispatch/dispatch_autorelease_frequency_t)
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DispatchAutoreleaseFrequency {
    /// The queue inherits its autorelease frequency from its target queue.
    Inherit = 0,

    /// The queue configures an autorelease pool before the execution of a block
    /// and releases the objects in that pool after the block finishes
    /// executing.
    WorkItem = 1,

    /// The queue does not set up an autorelease pool around executed blocks.
    Never = 2,
}

impl Default for DispatchAutoreleaseFrequency {
    #[inline]
    fn default() -> Self {
        Self::Inherit
    }
}
