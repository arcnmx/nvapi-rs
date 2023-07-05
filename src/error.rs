use std::error::Error as StdError;
use std::convert::Infallible;
use std::fmt;
use crate::Status;
use crate::sys::{Api, NvAPI_Status, ArgumentRangeError};

pub(crate) fn status_result(nvid: Api, status: NvAPI_Status) -> Result<(), NvapiError> {
    status.to_result()
        .map_err(|status| NvapiError::new(status, nvid))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Error {
    Nvapi(NvapiError),
    ArgumentRange(ArgumentRangeError),
}

impl Error {
    pub fn nvapi_status(&self) -> Option<Status> {
        match self {
            Error::Nvapi(e) => Some(e.status()),
            _ => None,
        }
    }

    pub const fn allow_version_incompat(self) -> Result<(), Self> {
        match self {
            Error::Nvapi(e) => match e.allow_version_incompat() {
                Ok(()) => Ok(()),
                Err(e) => Err(Error::Nvapi(e)),
            },
            e @ Error::ArgumentRange(..) => Err(e),
        }
    }

    pub fn allowable_result<T, E: Into<Self>>(v: Result<T, E>) -> Result<Option<T>, Self> {
        let res = v.map_err(Into::into).map_err(|e| match e.allow_version_incompat() {
            Err(Error::ArgumentRange(..)) => Ok(()),
            Err(Error::Nvapi(e)) if e.status() == ArgumentRangeError.to_status() => Ok(()),
            res => res,
        });
        match res {
            Ok(v) => Ok(Some(v)),
            Err(Ok(())) => Ok(None),
            Err(Err(e)) => Err(e.into()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NvapiError {
    pub status: NvAPI_Status,
    pub nvid: Api,
}

impl NvapiError {
    pub const fn new(status: NvAPI_Status, nvid: Api) -> Self {
        Self {
            status,
            nvid,
        }
    }

    pub const fn new_result(status: NvAPI_Status, nvid: Api) -> Result<(), Self> {
        match status.to_result() {
            Err(status) => Err(Self::new(status, nvid)),
            Ok(()) => Ok(()),
        }
    }

    pub const fn allow_version_incompat(self) -> Result<(), Self> {
        match self {
            Self { status: NvAPI_Status::IncompatibleStructVersion | NvAPI_Status::NoImplementation | NvAPI_Status::NotSupported, .. } => Ok(()),
            err => Err(err),
        }
    }

    pub const fn to_result<T>(self) -> Result<(), Self> {
        Err(self)
    }

    pub fn status(&self) -> Status {
        self.status.to_status()
    }
}

impl StdError for NvapiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.status as _)
    }
}

impl fmt::Display for NvapiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} failed: {}", self.nvid, self.status)
    }
}

impl From<NvapiError> for NvAPI_Status {
    fn from(e: NvapiError) -> Self {
        e.status
    }
}

impl<T> From<NvapiError> for Result<T, NvapiError> {
    fn from(e: NvapiError) -> Self {
        Err(e)
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
