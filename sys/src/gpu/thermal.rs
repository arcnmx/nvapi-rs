use crate::prelude_::*;

nvenum! {
    /// Used in NV_GPU_THERMAL_SETTINGS
    pub enum NV_THERMAL_TARGET / ThermalTarget {
        NVAPI_THERMAL_TARGET_NONE / None = 0,
        /// GPU core temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_GPU / Gpu = 1,
        /// GPU memory temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_MEMORY / Memory = 2,
        /// GPU power supply temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_POWER_SUPPLY / PowerSupply = 4,
        /// GPU board ambient temperature requires NvPhysicalGpuHandle
        NVAPI_THERMAL_TARGET_BOARD / Board = 8,
        /// Visual Computing Device Board temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_BOARD / VcdBoard = 9,
        /// Visual Computing Device Inlet temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_INLET / VcdInlet = 10,
        /// Visual Computing Device Outlet temperature requires NvVisualComputingDeviceHandle
        NVAPI_THERMAL_TARGET_VCD_OUTLET / VcdOutlet = 11,
        NVAPI_THERMAL_TARGET_ALL / All = 15,
        NVAPI_THERMAL_TARGET_UNKNOWN / Unknown = -1,
    }
}

nvenum_display! {
    ThermalTarget => {
        Gpu = "Core",
        Memory = "Memory",
        PowerSupply = "VRM",
        VcdBoard = "VCD Board",
        VcdInlet = "VCD Inlet",
        VcdOutlet = "VCD Outlet",
        _ = _,
    }
}

nvenum! {
    /// NV_GPU_THERMAL_SETTINGS
    pub enum NV_THERMAL_CONTROLLER / ThermalController {
        NVAPI_THERMAL_CONTROLLER_NONE / None = 0,
        NVAPI_THERMAL_CONTROLLER_GPU_INTERNAL / GpuInternal = 1,
        NVAPI_THERMAL_CONTROLLER_ADM1032 / ADM1032 = 2,
        NVAPI_THERMAL_CONTROLLER_MAX6649 / MAX6649 = 3,
        NVAPI_THERMAL_CONTROLLER_MAX1617 / MAX1617 = 4,
        NVAPI_THERMAL_CONTROLLER_LM99 / LM99 = 5,
        NVAPI_THERMAL_CONTROLLER_LM89 / LM89 = 6,
        NVAPI_THERMAL_CONTROLLER_LM64 / LM64 = 7,
        NVAPI_THERMAL_CONTROLLER_ADT7473 / ADT7473 = 8,
        NVAPI_THERMAL_CONTROLLER_SBMAX6649 / SBMAX6649 = 9,
        NVAPI_THERMAL_CONTROLLER_VBIOSEVT / VBIOSEVT = 10,
        NVAPI_THERMAL_CONTROLLER_OS / OS = 11,
        NVAPI_THERMAL_CONTROLLER_UNKNOWN / Unknown = -1,
    }
}

nvenum_display! {
    ThermalController => {
        GpuInternal = "Internal",
        _ = _,
    }
}

pub const NVAPI_MAX_THERMAL_SENSORS_PER_GPU: usize = 3;

nvstruct! {
    /// Used in NvAPI_GPU_GetThermalSettings()
    pub struct NV_GPU_THERMAL_SETTINGS_V1 {
        /// structure version
        pub version: NvVersion,
        /// number of associated thermal sensors
        pub count: u32,
        pub sensor: [NV_GPU_THERMAL_SETTINGS_SENSOR; NVAPI_MAX_THERMAL_SENSORS_PER_GPU],
    }
}

nvstruct! {
    /// Anonymous struct in NV_GPU_THERMAL_SETTINGS
    pub struct NV_GPU_THERMAL_SETTINGS_SENSOR {
        /// internal, ADM1032, MAX6649...
        pub controller: NV_THERMAL_CONTROLLER,
        /// The min default temperature value of the thermal sensor in degree Celsius
        pub defaultMinTemp: i32,
        /// The max default temperature value of the thermal sensor in degree Celsius
        pub defaultMaxTemp: i32,
        /// The current temperature value of the thermal sensor in degree Celsius
        pub currentTemp: i32,
        /// Thermal sensor targeted @ GPU, memory, chipset, powersupply, Visual Computing Device, etc.
        pub target: NV_THERMAL_TARGET,
    }
}

