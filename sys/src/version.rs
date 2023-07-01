use core::mem::size_of;
use zerocopy::{FromBytes, AsBytes};

pub use nvapi_macros::VersionedStructField;

/// NvAPI Version Definition
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, FromBytes, AsBytes)]
#[repr(transparent)]
pub struct NvVersion {
    pub data: u32,
}

impl NvVersion {
    pub const fn with_version(data: u32) -> Self {
        Self {
            data,
        }
    }

    pub const fn new(size: usize, version: u16) -> Self {
        let size = size as u32;

        // NOTE: version needs to contain upper bits of `size` when there's overflow
        #[cfg(debug_assertions)]
        match (size >> 16) as u16 {
            0 => (),
            size_shifted => {
                assert!(version & size_shifted == size_shifted)
            },
        }

        Self {
            data: size & 0xffff | (version as u32) << 16,
        }
    }

    #[doc(alias = "MAKE_NVAPI_VERSION")]
    pub const fn with_struct<T>(version: u16) -> Self {
        Self::new(size_of::<T>(), version)
    }

    #[doc(alias = "GET_NVAPI_VERSION")]
    pub const fn version(&self) -> u16 {
        (self.data >> 16) as u16
    }

    #[doc(alias = "GET_NVAPI_SIZE")]
    pub const fn size(&self) -> usize {
        self.data as usize & 0xffff
    }
}

impl From<u32> for NvVersion {
    fn from(version: u32) -> Self {
        Self::with_version(version)
    }
}

impl From<NvVersion> for u32 {
    fn from(ver: NvVersion) -> u32 {
        ver.data
    }
}

/// NvAPI Version Definition
///
/// Maintain per structure specific version
#[inline]
pub const fn MAKE_NVAPI_VERSION<T>(ver: u16) -> u32 {
    NvVersion::with_struct::<T>(ver).data
}

#[inline]
pub const fn GET_NVAPI_VERSION(ver: u32) -> u16 {
    NvVersion::with_version(ver).version()
}

#[inline]
pub const fn GET_NVAPI_SIZE(ver: u32) -> usize {
    NvVersion::with_version(ver).size()
}

pub trait VersionedStructField {
    fn nvapi_version_ref(&self) -> &NvVersion;
    fn nvapi_version_mut(&mut self) -> &mut NvVersion;

    fn nvapi_version_init<const VER: u16>(&mut self) where Self: StructVersion<VER> {
        *self.nvapi_version_mut() = Self::NVAPI_VERSION;
    }

    fn new_versioned<const VER: u16>() -> Self where
        Self: Sized + FromBytes + StructVersion<VER>,
    {
        let mut zero: Self = FromBytes::new_zeroed();
        zero.init_version();
        zero
    }
}

impl VersionedStructField for NvVersion {
    fn nvapi_version_mut(&mut self) -> &mut NvVersion {
        self
    }

    fn nvapi_version_ref(&self) -> &NvVersion {
        self
    }
}

pub trait VersionedStruct {
    fn nvapi_version(&self) -> NvVersion;
}

impl<T: VersionedStructField> VersionedStruct for T {
    fn nvapi_version(&self) -> NvVersion {
        *self.nvapi_version_ref()
    }
}

pub trait StructVersion<const VER: u16>: VersionedStruct {
    const NVAPI_VERSION: NvVersion;

    fn init_version(&mut self) where Self: VersionedStructField {
        self.nvapi_version_init::<VER>()
    }

    fn versioned() -> Self where Self: Sized + VersionedStructField + FromBytes {
        VersionedStructField::new_versioned::<VER>()
    }
}
