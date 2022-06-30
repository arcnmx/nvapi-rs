#![allow(non_camel_case_types, non_snake_case)]
#![doc(html_root_url = "http://docs.rs/nvapi-sys/0.2.0")]

#[macro_use]
mod macros;

pub mod nvid;
pub mod nvapi;
pub mod status;
pub mod types;

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

/// The GPU APIs retrieve and control various attributes of the GPU, such as outputs, VBIOS revision, APG rate, frame buffer size, and thermal settings.
pub mod gpu;

/// I2C API - Provides ability to read or write data using I2C protocol.
/// These APIs allow I2C access only to DDC monitors
pub mod i2c;

#[cfg(windows)]
pub mod dx;

pub mod dispcontrol;

pub use nvid::Api;
pub use nvapi::nvapi_QueryInterface;
pub use types::*;
pub use status::{NvAPI_Status, Status};

use std::error::Error as StdError;
use std::{result, fmt};
use std::convert::Infallible;

pub mod api {
    pub use crate::handles::*;
    #[cfg(windows)]
    pub use crate::dx::*;
    pub use crate::gpu::*;
    pub use crate::gpu::private::*;
    pub use crate::gpu::display::*;
    pub use crate::gpu::ecc::*;
    pub use crate::gpu::power::*;
    pub use crate::gpu::power::private::*;
    pub use crate::gpu::clock::*;
    pub use crate::gpu::clock::private::*;
    pub use crate::gpu::cooler::*;
    pub use crate::gpu::cooler::private::*;
    pub use crate::gpu::thermal::*;
    pub use crate::gpu::thermal::private::*;
    pub use crate::gpu::pstate::*;
    pub use crate::gpu::pstate::private::*;
    pub use crate::i2c::*;
    pub use crate::i2c::private::*;
    pub use crate::driverapi::*;
    pub use crate::nvapi::*;
}

/// The result of a fallible NVAPI call.
pub type Result<T> = result::Result<T, Status>;

/// Treat `NVAPI_OK` as `Ok(())` and all else as an `Err(..)`.
pub fn status_result(status: NvAPI_Status) -> Result<()> {
    match status {
        status::NVAPI_OK => Ok(()),
        status => Err(Status::from_raw(status).unwrap_or(Status::Error)),
    }
}

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

// TODO: NvAPI_SYS_GetChipSetInfo
