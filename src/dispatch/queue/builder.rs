use super::{
    super::{DispatchAutoreleaseFrequency, DispatchQos, DispatchQosClass},
    dispatch_queue_attr_t, dispatch_queue_t, DispatchQueue, DispatchQueueAttributes,
};
use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_ulong},
    ptr,
};

/// Configures and creates a [`DispatchQueue`](struct.DispatchQueue.html).
#[must_use = "This does nothing until `build` is called"]
#[derive(Clone)]
pub struct DispatchQueueBuilder<'a> {
    qos: DispatchQos,
    attr: DispatchQueueAttributes,
    autorelease_frequency: DispatchAutoreleaseFrequency,
    target: Option<DispatchQueue>,
    label: Option<&'a CStr>,
}

impl Default for DispatchQueueBuilder<'_> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl DispatchQueueBuilder<'_> {
    /// Creates a [`DispatchQueue`](struct.DispatchQueue.html) builder.
    #[inline]
    pub const fn new() -> Self {
        Self {
            qos: DispatchQos::UNSPECIFIED,
            attr: DispatchQueueAttributes::SERIAL,
            autorelease_frequency: DispatchAutoreleaseFrequency::Inherit,
            target: None,
            label: None,
        }
    }

    /// Creates a new [`DispatchQueue`](struct.DispatchQueue.html) with the
    /// configuration specified by this builder.
    #[inline]
    pub fn build(&self) -> DispatchQueue {
        extern "C" {
            // fn dispatch_queue_create(
            //     label: *const c_char,
            //     attr: dispatch_queue_attr_t,
            // ) -> dispatch_queue_t;

            // available(macOS 10.12, iOS 10.0, tvOS 10.0, watchOS 3.0)
            //
            // TODO: Handle availability. Call `dispatch_queue_create` if this
            // is unavailable
            fn dispatch_queue_create_with_target(
                label: *const c_char,
                attr: dispatch_queue_attr_t,
                target: dispatch_queue_t,
            ) -> DispatchQueue;

            // available(macOS 10.12, iOS 10.0, tvOS 10.0, watchOS 3.0)
            //
            // TODO: Handle availability.
            fn dispatch_queue_attr_make_with_autorelease_frequency(
                attr: dispatch_queue_attr_t,
                frequency: c_ulong,
            ) -> dispatch_queue_attr_t;

            // available(macOS 10.10, iOS 8.0)
            //
            // TODO: Handle availability.
            fn dispatch_queue_attr_make_with_qos_class(
                attr: dispatch_queue_attr_t,
                qos_class: DispatchQosClass,
                relative_priority: c_int,
            ) -> dispatch_queue_attr_t;
        }

        // TODO: Determine if retain/release is needed for
        // `dispatch_queue_attr_t` here.
        let mut attr = self.attr._to_raw();

        if self.autorelease_frequency != DispatchAutoreleaseFrequency::Inherit {
            unsafe {
                attr = dispatch_queue_attr_make_with_autorelease_frequency(
                    attr,
                    self.autorelease_frequency as c_ulong,
                );
            }
        }

        if self.qos != DispatchQos::UNSPECIFIED {
            unsafe {
                attr = dispatch_queue_attr_make_with_qos_class(
                    attr,
                    self.qos.qos_class,
                    self.qos.relative_priority,
                );
            }
        }

        let label = match self.label {
            Some(label) => label.as_ptr(),
            None => ptr::null(),
        };

        let target = match &self.target {
            Some(target) => target._as_queue(),
            None => ptr::null_mut(),
        };

        unsafe { dispatch_queue_create_with_target(label, attr, target) }
    }
}

/// Configuring the dispatch queue.
impl<'a> DispatchQueueBuilder<'a> {
    /// Sets the queue's quality-of-service (QoS), or execution priority.
    ///
    /// The value of `qos.relative_priority` is clamped between `0` and `-15`
    /// (`QOS_MIN_RELATIVE_PRIORITY`).
    ///
    /// Default value:
    /// [`DispatchQos::UNSPECIFIED`](struct.DispatchQos.html#associatedconstant.UNSPECIFIED).
    #[inline]
    pub const fn qos(self, qos: DispatchQos) -> Self {
        // `relative_priority` method handles clamping.
        self.qos_class(qos.qos_class)
            .relative_priority(qos.relative_priority)
    }

    /// Sets the queue's quality-of-service (QoS) class.
    ///
    /// Default value:
    /// [`DispatchQosClass::Unspecified`](struct.DispatchQos.html#associatedconstant.Unspecified).
    #[inline]
    pub const fn qos_class(mut self, qos_class: DispatchQosClass) -> Self {
        self.qos.qos_class = qos_class;
        self
    }

    /// Sets the queue's quality-of-service (QoS) relative priority.
    ///
    /// The value is clamped between `0` and `-15`
    /// (`QOS_MIN_RELATIVE_PRIORITY`).
    ///
    /// Default value: `0`.
    #[inline]
    pub const fn relative_priority(mut self, relative_priority: i32) -> Self {
        // The value must be clamped or else
        // `dispatch_queue_attr_make_with_qos_class` returns null.

        const QOS_MIN_RELATIVE_PRIORITY: i32 = -15;

        self.qos.relative_priority = if relative_priority > 0 {
            0
        } else if relative_priority < QOS_MIN_RELATIVE_PRIORITY {
            QOS_MIN_RELATIVE_PRIORITY
        } else {
            relative_priority
        };

        self
    }

    /// Sets the attributes that define the behavior of the queue.
    ///
    /// Default value:
    /// [`DispatchQueueAttributes::SERIAL`](struct.DispatchQueueAttributes.html#associatedconstant.SERIAL).
    #[inline]
    pub const fn attr(mut self, attr: DispatchQueueAttributes) -> Self {
        self.attr = attr;
        self
    }

    /// Sets the frequency with which the queue creates autorelease pools for
    /// its tasks.
    ///
    /// Default value:
    /// [`DispatchAutoreleaseFrequency::Inherit`](struct.DispatchAutoreleaseFrequency.html#variant.Inherit).
    #[inline]
    pub const fn autorelease_frequency(
        mut self,
        autorelease_frequency: DispatchAutoreleaseFrequency,
    ) -> Self {
        self.autorelease_frequency = autorelease_frequency;
        self
    }

    /// Sets a target queue to which all blocks are redirected, while keeping
    /// the semantics of the created queue.
    ///
    /// See "Dispatch queues" in
    /// [`dispatch_set_target_queue`](https://developer.apple.com/documentation/dispatch/1452989-dispatch_set_target_queue)
    /// for details.
    ///
    /// Default value: `None`.
    #[inline]
    pub fn target(mut self, target: DispatchQueue) -> Self {
        self.target = Some(target);
        self
    }

    /// Sets a string label to attach to the queue to uniquely identify it in
    /// debugging tools such as Instruments, `sample`, stackshots, and crash
    /// reports.
    ///
    /// Because applications, libraries, and frameworks can all create their own
    /// dispatch queues, a reverse-DNS naming style (`com.example.myqueue`) is
    /// recommended.
    ///
    /// Default value: `None`.
    #[inline]
    pub const fn label(mut self, label: &'a CStr) -> Self {
        self.label = Some(label);
        self
    }
}
