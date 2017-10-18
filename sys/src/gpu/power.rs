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
            pub percent: i32,
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

    nvstruct! {
        pub struct NV_VFP_CURVE_MEM_ENTRY {
            pub a: u32, // 1 for idle values?
            pub freq_kHz: u32,
            pub voltage_uV: u32,
            pub d: u32,
            pub e: u32,
            pub f: u32,
            pub g: u32,
        }
    }

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
}
