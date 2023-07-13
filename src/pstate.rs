use std::marker::PhantomData;

use crate::sys::gpu::pstate;
use crate::types::{Microvolts, MicrovoltsDelta, Kilohertz, KilohertzDelta, Percentage, Range, NvData, NvValue, Tagged};
use crate::clock::ClockDomain;

pub use crate::sys::gpu::pstate::{PstateId as PState, VoltageInfoDomain as VoltageDomain, UtilizationDomain};

nvwrap! {
    pub enum PStateSettings {
        V1(PStateSettingsV1 {
            @type = NvData<pstate::NV_GPU_PERF_PSTATES20_PSTATE> {
                pub id: PState {
                    @get fn(&self) {
                        self.sys().pstateId.get()
                    },
                },
                pub editable: bool {
                    @get fn(&self) {
                        self.sys().bIsEditable.get()
                    },
                },
                pub clocks: Vec<ClockEntry> {},
                pub base_voltages: Vec<BaseVoltage> {},
            /*
            clocks: settings.clocks[..num_clocks].iter().map(TryFrom::try_from).collect::<Result<_, _>>()?,
            base_voltages: settings.baseVoltages[..num_base_voltages].iter().map(TryFrom::try_from).collect::<Result<_, _>>()?,*/
            },
        }),
    }

    impl @TaggedData for PStateSettings { }

    impl PStateSettings {
        pub fn id(&self) -> PState;
    }
}

nvwrap! {
    pub type PStatesV2 = NvData<pstate::NV_GPU_PERF_PSTATES20_INFO_V2> {
    };

    impl @Deref(v1: PStatesV1) for PStatesV2 { }
}

nvwrap! {
    pub enum PStates {
        V2(PStatesV2),
        V1(PStatesV1 {
            @type = NvData<pstate::NV_GPU_PERF_PSTATES20_INFO_V1> {
                pub editable: bool {
                    @get fn(&self) {
                        self.sys().bIsEditable.get()
                    },
                },
                pub pstates: @iter(PStateSettingsV1) {
                    @into fn into_pstates(self) {
                        self.sys().get_pstates().into_iter().map(Into::into)
                    },
                },
    /*pub pstates: Vec<PStateSettings>,
    pub overvolt: Vec<BaseVoltage>,
        let pstates = info.pstates
            .get(..info.numPstates as usize)
            .ok_or(sys::ArgumentRangeError)?
            .iter().map(|ps| PStateSettings::from_raw(ps, info.numClocks as _, info.numBaseVoltages as _))
            .collect::<Result<_, _>>()?;
        let overvolt = info.voltages
            .get(..info.numVoltages as usize)
            .ok_or(sys::ArgumentRangeError)?
            .iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
*/
            },
        }),
    }

    impl @StructVersion for PStates { }
    impl @IntoIterator(into_pstates() -> PStateSettings) for PStates { }
    impl @Deref(PStatesV1) for PStates { }

    impl PStates {
        pub fn into_pstates(@iter self) -> PStateSettings;
    }
}

impl PStatesV1 {
    pub fn clocks<'a>(&'a self, pstate: &'a PStateSettingsV1) -> impl Iterator<Item = &'a ClockEntryV1> + 'a {
        self.sys().clocks(pstate.sys()).into_iter()
            .map(From::from)
    }

    pub fn base_voltages<'a>(&'a self, pstate: &'a PStateSettingsV1) -> impl Iterator<Item = &'a BaseVoltageV1> + 'a {
        self.sys().base_voltages(pstate.sys()).into_iter()
            .map(From::from)
    }
}

nvwrap! {
    pub enum BaseVoltage {
        V1(BaseVoltageV1 {
            @type = NvData<pstate::NV_GPU_PERF_PSTATE20_BASE_VOLTAGE_ENTRY_V1> {
                pub id: NvValue<VoltageDomain> {
                    @sys(domainId),
                },
                pub editable: bool {
                    @sys@BoolU32(bIsEditable),
                },
                pub voltage: Microvolts {
                    @sys(volt_uV),
                },
                pub delta: PStateDeltaV1 {
                    @sys(voltDelta_uV),
                },
                pub voltage_delta: Delta<MicrovoltsDelta> {
                    @get fn(&self) {
                        Delta::new(self.delta().into())
                    },
                },
            },
        }),
    }

    impl @TaggedData for BaseVoltage { }

    impl BaseVoltage {
        pub fn id(&self) -> NvValue<VoltageDomain>;
        pub fn editable(&self) -> bool;
        pub fn voltage(&self) -> Microvolts;
        pub fn voltage_delta(&self) -> Delta<MicrovoltsDelta>;
    }
}

