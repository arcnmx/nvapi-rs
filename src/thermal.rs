use std::collections::BTreeMap;
use std::convert::Infallible;
use std::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use log::trace;
use crate::sys::gpu::{thermal, cooler};
use crate::sys;
use crate::types::{Percentage, Rpm, Celsius, CelsiusShifted, Kilohertz, Range, RawConversion};

pub use sys::gpu::thermal::{ThermalController, ThermalTarget};
pub use sys::gpu::thermal::private::ThermalPolicyId;
pub use sys::gpu::cooler::private::{FanCoolerId, FanArbiterInfoFlags};

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
            controller: ThermalController::try_from(self.controller)?,
            default_temperature_range: Range {
                min: Celsius(self.defaultMinTemp),
                max: Celsius(self.defaultMaxTemp),
            },
            current_temperature: Celsius(self.currentTemp),
            target: ThermalTarget::try_from(self.target)?,
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

#[derive(Debug, Clone)]
pub struct ThermalInfo {
    pub policy: ThermalPolicyId,
    pub unknown: u32,
    pub pff: Option<PffCurve>,
    pub temperature_range: Range<CelsiusShifted>,
    pub default_temperature: CelsiusShifted,
    pub default_flags: u32,
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_ENTRY_V2 {
    type Target = ThermalInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalInfo {
            policy: self.policy_id.try_into()?,
            unknown: self.unknown,
            temperature_range: Range {
                min: CelsiusShifted(self.minTemp),
                max: CelsiusShifted(self.maxTemp),
            },
            default_temperature: CelsiusShifted(self.defaultTemp),
            default_flags: self.defaultFlags,
            pff: None,
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3 {
    type Target = ThermalInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalInfo {
            policy: self.policy_id.try_into()?,
            unknown: self.unknown,
            temperature_range: Range {
                min: CelsiusShifted(self.minTemp),
                max: CelsiusShifted(self.maxTemp),
            },
            default_temperature: CelsiusShifted(self.defaultTemp),
            default_flags: self.defaultFlags,
            pff: if self.has_pff() {
                Some(self.pff_curve.convert_raw()?)
            } else {
                None
            },
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2 {
    type Target = Vec<ThermalInfo>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries().iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3 {
    type Target = Vec<ThermalInfo>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries().iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ThermalLimit {
    pub policy: ThermalPolicyId,
    pub value: CelsiusShifted,
    pub remove_tdp_limit: bool,
    pub pff: Option<PffStatus>,
}

impl ThermalLimit {
    pub fn to_raw(&self) -> thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3 {
        let mut entry = thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3::default();
        entry.policy_id = self.policy.into();
        entry.temp_limit_C = self.value.0 as _;
        entry.remove_tdp_limit = self.remove_tdp_limit.into();
        if let Some(pff) = &self.pff {
            entry.set_pff(true);
            let (curve, values) = pff.to_raw();
            entry.pff_curve = curve;
            entry.pff_freqs = values;
        }
        entry
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffStatus {
    pub curve: PffCurve,
    pub values: Vec<Kilohertz>,
}

impl PffStatus {
    pub fn points<'a>(&'a self) -> impl Iterator<Item=PffPoint> + 'a {
        self.curve.points.iter().copied()
            .zip(self.values.iter().copied())
            .map(|(point, value)| PffPoint {
                x: point.x,
                y: value,
            })
    }

    pub fn curve(&self) -> PffCurve {
        self.points().collect()
    }

    pub fn to_raw(&self) -> (thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1, [u32; 3]) {
        let mut values = [0u32; 3];
        for (dest, src) in values.iter_mut().zip(&self.values) {
            *dest = src.0 as _;
        }
        (self.curve.to_raw(), values)
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_ENTRY_V2 {
    type Target = ThermalLimit;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalLimit {
            policy: self.policy_id.try_into()?,
            value: CelsiusShifted(self.temp_limit_C as _),
            remove_tdp_limit: false,
            pff: None,
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3 {
    type Target = ThermalLimit;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ThermalLimit {
            policy: self.policy_id.try_into()?,
            value: CelsiusShifted(self.temp_limit_C as _),
            remove_tdp_limit: self.remove_tdp_limit.get(),
            pff: match self.has_pff() {
                true => Some(PffStatus {
                    curve: self.pff_curve.convert_raw()?,
                    values: self.pff_freqs().iter().map(|&c| Kilohertz(c as _)).collect(),
                }),
                false => None,
            }
        })
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2 {
    type Target = Vec<ThermalLimit>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries().iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3 {
    type Target = Vec<ThermalLimit>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries().iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffPoint {
    pub x: CelsiusShifted,
    pub y: Kilohertz,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffCurve {
    pub points: Vec<PffPoint>,
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_PFF_CURVE_POINT_V1 {
    type Target = PffPoint;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(PffPoint {
            x: CelsiusShifted(self.temp as _),
            y: Kilohertz(self.uiT_Y),
        })
    }
}

impl PffPoint {
    pub fn to_raw(&self) -> thermal::private::NV_GPU_CLIENT_PFF_CURVE_POINT_V1 {
        thermal::private::NV_GPU_CLIENT_PFF_CURVE_POINT_V1 {
            enabled: true.into(),
            temp: self.x.0 as _,
            uiT_Y: self.y.0,
            padding: Default::default(),
        }
    }
}

impl RawConversion for thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1 {
    type Target = PffCurve;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(PffCurve {
            points: self.points().iter()
                .map(RawConversion::convert_raw)
                .collect::<Result<_, _>>()?,
        })
    }
}

impl PffCurve {
    pub fn to_raw(&self) -> thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1 {
        let mut curve = thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1::default();
        for (dest, src) in curve.points.iter_mut().zip(&self.points) {
            *dest = src.to_raw();
        }
        curve
    }
}

impl FromIterator<PffPoint> for PffCurve {
    fn from_iter<T: IntoIterator<Item=PffPoint>>(iter: T) -> Self {
        Self {
            points: Vec::from_iter(iter),
        }
    }
}

impl fmt::Display for PffPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.x, self.y)
    }
}

impl fmt::Display for PffCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, p) in self.points.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            fmt::Display::fmt(p, f)?;
        }
        Ok(())
    }
}

pub use sys::gpu::cooler::private::{CoolerType, CoolerController, CoolerPolicy, CoolerTarget, CoolerControl};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Cooler {
    pub info: CoolerInfo,
    pub status: CoolerStatus,
    pub control: CoolerSettings,
    pub unknown: u32,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerInfo {
    pub kind: CoolerType,
    pub controller: CoolerController,
    pub target: CoolerTarget,
    pub control: CoolerControl,
    pub default_level_range: Option<Range<Percentage>>,
    pub default_policy: CoolerPolicy,
    pub tach_range: Option<Range<Rpm>>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerStatus {
    pub current_level: Percentage,
    pub current_level_range: Range<Percentage>,
    pub active: bool,
    pub current_tach: Option<Rpm>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerSettings {
    pub policy: CoolerPolicy,
    pub level: Option<Percentage>,
}

impl CoolerSettings {
    pub fn new(level: Option<Percentage>) -> Self {
        Self {
            policy: match level {
                Some(..) => CoolerPolicy::Manual,
                None => CoolerPolicy::TemperatureContinuous,
            },
            level,
        }
    }

    pub fn to_raw(&self, cooler_id: FanCoolerId) -> cooler::private::NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1 {
        let mut raw = cooler::private::NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1 {
            cooler_id: cooler_id.into(),
            level: self.level.unwrap_or_default().0,
            .. Default::default()
        };
        raw.set_manual(match (self.policy, self.level) {
            (_, None) => false,
            (CoolerPolicy::Performance | CoolerPolicy::TemperatureDiscrete | CoolerPolicy::TemperatureContinuous, _) => true,
            _ => false,
        });
        raw
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTING_V1 {
    type Target = Cooler;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(Cooler {
            info: CoolerInfo {
                kind: CoolerType::try_from(self.type_)?,
                target: CoolerTarget::try_from(self.target)?,
                controller: CoolerController::try_from(self.controller)?,
                control: CoolerControl::try_from(self.controlType)?,
                default_policy: CoolerPolicy::try_from(self.defaultPolicy)?,
                default_level_range: Some(Range {
                    min: Percentage::from_raw(self.defaultMinLevel)?,
                    max: Percentage::from_raw(self.defaultMaxLevel)?,
                }),
                tach_range: None,
            },
            status: CoolerStatus {
                current_level_range: Range {
                    min: Percentage::from_raw(self.currentMinLevel)?,
                    max: Percentage::from_raw(self.currentMaxLevel)?,
                },
                current_level: Percentage::from_raw(self.currentLevel)?,
                active: cooler::private::CoolerActivityLevel::try_from(self.active)?.get(),
                current_tach: None,
            },
            control: CoolerSettings {
                policy: CoolerPolicy::try_from(self.currentPolicy)?,
                level: Some(Percentage::from_raw(self.currentLevel)?),
            },
            unknown: 0,
        })
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTING_V3 {
    type Target = Cooler;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        let mut cooler = self.v1.convert_raw()?;
        if self.tachometer.bSupported.get() {
            cooler.info.tach_range = Some(Range {
                min: Rpm(self.tachometer.minSpeedRPM),
                max: Rpm(self.tachometer.maxSpeedRPM),
            });
            cooler.status.current_tach = Some(Rpm(self.tachometer.speedRPM));
        }
        Ok(cooler)
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTING_V4 {
    type Target = Cooler;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        let mut cooler = self.v3.convert_raw()?;
        cooler.unknown = self.unknown;
        Ok(cooler)
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTINGS_V4 {
    type Target = Vec<Cooler>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTINGS_V3 {
    type Target = Vec<Cooler>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_GETCOOLER_SETTINGS_V1 {
    type Target = Vec<Cooler>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLER_INFO_V1 {
    type Target = (FanCoolerId, CoolerInfo);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.cooler_id.try_into()?, CoolerInfo {
            controller: CoolerController::Internal,
            kind: CoolerType::Fan,
            target: CoolerTarget::GPU,
            control: CoolerControl::Variable,
            default_policy: CoolerPolicy::None,
            default_level_range: None,
            tach_range: match self.tach_supported.get() {
                true => Some(Range {
                    min: Rpm(self.tach_min_rpm),
                    max: Rpm(self.tach_max_rpm),
                }),
                false => None,
            },
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLERS_INFO_V1 {
    type Target = BTreeMap<FanCoolerId, CoolerInfo>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLER_STATUS_V1 {
    type Target = (FanCoolerId, CoolerStatus);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.cooler_id.try_into()?, CoolerStatus {
            active: self.level != 0,
            current_level: Percentage::from_raw(self.level)?,
            current_level_range: Range {
                min: Percentage::from_raw(self.level_minimum)?,
                max: Percentage::from_raw(self.level_maximum)?,
            },
            current_tach: Some(Rpm(self.tach_rpm)),
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1 {
    type Target = BTreeMap<FanCoolerId, CoolerStatus>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1 {
    type Target = (FanCoolerId, CoolerSettings);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.cooler_id.try_into()?, match self.manual() {
            true => CoolerSettings {
                policy: CoolerPolicy::Manual,
                level: Some(Percentage::from_raw(self.level)?),
            },
            false => CoolerSettings {
                policy: CoolerPolicy::TemperatureContinuous,
                level: None,
            },
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1 {
    type Target = BTreeMap<FanCoolerId, CoolerSettings>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.coolers().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for cooler::private::NV_GPU_SETCOOLER_LEVEL_COOLER {
    type Target = CoolerSettings;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(CoolerSettings {
            level: Some(Percentage::from_raw(self.currentLevel)?),
            policy: CoolerPolicy::try_from(self.currentPolicy)?,
        })
    }
}

impl RawConversion for cooler::private::NV_GPU_SETCOOLER_LEVEL {
    type Target = Vec<CoolerSettings>;
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
            policy: CoolerPolicy::try_from(self.policy)?,
            levels: self.policyCoolerLevel.iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FanArbiterInfo {
    pub flags: FanArbiterInfoFlags,
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITER_INFO_V1 {
    type Target = (u32, FanArbiterInfo);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.arbiter_index, FanArbiterInfo {
            flags: self.flags.try_into()?,
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_INFO_V1 {
    type Target = BTreeMap<u32, FanArbiterInfo>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.arbiters().iter().map(RawConversion::convert_raw).collect()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FanArbiterStatus {
    pub fan_stopped: bool,
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1 {
    type Target = (u32, FanArbiterStatus);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.unknown0, FanArbiterStatus {
            fan_stopped: self.fan_stop_active()
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_STATUS_V1 {
    type Target = BTreeMap<u32, FanArbiterStatus>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.arbiters().iter().map(RawConversion::convert_raw).collect()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct FanArbiterControl {
    pub stop_fan: bool,
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1 {
    type Target = (u32, FanArbiterControl);
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok((self.arbiter_index, FanArbiterControl {
            stop_fan: self.flags.truncate().contains(cooler::private::FanArbiterControlFlags::FAN_STOP),
        }))
    }
}

impl RawConversion for cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_CONTROL_V1 {
    type Target = BTreeMap<u32, FanArbiterControl>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.arbiters().iter().map(RawConversion::convert_raw).collect()
    }
}
