use crate::sys::{gsync, handles::NvGSyncDeviceHandle};
use crate::types::NvData;
use crate::error::NvapiResultExt;
use crate::{PhysicalGpu, NvapiResult, Api};

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

    pub fn enumerate() -> NvapiResult<Vec<Self>> {
        NvGSyncDeviceHandle::EnumSyncDevices()
            .with_api(Api::NvAPI_GSync_EnumSyncDevices)
            .map(|handles| handles.into_iter().map(GSyncDevice::with_handle).collect())
    }

    pub fn sync_status(&self, gpu: &PhysicalGpu) -> NvapiResult<GSyncStatus> {
        self.handle().GetSyncStatus(*gpu.handle())
            .with_api(Api::NvAPI_GSync_GetSyncStatus)
            .map(GSyncStatusV1::from)
            .map(Into::into)
    }

    pub fn capabilities(&self) -> NvapiResult<GSyncCapabilities> {
        let res = self.handle().QueryCapabilities::<3, _>()
            .with_api(Api::NvAPI_GSync_QueryCapabilities)
            .map(GSyncCapabilitiesV3::from)
            .map(Into::into);
        allow_version_compat!(try res);

        let res = self.handle().QueryCapabilities::<2, _>()
            .with_api(Api::NvAPI_GSync_QueryCapabilities)
            .map(GSyncCapabilitiesV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().QueryCapabilities::<1, _>()
            .with_api(Api::NvAPI_GSync_QueryCapabilities)
            .map(GSyncCapabilitiesV1::from)
            .map(Into::into)
    }
}

nvwrap! {
    pub enum GSyncStatus {
        V1(GSyncStatusV1 {
            @type = NvData<gsync::NV_GSYNC_STATUS> {
                pub synced: bool {
                    @sys@BoolU32(bIsSynced),
                },
                pub stereo_synced: bool {
                    @sys@BoolU32(bIsStereoSynced),
                },
                pub sync_signal_available: bool {
                    @sys@BoolU32(bIsSyncSignalAvailable),
                },
            },
        }),
    }

    impl @StructVersion for GSyncStatus { }

    impl GSyncStatus {
        pub fn synced(&self) -> bool;
        pub fn stereo_synced(&self) -> bool;
        pub fn sync_signal_available(&self) -> bool;
    }
}

nvwrap! {
    pub enum GSyncCapabilities {
        V1(GSyncCapabilitiesV1 {
            @type = NvData<gsync::NV_GSYNC_CAPABILITIES_V1> {
                pub board_id: u32 {
                    @sys(boardId),
                },
                pub revision: u32 {
                    @sys(revision),
                },
            },
        }),
        V2(GSyncCapabilitiesV2),
        V3(GSyncCapabilitiesV3),
    }

    impl @StructVersion for GSyncCapabilities { }
    impl @Deref(GSyncCapabilitiesV1) for GSyncCapabilities { }
}

nvwrap! {
    pub type GSyncCapabilitiesV2 = NvData<gsync::NV_GSYNC_CAPABILITIES_V2> {
        pub extended_revision: u32 {
            @sys(extendedRevision),
        },
    };

    impl @Deref(v1: GSyncCapabilitiesV1) for GSyncCapabilitiesV2 { }
}

nvwrap! {
    pub type GSyncCapabilitiesV3 = NvData<gsync::NV_GSYNC_CAPABILITIES_V3> {
        pub max_mul_div: Option<u32> {
            @get fn(self) {
                self.sys().maxMulDiv()
            },
        },
    };

    impl @Deref(v2: GSyncCapabilitiesV2) for GSyncCapabilitiesV3 { }
}

impl GSyncCapabilities {
    pub fn extended_revision(&self) -> u32 {
        match self {
            Self::V2(caps) => caps.extended_revision(),
            Self::V3(caps) => caps.extended_revision(),
            Self::V1(..) => 0,
        }
    }

    pub fn max_mul_div(&self) -> Option<u32> {
        match self {
            Self::V3(caps) => caps.max_mul_div(),
            Self::V2(..) | Self::V1(..) => None,
        }
    }
}
