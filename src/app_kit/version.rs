/// Constants for determining which version of AppKit is available.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct NSAppKitVersion(pub f64);

impl NSAppKitVersion {
    /// The most recent version of AppKit.
    #[inline]
    #[doc(alias = "NSAppKitVersionNumber")]
    pub fn current() -> Self {
        extern "C" {
            static NSAppKitVersionNumber: NSAppKitVersion;
        }
        unsafe { NSAppKitVersionNumber }
    }
}

/// Known versions.
///
/// If you notice this list is outdated, please create a
/// [pull request](https://github.com/nvzqz/fruity) to update it.
#[allow(missing_docs)]
impl NSAppKitVersion {
    pub const V10_0: Self = Self(577.0);
    pub const V10_1: Self = Self(620.0);
    pub const V10_2: Self = Self(663.0);
    pub const V10_2_3: Self = Self(663.6);
    pub const V10_3: Self = Self(743.0);
    pub const V10_3_2: Self = Self(743.14);
    pub const V10_3_3: Self = Self(743.2);
    pub const V10_3_5: Self = Self(743.24);
    pub const V10_3_7: Self = Self(743.33);
    pub const V10_3_9: Self = Self(743.36);
    pub const V10_4: Self = Self(824.0);
    pub const V10_4_1: Self = Self(824.1);
    pub const V10_4_3: Self = Self(824.23);
    pub const V10_4_4: Self = Self(824.33);
    pub const V10_4_7: Self = Self(824.41);
    pub const V10_5: Self = Self(949.0);
    pub const V10_5_2: Self = Self(949.27);
    pub const V10_5_3: Self = Self(949.33);
    pub const V10_6: Self = Self(1038.0);
    pub const V10_7: Self = Self(1138.0);
    pub const V10_7_2: Self = Self(1138.23);
    pub const V10_7_3: Self = Self(1138.32);
    pub const V10_7_4: Self = Self(1138.47);
    pub const V10_8: Self = Self(1187.0);
    pub const V10_9: Self = Self(1265.0);
    pub const V10_10: Self = Self(1343.0);
    pub const V10_10_2: Self = Self(1344.0);
    pub const V10_10_3: Self = Self(1347.0);
    pub const V10_10_4: Self = Self(1348.0);
    pub const V10_10_5: Self = Self(1348.0);
    pub const V10_10_MAX: Self = Self(1349.0);
    pub const V10_11: Self = Self(1404.0);
    pub const V10_11_1: Self = Self(1404.13);
    pub const V10_11_2: Self = Self(1404.34);
    pub const V10_11_3: Self = Self(1404.34);
    pub const V10_12: Self = Self(1504.0);
    pub const V10_12_1: Self = Self(1504.60);
    pub const V10_12_2: Self = Self(1504.76);
    pub const V10_13: Self = Self(1561.0);
    pub const V10_13_1: Self = Self(1561.1);
    pub const V10_13_2: Self = Self(1561.2);
    pub const V10_13_4: Self = Self(1561.4);
    pub const V10_14: Self = Self(1671.0);
    pub const V10_14_1: Self = Self(1671.1);
    pub const V10_14_2: Self = Self(1671.2);
    pub const V10_14_3: Self = Self(1671.3);
    pub const V10_14_4: Self = Self(1671.4);
    pub const V10_14_5: Self = Self(1671.5);
    pub const V10_15: Self = Self(1894.0);
}
