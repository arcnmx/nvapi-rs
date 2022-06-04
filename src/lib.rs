//#![deny(missing_docs)]
#![doc(html_root_url = "https://docs.rs/nvapi/0.2.0")]

pub use nvapi_sys as sys;

#[macro_use]
mod macros;
mod error;
mod types;
mod ecc;
mod pstate;
mod clock;
mod thermal;
mod gpu;
mod gsync;
mod info;
#[cfg(feature = "i2c")]
mod i2c_impl;

pub use error::*;
pub use types::*;
pub use ecc::*;
pub use pstate::*;
pub use clock::*;
pub use thermal::*;
pub use gpu::*;
pub use info::*;
pub use gsync::*;
#[cfg(feature = "i2c")]
pub use i2c_impl::*;

pub use sys::Status;
/// The result of a fallible NVAPI call.
pub type Result<T> = std::result::Result<T, Error>;
/// The result of a fallible NVAPI call.
pub type NvapiResult<T> = std::result::Result<T, NvapiError>;
