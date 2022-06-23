use std::error::Error as StdError;
use std::convert::Infallible;
use std::fmt;
use crate::{Status, sys};
use sys::ArgumentRangeError;

pub fn status_result(nvid: sys::Api, status: sys::NvAPI_Status) -> Result<(), NvapiError> {
    sys::status_result(status)
        .map_err(|status| NvapiError::new(nvid, status))
}

#[derive(Debug)]
pub enum Error {
    Nvapi(NvapiError),
    ArgumentRange(ArgumentRangeError),
}

impl Error {
    pub fn nvapi_status(&self) -> Option<Status> {
        match self {
            Error::Nvapi(e) => Some(e.status),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct NvapiError {
    pub nvid: sys::Api,
    pub status: Status,
}

impl NvapiError {
    pub fn new(nvid: sys::Api, status: Status) -> Self {
        Self {
            nvid,
            status,
        }
    }
}

impl StdError for NvapiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.status as _)
    }
}

impl fmt::Display for NvapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = crate::error_message(self.status)
            .unwrap_or_else(|_| format!("{:?}", self));
        write!(f, "{:?} failed: {}", self.nvid, msg)
    }
}

impl From<Infallible> for NvapiError {
    fn from(e: Infallible) -> Self {
        match e { }
    }
}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        match e { }
    }
}

impl From<NvapiError> for Error {
    fn from(e: NvapiError) -> Self {
        Error::Nvapi(e)
    }
}

impl From<ArgumentRangeError> for Error {
    fn from(e: ArgumentRangeError) -> Self {
        Error::ArgumentRange(e)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            Error::Nvapi(e) => e as _,
            Error::ArgumentRange(e) => e as _,
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Nvapi(e) => fmt::Display::fmt(e, f),
            Error::ArgumentRange(e) => fmt::Display::fmt(e, f),
        }
    }
}
