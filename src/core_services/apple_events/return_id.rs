/// Specifies a return ID for a created Apple event.
///
/// See [documentation](https://developer.apple.com/documentation/coreservices/aereturnid?language=objc).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct AEReturnID(pub i16);

impl AEReturnID {
    /// If you pass this value for the `returnID` parameter of the
    /// `AECreateAppleEvent` function, the Apple Event Manager assigns to the
    /// created Apple event a return ID that is unique to the current session.
    ///
    /// If you pass any other value, the Apple Event Manager assigns that value
    /// for the ID.
    ///
    /// See [documentation](https://developer.apple.com/documentation/coreservices/kautogeneratereturnid?language=objc).
    #[doc(alias = "kAutoGenerateReturnID")]
    pub const AUTO_GENERATE: Self = Self(-1);
}
