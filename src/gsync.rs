use std::convert::Infallible;
use log::trace;
use crate::sys::{self, gsync, handles::NvGSyncDeviceHandle};
use crate::types::RawConversion;
use crate::PhysicalGpu;

#[derive(Debug)]
pub struct GSyncDevice {
    handle: NvGSyncDeviceHandle,
}

impl GSyncDevice {
    pub fn with_handle(handle: NvGSyncDeviceHandle) -> Self {
        Self {
            handle,
        }
    }

    pub fn handle(&self) -> &NvGSyncDeviceHandle {
        &self.handle
    }

    pub fn enumerate() -> crate::NvapiResult<Vec<Self>> {
        trace!("gsync.enumerate()");
        let mut handles = [Default::default(); gsync::NVAPI_MAX_GSYNC_DEVICES];
        match unsafe { nvcall!(NvAPI_GSync_EnumSyncDevices@get(&mut handles)) } {
            Err(crate::NvapiError { status: crate::Status::NvidiaDeviceNotFound, .. }) => Ok(Vec::new()),
            Ok(len) => Ok(handles[..len as usize].iter().cloned().map(GSyncDevice::with_handle).collect()),
            Err(e) => Err(e),
        }
    }

    pub fn sync_status(&self, gpu: &PhysicalGpu) -> crate::NvapiResult<<gsync::NV_GSYNC_STATUS as RawConversion>::Target> {
        trace!("gsync.sync_status()");
        unsafe {
            nvcall!(NvAPI_GSync_GetSyncStatus@get(*self.handle(), *gpu.handle()) => raw)
        }
    }

    pub fn capabilities(&self) -> crate::NvapiResult<<gsync::NV_GSYNC_CAPABILITIES as RawConversion>::Target> {
        trace!("gsync.capabilities()");
        unsafe {
            nvcall!(NvAPI_GSync_QueryCapabilities@get(*self.handle()) => raw)
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct GSyncStatus {
    pub synced: bool,
    pub stereo_synced: bool,
    pub sync_signal_available: bool,
}

impl RawConversion for gsync::NV_GSYNC_STATUS {
    type Target = GSyncStatus;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(GSyncStatus {
            synced: self.bIsSynced.get(),
            stereo_synced: self.bIsStereoSynced.get(),
            sync_signal_available: self.bIsSyncSignalAvailable.get(),
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Default, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct GSyncCapabilities {
    pub board_id: u32,
    pub revision: u32,
    pub extended_revision: u32,
    pub max_mul_div: Option<u32>,
}

impl RawConversion for gsync::NV_GSYNC_CAPABILITIES_V1 {
    type Target = GSyncCapabilities;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(GSyncCapabilities {
            board_id: self.boardId,
            revision: self.revision,
            .. Default::default()
        })
    }
}

impl RawConversion for gsync::NV_GSYNC_CAPABILITIES_V2 {
    type Target = GSyncCapabilities;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        self.v1.convert_raw().map(|v1| GSyncCapabilities {
            extended_revision: self.extendedRevision,
            .. v1
        })
    }
}

impl RawConversion for gsync::NV_GSYNC_CAPABILITIES_V3 {
    type Target = GSyncCapabilities;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        self.v2.convert_raw().map(|v2| GSyncCapabilities {
            max_mul_div: self.maxMulDiv(),
            .. v2
        })
    }
}
