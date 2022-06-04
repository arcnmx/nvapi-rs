use crate::prelude_::*;
use handles::{NvGSyncDeviceHandle, NvPhysicalGpuHandle};

pub const NVAPI_MAX_GSYNC_DEVICES: usize = 4;

nvapi! {
    pub type GSync_EnumSyncDevicesFn = extern "C" fn(nvGSyncHandles: *mut [NvGSyncDeviceHandle; NVAPI_MAX_GSYNC_DEVICES], gsyncCount: *mut u32) -> NvAPI_Status;

    /// This API returns an array of Sync device handles.
    ///
    /// A Sync device handle represents a single Sync device on the system.
    pub unsafe fn NvAPI_GSync_EnumSyncDevices;
}

/// GSync board ID 0x358
///
/// see [NV_GSYNC_CAPABILITIES]
pub const NVAPI_GSYNC_BOARD_ID_P358: u32 = 856;

/// GSync board ID 0x2060
///
/// see [NV_GSYNC_CAPABILITIES]
pub const NVAPI_GSYNC_BOARD_ID_P2060: u32 = 8288;

/// GSync board ID 0x2061
///
/// see [NV_GSYNC_CAPABILITIES]
pub const NVAPI_GSYNC_BOARD_ID_P2061: u32 = 8289;

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V1 {
        /// Version of the structure
        pub version: NvVersion,
        /// Board ID
        pub boardId: u32,
        /// FPGA Revision
        pub revision: u32,
        /// Capabilities of the Sync board.
        ///
        /// Reserved for future use
        pub capFlags: u32,
    }
}

nvstruct! {
    pub struct NV_GSYNC_CAPABILITIES_V2 {
        pub v1: NV_GSYNC_CAPABILITIES_V1,
        /// FPGA minor revision
        pub extendedRevision: u32,
    }
}

nvinherit! { NV_GSYNC_CAPABILITIES_V2(v1: NV_GSYNC_CAPABILITIES_V1) }

nvversion! { NV_GSYNC_CAPABILITIES_V1(1) }
nvversion! { @=NV_GSYNC_CAPABILITIES NV_GSYNC_CAPABILITIES_V2(2) }

nvapi! {
    pub type GSync_QueryCapabilitiesFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pNvGSyncCapabilities: *mut NV_GSYNC_CAPABILITIES) -> NvAPI_Status;

    /// This API returns the capabilities of the Sync device.
    pub unsafe fn NvAPI_GSync_QueryCapabilities;
}

nvenum! {
    /// Connector values for a GPU. Used in [NV_GSYNC_GPU].
    pub enum NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR / GSyncTopologyConnector {
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_NONE / None = 0,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_PRIMARY / Primary = 1,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_SECONDARY / Secondary = 2,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_TERTIARY / Tertiary = 3,
        NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR_QUARTERNARY / Quarternary = 4,
    }
}

nvenum! {
    /// Display sync states. Used in [NV_GSYNC_DISPLAY].
    pub enum NVAPI_GSYNC_DISPLAY_SYNC_STATE / GSyncDisplaySyncState {
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_UNSYNCED / Unsynced = 0,
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_SLAVE / Slave = 1,
        NVAPI_GSYNC_DISPLAY_SYNC_STATE_MASTER / Master = 2,
    }
}

nvstruct! {
    pub struct NV_GSYNC_GPU {
        /// Version of the structure
        pub version: NvVersion,
        /// GPU handle
        pub hPhysicalGpu: NvPhysicalGpuHandle,
        /// Indicates which connector on the device the GPU is connected to.
        pub connector: NVAPI_GSYNC_GPU_TOPOLOGY_CONNECTOR,
        /// GPU through which hPhysicalGpu is connected to the Sync device
        ///
        /// (if not directly connected) - this is NULL otherwise
        pub hProxyPhysicalGpu: NvPhysicalGpuHandle,
        /// Whether this GPU is sync'd or not.
        pub isSynced: BoolU32,
    }
}

nvversion! { @NV_GSYNC_GPU(1) }

nvstruct! {
    pub struct NV_GSYNC_DISPLAY {
        /// Version of the structure
        pub version: NvVersion,
        /// display identifier for displays.
        ///
        /// The GPU to which it is connected, can be retireved from [NvAPI_SYS_GetPhysicalGpuFromDisplayId]
        pub displayId: u32,
        /// Can this display be the master? (Read only)
        pub isMasterable: BoolU32,
        /// Is this display slave/master
        ///
        /// Retrieved with topology or set by caller for enable/disable sync
        pub syncState: NVAPI_GSYNC_DISPLAY_SYNC_STATE,
    }
}

nvversion! { @NV_GSYNC_DISPLAY(1) }

