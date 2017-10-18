use std::ffi::CStr;
use sys;

pub fn driver_version() -> sys::Result<(u32, String)> {
    let mut str = [0; sys::NVAPI_SHORT_STRING_MAX];
    let mut version = 0;
    unsafe {
        sys::status_result(sys::driverapi::NvAPI_SYS_GetDriverAndBranchVersion(&mut version, &mut str))
            .map(move |_| (version, CStr::from_ptr(str.as_ptr()).to_string_lossy().into_owned()))
    }
}
