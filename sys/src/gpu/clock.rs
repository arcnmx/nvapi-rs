use crate::prelude_::*;

pub const NVAPI_MAX_GPU_CLOCKS: usize = 32;
pub const NVAPI_MAX_GPU_PUBLIC_CLOCKS: usize = 32;
pub const NVAPI_MAX_GPU_PERF_CLOCKS: usize = 32;
pub const NVAPI_MAX_GPU_PERF_VOLTAGES: usize = 16;
pub const NVAPI_MAX_GPU_PERF_PSTATES: usize = 16;

nvenum! {
    /// An index into NV_GPU_CLOCK_FREQUENCIES.domain[]
    pub enum NV_GPU_PUBLIC_CLOCK_ID / PublicClockId {
        NVAPI_GPU_PUBLIC_CLOCK_GRAPHICS / Graphics = 0,
        NVAPI_GPU_PUBLIC_CLOCK_MEMORY / Memory = 4,
        NVAPI_GPU_PUBLIC_CLOCK_PROCESSOR / Processor = 7,
        NVAPI_GPU_PUBLIC_CLOCK_VIDEO / Video = 8,
        NVAPI_GPU_PUBLIC_CLOCK_UNDEFINED / Undefined = NVAPI_MAX_GPU_PUBLIC_CLOCKS,
    }
}

nvenum_display! {
    PublicClockId => _
}

nvstruct! {
    /// Used in [NvAPI_GPU_GetAllClockFrequencies]\(\)
    pub struct NV_GPU_CLOCK_FREQUENCIES_V1 {
        /// Structure version
        pub version: NvVersion,
        /// These bits are reserved for future use.
        ///
        /// `bits:2` is [NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE]. Used to specify the type of clock to be returned.
        pub reserved: u32,
        pub domain: Array<[NV_GPU_CLOCK_FREQUENCIES_DOMAIN; NVAPI_MAX_GPU_PUBLIC_CLOCKS]>,
    }
}

impl NV_GPU_CLOCK_FREQUENCIES_V1 {
    pub fn ClockType(&self) -> NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE {
        ((self.reserved & 3) as i32).into()
    }

    pub fn set_ClockType(&mut self, value: NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE) {
        self.reserved = (value.repr() as u32) & 3;
    }
}

nvversion! { NV_GPU_CLOCK_FREQUENCIES:
    NV_GPU_CLOCK_FREQUENCIES_V1(3),
    NV_GPU_CLOCK_FREQUENCIES_V1(2; @old),
    NV_GPU_CLOCK_FREQUENCIES_V1(1; @old)
}

nvenum! {
    /// Used in [NvAPI_GPU_GetAllClockFrequencies]\(\)
    pub enum NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE / ClockFrequencyType {
        NV_GPU_CLOCK_FREQUENCIES_CURRENT_FREQ / Current = 0,
        NV_GPU_CLOCK_FREQUENCIES_BASE_CLOCK / Base = 1,
        NV_GPU_CLOCK_FREQUENCIES_BOOST_CLOCK / Boost = 2,
        NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE_NUM / Count = 3,
    }
}

nvenum_display! {
    ClockFrequencyType => _
}

nvstruct! {
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_CLOCK_FREQUENCIES_DOMAIN {
        /// Set if this domain is present on this GPU
        pub bIsPresent: BoolU32,
        /// Clock frequency (kHz)
        pub frequency: u32,
    }
}

