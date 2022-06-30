use std::collections::BTreeMap;
use std::convert::Infallible;
use std::{iter, slice};
use crate::sys::gpu::{clock, power};
use crate::sys;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use log::trace;
use crate::types::{Kilohertz, Kilohertz2, KilohertzDelta, Kilohertz2Delta, Percentage, Percentage1000, Microvolts, CelsiusShifted, Range, RawConversion};

pub use sys::gpu::clock::PublicClockId as ClockDomain;
pub use sys::gpu::clock::private::ClockLockMode;
pub use sys::gpu::power::private::PerfFlags;

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
    pub mask: [u32; 4],
    pub types: Vec<VfpMaskType>,
}

impl VfpMask {
    pub fn get_bit(mask: &[u32; 4], mut bit: usize) -> bool {
        let mut mask = &mask[..];
        while bit >= 32 {
            mask = &mask[1..];
            bit -= 32;
        }

        mask[0] & (1u32 << bit) != 0
    }

    pub fn set_bit(mut mask: &mut [u32], mut bit: usize) {
        while bit >= 32 {
            mask = &mut { mask }[1..];
            bit -= 32;
        }

        mask[0] |= 1u32 << bit
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for &'a VfpMask {
    type Item = (usize, VfpMaskType);
    type IntoIter = iter::Zip<VfpMaskIter<'a>, iter::Cloned<slice::Iter<'a, VfpMaskType>>>;

    fn into_iter(self) -> Self::IntoIter {
        VfpMaskIter::new(&self.mask).zip(self.types.iter().cloned())
    }
}

pub struct VfpMaskIter<'a> {
    mask: &'a [u32],
    offset: usize,
}

impl<'a> VfpMaskIter<'a> {
    pub fn new(mask: &'a [u32]) -> Self {
        VfpMaskIter {
            mask: mask,
            offset: 0,
        }
    }
}

