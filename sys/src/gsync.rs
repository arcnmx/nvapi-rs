use status::NvAPI_Status;
use handles::NvPhysicalGpuHandle;
use handles::NvGSyncDeviceHandle;

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V1 {
        version: u32,
        boardId: u32,
        revision: u32,
        capFlags: u32,
    }
}

const NV_GSYNC_CAPABILITIES_V1_SIZE: usize = 4 * 4;

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V2 {
        v1: NV_GSYNC_CAPABILITIES_V1,
        extendedRevision: u32,
    }
}

nvinherit! { NV_GSYNC_CAPABILITIES_V2(v1: NV_GSYNC_CAPABILITIES_V1) }

const NV_GSYNC_CAPABILITIES_V2_SIZE: usize = NV_GSYNC_CAPABILITIES_V1_SIZE + 4;

pub type NV_GSYNC_CAPABILITIES = NV_GSYNC_CAPABILITIES_V2;

nvversion! { NV_GSYNC_CAPABILITIES_VER_1(NV_GSYNC_CAPABILITIES_V1 = NV_GSYNC_CAPABILITIES_V1_SIZE, 1) }
nvversion! { NV_GSYNC_CAPABILITIES_VER_2(NV_GSYNC_CAPABILITIES_V2 = NV_GSYNC_CAPABILITIES_V2_SIZE, 2) }
nvversion! { NV_GSYNC_CAPABILITIES_VER = NV_GSYNC_CAPABILITIES_VER_2 }

nvenum! {
    pub enum NVAPI_GSYNC_DISPLAY_SYNC_STATE / DisplaySyncState {
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_UNSYNCED / Unsynced = 0,	
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_SLAVE / Slave = 1,	
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_MASTER / Master = 2,
    }
}

