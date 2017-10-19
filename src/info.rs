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
