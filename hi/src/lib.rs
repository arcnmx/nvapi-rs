#![doc(html_root_url = "https://docs.rs/nvapi-hi/0.2.0")]

pub use nvapi;

mod gpu;
pub use gpu::*;

pub use nvapi::{
    Status, Result, Error, NvapiError,
    sys,
    initialize, unload, chipset_info, driver_version, interface_version
};

use std::result::Result as StdResult;

fn allowable_result_fallback<T, E: Into<Error>>(v: StdResult<T, E>, fallback: T) -> Result<T> {
    allowable_result(v).map(|res| res.unwrap_or(fallback))
}

fn allowable_result<T, E: Into<Error>>(v: StdResult<T, E>) -> Result<Option<T>> {
    Error::allowable_result(v)
}
