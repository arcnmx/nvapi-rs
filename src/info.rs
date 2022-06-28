use crate::sys;
use log::trace;

pub fn driver_version() -> sys::Result<(u32, String)> {
    trace!("driver_version()");
    let mut str = Default::default();
    let mut version = 0;
    unsafe {
        sys::status_result(sys::driverapi::NvAPI_SYS_GetDriverAndBranchVersion(&mut version, &mut str))
            .map(move |()| (version, str.into()))
    }
}

pub fn interface_version() -> sys::Result<String> {
    trace!("interface_version()");
    let mut str = Default::default();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetInterfaceVersionString(&mut str))
            .map(move |()| str.into())
    }
}

pub fn error_message(status: sys::Status) -> sys::Result<String> {
    trace!("error_message({:?})", status);
    let mut str = Default::default();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetErrorMessage(status.raw(), &mut str))
            .map(move |()| str.into())
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
