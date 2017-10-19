use std::collections::BTreeMap;
use std::{iter, slice};
use sys::gpu::{clock, power};
use sys;
use void::Void;
use types::{Kilohertz, KilohertzDelta, Percentage, Microvolts, CelsiusShifted, RawConversion};

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
    type Target = BTreeMap<ClockDomain, Percentage>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        println!("usages: {:#?}", self);
        // TODO: validate everything else is 0!

        ClockDomain::values().filter(|&c| c != ClockDomain::Undefined)
            .map(|id| (id, &self.usages[id.raw() as usize]))
            .filter(|&(id, _)| self.count > id.raw() as u32)
            .map(|(id, usage)| Percentage::from_raw(usage.percentage).map(|p| (id, p)))
            .collect()
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
    pub fn get_bit(mask: &[u32; 4], bit: usize) -> bool {
        let mut mask = &mask[..];
        while bit > 32 {
            mask = &mask[1..];
        }

        mask[0] & (1u32 << bit) != 0
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
    gpu_delta: Vec<(usize, KilohertzDelta)>,
    mem_delta: Vec<(usize, KilohertzDelta)>,
}

impl RawConversion for clock::private::NV_CLOCK_TABLE_GPU_DELTA {
    type Target = KilohertzDelta;
    type Error = sys::ArgumentRangeError;

    #[allow(non_snake_case)]
    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        match *self {
            clock::private::NV_CLOCK_TABLE_GPU_DELTA {
                a: 0, b: 0, c: 0, d: 0, e: 0, freqDeltaKHz, g: 0, h: 0, i: 0,
            } => Ok(KilohertzDelta(freqDeltaKHz)),
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
            }).map(|i| i.map(|i| (self.gpuDeltas.len() + i, KilohertzDelta(self.memDeltas[i]))))
            .collect::<Result<_, _>>()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ClockRange {
    pub domain: ClockDomain,
    pub range_max: KilohertzDelta,
    pub range_min: KilohertzDelta,
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
                range_max: KilohertzDelta(rangeMax),
                range_min: KilohertzDelta(rangeMin),
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
        self.entries[..self.numClocks as usize].iter()
            .map(RawConversion::convert_raw)
            .collect::<Result<_, _>>()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VfpEntry {
    /// 1 for idle values / low pstates? only populated for memory clocks
    pub unknown: u32,
    pub frequency: Kilohertz,
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
                frequency: Kilohertz(freq_kHz),
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
                .map(|i| self.gpuEntries[i].convert_raw().map(|mut e| {
                    e.frequency.0 >>= 1;
                    (i, e)
                }))
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

    fn to_raw(s: &Self::Target) -> Self where Self: Sized {
        let mut raw = power::private::NV_VOLTAGE_BOOST_PERCENT::zeroed();
        raw.percent = s.0;
        raw
    }
}