nvapi! {
    pub type GPU_GetAllClockFrequenciesFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pClkFreqs@StructVersion: *mut NV_GPU_CLOCK_FREQUENCIES) -> NvAPI_Status;

    /// This function retrieves the [NV_GPU_CLOCK_FREQUENCIES] structure for the specified physical GPU.
    ///
    /// For each clock domain:
    /// - bIsPresent is set for each domain that is present on the GPU
    /// - frequency is the domain's clock freq in kHz
    ///
    /// Each domain's info is indexed in the array.  For example:
    /// `clkFreqs.domain[NVAPI_GPU_PUBLIC_CLOCK_MEMORY]` holds the info for the MEMORY domain.
    pub fn NvAPI_GPU_GetAllClockFrequencies;

    impl self {
        pub fn GetAllClockFrequencies;
    }
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;
    use super::{NV_GPU_PUBLIC_CLOCK_ID, NVAPI_MAX_GPU_PUBLIC_CLOCKS, NVAPI_MAX_GPU_PERF_CLOCKS};

    // undocumented constants
    pub const NVAPI_MAX_USAGES_PER_GPU: usize = 8;
    pub const NVAPI_MAX_CLOCKS_PER_GPU: usize = 288;

    nvstruct! {
        #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
        pub struct NV_USAGES_INFO_USAGE {
            pub bIsPresent: BoolU32,
            /// % 0 to 100 usage
            pub percentage: u32,
            pub unknown: [u32; 2],
        }
    }

    nvstruct! {
        pub struct NV_USAGES_INFO_V1 {
            pub version: NvVersion,
            pub flags: u32,
            /// (core_usage, memory_usage, video_engine_usage), probably indexed by NV_GPU_UTILIZATION_DOMAIN_ID
            pub usages: Array<[NV_USAGES_INFO_USAGE; NVAPI_MAX_USAGES_PER_GPU]>,
        }
    }

    nvversion! { NV_USAGES_INFO:
        NV_USAGES_INFO_V1(1)
    }

    nvapi! {
        pub type GPU_GetUsagesFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pUsagesInfo@StructVersionOut: *mut NV_USAGES_INFO) -> NvAPI_Status;

        /// Undocumented function. Probably deprecated and replaced with NvAPI_GPU_GetDynamicPstatesInfoEx()
        pub fn NvAPI_GPU_GetUsages;

        impl self {
            pub fn GetUsages;
        }
    }

    nvstruct! {
        pub struct NV_CLOCKS_INFO_V1 {
            pub version: NvVersion,
            pub clocks: Array<[u32; NVAPI_MAX_CLOCKS_PER_GPU]>,
        }
    }

    nvversion! { NV_CLOCKS_INFO:
        NV_CLOCKS_INFO_V1(1)
    }

    nvapi! {
        pub type GPU_GetAllClocksFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pClocksInfo@StructVersionOut: *mut NV_CLOCKS_INFO) -> NvAPI_Status;

        /// Undocumented function. Probably deprecated and replaced with [NvAPI_GPU_GetAllClockFrequencies()](super::NvAPI_GPU_GetAllClockFrequencies)
        ///
        /// ```
        /// memory_clock = clocks[8] * 0.001f;
        ///
        /// if clocks[30] != 0 {
        /// core_clock = clocks[30] * 0.0005f
        /// shader_clock = clocks[30] * 0.001f
        /// } else {
        /// core_clock = clocks[0] * 0.001f
        /// shader_clock = clocks[14] * 0.001f
        /// }
        /// ```
        pub fn NvAPI_GPU_GetAllClocks;

        impl self {
            pub fn GetAllClocks;
        }
    }

    pub type NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_PROG_V1 = i32;

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_V1 {
            pub clock_type: u32,
            pub unknown0: Padding<[u32; 4]>,
            /// offsetFrequencyKhz
            pub freqDeltaKHz: NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_PROG_V1,
            pub unknown1: Padding<[u32; 3]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_V1 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u32; 8]>,
            pub points: Array<[NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_V1; 255]>,
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL:
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_V1(2),
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_V1(1; @old) = 9248
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClockClientClkVfPointsGetControl(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockTable@StructVersion: *mut NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL) -> NvAPI_Status;

        impl self {
            pub fn ClockClientClkVfPointsGetControl;
        }
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClockClientClkVfPointsSetControl(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockTable@StructVersion: *const NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL) -> NvAPI_Status;

        impl self {
            pub fn ClockClientClkVfPointsSetControl;
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY {
            pub disabled: BoolU32,
            pub clockType: NV_GPU_PUBLIC_CLOCK_ID,
            pub unknown0: Padding<[u32; 8]>,
            pub rangeMax: i32,
            pub rangeMin: i32,
            pub vfpIndexMin: u8,
            pub vfpIndexMax: u8,
            pub padding: Padding<[u8; 2]>,
            pub unknown1: Padding<[u32; 5]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_V1 {
            pub version: NvVersion,
            pub mask: ClockMask<1>,
            pub zero: Padding<[u32; 8]>,
            pub clocks: Array<[NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY; NVAPI_MAX_GPU_PUBLIC_CLOCKS]>,
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO:
        NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_V1(1) = 2344
    }

    nvapi! {
        /// Pascal only
        pub fn NvAPI_GPU_ClockClientClkDomainsGetInfo(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockRanges@StructVersionOut: *mut NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO) -> NvAPI_Status;

        impl self {
            pub fn ClockClientClkDomainsGetInfo;
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
            /// 1 for mem
            pub memDelta: u32,
            /// 1 for gpu
            pub gpuDelta: u32,
            pub unknown: Padding<[u32; 4]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_V1 {
            pub version: NvVersion,
            pub mask: ClockMask,
            pub unknown: Padding<[u32; 8]>,
            pub clocks: Array<[NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK; 255]>,
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO:
        NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_V1(1) = 6188
    }

    nvapi! {
        /// Pascal and later
        pub fn NvAPI_GPU_ClockClientClkVfPointsGetInfo(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockMasks@StructVersionOut: *mut NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO) -> NvAPI_Status;

        impl self {
            pub fn ClockClientClkVfPointsGetInfo;
        }
    }

    nvenum! {
        pub enum NV_GPU_CLOCK_LOCK_MODE / ClockLockMode {
            NVAPI_GPU_CLOCK_LOCK_NONE / None = 0,
            NVAPI_GPU_CLOCK_LOCK_MANUAL_FREQUENCY / ManualFrequency = 2,
            NVAPI_GPU_CLOCK_LOCK_MANUAL_VOLTAGE / ManualVoltage = 3,
        }
    }

    nvenum! {
        pub enum NV_PERF_CLIENT_LIMIT_ID / PerfLimitId {
            NV_PERF_CLIENT_LIMIT_ID_GPU / Gpu = 0,
            NV_PERF_CLIENT_LIMIT_ID_GPU_UNKNOWN / GpuUnknown = 1,
            NV_PERF_CLIENT_LIMIT_ID_MEMORY / Memory = 2,
            NV_PERF_CLIENT_LIMIT_ID_MEMORY_UNKNOWN / MemoryUnknown = 3,
            NV_PERF_CLIENT_LIMIT_ID_UNKNOWN_4 / Unknown_4 = 4,
            NV_PERF_CLIENT_LIMIT_ID_UNKNOWN_5 / Unknown_5 = 5,
            NV_PERF_CLIENT_LIMIT_ID_VOLTAGE / Voltage = 6,
        }
    }

    nvenum_display! {
        PerfLimitId => {
            Gpu = "GPU",
            _ = _,
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_PERF_CLIENT_LIMITS_ENTRY {
            pub id: NV_PERF_CLIENT_LIMIT_ID, // entry index
            pub b: u32, // 0
            pub mode: NV_GPU_CLOCK_LOCK_MODE, // 0 = default, 3 = manual voltage
            pub d: u32, // 0
            /// voltage uV or freq kHz depending on `id`
            pub value: u32, // 0 unless set explicitly, seems to always get set on the last/highest entry only
            pub clock_id: NV_GPU_PUBLIC_CLOCK_ID,
        }
    }

    nvstruct! {
        // 2-030c: 0C 03 02 00 00 00 00 00 01 00 00 00 06 00 00 00
        pub struct NV_GPU_PERF_CLIENT_LIMITS_V2 {
            pub version: NvVersion,
            pub flags: u32, // unknown, only see 0
            pub count: u32,
            pub entries: Array<[NV_GPU_PERF_CLIENT_LIMITS_ENTRY; NVAPI_MAX_GPU_PERF_CLOCKS]>,
        }
    }

    impl NV_GPU_PERF_CLIENT_LIMITS_V2 {
        pub fn entries(&self) -> &[NV_GPU_PERF_CLIENT_LIMITS_ENTRY] {
            &self.entries[..self.count as usize]
        }
    }

    nvversion! { NV_GPU_PERF_CLIENT_LIMITS:
        NV_GPU_PERF_CLIENT_LIMITS_V2(2) = 0x30c
    }

    nvapi! {
        /// Pascal only
        pub fn NvAPI_GPU_PerfClientLimitsGetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockLocks@StructVersion: *mut NV_GPU_PERF_CLIENT_LIMITS) -> NvAPI_Status;

        impl self {
            pub fn PerfClientLimitsGetStatus;
        }
    }

    nvapi! {
        /// Pascal only
        pub fn NvAPI_GPU_PerfClientLimitsSetStatus(hPhysicalGPU@self: NvPhysicalGpuHandle, pClockLocks@StructVersion: *const NV_GPU_PERF_CLIENT_LIMITS) -> NvAPI_Status;

        impl self {
            pub fn PerfClientLimitsSetStatus;
        }
    }
}
