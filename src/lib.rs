//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/nvapi-rs/")]

pub extern crate nvapi_sys as sys;
extern crate void;
#[cfg(feature = "serde_derive")]
#[macro_use]
extern crate serde_derive;

mod types;
mod pstate;
mod clock;
mod thermal;
mod gpu;
mod info;

pub use types::*;
pub use pstate::*;
pub use clock::*;
pub use thermal::*;
pub use gpu::*;
pub use info::*;

pub use sys::{Status, Result};
