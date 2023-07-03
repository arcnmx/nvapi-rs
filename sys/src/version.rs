use core::mem::{MaybeUninit, size_of};
use zerocopy::{FromBytes, AsBytes};

pub use nvapi_macros::VersionedStruct;

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
        //debug_assert!(size < 0x10000);
        Self {
            data: size as u32 | (version as u32) << 16,
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

pub trait VersionedStruct: Sized {
    fn nvapi_version_mut(&mut self) -> &mut NvVersion;
    fn nvapi_version(&self) -> NvVersion;
}

impl VersionedStruct for NvVersion {
    fn nvapi_version_mut(&mut self) -> &mut NvVersion {
        self
    }

    fn nvapi_version(&self) -> NvVersion {
        *self
    }
}

pub trait StructVersion<const VER: u16 = 0>: VersionedStruct {
    const NVAPI_VERSION: NvVersion;

    fn versioned() -> Self {
        let mut zero = unsafe {
            MaybeUninit::<Self>::zeroed().assume_init()
        };
        *zero.nvapi_version_mut() = Self::NVAPI_VERSION;
        zero
    }
}
