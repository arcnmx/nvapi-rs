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
    pub const NVAPI_MAX_COOLERS_PER_GPU_VER2: usize = 20;
    pub const NVAPI_MAX_COOLERS_PER_GPU_VER3: usize = NVAPI_MAX_COOLERS_PER_GPU_VER2;
    pub const NVAPI_MAX_COOLERS_PER_GPU_VER4: usize = NVAPI_MAX_COOLERS_PER_GPU_VER3;

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
            NVAPI_COOLER_POLICY_TEMPERATURE_CONTINUOUS_SW / TemperatureContinuousSoftware = 16,
            /// Apparently a default of some GPUs
            NVAPI_COOLER_POLICY_DEFAULT / Default = 32,
        }
    }

    nvenum_display! {
        CoolerPolicy => {
            TemperatureDiscrete = "Thermal (Discrete)",
            TemperatureContinuous = "Thermal",
            TemperatureContinuousSoftware = "Thermal (Silent)",
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
        pub struct NV_GPU_GETCOOLER_SETTING_V1 {
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
        pub struct NV_GPU_GETCOOLER_SETTINGS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub cooler: Array<[NV_GPU_GETCOOLER_SETTING_V1; NVAPI_MAX_COOLERS_PER_GPU]>,
        }
    }

    impl NV_GPU_GETCOOLER_SETTINGS_V1 {
        pub fn coolers(&self) -> &[NV_GPU_GETCOOLER_SETTING_V1] {
            &self.cooler[..self.count as usize]
        }
    }

    nvstruct! {
        pub struct NV_COOLER_TACHOMETER {
            /// current tachometer reading in RPM
            pub speedRPM: u32,
            /// cooler supports tach function?
            pub bSupported: BoolU32,
            /// Maximum RPM corresponding to 100% defaultMaxLevel
            pub maxSpeedRPM: u32,
            /// Minimum RPM corresponding to 100% defaultMinLevel
            pub minSpeedRPM: u32,
        }
    }

    nvstruct! {
        pub struct NV_GPU_GETCOOLER_SETTING_V3 {
            pub v1: NV_GPU_GETCOOLER_SETTING_V1,
            /// cooler tachometer info
            pub tachometer: NV_COOLER_TACHOMETER,
        }
    }

    nvinherit! { struct NV_GPU_GETCOOLER_SETTING_V3(v1: NV_GPU_GETCOOLER_SETTING_V1) }

    nvstruct! {
        pub struct NV_GPU_GETCOOLER_SETTINGS_V3 {
            /// structure version
            pub version: NvVersion,
            /// number of associated coolers with the selected GPU
            pub count: u32,
            pub cooler: Array<[NV_GPU_GETCOOLER_SETTING_V3; NVAPI_MAX_COOLERS_PER_GPU_VER3]>,
        }
    }

    impl NV_GPU_GETCOOLER_SETTINGS_V3 {
        pub fn coolers(&self) -> &[NV_GPU_GETCOOLER_SETTING_V3] {
            &self.cooler[..self.count as usize]
        }
    }

    nvstruct! {
        pub struct NV_GPU_GETCOOLER_SETTING_V4 {
            pub v3: NV_GPU_GETCOOLER_SETTING_V3,
            pub unknown: u32,
        }
    }

    nvinherit! { struct NV_GPU_GETCOOLER_SETTING_V4(v3: NV_GPU_GETCOOLER_SETTING_V3) }

    nvstruct! {
        pub struct NV_GPU_GETCOOLER_SETTINGS_V4 {
            pub version: NvVersion,
            pub count: u32,
            pub cooler: Array<[NV_GPU_GETCOOLER_SETTING_V4; NVAPI_MAX_COOLERS_PER_GPU_VER4]>,
        }
    }

    impl NV_GPU_GETCOOLER_SETTINGS_V4 {
        pub fn coolers(&self) -> &[NV_GPU_GETCOOLER_SETTING_V4] {
            &self.cooler[..self.count as usize]
        }
    }

    nvversion! { NV_GPU_GETCOOLER_SETTINGS_V1(1) = 152 }
    nvversion! { NV_GPU_GETCOOLER_SETTINGS_V3(3) = 1288 }
    nvversion! { @=NV_GPU_GETCOOLER_SETTINGS NV_GPU_GETCOOLER_SETTINGS_V4(4) = 1368 }

    nvapi! {
        pub type GPU_GetCoolerSettingsFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolerIndex: u32, pCoolerInfo: *mut NV_GPU_GETCOOLER_SETTINGS) -> NvAPI_Status;

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
            pub cooler: Array<[NV_GPU_SETCOOLER_LEVEL_COOLER; NVAPI_MAX_COOLERS_PER_GPU]>,
        }
    }

    nvversion! { @=NV_GPU_SETCOOLER_LEVEL NV_GPU_SETCOOLER_LEVEL_V1(1) }

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

    nvstruct! {
        pub struct NV_GPU_COOLER_POLICY_TABLE_V1 {
            /// structure version
            pub version: NvVersion,
            /// selected policy to update the cooler levels for, example NVAPI_COOLER_POLICY_PERF
            pub policy: NV_COOLER_POLICY,
            pub policyCoolerLevel: Array<[NV_GPU_COOLER_POLICY_LEVEL; NVAPI_MAX_COOLER_LEVELS]>,
        }
    }

    nvversion! { @=NV_GPU_COOLER_POLICY_TABLE NV_GPU_COOLER_POLICY_TABLE_V1(1) }

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

    nvbits! {
        pub enum NV_FAN_ARBITER_INFO_FLAGS / FanArbiterInfoFlags {
            /// Supports full fan stop
            NV_FAN_ARBITER_INFO_FLAGS_FAN_STOP / FAN_STOP = 1,
            /// Fan stop is enabled by default
            NV_FAN_ARBITER_INFO_FLAGS_FAN_STOP_DEFAULT / FAN_STOP_DEFAULT = 2,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITER_INFO_V1 {
            pub unknown: u32,
            pub flags: NV_FAN_ARBITER_INFO_FLAGS,
            pub arbiter_index: u32,
            pub padding: Padding<[u32; 40/4-3]>,
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITER_INFO_V1 {
        pub fn flags(&self) -> FanArbiterInfoFlags {
            FanArbiterInfoFlags::from_bits_truncate(self.flags)
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITERS_INFO_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub padding: Padding<[u32; 28/4]>,
            pub arbiters: Array<[NV_GPU_CLIENT_FAN_ARBITER_INFO_V1; 32]>, // offset 36
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITERS_INFO_V1 {
        pub fn arbiters(&self) -> &[NV_GPU_CLIENT_FAN_ARBITER_INFO_V1] {
            &self.arbiters[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_ARBITERS_INFO NV_GPU_CLIENT_FAN_ARBITERS_INFO_V1(1) = 1316 }

    nvapi! {
        pub type GPU_ClientFanArbitersGetInfoFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, arbiter: *mut NV_GPU_CLIENT_FAN_ARBITERS_INFO) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanArbitersGetInfo;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1 {
            pub unknown0: u32,
            pub unknown1: u32,
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1 {
        pub fn fan_stop_active(&self) -> bool {
            self.unknown1 != 0
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITERS_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub padding: Padding<[u32; 28/4]>,
            pub arbiters: Array<[NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1; 32]>, // offset 36
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITERS_STATUS_V1 {
        pub fn arbiters(&self) -> &[NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1] {
            &self.arbiters[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_ARBITERS_STATUS NV_GPU_CLIENT_FAN_ARBITERS_STATUS_V1(1) = 292 }

    nvapi! {
        pub type GPU_ClientFanArbitersGetStatusFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, arbiter: *mut NV_GPU_CLIENT_FAN_ARBITERS_STATUS) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanArbitersGetStatus;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1 {
            pub arbiter_index: u32,
            pub flags: NV_FAN_ARBITER_CONTROL_FLAGS,
        }
    }

    nvbits! {
        pub enum NV_FAN_ARBITER_CONTROL_FLAGS / FanArbiterControlFlags {
            /// Fan stop enabled
            NV_FAN_ARBITER_CONTROL_FLAGS_FAN_STOP / FAN_STOP = 1,
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1 {
        pub fn flags(&self) -> FanArbiterControlFlags {
            FanArbiterControlFlags::from_bits_truncate(self.flags)
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_ARBITERS_CONTROL_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub padding: Padding<[u32; 28/4]>,
            pub arbiters: Array<[NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1; 32]>, // offset 36
        }
    }

    impl NV_GPU_CLIENT_FAN_ARBITERS_CONTROL_V1 {
        pub fn arbiters(&self) -> &[NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1] {
            &self.arbiters[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_ARBITERS_CONTROL NV_GPU_CLIENT_FAN_ARBITERS_CONTROL_V1(1) = 292 }

    nvapi! {
        pub type GPU_ClientFanArbitersGetControlFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, arbiter: *mut NV_GPU_CLIENT_FAN_ARBITERS_CONTROL) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanArbitersGetControl;
    }

    nvapi! {
        pub type GPU_ClientFanArbitersSetControlFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, arbiter: *const NV_GPU_CLIENT_FAN_ARBITERS_CONTROL) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanArbitersSetControl;
    }

    nvenum! {
        pub enum NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID / FanCoolerId {
            NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID_NONE / None = 0,
            NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID_1 / Cooler1 = 1,
            NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID_2 / Cooler2 = 2,
        }
    }

    nvenum_display! {
        FanCoolerId => {
            Cooler1 = "Fan1",
            Cooler2 = "Fan2",
            _ = _,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_COOLER_INFO_V1 {
            pub cooler_id: NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID,
            pub tach_supported: BoolU32,
            pub tach_min_rpm: u32,
            pub tach_max_rpm: u32,
            pub padding: Padding<[u32; 8]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_COOLERS_INFO_V1 {
            pub version: NvVersion,
            pub flags: u32,
            pub count: u32,
            pub padding: Padding<[u32; 8]>,
            pub coolers: Array<[NV_GPU_CLIENT_FAN_COOLER_INFO_V1; 32]>, // offset 44
        }
    }

    impl NV_GPU_CLIENT_FAN_COOLERS_INFO_V1 {
        pub fn valid(&self) -> bool {
            self.flags & 1 != 0
        }

        pub fn coolers(&self) -> &[NV_GPU_CLIENT_FAN_COOLER_INFO_V1] {
            &self.coolers[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_COOLERS_INFO NV_GPU_CLIENT_FAN_COOLERS_INFO_V1(1) = 0x62c }

    nvapi! {
        pub type GPU_ClientFanCoolersGetInfoFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolers: *mut NV_GPU_CLIENT_FAN_COOLERS_INFO) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanCoolersGetInfo;
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_COOLER_STATUS_V1 {
            pub cooler_id: NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID,
            pub tach_rpm: u32,
            pub level_minimum: u32,
            pub level_maximum: u32,
            pub level: u32,
            pub padding: Padding<[u32; 8]>,
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1 {
            pub version: NvVersion,
            pub count: u32,
            pub padding: Padding<[u32; 8]>,
            pub coolers: Array<[NV_GPU_CLIENT_FAN_COOLER_STATUS_V1; 32]>,
        }
    }

    impl NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1 {
        pub fn coolers(&self) -> &[NV_GPU_CLIENT_FAN_COOLER_STATUS_V1] {
            &self.coolers[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_COOLERS_STATUS NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1(1) = 0x6a8 }

    nvapi! {
        pub type GPU_ClientFanCoolersGetStatusFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolers: *mut NV_GPU_CLIENT_FAN_COOLERS_STATUS) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanCoolersGetStatus;
    }

    nvstruct! {
        #[derive(Default)]
        pub struct NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1 {
            pub cooler_id: NV_GPU_CLIENT_FAN_COOLERS_COOLER_ID,
            pub level: u32,
            pub flags: u32,
            pub padding: Padding<[u32; 8]>,
        }
    }

    impl NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1 {
        pub fn manual(&self) -> bool {
            self.flags & 1 != 0
        }

        pub fn set_manual(&mut self, manual: bool) {
            self.flags = self.flags & 0xfffffffe | if manual { 1 } else { 0 }
        }
    }

    nvstruct! {
        pub struct NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1 {
            pub version: NvVersion,
            pub flags: u32,
            pub count: u32,
            pub padding: Padding<[u32; 8]>,
            pub coolers: Array<[NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1; 32]>,
        }
    }

    impl NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1 {
        pub fn valid(&self) -> bool {
            self.flags & 1 != 0
        }

        pub fn set_valid(&mut self, valid: bool) {
            self.flags = self.flags & 0xfffffffe | if valid { 1 } else { 0 }
        }

        pub fn coolers(&self) -> &[NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1] {
            &self.coolers[..self.count as usize]
        }
    }

    nvversion! { @=NV_GPU_CLIENT_FAN_COOLERS_CONTROL NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1(1) = 0x5ac }

    nvapi! {
        pub type GPU_ClientFanCoolersGetControlFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolers: *mut NV_GPU_CLIENT_FAN_COOLERS_CONTROL) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanCoolersGetControl;
    }

    nvapi! {
        pub type GPU_ClientFanCoolersSetControlFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, coolers: *const NV_GPU_CLIENT_FAN_COOLERS_CONTROL) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_ClientFanCoolersSetControl;
    }
}
