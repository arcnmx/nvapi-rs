use std::collections::BTreeMap;
use std::convert::Infallible;
use std::{iter, slice, fmt};
use crate::sys::gpu::{clock, power};
use crate::sys;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use log::trace;
use crate::sys::BoolU32;
use crate::sys::clock_mask::{ClockMask, ClockMaskIter};
use crate::gpu::VfpInfo;
use crate::types::{Kilohertz, Kilohertz2, KilohertzDelta, Kilohertz2Delta, Percentage, Percentage1000, Microvolts, Range, RawConversion};

pub use sys::gpu::clock::PublicClockId as ClockDomain;
pub use sys::gpu::clock::private::PerfLimitId;
pub use sys::gpu::power::private::{PerfFlags, PowerTopologyChannelId};

impl RawConversion for clock::NV_GPU_CLOCK_FREQUENCIES {
    type Target = BTreeMap<ClockDomain, Kilohertz>;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(ClockDomain::values().filter(|&c| c != ClockDomain::Undefined)
            .map(|id| (id, &self.domain[id.raw() as usize]))
            .filter(|&(_, ref clock)| clock.bIsPresent.get())
            .map(|(id, clock)| (id, Kilohertz(clock.frequency)))
            .collect()
        )
    }
}

impl RawConversion for clock::private::NV_USAGES_INFO {
    type Target = BTreeMap<crate::pstate::UtilizationDomain, Percentage>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.usages.iter().enumerate()
            .filter(|&(_, ref usage)| usage.bIsPresent.get())
            .map(|(i, usage)| crate::pstate::UtilizationDomain::from_raw(i as _)
                .and_then(|i| Percentage::from_raw(usage.percentage).map(|p| (i, p)))
            ).collect()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum VfpMaskType {
    Graphics,
    Memory,
    Unknown,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfpMask {
    pub mask: ClockMask,
    pub types: Vec<VfpMaskType>,
}

impl VfpMask {
    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a VfpMask {
    type Item = (usize, VfpMaskType);
    type IntoIter = iter::Zip<ClockMaskIter<'a>, iter::Cloned<slice::Iter<'a, VfpMaskType>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.mask.iter().zip(self.types.iter().cloned())
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
    type Target = VfpMaskType;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                memDelta: 1, gpuDelta: 0, unknown,
            } => Ok(VfpMaskType::Memory),
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                memDelta: 0, gpuDelta: 1, unknown,
            } => Ok(VfpMaskType::Graphics),
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                memDelta: 0, gpuDelta: 0, unknown,
            } => Ok(VfpMaskType::Unknown),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO {
    type Target = VfpMask;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        // TODO: validate everything else is 0!

        Ok(VfpMask {
            mask: self.mask,
            types: self.mask.iter()
                .filter_map(|i| self.clocks.get(i))
                .map(RawConversion::convert_raw)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockTable {
    pub delta_points: BTreeMap<ClockDomain, Vec<(usize, KilohertzDelta)>>,
}

impl ClockTable {
    pub fn from_raw(raw: &clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL, info: &VfpInfo) -> crate::Result<Self> {
        Ok(Self {
            delta_points: info.domains.domains.iter()
                .map(|d| info.index(d.domain, &raw.points[..])
                    .map(|(i, p)| p.convert_raw().map(|p| (i, p))).collect::<Result<_, _>>()
                    .map(|p| (d.domain, p))
                ).collect::<Result<_, _>>()?,
        })
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_V1 {
    type Target = KilohertzDelta;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_V1 {
                clock_type, freqDeltaKHz, unknown0, unknown1,
            } => Ok(freqDeltaKHz.into()),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockRange {
    pub domain: ClockDomain,
    pub range: Range<Kilohertz2Delta>,
    pub vfp_index: Range<usize>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockDomainInfo {
    pub domains: Vec<ClockRange>,
}

impl ClockDomainInfo {
    pub fn get(&self, domain: ClockDomain) -> Option<&ClockRange> {
        self.domains.iter()
            .find(|d| d.domain == domain)
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY {
    type Target = ClockRange;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY {
                disabled: BoolU32(0), clockType, rangeMax, rangeMin, vfpIndexMin, vfpIndexMax,
                unknown0, unknown1, padding,
            } => Ok(ClockRange {
                domain: ClockDomain::from_raw(clockType)?,
                range: Range {
                    max: Kilohertz2Delta(rangeMax),
                    min: Kilohertz2Delta(rangeMin),
                },
                vfp_index: Range {
                    min: vfpIndexMin as usize,
                    max: vfpIndexMax as usize,
                },
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO {
    type Target = ClockDomainInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        let domains = self.mask.index(&self.clocks[..])
            .map(|(_i, v)| v)
            .filter(|v| !v.disabled.get())
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()?;
        Ok(ClockDomainInfo {
            domains,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfPoint<T> {
    pub frequency: T,
    pub voltage: Microvolts,
}

impl<T: Default + PartialEq> VfPoint<T> {
    pub fn is_empty(&self) -> bool {
        self.voltage.0 == 0 && self.frequency == Default::default()
    }
}

impl<T> VfPoint<T> {
    pub fn from_entry<U>(e: VfPoint<U>) -> Self where T: From<U> {
        VfPoint {
            frequency: e.frequency.into(),
            voltage: e.voltage,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfpEntry<K> {
    /// 1 for idle values / low pstates? only populated for memory clocks
    pub unknown: u32,
    pub current: VfPoint<K>,
    pub default: VfPoint<K>,
    pub overclocked: VfPoint<K>,
}

impl<T> VfpEntry<T> {
    pub fn from_entry<K>(e: VfpEntry<K>) -> Self where T: From<K> {
        VfpEntry {
            unknown: e.unknown,
            current: VfPoint::from_entry(e.current),
            default: VfPoint::from_entry(e.default),
            overclocked: VfPoint::from_entry(e.overclocked),
        }
    }
}

impl<T: Default + PartialEq> VfpEntry<T> {
    pub fn configured(&self) -> &VfPoint<T> {
        match self.overclocked.is_empty() {
            false => &self.overclocked,
            true => &self.current,
        }
    }

    pub fn default(&self) -> Option<&VfPoint<T>> {
        match self.default.is_empty() {
            false => Some(&self.default),
            true => None,
        }
    }
}

impl<T: Default> From<VfPoint<T>> for VfpEntry<T> {
    fn from(current: VfPoint<T>) -> Self {
        Self {
            unknown: 0,
            current,
            default: Default::default(),
            overclocked: Default::default(),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT {
    type Target = VfPoint<u32>;
    type Error = Infallible;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VfPoint {
            frequency: self.freq_kHz,
            voltage: Microvolts(self.voltage_uV),
        })
    }
}

impl RawConversion for power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1 {
    type Target = VfPoint<u32>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1 {
                clock_type, point, unknown,
            } => point.convert_raw().map_err(Into::into),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3 {
    type Target = VfpEntry<u32>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3 {
                clock_type, point, point_default, point_overclocked, ..
            } => Ok(VfpEntry {
                unknown: clock_type,
                current: point.convert_raw()?,
                default: point_default.convert_raw()?,
                overclocked: point_overclocked.convert_raw()?,
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfpCurve {
    pub points: BTreeMap<ClockDomain, Vec<(usize, VfpEntry<Kilohertz>)>>,
}

impl VfpCurve {
    pub fn from_raw_v3(raw: &power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3, info: &VfpInfo) -> crate::Result<Self> {
        Ok(Self {
            points: info.domains.domains.iter()
                .map(|d| info.index(d.domain, &raw.entries[..])
                    .map(|(i, p)| p.convert_raw().map(|p| (i, VfpEntry::from_entry(p))))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|p| (d.domain, p))
                ).collect::<Result<_, _>>()?
        })
    }

    pub fn from_raw_v1(raw: &power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1, info: &VfpInfo) -> crate::Result<Self> {
        Ok(Self {
            points: info.domains.domains.iter()
                .map(|d| info.index(d.domain, &raw.entries[..])
                    .map(|(i, p)| p.convert_raw().map(|p| (i, VfPoint::from_entry(p).into())))
                    .collect::<Result<Vec<_>, _>>()
                    .map(|p| (d.domain, p))
                ).collect::<Result<_, _>>()?
        })
    }

    pub fn from_raw(raw: &power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS, info: &VfpInfo) -> crate::Result<Self> {
        Self::from_raw_v3(raw, info)
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 {
    type Target = Microvolts;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1 {
                version: _, flags: 0, ref zero,
                value_uV, ref unknown,
            } if zero.all_zero() && unknown.all_zero() => Ok(Microvolts(value_uV)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1 {
    type Target = Percentage;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL {
                version: _, percent, ref unknown,
            } if unknown.all_zero() => Percentage::from_raw(percent),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PowerInfoEntry {
    pub policy_id: power::private::NV_GPU_CLIENT_POWER_POLICIES_POLICY_ID,
    pub range: Range<Percentage1000>,
    pub default_limit: Percentage1000,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PowerInfo {
    pub valid: bool,
    pub entries: Vec<PowerInfoEntry>,
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V1 {
    type Target = PowerInfoEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V1 {
                policy_id, min_power, def_power, max_power,
                ..
            } => Ok(PowerInfoEntry {
                policy_id: policy_id.try_into()?,
                range: Range {
                    min: Percentage1000(min_power),
                    max: Percentage1000(max_power),
                },
                default_limit: Percentage1000(def_power),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V2 {
    type Target = PowerInfoEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_ENTRY_V2 {
                policy_id, min_power, def_power, max_power,
                ..
            } => Ok(PowerInfoEntry {
                policy_id: policy_id.try_into()?,
                range: Range {
                    min: Percentage1000(min_power),
                    max: Percentage1000(max_power),
                },
                default_limit: Percentage1000(def_power),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO {
    type Target = PowerInfo;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(PowerInfo {
            valid: self.valid != 0,
            entries: self.entries().iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY {
    type Target = (PowerTopologyChannelId, Percentage1000);
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY {
                channel, power, unknown0, unknown1,
            } => Ok((channel.try_into()?, Percentage1000(power))),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS {
    type Target = BTreeMap<PowerTopologyChannelId, Percentage1000>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries().iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V1 {
    type Target = Percentage1000;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V1 {
                policy_id, power_target, ..
            } => Ok(Percentage1000(power_target)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V2 {
    type Target = Percentage1000;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_ENTRY_V2 {
                policy_id, power_target, ..
            } => Ok(Percentage1000(power_target)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_INFO {
    type Target = Vec<PowerTopologyChannelId>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.channels().iter().copied()
            .map(|raw| raw.try_into())
            .collect()
    }
}

impl RawConversion for power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS {
    type Target = Vec<Percentage1000>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum ClockLockValue {
    Frequency(Kilohertz),
    Voltage(Microvolts),
}

impl ClockLockValue {
    pub fn value(&self) -> u32 {
        match self {
            ClockLockValue::Frequency(v) => v.0,
            ClockLockValue::Voltage(v) => v.0,
        }
    }

    pub fn voltage(&self) -> Option<Microvolts> {
        match self {
            &ClockLockValue::Voltage(v) => Some(v),
            _ => None,
        }
    }

    pub fn frequency(&self) -> Option<Kilohertz> {
        match self {
            &ClockLockValue::Frequency(v) => Some(v),
            _ => None,
        }
    }

    pub fn from_raw(raw: &clock::private::NV_GPU_PERF_CLIENT_LIMITS_ENTRY) -> Result<Option<Self>, sys::ArgumentRangeError> {
        Ok(match clock::private::ClockLockMode::from_raw(raw.mode)? {
            clock::private::ClockLockMode::None =>
                None,
            clock::private::ClockLockMode::ManualVoltage =>
                Some(ClockLockValue::Voltage(Microvolts(raw.value))),
            clock::private::ClockLockMode::ManualFrequency =>
                Some(ClockLockValue::Frequency(Kilohertz(raw.value))),
            _ => return Err(sys::ArgumentRangeError),
        })
    }
}

impl fmt::Display for ClockLockValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClockLockValue::Voltage(v) => fmt::Display::fmt(v, f),
            ClockLockValue::Frequency(v) => fmt::Display::fmt(v, f),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockLockEntry {
    pub limit: PerfLimitId,
    pub lock_value: Option<ClockLockValue>,
    pub clock: ClockDomain,
}

impl RawConversion for clock::private::NV_GPU_PERF_CLIENT_LIMITS_ENTRY {
    type Target = ClockLockEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_PERF_CLIENT_LIMITS_ENTRY {
                id, mode, value, clock_id, ..
            } => Ok(ClockLockEntry {
                limit: id.try_into()?,
                clock: clock_id.try_into()?,
                lock_value: ClockLockValue::from_raw(self)?,
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_PERF_CLIENT_LIMITS {
    type Target = Vec<ClockLockEntry>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        if self.flags != 0 {
            Err(sys::ArgumentRangeError)
        } else {
            self.entries().iter().map(RawConversion::convert_raw).collect()
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PerfInfo {
    pub max_unknown: u32,
    pub limits: PerfFlags,
}

impl RawConversion for power::private::NV_GPU_PERF_POLICIES_INFO_PARAMS {
    type Target = PerfInfo;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        // TODO: check padding
        Ok(PerfInfo {
            max_unknown: self.maxUnknown,
            limits: PerfFlags::from_bits(self.limitSupport).ok_or(sys::ArgumentRangeError)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PerfStatus {
    pub unknown: u32,
    pub limits: PerfFlags,
}

impl RawConversion for power::private::NV_GPU_PERF_POLICIES_STATUS_PARAMS {
    type Target = PerfStatus;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        // TODO: check padding
        match *self {
            power::private::NV_GPU_PERF_POLICIES_STATUS_PARAMS {
                flags: 0, limits, zero0: 0, unknown, zero1: 0, ..
            } => Ok(PerfStatus {
                unknown,
                limits: PerfFlags::from_bits(limits).ok_or(sys::ArgumentRangeError)?,
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VoltageEntry {
    pub voltage: Microvolts,
}

impl RawConversion for power::private::NV_VOLT_TABLE_ENTRY {
    type Target = VoltageEntry;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VoltageEntry {
            voltage: Microvolts(self.voltage_uV),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VoltageTable {
    pub flags: u32,
    pub entries: Vec<VoltageEntry>,
}

impl RawConversion for power::private::NV_VOLT_TABLE {
    type Target = VoltageTable;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VoltageTable {
            flags: self.flags,
            entries: self.entries().iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VoltageStatus {
    pub flags: u32,
    pub unknown0: u32,
    pub voltage: Microvolts,
    pub count: u32,
}

impl RawConversion for power::private::NV_VOLT_STATUS {
    type Target = VoltageStatus;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VoltageStatus {
            flags: self.flags,
            count: self.count,
            unknown0: self.unknown,
            voltage: Microvolts(self.value_uV),
        })
    }
}
