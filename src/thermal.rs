use sys::gpu::{thermal, cooler};
use sys;
use types::{Percentage, Celsius, CelsiusShifted, Range, RawConversion};

pub use sys::gpu::thermal::{ThermalController, ThermalTarget};

#[derive(Debug, Copy, Clone)]
pub struct Sensor {
    pub controller: ThermalController,
    pub default_temperature_range: Range<Celsius>,
    pub current_temperature: Celsius,
    pub target: ThermalTarget,
}

impl RawConversion for thermal::NV_GPU_THERMAL_SETTINGS_SENSOR {
    type Target = Sensor;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(Sensor {
            controller: ThermalController::from_raw(self.controller)?,
            default_temperature_range: Range {
                min: Celsius(self.defaultMinTemp),
                max: Celsius(self.defaultMaxTemp),
            },
            current_temperature: Celsius(self.currentTemp),
            target: ThermalTarget::from_raw(self.target)?,
        })
    }
}

impl RawConversion for thermal::NV_GPU_THERMAL_SETTINGS {
    type Target = Vec<Sensor>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.sensor[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ThermalInfo {
    pub controller: ThermalController,
    pub unknown: u32,
    pub temperature_range: Range<CelsiusShifted>,
    pub default_temperature: CelsiusShifted,
    pub default_flags: u32,
}

impl RawConversion for thermal::private::NV_GPU_THERMAL_INFO_ENTRY {
    type Target = ThermalInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalInfo {
            controller: ThermalController::from_raw(self.controller)?,
            unknown: self.unknown,
            temperature_range: Range {
                min: CelsiusShifted(self.minTemp),
                max: CelsiusShifted(self.maxTemp),
            },
            default_temperature: CelsiusShifted(self.defaultTemp),
            default_flags: self.defaultFlags,
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_THERMAL_INFO {
    type Target = (u32, Vec<ThermalInfo>);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries[..self.count as usize].iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
            .map(|t| (self.flags as _, t))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ThermalLimit {
    pub controller: ThermalController,
    pub value: CelsiusShifted,
    pub flags: u32,
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY {
    type Target = ThermalLimit;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalLimit {
            controller: ThermalController::from_raw(self.controller)?,
            value: CelsiusShifted(self.value as _),
            flags: self.flags,
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS {
    type Target = Vec<ThermalLimit>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries[..self.flags as usize].iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

pub use sys::gpu::cooler::private::{CoolerType, CoolerController, CoolerPolicy, CoolerTarget, CoolerControl};

#[derive(Debug, Copy, Clone)]
pub struct Cooler {
    pub kind: CoolerType,
    pub controller: CoolerController,
    pub default_level_range: Range<Percentage>,
    pub current_level_range: Range<Percentage>,
    pub current_level: Percentage,
    pub default_policy: CoolerPolicy,
    pub current_policy: CoolerPolicy,
    pub target: CoolerTarget,
    pub control: CoolerControl,
    pub active: bool,
}

impl RawConversion for cooler::private::NV_GPU_COOLER_SETTINGS_COOLER {
    type Target = Cooler;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(Cooler {
            kind: CoolerType::from_raw(self.type_)?,
            controller: CoolerController::from_raw(self.controller)?,
            default_level_range: Range {
                min: Percentage::from_raw(self.defaultMinLevel)?,
                max: Percentage::from_raw(self.defaultMaxLevel)?,
            },
            current_level_range: Range {
                min: Percentage::from_raw(self.currentMinLevel)?,
                max: Percentage::from_raw(self.currentMaxLevel)?,
            },
            current_level: Percentage::from_raw(self.currentLevel)?,
            default_policy: CoolerPolicy::from_raw(self.defaultPolicy)?,
            current_policy: CoolerPolicy::from_raw(self.currentPolicy)?,
            target: CoolerTarget::from_raw(self.target)?,
            control: CoolerControl::from_raw(self.controlType)?,
            active: cooler::private::CoolerActivityLevel::from_raw(self.active)?.get(),
        })
    }
}

impl RawConversion for cooler::private::NV_GPU_COOLER_SETTINGS {
    type Target = Vec<Cooler>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.cooler[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CoolerLevel {
    pub level: Percentage,
    pub policy: CoolerPolicy,
}

impl RawConversion for cooler::private::NV_GPU_SETCOOLER_LEVEL_COOLER {
    type Target = CoolerLevel;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(CoolerLevel {
            level: Percentage::from_raw(self.currentLevel)?,
            policy: CoolerPolicy::from_raw(self.currentPolicy)?,
        })
    }
}

impl RawConversion for cooler::private::NV_GPU_SETCOOLER_LEVEL {
    type Target = Vec<CoolerLevel>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.cooler.iter().map(RawConversion::convert_raw).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct CoolerPolicyLevel {
    pub level_id: u32,
    pub current_level: u32,
    pub default_level: u32,
}

impl RawConversion for cooler::private::NV_GPU_COOLER_POLICY_LEVEL {
    type Target = CoolerPolicyLevel;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(CoolerPolicyLevel {
            level_id: self.levelId,
            current_level: self.currentLevel,
            default_level: self.defaultLevel,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CoolerPolicyTable {
    pub policy: CoolerPolicy,
    pub levels: Vec<CoolerPolicyLevel>,
}

impl RawConversion for cooler::private::NV_GPU_COOLER_POLICY_TABLE {
    type Target = CoolerPolicyTable;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(CoolerPolicyTable {
            policy: CoolerPolicy::from_raw(self.policy)?,
            levels: self.policyCoolerLevel.iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}
