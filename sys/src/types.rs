use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fmt;
use crate::nvapi::NvVersion;

pub type NvBool = u8;

pub const NV_TRUE: NvBool = 1;
pub const NV_FALSE: NvBool = 0;

/// A boolean containing reserved bits
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BoolU32(pub u32);

impl BoolU32 {
    pub fn new(flag: bool, rest: u32) -> Self {
        let mut value = BoolU32(rest);
        value.set(flag);
        value
    }

    pub fn get(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn set(&mut self, value: bool) {
        self.0 = self.0 & 0xffffffe | if value { NV_TRUE } else { NV_FALSE } as u32
    }
}

impl From<bool> for BoolU32 {
    fn from(v: bool) -> Self {
        Self::new(v, 0)
    }
}

nvstruct! {
    pub struct NV_RECT {
        pub left: u32,
        pub top: u32,
        pub right: u32,
        pub bottom: u32,
    }
}

pub const NVAPI_GENERIC_STRING_MAX: usize = 4096;
pub const NVAPI_LONG_STRING_MAX: usize = 256;
pub const NVAPI_SHORT_STRING_MAX: usize = 64;

nvstruct! {
    pub struct NvSBox {
        pub sX: i32,
        pub sY: i32,
        pub sWidth: i32,
        pub sHeight: i32,
    }
}

nvstruct! {
    pub struct NvGUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
}

pub type NvLUID = NvGUID;

pub const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;

pub const NVAPI_MAX_PHYSICAL_BRIDGES: usize = 100;
pub const NVAPI_PHYSICAL_GPUS: usize = 32;
pub const NVAPI_MAX_LOGICAL_GPUS: usize = 64;
pub const NVAPI_MAX_AVAILABLE_GPU_TOPOLOGIES: usize = 256;
pub const NVAPI_MAX_AVAILABLE_SLI_GROUPS: usize = 265;
pub const NVAPI_MAX_GPU_TOPOLOGIES: usize = NVAPI_MAX_PHYSICAL_GPUS;
pub const NVAPI_MAX_GPU_PER_TOPOLOGY: usize = 8;
pub const NVAPI_MAX_DISPLAY_HEADS: usize = 2;
pub const NVAPI_ADVANCED_DISPLAY_HEADS: usize = 4;
pub const NVAPI_MAX_DISPLAYS: usize = NVAPI_PHYSICAL_GPUS * NVAPI_ADVANCED_DISPLAY_HEADS;
pub const NVAPI_MAX_ACPI_IDS: usize = 16;
pub const NVAPI_MAX_VIEW_MODES: usize = 8;
pub const NVAPI_MAX_HEADS_PER_GPU: usize = 32;

/// Maximum heads, each with `NVAPI_DESKTOP_RES` resolution
pub const NV_MAX_HEADS: usize = 4;
/// Maximum number of input video streams, each with a `NVAPI_VIDEO_SRC_INFO`
pub const NV_MAX_VID_STREAMS: usize = 4;
/// Maximum number of output video profiles supported
pub const NV_MAX_VID_PROFILES: usize = 4;

pub const NVAPI_SYSTEM_MAX_DISPLAYS: usize = NVAPI_MAX_PHYSICAL_GPUS * NV_MAX_HEADS;

pub const NVAPI_SYSTEM_MAX_HWBCS: usize = 128;
pub const NVAPI_SYSTEM_HWBC_INVALID_ID: usize = 0xffffffff;
pub const NVAPI_MAX_AUDIO_DEVICES: usize = 16;

pub type NvAPI_String = NvString<NVAPI_GENERIC_STRING_MAX>;
pub type NvAPI_LongString = NvString<NVAPI_LONG_STRING_MAX>;
pub type NvAPI_ShortString = NvString<NVAPI_SHORT_STRING_MAX>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NvString<const N: usize>(pub [c_char; N]);

impl<const N: usize> NvString<N> {
    pub fn as_bytes(&self) -> &[u8; N] {
        let ptr = &self.0 as *const [c_char; N] as *const [u8; N];
        unsafe { &*ptr }
    }

    pub fn str_bytes(&self) -> &[u8] {
        let n = self.iter().take_while(|&&c| c != 0).count();
        &self.as_bytes()[..n]
    }

    pub fn as_cstr(&self) -> Result<&CStr, std::ffi::FromBytesWithNulError> {
        CStr::from_bytes_with_nul(self.str_bytes())
    }

    pub fn to_cstr(&self) -> Cow<CStr> {
        match self.as_cstr() {
            Ok(str) => Cow::Borrowed(str),
            Err(..) => Cow::Owned(unsafe {
                CString::from_vec_unchecked(self.str_bytes().into())
            }),
        }
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.str_bytes())
    }
}

impl<const N: usize> Default for NvString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Deref for NvString<N> {
    type Target = [c_char; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for NvString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<NvString<N>> for String {
    fn from(str: NvString<N>) -> String {
        str.to_string_lossy().into_owned()
    }
}

/// NvAPI Version Definition
///
/// Maintain per structure specific version
pub const fn MAKE_NVAPI_VERSION<T>(ver: u16) -> u32 {
    NvVersion::with_struct::<T>(ver).data
}

pub const fn GET_NVAPI_VERSION(ver: u32) -> u16 {
    NvVersion::with_version(ver).version()
}

pub const fn GET_NVAPI_SIZE(ver: u32) -> usize {
    NvVersion::with_version(ver).size()
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Padding<T> {
    pub data: T,
}

impl<T: Copy + Default, const N: usize> Default for Padding<[T; N]> {
    fn default() -> Self {
        Self {
            data: [Default::default(); N],
        }
    }
}

impl<T: Default + PartialEq, const N: usize> Padding<[T; N]> {
    pub fn all_zero(&self) -> bool {
        let zero = T::default();
        self.data.iter().all(|v| v == &zero)
    }

    pub fn check_zero(&self) -> Result<(), crate::ArgumentRangeError> {
        match self.all_zero() {
            true => Ok(()),
            false => Err(crate::ArgumentRangeError),
        }
    }
}

impl<T: Default + PartialEq + fmt::Debug, const N: usize> fmt::Debug for Padding<[T; N]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.all_zero() {
            write!(f, "[0; {}]", N)
        } else {
            fmt::Debug::fmt(&self.data, f)
        }
    }
}
