use std::fmt;
use sys::gpu::{pstate, clock};
use sys;

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Microvolts(pub u32);

impl fmt::Debug for Microvolts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mV", self.0 as f32 / 1000.0)
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Kilohertz(pub u32);

impl fmt::Debug for Kilohertz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 1000 {
            write!(f, "{} kHz", self.0)
        } else {
            write!(f, "{} MHz", self.0 as f32 / 1000.0)
        }
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Percentage(pub u32);

impl fmt::Debug for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} %", self.0)
    }
}

pub type PState = pstate::PstateId;

pub type VoltageDomain = pstate::VoltageInfoDomain;

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
pub struct Delta {
    pub value: i32,
    pub min: i32,
    pub max: i32,
}

pub type ClockDomain = clock::PublicClockId;

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum ClockEntry {
    Single {
        domain: ClockDomain,
        editable: bool,
        frequency_delta: Delta,
        frequency: Kilohertz,
    },
    Range {
        domain: ClockDomain,
        editable: bool,
        frequency_delta: Delta,
        min_frequency: Kilohertz,
        max_frequency: Kilohertz,
        voltage_domain: VoltageDomain,
        min_voltage: Microvolts,
        max_voltage: Microvolts,
    },
}

#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct BaseVoltage {
    pub voltage_domain: VoltageDomain,
    pub editable: bool,
    pub voltage: Microvolts,
    pub voltage_delta: Delta,
}

impl PStates {
    pub fn from_raw(info: &pstate::NV_GPU_PERF_PSTATES20_INFO_V2) -> sys::Result<Self> {
        Ok(PStates {
            editable: info.bIsEditable.get(),
            pstates: info.pstates[..info.numPstates as usize].iter().map(|ps| PStateSettings::from_raw(ps, info.numClocks as _, info.numBaseVoltages as _)).collect::<sys::Result<_>>()?,
            overvolt: info.voltages[..info.numVoltages as usize].iter().map(BaseVoltage::from_raw).collect::<sys::Result<_>>()?,
        })
    }
}

impl PStateSettings {
    pub fn from_raw(settings: &pstate::NV_GPU_PERF_PSTATES20_PSTATE, num_clocks: usize, num_base_voltages: usize) -> sys::Result<Self> {
        Ok(PStateSettings {
            id: PState::from_raw(settings.pstateId)?,
            editable: settings.bIsEditable.get(),
            clocks: settings.clocks[..num_clocks].iter().map(ClockEntry::from_raw).collect::<sys::Result<_>>()?,
            base_voltages: settings.baseVoltages[..num_base_voltages].iter().map(BaseVoltage::from_raw).collect::<sys::Result<_>>()?,
        })
    }
}

impl BaseVoltage {
    pub fn from_raw(s: &pstate::NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1) -> sys::Result<Self> {
        Ok(BaseVoltage {
            voltage_domain: VoltageDomain::from_raw(s.domainId)?,
            editable: s.bIsEditable.get(),
            voltage: Microvolts(s.volt_uV),
            voltage_delta: Delta::from_raw(&s.voltDelta_uV),
        })
    }
}

impl ClockEntry {
    pub fn from_raw(clock: &pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_V1) -> sys::Result<Self> {
        Ok(match clock.data.get(pstate::PstateClockType::from_raw(clock.typeId)?) {
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Single(single) => ClockEntry::Single {
                domain: ClockDomain::from_raw(clock.domainId)?,
                editable: clock.bIsEditable.get(),
                frequency_delta: Delta::from_raw(&clock.freqDelta_kHz),
                frequency: Kilohertz(single.freq_kHz),
            },
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Range(range) => ClockEntry::Range {
                domain: ClockDomain::from_raw(clock.domainId)?,
                editable: clock.bIsEditable.get(),
                frequency_delta: Delta::from_raw(&clock.freqDelta_kHz),
                min_frequency: Kilohertz(range.minFreq_kHz),
                max_frequency: Kilohertz(range.maxFreq_kHz),
                voltage_domain: VoltageDomain::from_raw(range.domainId)?,
                min_voltage: Microvolts(range.minVoltage_uV),
                max_voltage: Microvolts(range.maxVoltage_uV),
            },
        })
    }
}

impl Delta {
    pub fn from_raw(delta: &pstate::NV_GPU_PERF_PSTATES20_PARAM_DELTA) -> Self {
        Delta {
            value: delta.value,
            min: delta.min,
            max: delta.max,
        }
    }
}
