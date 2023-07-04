#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use crate::sys;
use crate::types::RawConversion;
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
        nvcall!(NvAPI_GetErrorMessage@get(status.value()) => into)
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

pub fn chipset_info() -> crate::Result<<sys::sysgeneral::NV_CHIPSET_INFO as RawConversion>::Target> {
    trace!("gpu.chipset_info()");

    unsafe {
        nvcall!(NvAPI_SYS_GetChipSetInfo@get() => raw)
    }
}

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
}

impl RawConversion for sys::sysgeneral::NV_CHIPSET_INFO_v1 {
    type Target = ChipsetId;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(ChipsetId {
            vendor: self.vendorId,
            vendor_name: self.szVendorName.into(),
            device: self.deviceId,
            name: self.szChipsetName.into(),
        })
    }
}

impl RawConversion for sys::sysgeneral::NV_CHIPSET_INFO_v2 {
    type Target = ChipsetId;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        self.v1.convert_raw()
    }
}

impl RawConversion for sys::sysgeneral::NV_CHIPSET_INFO_v3 {
    type Target = ChipsetIds;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(ChipsetIds {
            system: self.v2.convert_raw()?,
            subsystem: ChipsetId {
                vendor: self.subSysVendorId,
                vendor_name: self.szSubSysVendorName.into(),
                device: self.subSysDeviceId,
                .. Default::default()
            },
        })
    }
}

impl RawConversion for sys::sysgeneral::NV_CHIPSET_INFO_v4 {
    type Target = ChipsetInfo;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(ChipsetInfo {
            chipset: self.v3.convert_raw()?,
            host_bridge: ChipsetIds {
                system: ChipsetId {
                    vendor: self.HBvendorId,
                    device: self.HBdeviceId,
                    .. Default::default()
                },
                subsystem: ChipsetId {
                    vendor: self.HBsubSysVendorId,
                    device: self.HBsubSysDeviceId,
                    .. Default::default()
                },
            },
        })
    }
}
