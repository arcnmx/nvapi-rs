use crate::sys;
use crate::types::NvData;
use sys::gpu::ecc;
use crate::types::NvValue;

pub use sys::gpu::ecc::EccConfiguration;

nvwrap! {
    pub enum EccConfigurationInfo {
        V1(EccConfigurationInfoV1 {
            @type = NvData<ecc::NV_GPU_ECC_CONFIGURATION_INFO> {
                pub enabled: bool {
                    @get fn(&self) {
                        self.sys().isEnabled()
                    },
                },
                pub enabled_by_default: bool {
                    @get fn(&self) {
                        self.sys().isEnabledByDefault()
                    },
                },
            },
        }),
    }

    impl @StructVersion for EccConfigurationInfo { }

    impl EccConfigurationInfo {
        pub fn enabled(&self) -> bool;
        pub fn enabled_by_default(&self) -> bool;
    }
}

nvwrap! {
    pub enum EccStatus {
        V1(EccStatusV1 {
            @type = NvData<ecc::NV_GPU_ECC_STATUS_INFO> {
                pub supported: bool {
                    @sys@BoolU32(isSupported),
                },
                pub enabled: bool {
                    @sys@BoolU32(isEnabled),
                },
                pub configuration: NvValue<EccConfiguration> {
                    @sys(configurationOptions),
                },
            },
        }),
    }

    impl @StructVersion for EccStatus { }

    impl EccStatus {
        pub fn supported(&self) -> bool;
        pub fn enabled(&self) -> bool;
        pub fn configuration(&self) -> NvValue<EccConfiguration>;
    }
}

nvwrap! {
    #[derive(PartialOrd, Ord, PartialEq, Eq)]
    pub enum EccErrorCount {
        V1(EccErrorCountV1 {
            @type = NvData<ecc::NV_GPU_ECC_ERROR_INFO_ERRORS> {
                pub single_bit: u64 {
                    @get fn(&self) {
                        self.sys().single_bit_errors
                    },
                },
                pub double_bit: u64 {
                    @get fn(&self) {
                        self.sys().double_bit_errors
                    },
                },
            },
        }),
    }

    impl EccErrorCount {
        pub fn single_bit(&self) -> u64;
        pub fn double_bit(&self) -> u64;
    }
}

nvwrap! {
    pub enum EccErrors {
        V1(EccErrorsV1 {
            @type = NvData<ecc::NV_GPU_ECC_ERROR_INFO> {
                pub current: EccErrorCountV1 {
                    @sys,
                },
                pub aggregate: EccErrorCountV1 {
                    @sys,
                },
            },
        }),
    }

    impl @StructVersion for EccErrors { }

    impl EccErrors {
        pub fn current(&self) -> EccErrorCount;
        pub fn aggregate(&self) -> EccErrorCount;
    }
}