pub type NV_GPU_THERMAL_SETTINGS_V2 = NV_GPU_THERMAL_SETTINGS_V1; // the only difference is the _SENSOR struct uses i32 instead of u32 fields
pub type NV_GPU_THERMAL_SETTINGS = NV_GPU_THERMAL_SETTINGS_V2;

const NV_GPU_THERMAL_SETTINGS_V1_SIZE: usize = 4 * 2 + (4 * 5) * NVAPI_MAX_THERMAL_SENSORS_PER_GPU;
nvversion! { NV_GPU_THERMAL_SETTINGS_VER_1(NV_GPU_THERMAL_SETTINGS_V1 = NV_GPU_THERMAL_SETTINGS_V1_SIZE, 1) }
nvversion! { NV_GPU_THERMAL_SETTINGS_VER_2(NV_GPU_THERMAL_SETTINGS_V2 = NV_GPU_THERMAL_SETTINGS_V1_SIZE, 2) }
nvversion! { NV_GPU_THERMAL_SETTINGS_VER = NV_GPU_THERMAL_SETTINGS_VER_2 }

nvapi! {
    pub type GPU_GetThermalSettingsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, sensorIndex: u32, pThermalSettings: *mut NV_GPU_THERMAL_SETTINGS) -> NvAPI_Status;

    /// This function retrieves the thermal information of all thermal sensors or specific thermal sensor associated with the selected GPU.
    ///
    /// Thermal sensors are indexed 0 to NVAPI_MAX_THERMAL_SENSORS_PER_GPU-1.
    /// - To retrieve specific thermal sensor info, set the sensorIndex to the required thermal sensor index.
    /// - To retrieve info for all sensors, set sensorIndex to NVAPI_THERMAL_TARGET_ALL.
    pub unsafe fn NvAPI_GPU_GetThermalSettings;
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;

    pub const NVAPI_MAX_THERMAL_INFO_ENTRIES: usize = 4;

    nvenum! {
        pub enum NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID / ThermalPolicyId {
            NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID_DEFAULT / Default = 1,
        }
    }

    nvenum_display! {
        ThermalPolicyId => {
            Default = "GPU Thermal Policy",
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_V2 {
            pub policy_id: NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID,
            pub unknown: u32,
            pub minTemp: i32,
            pub defaultTemp: i32,
            pub maxTemp: i32,
            pub defaultFlags: u32,
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_SIZE: usize = 4 * 6;

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2 {
            pub version: NvVersion,
            pub count: u8,
            pub flags: u8,
            pub padding: [u8; 2],
            pub entries: [NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_V2; NVAPI_MAX_THERMAL_INFO_ENTRIES],
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2_SIZE: usize = 4 * 2 + NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_SIZE * NVAPI_MAX_THERMAL_INFO_ENTRIES;

    impl NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_V2] {
            &self.entries[..self.count as usize]
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3 {
            pub policy_id: NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID,
            pub flags: u32,
            pub unknown: u32,
            pub minTemp: i32,
            pub defaultTemp: i32,
            pub maxTemp: i32,
            pub defaultFlags: u32,
            pub padding0: Padding<[u32; 16]>,
            pub pff_curve: NV_GPU_CLIENT_PFF_CURVE_V1,
            pub padding1: Padding<[u32; 49]>,
        }
    }

    impl NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3 {
        pub fn has_pff(&self) -> bool {
            self.flags == 1
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3 {
            pub version: NvVersion,
            pub flags: u8,
            pub count: u8,
            pub padding: Padding<[u8; 2]>,
            pub entries: [NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3; NVAPI_MAX_THERMAL_INFO_ENTRIES],
        }
    }

    impl NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3] {
            &self.entries[..self.count as usize]
        }

        pub fn valid(&self) -> bool {
            self.flags & 1 != 0
        }
    }

    pub type NV_GPU_CLIENT_THERMAL_POLICIES_INFO = NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3;

    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_INFO_VER_2(NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2 = NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2_SIZE, 2) }
    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_INFO_VER_3(NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3 = 1400, 3) }
    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_INFO_VER = NV_GPU_CLIENT_THERMAL_POLICIES_INFO_VER_3 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesGetInfo(hPhysicalGPU: NvPhysicalGpuHandle, pThermalInfo: *mut NV_GPU_CLIENT_THERMAL_POLICIES_INFO) -> NvAPI_Status;
    }

    pub const NVAPI_MAX_THERMAL_LIMIT_ENTRIES: usize = 4;

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_V2 {
            pub policy_id: NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID,
            /// shifted 8 bits
            pub temp_limit_C: u32,
            pub pstate: crate::gpu::pstate::NV_GPU_PERF_PSTATE_ID,
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_SIZE: usize = 4 * 3;

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: [NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_V2; NVAPI_MAX_THERMAL_LIMIT_ENTRIES],
        }
    }
    const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2_SIZE: usize = 4 * 2 + NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_SIZE * NVAPI_MAX_THERMAL_LIMIT_ENTRIES;

    impl NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_V2] {
            &self.entries[..self.count as usize]
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3 {
            pub policy_id: NV_GPU_CLIENT_THERMAL_POLICIES_POLICY_ID,
            pub flags: u32,
            /// shifted 8 bits
            ///
            /// aka iT0X
            pub temp_limit_C: u32,
            /// aka bRemoveTdpLimit
            pub remove_tdp_limit: BoolU32,
            pub padding0: Padding<[u32; 17]>,
            pub pff_curve: NV_GPU_CLIENT_PFF_CURVE_V1, // 92-8
            /// aka uiT{1,2,3}OCY
            pub pff_freqs: [u32; 3], // 152-8 ~ 160-8
            pub padding1: Padding<[u32; 45]>,
        }
    }

    impl NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3 {
        pub fn has_pff(&self) -> bool {
            self.flags == 1
        }

        pub fn set_pff(&mut self, enabled: bool) {
            self.flags = if enabled { 1 } else { 0 }
        }

        pub fn pff_freqs(&self) -> &[u32] {
            let count = self.pff_curve.points().len();
            &self.pff_freqs[..count]
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3 {
            pub version: NvVersion,
            pub count: u32,
            pub entries: [NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3; NVAPI_MAX_THERMAL_LIMIT_ENTRIES],
        }
    }

    impl NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3 {
        pub fn entries(&self) -> &[NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3] {
            &self.entries[..self.count as usize]
        }
    }

    pub type NV_GPU_CLIENT_THERMAL_POLICIES_STATUS = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3;

    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER_2(NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2_SIZE, 2) }
    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER_3(NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3 = 1352, 3) }
    nvversion! { NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER = NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_VER_3 }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesGetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pThermalLimit: *mut NV_GPU_CLIENT_THERMAL_POLICIES_STATUS) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_ClientThermalPoliciesSetStatus(hPhysicalGPU: NvPhysicalGpuHandle, pThermalLimit: *const NV_GPU_CLIENT_THERMAL_POLICIES_STATUS) -> NvAPI_Status;
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_PFF_CURVE_POINT_V1 {
            pub enabled: BoolU32,
            /// uiT{1,2,3}Y
            pub uiT_Y: u32,
            /// iT{1,2,3}X
            pub temp: u32,
            pub padding: Padding<[u32; 2]>,
        }
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_PFF_CURVE_V1 {
            pub points: [NV_GPU_CLIENT_PFF_CURVE_POINT_V1; 3],
        }
    }

    impl NV_GPU_CLIENT_PFF_CURVE_V1 {
        pub fn points(&self) -> &[NV_GPU_CLIENT_PFF_CURVE_POINT_V1] {
            let count = self.points.iter().take_while(|p| p.enabled.get()).count();
            &self.points[..count]
        }
    }
}
