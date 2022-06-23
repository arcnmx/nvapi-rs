use crate::sys;
use log::trace;
use crate::types::RawConversion;

pub fn driver_version() -> sys::Result<(u32, String)> {
    trace!("driver_version()");
    let mut str = sys::types::short_string();
    let mut version = 0;
    unsafe {
        sys::status_result(sys::driverapi::NvAPI_SYS_GetDriverAndBranchVersion(&mut version, &mut str))
            .and_then(move |_| str.convert_raw().map_err(Into::into).map(|str| (version, str)))
    }
}

pub fn interface_version() -> sys::Result<String> {
    trace!("interface_version()");
    let mut str = sys::types::short_string();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetInterfaceVersionString(&mut str))
            .and_then(move |_| str.convert_raw().map_err(Into::into))
    }
}

pub fn error_message(status: sys::Status) -> sys::Result<String> {
    trace!("error_message({:?})", status);
    let mut str = sys::types::short_string();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetErrorMessage(status.raw(), &mut str))
            .and_then(move |_| str.convert_raw().map_err(Into::into))
    }
}

pub fn initialize() -> sys::Result<()> {
    trace!("initialize()");
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_Initialize())
    }
}

pub fn unload() -> sys::Result<()> {
    trace!("unload()");
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_Unload())
    }
}
