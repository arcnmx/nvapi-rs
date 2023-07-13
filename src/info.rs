#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use crate::{types::NvData, sys};
use sys::nvid as nvapi;
use sys::NvAPI_ShortString;
use crate::{Api, NvapiResult, NvapiResultExt};

pub fn driver_version() -> NvapiResult<(u32, String)> {
    nvapi::NvAPI_SYS_GetDriverAndBranchVersion.call()
        .with_api(Api::NvAPI_SYS_GetDriverAndBranchVersion)
        .map(|(version, str)| (version, str.into()))
}

pub fn interface_version() -> NvapiResult<String> {
    nvapi::NvAPI_GetInterfaceVersionString.call()
        .with_api(Api::NvAPI_GetInterfaceVersionString)
        .map(Into::into)
}

pub fn initialize() -> NvapiResult<()> {
    nvapi::NvAPI_Initialize.call()
        .with_api(Api::NvAPI_Initialize)
}

pub fn unload() -> NvapiResult<()> {
    nvapi::NvAPI_Unload.call()
        .with_api(Api::NvAPI_Unload)
}

pub fn chipset_info() -> NvapiResult<ChipsetInfo> {
    let res = nvapi::NvAPI_SYS_GetChipSetInfo.call::<4, ChipsetInfoV4>()
        .with_api(Api::NvAPI_SYS_GetChipSetInfo)
        .map(Into::into);
    allow_version_compat!(try res);
    let res = nvapi::NvAPI_SYS_GetChipSetInfo.call::<3, ChipsetInfoV3>()
        .with_api(Api::NvAPI_SYS_GetChipSetInfo)
        .map(Into::into);
    allow_version_compat!(try res);
    let res = nvapi::NvAPI_SYS_GetChipSetInfo.call::<2, ChipsetInfoV2>()
        .with_api(Api::NvAPI_SYS_GetChipSetInfo)
        .map(Into::into);
    allow_version_compat!(try res);
    nvapi::NvAPI_SYS_GetChipSetInfo.call::<1, ChipsetInfoV1>()
        .with_api(Api::NvAPI_SYS_GetChipSetInfo)
        .map(Into::into)
}

/*
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ChipsetId {
    pub vendor: u32,
    pub device: u32,
    pub vendor_name: String,
    pub name: String,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ChipsetIds {
    pub system: ChipsetId,
    pub subsystem: ChipsetId,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ChipsetInfo {
    pub chipset: ChipsetIds,
    pub host_bridge: ChipsetIds,
}*/

nvwrap! {
    pub type ChipsetInfoV1 = NvData<sys::sysgeneral::NV_CHIPSET_INFO_v1> {
        pub vendor: u32 {
            @sys(vendorId),
        },
        pub device: u32 {
            @sys(deviceId),
        },
        pub vendor_name: NvAPI_ShortString {
            @sys(szVendorName),
        },
        pub name: NvAPI_ShortString {
            @sys(szChipsetName),
        },
    };
}

nvwrap! {
    pub type ChipsetInfoV2 = NvData<sys::sysgeneral::NV_CHIPSET_INFO_v2> {
        /*
        #[deprecated]
        pub flags: u32 {
            @sys,
        },*/
    };

    impl @Deref(v1: ChipsetInfoV1) for ChipsetInfoV2 { }
}

nvwrap! {
    pub type ChipsetInfoV3 = NvData<sys::sysgeneral::NV_CHIPSET_INFO_v3> {
        pub subsystem_vendor: u32 {
            @sys(subSysVendorId),
        },
        pub subsystem_device: u32 {
            @sys(subSysDeviceId),
        },
        pub subsystem_vendor_name: NvAPI_ShortString {
            @sys(szSubSysVendorName),
        },
    };

    impl @Deref(v2: ChipsetInfoV2) for ChipsetInfoV3 { }
}

nvwrap! {
    pub type ChipsetInfoV4 = NvData<sys::sysgeneral::NV_CHIPSET_INFO_v4> {
        pub host_bridge_vendor: u32 {
            @sys(HBvendorId),
        },
        pub host_bridge_device: u32 {
            @sys(HBdeviceId),
        },
        pub host_bridge_subsystem_vendor: u32 {
            @sys(HBsubSysVendorId),
        },
        pub host_bridge_subsystem_device: u32 {
            @sys(HBsubSysDeviceId),
        },
    };

    impl @Deref(v3: ChipsetInfoV3) for ChipsetInfoV4 { }
}

nvwrap! {
    pub enum ChipsetInfo {
        V4(ChipsetInfoV4),
        V3(ChipsetInfoV3),
        V2(ChipsetInfoV2),
        V1(ChipsetInfoV1),
    }

    impl @Deref(ChipsetInfoV1) for ChipsetInfo { }
}

/*impl<'a> From<&'a sys::sysgeneral::NV_CHIPSET_INFO_v1> for ChipsetId {
    fn from(info: &'a sys::sysgeneral::NV_CHIPSET_INFO_v1) -> Self {
        ChipsetId {
            vendor: info.vendorId,
            vendor_name: info.szVendorName.into(),
            device: info.deviceId,
            name: info.szChipsetName.into(),
        }
    }
}*/

/*impl<'a> From<&'a sys::sysgeneral::NV_CHIPSET_INFO_v2> for ChipsetId {
    fn from(info: &'a sys::sysgeneral::NV_CHIPSET_INFO_v2) -> Self {
        info.v1.into()
    }
}

nvconv! { sys::sysgeneral::NV_CHIPSET_INFO_v1 as ChipsetId | @From }
nvconv! { sys::sysgeneral::NV_CHIPSET_INFO_v2 as ChipsetId | @From }

impl<'a> From<&'a sys::sysgeneral::NV_CHIPSET_INFO_v3> for ChipsetIds {
    fn from(info: &'a sys::sysgeneral::NV_CHIPSET_INFO_v3) -> Self {
        ChipsetIds {
            system: info.v2.into(),
            subsystem: ChipsetId {
                vendor: info.subSysVendorId,
                vendor_name: info.szSubSysVendorName.into(),
                device: info.subSysDeviceId,
                .. Default::default()
            },
        }
    }
}

nvconv! { sys::sysgeneral::NV_CHIPSET_INFO_v3 as ChipsetIds | @From }

impl<'a> From<&'a sys::sysgeneral::NV_CHIPSET_INFO_v4> for ChipsetInfo {
    fn from(info: &'a sys::sysgeneral::NV_CHIPSET_INFO_v4) -> Self {
        ChipsetInfo {
            chipset: info.v3.into(),
            host_bridge: ChipsetIds {
                system: ChipsetId {
                    vendor: info.HBvendorId,
                    device: info.HBdeviceId,
                    .. Default::default()
                },
                subsystem: ChipsetId {
                    vendor: info.HBsubSysVendorId,
                    device: info.HBsubSysDeviceId,
                    .. Default::default()
                },
            },
        }
    }
}

nvconv! { sys::sysgeneral::NV_CHIPSET_INFO_v4 as ChipsetInfo | @From }
*/
