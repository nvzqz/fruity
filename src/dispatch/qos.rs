/// The quality of service, or the execution priority, to apply to tasks.
///
/// This is semantically equivalent to Swift's
/// [`DispatchQoS`](https://developer.apple.com/documentation/dispatch/dispatchqos).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct DispatchQos {
    /// The quality-of-service class.
    pub qos_class: DispatchQosClass,

    /// The priority of a quality of service relative to others with the same
    /// class.
    pub relative_priority: i32,
}

impl Default for DispatchQos {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl DispatchQos {
    /// The quality-of-service class for user-interactive tasks, such as
    /// animations, event handling, or updating your app's user interface.
    pub const INTERACTIVE: Self = Self::new(DispatchQosClass::Interactive, 0);

    /// The quality-of-service class for tasks that prevent the user from
    /// actively using your app.
    pub const USER_INITIATED: Self = Self::new(DispatchQosClass::UserInitiated, 0);

    /// The default quality-of-service class.
    pub const DEFAULT: Self = Self::new(DispatchQosClass::Default, 0);

    /// The quality-of-service class for tasks that the user does not track
    /// actively.
    pub const UTILITY: Self = Self::new(DispatchQosClass::Utility, 0);

    /// The quality-of-service class for maintenance or cleanup tasks that you
    /// create.
    pub const BACKGROUND: Self = Self::new(DispatchQosClass::Background, 0);

    /// The absence of a quality-of-service class.
    pub const UNSPECIFIED: Self = Self::new(DispatchQosClass::Unspecified, 0);

    /// Creates a new instance with the specified QoS class and relative
    /// priority.
    #[inline]
    pub const fn new(qos_class: DispatchQosClass, relative_priority: i32) -> Self {
        Self {
            qos_class,
            relative_priority,
        }
    }

    /// Reassigns the value of [`qos_class`](#structfield.qos_class).
    #[inline]
    pub const fn with_qos_class(self, qos_class: DispatchQosClass) -> Self {
        Self { qos_class, ..self }
    }

    /// Reassigns the value of
    /// [`relative_priority`](#structfield.relative_priority).
    #[inline]
    pub const fn with_relative_priority(self, relative_priority: i32) -> Self {
        Self {
            relative_priority,
            ..self
        }
    }
}

/// Quality-of-service classes that specify the priorities for executing tasks.
///
/// This is semantically equivalent to Swift's
/// [`DispatchQoS.QoSClass`](https://developer.apple.com/documentation/dispatch/dispatchqos/qosclass).
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DispatchQosClass {
    /// The quality-of-service class for user-interactive tasks, such as
    /// animations, event handling, or updating your app's user interface.
    Interactive = 0x21,

    /// The quality-of-service class for tasks that prevent the user from
    /// actively using your app.
    UserInitiated = 0x19,

    /// The default quality-of-service class.
    Default = 0x15,

    /// The quality-of-service class for tasks that the user does not track
    /// actively.
    Utility = 0x11,

    /// The quality-of-service class for maintenance or cleanup tasks that you
    /// create.
    Background = 0x09,

    /// The absence of a quality-of-service class.
    Unspecified = 0x00,
}

impl Default for DispatchQosClass {
    #[inline]
    fn default() -> Self {
        Self::Default
    }
}
