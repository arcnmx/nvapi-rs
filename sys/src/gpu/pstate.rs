use crate::prelude_::*;
use crate::gpu::clock::{PublicClockId, NV_GPU_PUBLIC_CLOCK_ID, NVAPI_MAX_GPU_PERF_PSTATES, NVAPI_MAX_GPU_PERF_VOLTAGES};

pub const NVAPI_MAX_GPU_PSTATE20_PSTATES: usize = 16;
pub const NVAPI_MAX_GPU_PSTATE20_CLOCKS: usize = 8;
pub const NVAPI_MAX_GPU_PSTATE20_BASE_VOLTAGES: usize = 4;

nvstruct! {
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_DYNAMIC_PSTATES_INFO_EX_UTILIZATION {
        /// Set if this utilization domain is present on this GPU
        pub bIsPresent: BoolU32,
        /// Percentage of time where the domain is considered busy in the last 1 second interval
        pub percentage: u32,
    }
}

pub const NVAPI_MAX_GPU_UTILIZATIONS: usize = 8;

nvstruct! {
    /// Used in NvAPI_GPU_GetDynamicPstatesInfoEx().
    pub struct NV_GPU_DYNAMIC_PSTATES_INFO_EX {
        /// Structure version
        pub version: NvVersion,
        /// bit 0 indicates if the dynamic Pstate is enabled or not
        pub flags: u32,
        pub utilization: Array<[NV_GPU_DYNAMIC_PSTATES_INFO_EX_UTILIZATION; NVAPI_MAX_GPU_UTILIZATIONS]>,
    }
}

impl NV_GPU_DYNAMIC_PSTATES_INFO_EX {
    pub fn flag_enabled(&self) -> bool {
        self.flags & 1 != 0
    }
}

nvenum! {
    /// Domain index into NV_GPU_DYNAMIC_PSTATES_INFO_EX.utilization.
    ///
    /// Definition missing from the nvapi headers for some reason.
    pub enum NV_GPU_UTILIZATION_DOMAIN_ID / UtilizationDomain {
        NVAPI_GPU_UTILIZATION_DOMAIN_GPU / Graphics = 0,
        NVAPI_GPU_UTILIZATION_DOMAIN_FB / FrameBuffer = 1,
        NVAPI_GPU_UTILIZATION_DOMAIN_VID / VideoEngine = 2,
        NVAPI_GPU_UTILIZATION_DOMAIN_BUS / BusInterface = 3,
    }
}

nvenum_display! {
    UtilizationDomain => {
        FrameBuffer = "Frame Buffer",
        VideoEngine = "Video Engine",
        BusInterface = "Bus Interface",
        _ = _,
    }
}

impl UtilizationDomain {
    pub fn from_clock(c: PublicClockId) -> Option<Self> {
        match c {
            PublicClockId::Graphics => Some(UtilizationDomain::Graphics),
            PublicClockId::Memory => Some(UtilizationDomain::FrameBuffer),
            PublicClockId::Video => Some(UtilizationDomain::VideoEngine),
            _ => None,
        }
    }
}

nvversion! { _:
    NV_GPU_DYNAMIC_PSTATES_INFO_EX(1)
}

nvapi! {
    pub type GPU_GetDynamicPstatesInfoExFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pDynamicPstatesInfoEx: *mut NV_GPU_DYNAMIC_PSTATES_INFO_EX) -> NvAPI_Status;

    /// This API retrieves the NV_GPU_DYNAMIC_PSTATES_INFO_EX structure for the specified physical GPU.
    ///
    /// Each domain's info is indexed in the array.  For example:
    /// - pDynamicPstatesInfo->utilization[NVAPI_GPU_UTILIZATION_DOMAIN_GPU] holds the info for the GPU domain.
    ///
    /// There are currently 4 domains for which GPU utilization and dynamic P-State thresholds can be retrieved:
    /// - graphic engine (GPU)
    /// - frame buffer (FB)
    /// - video engine (VID)
    /// - bus interface (BUS)
    pub unsafe fn NvAPI_GPU_GetDynamicPstatesInfoEx;
}

