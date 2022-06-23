//#![deny(missing_docs)]
#![doc(html_root_url = "http://docs.rs/nvapi/0.1.3")]

pub extern crate nvapi_sys as sys;
extern crate void;
#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
#[cfg(feature = "i2c")]
extern crate i2c;

mod types;
mod pstate;
mod clock;
mod thermal;
mod gpu;
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
