use crate::sys;
use log::trace;

pub fn driver_version() -> crate::NvapiResult<(u32, String)> {
    trace!("driver_version()");
    let mut version = 0;
    unsafe {
        nvcall!(NvAPI_SYS_GetDriverAndBranchVersion@get(&mut version))
            .map(|str| (version, str.into()))
    }
}

pub fn interface_version() -> crate::NvapiResult<String> {
    trace!("interface_version()");
    unsafe {
        nvcall!(NvAPI_GetInterfaceVersionString@get())
            .map(|str| str.into())
    }
}

pub fn error_message(status: sys::Status) -> crate::NvapiResult<String> {
    trace!("error_message({:?})", status);
    unsafe {
        nvcall!(NvAPI_GetErrorMessage@get(status.raw()) => into)
    }
}

pub fn initialize() -> crate::NvapiResult<()> {
    trace!("initialize()");
    unsafe {
        nvcall!(NvAPI_Initialize())
    }
}

pub fn unload() -> crate::NvapiResult<()> {
    trace!("unload()");
    unsafe {
        nvcall!(NvAPI_Unload())
    }
}
