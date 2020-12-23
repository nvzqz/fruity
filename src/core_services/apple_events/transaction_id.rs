/// Specifies a transaction ID.
///
/// A transaction is a sequence of Apple events that are sent back and forth
/// between the client and server applications, beginning with the client’s
/// initial request for a service.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/aetransactionid?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AETransactionID(pub i32);

impl AETransactionID {
    /// You pass this value for the transactionID parameter of the
    /// `AECreateAppleEvent` function if the Apple event is not one of a series
    /// of interdependent Apple events.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kanytransactionid?language=objc).
    #[doc(alias = "kAnyTransactionID")]
    pub const ANY: Self = Self(0);
}