nvenum! {
    pub enum NV_GPU_PERF_PSTATE_ID / PstateId {
        NVAPI_GPU_PERF_PSTATE_P0 / P0 = 0,
        NVAPI_GPU_PERF_PSTATE_P1 / P1 = 1,
        NVAPI_GPU_PERF_PSTATE_P2 / P2 = 2,
        NVAPI_GPU_PERF_PSTATE_P3 / P3 = 3,
        NVAPI_GPU_PERF_PSTATE_P4 / P4 = 4,
        NVAPI_GPU_PERF_PSTATE_P5 / P5 = 5,
        NVAPI_GPU_PERF_PSTATE_P6 / P6 = 6,
        NVAPI_GPU_PERF_PSTATE_P7 / P7 = 7,
        NVAPI_GPU_PERF_PSTATE_P8 / P8 = 8,
        NVAPI_GPU_PERF_PSTATE_P9 / P9 = 9,
        NVAPI_GPU_PERF_PSTATE_P10 / P10 = 10,
        NVAPI_GPU_PERF_PSTATE_P11 / P11 = 11,
        NVAPI_GPU_PERF_PSTATE_P12 / P12 = 12,
        NVAPI_GPU_PERF_PSTATE_P13 / P13 = 13,
        NVAPI_GPU_PERF_PSTATE_P14 / P14 = 14,
        NVAPI_GPU_PERF_PSTATE_P15 / P15 = 15,
        NVAPI_GPU_PERF_PSTATE_UNDEFINED / Undefined = NVAPI_MAX_GPU_PERF_PSTATES as i32,
        NVAPI_GPU_PERF_PSTATE_ALL / All = NVAPI_MAX_GPU_PERF_PSTATES as i32 + 1,
    }
}

nvenum_display! {
    PstateId => _
}

nvapi! {
    pub type GPU_GetCurrentPstateFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pCurrentPstate: *mut NV_GPU_PERF_PSTATE_ID) -> NvAPI_Status;

    /// This function retrieves the current performance state (P-State).
    pub unsafe fn NvAPI_GPU_GetCurrentPstate;
}

nvenum! {
    pub enum NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID / VoltageInfoDomain {
        NVAPI_GPU_PERF_VOLTAGE_INFO_DOMAIN_CORE / Core = 0,
        // 1 - 15?
        NVAPI_GPU_PERF_VOLTAGE_INFO_DOMAIN_UNDEFINED / Undefined = NVAPI_MAX_GPU_PERF_VOLTAGES as i32,
    }
}

nvenum_display! {
    VoltageInfoDomain => _
}

nvenum! {
    /// Used to identify clock type
    pub enum NV_GPU_PERF_PSTATE20_CLOCK_TYPE_ID / PstateClockType {
        /// Clock domains that use single frequency value within given pstate
        NVAPI_GPU_PERF_PSTATE20_CLOCK_TYPE_SINGLE / Single = 0,
        /// Clock domains that allow range of frequency values within given pstate
        NVAPI_GPU_PERF_PSTATE20_CLOCK_TYPE_RANGE / Range = 1,
    }
}

nvstruct! {
    /// Used to describe both voltage and frequency deltas
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_PERF_PSTATES20_PARAM_DELTA {
        /// Value of parameter delta (in respective units [kHz, uV])
        pub value: i32,
        /// Min value allowed for parameter delta (in respective units [kHz, uV])
        pub min: i32,
        /// Max value allowed for parameter delta (in respective units [kHz, uV])
        pub max: i32,
    }
}

nvstruct! {
    /// Used to describe single clock entry
    #[derive(Default)]
    pub struct NV_GPU_PSTATE20_CLOCK_ENTRY_V1 {
        /// ID of the clock domain
        pub domainId: NV_GPU_PUBLIC_CLOCK_ID,
        /// Clock type ID
        pub typeId: NV_GPU_PERF_PSTATE20_CLOCK_TYPE_ID,
        pub bIsEditable: BoolU32,
        /// Current frequency delta from nominal settings in (kHz)
        pub freqDelta_kHz: NV_GPU_PERF_PSTATES20_PARAM_DELTA,
        pub data: NV_GPU_PSTATE20_CLOCK_ENTRY_DATA,
    }
}

#[NvStruct]
#[repr(transparent)]
#[derive(Default)]
pub struct NV_GPU_PSTATE20_CLOCK_ENTRY_DATA(NV_GPU_PSTATE20_CLOCK_ENTRY_RANGE);

#[derive(Copy, Clone, Debug)]
pub enum NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE {
    Single(NV_GPU_PSTATE20_CLOCK_ENTRY_SINGLE),
    Range(NV_GPU_PSTATE20_CLOCK_ENTRY_RANGE),
}

impl NV_GPU_PSTATE20_CLOCK_ENTRY_DATA {
    pub fn get(&self, kind: PstateClockType) -> NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE {
        match kind {
            PstateClockType::Single => NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Single(
                NV_GPU_PSTATE20_CLOCK_ENTRY_SINGLE {
                    freq_kHz: (self.0).minFreq_kHz,
                }
            ),
            PstateClockType::Range => NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Range(self.0),
        }
    }

    pub fn set_single(&mut self, value: NV_GPU_PSTATE20_CLOCK_ENTRY_SINGLE) {
        (self.0).minFreq_kHz = value.freq_kHz;
    }

    pub fn set_range(&mut self, value: NV_GPU_PSTATE20_CLOCK_ENTRY_RANGE) {
        self.0 = value;
    }
}

nvstruct! {
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_PSTATE20_CLOCK_ENTRY_SINGLE {
        /// Clock frequency within given pstate in (kHz)
        pub freq_kHz: u32,
    }
}

