/// Undocumented API
pub mod private {
    use crate::status::NvAPI_Status;
    use crate::handles::NvPhysicalGpuHandle;

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 {
            pub version: u32,
            pub flags: u32,
            pub zero: [u32; 8],
            pub value_uV: u32,
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER_1(NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 = 4 * (2 + 8 + 1 + 8), 1) }
    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER = NV_GPU_CLIENT_VOLT_RAILS_STATUS_VER_1 }

    pub type NV_GPU_CLIENT_VOLT_RAILS_STATUS = NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_ClientVoltRailsGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVoltageStatus: *mut NV_GPU_CLIENT_VOLT_RAILS_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 {
            pub version: u32,
            pub percent: u32, // apparently actually i32?
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER_1(NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 = 4 * (2 + 8), 1) }
    nvversion! { NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER = NV_GPU_CLIENT_VOLT_RAILS_CONTROL_VER_1 }

    pub type NV_GPU_CLIENT_VOLT_RAILS_CONTROL = NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_ClientVoltRailsGetControl(hPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *mut NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;
    }

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_ClientVoltRailsSetControl(hPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *const NV_GPU_CLIENT_VOLT_RAILS_CONTROL) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_GPU_ENTRY {
            pub a: u32, // 0
            pub freq_kHz: u32,
            pub voltage_uV: u32,
            pub d: u32,
            pub e: u32,
            pub f: u32,
            pub g: u32,
        }
    }

    // no real difference here
    pub type NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_MEM_ENTRY = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_GPU_ENTRY;
    /*nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_MEM_ENTRY {
            pub a: u32, // 1 for idle values?
            pub freq_kHz: u32,
            pub voltage_uV: u32,
            pub d: u32,
            pub e: u32,
            pub f: u32,
            pub g: u32,
        }
    }*/

    nvstruct! {
        pub struct NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 {
            pub version: u32,
            pub mask: [u32; 4], // 80 bits
            pub unknown: [u32; 12],
            pub gpuEntries: [NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_GPU_ENTRY; 80],
            pub memEntries: [NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_MEM_ENTRY; 23],
            pub unknown2: [u32; 1064],
        }
    }

    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_1(NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1 = 0x1c28, 1) }
    nvversion! { NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_VER_1 }

    pub type NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS = NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_ClockClientClkVfPointsGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVfpCurve: *mut NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_POWER_INFO_ENTRY {
            pub pstate: u32, // assumption
            pub b: u32,
            pub c: u32,
            pub min_power: u32,
            pub e: u32,
            pub f: u32,
            pub def_power: u32,
            pub h: u32,
            pub i: u32,
            pub max_power: u32,
            pub k: u32, // 0
        }
    }

    nvstruct! {
        pub struct NV_GPU_POWER_INFO_V1 {
            pub version: u32,
            pub valid: u8,
            pub count: u8,
            pub padding: [u8; 2],
            pub entries: [NV_GPU_POWER_INFO_ENTRY; 4],
        }
    }

    pub type NV_GPU_POWER_INFO = NV_GPU_POWER_INFO_V1;

    nvversion! { NV_GPU_POWER_INFO_VER_1(NV_GPU_POWER_INFO_V1 = 4 * 2 + 4 * (4 * 11), 1) }
    nvversion! { NV_GPU_POWER_INFO_VER = NV_GPU_POWER_INFO_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pPowerInfo: *mut NV_GPU_POWER_INFO) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_POWER_STATUS_ENTRY {
            pub a: u32,
            pub b: u32,
            pub power: u32,
            pub d: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_POWER_STATUS_V1 {
            pub version: u32,
            pub count: u32,
            pub entries: [NV_GPU_POWER_STATUS_ENTRY; 4],
        }
    }

    pub type NV_GPU_POWER_STATUS = NV_GPU_POWER_STATUS_V1;

    nvversion! { NV_GPU_POWER_STATUS_VER_1(NV_GPU_POWER_STATUS_V1 = 4 * 2 + 4 * (4 * 4), 1) }
    nvversion! { NV_GPU_POWER_STATUS_VER = NV_GPU_POWER_STATUS_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *mut NV_GPU_POWER_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesSetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *const NV_GPU_POWER_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_POWER_TOPO_ENTRY {
            pub a: u32,
            pub b: u32,
            pub power: u32,
            pub d: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_POWER_TOPO_V1 {
            pub version: u32,
            pub count: u32,
            pub entries: [NV_GPU_POWER_TOPO_ENTRY; 4],
        }
    }

    pub type NV_GPU_POWER_TOPO = NV_GPU_POWER_TOPO_V1;

    nvversion! { NV_GPU_POWER_TOPO_VER_1(NV_GPU_POWER_TOPO_V1 = 4 * 2 + 4 * (4 * 4), 1) }
    nvversion! { NV_GPU_POWER_TOPO_VER = NV_GPU_POWER_TOPO_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerTopologyGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPowerTopo: *mut NV_GPU_POWER_TOPO) -> NvAPI_Status;
    }

    nvbits! {
        pub enum NV_GPU_PERF_FLAGS / PerfFlags {
            NV_GPU_PERF_FLAGS_POWER_LIMIT / POWER_LIMIT = 1,
            NV_GPU_PERF_FLAGS_THERMAL_LIMIT / THERMAL_LIMIT = 2,
            /// Reliability voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_REL_LIMIT / VOLTAGE_REL_LIMIT = 4,
            /// Operating voltage
            NV_GPU_PERF_FLAGS_VOLTAGE_OP_LIMIT / VOLTAGE_OP_LIMIT = 8,
            /// GPU utilization
            NV_GPU_PERF_FLAGS_NO_LOAD_LIMIT / NO_LOAD_LIMIT = 16,
            /// Never seen this
            NV_GPU_PERF_FLAGS_UNKNOWN_32 / UNKNOWN_32 = 32,
        }
    }

    nvenum_display! {
        PerfFlags => {
            POWER_LIMIT = "Power",
            THERMAL_LIMIT = "Temperature",
            VOLTAGE_REL_LIMIT = "Reliability Voltage",
            VOLTAGE_OP_LIMIT = "Operating Voltage",
            NO_LOAD_LIMIT = "No Load",
            UNKNOWN_32 = "Unknown32",
            _ = _,
        }
    }

    nvstruct! {
        pub struct NV_GPU_PERF_INFO_V1 {
            pub version: u32,
            pub maxUnknown: u32,
            pub limitSupport: NV_GPU_PERF_FLAGS,
            pub padding: [u32; 16],
        }
    }

    pub type NV_GPU_PERF_INFO = NV_GPU_PERF_INFO_V1;

    nvversion! { NV_GPU_PERF_INFO_VER_1(NV_GPU_PERF_INFO_V1 = 76, 1) }
    nvversion! { NV_GPU_PERF_INFO_VER = NV_GPU_PERF_INFO_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_PerfPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pPerfInfo: *mut NV_GPU_PERF_INFO) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_GPU_PERF_STATUS_V1 {
            pub version: u32,
            pub flags: u32,
            /// nanoseconds
            pub timer: u64,
            /// - 1 = power limit
            /// - 2 = temp limit
            /// - 4 = voltage limit
            /// - 8 = only got with 15 in driver crash
            /// - 16 = no-load limit
            pub limits: NV_GPU_PERF_FLAGS,
            pub zero0: u32,
            /// - 1 on load
            /// - 3 in low clocks
            /// - 7 in idle
            pub unknown: u32,
            pub zero1: u32,
            /// nanoseconds
            pub timers: [u64; 3],
            pub padding: [u32; 326],
        }
    }

    pub type NV_GPU_PERF_STATUS = NV_GPU_PERF_STATUS_V1;

    nvversion! { NV_GPU_PERF_STATUS_VER_1(NV_GPU_PERF_STATUS_V1 = 0x550, 1) }
    nvversion! { NV_GPU_PERF_STATUS_VER = NV_GPU_PERF_STATUS_VER_1 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_PerfPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pPerfStatus: *mut NV_GPU_PERF_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VOLT_STATUS_V1 {
            pub version: u32,
            pub flags: u32,
            /// unsure
            pub count: u32,
            pub unknown: u32,
            pub value_uV: u32,
            pub buf1: [u32; 30],
        }
    }

    pub type NV_VOLT_STATUS = NV_VOLT_STATUS_V1;

    nvversion! { NV_VOLT_STATUS_VER_1(NV_VOLT_STATUS_V1 = 140, 1) }
    nvversion! { NV_VOLT_STATUS_VER = NV_VOLT_STATUS_VER_1 }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltageDomainsStatus(hPhysicalGPU: NvPhysicalGpuHandle, pVoltStatus: *mut NV_VOLT_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltageStep(hPhysicalGPU: NvPhysicalGpuHandle, pVoltStep: *mut NV_VOLT_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VOLT_TABLE_ENTRY {
            pub voltage_uV: u32,
            pub unknown: u32,
        }
    }

    nvstruct! {
        pub struct NV_VOLT_TABLE_V1 {
            pub version: u32,
            pub flags: u32,
            /// 1
            pub filled: u32,
            pub entries: [NV_VOLT_TABLE_ENTRY; 128],
            /// empty tables?
            pub buf1: [u32; 3888],
        }
    }

    pub type NV_VOLT_TABLE = NV_VOLT_TABLE_V1;

    nvversion! { NV_VOLT_TABLE_VER_1(NV_VOLT_TABLE_V1 = 0x40cc, 1) }
    nvversion! { NV_VOLT_TABLE_VER = NV_VOLT_TABLE_VER_1 }

    nvapi! {
        /// Maxwell only
        pub unsafe fn NvAPI_GPU_GetVoltages(hPhysicalGPU: NvPhysicalGpuHandle, pVolts: *mut NV_VOLT_TABLE) -> NvAPI_Status;
    }
}
