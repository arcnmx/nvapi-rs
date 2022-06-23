//#![deny(missing_docs)]
#![doc(html_root_url = "http://docs.rs/nvapi/0.2.0")]

pub use nvapi_sys as sys;

mod types;
mod pstate;
mod clock;
mod thermal;
mod gpu;
mod gsync;
mod info;
#[cfg(feature = "i2c")]
mod i2c_impl;

pub use types::*;
pub use pstate::*;
pub use clock::*;
pub use thermal::*;
pub use gpu::*;
pub use info::*;
#[cfg(feature = "i2c")]
pub use i2c_impl::*;

pub use sys::{Status, Result};
