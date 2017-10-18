#![allow(non_camel_case_types, non_snake_case)]
#![doc(html_root_url = "http://arcnmx.github.io/nvapi-rs/")]

#[cfg(windows)]
extern crate winapi;
#[cfg(all(windows, not(feature = "winapi3")))]
extern crate kernel32;

#[macro_use]
mod macros;

#[macro_use]
mod debug_array;
pub use debug_array::Array;

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

#[cfg(windows)]
pub mod dx;

pub mod dispcontrol;

pub use nvid::Api;
pub use nvapi::nvapi_QueryInterface;
pub use types::*;
pub use status::{NvAPI_Status, Status};

use std::result;

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

impl From<ArgumentRangeError> for Status {
    fn from(_: ArgumentRangeError) -> Self {
        Status::InvalidArgument
    }
}

// TODO: NvAPI_SYS_GetChipSetInfo
