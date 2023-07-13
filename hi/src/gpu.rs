use std::collections::BTreeMap;
use std::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use once_cell::sync::OnceCell;
use crate::{allowable_result, allowable_result_fallback};

use nvapi::{self,
    ArchInfo,
    ClockTable, VfpCurve, VfpEntry, Sensor, ThermalInfo, PowerInfoEntry,
    ClockFrequencyType, ClockEntry, ClockEntryData,
    BaseVoltage, PStates, ClockRange/*, VfpInfo*/,
    ThermalLimit, ThermalPolicyId/*, PffStatus*/,
    Error, ArgumentRangeError,
    Api, NvapiResultExt,
};
pub use nvapi::{
    PhysicalGpu,
    Vendor, SystemType, GpuType, RamType, RamMaker, Foundry, Architecture, ChipRevision,
    EccErrors,
    ClockFrequencies, ClockDomain, VoltageDomain, UtilizationDomain, Utilizations,
    /*ClockLockValue,*/ ClockLockEntry, PerfLimitId,
    CoolerType, CoolerController, CoolerControl, CoolerPolicy, CoolerTarget,
    FanCoolerId, CoolersInfo, CoolerInfo, CoolerStatus, CoolerSettings,
    VoltageStatus, VoltageTable, PowerTopologyChannel, PowerPolicyId,
    PerfInfo, PerfStatus,
    ThermalController, ThermalTarget, PffPoint, PffCurve,
    MemoryInfo, PciIdentifiers, BusInfo, Bus, BusType, DriverModel,
    Percentage, Celsius, Rpm,
    Range,
    Kibibytes, Microvolts, MicrovoltsDelta, Kilohertz, KilohertzDelta,
    PState,
};

pub struct Gpu {
    gpu: PhysicalGpu,
    #[cfg(never)]
    vfp_info: OnceCell<VfpInfo>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Arch {
    pub architecture: Architecture,
    pub revision: ChipRevision,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", &self.architecture, &self.revision)
    }
}

impl TryFrom<ArchInfo> for Arch {
    type Error = crate::Error;

    fn try_from(info: ArchInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            architecture: info.arch(),
            revision: info.revision().try_get()
                .with_api(Api::NvAPI_GPU_GetArchInfo)?,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone/*, PartialOrd, Ord, PartialEq, Eq, Hash*/)]
pub struct GpuInfo {
    pub id: usize,
    pub name: String,
    pub codename: String,
    pub bios_version: String,
    pub driver_model: Option<DriverModel>,
    pub bus: BusInfo,
    pub memory: Option<MemoryInfo>,
    pub system_type: SystemType,
    pub gpu_type: GpuType,
    pub arch: Arch,
    pub ram_type: RamType,
    pub ram_maker: RamMaker,
    pub ram_bus_width: u32,
    pub ram_bank_count: u32,
    pub ram_partition_count: u32,
    pub foundry: Foundry,
    pub core_count: u32,
    pub shader_pipe_count: u32,
    pub shader_sub_pipe_count: u32,
    pub ecc: EccInfo,
    pub base_clocks: ClockFrequencies,
    pub boost_clocks: ClockFrequencies,
    pub sensors: Vec<SensorDesc>,
    pub coolers: CoolersInfo,
    pub perf: PerfInfo,
    pub sensor_limits: Vec<SensorLimit>,
    pub power_limits: Vec<PowerLimit>,
    #[cfg(never)]
    pub pstate_limits: BTreeMap<PState, BTreeMap<ClockDomain, PStateLimit>>,
    // TODO: pstate base_voltages
    #[cfg(never)]
    pub overvolt_limits: Vec<OvervoltLimit>,
    #[cfg(never)]
    pub vfp_limits: BTreeMap<ClockDomain, VfpRange>,
}

impl GpuInfo {
    pub fn vendor(&self) -> Option<Vendor> {
        self.bus.vendor_id().and_then(|id| id.try_get().ok())
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone/*, PartialOrd, Ord, PartialEq, Eq, Hash*/)]
pub struct EccInfo {
    pub enabled_by_default: bool,
    pub info: nvapi::EccStatus,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone/*, PartialOrd, Ord, PartialEq, Eq, Hash*/)]