nvstruct! {
    pub struct NV_GSYNC_DISPLAY {
        version: u32,
        displayId: u32,
        isMasterable: u32,
        reserved: u32,
 	    syncState: NVAPI_GSYNC_DISPLAY_SYNC_STATE,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR / TopologyConnector {
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_NONE / None = 0,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_PRIMARY / Primary = 1,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_SECONDARY / Secondary = 2,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_TERTIARY / Tertiary = 3,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_QUARTERNARY / Quarternary = 4,
    }
}

nvstruct! {
    pub struct NV_GSYNC_GPU {
        version: u32,
        hPhysicalGpu: NvPhysicalGpuHandle,
        connector: NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR,
        hProxyPhysicalGpu: NvPhysicalGpuHandle,
        isSynced: u32,
        reserved: u32,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_POLARITY / Polarity {
        NVAPI_GSYNC_POLARITY_RISING_EDGE / RisingEdge = 0, 
        NVAPI_GSYNC_POLARITY_FALLING_EDGE / FallingEdge = 1,	
        NVAPI_GSYNC_POLARITY_BOTH_EDGES / BothEdges = 2,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_VIDEO_MODE / VideoMode {
        NVAPI_GSYNC_VIDEO_MODE_NONE / None = 0,
        NVAPI_GSYNC_VIDEO_MODE_TTL / TTL = 1,
        NVAPI_GSYNC_VIDEO_MODE_NTSCPALSECAM / NtscPalCam = 2,
        NVAPI_GSYNC_VIDEO_MODE_HDTV / Hdtv = 3,
        NVAPI_GSYNC_VIDEO_MODE_COMPOSITE / Composite = 4,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_SYNC_SOURCE / SyncSource {
        NVAPI_GSYNC_SYNC_SOURCE_VSYNC / VSync = 0,
        NVAPI_GSYNC_SYNC_SOURCE_HOUSESYNC / HouseSync = 1,
    }
}

nvstruct! {
    pub struct NV_GSYNC_DELAY {
        version: u32,
        numLines: u32,
        numPixels: u32,
        maxLines: u32,
        minPixels: u32,
    }
}

nvstruct! {
    pub struct NV_GSYNC_CONTROL_PARAMS {
        version: u32,
        polarity: NVAPI_GSYNC_POLARITY,
        vmode: NVAPI_GSYNC_VIDEO_MODE,
        interval: u32,
        source: NVAPI_GSYNC_SYNC_SOURCE,
        interlaceMode: u32,
        syncSourceIsOutput: u32,
        reserved: u32,
        syncSkew: NV_GSYNC_DELAY,
        startupDelay: NV_GSYNC_DELAY,
    }
}

nvenum! {
    pub enum NVAPI_GSYNC_DELAY_TYPE / DelayType {
        NVAPI_GSYNC_DELAY_TYPE_UNKNOWN / Unknown = 0,
        NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW / SyncSkew = 1,
        NVAPI_GSYNC_DELAY_TYPE_STARTUP / Startup = 2,
    }
}

nvstruct! {
    #[derive(Debug, Default)]
    pub struct NV_GSYNC_STATUS {
        pub version: u32,
        pub bIsSynced: u32,
        pub bIsStereoSynced: u32,
        pub bIsSyncSignalAvailable: u32,
    }
}

// TODO: this most likely wont work and needs to be updated.
nvversion! { NV_GSYNC_STATUS_VER(NV_GSYNC_STATUS = 0, 1) /* temp */}

nvenum! {
    pub enum NVAPI_GSYNC_RJ45_IO / RJ45_IO {
        NVAPI_GSYNC_RJ45_OUTPUT / Output = 0,
        NVAPI_GSYNC_RJ45_INPUT 	/ Input = 1,
        NVAPI_GSYNC_RJ45_UNUSED / Unused = 2,
    }
}

pub const NVAPI_MAX_RJ45_PER_GSYNC: usize = 2;

nvstruct! {
    pub struct NV_GSYNC_STATUS_PARAMS_V1 {
        version: u32,
     	refreshRate: u32,
     	RJ45_IO: [NVAPI_GSYNC_RJ45_IO; NVAPI_MAX_RJ45_PER_GSYNC],
     	RJ45_Ethernet: [u32; NVAPI_MAX_RJ45_PER_GSYNC],
     	houseSyncIncoming: u32,
     	bHouseSync: u32,
    }
}

const NV_GSYNC_STATUS_PARAMS_V1_SIZE: usize = std::mem::size_of::<NV_GSYNC_STATUS_PARAMS_V1>();

nvstruct! {
    pub struct NV_GSYNC_STATUS_PARAMS_V2 {
        v1: NV_GSYNC_STATUS_PARAMS_V1,
        bInternalSlave: u32,
        reserved: u32,
    }
}

nvinherit! { NV_GSYNC_STATUS_PARAMS_V2(v1: NV_GSYNC_STATUS_PARAMS_V1) }

const NV_GSYNC_STATUS_PARAMS_V2_SIZE: usize = std::mem::size_of::<NV_GSYNC_STATUS_PARAMS_V2>();

pub type NV_GSYNC_STATUS_PARAMS = NV_GSYNC_STATUS_PARAMS_V2;

nvversion! { NV_GSYNC_STATUS_PARAMS_VER_1(NV_GSYNC_STATUS_PARAMS_V1 = NV_GSYNC_STATUS_PARAMS_V1_SIZE, 1) }
nvversion! { NV_GSYNC_STATUS_PARAMS_VER_2(NV_GSYNC_STATUS_PARAMS_V2 = NV_GSYNC_STATUS_PARAMS_V2_SIZE, 2) }
nvversion! { NV_GSYNC_STATUS_PARAMS_VER = NV_GSYNC_STATUS_PARAMS_VER_2 }

nvapi! {
    pub type GSync_EnumSyncDevicesFn = extern "C" fn(nvGSyncHandles: *mut [NvGSyncDeviceHandle; super::types::NVAPI_MAX_GSYNC_DEVICES], gsyncCount: *mut u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_EnumSyncDevices;
}

nvapi! {
    pub type GSync_QueryCapabilitiesFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pNvGSyncCapabilities: *mut NV_GSYNC_CAPABILITIES) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_QueryCapabilities;
}

nvapi! {
    pub type GSync_GetTopologyFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, gsyncGpuCount: *mut u32, gsyncGPUs: *mut NV_GSYNC_GPU, gsyncDisplayCount: u32, gsyncDisplays: *mut NV_GSYNC_DISPLAY) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetTopology;
}

nvapi! {
    pub type GSync_SetSyncStateSettingsFn = extern "C" fn(gsyncDisplayCount: u32, pGsyncDisplays: NV_GSYNC_DISPLAY, flags: u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_SetSyncStateSettings;
}

nvapi! {
    pub type GSync_GetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetControlParameters;
}

nvapi! {
    pub type GSync_SetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_SetControlParameters;
}

nvapi! {
    // Parameter should be pointer?
    pub type GSync_AdjustSyncDelayFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, delayType: NVAPI_GSYNC_DELAY_TYPE, pGsyncDelay: *mut NV_GSYNC_DELAY, syncSteps: *mut u32) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_AdjustSyncDelay;
}

nvapi! {
    pub type GSync_GetSyncStatusFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, hPhysicalGpu: NvPhysicalGpuHandle, status: *mut NV_GSYNC_STATUS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetSyncStatus;
}

nvapi! {
    pub type GSync_GetStatusParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pStatusParams: *mut NV_GSYNC_STATUS_PARAMS) -> NvAPI_Status;
    pub unsafe fn NvAPI_GSync_GetStatusParameters;
}