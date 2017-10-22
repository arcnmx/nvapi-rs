use sys;
use void::ResultVoidExt;
use types::RawConversion;

pub fn driver_version() -> sys::Result<(u32, String)> {
    let mut str = sys::types::short_string();
    let mut version = 0;
    unsafe {
        sys::status_result(sys::driverapi::NvAPI_SYS_GetDriverAndBranchVersion(&mut version, &mut str))
            .map(move |_| (version, str.convert_raw().void_unwrap()))
    }
}

pub fn interface_version() -> sys::Result<String> {
    let mut str = sys::types::short_string();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetInterfaceVersionString(&mut str))
            .map(move |_| str.convert_raw().void_unwrap())
    }
}

pub fn error_message(status: sys::Status) -> sys::Result<String> {
    let mut str = sys::types::short_string();
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_GetErrorMessage(status.raw(), &mut str))
            .map(move |_| str.convert_raw().void_unwrap())
    }
}

pub fn initialize() -> sys::Result<()> {
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_Initialize())
    }
}

pub fn unload() -> sys::Result<()> {
    unsafe {
        sys::status_result(sys::nvapi::NvAPI_Unload())
    }
}
