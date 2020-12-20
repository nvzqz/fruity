//! [System Configuration](https://developer.apple.com/documentation/systemconfiguration)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`system_configuration`**
//! [feature flag](../index.html#feature-flags).
//!
//! # Documentation
//!
//! - [Introduction](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/SystemConfigFrameworks/SC_Intro/SC_Intro.html)
//! - [Goals and Architecture](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/SystemConfigFrameworks/SC_Overview/SC_Overview.html)
//! - [Components](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/SystemConfigFrameworks/SC_Components/SC_Components.html)
//! - [Schema](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/SystemConfigFrameworks/SC_UnderstandSchema/SC_UnderstandSchema.html)
//! - [Determining Reachability and Getting Connected](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/SystemConfigFrameworks/SC_ReachConnect/SC_ReachConnect.html)

#![cfg(feature = "system_configuration")]

#[link(name = "SystemConfiguration", kind = "framework")]
extern "C" {}
