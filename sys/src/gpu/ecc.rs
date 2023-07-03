use crate::prelude_::*;

nvenum! {
    /// Used in [NV_GPU_ECC_STATUS_INFO].
    pub enum NV_ECC_CONFIGURATION / EccConfiguration {
        NV_ECC_CONFIGURATION_NOT_SUPPORTED / NotSupported = 0,
        /// Changes require a POST to take effect
        NV_ECC_CONFIGURATION_DEFERRED / Deferred = 1,
        /// Changes can optionally be made to take effect immediately
        NV_ECC_CONFIGURATION_IMMEDIATE / Immediate = 2,
    }
}

nvenum_display! {
    EccConfiguration => {
        NotSupported = "Not Supported",
        _ = _,
    }
}

impl Default for EccConfiguration {
    fn default() -> Self {
        EccConfiguration::NotSupported
    }
}

nvstruct! {
    /// Used in [NvAPI_GPU_GetECCStatusInfo]().
    pub struct NV_GPU_ECC_STATUS_INFO {
        pub version: NvVersion,
        /// ECC memory feature support
        pub isSupported: BoolU32,
        /// Supported ECC memory feature configuration options
        pub configurationOptions: NV_ECC_CONFIGURATION,
        /// Active ECC memory setting
        pub isEnabled: BoolU32,
    }
}

nvversion! { _(NvAPI_GPU_GetECCStatusInfo):
    NV_GPU_ECC_STATUS_INFO(1)
}

nvapi! {
    pub type GPU_GetECCStatusInfoFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pECCStatusInfo@StructVersionOut: *mut NV_GPU_ECC_STATUS_INFO) -> NvAPI_Status;

    /// This function returns ECC memory status information.
    pub fn NvAPI_GPU_GetECCStatusInfo;

    impl self {
        pub fn GetECCStatusInfo;
    }
}

nvstruct! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
    #[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
    pub struct NV_GPU_ECC_ERROR_INFO_ERRORS {
        /// Number of single-bit ECC errors detected
        #[doc(alias = "singleBitErrors")]
        pub single_bit_errors: u64,
        /// Number of double-bit ECC errors detected
        #[doc(alias = "doubleBitErrors")]
        pub double_bit_errors: u64,
    }
}

nvstruct! {
    /// Used in [NvAPI_GPU_GetECCErrorInfo]()
    pub struct NV_GPU_ECC_ERROR_INFO {
        pub version: NvVersion,
        pub padding: u32,
        /// Number of ECC errors detected since last boot
        pub current: NV_GPU_ECC_ERROR_INFO_ERRORS,
        /// Number of ECC errors detected since last counter reset
        pub aggregate: NV_GPU_ECC_ERROR_INFO_ERRORS,
    }
}

nvversion! { _(NvAPI_GPU_GetECCErrorInfo):
    NV_GPU_ECC_ERROR_INFO(1)
}

nvapi! {
    pub type GPU_GetECCErrorInfoFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pECCErrorInfo@StructVersionOut: *mut NV_GPU_ECC_ERROR_INFO) -> NvAPI_Status;

    /// This function returns ECC memory error information.
    pub fn NvAPI_GPU_GetECCErrorInfo;

    impl self {
        pub fn GetECCErrorInfo;
    }
}

nvapi! {
    pub type GPU_ResetECCErrorInfoFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, bResetCurrent: u8, bResetAggregate: u8) -> NvAPI_Status;

    /// This function resets ECC memory error counters.
    pub fn NvAPI_GPU_ResetECCErrorInfo;

    impl self {
        pub fn ResetECCErrorInfo;
    }
}

nvstruct! {
    /// Used in [NvAPI_GPU_GetECCConfigurationInfo]().
    pub struct NV_GPU_ECC_CONFIGURATION_INFO {
        /// Structure version
        pub version: NvVersion,
        /// Current ECC configuration stored in non-volatile memory
        ///
        /// bit 1: Factory default ECC configuration (static)
        pub isEnabled: BoolU32,
    }
}

nvversion! { _(NvAPI_GPU_GetECCConfigurationInfo):
    NV_GPU_ECC_CONFIGURATION_INFO(1) = 8
}

impl NV_GPU_ECC_CONFIGURATION_INFO {
    pub fn isEnabled(&self) -> bool {
        self.isEnabled.get()
    }

    pub fn isEnabledByDefault(&self) -> bool {
        self.isEnabled.flags() & 2 != 0
    }
}

nvapi! {
    pub type GPU_GetECCConfigurationInfoFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pECCConfigurationInfo@StructVersionOut: *mut NV_GPU_ECC_CONFIGURATION_INFO) -> NvAPI_Status;

    /// This function returns ECC memory configuration information.
    pub fn NvAPI_GPU_GetECCConfigurationInfo;

    impl self {
        pub fn GetECCConfigurationInfo;
    }
}

nvapi! {
    pub type GPU_SetECCConfigurationFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, bEnable: u8, bEnableImmediately: bool) -> NvAPI_Status;

    /// This function updates the ECC memory configuration setting.
    pub fn NvAPI_GPU_SetECCConfiguration;

    impl self {
        pub fn SetECCConfiguration;
    }
}
