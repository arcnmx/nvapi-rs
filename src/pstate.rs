use std::collections::BTreeMap;
use void::{Void, ResultVoidExt};
use sys::gpu::pstate;
use sys;
use types::{Microvolts, MicrovoltsDelta, Kilohertz, KilohertzDelta, Percentage, Range, Delta, RawConversion};
use clock::ClockDomain;

pub use sys::gpu::pstate::{PstateId as PState, VoltageInfoDomain as VoltageDomain, UtilizationDomain};

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PStateSettings {
    pub id: PState,
    pub editable: bool,
    pub clocks: Vec<ClockEntry>,
    pub base_voltages: Vec<BaseVoltage>,
}

#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct PStates {
    pub editable: bool,
    pub pstates: Vec<PStateSettings>,
    pub overvolt: Vec<BaseVoltage>,
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum ClockEntry {
    Single {
        domain: ClockDomain,
        editable: bool,
        frequency_delta: Delta<KilohertzDelta>,
        frequency: Kilohertz,
    },
    Range {
        domain: ClockDomain,
        editable: bool,
        frequency_delta: Delta<KilohertzDelta>,
        frequency_range: Range<Kilohertz>,
        voltage_domain: VoltageDomain,
        voltage_range: Range<Microvolts>,
    },
}

impl ClockEntry {
    pub fn domain(&self) -> ClockDomain {
        match *self {
            ClockEntry::Single { domain, .. } => domain,
            ClockEntry::Range { domain, .. } => domain,
        }
    }

    pub fn editable(&self) -> bool {
        match *self {
            ClockEntry::Single { editable, .. } => editable,
            ClockEntry::Range { editable, .. } => editable,
        }
    }

    pub fn frequency_delta(&self) -> Delta<KilohertzDelta> {
        match *self {
            ClockEntry::Single { frequency_delta, .. } => frequency_delta,
            ClockEntry::Range { frequency_delta, .. } => frequency_delta,
        }
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct BaseVoltage {
    pub voltage_domain: VoltageDomain,
    pub editable: bool,
    pub voltage: Microvolts,
    pub voltage_delta: Delta<MicrovoltsDelta>,
}

impl PStateSettings {
    pub fn from_raw(settings: &pstate::NV_GPU_PERF_PSTATES20_PSTATE, num_clocks: usize, num_base_voltages: usize) -> Result<Self, sys::ArgumentRangeError> {
        Ok(PStateSettings {
            id: PState::from_raw(settings.pstateId)?,
            editable: settings.bIsEditable.get(),
            clocks: settings.clocks[..num_clocks].iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
            base_voltages: settings.baseVoltages[..num_base_voltages].iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

impl RawConversion for pstate::NV_GPU_PERF_PSTATES20_INFO_V2 {
    type Target = PStates;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(PStates {
            editable: self.bIsEditable.get(),
            pstates: self.pstates[..self.numPstates as usize].iter().map(|ps| PStateSettings::from_raw(ps, self.numClocks as _, self.numBaseVoltages as _)).collect::<Result<_, _>>()?,
            overvolt: self.voltages[..self.numVoltages as usize].iter().map(RawConversion::convert_raw).collect::<Result<_, _>>()?,
        })
    }
}

impl RawConversion for pstate::NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1 {
    type Target = BaseVoltage;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(BaseVoltage {
            voltage_domain: VoltageDomain::from_raw(self.domainId)?,
            editable: self.bIsEditable.get(),
            voltage: Microvolts(self.volt_uV),
            voltage_delta: match self.voltDelta_uV.convert_raw().void_unwrap() {
                Delta { value, range } => Delta {
                    value: MicrovoltsDelta(value.0),
                    range: Range {
                        min: MicrovoltsDelta(range.min.0),
                        max: MicrovoltsDelta(range.max.0),
                    },
                },
            },
        })
    }
}

impl RawConversion for pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_V1 {
    type Target = ClockEntry;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(match self.data.get(pstate::PstateClockType::from_raw(self.typeId)?) {
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Single(single) => ClockEntry::Single {
                domain: ClockDomain::from_raw(self.domainId)?,
                editable: self.bIsEditable.get(),
                frequency_delta: self.freqDelta_kHz.convert_raw().void_unwrap(),
                frequency: Kilohertz(single.freq_kHz),
            },
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Range(range) => ClockEntry::Range {
                domain: ClockDomain::from_raw(self.domainId)?,
                editable: self.bIsEditable.get(),
                frequency_delta: self.freqDelta_kHz.convert_raw().void_unwrap(),
                frequency_range: Range {
                    min: Kilohertz(range.minFreq_kHz),
                    max: Kilohertz(range.maxFreq_kHz),
                },
                voltage_domain: VoltageDomain::from_raw(range.domainId)?,
                voltage_range: Range {
                    min: Microvolts(range.minVoltage_uV),
                    max: Microvolts(range.maxVoltage_uV),
                },
            },
        })
    }
}

impl RawConversion for pstate::NV_GPU_PERF_PSTATES20_PARAM_DELTA {
    type Target = Delta<KilohertzDelta>;
    type Error = Void;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(Delta {
            value: KilohertzDelta(self.value),
            range: Range {
                min: KilohertzDelta(self.min),
                max: KilohertzDelta(self.max),
            },
        })
    }
}

impl RawConversion for pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX {
    type Target = BTreeMap<pstate::UtilizationDomain, Percentage>;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        if self.flag_enabled() {
            Ok(BTreeMap::new())
        } else {
            pstate::UtilizationDomain::values()
                .map(|domain| (domain, &self.utilization[domain.raw() as usize]))
                .filter(|&(_, util)| util.bIsPresent.get())
                .map(|(id, util)| Percentage::from_raw(util.percentage).map(|p| (id, p)))
                .collect()
        }
    }
}
