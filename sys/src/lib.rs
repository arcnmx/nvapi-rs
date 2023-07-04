#![allow(non_camel_case_types, non_snake_case)]
#![doc(html_root_url = "https://docs.rs/nvapi-sys/0.2.0")]

#[macro_use]
mod macros;
mod boolu32;
mod string;

pub mod nvid;
pub mod nvapi;
pub mod status;
pub mod value;
pub mod array;
pub mod version;
pub mod clock_mask;

/// NVAPI Handles - These handles are retrieved from various calls and passed in
/// to others in NvAPI These are meant to be opaque types. Do not assume they
/// correspond to indices, HDCs, display indexes or anything else.
///
/// Most handles remain valid until a display re-configuration (display mode set)
/// or GPU reconfiguration (going into or out of SLI modes) occurs. If
/// NVAPI_HANDLE_INVALIDATED is received by an app, it should discard all
/// handles, and re-enumerate them.
pub mod handles;

/// The display driver APIs are used to retrieve information about the display driver.
pub mod driverapi;

pub mod sysgeneral;

/// Video Input Output (VIO) API
pub mod vidio;

/// The GPU APIs retrieve and control various attributes of the GPU, such as outputs, VBIOS revision, APG rate, frame buffer size, and thermal settings.
pub mod gpu;

/// Sync Display APIs
pub mod gsync;

/// I2C API - Provides ability to read or write data using I2C protocol.
/// These APIs allow I2C access only to DDC monitors
pub mod i2c;

#[cfg(windows)]
pub mod dx;

pub mod dispcontrol;

pub use self::array::Array;
pub use self::version::NvVersion;
pub use self::boolu32::BoolU32;
pub use self::string::NvString;
pub use self::clock_mask::ClockMask;
pub use self::nvid::Api;
pub use self::nvapi::*;
pub use self::status::{NvAPI_Status, Status};
pub use self::value::{NvEnum, NvBits, NvValue};

use std::error::Error as StdError;
use std::{result, fmt};
use std::convert::Infallible;

pub mod api {
    pub use crate::handles::*;
    #[cfg(windows)]
    pub use crate::dx::*;
    pub use crate::gpu::*;
    pub use crate::gpu::display::*;
    pub use crate::gpu::ecc::*;
    pub use crate::gpu::power::*;
    pub use crate::gpu::clock::*;
    pub use crate::gpu::cooler::*;
    pub use crate::gpu::thermal::*;
    pub use crate::gpu::pstate::*;
    pub use crate::gsync::*;
    pub use crate::i2c::*;
    pub use crate::driverapi::*;
    pub use crate::sysgeneral::*;
    pub use crate::vidio::*;
    pub use crate::nvapi::*;
    pub use self::private::*;

    pub mod private {
        pub use crate::gpu::private::*;
        pub use crate::gpu::power::private::*;
        pub use crate::gpu::clock::private::*;
        pub use crate::gpu::cooler::private::*;
        pub use crate::gpu::thermal::private::*;
        pub use crate::gpu::pstate::private::*;
        pub use crate::driverapi::private::*;
        pub use crate::i2c::private::*;
    }
}

pub(crate) mod prelude_ {
    pub(crate) use crate::nvapi::*;
    pub(crate) use crate::handles::{self, NvPhysicalGpuHandle};
    pub(crate) use crate::status::NvAPI_Status;
    pub(crate) use crate::version::{StructVersion, StructVersionInfo, VersionedStructField};
    pub(crate) use crate::{Array, BoolU32, NvVersion, ClockMask};
    pub(crate) type Padding<T> = Array<T>;
}

/// The result of a fallible NVAPI call.
pub type Result<T> = result::Result<T, Status>;

/// Error type indicating a raw value is out of the range of known enum values.
#[derive(Debug, Copy, Clone, Default)]
pub struct ArgumentRangeError;

impl fmt::Display for ArgumentRangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("received data out of range")
    }
}

impl StdError for ArgumentRangeError { }

impl From<ArgumentRangeError> for Status {
    fn from(_: ArgumentRangeError) -> Self {
        Status::ArgumentExceedMaxSize
    }
}

impl From<Infallible> for ArgumentRangeError {
    fn from(e: Infallible) -> Self {
        match e { }
    }
}