impl<'a> Iterator for VfpMaskIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.mask.len() > 0 {
            let offset = self.offset;
            let bit = offset % 32;
            let set = self.mask[0] & (1u32 << bit) != 0;

            self.offset += 1;
            if bit == 31 {
                self.mask = &self.mask[1..]
            }

            if set {
                return Some(offset)
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.mask.len() * 32 - (self.offset % 32)))
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
    type Target = VfpMaskType;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 1, gpuDelta: 0,
            } => Ok(VfpMaskType::Memory),
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 0, gpuDelta: 1,
            } => Ok(VfpMaskType::Graphics),
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 0, gpuDelta: 0,
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
            types: VfpMaskIter::new(&self.mask)
                .filter_map(|i| self.clocks.get(i))
                .map(RawConversion::convert_raw)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockTable {
    pub gpu_delta: Vec<(usize, Kilohertz2Delta)>,
    pub mem_delta: Vec<(usize, KilohertzDelta)>,
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_GPU_DELTA {
    type Target = i32;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_GPU_DELTA {
                a: 0, b: 0, c: 0, d: 0, e: 0, freqDeltaKHz, g: 0, h: 0, i: 0,
            } => Ok(freqDeltaKHz),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL {
    type Target = ClockTable;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        // TODO: validate everything else is 0!

        Ok(ClockTable {
            gpu_delta: VfpMaskIter::new(&self.mask)
                .filter(|&i| i < self.gpuDeltas.len())
                .map(|i| (i, &self.gpuDeltas[i]))
                .map(|(i, delta)| delta.convert_raw().map(|delta| (i, delta.into())))
                .collect::<Result<_, _>>()?,
            mem_delta: self.memFilled.iter().enumerate().filter_map(|(i, &filled)| match filled {
                1 => Some(Ok(i)),
                0 => None,
                _ => Some(Err(sys::ArgumentRangeError)),
            }).map(|i| i.map(|i| (self.gpuDeltas.len() + i, self.memDeltas[i].into())))
            .collect::<Result<_, _>>()?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ClockRange {
    pub domain: ClockDomain,
    pub range: Range<Kilohertz2Delta>,
    /// unsure???
    pub temp_max: CelsiusShifted,
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY {
    type Target = ClockRange;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_ENTRY {
                a: 0, clockType, c: 0, d: 0, e: 0, f: 0, g: 0, h: 0, i: 0,
                j: 0, rangeMax, rangeMin, tempMax, n: 0, o: 0, p: 0, q: 0, r: 0,
            } => Ok(ClockRange {
                domain: ClockDomain::from_raw(clockType)?,
                range: Range {
                    max: Kilohertz2Delta(rangeMax),
                    min: Kilohertz2Delta(rangeMin),
                },
                temp_max: CelsiusShifted(tempMax),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO {
    type Target = Vec<ClockRange>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries[..(self.numClocks - 1) as usize].iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfpEntry<K> {
    /// 1 for idle values / low pstates? only populated for memory clocks
    pub unknown: u32,
    pub frequency: K,
    pub voltage: Microvolts,
}

impl<T> VfpEntry<T> {
    pub fn from_entry<K>(e: VfpEntry<K>) -> Self where T: From<K> {
        VfpEntry {
            unknown: e.unknown,
            frequency: e.frequency.into(),
            voltage: e.voltage,
        }
    }
}

impl RawConversion for power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_GPU_ENTRY {
    type Target = VfpEntry<u32>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_GPU_ENTRY {
                a, freq_kHz, voltage_uV, d: 0, e: 0, f: 0, g: 0,
            } => Ok(VfpEntry {
                unknown: a,
                frequency: freq_kHz,
                voltage: Microvolts(voltage_uV),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VfpCurve {
    pub graphics: Vec<(usize, VfpEntry<Kilohertz2>)>,
    pub memory: Vec<(usize, VfpEntry<Kilohertz>)>,
}

impl RawConversion for power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS {
    type Target = VfpCurve;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VfpCurve {
            graphics: VfpMaskIter::new(&self.mask)
                .filter(|&i| i < self.gpuEntries.len())
                .map(|i| self.gpuEntries[i].convert_raw().map(|e| (i, VfpEntry::from_entry(e))))
                .collect::<Result<_, _>>()?,
            memory: VfpMaskIter::new(&self.mask)
                .filter(|&i| i >= self.gpuEntries.len() && i < self.gpuEntries.len() + self.memEntries.len())
                .map(|i| self.memEntries[i - self.gpuEntries.len()].convert_raw().map(|e| (i, VfpEntry::from_entry(e))))
                .collect::<Result<_, _>>()?,
        })
    }
}

fn all_zero(s: &[u32]) -> bool {
    return s.iter().all(|&v| v == 0)
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
            } if all_zero(zero) && all_zero(unknown) => Ok(Microvolts(value_uV)),
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
            } if all_zero(unknown) => Percentage::from_raw(percent),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PowerInfoEntry {
    pub pstate: crate::pstate::PState,
    pub range: Range<Percentage1000>,
    pub default_limit: Percentage1000,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PowerInfo {
    pub valid: bool,
    pub entries: Vec<PowerInfoEntry>,
}

impl RawConversion for power::private::NV_GPU_POWER_INFO_ENTRY {
    type Target = PowerInfoEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_POWER_INFO_ENTRY {
                pstate, b: 0, c: 0, min_power, e: 0, f: 0,
                def_power, h: 0, i: 0, max_power, k: 0,
            } => Ok(PowerInfoEntry {
                pstate: crate::pstate::PState::from_raw(pstate as _)?,
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

impl RawConversion for power::private::NV_GPU_POWER_INFO {
    type Target = PowerInfo;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(PowerInfo {
            valid: self.valid != 0,
            entries: self.entries[..self.count as usize].iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

impl RawConversion for power::private::NV_GPU_POWER_TOPO_ENTRY {
    type Target = Percentage1000;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_POWER_TOPO_ENTRY {
                a: unknown, b: 0, power, d: 0
            } => Ok(Percentage1000(power)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_POWER_TOPO {
    type Target = Vec<Percentage1000>;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        self.entries[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for power::private::NV_GPU_POWER_STATUS_ENTRY {
    type Target = Percentage1000;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            power::private::NV_GPU_POWER_STATUS_ENTRY {
                a: 0, b: 0, power, d: 0,
            } => Ok(Percentage1000(power)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_GPU_POWER_STATUS {
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
pub struct ClockLockEntry {
    pub mode: ClockLockMode,
    pub voltage: Microvolts,
}

impl RawConversion for clock::private::NV_GPU_PERF_CLIENT_LIMITS_ENTRY {
    type Target = ClockLockEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        match *self {
            clock::private::NV_GPU_PERF_CLIENT_LIMITS_ENTRY {
                id: _id, b: 0, mode, d: 0, voltage_uV, f: 0,
            } => Ok(ClockLockEntry {
                mode: ClockLockMode::from_raw(mode)?,
                voltage: Microvolts(voltage_uV),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_GPU_PERF_CLIENT_LIMITS {
    type Target = BTreeMap<usize, ClockLockEntry>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        if self.flags != 0 {
            Err(sys::ArgumentRangeError)
        } else {
            self.entries[..self.count as usize].iter().map(|v| v.convert_raw().map(|e| (v.id as usize, e))).collect()
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PerfInfo {
    pub max_unknown: u32,
    pub limits: PerfFlags,
}

impl RawConversion for power::private::NV_GPU_PERF_INFO {
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

impl RawConversion for power::private::NV_GPU_PERF_STATUS {
    type Target = PerfStatus;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        // TODO: check padding
        match *self {
            power::private::NV_GPU_PERF_STATUS {
                flags: 0, limits, zero0: 0, unknown, zero1: 0, ..
            } => Ok(PerfStatus {
                unknown: unknown,
                limits: PerfFlags::from_bits(limits).ok_or(sys::ArgumentRangeError)?,
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct VoltageEntry {
    pub unknown: u32,
    pub voltage: Microvolts,
}

impl RawConversion for power::private::NV_VOLT_TABLE_ENTRY {
    type Target = VoltageEntry;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:#?})", self);
        Ok(VoltageEntry {
            unknown: self.unknown,
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
            entries: self.entries.iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
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
    pub unknown1: [u32; 30],
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
            unknown1: self.buf1,
        })
    }
}
