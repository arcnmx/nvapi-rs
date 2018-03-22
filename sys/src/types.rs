use std::os::raw::c_char;
use std::mem::size_of;

pub type NvBool = u8;

pub const NV_TRUE: NvBool = 1;
pub const NV_FALSE: NvBool = 0;

/// A boolean containing reserved bits
#[derive(Copy, Clone, Debug)]
pub struct BoolU32(pub u32);

impl BoolU32 {
    pub fn get(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn set(&mut self, value: bool) {
        self.0 = self.0 & 0xffffffe | if value { NV_TRUE } else { NV_FALSE } as u32
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

pub type NvAPI_String = [c_char; NVAPI_GENERIC_STRING_MAX];
pub type NvAPI_LongString = [c_char; NVAPI_LONG_STRING_MAX];
pub type NvAPI_ShortString = [c_char; NVAPI_SHORT_STRING_MAX];

pub fn short_string() -> NvAPI_ShortString {
    [0; NVAPI_SHORT_STRING_MAX]
}

pub fn long_string() -> NvAPI_LongString {
    [0; NVAPI_LONG_STRING_MAX]
}

pub fn string() -> NvAPI_String {
    [0; NVAPI_GENERIC_STRING_MAX]
}

/// NvAPI Version Definition
///
/// Maintain per structure specific version, meant to be a `const fn`.
pub fn MAKE_NVAPI_VERSION<T>(ver: u16) -> u32 {
    size_of::<T>() as u32 | (ver as u32) << 16
}

pub fn GET_NVAPI_VERSION(ver: u32) -> u16 {
    (ver >> 16) as u16
}

pub fn GET_NVAPI_SIZE(ver: u32) -> usize {
    ver as usize & 0xffff
}