nvstruct! {
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_PSTATE20_CLOCK_ENTRY_RANGE {
        /// Min clock frequency within given pstate in (kHz)
        pub minFreq_kHz: u32,
        /// Max clock frequency within given pstate in (kHz)
        pub maxFreq_kHz: u32,
        /// Voltage domain ID
        pub domainId: NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID,
        /// Minimum value in (uV) required for this clock
        pub minVoltage_uV: u32,
        /// Maximum value in (uV) required for this clock
        pub maxVoltage_uV: u32,
    }
}

nvstruct! {
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1 {
        /// ID of the voltage domain
        pub domainId: NV_GPU_PERF_VOLTAGE_INFO_DOMAIN_ID,
        pub bIsEditable: BoolU32,
        /// Current base voltage settings in \[uV\]
        pub volt_uV: u32,
        /// Current base voltage delta from nominal settings in \[uV\]
        pub voltDelta_uV: NV_GPU_PERF_PSTATES20_PARAM_DELTA,
    }
}

nvstruct! {
    /// Performance state (P-State) settings
    #[derive(Default)]
    pub struct NV_GPU_PERF_PSTATES20_PSTATE {
        /// ID of the P-State
        pub pstateId: NV_GPU_PERF_PSTATE_ID,
        /// Value must be 0 or 1.
        /// These bits are reserved for future use (must be always 0)
        pub bIsEditable: BoolU32,
        /// Array of clock entries
        /// Valid index range is 0 to numClocks-1
        pub clocks: Array<[NV_GPU_PSTATE20_CLOCK_ENTRY_V1; NVAPI_MAX_GPU_PSTATE20_CLOCKS]>,
        /// Array of baseVoltage entries
        /// Valid index range is 0 to numBaseVoltages-1
        pub baseVoltages: Array<[NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1; NVAPI_MAX_GPU_PSTATE20_BASE_VOLTAGES]>,
    }
}

nvstruct! {
    /// Used in NvAPI_GPU_GetPstates20() interface call.
    pub struct NV_GPU_PERF_PSTATES20_INFO_V1 {
        /// Version info of the structure (`NV_GPU_PERF_PSTATES20_INFO_VER<n>`)
        pub version: NvVersion,
        pub bIsEditable: BoolU32,
        /// Number of populated pstates
        pub numPstates: u32,
        /// Number of populated clocks (per pstate)
        pub numClocks: u32,
        /// Number of populated base voltages (per pstate)
        pub numBaseVoltages: u32,
        /// Performance state (P-State) settings
        /// Valid index range is 0 to numPstates-1
        pub pstates: Array<[NV_GPU_PERF_PSTATES20_PSTATE; NVAPI_MAX_GPU_PSTATE20_PSTATES]>,
    }
}

nvstruct! {
    /// Used in NvAPI_GPU_GetPstates20() interface call.
    pub struct NV_GPU_PERF_PSTATES20_INFO_V2 {
        #[nv_inherit] #[nv_version_field]
        pub v1: NV_GPU_PERF_PSTATES20_INFO_V1,
        /// Number of populated voltages
        pub numVoltages: u32,
        /// OV settings - Please refer to NVIDIA over-volting recommendation to understand impact of this functionality
        /// Valid index range is 0 to numVoltages-1
        pub voltages: Array<[NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1; NVAPI_MAX_GPU_PSTATE20_BASE_VOLTAGES]>,
    }
}

nvversion! { NV_GPU_PERF_PSTATES20_INFO:
    NV_GPU_PERF_PSTATES20_INFO_V2(3; @inherit(v1: NV_GPU_PERF_PSTATES20_INFO_V1)),
    NV_GPU_PERF_PSTATES20_INFO_V2(2; @old),
    NV_GPU_PERF_PSTATES20_INFO_V1(1)
}

nvapi! {
    pub type GPU_GetPstates20Fn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pPstatesInfo: *mut NV_GPU_PERF_PSTATES20_INFO) -> NvAPI_Status;

    /// This API retrieves all performance states (P-States) 2.0 information.
    ///
    /// P-States are GPU active/executing performance capability states.
    /// They range from P0 to P15, with P0 being the highest performance state,
    /// and P15 being the lowest performance state. Each P-State, if available,
    /// maps to a performance level. Not all P-States are available on a given system.
    /// The definition of each P-States are currently as follow:
    /// - P0/P1 - Maximum 3D performance
    /// - P2/P3 - Balanced 3D performance-power
    /// - P8 - Basic HD video playback
    /// - P10 - DVD playback
    /// - P12 - Minimum idle power consumption
    pub unsafe fn NvAPI_GPU_GetPstates20;
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;
    use super::{NvPhysicalGpuHandle, NV_GPU_PERF_PSTATES20_INFO};

    nvapi! {
        pub type GPU_SetPstates20Fn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pPstatesInfo: *const NV_GPU_PERF_PSTATES20_INFO) -> NvAPI_Status;

        /// Undocumented private API
        pub unsafe fn NvAPI_GPU_SetPstates20;
    }
}
