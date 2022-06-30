use crate::prelude_::*;

nvapi! {
    pub type GPU_GetTachReadingFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pValue: *mut u32) -> NvAPI_Status;

    /// This API retrieves the fan speed tachometer reading for the specified physical GPU.
    pub unsafe fn NvAPI_GPU_GetTachReading;
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;

    pub const NVAPI_MIN_COOLER_LEVEL: usize = 0;
    pub const NVAPI_MAX_COOLER_LEVEL: usize = 100;
    pub const NVAPI_MAX_COOLER_LEVELS: usize = 24;
    pub const NVAPI_MAX_COOLERS_PER_GPU: usize = 3;

    nvenum! {
        pub enum NV_COOLER_TYPE / CoolerType {
            NVAPI_COOLER_TYPE_NONE / None = 0,
            NVAPI_COOLER_TYPE_FAN / Fan = 1,
            NVAPI_COOLER_TYPE_WATER / Water = 2,
            NVAPI_COOLER_TYPE_LIQUID_NO2 / LiquidNO2 = 3,
        }
    }

    nvenum_display! {
        CoolerType => _
    }

    nvenum! {
        pub enum NV_COOLER_CONTROLLER / CoolerController {
            NVAPI_COOLER_CONTROLLER_NONE / None = 0,
            NVAPI_COOLER_CONTROLLER_ADI / ADI = 1,
            NVAPI_COOLER_CONTROLLER_INTERNAL / Internal = 2,
        }
    }

    nvenum_display! {
        CoolerController => _
    }

    nvenum! {
        pub enum NV_COOLER_POLICY / CoolerPolicy {
            NVAPI_COOLER_POLICY_NONE / None = 0,
            /// Manual adjustment of cooler level. Gets applied right away independent of temperature or performance level.
            NVAPI_COOLER_POLICY_MANUAL / Manual = 1,
            /// GPU performance controls the cooler level.
            NVAPI_COOLER_POLICY_PERF / Performance = 2,
            /// Discrete thermal levels control the cooler level.
            NVAPI_COOLER_POLICY_TEMPERATURE_DISCRETE / TemperatureDiscrete = 4,
            /// Cooler level adjusted at continuous thermal levels.
            NVAPI_COOLER_POLICY_TEMPERATURE_CONTINUOUS / TemperatureContinuous = 8,
            /// Hybrid of performance and temperature levels.
            NVAPI_COOLER_POLICY_HYBRID / Hybrid = 9, // are you sure this isn't just a bitmask?
            /// Fan turns off at idle, default of MSI Gaming X
            NVAPI_COOLER_POLICY_SILENT / Silent = 16,
            /// Apparently a default of some GPUs
            NVAPI_COOLER_POLICY_UNKNOWN_32 / Unknown32 = 32,
        }
    }

    nvenum_display! {
        CoolerPolicy => {
            TemperatureDiscrete = "Discrete Thermal",
            TemperatureContinuous = "Continuous Thermal",
            Silent = "Silent",
            _ = _,
        }
    }

    nvenum! {
        pub enum NV_COOLER_TARGET / CoolerTarget {
            NVAPI_COOLER_TARGET_NONE / None = 0,
            NVAPI_COOLER_TARGET_GPU / GPU = 1,
            NVAPI_COOLER_TARGET_MEMORY / Memory = 2,
            NVAPI_COOLER_TARGET_POWER_SUPPLY / PowerSupply = 4,
            /// This cooler cools all of the components related to its target gpu.
            NVAPI_COOLER_TARGET_ALL / All = 7,
        }
    }

    nvenum_display! {
        CoolerTarget => {
            GPU = "Core",
            PowerSupply = "VRM",
            _ = _,
        }
    }

    nvenum! {
        pub enum NV_COOLER_CONTROL / CoolerControl {
            NVAPI_COOLER_CONTROL_NONE / None = 0,
            /// ON/OFF
            NVAPI_COOLER_CONTROL_TOGGLE / Toggle = 1,
            /// Suppports variable control.
            NVAPI_COOLER_CONTROL_VARIABLE / Variable = 2,
        }
    }

    nvenum_display! {
        CoolerControl => _
    }

    nvenum! {
        pub enum NV_COOLER_ACTIVITY_LEVEL / CoolerActivityLevel {
            NVAPI_INACTIVE / Inactive = 0,
            NVAPI_ACTIVE / Active = 1,
        }
    }

    impl CoolerActivityLevel {
        pub fn get(&self) -> bool {
            match *self {
                CoolerActivityLevel::Active => true,
                CoolerActivityLevel::Inactive => false,
            }
        }
    }

    nvstruct! {
        pub struct NV_GPU_COOLER_SETTINGS_COOLER {
            /// type of cooler - FAN, WATER, LIQUID_NO2...
            pub type_: NV_COOLER_TYPE,
            /// internal, ADI...
            pub controller: NV_COOLER_CONTROLLER,
            /// the min default value % of the cooler
            pub defaultMinLevel: u32,
            /// the max default value % of the cooler
            pub defaultMaxLevel: u32,
            /// the current allowed min value % of the cooler
            pub currentMinLevel: u32,
            /// the current allowed max value % of the cooler
            pub currentMaxLevel: u32,
            /// the current value % of the cooler
            pub currentLevel: u32,
            /// cooler control policy - auto-perf, auto-thermal, manual, hybrid...
            pub defaultPolicy: NV_COOLER_POLICY,
            /// cooler control policy - auto-perf, auto-thermal, manual, hybrid...
            pub currentPolicy: NV_COOLER_POLICY,
            /// cooling target - GPU, memory, chipset, powersupply, canoas...
            pub target: NV_COOLER_TARGET,
            /// toggle or variable
            pub controlType: NV_COOLER_CONTROL,
            /// is the cooler active - fan spinning...
            pub active: NV_COOLER_ACTIVITY_LEVEL,
        }
    }

    nvstruct! {
        pub struct NV_GPU_COOLER_SETTINGS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub cooler: [NV_GPU_COOLER_SETTINGS_COOLER; NVAPI_MAX_COOLERS_PER_GPU],
        }
    }

    const NV_GPU_COOLER_SETTINGS_COOLER_SIZE: usize = 4 * 12;

    nvversion! { NV_GPU_COOLER_SETTINGS_VER_1(NV_GPU_COOLER_SETTINGS_V1 = 4 * 2 + NV_GPU_COOLER_SETTINGS_COOLER_SIZE * NVAPI_MAX_COOLERS_PER_GPU, 1) }
    nvversion! { NV_GPU_COOLER_SETTINGS_VER = NV_GPU_COOLER_SETTINGS_VER_1 }

    pub type NV_GPU_COOLER_SETTINGS = NV_GPU_COOLER_SETTINGS_V1;

    nvapi! {
        pub type GPU_GetCoolerSettingsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: u32, pCoolerInfo: *mut NV_GPU_COOLER_SETTINGS) -> NvAPI_Status;

        /// Undocumented function.
        /// Retrieves the cooler information of all coolers or a specific cooler associated with the selected GPU.
        ///
        /// Coolers are indexed 0 to NVAPI_MAX_COOLERS_PER_GPU-1.
        /// To retrieve specific cooler info set the coolerIndex to the appropriate cooler index.
        /// To retrieve info for all cooler set coolerIndex to NVAPI_COOLER_TARGET_ALL.
        pub unsafe fn NvAPI_GPU_GetCoolerSettings;
    }

    nvstruct! {
        pub struct NV_GPU_SETCOOLER_LEVEL_COOLER {
            /// the new value % of the cooler
            pub currentLevel: u32,
            /// the new cooler control policy - auto-perf, auto-thermal, manual, hybrid...
            pub currentPolicy: NV_COOLER_POLICY,
        }
    }

    nvstruct! {
        pub struct NV_GPU_SETCOOLER_LEVEL_V1 {
            pub version: NvVersion,
            pub cooler: [NV_GPU_SETCOOLER_LEVEL_COOLER; NVAPI_MAX_COOLERS_PER_GPU],
        }
    }

    const NV_GPU_SETCOOLER_LEVEL_COOLER_SIZE: usize = 4 * 2;

    nvversion! { NV_GPU_SETCOOLER_LEVEL_VER_1(NV_GPU_SETCOOLER_LEVEL_V1 = 4 + NV_GPU_SETCOOLER_LEVEL_COOLER_SIZE * NVAPI_MAX_COOLERS_PER_GPU, 1) }
    nvversion! { NV_GPU_SETCOOLER_LEVEL_VER = NV_GPU_SETCOOLER_LEVEL_VER_1 }

    pub type NV_GPU_SETCOOLER_LEVEL = NV_GPU_SETCOOLER_LEVEL_V1;

    nvapi! {
        pub type GPU_SetCoolerLevelsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: u32, pCoolerLevels: *const NV_GPU_SETCOOLER_LEVEL) -> NvAPI_Status;

        /// Undocumented function.
        /// Set the cooler levels for all coolers or a specific cooler associated with the selected GPU.
        ///
        /// Coolers are indexed 0 to NVAPI_MAX_COOLERS_PER_GPU-1. Every cooler level with non-zero currentpolicy gets applied.
        ///
        /// The new level should be in the range of minlevel and maxlevel retrieved from GetCoolerSettings API or between
        /// and NVAPI_MIN_COOLER_LEVEL to MAX_COOLER_LEVEL.
        /// To set level for a specific cooler set the coolerIndex to the appropriate cooler index.
        /// To set level for all coolers set coolerIndex to NVAPI_COOLER_TARGET_ALL.
        ///
        /// NOTE: To lock the fan speed independent of the temperature or performance changes set the cooler currentPolicy to
        /// NVAPI_COOLER_POLICY_MANUAL else set it to the current policy retrieved from the GetCoolerSettings API.
        pub unsafe fn NvAPI_GPU_SetCoolerLevels;
    }

    nvapi! {
        pub type GPU_RestoreCoolerSettingsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: *const u32, coolerCount: u32) -> NvAPI_Status;

        /// Undocumented function.
        /// Restore the modified cooler settings to NVIDIA defaults.
        ///
        /// pCoolerIndex: Array containing absolute cooler indexes to restore. Pass NULL restore all coolers.
        ///
        /// coolerCount: Number of coolers to restore.
        pub unsafe fn NvAPI_GPU_RestoreCoolerSettings;
    }

    nvstruct! {
        pub struct NV_GPU_COOLER_POLICY_LEVEL {
            /// level indicator for a policy
            pub levelId: u32,
            /// new cooler level for the selected policy level indicator.
            pub currentLevel: u32,
            /// default cooler level for the selected policy level indicator.
            pub defaultLevel: u32,
        }
    }

    const NV_GPU_COOLER_POLICY_LEVEL_SIZE: usize = 4 * 3;

    nvstruct! {
        pub struct NV_GPU_COOLER_POLICY_TABLE_V1 {
            /// structure version
            pub version: NvVersion,
            /// selected policy to update the cooler levels for, example NVAPI_COOLER_POLICY_PERF
            pub policy: NV_COOLER_POLICY,
            pub policyCoolerLevel: [NV_GPU_COOLER_POLICY_LEVEL; NVAPI_MAX_COOLER_LEVELS],
        }
    }

    nvversion! { NV_GPU_COOLER_POLICY_TABLE_VER_1(NV_GPU_COOLER_POLICY_TABLE_V1 = 4 * 2 + NV_GPU_COOLER_POLICY_LEVEL_SIZE * NVAPI_MAX_COOLER_LEVELS, 1) }
    nvversion! { NV_GPU_COOLER_POLICY_TABLE_VER = NV_GPU_COOLER_POLICY_TABLE_VER_1 }

    pub type NV_GPU_COOLER_POLICY_TABLE = NV_GPU_COOLER_POLICY_TABLE_V1;

    nvapi! {
        pub type GPU_GetCoolerPolicyTableFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: u32, pCoolerTable: *mut NV_GPU_COOLER_POLICY_TABLE, count: *mut u32) -> NvAPI_Status;

        /// Undocumented function.
        /// Retrieves the table of cooler and policy levels for the selected policy. Supported only for NVAPI_COOLER_POLICY_PERF.
        pub unsafe fn NvAPI_GPU_GetCoolerPolicyTable;
    }

    nvapi! {
        pub type GPU_SetCoolerPolicyTableFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: u32, pCoolerTable: *const NV_GPU_COOLER_POLICY_TABLE, count: u32) -> NvAPI_Status;

        /// Undocumented function.
        /// Restore the modified cooler settings to NVIDIA defaults. Supported only for NVAPI_COOLER_POLICY_PERF.
        ///
        /// pCoolerTable: Updated table of policy levels and associated cooler levels. Every non-zero policy level gets updated.
        ///
        /// count: Number of valid levels in the policy table.
        pub unsafe fn NvAPI_GPU_SetCoolerPolicyTable;
    }

    nvapi! {
        pub type GPU_RestoreCoolerPolicyTableFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: *const u32, coolerCount: u32, policy: NV_COOLER_POLICY) -> NvAPI_Status;

        /// Undocumented function.
        /// Restores the perf table policy levels to the defaults.
        ///
        /// pCoolerIndex: Array containing absolute cooler indexes to restore. Pass NULL restore all coolers.
        ///
        /// coolerCount: Number of coolers to restore.
        pub unsafe fn NvAPI_GPU_RestoreCoolerPolicyTable;
    }
}
