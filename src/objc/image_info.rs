use std::fmt;

/// Information for Objective-C binaries.
///
/// This corresponds to data that lives in the `__DATA,__objc_imageinfo` section
/// and often is defined by a `L_OBJC_IMAGE_INFO` symbol.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageInfo {
    /// This seems to always be 0.
    pub version: u32,

    /// Indicates runtime features to use.
    pub flags: ImageInfoFlags,
}

/// Runtime features indicated in [`ImageInfo`].
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ImageInfoFlags(u32);

enum ImageInfoFlag {
    IsReplacement = 0,
    SupportsGc = 1,
    RequiresGc = 2,
    OptimizedDyDyld = 3,
    SupportsCompaction = 4,
    IsSimulated = 5,
    HasCategoryClassProperties = 6,
}

// TODO: Investigate other flags.
//
// Swift binaries seem to have more flags, as seen in these examples:
// - 0000 0101 0000 0011 0000 0111 0100 0000 (84084544)
// - 0000 0101 0000 0001 0000 0111 0100 0000 (83953472)

const SWIFT_ABI_SHIFT: u32 = 8;
const SWIFT_ABI_MASK: u32 = 0xFF << SWIFT_ABI_SHIFT;

impl fmt::Debug for ImageInfoFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ImageInfoFlags")
            .field("is_replacement", &self.is_replacement())
            .field("supports_gc", &self.supports_gc())
            .field("requires_gc", &self.requires_gc())
            .field("optimized_by_dyld", &self.optimized_by_dyld())
            .field("supports_compaction", &self.supports_compaction())
            .field("is_simulated", &self.is_simulated())
            .field(
                "has_category_class_properties",
                &self.has_category_class_properties(),
            )
            .field("swift_abi", &self.swift_abi())
            .finish()
    }
}

// TODO: Document these methods.
#[allow(missing_docs)]
impl ImageInfoFlags {
    #[inline]
    const fn with_flag(self, flag: ImageInfoFlag, yes: bool) -> Self {
        let shift = flag as u32;
        Self((self.0 & !(1 << shift)) | ((yes as u32) << shift))
    }

    #[inline]
    const fn contains(&self, flag: ImageInfoFlag) -> bool {
        self.0 & (1 << flag as u32) != 0
    }

    #[inline]
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    #[inline]
    pub const fn into_bits(self) -> u32 {
        self.0
    }

    #[inline]
    pub fn is_replacement(&self) -> bool {
        self.contains(ImageInfoFlag::IsReplacement)
    }

    #[inline]
    pub const fn with_replacement(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::IsReplacement, yes)
    }

    #[inline]
    pub fn supports_gc(&self) -> bool {
        self.contains(ImageInfoFlag::SupportsGc)
    }

    #[inline]
    pub const fn with_supports_gc(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::SupportsGc, yes)
    }

    #[inline]
    pub fn requires_gc(&self) -> bool {
        self.contains(ImageInfoFlag::RequiresGc)
    }

    #[inline]
    pub const fn with_requires_gc(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::RequiresGc, yes)
    }

    #[inline]
    pub fn optimized_by_dyld(&self) -> bool {
        self.contains(ImageInfoFlag::OptimizedDyDyld)
    }

    #[inline]
    pub const fn with_optimized_by_dyld(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::OptimizedDyDyld, yes)
    }

    #[inline]
    pub fn supports_compaction(&self) -> bool {
        self.contains(ImageInfoFlag::SupportsCompaction)
    }

    #[inline]
    pub const fn with_supports_compaction(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::SupportsCompaction, yes)
    }

    #[inline]
    pub fn is_simulated(&self) -> bool {
        self.contains(ImageInfoFlag::IsSimulated)
    }

    #[inline]
    pub const fn with_simulated(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::IsSimulated, yes)
    }

    #[inline]
    pub fn has_category_class_properties(&self) -> bool {
        self.contains(ImageInfoFlag::HasCategoryClassProperties)
    }

    #[inline]
    pub const fn with_category_class_properties(self, yes: bool) -> Self {
        self.with_flag(ImageInfoFlag::HasCategoryClassProperties, yes)
    }

    #[inline]
    pub fn swift_abi(&self) -> u8 {
        (self.0 >> SWIFT_ABI_SHIFT) as u8
    }

    #[inline]
    pub const fn with_swift_abi(mut self, version: u8) -> Self {
        self.0 &= !SWIFT_ABI_MASK;
        self.0 |= (version as u32) << SWIFT_ABI_SHIFT;
        self
    }
}
