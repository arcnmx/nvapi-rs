/// Undocumented API
pub mod private {
    use status::NvAPI_Status;
    use handles::NvPhysicalGpuHandle;
    use debug_array::Array;

    nvstruct! {
        pub struct NV_VOLTAGE_STATUS_V1 {
            pub version: u32,
            pub flags: u32,
            pub zero: [u32; 8],
            pub value_uV: u32,
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_VOLTAGE_STATUS_VER_1(NV_VOLTAGE_STATUS_V1 = 4 * (2 + 8 + 1 + 8), 1) }
    nvversion! { NV_VOLTAGE_STATUS_VER = NV_VOLTAGE_STATUS_VER_1 }

    pub type NV_VOLTAGE_STATUS = NV_VOLTAGE_STATUS_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetCurrentVoltage(pPhysicalGPU: NvPhysicalGpuHandle, pVoltageStatus: *mut NV_VOLTAGE_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VOLTAGE_BOOST_PERCENT_V1 {
            pub version: u32,
            pub percent: u32, // apparently actually i32?
            pub unknown: [u32; 8],
        }
    }

    nvversion! { NV_VOLTAGE_BOOST_PERCENT_VER_1(NV_VOLTAGE_BOOST_PERCENT_V1 = 4 * (2 + 8), 1) }
    nvversion! { NV_VOLTAGE_BOOST_PERCENT_VER = NV_VOLTAGE_BOOST_PERCENT_VER_1 }

    pub type NV_VOLTAGE_BOOST_PERCENT = NV_VOLTAGE_BOOST_PERCENT_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetCoreVoltageBoostPercent(pPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *mut NV_VOLTAGE_BOOST_PERCENT) -> NvAPI_Status;
    }

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_SetCoreVoltageBoostPercent(pPhysicalGPU: NvPhysicalGpuHandle, pVoltboostPercent: *const NV_VOLTAGE_BOOST_PERCENT) -> NvAPI_Status;
    }

    nvstruct! {
        pub struct NV_VFP_CURVE_GPU_ENTRY {
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
    pub type NV_VFP_CURVE_MEM_ENTRY = NV_VFP_CURVE_GPU_ENTRY;
    /*nvstruct! {
        pub struct NV_VFP_CURVE_MEM_ENTRY {
            pub a: u32, // 1 for idle values?
            pub freq_kHz: u32,
            pub voltage_uV: u32,
            pub d: u32,
            pub e: u32,
            pub f: u32,
            pub g: u32,
        }
    }*/

    debug_array_impl! { [NV_VFP_CURVE_GPU_ENTRY; 80] }
    debug_array_impl! { [u32; 1064] }

    nvstruct! {
        pub struct NV_VFP_CURVE_V1 {
            pub version: u32,
            pub mask: [u32; 4], // 80 bits
            pub unknown: [u32; 12],
            pub gpuEntries: Array<[NV_VFP_CURVE_GPU_ENTRY; 80]>,
            pub memEntries: [NV_VFP_CURVE_MEM_ENTRY; 23],
            pub unknown2: Array<[u32; 1064]>,
        }
    }

    nvversion! { NV_VFP_CURVE_VER_1(NV_VFP_CURVE_V1 = 0x1c28, 1) }
    nvversion! { NV_VFP_CURVE_VER = NV_VFP_CURVE_VER_1 }

    pub type NV_VFP_CURVE = NV_VFP_CURVE_V1;

    nvapi! {
        /// Pascal only
        pub unsafe fn NvAPI_GPU_GetVFPCurve(pPhysicalGPU: NvPhysicalGpuHandle, pVfpCurve: *mut NV_VFP_CURVE) -> NvAPI_Status;
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
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetInfo(pPhysicalGPU: NvPhysicalGpuHandle, pPowerInfo: *mut NV_GPU_POWER_INFO) -> NvAPI_Status;
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
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesGetStatus(pPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *mut NV_GPU_POWER_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientPowerPoliciesSetStatus(pPhysicalGPU: NvPhysicalGpuHandle, pPowerStatus: *const NV_GPU_POWER_STATUS) -> NvAPI_Status;
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
        pub unsafe fn NvAPI_GPU_ClientPowerTopologyGetStatus(pPhysicalGPU: NvPhysicalGpuHandle, pPowerTopo: *mut NV_GPU_POWER_TOPO) -> NvAPI_Status;
    }
}