nvapi! {
    pub type GSync_GetTopologyFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, gsyncGpuCount: *mut u32, gsyncGPUs: *mut NV_GSYNC_GPU, gsyncDisplayCount: u32, gsyncDisplays: *mut NV_GSYNC_DISPLAY) -> NvAPI_Status;

    /// This API returns the topology for the specified Sync device.
    ///
    /// HOW TO USE:
    /// 1. make a call to get the number of GPUs connected OR displays synced through Sync device
    ///    by passing the gsyncGPUs OR gsyncDisplays as NULL respectively.
    ///    Both gsyncGpuCount and gsyncDisplayCount can be retrieved in same call by passing
    ///    both gsyncGPUs and gsyncDisplays as NULL
    /// 2. On call success: Allocate memory based on gsyncGpuCount(for gsyncGPUs)
    ///    and/or gsyncDisplayCount(for gsyncDisplays) then make a call to populate
    ///    gsyncGPUs and/or gsyncDisplays respectively.
    pub unsafe fn NvAPI_GSync_GetTopology;
}

nvapi! {
    pub type GSync_SetSyncStateSettingsFn = extern "C" fn(gsyncDisplayCount: u32, pGsyncDisplays: *mut NV_GSYNC_DISPLAY, flags: u32) -> NvAPI_Status;

    /// Sets a new sync state for the displays in system.
    pub unsafe fn NvAPI_GSync_SetSyncStateSettings;
}

nvenum! {
    /// Source signal edge to be used for output pulse. See [NV_GSYNC_CONTROL_PARAMS].
    pub enum NVAPI_GSYNC_POLARITY / GSyncPolarity {
        NVAPI_GSYNC_POLARITY_RISING_EDGE / RisingEdge = 0,
        NVAPI_GSYNC_POLARITY_FALLING_EDGE / FallingEdge = 1,
        NVAPI_GSYNC_POLARITY_BOTH_EDGES / BothEdges = 2,
    }
}

nvenum! {
    /// Used in [NV_GSYNC_CONTROL_PARAMS].
    pub enum NVAPI_GSYNC_VIDEO_MODE / GSyncVideoMode {
        NVAPI_GSYNC_VIDEO_MODE_NONE / None = 0,
        NVAPI_GSYNC_VIDEO_MODE_TTL / TTL = 1,
        NVAPI_GSYNC_VIDEO_MODE_NTSCPALSECAM / NtscPalCam = 2,
        NVAPI_GSYNC_VIDEO_MODE_HDTV / Hdtv = 3,
        NVAPI_GSYNC_VIDEO_MODE_COMPOSITE / Composite = 4,
    }
}

nvenum! {
    /// Used in [NV_GSYNC_CONTROL_PARAMS].
    pub enum NVAPI_GSYNC_SYNC_SOURCE / GSyncSource {
        NVAPI_GSYNC_SYNC_SOURCE_VSYNC / VSync = 0,
        NVAPI_GSYNC_SYNC_SOURCE_HOUSESYNC / HouseSync = 1,
    }
}

nvstruct! {
    /// Used in [NV_GSYNC_CONTROL_PARAMS].
    pub struct NV_GSYNC_DELAY {
        /// Version of the structure
        pub version: NvVersion,
        /// delay to be induced in number of horizontal lines.
        pub numLines: u32,
        /// delay to be induced in number of pixels.
        pub numPixels: u32,
        /// maximum number of lines supported at current display mode to induce delay.
        ///
        /// Updated by [NvAPI_GSync_GetControlParameters]\(\).
        /// Read only.
        pub maxLines: u32,
        /// minimum number of pixels required at current display mode to induce delay.
        ///
        /// Updated by [NvAPI_GSync_GetControlParameters]\(\). Read only.
        pub minPixels: u32,
    }
}

nvbits! {
    /// Bitfield in [NV_GSYNC_CONTROL_PARAMS]
    pub enum NV_GSYNC_CONTROL_PARAMS_FLAGS / GSyncControlFlags {
        /// interlace mode for a Sync device
        NV_GSYNC_CONTROL_PARAMS_FLAGS_INTERLACE_MODE / INTERLACE_MODE = 0b00000001,
        /// Set this to make house sync as an output; valid only when
        /// [NV_GSYNC_CONTROL_PARAMS.source] is [NVAPI_GSYNC_SYNC_SOURCE_VSYNC] on P2061 boards.
        NV_GSYNC_CONTROL_PARAMS_FLAGS_SYNC_SOURCE_IS_OUTPUT / SYNC_SOURCE_IS_OUTPUT = 0b00000010,
    }
}

nvversion! { @NV_GSYNC_DELAY(1) }

nvstruct! {
    /// Used in [NvAPI_GSync_GetControlParameters]\(\) and [NvAPI_GSync_SetControlParameters]\(\).
    pub struct NV_GSYNC_CONTROL_PARAMS {
        /// Version of the structure
        pub version: NvVersion,
        /// Leading edge / Falling edge / both
        pub polarity: NVAPI_GSYNC_POLARITY,
        /// None, TTL, NTSCPALSECAM, HDTV
        pub vmode: NVAPI_GSYNC_VIDEO_MODE,
        /// Number of pulses to wait between framelock signal generation
        pub interval: u32,
        /// VSync/House sync
        pub source: NVAPI_GSYNC_SYNC_SOURCE,
        pub flags: NV_GSYNC_CONTROL_PARAMS_FLAGS,
        /// time delay between the frame sync signal and the GPUs signal.
        pub syncSkew: NV_GSYNC_DELAY,
        /// Sync start delay for master.
        pub startupDelay: NV_GSYNC_DELAY,
    }
}

