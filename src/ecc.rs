#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use std::convert::Infallible;
use log::trace;
use crate::sys;
use crate::types::RawConversion;
use sys::gpu::ecc;

pub use sys::gpu::ecc::{EccConfiguration, NV_GPU_ECC_ERROR_INFO_ERRORS as EccErrorCount};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct EccStatus {
    pub supported: bool,
    pub enabled: bool,
    pub configuration: EccConfiguration,
}

impl RawConversion for ecc::NV_GPU_ECC_STATUS_INFO {
    type Target = EccStatus;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.configurationOptions.try_into()
            .map(|configuration| EccStatus {
                supported: self.isSupported.get(),
                enabled: self.isEnabled.get(),
                configuration,
            })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct EccErrors {
    pub current: EccErrorCount,
    pub aggregate: EccErrorCount,
}

impl RawConversion for ecc::NV_GPU_ECC_ERROR_INFO {
    type Target = EccErrors;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(EccErrors {
            current: self.current,
            aggregate: self.aggregate,
        })
    }
}
