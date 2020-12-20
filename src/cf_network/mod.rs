//! [CFNetwork](https://developer.apple.com/documentation/cfnetwork)
//! framework.
//!
//! # Feature Flag
//!
//! This module corresponds to the **`cf_network`**
//! [feature flag](../index.html#feature-flags).
//!
//! # Documentation
//!
//! - [Introduction](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/Introduction/Introduction.html)
//! - [Concepts](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/Concepts/Concepts.html)
//! - [Working with Streams](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/CFStreamTasks/CFStreamTasks.html)
//! - [Communicating with HTTP Servers](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/CFHTTPTasks/CFHTTPTasks.html)
//! - [Working with FTP Servers](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/CFFTPTasks/CFFTPTasks.html)
//! - [Using Network Diagnostics](https://developer.apple.com/library/archive/documentation/Networking/Conceptual/CFNetwork/UsingNetworkDiagnostics/UsingNetworkDiagnostics.html)

#![cfg(feature = "cf_network")]

#[link(name = "CFNetwork", kind = "framework")]
extern "C" {}
