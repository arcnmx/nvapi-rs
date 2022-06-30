#![doc(html_root_url = "http://docs.rs/nvapi-hi/0.2.0")]

pub use nvapi;

mod gpu;
pub use gpu::*;

pub use nvapi::{
    Status, Result, Error, NvapiError,
    sys,
    initialize, unload, chipset_info, driver_version, interface_version, error_message
};

use std::result::Result as StdResult;

pub fn allowable_result_fallback<T, E: Into<Error>>(v: StdResult<T, E>, fallback: T) -> Result<T> {
    match v.map_err(Into::into) {
        Ok(v) => Ok(v),
        Err(Error::Nvapi(NvapiError { status: Status::NotSupported, .. }))
        | Err(Error::Nvapi(NvapiError { status: Status::NoImplementation, .. }))
        | Err(Error::Nvapi(NvapiError { status: Status::ArgumentExceedMaxSize, .. }))
        | Err(Error::ArgumentRange(..))
        => Ok(fallback),
        Err(e) => Err(e),
    }
}

pub fn allowable_result<T, E: Into<Error>>(v: StdResult<T, E>) -> Result<Result<T>> {
    match v.map_err(Into::into) {
        Ok(v) => Ok(Ok(v)),
        Err(e @ Error::Nvapi(NvapiError { status: Status::NotSupported, .. }))
        | Err(e @ Error::Nvapi(NvapiError { status: Status::NoImplementation, .. }))
        | Err(e @ Error::ArgumentRange(..))
        => Ok(Err(e)),
        Err(e) => Err(e),
    }
}