pub struct EccStatus {
    pub enabled: bool,
    pub errors: EccErrors,
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfpRange {
    pub range: Range<KilohertzDelta>,
}

#[cfg(never)]
impl From<ClockRange> for VfpRange {
    fn from(c: ClockRange) -> Self {
        VfpRange {
            range: Range::range_from(c.range()),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone/*, PartialOrd, Ord, PartialEq, Eq, Hash*/)]
pub struct GpuStatus {
    pub pstate: PState,
    pub clocks: ClockFrequencies,
    pub memory: Option<MemoryInfo>,
    pub pcie_lanes: Option<u32>,
    pub ecc: EccStatus,
    pub voltage: Option<Microvolts>,
    pub voltage_domains: Option<VoltageStatus>,
    pub voltage_step: Option<VoltageStatus>,
    pub voltage_table: Option<VoltageTable>,
    pub tachometer: Option<Rpm>,
    pub utilization: Utilizations,
    pub power: BTreeMap<PowerTopologyChannel, Percentage>,
    pub sensors: Vec<(SensorDesc, Celsius)>,
    #[cfg(never)]
    pub coolers: BTreeMap<FanCoolerId, CoolerStatus>,
    pub perf: PerfStatus,
    #[cfg(never)]
    pub vfp: Option<VfpTable>,
    #[cfg(never)]
    pub vfp_locks: BTreeMap<PerfLimitId, ClockLockValue>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone/*, PartialOrd, Ord, PartialEq, Eq, Hash*/)]
pub struct GpuSettings {
    pub voltage_boost: Option<Percentage>,
    pub sensor_limits: Vec<SensorThrottle>,
    pub power_limits: Vec<Percentage>,
    #[cfg(never)]
    pub coolers: BTreeMap<FanCoolerId, CoolerSettings>,
    #[cfg(never)]
    pub vfp: Option<VfpDeltas>,
    #[cfg(never)]
    pub pstate_deltas: BTreeMap<PState, BTreeMap<ClockDomain, KilohertzDelta>>,
    #[cfg(never)]
    pub overvolt: Vec<MicrovoltsDelta>,
    #[cfg(never)]
    pub vfp_locks: BTreeMap<PerfLimitId, ClockLockEntry>,
}

impl Gpu {
    pub fn new(gpu: PhysicalGpu) -> Self {
        Gpu {
            gpu,
            #[cfg(never)]
            vfp_info: OnceCell::new(),
        }
    }

    pub fn into_inner(self) -> PhysicalGpu {
        self.gpu
    }

    pub fn inner(&self) -> &PhysicalGpu {
        &self.gpu
    }

    pub fn id(&self) -> usize {
        self.gpu.handle().as_ptr() as _
    }

    pub fn enumerate() -> crate::Result<Vec<Self>> {
        PhysicalGpu::enumerate()
            .map_err(Into::into)
            .map(|v| v.into_iter().map(Gpu::new).collect())
    }

