/// Specify send preferences to the `AESend` function.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/aesendmode?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AESendMode(pub i32);

impl AESendMode {
    /// The reply preference—your application does not want a reply Apple event.
    /// If you set the bit specified by this constant, the server processes the
    /// Apple event as soon as it has the opportunity.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaenoreply?language=objc).
    #[doc(alias = "kAENoReply")]
    pub const AE_NO_REPLY: Self = Self(0x00000001);

    /// The reply preference—your application wants a reply Apple event. If you
    /// set the bit specified by this constant, the reply appears in your event
    /// queue as soon as the server has the opportunity to process and respond
    /// to your Apple event.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaequeuereply?language=objc).
    #[doc(alias = "kAEQueueReply")]
    pub const AE_QUEUE_REPLY: Self = Self(0x00000002);

    /// The reply preference—your application wants a reply Apple event and is
    /// willing to give up the processor while waiting for the reply. For
    /// example, if the server application is on the same computer as your
    /// application, your application yields the processor to allow the server
    /// to respond to your Apple event.
    ///
    /// If you set the bit specified by this constant, you must provide an idle
    /// function. This function should process any update events, null events,
    /// operating-system events, or activate events that occur while your
    /// application is waiting for a reply. For more information on idle
    /// routines, see AEInteractWithUser.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaewaitreply?language=objc).
    #[doc(alias = "kAEWaitReply")]
    pub const AE_WAIT_REPLY: Self = Self(0x00000003);

    /// Deprecated and unsupported in macOS. The reconnection preference—the
    /// Apple Event Manager must not automatically try to reconnect if it
    /// receives a `sessClosedErr` result code from the PPC Toolbox. If you
    /// don’t set this flag, the Apple Event Manager automatically attempts to
    /// reconnect and reestablish the session.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaedontreconnect?language=objc).
    #[doc(alias = "kAEDontReconnect")]
    pub const AE_DONT_RECONNECT: Self = Self(0x00000080);

    /// Deprecated and unsupported in macOS. The return receipt preference—the
    /// sender wants to receive a return receipt for this Apple event from the
    /// Event Manager. (A return receipt means only that the receiving
    /// application accepted the Apple event the Apple event may or may not be
    /// handled successfully after it is accepted.) If the receiving application
    /// does not send a return receipt before the request times out, `AESend`
    /// returns `errAETimeout` as its function result.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaewantreceipt?language=objc).
    #[doc(alias = "kAEWantReceipt")]
    pub const AE_WANT_RECEIPT: Self = Self(0x00000200);

    /// The user interaction preference—the server application should never
    /// interact with the user in response to the Apple event. If you set the
    /// bit specified by this constant, the `AEInteractWithUser` function (when
    /// called by the server) returns the `errAENoUserInteraction` result code.
    /// When you send an Apple event to a remote application, the default is to
    /// set this bit.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeneverinteract?language=objc).
    #[doc(alias = "kAENeverInteract")]
    pub const AE_NEVER_INTERACT: Self = Self(0x00000010);

    /// The user interaction preference—the server application can interact with
    /// the user in response to the Apple event. By convention, you set the bit
    /// specified by this constant if the user needs to supply information to
    /// the server. If you set the bit and the server allows interaction, the
    /// `AEInteractWithUser` function either brings the server application to
    /// the foreground or posts a notification request. When you send an Apple
    /// event to a local application, the default is to set this bit.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaecaninteract?language=objc).
    #[doc(alias = "kAECanInteract")]
    pub const AE_CAN_INTERACT: Self = Self(0x00000020);

    /// The user interaction preference—the server application should always
    /// interact with the user in response to the Apple event. By convention,
    /// you set the bit specified by this constant whenever the server
    /// application normally asks a user to confirm a decision or interact in
    /// any other way, even if no additional information is needed from the
    /// user. If you set the bit specified by this constant, the
    /// `AEInteractWithUser` function either brings the server application to
    /// the foreground or posts a notification request.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaealwaysinteract?language=objc).
    #[doc(alias = "kAEAlwaysInteract")]
    pub const AE_ALWAYS_INTERACT: Self = Self(0x00000030);

    /// The application switch preference—if both the client and server allow
    /// interaction, and if the client application is the active application on
    /// the local computer and is waiting for a reply (that is, it has set the
    /// [`AE_WAIT_REPLY`](Self::AE_WAIT_REPLY) flag), `AEInteractWithUser`
    /// brings the server directly to the foreground. Otherwise,
    /// `AEInteractWithUser` uses the Notification Manager to request that the
    /// user bring the server application to the foreground.
    ///
    /// You should specify the `AE_CAN_SWITCH_LAYER` flag only when the client
    /// and server applications reside on the same computer. In general, you
    /// should not set this flag if it would be confusing or inconvenient to the
    /// user for the server application to come to the front unexpectedly. This
    /// flag is ignored if you are sending an Apple event to a remote computer.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaecanswitchlayer?language=objc).
    #[doc(alias = "kAECanSwitchLayer")]
    pub const AE_CAN_SWITCH_LAYER: Self = Self(0x00000040);

    /// The recording preference—your application is sending an event to itself
    /// but does not want the event recorded. When Apple event recording is on,
    /// the Apple Event Manager records a copy of every event your application
    /// sends to itself except for those events for which this flag is set.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaedontrecord?language=objc).
    #[doc(alias = "kAEDontRecord")]
    pub const AE_DONT_RECORD: Self = Self(0x00001000);

    /// The execution preference—your application is sending an Apple event to
    /// itself for recording purposes only—that is, you want the Apple Event
    /// Manager to send a copy of the event to the recording process but you do
    /// not want your application actually to receive the event.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaedontexecute?language=objc).
    #[doc(alias = "kAEDontExecute")]
    pub const AE_DONT_EXECUTE: Self = Self(0x00002000);

    /// Allow processing of non-reply Apple events while awaiting a synchronous
    /// Apple event reply (you specified [`AE_WAIT_REPLY`](Self::AE_WAIT_REPLY)
    /// for the reply preference).
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaeprocessnonreplyevents?language=objc).
    #[doc(alias = "kAEProcessNonReplyEvents")]
    pub const AE_PROCESS_NON_REPLY_EVENTS: Self = Self(0x00008000);

    /// If set, don't automatically add any sandbox or other annotations to the
    /// event.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kaedonotautomaticallyaddannotationstoevent?language=objc).
    #[doc(alias = "kAEDoNotAutomaticallyAddAnnotationsToEvent")]
    pub const AE_DO_NOT_AUTOMATICALLY_ADD_ANNOTATIONS_TO_EVENT: Self = Self(0x00010000);
}