nvwrap! {
    pub enum ClockEntryData {
        Single(ClockEntrySingleV1 {
            @type = NvData<pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_SINGLE> {
                pub frequency: Kilohertz {
                    @sys(freq_kHz),
                },
            },
        }),
        Range(ClockEntryRangeV1 {
            @type = NvData<pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_RANGE> {
                pub frequency_range: Range<Kilohertz> {
                    @get fn(&self) {
                        Range {
                            min: Kilohertz(self.sys().minFreq_kHz),
                            max: Kilohertz(self.sys().maxFreq_kHz),
                        }
                    },
                },
                pub voltage_domain: NvValue<VoltageDomain> {
                    @sys(domainId),
                },
                pub voltage_range: Range<Microvolts> {
                    @get fn(&self) {
                        Range {
                            min: Microvolts(self.sys().minVoltage_uV),
                            max: Microvolts(self.sys().maxVoltage_uV),
                        }
                    },
                },
            },
        }),
    }
}

impl From<pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE> for ClockEntryData {
    fn from(value: pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE) -> Self {
        match value {
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Single(data) => ClockEntrySingleV1::from(data).into(),
            pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_DATA_VALUE::Range(data) => ClockEntryRangeV1::from(data).into(),
        }
    }
}

nvwrap! {
    pub enum ClockEntry {
        V1(ClockEntryV1 {
            @type = NvData<pstate::NV_GPU_PSTATE20_CLOCK_ENTRY_V1> {
                pub id: NvValue<ClockDomain> {
                    @sys(domainId),
                },
                pub editable: bool {
                    @sys@BoolU32(bIsEditable),
                },
                pub delta: PStateDeltaV1 {
                    @sys(freqDelta_kHz),
                },
                pub frequency_delta: Delta<KilohertzDelta> {
                    @get fn(&self) {
                        Delta::new(self.delta().into())
                    },
                },
                pub data: ClockEntryData {
                    @get fn(&self) {
                        self.sys().data.get(self.sys().typeId.get()).into()
                    },
                },
            },
        }),
    }

    impl @TaggedData for ClockEntry { }

    impl ClockEntry {
        pub fn id(&self) -> NvValue<ClockDomain>;
        pub fn editable(&self) -> bool;
        pub fn frequency_delta(&self) -> Delta<KilohertzDelta>;
        pub fn data(&self) -> ClockEntryData;
    }
}

nvwrap! {
    pub enum PStateDelta {
        V1(PStateDeltaV1 {
            @type = NvData<pstate::NV_GPU_PERF_PSTATES20_PARAM_DELTA> {
                pub value: i32 {
                    @sys,
                },
                pub range: Range<i32> {
                    @get fn(&self) {
                        Range {
                            min: self.sys().min,
                            max: self.sys().max,
                        }
                    },
                },
            },
        }),
    }

    impl PStateDelta {
        pub fn value(&self) -> i32;
        pub fn range(&self) -> Range<i32>;
    }
}

pub struct Delta<T> {
    pub data: PStateDelta,
    _unit: PhantomData<T>,
}

impl<T> Delta<T> {
    pub const fn new(data: PStateDelta) -> Self {
        Self {
            data,
            _unit: PhantomData,
        }
    }
}

impl<T: From<i32>> Delta<T> {
    pub fn value(&self) -> T {
        self.data.value().into()
    }

    pub fn range(&self) -> Range<T> {
        self.data.range().map(Into::into)
    }
}

impl Delta<KilohertzDelta> {
    pub fn frequency(&self) -> KilohertzDelta {
        self.value()
    }
}

impl Delta<MicrovoltsDelta> {
    pub fn voltage(&self) -> MicrovoltsDelta {
        self.value()
    }
}

nvwrap! {
    pub enum Utilization {
        V1(UtilizationV1 {
            @type = NvData<pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX_UTILIZATION> {
                pub usage: Percentage {
                    @get fn(&self) {
                        Percentage(self.sys().percentage)
                    },
                },
            },
        }),
    }

    impl @TaggedFrom(NvValue<UtilizationDomain>) for Utilization { }

    impl Utilization {
        pub fn usage(&self) -> Percentage;
    }
}

nvwrap! {
    pub enum Utilizations {
        V1(UtilizationsV1 {
            @type = NvData<pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX> {
                pub utilizations: @iter(Tagged<NvValue<UtilizationDomain>, UtilizationV1>) {
                    @into fn into_utilizations(self) {
                        self.sys().utilization().map(Tagged::from)
                    },
                },
            },
        }),
    }

    impl @StructVersion for Utilizations { }
    impl @IntoIterator(into_utilizations() -> Tagged<NvValue<UtilizationDomain>, Utilization>) for Utilizations { }

    impl Utilizations {
        pub fn into_utilizations(@iter self) -> Tagged<NvValue<UtilizationDomain>, Utilization>;
    }
}