    pub fn info(&self) -> crate::Result<GpuInfo> {
        let pstates = allowable_result(self.gpu.pstates())?;
        #[cfg(never)]
        let (pstates, ov) = match pstates {
            Some(ps) => (ps.into_pstates(), ps.overvolt()),
            None => (Default::default(), Default::default()),
        };

        Ok(GpuInfo {
            id: self.id(),
            name: self.gpu.full_name()?,
            codename: self.gpu.short_name()?,
            bios_version: self.gpu.vbios_version_string()?,
            driver_model: allowable_result(self.gpu.driver_model())?,
            bus: allowable_result_fallback(self.gpu.bus_info(), Default::default())?,
            memory: allowable_result(self.gpu.memory_info())?,
            ecc: EccInfo {
                enabled_by_default: allowable_result_fallback(
                    self.gpu.ecc_configuration().map(|config| config.enabled_by_default()),
                    false
                )?,
                info: allowable_result_fallback(self.gpu.ecc_status(), Default::default())?,
            },
            system_type: allowable_result_fallback(self.gpu.handle().GetSystemType().with_api(Api::NvAPI_GPU_GetSystemType)
                .and_then(|v|
                    v.try_get().with_api(Api::NvAPI_GPU_GetSystemType)
                ),
                SystemType::Unknown
            )?,
            gpu_type: allowable_result_fallback(self.gpu.handle().GetGPUType().with_api(Api::NvAPI_GPU_GetGPUType)
                .and_then(|v|
                    v.try_get().with_api(Api::NvAPI_GPU_GetGPUType)
                ),
                GpuType::Unknown
            )?,
            arch: allowable_result_fallback(self.gpu.architecture().map_err(Into::into).and_then(TryInto::try_into), Default::default())?,
            ram_type: allowable_result_fallback(self.gpu.handle().GetRamType().with_api(Api::NvAPI_GPU_GetRamType)
                .and_then(|v|
                    v.try_get().with_api(Api::NvAPI_GPU_GetRamType)
                ),
                RamType::Unknown
            )?,
            ram_maker: allowable_result_fallback(self.gpu.handle().GetRamMaker().with_api(Api::NvAPI_GPU_GetRamMaker)
                .and_then(|v|
                    v.try_get().with_api(Api::NvAPI_GPU_GetRamMaker)
                ),
                RamMaker::Unknown
            )?,
            ram_bus_width: allowable_result_fallback(self.gpu.handle().GetRamBusWidth().with_api(Api::NvAPI_GPU_GetRamBusWidth), 0)?,
            ram_bank_count: allowable_result_fallback(self.gpu.handle().GetRamBankCount().with_api(Api::NvAPI_GPU_GetRamBankCount), 0)?,
            ram_partition_count: allowable_result_fallback(self.gpu.handle().GetPartitionCount().with_api(Api::NvAPI_GPU_GetPartitionCount), 0)?,
            foundry: allowable_result_fallback(self.gpu.handle().GetFoundry().with_api(Api::NvAPI_GPU_GetFoundry)
                .and_then(|v|
                    v.try_get().with_api(Api::NvAPI_GPU_GetFoundry)
                ),
                Foundry::Unknown
            )?,
            core_count: self.gpu.handle().GetGpuCoreCount().with_api(Api::NvAPI_GPU_GetGpuCoreCount)?,
            shader_pipe_count: self.gpu.handle().GetShaderPipeCount().with_api(Api::NvAPI_GPU_GetShaderPipeCount)?,
            shader_sub_pipe_count: self.gpu.handle().GetShaderSubPipeCount().with_api(Api::NvAPI_GPU_GetShaderSubPipeCount)?,
            base_clocks: self.gpu.clock_frequencies(ClockFrequencyType::Base)?,
            boost_clocks: self.gpu.clock_frequencies(ClockFrequencyType::Boost)?,
            sensors: match allowable_result(self.gpu.thermal_settings(None))? {
                Some(s) => s.into_iter().map(SensorDesc::try_from).collect::<Result<_, _>>()?,
                None => Default::default(),
            },
            coolers: match allowable_result(self.gpu.cooler_info())? {
                Some(c) => c,
                None => Default::default(),
            },
            perf: self.gpu.perf_info()?,
            sensor_limits: match allowable_result(self.gpu.thermal_limit_info())? {
                Some(l) => l.into_iter().map(SensorLimit::from).collect(),
                None => Default::default(),
            },
            power_limits: match allowable_result(self.gpu.power_limit_info())? {
                Some(p) => p.into_entries().map(PowerLimit::from).collect(),
                None => Default::default(),
            },
            #[cfg(never)]
            pstate_limits: pstates.into_iter().map(|p| (p.id, p.clocks.into_iter().map(|p| (p.domain(), p.into())).collect())).collect(),
            #[cfg(never)]
            overvolt_limits: ov.into_iter().map(From::from).collect(),
            #[cfg(never)]
            vfp_limits: match allowable_result(self.gpu.vfp_ranges())? {
                Some(l) => l.into_domains().map(|v| (v.tag, v.value)).collect(),
                None => Default::default(),
            },
        })
    }

    #[cfg(never)]
    fn vfp_info(&self) -> create::Result<Option<&VfpInfo>> {
        allowable_result(self.vfp_info.get_or_try_init(|| {
            self.gpu.vfp_info()
        }))
    }

