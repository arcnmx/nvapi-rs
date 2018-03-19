use status::NvAPI_Status;
use handles::NvPhysicalGpuHandle;
use types::BoolU32;

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
    /// Used in NvAPI_GPU_GetAllClockFrequencies()
    pub struct NV_GPU_CLOCK_FREQUENCIES_V1 {
        /// Structure version
        pub version: u32,
        /// These bits are reserved for future use.
        ///
        /// bits:2 is NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE. Used to specify the type of clock to be returned.
        pub reserved: u32,
        pub domain: [NV_GPU_CLOCK_FREQUENCIES_DOMAIN; NVAPI_MAX_GPU_PUBLIC_CLOCKS],
    }
}

impl NV_GPU_CLOCK_FREQUENCIES_V1 {
    pub fn ClockType(&self) -> NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE {
        (self.reserved & 3) as _
    }

    pub fn set_ClockType(&mut self, value: NV_GPU_CLOCK_FREQUENCIES_CLOCK_TYPE) {
        self.reserved = (value as u32) & 3;
    }
}

pub type NV_GPU_CLOCK_FREQUENCIES_V2 = NV_GPU_CLOCK_FREQUENCIES_V1;

/// Used in NvAPI_GPU_GetAllClockFrequencies()
pub type NV_GPU_CLOCK_FREQUENCIES = NV_GPU_CLOCK_FREQUENCIES_V2;

nvversion! { NV_GPU_CLOCK_FREQUENCIES_VER_1(NV_GPU_CLOCK_FREQUENCIES_V1 = 4 * 2 + (4 * 2) * NVAPI_MAX_GPU_PUBLIC_CLOCKS, 1) }
nvversion! { NV_GPU_CLOCK_FREQUENCIES_VER_2(NV_GPU_CLOCK_FREQUENCIES_V2 = 4 * 2 + (4 * 2) * NVAPI_MAX_GPU_PUBLIC_CLOCKS, 2) }
nvversion! { NV_GPU_CLOCK_FREQUENCIES_VER_3(NV_GPU_CLOCK_FREQUENCIES_V2 = 4 * 2 + (4 * 2) * NVAPI_MAX_GPU_PUBLIC_CLOCKS, 3) }
nvversion! { NV_GPU_CLOCK_FREQUENCIES_VER = NV_GPU_CLOCK_FREQUENCIES_VER_3 }

nvenum! {
    /// Used in NvAPI_GPU_GetAllClockFrequencies()
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
    pub struct NV_GPU_CLOCK_FREQUENCIES_DOMAIN {
        /// Set if this domain is present on this GPU
        pub bIsPresent: BoolU32,
        /// Clock frequency (kHz)
        pub frequency: u32,
    }
}

nvapi! {
    pub type GPU_GetAllClockFrequenciesFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pClkFreqs: *mut NV_GPU_CLOCK_FREQUENCIES) -> NvAPI_Status;

    /// This function retrieves the NV_GPU_CLOCK_FREQUENCIES structure for the specified physical GPU.
    ///
    /// For each clock domain:
    /// - bIsPresent is set for each domain that is present on the GPU
    /// - frequency is the domain's clock freq in kHz
    ///
    /// Each domain's info is indexed in the array.  For example:
    /// clkFreqs.domain[NVAPI_GPU_PUBLIC_CLOCK_MEMORY] holds the info for the MEMORY domain.
    pub unsafe fn NvAPI_GPU_GetAllClockFrequencies;
}

/// Undocumented API
pub mod private {
    // undocumented constants
    pub const NVAPI_MAX_USAGES_PER_GPU: usize = 8;
    pub const NVAPI_MAX_CLOCKS_PER_GPU: usize = 288;

    use types::BoolU32;
    use status::NvAPI_Status;
    use handles::NvPhysicalGpuHandle;
    use debug_array::Array;

    nvstruct! {
        pub struct NV_USAGES_INFO_USAGE {
            pub bIsPresent: BoolU32,
            /// % 0 to 100 usage
            pub percentage: u32,
            pub unknown: [u32; 2],
        }
    }

