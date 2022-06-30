use crate::nvapi::NvVersion;
use crate::status::NvAPI_Status;
use crate::types::NvAPI_ShortString;
use crate::handles;

nvstruct! {
    pub struct NV_CHIPSET_INFO_v1 {
        /// structure version
        pub version: NvVersion,
        /// vendor ID
        pub vendorId: u32,
        /// device ID
        pub deviceId: u32,
        /// vendor Name
        pub szVendorName: NvAPI_ShortString,
        /// device Name
        pub szChipsetName: NvAPI_ShortString,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v2 {
        pub v1: NV_CHIPSET_INFO_v1,
        /// Chipset info flags - obsolete
        #[deprecated]
        pub flags: u32,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v3 {
        pub v2: NV_CHIPSET_INFO_v2,
        /// subsystem vendor ID
        pub subSysVendorId: u32,
        /// subsystem device ID
        pub subSysDeviceId: u32,
        /// subsystem vendor Name
        pub szSubSysVendorName: NvAPI_ShortString,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v4 {
        pub v3: NV_CHIPSET_INFO_v3,
        /// Host bridge vendor identification
        pub HBvendorId: u32,
        /// Host bridge device identification
        pub HBdeviceId: u32,
        /// Host bridge subsystem vendor identification
        pub HBsubSysVendorId: u32,
        /// Host bridge subsystem device identification
        pub HBsubSysDeviceId: u32,
    }
}

nvinherit! { NV_CHIPSET_INFO_v2(v1: NV_CHIPSET_INFO_v1) }
nvinherit! { NV_CHIPSET_INFO_v3(v2: NV_CHIPSET_INFO_v2) }
nvinherit! { NV_CHIPSET_INFO_v4(v3: NV_CHIPSET_INFO_v3) }

nvversion! { NV_CHIPSET_INFO_VER_1(NV_CHIPSET_INFO_v1 = 4*3+crate::NVAPI_SHORT_STRING_MAX*2, 1) }
nvversion! { NV_CHIPSET_INFO_VER_2(NV_CHIPSET_INFO_v2 = 4*4+crate::NVAPI_SHORT_STRING_MAX*2, 2) }
nvversion! { NV_CHIPSET_INFO_VER_3(NV_CHIPSET_INFO_v3 = 4*6+crate::NVAPI_SHORT_STRING_MAX*3, 3) }
nvversion! { NV_CHIPSET_INFO_VER_4(NV_CHIPSET_INFO_v4 = 4*10+crate::NVAPI_SHORT_STRING_MAX*3, 4) }
nvversion! { NV_CHIPSET_INFO_VER = NV_CHIPSET_INFO_VER_4 }

pub type NV_CHIPSET_INFO = NV_CHIPSET_INFO_v4;

nvapi! {
    pub type SYS_GetChipSetInfoFn = extern "C" fn(pChipSetInfo: *mut NV_CHIPSET_INFO) -> NvAPI_Status;

    /// This API returns display driver version and driver-branch string.
    pub unsafe fn NvAPI_SYS_GetChipSetInfo;
}
