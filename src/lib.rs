//#![deny(missing_docs)]
#![doc(html_root_url = "http://arcnmx.github.io/nvapi-rs/")]

extern crate nvapi_sys as sys;
extern crate void;

pub mod types;
pub mod pstate;
pub mod clock;
pub mod thermal;
pub mod gpu;
pub mod info;
