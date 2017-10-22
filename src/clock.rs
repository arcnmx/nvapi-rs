use std::collections::BTreeMap;
use std::{iter, slice};
use sys::gpu::{clock, power};
use sys;
use void::Void;
use types::{Kilohertz, Kilohertz2, Kilohertz2Delta, Percentage, Percentage1000, Microvolts, CelsiusShifted, Range, RawConversion};

pub use sys::gpu::clock::PublicClockId as ClockDomain;

impl RawConversion for clock::NV_GPU_CLOCK_FREQUENCIES {
    type Target = BTreeMap<ClockDomain, Kilohertz>;
    type Error = Void;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(ClockDomain::values().filter(|&c| c != ClockDomain::Undefined)
            .map(|id| (id, &self.domain[id.raw() as usize]))
            .filter(|&(_, ref clock)| clock.bIsPresent.get())
            .map(|(id, clock)| (id, Kilohertz(clock.frequency)))
            .collect()
        )
    }
}

impl RawConversion for clock::private::NV_USAGES_INFO {
    type Target = BTreeMap<::pstate::UtilizationDomain, Percentage>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        self.usages.iter().enumerate()
            .filter(|&(_, ref usage)| usage.bIsPresent.get())
            .map(|(i, usage)| ::pstate::UtilizationDomain::from_raw(i as _)
                .and_then(|i| Percentage::from_raw(usage.percentage).map(|p| (i, p)))
            ).collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum VfpMaskType {
    Graphics,
    Memory,
    Unknown,
}

#[derive(Debug, Clone)]
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

impl RawConversion for clock::private::NV_CLOCK_MASKS_CLOCK {
    type Target = VfpMaskType;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            clock::private::NV_CLOCK_MASKS_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 1, gpuDelta: 0,
            } => Ok(VfpMaskType::Memory),
            clock::private::NV_CLOCK_MASKS_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 0, gpuDelta: 1,
            } => Ok(VfpMaskType::Graphics),
            clock::private::NV_CLOCK_MASKS_CLOCK {
                a: 0, b: 0, c: 0, d: 0, memDelta: 0, gpuDelta: 0,
            } => Ok(VfpMaskType::Unknown),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_CLOCK_MASKS {
    type Target = VfpMask;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        // TODO: validate everything else is 0!

        Ok(VfpMask {
            mask: self.mask,
            types: VfpMaskIter::new(&self.mask)
                .map(|i| &self.clocks.inner()[i])
                .map(RawConversion::convert_raw)
                .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ClockTable {
    pub gpu_delta: Vec<(usize, Kilohertz2Delta)>,
    pub mem_delta: Vec<(usize, Kilohertz2Delta)>,
}

impl RawConversion for clock::private::NV_CLOCK_TABLE_GPU_DELTA {
    type Target = Kilohertz2Delta;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            clock::private::NV_CLOCK_TABLE_GPU_DELTA {
                a: 0, b: 0, c: 0, d: 0, e: 0, freqDeltaKHz, g: 0, h: 0, i: 0,
            } => Ok(Kilohertz2Delta(freqDeltaKHz)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for clock::private::NV_CLOCK_TABLE {
    type Target = ClockTable;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        // TODO: validate everything else is 0!

        Ok(ClockTable {
            gpu_delta: VfpMaskIter::new(&self.mask)
                .filter(|&i| i < self.gpuDeltas.len())
                .map(|i| (i, &self.gpuDeltas.inner()[i]))
                .map(|(i, delta)| delta.convert_raw().map(|delta| (i, delta)))
                .collect::<Result<_, _>>()?,
            mem_delta: self.memFilled.iter().enumerate().filter_map(|(i, &filled)| match filled {
                1 => Some(Ok(i)),
                0 => None,
                _ => Some(Err(sys::ArgumentRangeError)),
            }).map(|i| i.map(|i| (self.gpuDeltas.len() + i, Kilohertz2Delta(self.memDeltas[i]))))
            .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClockRange {
    pub domain: ClockDomain,
    pub range: Range<Kilohertz2Delta>,
    /// unsure???
    pub temp_max: CelsiusShifted,
}

impl RawConversion for clock::private::NV_CLOCK_RANGES_ENTRY {
    type Target = ClockRange;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            clock::private::NV_CLOCK_RANGES_ENTRY {
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

impl RawConversion for clock::private::NV_CLOCK_RANGES {
    type Target = Vec<ClockRange>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        self.entries[..(self.numClocks - 1) as usize].iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VfpEntry {
    /// 1 for idle values / low pstates? only populated for memory clocks
    pub unknown: u32,
    pub frequency: Kilohertz2,
    pub voltage: Microvolts,
}

impl RawConversion for power::private::NV_VFP_CURVE_GPU_ENTRY {
    type Target = VfpEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            power::private::NV_VFP_CURVE_GPU_ENTRY {
                a, freq_kHz, voltage_uV, d: 0, e: 0, f: 0, g: 0,
            } => Ok(VfpEntry {
                unknown: a,
                frequency: Kilohertz2(freq_kHz),
                voltage: Microvolts(voltage_uV),
            }),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VfpCurve {
    pub graphics: Vec<(usize, VfpEntry)>,
    pub memory: Vec<(usize, VfpEntry)>,
}

impl RawConversion for power::private::NV_VFP_CURVE {
    type Target = VfpCurve;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(VfpCurve {
            graphics: VfpMaskIter::new(&self.mask)
                .filter(|&i| i < self.gpuEntries.len())
                .map(|i| self.gpuEntries[i].convert_raw().map(|e| (i, e)))
                .collect::<Result<_, _>>()?,
            memory: VfpMaskIter::new(&self.mask)
                .filter(|&i| i >= self.gpuEntries.len())
                .map(|i| self.memEntries[i - self.gpuEntries.len()].convert_raw().map(|e| (i, e)))
                .collect::<Result<_, _>>()?,
        })
    }
}

fn all_zero(s: &[u32]) -> bool {
    return s.iter().all(|&v| v == 0)
}

impl RawConversion for power::private::NV_VOLTAGE_STATUS_V1 {
    type Target = Microvolts;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            power::private::NV_VOLTAGE_STATUS_V1 {
                version: _, flags: 0, ref zero,
                value_uV, ref unknown,
            } if all_zero(zero) && all_zero(unknown) => Ok(Microvolts(value_uV)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

impl RawConversion for power::private::NV_VOLTAGE_BOOST_PERCENT_V1 {
    type Target = Percentage;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            power::private::NV_VOLTAGE_BOOST_PERCENT {
                version: _, percent, ref unknown,
            } if all_zero(unknown) => Percentage::from_raw(percent),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerInfoEntry {
    pub pstate: ::pstate::PState,
    pub range: Range<Percentage1000>,
    pub default_limit: Percentage1000,
}

#[derive(Debug, Clone)]
pub struct PowerInfo {
    pub valid: bool,
    pub entries: Vec<PowerInfoEntry>,
}

impl RawConversion for power::private::NV_GPU_POWER_INFO_ENTRY {
    type Target = PowerInfoEntry;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            power::private::NV_GPU_POWER_INFO_ENTRY {
                pstate, b: 0, c: 0, min_power, e: 0, f: 0,
                def_power, h: 0, i: 0, max_power, k: 0,
            } => Ok(PowerInfoEntry {
                pstate: ::pstate::PState::from_raw(pstate as _)?,
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
        self.entries[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}

impl RawConversion for power::private::NV_GPU_POWER_STATUS_ENTRY {
    type Target = Percentage1000;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
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
        self.entries[..self.count as usize].iter().map(RawConversion::convert_raw).collect()
    }
}