    pub fn status(&self) -> crate::Result<GpuStatus> {
        #[cfg(never)]
        let vfp_info = self.vfp_info()?;

        Ok(GpuStatus {
            pstate: self.gpu.current_pstate().and_then(|ps| ps.try_into().with_api(Api::NvAPI_GPU_GetCurrentPstate))?,
            clocks: self.gpu.clock_frequencies(ClockFrequencyType::Current)?,
            memory: allowable_result(self.gpu.memory_info())?,
            pcie_lanes: match self.gpu.handle().GetBusType().with_api(Api::NvAPI_GPU_GetBusType) {
                Ok(nvapi::NvValue::<BusType>::PciExpress) => allowable_result(self.gpu.handle().GetCurrentPCIEDownstreamWidth().with_api(Api::NvAPI_GPU_GetCurrentPCIEDownstreamWidth))?,
                _ => None,
            },
            ecc: EccStatus {
                enabled: allowable_result_fallback(
                    self.gpu.ecc_configuration().map(|config| config.enabled()),
                    false
                )?,
                errors: allowable_result_fallback(self.gpu.ecc_errors(), Default::default())?,
            },
            voltage: allowable_result(self.gpu.core_voltage().map(|v| v.voltage()))?,
            voltage_domains: allowable_result(self.gpu.voltage_domains_status())?,
            voltage_step: allowable_result(self.gpu.voltage_step())?,
            voltage_table: allowable_result(self.gpu.voltage_table())?,
            tachometer: allowable_result(self.gpu.tachometer())?,
            utilization: self.gpu.dynamic_pstates_info()?,
            power: self.gpu.power_usage(self.gpu.power_usage_channels()?)?
                .into_iter().map(|power| power.channel().try_get()
                    .with_api(Api::NvAPI_GPU_ClientPowerTopologyGetStatus)
                    .map(|ch| (ch, power.power().into()))
                ).collect::<Result<_, _>>()?,
            sensors: match allowable_result(self.gpu.thermal_settings(None))? {
                Some(s) => s.into_iter().map(|s| SensorDesc::try_from(s).map(|desc| (desc, s.current_temperature()))).collect::<Result<_, _>>()?,
                None => Default::default(),
            },
            #[cfg(never)]
            coolers: match allowable_result(self.gpu.cooler_status())? {
                Some(c) => c,
                None => Default::default(),
            },
            perf: self.gpu.perf_status()?,
            #[cfg(never)]
            vfp: match &vfp_info {
                Some(info) => allowable_result(self.gpu.vfp_curve(info))?.map(From::from),
                None => None,
            },
            #[cfg(never)]
            vfp_locks: match allowable_result(self.gpu.vfp_locks(PerfLimitId::values()))? {
                Some(l) => l.into_iter().filter_map(|lock| lock.lock_value
                    .map(|value| (lock.limit, value))
                ).collect(),
                None => Default::default(),
            },
        })
    }

    pub fn settings(&self) -> crate::Result<GpuSettings> {
        #[cfg(never)]
        let vfp_info = self.vfp_info()?;
        let pstates = allowable_result(self.gpu.pstates())?;
        #[cfg(never)]
        let (pstates, clocks, voltages) = match pstates {
            Some(ps) => (
                ps.clocks().copied().collect(),
                ps.base_voltages().copied().collect(),
                ps.into_pstates(),
            ),
            None => (Default::default(), Default::default()),
        };

        Ok(GpuSettings {
            voltage_boost: allowable_result(self.gpu.core_voltage_boost().map(|boost| boost.core_voltage_boost()))?,
            sensor_limits: match allowable_result(self.gpu.thermal_limit())? {
                Some(l) => l.into_iter().map(|l| SensorThrottle::from_limit(&l)).collect(),
                None => Default::default(),
            },
            power_limits: match allowable_result(self.gpu.power_limit())? {
                Some(l) => l.into_iter().map(|l| Percentage::from(l.power_target())).collect(),
                None => Default::default(),
            },
            #[cfg(never)]
            coolers: match allowable_result(self.gpu.cooler_control())? {
                Some(c) => c,
                None => Default::default(),
            },
            #[cfg(never)]
            vfp: match &vfp_info {
                Some(info) => allowable_result(self.gpu.vfp_table(info))?.map(From::from),
                None => None,
            },
            #[cfg(never)]
            vfp_locks: match allowable_result(self.gpu.vfp_locks(PerfLimitId::values()))? {
                Some(v) => v.into_iter().map(|lock| (lock.limit, lock)).collect(),
                None => Default::default(),
            },
            #[cfg(never)]
            pstate_deltas: pstates.into_iter().filter(|p| p.editable)
                .map(|p| (p.id, p.clocks.into_iter().filter(|p| p.editable())
                    .map(|p| (p.domain(), p.frequency_delta().value)).collect())
                ).collect(),
            #[cfg(never)]
            overvolt: ov.into_iter().filter(|v| v.editable).map(|v| v.voltage_delta.value).collect(),
        })
    }