    nvstruct! {
        pub struct NV_USAGES_INFO_V1 {
            pub version: u32,
            pub flags: u32,
            /// (core_usage, memory_usage, video_engine_usage), probably indexed by NV_GPU_UTILIZATION_DOMAIN_ID
            pub usages: [NV_USAGES_INFO_USAGE; NVAPI_MAX_USAGES_PER_GPU],
        }
    }

    nvversion! { NV_USAGES_INFO_VER_1(NV_USAGES_INFO_V1 = 4 * (2 + 4 * NVAPI_MAX_USAGES_PER_GPU), 1) }
    nvversion! { NV_USAGES_INFO_VER = NV_USAGES_INFO_VER_1 }

    pub type NV_USAGES_INFO = NV_USAGES_INFO_V1;

    nvapi! {
        pub type GPU_GetUsagesFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pUsagesInfo: *mut NV_USAGES_INFO) -> NvAPI_Status;

        /// Undocumented function. Probably deprecated and replaced with NvAPI_GPU_GetDynamicPstatesInfoEx()
        pub unsafe fn NvAPI_GPU_GetUsages;
    }

    debug_array_impl! { [u32; NVAPI_MAX_CLOCKS_PER_GPU] }

    nvstruct! {
        pub struct NV_CLOCKS_INFO_V1 {
            pub version: u32,
            pub clocks: Array<[u32; NVAPI_MAX_CLOCKS_PER_GPU]>,
        }
    }

    nvversion! { NV_CLOCKS_INFO_VER_1(NV_CLOCKS_INFO_V1 = 4 * (1 + NVAPI_MAX_CLOCKS_PER_GPU), 1) }
    nvversion! { NV_CLOCKS_INFO_VER = NV_CLOCKS_INFO_VER_1 }

    pub type NV_CLOCKS_INFO = NV_CLOCKS_INFO_V1;

    nvapi! {
        pub type GPU_GetAllClocksFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pClocksInfo: *mut NV_CLOCKS_INFO) -> NvAPI_Status;

        /// Undocumented function. Probably deprecated and replaced with NvAPI_GPU_GetAllClockFrequencies()
        ///
        /// memory_clock = clocks[8] * 0.001f;
        ///
        /// if clocks[30] != 0 {
        /// core_clock = clocks[30] * 0.0005f
        /// shader_clock = clocks[30] * 0.001f
        /// } else {
        /// core_clock = clocks[0] * 0.001f
        /// shader_clock = clocks[14] * 0.001f
        /// }
        pub unsafe fn NvAPI_GPU_GetAllClocks;
    }

    nvstruct! {
        pub struct NV_CLOCK_TABLE_GPU_DELTA {
            pub a: u32,
            pub b: u32,
            pub c: u32,
            pub d: u32,
            pub e: u32,
            pub freqDeltaKHz: i32,
            pub g: u32,
            pub h: u32,
            pub i: u32,
        }
    }

    debug_array_impl! { [NV_CLOCK_TABLE_GPU_DELTA; 80] @nodefault }
    debug_array_impl! { [u32; 1529] }

    nvstruct! {
        pub struct NV_CLOCK_TABLE_V1 {
            pub version: u32,
            pub mask: [u32; 4], // 80 bits (might be 8xu32?)
            pub unknown: [u32; 12],
            pub gpuDeltas: Array<[NV_CLOCK_TABLE_GPU_DELTA; 80]>,
            pub memFilled: [u32; 23], // maybe only 4 max
            pub memDeltas: [i32; 23],
            pub unknown2: Array<[u32; 1529]>,
        }
    }

    nvversion! { NV_CLOCK_TABLE_VER_1(NV_CLOCK_TABLE_V1 = 9248, 1) }
    nvversion! { NV_CLOCK_TABLE_VER = NV_CLOCK_TABLE_VER_1 }

    pub type NV_CLOCK_TABLE = NV_CLOCK_TABLE_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetClockBoostTable(hPhysicalGPU: NvPhysicalGpuHandle, pClockTable: *mut NV_CLOCK_TABLE) -> NvAPI_Status;
    }

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_SetClockBoostTable(hPhysicalGPU: NvPhysicalGpuHandle, pClockTable: *const NV_CLOCK_TABLE) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_CLOCK_RANGES_ENTRY {
            pub a: u32,
            pub clockType: super::NV_GPU_PUBLIC_CLOCK_ID,
            pub c: u32,
            pub d: u32,
            pub e: u32,
            pub f: u32,
            pub g: u32,
            pub h: u32,
            pub i: u32,
            pub j: u32,
            pub rangeMax: i32,
            pub rangeMin: i32,
            pub tempMax: i32, // unsure
            pub n: u32,
            pub o: u32,
            pub p: u32,
            pub q: u32,
            pub r: u32,
        }
    }

    nvstruct! {
        pub struct NV_CLOCK_RANGES_V1 {
            pub version: u32,
            pub numClocks: u32, // unsure
            pub zero: [u32; 8],
            pub entries: [NV_CLOCK_RANGES_ENTRY; 32],
        }
    }

    nvversion! { NV_CLOCK_RANGES_VER_1(NV_CLOCK_RANGES_V1 = 2344, 1) }
    nvversion! { NV_CLOCK_RANGES_VER = NV_CLOCK_RANGES_VER_1 }

    pub type NV_CLOCK_RANGES = NV_CLOCK_RANGES_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetClockBoostRanges(hPhysicalGPU: NvPhysicalGpuHandle, pClockRanges: *mut NV_CLOCK_RANGES) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_CLOCK_MASKS_CLOCK {
            pub a: u32,
            pub b: u32,
            pub c: u32,
            pub d: u32,
            pub memDelta: u32, // 1 for mem
            pub gpuDelta: u32, // 1 for gpu
        }
    }

    debug_array_impl! { [NV_CLOCK_MASKS_CLOCK; 80 + 23] @nodefault }
    debug_array_impl! { [u32; 916] }

    nvstruct! {
        pub struct NV_CLOCK_MASKS_V1 {
            pub version: u32,
            pub mask: [u32; 4], // 80 bits
            pub unknown: [u32; 8],
            pub clocks: Array<[NV_CLOCK_MASKS_CLOCK; 80 + 23]>,
            pub unknown2: Array<[u32; 916]>,
        }
    }

    nvversion! { NV_CLOCK_MASKS_VER_1(NV_CLOCK_MASKS_V1 = 6188, 1) }
    nvversion! { NV_CLOCK_MASKS_VER = NV_CLOCK_MASKS_VER_1 }

    pub type NV_CLOCK_MASKS = NV_CLOCK_MASKS_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetClockBoostMask(hPhysicalGPU: NvPhysicalGpuHandle, pClockMasks: *mut NV_CLOCK_MASKS) -> NvAPI_Status;
    }

    nvenum! {
        pub enum NV_GPU_CLOCK_LOCK_MODE / ClockLockMode {
            NVAPI_GPU_CLOCK_LOCK_NONE / None = 0,
            NVAPI_GPU_CLOCK_LOCK_MANUAL / Manual = 3,
        }
    }

    nvstruct! {
        pub struct NV_CLOCK_LOCK_ENTRY {
            pub id: u32, // entry index
            pub b: u32, // 0
            pub mode: NV_GPU_CLOCK_LOCK_MODE, // 0 = default, 3 = manual voltage
            pub d: u32, // 0
            pub voltage_uV: u32, // 0 unless set explicitly, seems to always get set on the last/highest entry only
            pub f: u32, // 0
        }
    }

    nvstruct! {
        // 2-030c: 0C 03 02 00 00 00 00 00 01 00 00 00 06 00 00 00
        pub struct NV_CLOCK_LOCK_V2 {
            pub version: u32,
            pub flags: u32, // unknown, only see 0
            pub count: u32,
            pub entries: [NV_CLOCK_LOCK_ENTRY; 0x20],
        }
    }

    pub type NV_CLOCK_LOCK = NV_CLOCK_LOCK_V2;

    nvversion! { NV_CLOCK_LOCK_VER_2(NV_CLOCK_LOCK_V2 = 0x30c, 2) }
    nvversion! { NV_CLOCK_LOCK_VER = NV_CLOCK_LOCK_VER_2 }

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetClockBoostLock(hPhysicalGPU: NvPhysicalGpuHandle, pClockLocks: *mut NV_CLOCK_LOCK) -> NvAPI_Status;
    }

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_SetClockBoostLock(hPhysicalGPU: NvPhysicalGpuHandle, pClockLocks: *const NV_CLOCK_LOCK) -> NvAPI_Status;
    }
}
