use crate::sys::gpu::{clock, power};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use crate::pstate::UtilizationDomain;
use crate::types::{Kilohertz, KilohertzDelta, Percentage, Percentage1000, Microvolts, Range, NvData, NvValue, Tagged};

pub use crate::sys::ClockMask;
pub use crate::sys::gpu::clock::PublicClockId as ClockDomain;
pub use crate::sys::gpu::pstate::VoltageInfoDomain as VoltageDomain;
pub use crate::sys::gpu::clock::private::{PerfLimitId, ClockLockMode};
pub use crate::sys::gpu::power::private::{PerfFlags, PowerTopologyChannelId as PowerTopologyChannel, PowerPolicyId};

nvwrap! {
    pub type ClockFrequencyV1 = NvData<clock::NV_GPU_CLOCK_FREQUENCIES_DOMAIN> {
        pub frequency: Kilohertz {
            @get fn(&self) {
                Kilohertz(self.sys().frequency)
            },
        },
    };
}

pub type ClockFrequency = ClockFrequencyV1;

nvwrap! {
    pub enum ClockFrequencies {
        V1(ClockFrequenciesV1 {
            @type = NvData<clock::NV_GPU_CLOCK_FREQUENCIES_V1> {
                pub clock_type@set(set_clock_type): NvValue<clock::ClockFrequencyType> {
                    @get fn(&self) {
                        self.sys().ClockType()
                    },
                    @set fn self value {
                        self.sys_mut().set_ClockType(value)
                    },
                },
                pub frequencies: @iter(Tagged<NvValue<ClockDomain>, ClockFrequencyV1>) {
                    @into fn into_frequencies(self) {
                        self.into_sys().domain().map(Tagged::from)
                    },
                },
            },
        }),
    }

    impl @StructVersion for ClockFrequencies { }
    impl @IntoIterator(into_frequencies() -> Tagged<NvValue<ClockDomain>, ClockFrequency>) for ClockFrequencies { }

    impl ClockFrequencies {
        pub fn into_frequencies(@iter self) -> Tagged<NvValue<ClockDomain>, ClockFrequency>;
    }
}

nvwrap! {
    pub enum Usage {
        V1(UsageV1 {
            @type = NvData<clock::private::NV_USAGES_INFO_USAGE> {
                pub percentage: Percentage {
                    @get fn(&self) {
                        Percentage(self.sys().percentage)
                    },
                },
            },
        }),
    }

    impl @TaggedFrom(NvValue<UtilizationDomain>) for Usage { }

    impl Usage {
        pub fn percentage(&self) -> Percentage;
    }
}

nvwrap! {
    pub enum Usages {
        V1(UsagesV1 {
            @type = NvData<clock::private::NV_USAGES_INFO_V1> {
                pub utilization: @iter(Tagged<NvValue<UtilizationDomain>, UsageV1>) {
                    @into fn into_utilizations(self) {
                        self.into_sys().usages().map(Tagged::from)
                    },
                },
            },
        }),
    }

    impl @StructVersion for Usages { }
    impl @IntoIterator(into_utilizations() -> Tagged<NvValue<UtilizationDomain>, Usage>) for Usages { }

    impl Usages {
        pub fn into_utilizations(@iter self) -> Tagged<NvValue<UtilizationDomain>, Usage>;
    }
}

/*nvconv! {
    fn try_from(info: &clock::private::NV_USAGES_INFO_V1) -> Result<Utilization, sys::ArgumentRangeError> {
        let utilization = info.usages.iter().enumerate()
            .filter(|&(_, ref usage)| usage.bIsPresent.get())
            .map(|(i, usage)| crate::pstate::UtilizationDomain::from_repr(i as _)
                .and_then(|i| Percentage::from_raw(usage.percentage).map(|p| (i, p)))
            ).collect::<Result<_, _>>()?;
        Ok(Self {
            utilization,
        })
    }
}*/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum VfpMaskType {
    Graphics,
    Memory,
    Unknown,
}