    pub fn set_voltage_boost(&self, boost: Percentage) -> crate::Result<()> {
        self.gpu.set_core_voltage_boost(boost)
            .map_err(Into::into)
    }

    pub fn set_power_limits<I: IntoIterator<Item=(PowerPolicyId, Percentage)>>(&self, limits: I) -> crate::Result<()> {
        // TODO: match against power_limit_info, use range.min/max from there if it matches (can get fraction of a percent!)
        self.gpu.set_power_limit(limits.into_iter().map(From::from))
            .map_err(Into::into)
    }

    #[cfg(never)]
    pub fn set_sensor_limits<I: IntoIterator<Item=SensorThrottle>>(&self, limits: I) -> crate::Result<()> {
        self.gpu.thermal_limit_info()
            .map_err(Into::into)
            .and_then(|info| self.gpu.set_thermal_limit(
            limits.into_iter().zip(info.into_iter()).map(|(limit, info)| limit.to_limit(info.policy, info.pff.as_ref()))
        ).map_err(Into::into))
    }

    #[cfg(never)]
    pub fn set_cooler_levels<I: IntoIterator<Item=(FanCoolerId, CoolerSettings)>>(&self, levels: I) -> crate::Result<()> {
        self.gpu.set_cooler(levels)
            .map_err(Into::into)
    }

    #[cfg(never)]
    pub fn reset_cooler_levels(&self) -> crate::Result<()> {
        self.gpu.restore_cooler_settings(&[])
            .map_err(Into::into)
    }

    #[cfg(never)]
    pub fn set_vfp<I: Iterator<Item=(usize, KilohertzDelta)>, M: Iterator<Item=(usize, KilohertzDelta)>>(&self, clock_deltas: I, mem_deltas: M) -> crate::Result<()> {
        let info = self.vfp_info()?.ok_or(Error::from(ArgumentRangeError))?;
        self.gpu.set_vfp_table(info, clock_deltas.map(|(i, d)| (i, d.into())), mem_deltas.map(|(i, d)| (i, d.into())))
            .map_err(Into::into)
    }

    #[cfg(never)]
    pub fn set_vfp_lock_voltage(&self, voltage: Option<Microvolts>) -> crate::Result<()> {
        self.gpu.set_vfp_locks([ClockLockEntry {
            limit: PerfLimitId::Voltage,
            clock: ClockDomain::Graphics,
            lock_value: voltage.map(ClockLockValue::Voltage),
        }]).map_err(Into::into)
    }

    #[cfg(never)]
    pub fn set_vfp_lock(&self, domain: ClockDomain, frequency: Option<Kilohertz>) -> crate::Result<()> {
        let gpu = match domain {
            ClockDomain::Graphics => true,
            ClockDomain::Memory => false,
            _ => return Err(ArgumentRangeError.into()),
        };
        self.gpu.set_vfp_locks([
            ClockLockEntry {
                limit: match gpu {
                    true => PerfLimitId::Gpu,
                    false => PerfLimitId::Memory,
                },
                clock: domain,
                lock_value: frequency.map(ClockLockValue::Frequency),
            },
            ClockLockEntry {
                limit: match gpu {
                    true => PerfLimitId::GpuUnknown,
                    false => PerfLimitId::MemoryUnknown,
                },
                clock: domain,
                lock_value: frequency.map(ClockLockValue::Frequency),
            },
        ]).map_err(Into::into)
    }