nvversion! { @NV_GSYNC_CONTROL_PARAMS(1) }

nvapi! {
    pub type GSync_GetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;

    /// This API queries for sync control parameters as defined in [NV_GSYNC_CONTROL_PARAMS].
    pub unsafe fn NvAPI_GSync_GetControlParameters;
}

nvapi! {
    pub type GSync_SetControlParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pGsyncControls: *mut NV_GSYNC_CONTROL_PARAMS) -> NvAPI_Status;

    /// This API sets control parameters as defined in [NV_SYNC_CONTROL_PARAMS].
    pub unsafe fn NvAPI_GSync_SetControlParameters;
}

nvenum! {
    pub enum NVAPI_GSYNC_DELAY_TYPE / GSyncDelayType {
        NVAPI_GSYNC_DELAY_TYPE_UNKNOWN / Unknown = 0,
        NVAPI_GSYNC_DELAY_TYPE_SYNC_SKEW / SyncSkew = 1,
        NVAPI_GSYNC_DELAY_TYPE_STARTUP / Startup = 2,
    }
}

nvapi! {
    pub type GSync_AdjustSyncDelayFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, delayType: NVAPI_GSYNC_DELAY_TYPE, pGsyncDelay: *mut NV_GSYNC_DELAY, syncSteps: *mut u32) -> NvAPI_Status;

    /// This API adjusts the skew and startDelay to the closest possible values.
    ///
    /// Use this API before calling [NvAPI_GSync_SetControlParameters] for skew or startDelay.
    pub unsafe fn NvAPI_GSync_AdjustSyncDelay;
}

nvstruct! {
    /// Used in [NvAPI_GSync_GetSyncStatus]\(\).
    pub struct NV_GSYNC_STATUS {
        /// Version of the structure
        pub version: NvVersion,
        /// Is timing in sync?
        pub bIsSynced: BoolU32,
        /// Does the phase of the timing signal from the GPU = the phase of the master sync signal?
        pub bIsStereoSynced: BoolU32,
        /// Is the sync signal available?
        pub bIsSyncSignalAvailable: BoolU32,
    }
}

nvversion! { @NV_GSYNC_STATUS(1) }

nvapi! {
    pub type GSync_GetSyncStatusFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, hPhysicalGpu: NvPhysicalGpuHandle, status: *mut NV_GSYNC_STATUS) -> NvAPI_Status;

    /// This API queries the sync status of a GPU - timing, stereosync and sync signal availability.
    pub unsafe fn NvAPI_GSync_GetSyncStatus;
}

pub const NVAPI_MAX_RJ45_PER_GSYNC: usize = 2;

nvenum! {
    /// Used in [NV_GSYNC_STATUS_PARAMS].
    pub enum NVAPI_GSYNC_RJ45_IO / GSyncRJ45Configuration {
        NVAPI_GSYNC_RJ45_OUTPUT / Output = 0,
        NVAPI_GSYNC_RJ45_INPUT / Input = 1,
        /// This field is used to notify that the framelock is not actually present.
        NVAPI_GSYNC_RJ45_UNUSED / Unused = 2,
    }
}

nvstruct! {
    /// Used in [NvAPI_GSync_GetStatusParameters]\(\).
    pub struct NV_GSYNC_STATUS_PARAMS_V1 {
        pub version: NvVersion,
        /// The refresh rate
        pub refreshRate: u32,
        /// Configured as input / output
        pub RJ45_IO: [NVAPI_GSYNC_RJ45_IO; NVAPI_MAX_RJ45_PER_GSYNC],
        /// Connected to ethernet hub? \[ERRONEOUSLY CONNECTED!\]
        pub RJ45_Ethernet: [u32; NVAPI_MAX_RJ45_PER_GSYNC],
        /// Incoming house sync frequency in Hz
        pub houseSyncIncoming: u32,
        /// Is house sync connected?
        pub bHouseSync: BoolU32,
    }
}

nvstruct! {
    pub struct NV_GSYNC_STATUS_PARAMS_V2 {
        pub v1: NV_GSYNC_STATUS_PARAMS_V1,
        /// Valid only for P2061 board.
        ///
        /// If set to 1, it means that this P2061 board receives input from another P2061 board.
        pub bInternalSlave: BoolU32,
    }
}

nvinherit! { NV_GSYNC_STATUS_PARAMS_V2(v1: NV_GSYNC_STATUS_PARAMS_V1) }

nvversion! { NV_GSYNC_STATUS_PARAMS_V1(1) }
nvversion! { @=NV_GSYNC_STATUS_PARAMS NV_GSYNC_STATUS_PARAMS_V2(2) }

nvapi! {
    pub type GSync_GetStatusParametersFn = extern "C" fn(hNvGSyncDevice: NvGSyncDeviceHandle, pStatusParams: *mut NV_GSYNC_STATUS_PARAMS) -> NvAPI_Status;

    /// This API queries for sync status parameters as defined in [NV_GSYNC_STATUS_PARAMS].
    pub unsafe fn NvAPI_GSync_GetStatusParameters;
}
