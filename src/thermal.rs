use std::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use crate::sys::gpu::{thermal, cooler};
use crate::sys;
use crate::types::{Percentage, Rpm, Celsius, CelsiusShifted, Kilohertz, Range, NvData, NvValue, Tagged};

pub use sys::gpu::thermal::{ThermalController, ThermalTarget};
pub use sys::gpu::thermal::private::ThermalPolicyId;
pub use sys::gpu::cooler::private::{FanCoolerId, FanArbiterInfoFlags, CoolerActivityLevel};

/*#[derive(Debug, Copy, Clone)]
pub struct Sensor {
    pub controller: ThermalController,
    pub default_temperature_range: Range<Celsius>,
    pub current_temperature: Celsius,
    pub target: ThermalTarget,
}*/

nvwrap! {
    pub type Sensor = NvData<thermal::NV_GPU_THERMAL_SETTINGS_SENSOR> {
        pub controller: NvValue<ThermalController> {
            @sys,
        },
        pub default_temperature_range: Range<Celsius> {
            @get fn(&self) {
                Range {
                    min: Celsius(self.sys().defaultMinTemp),
                    max: Celsius(self.sys().defaultMaxTemp),
                }
            },
        },
        pub current_temperature: Celsius {
            @sys(currentTemp),
        },
        pub target: NvValue<ThermalTarget> {
            @sys,
        },
    };
}

