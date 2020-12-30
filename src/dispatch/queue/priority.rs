// Used for simplifying doc comments.
#[allow(unused_imports)]
use super::DispatchQosClass;

/// The execution priority for tasks in a global concurrent queue.
///
/// In macOS 10.10 and later, use [`DispatchQosClass`] to specify the priority
/// of tasks instead.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(isize)] // long
pub enum DispatchQueuePriority {
    /// The queue is scheduled for execution before any default priority or low
    /// priority queue.
    ///
    /// This maps to [`DispatchQosClass::UserInitiated`].
    High = 2,

    /// The queue is scheduled for execution after all high priority queues have
    /// been scheduled, but before any low priority queues have been scheduled.
    ///
    /// This maps to [`DispatchQosClass::Default`].
    Default = 0,

    /// The queue is scheduled for execution after all default priority and high
    /// priority queues have been scheduled.
    ///
    /// This maps to [`DispatchQosClass::Utility`].
    Low = -2,

    /// The queue is scheduled for execution after all high priority queues have
    /// been scheduled and the system runs items on a thread whose priority is
    /// set for background status. Such a thread has the lowest priority and any
    /// disk I/O is throttled to minimize the impact on the system.
    ///
    /// This maps to [`DispatchQosClass::Background`].
    Background = i16::MIN as _,
}

impl Default for DispatchQueuePriority {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}