/*
impl<'a> IntoIterator for &'a VfpMask {
    type Item = (usize, VfpMaskType);
    type IntoIter = iter::Zip<ClockMaskIter<'a>, iter::Cloned<slice::Iter<'a, VfpMaskType>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.mask.iter().zip(self.types.iter().cloned())
    }
}
*/

nvwrap! {
    pub enum VfpMaskClock {
        V1(VfpMaskClockV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK> {
                pub kind: Option<VfpMaskType> {
                    @get fn(&self) {
                        match self.sys() {
                            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                                memDelta: 1, gpuDelta: 0, unknown: _,
                            } => Some(VfpMaskType::Memory),
                            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                                memDelta: 0, gpuDelta: 1, unknown: _,
                            } => Some(VfpMaskType::Graphics),
                            clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_CLOCK {
                                memDelta: 0, gpuDelta: 0, unknown: _,
                            } => Some(VfpMaskType::Unknown),
                            _ => None,
                        }
                    },
                },
            },
        }),
    }
}

nvwrap! {
    pub enum VfpMask {
        V1(VfpMaskV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO_V1> {
                pub mask: ClockMask {
                    @get fn(&self) {
                        self.sys().mask
                    },
                },
                pub clocks: @iter(VfpMaskClockV1) {
                    @into fn into_clocks(self) {
                        self.into_sys().into_clocks().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for VfpMask { }
    impl @IntoIterator(into_clocks() -> VfpMaskClock) for VfpMask { }

    impl VfpMask {
        pub fn into_clocks(@iter self) -> VfpMaskClock;
    }
}

nvwrap! {
    pub enum ClockTablePoint {
        V1(ClockTablePointV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_CONTROL_V1> {
                pub unknown: u32 {
                    @get fn(&self) {
                        self.sys().clock_type
                    },
                },
                pub offset: KilohertzDelta {
                    @get fn(&self) {
                        self.sys().freqDeltaKHz.into()
                    },
                },
            },
        }),
    }

    impl ClockTablePoint {
        pub fn offset(&self) -> KilohertzDelta;
    }
}

nvwrap! {
    pub enum ClockTable {
        V1(ClockTableV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL_V1> {
                pub mask: ClockMask {
                    @get fn(&self) {
                        self.sys().mask
                    },
                },
                pub points: @iter(ClockTablePointV1) {
                    @into fn into_points(self) {
                        self.into_sys().into_points().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for ClockTable { }
    impl @IntoIterator(into_points() -> ClockTablePoint) for ClockTable { }

    impl ClockTable {
        pub fn into_points(@iter self) -> ClockTablePoint;
    }
}

nvwrap! {
    pub enum ClockRange {
        V1(ClockRangeV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAIN_INFO> {
                pub domain: NvValue<ClockDomain> {
                    @sys(clockType),
                },
                pub range: Range<KilohertzDelta> {
                    @get fn(&self) {
                        Range {
                            max: KilohertzDelta(self.sys().rangeMax),
                            min: KilohertzDelta(self.sys().rangeMin),
                        }
                    },
                },
                pub index_range: Range<usize> {
                    @get fn(&self) {
                        Range {
                            min: self.sys().vfpIndexMin as usize,
                            max: self.sys().vfpIndexMax as usize,
                        }
                    },
                },
            },
        }),
    }

    impl @TaggedData for ClockRange { }
    impl @TaggedFrom(NvValue<ClockDomain>) for ClockRange { }
}

nvwrap! {
    pub enum VfpDomains {
        V1(VfpDomainsV1 {
            @type = NvData<clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO_V1> {
                pub domains: @iter(Tagged<NvValue<ClockDomain>, ClockRangeV1>) {
                    @into fn into_domains(self) {
                        self.into_sys().clocks().map(Tagged::from)
                    },
                },
            },
        }),
    }

    impl @StructVersion for VfpDomains { }
    impl @IntoIterator(into_domains() -> Tagged<NvValue<ClockDomain>, ClockRange>) for VfpDomains { }

    impl VfpDomains {
        pub fn into_domains(@iter self) -> Tagged<NvValue<ClockDomain>, ClockRange>;
    }
        /*let domains = info.mask.index(&info.entries[..])
            .map(|(_i, v)| v)
            .filter(|v| v.disabled == 0)
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;*/
}

nvwrap! {
    pub enum VfPoint {
        V1(VfPointV1 {
            @type = NvData<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT> {
                pub frequency: Kilohertz {
                    @get fn(&self) {
                        Kilohertz(self.sys().freq_kHz)
                    },
                },
                pub voltage: Microvolts {
                    @get fn(&self) {
                        Microvolts(self.sys().voltage_uV)
                    },
                },
            },
        }),
    }

    impl VfPoint {
        pub fn frequency(&self) -> Kilohertz;
        pub fn voltage(&self) -> Microvolts;
    }
}

nvwrap! {
    pub enum VfpEntry {
        V3(VfpEntryV3 {
            @type = NvData<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V3> {
                pub unknown: u32 {
                    @get fn(&self) {
                        self.sys().clock_type
                    },
                },
                pub point: VfPointV1 {
                    @get fn(&self) {
                        self.sys().point.into()
                    },
                },
                pub default_point: Option<VfPointV1> {
                    @get fn(&self) {
                        self.sys().point_default.to_option().map(Into::into)
                    },
                },
                pub overclocked_point: Option<VfPointV1> {
                    @get fn(&self) {
                        self.sys().point_overclocked.to_option().map(Into::into)
                    },
                },
            },
        }),
        V1(VfpEntryV1 {
            @type = NvData<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINT_STATUS_V1> {
                pub unknown: u32 {
                    @get fn(&self) {
                        self.sys().clock_type
                    },
                },
                pub point: VfPointV1 {
                    @get fn(&self) {
                        self.sys().point.into()
                    },
                },
            },
        }),
    }

    impl VfpEntry {
        // 1 for idle values / low pstates? only populated for memory clocks
        pub fn unknown(&self) -> u32;
        pub fn point(&self) -> VfPoint;
    }
}

impl VfpEntry {
    pub fn default_point(&self) -> Option<VfPoint> {
        match self {
            Self::V3(entry) => entry.default_point().map(Into::into),
            Self::V1(..) => None,
        }
    }

    pub fn overclocked_point(&self) -> Option<VfPoint> {
        match self {
            Self::V3(entry) => entry.overclocked_point().map(Into::into),
            Self::V1(..) => None,
        }
    }

    pub fn configured_point(&self) -> VfPoint {
        self.overclocked_point().unwrap_or(self.point())
    }
}

nvwrap! {
    pub enum VfpCurve {
        V3(VfpCurveV3 {
            @type = NvData<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V3> {
                pub mask: ClockMask {
                    @get fn(&self) {
                        self.sys().mask
                    },
                },
                pub points: @iter(VfpEntryV3) {
                    @into fn into_points(self) {
                        self.into_sys().into_points().map(Into::into)
                    },
                },
            },
        }),
        V1(VfpCurveV1 {
            @type = NvData<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS_V1> {
                pub mask: ClockMask {
                    @get fn(&self) {
                        self.sys().mask
                    },
                },
                pub points: @iter(VfpEntryV1) {
                    @into fn into_points(self) {
                        self.into_sys().into_points().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for VfpCurve { }
    impl @IntoIterator(into_points() -> VfpEntry) for VfpCurve { }

    impl VfpCurve {
        pub fn mask(&self) -> ClockMask;
        pub fn into_points(@iter self) -> VfpEntry;
    }
}

nvwrap! {
    pub enum VoltageRails {
        V1(VoltageRailsV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS_V1> {
                pub voltage: Microvolts {
                    @sys(value_uV),
                },
            },
        }),
    }

    impl VoltageRails {
        pub fn voltage(&self) -> Microvolts;
    }
}

nvwrap! {
    pub enum VoltageSettings {
        V1(VoltageSettingsV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL_V1> {
                pub core_voltage_boost@mut(core_voltage_boost_mut)@set(set_core_voltage_boost): Percentage {
                    @sys(percent),
                },
            },
        }),
    }

    impl VoltageSettings {
        pub fn core_voltage_boost(&self) -> Percentage;
    }
}

nvwrap! {
    pub enum PowerInfoEntry {
        V2(PowerInfoEntryV2 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICY_INFO_V2> {
                pub policy: NvValue<PowerPolicyId> {
                    @sys(policy_id),
                },
                pub range: Range<Percentage1000> {
                    @get fn(&self) {
                        Range {
                            min: Percentage1000(self.sys().min_power),
                            max: Percentage1000(self.sys().max_power),
                        }
                    },
                },
                pub default_limit: Percentage1000 {
                    @sys(def_power),
                },
            },
        }),
        V1(PowerInfoEntryV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICY_INFO_V1> {
                pub policy: NvValue<PowerPolicyId> {
                    @sys(policy_id),
                },
                pub range: Range<Percentage1000> {
                    @get fn(&self) {
                        Range {
                            min: Percentage1000(self.sys().min_power),
                            max: Percentage1000(self.sys().max_power),
                        }
                    },
                },
                pub default_limit: Percentage1000 {
                    @sys(def_power),
                },
            },
        }),
    }

    impl @TaggedData for PowerInfoEntry { }

    impl PowerInfoEntry {
        pub fn policy(&self) -> NvValue<PowerPolicyId>;
        pub fn range(&self) -> Range<Percentage1000>;
        pub fn default_limit(&self) -> Percentage1000;
    }
}

nvwrap! {
    pub enum PowerInfo {
        V2(PowerInfoV2 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_V2> {
                pub valid: bool {
                    @get fn(&self) {
                        self.sys().valid != 0
                    },
                },
                pub entries: @iter(PowerInfoEntryV2) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
        V1(PowerInfoV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICIES_INFO_V1> {
                pub valid: bool {
                    @get fn(&self) {
                        self.sys().valid != 0
                    },
                },
                pub entries: @iter(PowerInfoEntryV1) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for PowerInfo { }
    impl @IntoIterator(into_entries() -> PowerInfoEntry) for PowerInfo { }

    impl PowerInfo {
        pub fn into_entries(@iter self) -> PowerInfoEntry;
    }
}

nvwrap! {
    pub enum PowerStatus {
        V1(PowerStatusV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_ENTRY> {
                pub channel: NvValue<PowerTopologyChannel> {
                    @sys,
                },
                pub power: Percentage1000 {
                    @sys,
                },
            },
        }),
    }

    impl @TaggedData for PowerStatus { }

    impl PowerStatus {
        pub fn channel(&self) -> NvValue<PowerTopologyChannel>;
        pub fn power(&self) -> Percentage1000;
    }
}

nvwrap! {
    pub enum PowerTopology {
        V1(PowerTopologyV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_STATUS_V1> {
                pub entries: @iter(PowerStatusV1) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for PowerTopology { }
    impl @IntoIterator(into_entries() -> PowerStatus) for PowerTopology { }

    impl PowerTopology {
        pub fn into_entries(@iter self) -> PowerStatus;
    }
}

nvwrap! {
    pub enum PowerChannels {
        V1(PowerChannelsV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_TOPOLOGY_INFO_V1> {
                pub channels: @iter(NvValue<PowerTopologyChannel>) {
                    @into fn into_channels(self) {
                        self.into_sys().get_channels().into_iter()
                    },
                },
            },
        }),
    }

    impl @StructVersion for PowerChannels { }
    impl @IntoIterator(into_channels() -> NvValue<PowerTopologyChannel>) for PowerChannels { }

    impl PowerChannels {
        pub fn into_channels(@iter self) -> NvValue<PowerTopologyChannel>;
    }
}

nvwrap! {
    pub enum PowerPolicyStatus {
        V2(PowerPolicyStatusV2 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICY_STATUS_V2> {
                pub policy: NvValue<PowerPolicyId> {
                    @sys(policy_id),
                },
                pub power_target: Percentage1000 {
                    @sys,
                },
            },
        }),
        V1(PowerPolicyStatusV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICY_STATUS_V1> {
                pub policy: NvValue<PowerPolicyId> {
                    @sys(policy_id),
                },
                pub power_target: Percentage1000 {
                    @sys,
                },
            },
        }),
    }

    impl @TaggedData for PowerPolicyStatus { }

    impl PowerPolicyStatus {
        pub fn policy(&self) -> NvValue<PowerPolicyId>;
        pub fn power_target(&self) -> Percentage1000;
    }
}

nvwrap! {
    pub enum PowerPolicies {
        V2(PowerPoliciesV2 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_V2> {
                pub entries: @iter(PowerPolicyStatusV2) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
        V1(PowerPoliciesV1 {
            @type = NvData<power::private::NV_GPU_CLIENT_POWER_POLICIES_STATUS_V1> {
                pub entries: @iter(PowerPolicyStatusV1) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for PowerPolicies { }
    impl @IntoIterator(into_entries() -> PowerPolicyStatus) for PowerPolicies { }

    impl PowerPolicies {
        pub fn into_entries(@iter self) -> PowerPolicyStatus;
    }
}

nvwrap! {
    pub enum ClockLockEntry {
        V2(ClockLockEntryV2 {
            @type = NvData<clock::private::NV_GPU_PERF_CLIENT_LIMIT> {
                pub id@mut(id_mut)@set(set_id): NvValue<PerfLimitId> {
                    @sys,
                },
                pub domain@mut(domain_mut)@set(set_domain): NvValue<ClockDomain> {
                    @sys(clock_id),
                },
                pub mode@mut(mode_mut)@set(set_mode): NvValue<ClockLockMode> {
                    @sys,
                },
                pub value@set(set_value): Option<u32> {
                    @get fn(&self) {
                        match self.mode() {
                            NvValue::<ClockLockMode>::None => None,
                            _ => Some(self.sys().value),
                        }
                    },
                    @set fn self value {
                        let sys = self.sys_mut();
                        sys.value = match value {
                            Some(value) => value,
                            None => {
                                sys.mode = ClockLockMode::None.into();
                                0
                            },
                        };
                    },
                },
                pub voltage@set(set_voltage): Option<Microvolts> {
                    @get fn(&self) {
                        match self.mode() {
                            NvValue::<ClockLockMode>::ManualVoltage => self.value().map(Microvolts),
                            _ => None,
                        }
                    },
                    @set fn(&mut self, voltage: Microvolts) {
                        let sys = self.sys_mut();
                        sys.mode = ClockLockMode::ManualVoltage.into();
                        sys.value = voltage.0;
                    },
                },
                pub frequency@set(set_frequency): Option<Kilohertz> {
                    @get fn(&self) {
                        match self.mode() {
                            NvValue::<ClockLockMode>::ManualFrequency => self.value().map(Kilohertz),
                            _ => None,
                        }
                    },
                    @set fn(&mut self, frequency: Kilohertz) {
                        let sys = self.sys_mut();
                        sys.mode = ClockLockMode::ManualFrequency.into();
                        sys.value = frequency.0;
                    },
                },
            },
        }),
    }

    impl @TaggedData for ClockLockEntry { }

    impl ClockLockEntry {
        pub fn id(&self) -> NvValue<PerfLimitId>;
        pub fn domain(&self) -> NvValue<ClockDomain>;
        pub fn mode(&self) -> NvValue<ClockLockMode>;
        pub fn value(&self) -> Option<u32>;
        pub fn voltage(&self) -> Option<Microvolts>;
        pub fn frequency(&self) -> Option<Kilohertz>;
    }
}

nvwrap! {
    pub enum ClockLimits {
        V2(ClockLimitsV2 {
            @type = NvData<clock::private::NV_GPU_PERF_CLIENT_LIMITS_V2> {
                pub entries: @iter(ClockLockEntryV2) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for ClockLimits { }
    impl @IntoIterator(into_entries() -> ClockLockEntry) for ClockLimits { }

    impl ClockLimits {
        pub fn into_entries(@iter self) -> ClockLockEntry;
    }
}

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum ClockLockValue {
    Frequency(Kilohertz),
    Voltage(Microvolts),
}*/

/*impl fmt::Display for ClockLockValue*/

nvwrap! {
    pub enum PerfInfo {
        V1(PerfInfoV1 {
            @type = NvData<power::private::NV_GPU_PERF_POLICIES_INFO_PARAMS_V1> {
                pub max_unknown: u32 {
                    @get fn(&self) {
                        self.sys().maxUnknown
                    },
                },
                pub limits: PerfFlags {
                    @get fn(&self) {
                        self.sys().limitSupport.truncate()
                    },
                },
            },
        }),
    }

    impl @StructVersion for PerfInfo { }

    impl PerfInfo {
        pub fn limits(&self) -> PerfFlags;
    }
}

//limits: PerfFlags::from_bits(info.limitSupport).ok_or(sys::ArgumentRangeError)?,

nvwrap! {
    pub enum PerfStatus {
        V1(PerfStatusV1 {
            @type = NvData<power::private::NV_GPU_PERF_POLICIES_STATUS_PARAMS_V1> {
                pub unknown: u32 {
                    @get fn(&self) {
                        self.sys().unknown
                    },
                },
                pub limits: PerfFlags {
                    @get fn(&self) {
                        self.sys().limits.truncate()
                    },
                },
            },
        }),
    }

    impl @StructVersion for PerfStatus { }

    impl PerfStatus {
        pub fn limits(&self) -> PerfFlags;
    }
}

nvwrap! {
    pub enum VoltageEntry {
        V1(VoltageEntryV1 {
            @type = NvData<power::private::NV_VOLT_TABLE_ENTRY> {
                pub domain: NvValue<VoltageDomain> {
                    @sys(voltage_domain),
                },
                pub voltage: Microvolts {
                    @sys(voltage_uV),
                },
            },
        }),
    }

    impl @TaggedData for VoltageEntry { }

    impl VoltageEntry {
        pub fn domain(&self) -> NvValue<VoltageDomain>;
        pub fn voltage(&self) -> Microvolts;
    }
}

nvwrap! {
    pub enum VoltageTable {
        V1(VoltageTableV1 {
            @type = NvData<power::private::NV_VOLT_TABLE_V1> {
                pub entries: @iter(VoltageEntryV1) {
                    @into fn into_entries(self) {
                        self.into_sys().get_entries().into_iter().map(Into::into)
                    },
                },
            },
        }),
    }

    impl @StructVersion for VoltageTable { }
    impl @IntoIterator(into_entries() -> VoltageEntry) for VoltageTable { }

    impl VoltageTable {
        pub fn into_entries(@iter self) -> VoltageEntry;
    }
}

nvwrap! {
    pub enum VoltageStatus {
        V1(VoltageStatusV1 {
            @type = NvData<power::private::NV_VOLT_STATUS_V1> {
                pub voltage: Microvolts {
                    @get fn(&self) {
                        Microvolts(self.sys().value_uV)
                    },
                },
            },
        }),
    }

    impl @StructVersion for VoltageStatus { }
}