    #[cfg(never)]
    pub fn reset_vfp_lock(&self) -> crate::Result<()> {
        self.gpu.set_vfp_locks(self.gpu.vfp_locks(None)?.into_iter().map(|mut lock| {
            lock.lock_value = None;
            lock
        })).map_err(Into::into)
    }

    #[cfg(never)]
    pub fn reset_vfp(&self) -> crate::Result<()> {
        use std::iter;

        let info = self.vfp_info()?.ok_or(Error::from(ArgumentRangeError))?;
        self.gpu.set_vfp_table(info, iter::empty(), iter::empty())
            .map_err(Into::into)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct OvervoltLimit {
    pub domain: VoltageDomain,
    pub voltage: Microvolts,
    pub range: Option<Range<MicrovoltsDelta>>,
}

impl TryFrom<BaseVoltage> for OvervoltLimit {
    type Error = crate::Error;

    fn try_from(v: BaseVoltage) -> Result<Self, Self::Error> {
        Ok(OvervoltLimit {
            domain: v.id().try_get()
                .with_api(Api::NvAPI_GPU_GetPstates20)?,
            voltage: v.voltage(),
            range: if v.editable() {
                Some(v.voltage_delta().range())
            } else {
                None
            },
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PStateLimit {
    pub frequency_delta: Option<Range<KilohertzDelta>>,
    pub frequency: Range<Kilohertz>,
    pub voltage: Range<Microvolts>,
    pub voltage_domain: VoltageDomain,
}

impl TryFrom<ClockEntry> for PStateLimit {
    type Error = crate::NvapiError;

    fn try_from(entry: ClockEntry) -> Result<Self, Self::Error> {
        Ok(match entry.data() {
            ClockEntryData::Range(data) => PStateLimit {
                frequency_delta: if entry.editable() { Some(entry.frequency_delta().range()) } else { None },
                frequency: data.frequency_range(),
                voltage: data.voltage_range(),
                voltage_domain: data.voltage_domain().try_get()
                    .with_api(Api::NvAPI_GPU_GetPstates20)?,
            },
            ClockEntryData::Single(data) => PStateLimit {
                frequency_delta: if entry.editable() { Some(entry.frequency_delta().range()) } else { None },
                frequency: Range::from_scalar(data.frequency()),
                voltage: Default::default(),
                voltage_domain: VoltageDomain::Undefined,
            },
            _ => return Err(ArgumentRangeError)
                .with_api(Api::NvAPI_GPU_GetPstates20),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PowerLimit {
    pub range: Range<Percentage>,
    pub default: Percentage,
}

impl From<PowerInfoEntry> for PowerLimit {
    fn from(info: PowerInfoEntry) -> Self {
        PowerLimit {
            range: Range::range_from(info.range()),
            default: info.default_limit().into(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SensorLimit {
    pub range: Range<Celsius>,
    pub default: Celsius,
    pub flags: u32,
    #[cfg(never)]
    pub throttle_curve: Option<PffCurve>,
}

impl From<ThermalInfo> for SensorLimit {
    fn from(info: ThermalInfo) -> Self {
        SensorLimit {
            range: Range::range_from(info.temperature_range()),
            default: info.default_temperature().into(),
            flags: info.default_flags(),
            #[cfg(never)]
            throttle_curve: info.pff,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SensorThrottle {
    pub value: Celsius,
    pub tdp_unlimited: bool,
    #[cfg(never)]
    pub curve: Option<PffCurve>,
}

impl SensorThrottle {
    pub fn to_limit(&self, policy: ThermalPolicyId, info: Option<&PffCurve>) -> ThermalLimit {
        let mut limit = ThermalLimit::default();
        limit.set_id(policy.into());
        limit.set_temperature(self.value.into());
        limit.set_tdp_unlimited(self.tdp_unlimited);
        #[cfg(never)] {
            self.curve.as_ref().map(|pff| PffStatus {
                values: pff.points.iter().map(|p| p.y.into()).collect(),
                curve: match info {
                    Some(curve) => curve.clone(),
                    None => pff.clone(),
                },
            })
        }
        limit
    }

    pub fn from_limit(limit: &ThermalLimit) -> Self {
        Self {
            value: limit.temperature().into(),
            tdp_unlimited: limit.tdp_unlimited(),
            #[cfg(never)]
            curve: limit.pff.as_ref().map(|pff| pff.curve()),
        }
    }

    pub fn from_default(info: SensorLimit) -> Self {
        Self {
            value: info.default,
            #[cfg(never)]
            curve: info.throttle_curve.clone(),
            tdp_unlimited: false,
        }
    }
}

impl From<Celsius> for SensorThrottle {
    fn from(value: Celsius) -> Self {
        Self {
            value,
            .. Default::default()
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct SensorDesc {
    pub controller: ThermalController,
    pub target: ThermalTarget,
    pub range: Range<Celsius>,
}

impl TryFrom<Sensor> for SensorDesc {
    type Error = crate::Error;

    fn try_from(sensor: Sensor) -> Result<Self, Self::Error> {
        Ok(SensorDesc {
            controller: sensor.controller().try_get()
                .with_api(Api::NvAPI_GPU_GetArchInfo)?,
            target: sensor.target().try_get()
                .with_api(Api::NvAPI_GPU_GetArchInfo)?,
            range: sensor.default_temperature_range(),
        })
    }
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfpPoint {
    pub default_frequency: Kilohertz,
    pub frequency: Kilohertz,
    pub voltage: Microvolts,
}

#[cfg(never)]
impl<T: Default + PartialEq + Copy> From<VfpEntry<T>> for VfpPoint where Kilohertz: From<T> {
    fn from(v: VfpEntry<T>) -> Self {
        debug_assert!(v.configured().voltage == v.current.voltage);
        if !v.overclocked.is_empty() {
            debug_assert!(v.overclocked.voltage == v.current.voltage);
        }
        debug_assert!(v.current.frequency == v.overclocked.frequency);
        VfpPoint {
            default_frequency: v.default.frequency.into(),
            frequency: v.configured().frequency.into(),
            voltage: v.configured().voltage,
        }
    }
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfpTable {
    pub graphics: BTreeMap<usize, VfpPoint>,
    pub memory: BTreeMap<usize, VfpPoint>,
}

#[cfg(never)]
impl From<VfpCurve> for VfpTable {
    fn from(v: VfpCurve) -> Self {
        VfpTable {
            graphics: v.points.get(&ClockDomain::Graphics).map(|d| d
                .iter()
                .map(|&(i, e)| (i, e.into())).collect()
            ).unwrap_or_default(),
            memory: v.points.get(&ClockDomain::Memory).map(|d| d
                .iter()
                .map(|&(i, e)| (i, e.into())).collect()
            ).unwrap_or_default(),
        }
    }
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfpDeltas {
    pub graphics: BTreeMap<usize, KilohertzDelta>,
    pub memory: BTreeMap<usize, KilohertzDelta>,
}

#[cfg(never)]
impl From<ClockTable> for VfpDeltas {
    fn from(c: ClockTable) -> Self {
        VfpDeltas {
            graphics: c.delta_points.get(&ClockDomain::Graphics).map(|d| d
                .iter()
                .map(|&(i, d)| (i, d.into())).collect()
            ).unwrap_or_default(),
            memory: c.delta_points.get(&ClockDomain::Memory).map(|d| d
                .iter()
                .map(|&(i, d)| (i, d.into())).collect()
            ).unwrap_or_default(),
        }
    }
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfPoint {
    pub voltage: Microvolts,
    pub frequency: Kilohertz,
    pub delta: KilohertzDelta,
    pub default_frequency: Kilohertz,
}

#[cfg(never)]
impl VfPoint {
    pub fn new(point: VfpPoint, delta: KilohertzDelta) -> Self {
        VfPoint {
            voltage: point.voltage,
            frequency: point.frequency,
            default_frequency: point.default_frequency,
            delta,
        }
    }
}