nvwrap! {
    pub enum Sensors {
        V1(SensorsV1 {
            @type = NvData<thermal::NV_GPU_THERMAL_SETTINGS_V1> {
                pub sensors: @iter(Sensor) {
                    @into fn into_sensors(self) {
                        self.into_sys().get_sensor().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for Sensors { }
    impl @IntoIterator(into_sensors() -> Sensor) for Sensors { }

    impl Sensors {
        pub fn into_sensors(@iter self) -> Sensor;
    }
}

nvwrap! {
    pub enum ThermalInfo {
        V3(ThermalInfoV3 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3> {
                pub id: NvValue<ThermalPolicyId> {
                    @sys(policy_id),
                },
                pub unknown: u32 {
                    @sys,
                },
                pub temperature_range: Range<CelsiusShifted> {
                    @get fn(&self) {
                        Range {
                            min: CelsiusShifted(self.sys().minTemp),
                            max: CelsiusShifted(self.sys().maxTemp),
                        }
                    },
                },
                pub default_temperature: CelsiusShifted {
                    @sys(defaultTemp),
                },
                pub default_flags: u32 {
                    @sys(defaultFlags),
                },
                pub pff_curve: Option<PffCurveV1> {
                    @get fn(&self) {
                        if self.sys().has_pff() {
                            Some(self.sys().pff_curve.into())
                        } else {
                            None
                        }
                    },
                },
            },
        }),
        V2(ThermalInfoV2 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_INFO_V2> {
                pub id: NvValue<ThermalPolicyId> {
                    @sys(policy_id),
                },
                pub unknown: u32 {
                    @sys,
                },
                pub temperature_range: Range<CelsiusShifted> {
                    @get fn(&self) {
                        Range {
                            min: CelsiusShifted(self.sys().minTemp),
                            max: CelsiusShifted(self.sys().maxTemp),
                        }
                    },
                },
                pub default_temperature: CelsiusShifted {
                    @sys(defaultTemp),
                },
                pub default_flags: u32 {
                    @sys(defaultFlags),
                },
            },
        }),
    }

    impl @TaggedData for ThermalInfo { }

    impl ThermalInfo {
        pub fn id(&self) -> NvValue<ThermalPolicyId>;
        pub fn unknown(&self) -> u32;
        pub fn temperature_range(&self) -> Range<CelsiusShifted>;
        pub fn default_temperature(&self) -> CelsiusShifted;
        pub fn default_flags(&self) -> u32;
    }
}

impl ThermalInfo {
}

nvwrap! {
    pub enum ThermalPolicies {
        V3(ThermalPoliciesV3 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3> {
                pub policies: @iter(ThermalInfoV3) {
                    @into fn into_policies(self) {
                        self.into_sys().get_policies().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
        V2(ThermalPoliciesV2 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2> {
                pub policies: @iter(ThermalInfoV2) {
                    @into fn into_policies(self) {
                        self.into_sys().get_policies().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for ThermalPolicies { }
    impl @IntoIterator(into_policies() -> ThermalInfo) for ThermalPolicies { }

    impl ThermalPolicies {
        pub fn into_policies(@iter self) -> ThermalInfo;
    }
}

nvwrap! {
    pub enum ThermalLimit {
        V3(ThermalLimitV3 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3> {
                pub id@mut(id_mut)@set(set_id): NvValue<ThermalPolicyId> {
                    @sys(policy_id),
                },
                pub temperature@set(set_temperature): CelsiusShifted {
                    @sys(temp_limit_C),
                },
                pub tdp_unlimited@mut(tdp_unlimited_mut)@set(set_tdp_unlimited): bool {
                    @sys@BoolU32(remove_tdp_limit),
                },
                pub pff_curve: Option<PffCurveV1> {
                    @get fn(&self) {
                        match self.sys().has_pff() {
                            true => Some(self.sys().pff_curve.into()),
                            false => None,
                        }
                    },
                },
                pub pff_frequencies: Option<Vec<u32>> {
                    @get fn(&self) {
                        match self.sys().has_pff() {
                            true => Some(self.sys().pff_freqs().collect()),
                            false => None,
                        }
                    },
                },
            },
        }),
        V2(ThermalLimitV2 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V2> {
                pub id@mut(id_mut)@set(set_id): NvValue<ThermalPolicyId> {
                    @sys(policy_id),
                },
                pub temperature@set(set_temperature): CelsiusShifted {
                    @sys(temp_limit_C),
                },
            },
        }),
    }

    impl @TaggedData for ThermalLimit { }

    impl ThermalLimit {
        pub fn id(&self) -> NvValue<ThermalPolicyId>;
        pub fn temperature(&self) -> CelsiusShifted;

        pub fn set_id(&mut self, id: NvValue<ThermalPolicyId>) -> ();
        pub fn set_temperature(&mut self, temperature: CelsiusShifted) -> ();
    }
}

impl ThermalLimit {
    pub fn tdp_unlimited(&self) -> bool {
        match self {
            Self::V3(limit) => limit.tdp_unlimited(),
            Self::V2(..) => false,
        }
    }

    pub fn set_tdp_unlimited(&mut self, value: bool) -> bool {
        match self {
            Self::V3(limit) => {
                limit.set_tdp_unlimited(value);
                true
            },
            Self::V2(..) => false,
        }
    }

    pub fn pff_curve(&self) -> Option<PffCurve> {
        match self {
            Self::V3(limit) => limit.pff_curve().map(Into::into),
            Self::V2(..) => None,
        }
    }

    pub fn pff_frequencies(&self) -> Option<Vec<u32>> {
        match self {
            Self::V3(limit) => limit.pff_frequencies(),
            Self::V2(..) => None,
        }
    }
}

nvwrap! {
    pub enum ThermalLimits {
        V3(ThermalLimitsV3 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3> {
                pub policies: @iter(ThermalLimitV3) {
                    @into fn into_policies(self) {
                        self.into_sys().get_policies().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
        V2(ThermalLimitsV2 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2> {
                pub policies: @iter(ThermalLimitV2) {
                    @into fn into_policies(self) {
                        self.into_sys().get_policies().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for ThermalLimits { }
    impl @IntoIterator(into_policies() -> ThermalLimit) for ThermalLimits { }

    impl ThermalLimits {
        pub fn into_policies(@iter self) -> ThermalLimit;
    }
}

/*#[derive(Debug, Clone)]
pub struct ThermalInfo {
    pub policy: ThermalPolicyId,
    pub unknown: u32,
    pub pff: Option<PffCurve>,
    pub temperature_range: Range<CelsiusShifted>,
    pub default_temperature: CelsiusShifted,
    pub default_flags: u32,
}*/

/*nvconv! {
    fn try_from(entry: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_INFO_V2) -> Result<ThermalInfo, sys::ArgumentRangeError> {
        Ok(Self {
            policy: entry.policy_id.try_into()?,
            unknown: entry.unknown,
            temperature_range: Range {
                min: CelsiusShifted(entry.minTemp),
                max: CelsiusShifted(entry.maxTemp),
            },
            default_temperature: CelsiusShifted(entry.defaultTemp),
            default_flags: entry.defaultFlags,
            pff: None,
        })
    }
}*/

/*nvconv! {
    fn try_from(info: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_INFO_V3) -> Result<ThermalInfo, sys::ArgumentRangeError> {
        Ok(Self {
            policy: info.policy_id.try_into()?,
            unknown: info.unknown,
            temperature_range: Range {
                min: CelsiusShifted(info.minTemp),
                max: CelsiusShifted(info.maxTemp),
            },
            default_temperature: CelsiusShifted(info.defaultTemp),
            default_flags: info.defaultFlags,
            pff: if info.has_pff() {
                Some(info.pff_curve.try_into()?)
            } else {
                None
            },
        })
    }
}*/

/*
nvconv! {
    fn try_from(info: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V2) -> Result<List<ThermalInfo>, sys::ArgumentRangeError> {
        let values = info.entries().iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}

nvconv! {
    fn try_from(info: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_INFO_V3) -> Result<List<ThermalInfo>, sys::ArgumentRangeError> {
        let values = info.entries().iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}*/

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ThermalLimit {
    pub policy: ThermalPolicyId,
    pub value: CelsiusShifted,
    pub remove_tdp_limit: bool,
    pub pff: Option<PffStatus>,
}*/

/*impl ThermalLimit {
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
}*/

/*
nvwrap! {
    pub enum PffStatus {
        V3(PffStatusV3 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3> {
                pub curve: PffCurve {
                },
                pub frequencies: [u32; 3] {
                },
            },
        }),
        V2(PffStatusV2 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V2> {
                pub curve: PffCurve {
                },
                pub frequencies: [u32; 3] {
                },
            },
        }),
    }
}*/

nvwrap! {
    pub enum PffPoint {
        V1(PffPointV1 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_PFF_CURVE_POINT_V1> {
                pub x: CelsiusShifted {
                    @sys(temp),
                },
                pub y: Kilohertz {
                    @sys(uiT_Y),
                },
            },
        }),
    }

    impl @TaggedFrom(i32) for PffPoint { }

    impl PffPoint {
        pub fn x(&self) -> CelsiusShifted;
        pub fn y(&self) -> Kilohertz;
    }
}

nvwrap! {
    pub enum PffCurve {
        V1(PffCurveV1 {
            @type = NvData<thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1> {
                pub points: @iter(Tagged<i32, PffPointV1>) {
                    @into fn into_points(self) {
                        self.into_sys().points().into_iter()
                            .map(Tagged::from)
                    },
                },
            },
        }),
    }

    impl @IntoIterator(into_points() -> Tagged<i32, PffPoint>) for PffCurve { }

    impl PffCurve {
        pub fn into_points(@iter self) -> Tagged<i32, PffPoint>;
    }
}

/*
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffStatus {
    pub curve: PffCurve,
    pub frequencies: [u32; 3],
}

impl PffStatus {
    pub fn frequencies(&self) -> impl Iterator<Item=Kilohertz> {
        self.frequencies.iter().map(|&c| Kilohertz(c as _))
    }
    pub fn points<'a>(&'a self) -> impl Iterator<Item=PffPoint> + 'a {
        self.curve.points.iter().copied()
            .zip(self.frequencies())
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
}*/

/*nvconv! {
    fn try_from(entry: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V2) -> Result<ThermalLimit, sys::ArgumentRangeError> {
        Ok(ThermalLimit {
            policy: entry.policy_id.try_into()?,
            value: CelsiusShifted(entry.temp_limit_C as _),
            remove_tdp_limit: false,
            pff: None,
        })
    }
}

nvconv! {
    fn try_from(status: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICY_STATUS_V3) -> Result<ThermalLimit, sys::ArgumentRangeError> {
        Ok(ThermalLimit {
            policy: status.policy_id.try_into()?,
            value: CelsiusShifted(status.temp_limit_C as _),
            remove_tdp_limit: status.remove_tdp_limit.get(),
            pff: match status.has_pff() {
                true => Some(PffStatus {
                    curve: status.pff_curve.try_into()?,
                    values: status.pff_freqs().iter().map(|&c| Kilohertz(c as _)).collect(),
                }),
                false => None,
            }
        })
    }
}*/

/*nvconv! {
    fn try_from(status: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V2) -> Result<List<ThermalLimit>, sys::ArgumentRangeError> {
        let values = status.entries().iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}

nvconv! {
    fn try_from(status: &thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS_V3) -> Result<List<ThermalLimit>, sys::ArgumentRangeError> {
        let values = status.entries().iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}*/

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffPoint {
    pub x: CelsiusShifted,
    pub y: Kilohertz,
}*/

/*
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PffCurve {
    pub points: Vec<PffPoint>,
}

nvconv! {
    fn from(point: &thermal::private::NV_GPU_CLIENT_PFF_CURVE_POINT_V1) -> PffPoint {
        PffPoint {
            x: CelsiusShifted(point.temp as _),
            y: Kilohertz(point.uiT_Y),
        }
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

nvconv! {
    fn from(curve: &thermal::private::NV_GPU_CLIENT_PFF_CURVE_V1) -> PffCurve {
        PffCurve {
            points: curve.points().iter()
                .map(Into::into)
                .collect(),
        }
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
}*/

impl fmt::Display for PffPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.x(), self.y())
    }
}

impl fmt::Display for PffCurve {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO!!
        for Tagged { tag, value: p } in self.clone().into_points() {
            if tag > 0 {
                f.write_str(", ")?;
            }
            fmt::Display::fmt(&p, f)?;
        }
        Ok(())
    }
}

pub use sys::gpu::cooler::private::{CoolerType, CoolerController, CoolerPolicy, CoolerTarget, CoolerControl};

pub type Cooler = GetCoolerSettings;

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]*/
/*pub struct Cooler {
    pub info: CoolerInfo,
    pub status: CoolerStatus,
    pub control: CoolerSettings,
    pub unknown: u32,
}*/

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerInfo {
    pub kind: CoolerType,
    pub controller: CoolerController,
    pub target: CoolerTarget,
    pub control: CoolerControl,
    pub default_level_range: Option<Range<Percentage>>,
    pub default_policy: CoolerPolicy,
    pub tach_range: Option<Range<Rpm>>,
}*/

nvwrap! {
    pub enum CoolerInfo {
        V1(CoolerInfoV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLER_INFO_V1> {
                pub id@mut(id_mut)@set(set_id): NvValue<FanCoolerId> {
                    @sys(cooler_id),
                },
                pub tach_range@set(set_tach_range): Option<Range<Rpm>> {
                    @get fn(&self) {
                        match self.sys().tach_supported.get() {
                            true => Some(Range {
                                min: Rpm(self.sys().tach_min_rpm),
                                max: Rpm(self.sys().tach_max_rpm),
                            }),
                            false => None,
                        }
                    },
                    @set fn(&mut self, tach_range: Option<Range<Rpm>>) {
                        let sys = self.sys_mut();
                        let (supported, min, max) = match tach_range {
                            Some(Range { min, max }) => (true, min.0, max.0),
                            None => (false, 0, 0),
                        };
                        sys.tach_supported = supported.into();
                        sys.tach_min_rpm = min;
                        sys.tach_max_rpm = max;
                    },
                },
            },
        }),
    }

    impl @TaggedData for CoolerInfo { }
    impl @Deref(CoolerInfoV1) for CoolerInfo { }

    impl CoolerInfo {
        pub fn id(&self) -> NvValue<FanCoolerId>;
        pub fn tach_range(&self) -> Option<Range<Rpm>>;
    }
}

impl From<(FanCoolerId, GetCoolerSettings)> for CoolerInfo {
    fn from((id, settings): (FanCoolerId, GetCoolerSettings)) -> Self {
        let mut info = Self::default();
        info.set_id(id.into());
        info.set_tach_range(settings.tach_range());
        info
    }
}

nvwrap! {
    pub enum CoolersInfo {
        V1(CoolersInfoV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLERS_INFO_V1> {
                pub coolers: @iter(CoolerInfoV1) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for CoolersInfo { }
    impl @IntoIterator(into_coolers() -> CoolerInfo) for CoolersInfo { }

    impl CoolersInfo {
        pub fn into_coolers(@iter self) -> CoolerInfo;
    }
}

nvwrap! {
    pub type GetCoolerSettingsV1 = NvData<cooler::private::NV_GPU_GETCOOLER_SETTING_V1> {
        pub kind@mut(kind_mut)@set(set_kind): NvValue<CoolerType> {
            @sys(type_),
        },
        pub target@mut(target_mut)@set(set_target): NvValue<CoolerTarget> {
            @sys,
        },
        pub controller@mut(controller_mut)@set(set_controller): NvValue<CoolerController> {
            @sys(controller),
        },
        pub control@mut(control_mut)@set(set_control): NvValue<CoolerControl> {
            @sys(controlType),
        },
        pub default_policy@mut(default_policy_mut)@set(set_default_policy): NvValue<CoolerPolicy> {
            @sys(defaultPolicy),
        },
        pub policy@mut(policy_mut)@set(set_policy): NvValue<CoolerPolicy> {
            @sys(currentPolicy),
        },
        pub default_level_range@set(set_default_level_range): Range<Percentage> {
            @get fn(&self) {
                Range {
                    min: Percentage(self.sys().defaultMinLevel),
                    max: Percentage(self.sys().defaultMaxLevel),
                }
            },
            @set fn self value {
                self.sys_mut().defaultMinLevel = value.min.into();
                self.sys_mut().defaultMaxLevel = value.max.into();
            },
        },
        pub level_range@set(set_level_range): Range<Percentage> {
            @get fn(&self) {
                Range {
                    min: Percentage(self.sys().currentMinLevel),
                    max: Percentage(self.sys().currentMaxLevel),
                }
            },
        },
        pub level@mut(level_mut)@set(set_level): Percentage {
            @sys(currentLevel),
        },
        pub active@mut(active_mut)@set(set_active): NvValue<CoolerActivityLevel> {
            @sys,
        },
    };
}

nvwrap! {
    pub type GetCoolerSettingsV3 = NvData<cooler::private::NV_GPU_GETCOOLER_SETTING_V3> {
        pub tach_range@set(set_tach_range): Option<Range<Rpm>> {
            @get fn(&self) {
                let sys = self.sys();
                if sys.tachometer.bSupported.get() {
                    Some(Range {
                        min: Rpm(sys.tachometer.minSpeedRPM),
                        max: Rpm(sys.tachometer.maxSpeedRPM),
                    })
                } else {
                    None
                }
            },
            @set fn(&mut self, value: Option<Range<Rpm>>) {
                let sys = self.sys_mut();
                let (supported, min, max) = match value {
                    None => (false, 0, 0),
                    Some(value) => (true, value.min.0, value.max.0),
                };
                sys.tachometer.bSupported.set(supported);
                sys.tachometer.minSpeedRPM = min;
                sys.tachometer.minSpeedRPM = max;
            },
        },
        pub tach@set(set_tach): Option<Rpm> {
            @get fn(&self) {
                let sys = self.sys();
                if sys.tachometer.bSupported.get() {
                    Some(Rpm(sys.tachometer.speedRPM))
                } else {
                    None
                }
            },
            @set fn(&mut self, value: Option<Rpm>) {
                let sys = self.sys_mut();
                let (supported, speed) = match value {
                    None => (false, 0),
                    Some(value) => (true, value.0),
                };
                sys.tachometer.bSupported.set(supported);
                sys.tachometer.speedRPM = speed;
            },
        },
    };

    impl @Deref(v1: GetCoolerSettingsV1) for GetCoolerSettingsV3 { }
}

nvwrap! {
    pub type GetCoolerSettingsV4 = NvData<cooler::private::NV_GPU_GETCOOLER_SETTING_V4> {
        pub unknown: u32 {
            @sys,
        },
    };

    impl @Deref(v3: GetCoolerSettingsV3) for GetCoolerSettingsV4 { }
}

nvwrap! {
    pub enum GetCoolerSettings {
        V4(GetCoolerSettingsV4),
        V3(GetCoolerSettingsV3),
        V1(GetCoolerSettingsV1),
    }

    impl @Deref(GetCoolerSettingsV1) for GetCoolerSettings { }

    impl GetCoolerSettings {
        pub fn set_from_info(&mut self, info: &CoolerInfo) -> ();
        pub fn set_from_status(&mut self, status: &CoolerStatus) -> ();
        pub fn set_from_settings(&mut self, settings: &CoolerSettings) -> ();
    }
}

impl GetCoolerSettings {
    pub fn with_cooler(info: &CoolerInfo, status: &CoolerStatus, settings: &CoolerSettings) -> (NvValue<FanCoolerId>, Self) {
        let mut this = GetCoolerSettingsV1::default();
        this.set_controller(CoolerController::Internal.into());
        this.set_kind(CoolerType::Fan.into());
        this.set_target(CoolerTarget::GPU.into());
        this.set_control(CoolerControl::Variable.into());
        this.set_default_policy(CoolerPolicy::None.into());

        this.set_from_info(info);
        this.set_from_status(status);
        this.set_from_settings(settings);

        (info.id().into(), this.into())
    }

    pub fn tach_range(&self) -> Option<Range<Rpm>> {
        match self {
            Self::V4(settings) => settings.tach_range(),
            _ => None,
        }
    }

    pub fn tach(&self) -> Option<Rpm> {
        match self {
            Self::V4(settings) => settings.tach(),
            _ => None,
        }
    }

    pub fn set_tach(&mut self, value: Option<Rpm>) {
        match self {
            Self::V4(settings) => settings.set_tach(value),
            Self::V3(settings) => settings.set_tach(value),
            #[cfg(feature = "log")]
            _ if value.is_some() =>
                log::warn!("GetCoolerSettingsV2 and older cannot set_tach({value:?})"),
            _ => (),
        }
    }

    pub fn set_tach_range(&mut self, value: Option<Range<Rpm>>) {
        match self {
            Self::V4(settings) => settings.set_tach_range(value),
            Self::V3(settings) => settings.set_tach_range(value),
            #[cfg(feature = "log")]
            _ if value.is_some() =>
                log::warn!("GetCoolerSettingsV1 cannot set_tach({value:?})"),
            _ => (),
        }
    }
}

impl GetCoolerSettingsV1 {
    pub fn set_from_info(&mut self, info: &CoolerInfo) {
        todo!()
    }

    pub fn set_from_status(&mut self, status: &CoolerStatus) {
        todo!()
    }

    pub fn set_from_settings(&mut self, settings: &CoolerSettings) {
        self.set_policy(settings.policy());
        self.set_level(settings.level().unwrap_or_default());
        todo!()
    }
}

impl GetCoolerSettingsV3 {
    pub fn set_from_info(&mut self, info: &CoolerInfo) {
        GetCoolerSettingsV1::set_from_info(self, info);
        self.set_tach_range(info.tach_range());
        todo!()
    }

    pub fn set_from_status(&mut self, status: &CoolerStatus) {
        GetCoolerSettingsV1::set_from_status(self, status);
        todo!()
    }

    pub fn set_from_settings(&mut self, settings: &CoolerSettings) {
        GetCoolerSettingsV1::set_from_settings(self, settings);
        todo!()
    }
}

nvwrap! {
    pub enum GetCoolersSettings {
        V4(GetCoolersSettingsV4 {
            @type = NvData<cooler::private::NV_GPU_GETCOOLER_SETTINGS_V4> {
                pub coolers: @iter(GetCoolerSettingsV4) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
        V3(GetCoolersSettingsV3 {
            @type = NvData<cooler::private::NV_GPU_GETCOOLER_SETTINGS_V3> {
                pub coolers: @iter(GetCoolerSettingsV3) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
        V1(GetCoolersSettingsV1 {
            @type = NvData<cooler::private::NV_GPU_GETCOOLER_SETTINGS_V1> {
                pub coolers: @iter(GetCoolerSettingsV1) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for GetCoolersSettings { }
    impl @IntoIterator(into_coolers() -> GetCoolerSettings) for GetCoolersSettings { }

    impl GetCoolersSettings {
        pub fn into_coolers(@iter self) -> GetCoolerSettings;
    }
}

impl GetCoolersSettings {
    pub fn coolers(self) -> impl Iterator<Item = (NvValue<FanCoolerId>, GetCoolerSettings)> {
        self.into_coolers().enumerate()
            .map(|(id, cooler)| ((id as i32).into(), cooler))
    }
}

nvwrap! {
    pub enum SetCoolerLevel {
        V1(SetCoolerLevelV1 {
            @type = NvData<cooler::private::NV_GPU_SETCOOLER_LEVEL_COOLER> {
                pub policy@mut(policy_mut)@set(set_policy): NvValue<CoolerPolicy> {
                    @sys(currentPolicy),
                },
                pub level@mut(level_mut)@set(set_level): Percentage {
                    @sys(currentLevel),
                },
            },
        }),
    }

    impl SetCoolerLevel {
        pub fn policy(&self) -> NvValue<CoolerPolicy>;
        pub fn level(&self) -> Percentage;
        /*pub fn level_mut(&mut self) -> &mut Percentage;
        pub fn set_level(&mut self, value: Percentage);*/
    }
}
nvwrap! {
    pub enum SetCoolerLevels {
        V1(SetCoolerLevelsV1 {
            @type = NvData<cooler::private::NV_GPU_SETCOOLER_LEVEL_V1> {
                pub coolers: @iter(SetCoolerLevelV1) {
                    /*@into fn into_coolers(self) {
                        self.into_sys().get_coolers().into_iter()
                            .into_iter().map(Into::into)
                    },*/
                },
            },
        }),
    }

    impl @StructVersion for SetCoolerLevels { }
    /*impl @IntoIterator(into_coolers() -> SetCoolerLevel) for SetCoolerLevels { }

    impl SetCoolerLevels {
        pub fn into_coolers(@iter self) -> SetCoolerLevel;
    }*/
}

nvwrap! {
    pub enum CoolerStatus {
        V1(CoolerStatusV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLER_STATUS_V1> {
                pub id@mut(id_mut)@set(set_id): NvValue<FanCoolerId> {
                    @sys(cooler_id),
                },
                pub level@mut(level_mut)@set(set_level): Percentage {
                    @sys,
                },
                pub level_range@set(set_level_range): Range<Percentage> {
                    @get fn(&self) {
                        Range {
                            min: Percentage(self.sys().level_minimum),
                            max: Percentage(self.sys().level_maximum),
                        }
                    },
                    @set fn(&mut self, level: Range<Percentage>) {
                        self.sys_mut().level_minimum = level.min.into();
                        self.sys_mut().level_maximum = level.max.into();
                    },
                },
                pub tach@mut(tach_mut)@set(set_tach): Rpm {
                    @sys(tach_rpm),
                },
            },
        }),
    }

    impl @TaggedData for CoolerStatus { }
    impl @Deref(CoolerStatusV1) for CoolerStatus { }
}

impl From<GetCoolerSettings> for CoolerStatus {
    fn from(settings: GetCoolerSettings) -> Self {
        let mut status = Self::default();
        status.set_level(settings.level());
        status.set_level_range(settings.level_range());
        status.set_tach(settings.tach().unwrap_or_default());
        status
                //active: cooler::private::CoolerActivityLevel::try_from(settings.active)?.get(),
    }
}

nvwrap! {
    pub enum CoolersStatus {
        V1(CoolersStatusV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1> {
                pub coolers: @iter(CoolerStatusV1) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers()
                            .into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for CoolersStatus { }
    impl @IntoIterator(into_coolers() -> CoolerStatus) for CoolersStatus { }

    impl CoolersStatus {
        pub fn into_coolers(@iter self) -> CoolerStatus;
    }
}

nvwrap! {
    pub enum CoolerSettings {
        V1(CoolerSettingsV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1> {
                pub id@mut(id_mut)@set(set_id): NvValue<FanCoolerId> {
                    @sys(cooler_id),
                },
                pub policy@set(set): NvValue<CoolerPolicy> {
                    @get fn(&self) {
                        match self.sys().manual() {
                            true => CoolerPolicy::Manual.into(),
                            false => CoolerPolicy::TemperatureContinuous.into(),
                        }
                    },
                    @set fn(&mut self, policy: CoolerPolicy, level: Option<Rpm>) {
                        self.sys_mut().level = level.unwrap_or_default().0;
                        self.sys_mut().set_manual(match (policy, level) {
                            (_, None) => false,
                            (CoolerPolicy::Performance | CoolerPolicy::TemperatureDiscrete | CoolerPolicy::TemperatureContinuous, _) => true,
                            _ => false,
                        });
                    },
                },
                pub level: Option<Percentage> {
                    @get fn(&self) {
                        match self.sys().manual() {
                            true => Some(Percentage(self.sys().level)),
                            false => None,
                        }
                    },
                },
            },
        }),
    }

    impl @TaggedData for CoolerSettings { }
    impl @Deref(CoolerSettingsV1) for CoolerSettings { }
}

nvwrap! {
    pub enum CoolersSettings {
        V1(CoolersSettingsV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1> {
                pub coolers: @iter(CoolerSettingsV1) {
                    @into fn into_coolers(self) {
                        self.into_sys().get_coolers()
                            .into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for CoolersSettings { }
    impl @IntoIterator(into_coolers() -> CoolerSettings) for CoolersSettings { }

    impl CoolersSettings {
        pub fn into_coolers(@iter self) -> CoolerSettings;
    }
}

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerStatus {
    pub current_level: Percentage,
    pub current_level_range: Range<Percentage>,
    pub active: bool,
    pub current_tach: Option<Rpm>,
}*/

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CoolerSettings {
    pub policy: CoolerPolicy,
    pub level: Option<Percentage>,
}*/

/*impl CoolerSettings {
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
}*/

/*nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTING_V1) -> Result<Cooler, sys::ArgumentRangeError> {
        Ok(Cooler {
            info: CoolerInfo {
                kind: CoolerType::try_from(settings.type_)?,
                target: CoolerTarget::try_from(settings.target)?,
                controller: CoolerController::try_from(settings.controller)?,
                control: CoolerControl::try_from(settings.controlType)?,
                default_policy: CoolerPolicy::try_from(settings.defaultPolicy)?,
                default_level_range: Some(Range {
                    min: Percentage::from_raw(settings.defaultMinLevel)?,
                    max: Percentage::from_raw(settings.defaultMaxLevel)?,
                }),
                tach_range: None,
            },
            status: CoolerStatus {
                current_level_range: Range {
                    min: Percentage::from_raw(settings.currentMinLevel)?,
                    max: Percentage::from_raw(settings.currentMaxLevel)?,
                },
                current_level: Percentage::from_raw(settings.currentLevel)?,
                active: cooler::private::CoolerActivityLevel::try_from(settings.active)?.get(),
                current_tach: None,
            },
            control: CoolerSettings {
                policy: CoolerPolicy::try_from(settings.currentPolicy)?,
                level: Some(Percentage::from_raw(settings.currentLevel)?),
            },
            unknown: 0,
        })
    }
}*/

/*nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTING_V3) -> Result<Cooler, sys::ArgumentRangeError> {
        let mut cooler: Self = settings.v1.try_into()?;
        if settings.tachometer.bSupported.get() {
            cooler.info.tach_range = Some(Range {
                min: Rpm(settings.tachometer.minSpeedRPM),
                max: Rpm(settings.tachometer.maxSpeedRPM),
            });
            cooler.status.current_tach = Some(Rpm(settings.tachometer.speedRPM));
        }
        Ok(cooler)
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTING_V4) -> Result<Cooler, sys::ArgumentRangeError> {
        let mut cooler: Self = settings.v3.try_into()?;
        cooler.unknown = settings.unknown;
        Ok(cooler)
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTINGS_V4) -> Result<List<Cooler>, sys::ArgumentRangeError> {
        let values = settings.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTINGS_V3) -> Result<List<Cooler>, sys::ArgumentRangeError> {
        let values = settings.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_GETCOOLER_SETTINGS_V1) -> Result<List<Cooler>, sys::ArgumentRangeError> {
        let values = settings.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}*/

//type FanCooler<T> = Tagged<FanCoolerId, T>;

/*nvconv! {
    fn try_from(info: &cooler::private::NV_GPU_CLIENT_FAN_COOLER_INFO_V1) -> Result<FanCooler<CoolerInfo>, sys::ArgumentRangeError> {
        let id = info.cooler_id.try_into()?;
        let value = CoolerInfo {
            controller: CoolerController::Internal,
            kind: CoolerType::Fan,
            target: CoolerTarget::GPU,
            control: CoolerControl::Variable,
            default_policy: CoolerPolicy::None,
            default_level_range: None,
            tach_range: match info.tach_supported.get() {
                true => Some(Range {
                    min: Rpm(info.tach_min_rpm),
                    max: Rpm(info.tach_max_rpm),
                }),
                false => None,
            },
        };
        Ok(Self {
            id,
            value,
        })
    }
}*/

/*pub type FanCoolers<T> = Map<FanCoolerId, T>;

nvconv! {
    fn try_from(info: &cooler::private::NV_GPU_CLIENT_FAN_COOLERS_INFO_V1) -> Result<FanCoolers<CoolerInfo>, sys::ArgumentRangeError> {
        info.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()
    }
}

nvconv! {
    fn try_from(status: &cooler::private::NV_GPU_CLIENT_FAN_COOLER_STATUS_V1) -> Result<FanCooler<CoolerStatus>, sys::ArgumentRangeError> {
        let id =status.cooler_id.try_into()?;
        let value = CoolerStatus {
            active: status.level != 0,
            current_level: Percentage::from_raw(status.level)?,
            current_level_range: Range {
                min: Percentage::from_raw(status.level_minimum)?,
                max: Percentage::from_raw(status.level_maximum)?,
            },
            current_tach: Some(Rpm(status.tach_rpm)),
        };
        Ok(Self {
            id,
            value,
        })
    }
}

nvconv! {
    fn try_from(status: &cooler::private::NV_GPU_CLIENT_FAN_COOLERS_STATUS_V1) -> Result<FanCoolers<CoolerStatus>, sys::ArgumentRangeError> {
        status.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()
    }
}

nvconv! {
    fn try_from(control: &cooler::private::NV_GPU_CLIENT_FAN_COOLER_CONTROL_V1) -> Result<FanCooler<CoolerSettings>, sys::ArgumentRangeError> {
        let id = control.cooler_id.try_into()?;
        let value = match control.manual() {
            true => CoolerSettings {
                policy: CoolerPolicy::Manual,
                level: Some(Percentage::from_raw(control.level)?),
            },
            false => CoolerSettings {
                policy: CoolerPolicy::TemperatureContinuous,
                level: None,
            },
        };
        Ok(Self {
            id,
            value,
        })
    }
}

nvconv! {
    fn try_from(control: &cooler::private::NV_GPU_CLIENT_FAN_COOLERS_CONTROL_V1) -> Result<FanCoolers<CoolerSettings>, sys::ArgumentRangeError> {
        control.coolers().iter().map(TryInto::try_into).collect::<Result<_, _>>()
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_SETCOOLER_LEVEL_COOLER) -> Result<CoolerSettings, sys::ArgumentRangeError> {
        Ok(CoolerSettings {
            level: Some(Percentage::from_raw(settings.currentLevel)?),
            policy: CoolerPolicy::try_from(settings.currentPolicy)?,
        })
    }
}

nvconv! {
    fn try_from(settings: &cooler::private::NV_GPU_SETCOOLER_LEVEL) -> Result<List<CoolerSettings>, sys::ArgumentRangeError> {
        let values = settings.cooler.iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
        Ok(Self {
            values,
        })
    }
}*/

/*#[derive(Debug, Copy, Clone)]
pub struct CoolerPolicyLevel {
    pub level_id: u32,
    pub current_level: u32,
    pub default_level: u32,
}

nvconv! {
    fn try_from(level: &cooler::private::NV_GPU_COOLER_POLICY_LEVEL) -> Result<CoolerPolicyLevel, sys::ArgumentRangeError> {
        Ok(CoolerPolicyLevel {
            level_id: level.levelId,
            current_level: level.currentLevel,
            default_level: level.defaultLevel,
        })
    }
}*/

nvwrap! {
    pub enum CoolerPolicyLevel {
        V1(CoolerPolicyLevelV1 {
            @type = NvData<cooler::private::NV_GPU_COOLER_POLICY_LEVEL> {
                pub id: u32 {
                    @sys(levelId),
                },
                pub level: u32 {
                    @sys(currentLevel),
                },
                pub default_level: u32 {
                    @sys(defaultLevel),
                },
            },
        }),
    }

    impl @TaggedData for CoolerPolicyLevel { }
    impl CoolerPolicyLevel {
        pub fn id(&self) -> u32;
        pub fn level(&self) -> u32;
        pub fn default_level(&self) -> u32;
    }
}

nvwrap! {
    pub enum CoolerPolicyTable {
        V1(CoolerPolicyTableV1 {
            @type = NvData<cooler::private::NV_GPU_COOLER_POLICY_TABLE_V1> {
                pub policy@mut(policy_mut)@set(set_policy): NvValue<CoolerPolicy> {
                    @sys(policy),
                },
            },
        }),
    }

    impl @StructVersion for CoolerPolicyTable { }

    impl CoolerPolicyTable {
        pub fn policy(&self) -> NvValue<CoolerPolicy>;
    }
}

impl CoolerPolicyTableV1 {
    pub fn levels(&self, count: usize) -> impl Iterator<Item = &CoolerPolicyLevelV1> {
        self.sys().policyCoolerLevel[..count]
            .iter().map(From::from)
    }
}

impl CoolerPolicyTable {
    pub fn levels<'a>(&'a self, count: usize) -> impl Iterator<Item = CoolerPolicyLevel> + 'a {
        match self {
            Self::V1(table) => table.levels(count)
                .copied().map(Into::into),
        }
    }
}

/*#[derive(Debug, Clone)]
pub struct CoolerPolicyTable {
    pub policy: CoolerPolicy,
    pub levels: Vec<CoolerPolicyLevel>,
}*/

/*nvconv! {
    fn try_from(table: &cooler::private::NV_GPU_COOLER_POLICY_TABLE) -> Result<CoolerPolicyTable, sys::ArgumentRangeError> {
        Ok(CoolerPolicyTable {
            policy: CoolerPolicy::try_from(table.policy)?,
            levels: table.policyCoolerLevel.iter().map(TryInto::try_into).collect::<Result<_, _>>()?,
        })
    }
}*/

nvwrap! {
    pub enum FanArbiter {
        V1(FanArbiterV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITER_INFO_V1> {
                pub flags: FanArbiterInfoFlags {
                    @get fn(&self) {
                        self.sys().flags.truncate()
                    },
                },
            },
        }),
    }

    impl @TaggedData(@i32) for FanArbiter { }

    impl FanArbiter {
        pub fn flags(&self) -> FanArbiterInfoFlags;
    }
}

nvwrap! {
    pub enum FanArbiters {
        V1(FanArbitersV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_INFO_V1> {
                pub arbiters: @iter(FanArbiterV1) {
                    @into fn into_arbiters(self) {
                        self.into_sys().get_arbiters().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for FanArbiters { }
    impl @IntoIterator(into_arbiters() -> FanArbiter) for FanArbiters { }

    impl FanArbiters {
        pub fn into_arbiters(@iter self) -> FanArbiter;
    }
}

nvwrap! {
    pub enum FanArbiterStatus {
        V1(FanArbiterStatusV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITER_STATUS_V1> {
                pub fan_stopped: bool {
                    @get fn(&self) {
                        self.sys().fan_stop_active()
                    },
                },
            },
        }),
    }

    impl @TaggedData(@i32) for FanArbiterStatus { }

    impl FanArbiterStatus {
        pub fn fan_stopped(&self) -> bool;
    }
}

nvwrap! {
    pub enum FanArbitersStatus {
        V1(FanArbitersStatusV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_STATUS_V1> {
                pub arbiters: @iter(FanArbiterStatusV1) {
                    @into fn into_arbiters(self) {
                        self.into_sys().get_arbiters().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for FanArbitersStatus { }
    impl @IntoIterator(into_arbiters() -> FanArbiterStatus) for FanArbitersStatus { }

    impl FanArbitersStatus {
        pub fn into_arbiters(@iter self) -> FanArbiterStatus;
    }
}

nvwrap! {
    pub enum FanArbiterControl {
        V1(FanArbiterControlV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITER_CONTROL_V1> {
                pub flags: cooler::private::FanArbiterControlFlags {
                    @get fn(&self) {
                        self.sys().flags.truncate()
                    },
                },
            },
        }),
    }

    impl @TaggedData(@i32) for FanArbiterControl { }

    impl FanArbiterControl {
        pub fn flags(&self) -> cooler::private::FanArbiterControlFlags;
    }
}

nvwrap! {
    pub enum FanArbitersControl {
        V1(FanArbitersControlV1 {
            @type = NvData<cooler::private::NV_GPU_CLIENT_FAN_ARBITERS_CONTROL_V1> {
                pub arbiters: @iter(FanArbiterControlV1) {
                    @into fn into_arbiters(self) {
                        self.into_sys().get_arbiters().into_iter()
                            .map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for FanArbitersControl { }
    impl @IntoIterator(into_arbiters() -> FanArbiterControl) for FanArbitersControl { }

    impl FanArbitersControl {
        pub fn into_arbiters(@iter self) -> FanArbiterControl;
    }
}
