use std::{ptr, fmt};
use std::collections::BTreeMap;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use sys::version::VersionedStructField;
use sys::nvid as nvapi;
use sys::NvValue;
use crate::sys::gpu::{pstate, clock, power, cooler, thermal, display, ecc};
use crate::sys::{self, driverapi, i2c, gpu, ArgumentRangeError};
use crate::sys::handles::NvPhysicalGpuHandle;
use crate::types::{Kibibytes, KilohertzDelta, Kilohertz2Delta, Rpm, Percentage, Percentage1000, RawConversion, Map, List, NvData, TaggedIterator, Tagged};
use crate::clock::{ClockDomain, VfpMask};
use crate::pstate::PState;
use crate::error::NvapiResultExt;
use crate::{Status, NvapiError, NvapiResult, Api};

#[derive(Debug)]
pub struct PhysicalGpu(NvPhysicalGpuHandle);

unsafe impl Send for PhysicalGpu { }

pub use sys::gpu::{SystemType, GpuType, BusType, PerformanceDecreaseReason, WorkstationFeatureMask, ArchitectureId, ChipRevision};
pub use sys::gpu::private::{RamType, RamMaker, Foundry, VendorId as Vendor};
pub use sys::gpu::clock::ClockFrequencyType;
pub use sys::gpu::display::{ConnectedIdsFlags, DisplayIdsFlags as DisplayIdFlags, MonitorConnectorType};

impl PhysicalGpu {
    pub const unsafe fn with_handle(handle: NvPhysicalGpuHandle) -> Self {
        Self(handle)
    }

    pub fn handle(&self) -> &NvPhysicalGpuHandle {
        &self.0
    }

    pub fn enumerate() -> NvapiResult<Vec<Self>> {
        sys::handles::NvPhysicalGpuHandle::EnumPhysicalGPUs()
            .with_api(Api::NvAPI_EnumPhysicalGPUs)
            .map(|gpus| gpus.into_iter().map(|g| unsafe { Self::with_handle(g) }).collect())
    }

    pub fn tachometer(&self) -> NvapiResult<Rpm> {
        self.handle().GetTachReading().map(Rpm)
            .with_api(Api::NvAPI_GPU_GetTachReading)
    }

    pub fn short_name(&self) -> NvapiResult<String> {
        self.handle().GetShortName().map(Into::into)
            .with_api(Api::NvAPI_GPU_GetShortName)
    }

    pub fn full_name(&self) -> NvapiResult<String> {
        self.handle().GetFullName().map(Into::into)
            .with_api(Api::NvAPI_GPU_GetFullName)
    }

    pub fn vbios_version(&self) -> NvapiResult<(u32, u32)> {
        Ok((
            self.handle().GetVbiosRevision()
                .with_api(Api::NvAPI_GPU_GetVbiosRevision)?,
            self.handle().GetVbiosOEMRevision()
                .with_api(Api::NvAPI_GPU_GetVbiosOEMRevision)?,
        ))
    }

    pub fn vbios_version_string(&self) -> NvapiResult<String> {
        self.handle().GetVbiosVersionString().map(Into::into)
            .with_api(Api::NvAPI_GPU_GetVbiosVersionString)
    }

    pub fn driver_model(&self) -> NvapiResult<DriverModel> {
        self.handle().GetDriverModel().map(DriverModel::new)
            .with_api(Api::NvAPI_GetDriverModel)
    }

    pub fn gpu_id(&self) -> NvapiResult<u32> {
        self.handle().GetGPUID()
            .with_api(Api::NvAPI_GetGPUIDfromPhysicalGPU)
    }

    pub fn pci_identifiers(&self) -> NvapiResult<PciIdentifiers> {
        self.handle().GetPCIIdentifiers()
            .with_api(Api::NvAPI_GPU_GetPCIIdentifiers)
            .map(|(device_id, subsystem_id, revision_id, ext_device_id)| PciIdentifiers {
                device_id, subsystem_id, revision_id, ext_device_id
            })
    }

    pub fn bus_info(&self) -> NvapiResult<BusInfo> {
        let bus_type = self.handle().GetBusType()
            .with_api(Api::NvAPI_GPU_GetBusType)?;
        Ok(BusInfo {
            irq: self.handle().GetIRQ()
                .with_api(Api::NvAPI_GPU_GetIRQ)?,
            id: self.handle().GetBusId()
                .with_api(Api::NvAPI_GPU_GetBusId)?,
            slot_id: self.handle().GetBusSlotId()
                .with_api(Api::NvAPI_GPU_GetBusSlotId)?,
            bus: match bus_type {
                NvValue::<BusType>::Pci => Bus::Pci {
                    ids: self.pci_identifiers()?,
                },
                NvValue::<BusType>::PciExpress => Bus::PciExpress {
                    ids: self.pci_identifiers()?,
                    lanes: self.handle().GetCurrentPCIEDownstreamWidth()
                        .with_api(Api::NvAPI_GPU_GetCurrentPCIEDownstreamWidth)?,
                },
                ty => Bus::Other(ty),
            },
        })
    }

    pub fn board_number(&self) -> NvapiResult<[u8; 0x10]> {
        self.handle().GetBoardInfo::<1, _>().map(Into::into)
            .with_api(Api::NvAPI_GPU_GetBoardInfo)
    }

    pub fn memory_info(&self) -> NvapiResult<MemoryInfo> {
        let res = self.handle().GetMemoryInfo::<3, _>()
            .with_api(Api::NvAPI_GPU_GetMemoryInfo)
            .map(MemoryInfoV3::from)
            .map(Into::into);
        allow_version_compat!(try res);

        let res = self.handle().GetMemoryInfo::<2, _>()
            .with_api(Api::NvAPI_GPU_GetMemoryInfo)
            .map(MemoryInfoV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().GetMemoryInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_GetMemoryInfo)
            .map(MemoryInfoV1::from)
            .map(Into::into)
    }

    pub fn architecture(&self) -> NvapiResult<ArchInfo> {
        let res = self.handle().GetArchInfo::<2, _>()
            .with_api(Api::NvAPI_GPU_GetArchInfo)
            .map(ArchInfoV1::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().GetArchInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_GetArchInfo)
            .map(ArchInfoV1::from)
            .map(Into::into)
    }

    pub fn ecc_status(&self) -> NvapiResult<crate::ecc::EccStatus> {
        self.handle().GetECCStatusInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_GetECCStatusInfo)
            .map(crate::ecc::EccStatusV1::from)
            .map(Into::into)
    }

    pub fn ecc_errors(&self) -> NvapiResult<crate::ecc::EccErrors> {
        self.handle().GetECCErrorInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_GetECCErrorInfo)
            .map(crate::ecc::EccErrorsV1::from)
            .map(Into::into)
    }

    pub fn ecc_reset(&self, current: bool, aggregate: bool) -> NvapiResult<()> {
        self.handle().ResetECCErrorInfo(current.into(), aggregate.into())
            .with_api(Api::NvAPI_GPU_ResetECCErrorInfo)
    }

    pub fn ecc_configuration(&self) -> NvapiResult<crate::ecc::EccConfigurationInfo> {
        self.handle().GetECCConfigurationInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_GetECCConfigurationInfo)
            .map(crate::ecc::EccConfigurationInfoV1::from)
            .map(Into::into)
    }

    pub fn ecc_configure(&self, enable: bool, immediately: bool) -> NvapiResult<()> {
        self.handle().SetECCConfiguration(enable.into(), immediately.into())
            .with_api(Api::NvAPI_GPU_SetECCConfiguration)
    }

    pub fn clock_frequencies(&self, clock_type: ClockFrequencyType) -> NvapiResult<crate::clock::ClockFrequencies> {
        let mut data = crate::clock::ClockFrequenciesV1::new_versioned::<3>();
        data.set_clock_type(clock_type.into());
        let res = self.handle().GetAllClockFrequencies::<3, _>(data.sys_mut())
            .with_api(Api::NvAPI_GPU_GetAllClockFrequencies)
            .map(|()| data.into());
        allow_version_compat!(try res);

        let mut data = crate::clock::ClockFrequenciesV1::new_versioned::<2>();
        data.set_clock_type(clock_type.into());
        let res = self.handle().GetAllClockFrequencies::<2, _>(data.sys_mut())
            .with_api(Api::NvAPI_GPU_GetAllClockFrequencies)
            .map(|()| data.into());
        allow_version_compat!(try res);

        let mut data = crate::clock::ClockFrequenciesV1::new_versioned::<1>();
        data.set_clock_type(clock_type.into());
        self.handle().GetAllClockFrequencies::<1, _>(data.sys_mut())
            .with_api(Api::NvAPI_GPU_GetAllClockFrequencies)
            .map(|()| data.into())
    }

    pub fn current_pstate(&self) -> NvapiResult<NvValue<PState>> {
        self.handle().GetCurrentPstate()
            .with_api(Api::NvAPI_GPU_GetCurrentPstate)
    }

    pub fn pstates(&self) -> NvapiResult<crate::pstate::PStates> {
        let res = self.handle().GetPstates20::<3, _>()
            .with_api(Api::NvAPI_GPU_GetPstates20)
            .map(crate::pstate::PStatesV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        let res = self.handle().GetPstates20::<2, _>()
            .with_api(Api::NvAPI_GPU_GetPstates20)
            .map(crate::pstate::PStatesV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().GetPstates20::<1, _>()
            .with_api(Api::NvAPI_GPU_GetPstates20)
            .map(crate::pstate::PStatesV1::from)
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn set_pstates<I: IntoIterator<Item=(PState, ClockDomain, KilohertzDelta)>>(&self, deltas: I) -> Result<()> {
        let mut info: crate::pstate::PStatesV2 = StructVersion::<3>::versioned();
        let mut map: BTreeMap<PState, (usize, usize)> = Default::default();
        for (pstate, clock, delta) in deltas {
            trace!("gpu.set_pstate({:?}, {:?}, {:?})", pstate, clock, delta);
            let pstates = map.len();
            let map = map.entry(pstate).or_insert((pstates, 0));
            let entry = &mut info.pstates[map.0];
            entry.pstateId = pstate.value();
            let entry = &mut entry.clocks[map.1];
            entry.domainId = clock.value();
            entry.freqDelta_kHz.value = delta.0;
            map.1 += 1;
        }
        info.numPstates = map.len() as _;
        info.numClocks = map.iter().map(|v| (v.1).1).max().unwrap_or(0) as _;

        match unsafe { self.handle().set_pstates20_v3_(&info) } {
            Err(NvapiError { status: sys::Status::IncompatibleStructVersion, .. }) => (),
            res => return res.map(Into::into),
        }

        StructVersion::<2>::init_version(&mut info);
        match unsafe { self.handle().set_pstates20_v2_(&info) } {
            Err(NvapiError { status: sys::Status::IncompatibleStructVersion, .. }) => (),
            res => return res.map(Into::into),
        }

        let mut info: crate::pstate::PStatesV1 = StructVersion::<1>::versioned();
        let mut map: BTreeMap<PState, (usize, usize)> = Default::default();
        for (pstate, clock, delta) in deltas {
            trace!("gpu.set_pstate({:?}, {:?}, {:?})", pstate, clock, delta);
            let pstates = map.len();
            let map = map.entry(pstate).or_insert((pstates, 0));
            let entry = &mut info.pstates[map.0];
            entry.pstateId = pstate.raw();
            let entry = &mut entry.clocks[map.1];
            entry.domainId = clock.raw();
            entry.freqDelta_kHz.value = delta.0;
            map.1 += 1;
        }
        info.numPstates = map.len() as _;
        info.numClocks = map.iter().map(|v| (v.1).1).max().unwrap_or(0) as _;
        unsafe {
            self.handle().set_pstates20_v1_(&info)
        }
    }

    pub fn dynamic_pstates_info(&self) -> NvapiResult<crate::pstate::Utilizations> {
        self.handle().GetDynamicPstatesInfoEx::<1, _>()
            .with_api(Api::NvAPI_GPU_GetDynamicPstatesInfoEx)
            .map(crate::pstate::UtilizationsV1::from)
            .map(Into::into)
    }

    /// Private and deprecated, use `dynamic_pstates_info()` instead.
    pub fn usages(&self) -> NvapiResult<crate::clock::Usages> {
        self.handle().GetUsages::<1, _>()
            .with_api(Api::NvAPI_GPU_GetUsages)
            .map(crate::clock::UsagesV1::from)
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn vfp_mask(&self) -> Result<crate::clock::VfpMask> {
        self.handle().ClockClientClkVfPointsGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClockClientClkVfPointsGetInfo)
    }

    #[cfg(never)]
    pub fn vfp_info(&self) -> crate::Result<VfpInfo> {
        Ok(VfpInfo {
            domains: self.vfp_ranges()?,
            mask: self.vfp_mask()?,
        })
    }

    #[cfg(never)]
    pub fn vfp_table(&self, mask: &crate::clock::ClockMask) -> Result<crate::clock::ClockTable> {
        let res = self.handle().ClockClientClkVfPointsGetControl::<2, _>(*mask)
            .with_api()
            .map(Into::into);
        allow_version_compat!(try res);
        self.handle().ClockClientClkVfPointsGetControl::<1_, _>(*mask).map(Into::into)
            .with_api()
    }

    #[cfg(never)]
    pub fn set_vfp_table<I: Iterator<Item=(usize, Kilohertz2Delta)>, M: Iterator<Item=(usize, Kilohertz2Delta)>>(&self, info: &VfpInfo, clocks: I, memory: M) -> crate::Result<()> {
        trace!("gpu.set_vfp_table({:?})", info);
        let mut data = self.vfp_table(info)?;
        data.mask = info.mask.mask;
        for (i, delta) in clocks {
            trace!("gpu.set_vfp_table({:?}, {:?})", i, delta);
            data.points[i].freqDeltaKHz = delta.0 / 2;
            data.mask.set_bit(i);
        }
        /*for (i, delta) in memory {
            data.memFilled[i] = 1;
            data.memDeltas[i] = delta.0;
        }*/

        match unsafe { self.handle().clock_client_clk_vf_points_set_control_v1_(&data).with_api() } {
            Err(NvapiError { status: sys::Status::IncompatibleStructVersion, .. }) if data.nvapi_version().version() > 1 => (),
            res => return res.map(Into::into),
        }

        let mut v1 = VfpData::new_versioned::<1>();
        v1.set_from(&data);
        unsafe {
            self.handle().clock_client_clk_vf_points_set_control_v1_(&v1).with_api()
        }
    }

    pub fn vfp_ranges(&self) -> NvapiResult<crate::clock::VfpDomains> {
        self.handle().ClockClientClkDomainsGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClockClientClkDomainsGetInfo)
            .map(crate::clock::VfpDomainsV1::from)
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn vfp_locks<I: IntoIterator<Item=crate::clock::PerfLimitId>>(&self, limits: I) -> Result<crate::clock::ClockLimits> {
        self.handle().PerfClientLimitsGetStatus::<2, _>(limits.into_iter().map(Into::into))
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn set_vfp_locks<I: IntoIterator<Item=crate::clock::ClockLockEntry>>(&self, values: I) -> Result<()> {
        trace!("gpu.set_vfp_locks()");
        use clock::private::ClockLockMode;

        let mut data = clock::private::NV_GPU_PERF_CLIENT_LIMITS::default();
        for (lock, entry) in values.into_iter().zip(&mut data.entries) {
            trace!("gpu.set_vfp_lock({:?})", lock);
            data.count += 1;
            entry.id = lock.limit.into();
            let (mode, value) = match lock.lock_value {
                Some(crate::clock::ClockLockValue::Frequency(v)) =>
                    (ClockLockMode::ManualFrequency.value(), v.0),
                Some(crate::clock::ClockLockValue::Voltage(v)) =>
                    (ClockLockMode::ManualVoltage.value(), v.0),
                None => (ClockLockMode::None.value(), 0),
            };
            entry.mode = mode;
            entry.value = value;
            entry.clock_id = lock.clock.into();
        }

        unsafe {
            nvcall!(NvAPI_GPU_PerfClientLimitsSetStatus(self.0, &data))
        }
    }

    #[cfg(never)]
    pub fn vfp_curve(&self, mask: &crate::clock::ClockMask) -> Result<crate::clock::VfpCurve> {
        let mask = *mask;

        let res = self.handle().ClockClientClkVfPointsGetStatus::<3, _>(mask)
            .map(Into::into);
        allow_version_compat!(try res);

        let res = self.handle().ClockClientClkVfPointsGetStatus::<2, _>(mask)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().ClockClientClkVfPointsGetStatus::<1, _>(mask)
            .map(Into::into);
    }

    pub fn core_voltage(&self) -> NvapiResult<crate::clock::VoltageRails> {
        self.handle().ClientVoltRailsGetStatus::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientVoltRailsGetStatus)
            .map(crate::clock::VoltageRailsV1::from)
            .map(Into::into)
    }

    pub fn core_voltage_boost(&self) -> NvapiResult<crate::clock::VoltageSettings> {
        self.handle().ClientVoltRailsGetControl::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientVoltRailsGetControl)
            .map(crate::clock::VoltageSettingsV1::from)
            .map(Into::into)
    }

    pub fn set_core_voltage_boost(&self, value: Percentage) -> NvapiResult<()> {
        let mut data = crate::clock::VoltageSettingsV1::new_versioned::<1>();
        data.set_core_voltage_boost(value);

        self.handle().ClientVoltRailsSetControl(data.sys())
            .with_api(Api::NvAPI_GPU_ClientVoltRailsSetControl)
    }

    pub fn power_usage<C: IntoIterator<Item=NvValue<crate::clock::PowerTopologyChannel>>>(&self, channels: C) -> NvapiResult<crate::clock::PowerTopology> {
        let mut status = crate::clock::PowerTopologyV1::new_versioned::<1>();
        status.sys_mut().set_entries(channels.into_iter().map(Into::into));
        self.handle().ClientPowerTopologyGetStatus::<1, _>(status.sys_mut())
            .with_api(Api::NvAPI_GPU_ClientPowerTopologyGetStatus)
            .map(|()| status.into())
    }

    pub fn power_usage_channels(&self) -> NvapiResult<crate::clock::PowerChannels> {
        self.handle().ClientPowerTopologyGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientPowerTopologyGetInfo)
            .map(crate::clock::PowerChannelsV1::from)
            .map(Into::into)
    }

    pub fn power_limit_info(&self) -> NvapiResult<crate::clock::PowerInfo> {
        let res = self.handle().ClientPowerPoliciesGetInfo::<2, _>()
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesGetInfo)
            .map(crate::clock::PowerInfoV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().ClientPowerPoliciesGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesGetInfo)
            .map(crate::clock::PowerInfoV1::from)
            .map(Into::into)
    }

    pub fn power_limit(&self) -> NvapiResult<crate::clock::PowerPolicies> {
        let res = self.handle().ClientPowerPoliciesGetStatus::<2, _>()
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesGetStatus)
            .map(crate::clock::PowerPoliciesV2::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().ClientPowerPoliciesGetStatus::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesGetStatus)
            .map(crate::clock::PowerPoliciesV1::from)
            .map(Into::into)
    }

    pub fn set_power_limit<I: IntoIterator<Item=Tagged<NvValue<crate::clock::PowerPolicyId>, Percentage1000>>>(&self, values: I) -> NvapiResult<()> {
        let mut data = crate::clock::PowerPoliciesV2::new_versioned::<2>();
        //data.sys_mut().valid = 1;
        data.sys_mut().set_entries(values.into_iter().map(Tagged::into_tuple).map(|(id, power)| (id, power.value()).into()));
        let res = self.handle().ClientPowerPoliciesSetStatus::<2, _>(data.sys_mut())
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesSetStatus);
        allow_version_compat!(try res);

        let mut v1 = crate::clock::PowerPoliciesV1::new_versioned::<1>();
        v1.sys_mut().set_entries(data.into_sys().get_entries().into_iter().map(Into::into));

        self.handle().ClientPowerPoliciesSetStatus::<1, _>(v1.sys_mut())
            .with_api(Api::NvAPI_GPU_ClientPowerPoliciesSetStatus)
    }

    pub fn thermal_settings(&self, index: Option<sys::gpu::thermal::ThermalTarget>) -> NvapiResult<crate::thermal::Sensors> {
        let index = index
            .map(Into::into)
            .unwrap_or(thermal::NVAPI_THERMAL_TARGET_ALL);

        let res = self.handle().GetThermalSettings::<2, _>(index.repr() as _)
            .with_api(Api::NvAPI_GPU_GetThermalSettings)
            .map(crate::thermal::SensorsV1::from)
            .map(Into::into);
        allow_version_compat!(try res);
        self.handle().GetThermalSettings::<1, _>(index.repr() as _)
            .with_api(Api::NvAPI_GPU_GetThermalSettings)
            .map(crate::thermal::SensorsV1::from)
            .map(Into::into)
    }

    pub fn thermal_limit_info(&self) -> NvapiResult<crate::thermal::ThermalPolicies> {
        let res = self.handle().ClientThermalPoliciesGetInfo::<3, _>()
            .with_api(Api::NvAPI_GPU_ClientThermalPoliciesGetInfo)
            .map(crate::thermal::ThermalPoliciesV3::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().ClientThermalPoliciesGetInfo::<2, _>()
            .with_api(Api::NvAPI_GPU_ClientThermalPoliciesGetInfo)
            .map(crate::thermal::ThermalPoliciesV2::from)
            .map(Into::into)
    }

    pub fn thermal_limit(&self) -> NvapiResult<crate::thermal::ThermalLimits> {
        let res = self.handle().ClientThermalPoliciesGetStatus::<3, _>()
            .with_api(Api::NvAPI_GPU_ClientThermalPoliciesGetStatus)
            .map(crate::thermal::ThermalLimitsV3::from)
            .map(Into::into);
        allow_version_compat!(try res);

        self.handle().ClientThermalPoliciesGetStatus::<2, _>()
            .with_api(Api::NvAPI_GPU_ClientThermalPoliciesGetStatus)
            .map(crate::thermal::ThermalLimitsV2::from)
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn set_thermal_limit<I: IntoIterator<Item=crate::thermal::ThermalLimit>>(&self, value: I) -> Result<()> {
        trace!("gpu.set_thermal_limit()");
        let mut data = thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS::default();
        data.set_policies(value.into_iter().map(|v| v.to_raw()));

        unsafe {
            nvcall!(NvAPI_GPU_ClientThermalPoliciesSetStatus(self.0, &data))
            .with_api(Api::NvAPI_GPU_ClientThermalPoliciesSetStatus)
        }
    }

    pub fn cooler_info(&self) -> NvapiResult<crate::thermal::CoolersInfo> {
        self.handle().ClientFanCoolersGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientFanCoolersGetInfo)
            .map(crate::thermal::CoolersInfoV1::from)
            .map(Into::into)
    }

    #[cfg(never)]
    pub fn cooler_status(&self) -> Result<crate::thermal::CoolersStatus> {
        trace!("gpu.cooler_status()");

        let res = unsafe {
            nvcall!(NvAPI_GPU_ClientFanCoolersGetStatus@get(self.0) => err => into)
            .with_api(Api::NvAPI_GPU_ClientFanCoolersGetStatus)
        };

        match res {
            Err(e) => e.allow_version_incompat()?,
            res => return res,
        }

        self.cooler_settings_().map(|c|
            c.values.into_iter().map(Into::into).collect()
        )
    }

    #[cfg(never)]
    pub fn cooler_control(&self) -> Result<crate::thermal::CoolersSettings> {
        trace!("gpu.cooler_status()");

        let res = unsafe {
            nvcall!(NvAPI_GPU_ClientFanCoolersGetControl@get(self.0) => err => into)
            .with_api(Api::NvAPI_GPU_ClientFanCoolersGetControl)
        };

        match res {
            Err(e) => e.allow_version_incompat()?,
            res => return res,
        }

        self.cooler_settings_().map(|c|
            c.values.into_iter().map(Into::into).collect()
        )
    }

    #[cfg(never)]
    pub fn getcooler_settings(&self, index: Option<u32>) -> Result<crate::thermal::GetCoolersSettings> {
        trace!("gpu.getcooler_settings({:?})", index);

        let index = match index {
            Some(index) => index,
            None if <cooler::private::NV_GPU_GETCOOLER_SETTINGS as sys::version::StructVersion>::NVAPI_VERSION.version() < 4 =>
                // TODO: fall back to older versions if fail?
                cooler::private::NVAPI_COOLER_TARGET_ALL as _,
            None => 0,
        };
        unsafe {
            nvcall!(NvAPI_GPU_GetCoolerSettings@get(self.0, index) => err => into)
            .with_api(Api::NvAPI_GPU_GetCoolerSettings)
        }
    }

    #[cfg(never)]
    fn cooler_settings_(&self) -> Result<crate::thermal::CoolersSettings> {
        self.getcooler_settings(None).and_then(|c| c.values.into_iter()/*.enumerate()
            .map(|(i, value)| (i as i32 + 1).try_into().map_err(Into::into)
                .map(|id| Tagged { id, value })
            )*/
            .map(Into::into)
            .collect()
        )
    }

    #[cfg(never)]
    pub fn cooler_settings(&self) -> Result<crate::thermal::CoolersSettings> {
        match self.cooler_settings_() {
            Err(e) => e.allow_version_incompat()?,
            res => return res,
        }

        self.cooler_info()?.values.into_iter()
            .zip(self.cooler_status()?.into_iter())
            .zip(self.cooler_control()?.into_iter())
            .map(|((info, status), control)| match (info.id(), status.id(), control.id()) {
                (id, ids, idc) if id == ids && id == idc => Ok(crate::thermal::CoolersSettings::with_cooler(info, status, control)),
                _ => Err(ArgumentRangeError.with_api(Api::NvAPI_GPU_GetCoolerSettings)),
            }).collect()
    }

    #[cfg(never)]
    #[deprecated]
    pub fn set_cooler_levels<I: IntoIterator<Item=crate::thermal::CoolerSettings>>(&self, index: Option<u32>, values: I) -> Result<()> {
        trace!("gpu.set_cooler_levels({:?})", index);
        let mut data = cooler::private::NV_GPU_SETCOOLER_LEVEL::default();
        for (entry, level) in data.cooler.iter_mut().zip(values) {
            trace!("gpu.set_cooler_level({:?})", level);
            entry.currentLevel = level.level.unwrap_or_default().0;
            entry.currentPolicy = level.policy.value();
        }

        unsafe {
            nvcall!(NvAPI_GPU_SetCoolerLevels(self.0, index.unwrap_or(cooler::private::NVAPI_COOLER_TARGET_ALL.repr() as _), &data))
        }
    }

    #[cfg(never)]
    pub fn set_cooler<I: IntoIterator<Item=(crate::thermal::FanCoolerId, crate::thermal::CoolerSettings)>>(&self, values: I) -> Result<()> {
        trace!("gpu.set_cooler()");
        let mut backup = cooler::private::NV_GPU_SETCOOLER_LEVEL::default();
        let mut data = cooler::private::NV_GPU_CLIENT_FAN_COOLERS_CONTROL::default();

        for (entry, (backup_entry, (id, settings))) in data.coolers.iter_mut().zip(backup.cooler.iter_mut().zip(values)) {
            trace!("gpu.set_cooler({:?})", settings);
            *entry = settings.to_raw(id);
            data.count += 1;

            backup_entry.currentLevel = settings.level.unwrap_or_default().0;
            backup_entry.currentPolicy = settings.policy.value();
        }

        let res = unsafe {
            nvcall!(NvAPI_GPU_ClientFanCoolersSetControl(self.0, &data))
        };

        match res {
            Err(e) => e.allow_version_incompat()?,
            res => return res,
        }

        unsafe {
            nvcall!(NvAPI_GPU_SetCoolerLevels(self.0, cooler::private::NVAPI_COOLER_TARGET_ALL.repr() as _, &backup))
        }
    }

    #[cfg(never)]
    pub fn restore_cooler_settings(&self, index: &[u32]) -> Result<()> {
        trace!("gpu.restore_cooler_settings({:?})", index);
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        unsafe {
            nvcall!(NvAPI_GPU_RestoreCoolerSettings(self.0, ptr, index.len() as u32))
        }
    }

    #[cfg(never)]
    pub fn cooler_policy_table(&self, index: u32, policy: crate::thermal::CoolerPolicy) -> Result<crate::thermal::CoolerPolicyTable> {
        self.handle().cooler_policy_table_v1(index, policy).map(Into::into)
    }

    #[cfg(never)]
    pub fn set_cooler_policy_table(&self, index: u32, value: &crate::thermal::CoolerPolicyTable) -> Result<()> {
        trace!("gpu.set_cooler_policy_table({:?}, {:?})", index, value);
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::default();
        data.policy = value.policy.value();
        // TODO: data.policyCoolerLevel

        unsafe {
            self.handle().set_cooler_policy_table_v1(index, &data, levels.len())
        }
    }

    pub fn restore_cooler_policy_table(&self, index: &[u32], policy: crate::thermal::CoolerPolicy) -> NvapiResult<()> {
        unsafe {
            //let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
            self.handle().RestoreCoolerPolicyTable(index.as_ptr(), index.len() as u32, policy.into())
                .with_api(Api::NvAPI_GPU_RestoreCoolerPolicyTable)
        }
    }

    pub fn fan_arbiter_info(&self) -> NvapiResult<crate::thermal::FanArbiters> {
        self.handle().ClientFanArbitersGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientFanArbitersGetInfo)
            .map(crate::thermal::FanArbitersV1::from)
            .map(Into::into)
    }

    pub fn fan_arbiter_status(&self) -> NvapiResult<crate::thermal::FanArbitersStatus> {
        self.handle().ClientFanArbitersGetStatus::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientFanArbitersGetStatus)
            .map(crate::thermal::FanArbitersStatusV1::from)
            .map(Into::into)
    }

    pub fn fan_arbiter_control(&self) -> NvapiResult<crate::thermal::FanArbitersControl> {
        self.handle().ClientFanArbitersGetControl::<1, _>()
            .with_api(Api::NvAPI_GPU_ClientFanArbitersGetControl)
            .map(crate::thermal::FanArbitersControlV1::from)
            .map(Into::into)
    }

    pub fn perf_info(&self) -> NvapiResult<crate::clock::PerfInfo> {
        self.handle().PerfPoliciesGetInfo::<1, _>()
            .with_api(Api::NvAPI_GPU_PerfPoliciesGetInfo)
            .map(crate::clock::PerfInfoV1::from)
            .map(Into::into)
    }

    pub fn perf_status(&self) -> NvapiResult<crate::clock::PerfStatus> {
        self.handle().PerfPoliciesGetStatus::<1, _>()
            .with_api(Api::NvAPI_GPU_PerfPoliciesGetStatus)
            .map(crate::clock::PerfStatusV1::from)
            .map(Into::into)
    }

    pub fn voltage_domains_status(&self) -> NvapiResult<crate::clock::VoltageStatus> {
        self.handle().GetVoltageDomainsStatus::<1, _>()
            .with_api(Api::NvAPI_GPU_GetVoltageDomainsStatus)
            .map(crate::clock::VoltageStatusV1::from)
            .map(Into::into)
    }

    pub fn voltage_step(&self) -> NvapiResult<crate::clock::VoltageStatus> {
        self.handle().GetVoltageStep::<1, _>()
            .with_api(Api::NvAPI_GPU_GetVoltageStep)
            .map(crate::clock::VoltageStatusV1::from)
            .map(Into::into)
    }

    pub fn voltage_table(&self) -> NvapiResult<crate::clock::VoltageTable> {
        self.handle().GetVoltages::<1, _>()
            .with_api(Api::NvAPI_GPU_GetVoltages)
            .map(crate::clock::VoltageTableV1::from)
            .map(Into::into)
    }

    pub fn performance_decrease(&self) -> NvapiResult<PerformanceDecreaseReason> {
        self.handle().GetPerfDecreaseInfo().map(|flags| flags.get())
            .with_api(Api::NvAPI_GPU_GetPerfDecreaseInfo)
    }

    pub fn display_ids_all(&self) -> NvapiResult<Vec<DisplayId>> {
        let res = self.handle().GetAllDisplayIds3()
            .with_api(Api::NvAPI_GPU_GetAllDisplayIds)
            .map(|ids| ids.into_iter()
                .map(DisplayIdV1::from)
                .map(DisplayId::from)
                .collect()
            );
        allow_version_compat!(try res);

        self.handle().GetAllDisplayIds1()
            .with_api(Api::NvAPI_GPU_GetAllDisplayIds)
            .map(|ids| ids.into_iter()
                .map(DisplayIdV1::from)
                .map(DisplayId::from)
                .collect()
            )
    }

    pub fn display_ids_connected(&self, flags: ConnectedIdsFlags) -> NvapiResult<Vec<DisplayId>> {
        let res = self.handle().GetConnectedDisplayIds3(flags.into())
            .with_api(Api::NvAPI_GPU_GetConnectedDisplayIds)
            .map(|ids| ids.into_iter()
                .map(DisplayIdV1::from)
                .map(DisplayId::from)
                .collect()
            );
        allow_version_compat!(try res);

        self.handle().GetConnectedDisplayIds1(flags.into())
            .with_api(Api::NvAPI_GPU_GetConnectedDisplayIds)
            .map(|ids| ids.into_iter()
                .map(DisplayIdV1::from)
                .map(DisplayId::from)
                .collect()
            )
    }

    pub fn i2c_read(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &mut [u8], speed: i2c::I2cSpeed) -> NvapiResult<usize> {
        //trace!("i2c_read({}, {:?}, {:?}, 0x{:02x}, {:?}, {:?})", display_mask, port, port_is_ddc, address, register, speed);
        let mut data = i2c::NV_I2C_INFO_V3::default();
        data.displayMask = display_mask;
        data.bIsDDCPort = if port_is_ddc { sys::NV_TRUE } else { sys::NV_FALSE } as _;
        data.i2cDevAddress = address << 1;
        data.pbI2cRegAddress = if register.is_empty() { ptr::null_mut() } else { register.as_ptr() as *mut _ };
        data.regAddrSize = register.len() as _;
        data.pbData = bytes.as_mut_ptr();
        data.cbSize = bytes.len() as _;
        data.i2cSpeed = i2c::NVAPI_I2C_SPEED_DEPRECATED;
        data.i2cSpeedKhz = speed.value();
        if let Some(port) = port {
            data.portId = port;
            data.bIsPortIdSet = sys::NV_TRUE as _;
        }

        unsafe {
            self.handle().I2CRead::<3, _>(&mut data)
        }
            .with_api(Api::NvAPI_I2CRead)
            .map(|()| data.cbSize as usize) // TODO: not actually sure if this ever changes?
    }

    pub fn i2c_write(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &[u8], speed: i2c::I2cSpeed) -> NvapiResult<()> {
        //trace!("i2c_write({}, {:?}, {:?}, 0x{:02x}, {:?}, {:?})", display_mask, port, port_is_ddc, address, register, speed);
        let mut data = i2c::NV_I2C_INFO_V3::default();
        data.displayMask = display_mask;
        data.bIsDDCPort = if port_is_ddc { sys::NV_TRUE } else { sys::NV_FALSE } as _;
        data.i2cDevAddress = address << 1;
        data.pbI2cRegAddress = if register.is_empty() { ptr::null_mut() } else { register.as_ptr() as *mut _ };
        data.regAddrSize = register.len() as _;
        data.pbData = bytes.as_ptr() as *mut _;
        data.cbSize = bytes.len() as _;
        data.i2cSpeed = i2c::NVAPI_I2C_SPEED_DEPRECATED;
        data.i2cSpeedKhz = speed.value();
        if let Some(port) = port {
            data.portId = port;
            data.bIsPortIdSet = sys::NV_TRUE as _;
        }

        unsafe {
            self.handle().I2CWrite(&mut data)
        }
            .with_api(Api::NvAPI_I2CWrite)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct PciIdentifiers {
    pub device_id: u32,
    pub subsystem_id: u32,
    pub revision_id: u32,
    pub ext_device_id: u32,
}

impl fmt::Display for PciIdentifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08x} - {:08x} - {:08x} - {:x}", self.device_id, self.subsystem_id, self.ext_device_id, self.revision_id)
    }
}

impl PciIdentifiers {
    pub fn vendor_id(&self) -> NvValue<Vendor> {
        (self.ids().0 as i32).into()
    }

    pub fn product_id(&self) -> u16 {
        self.ids().1
    }

    pub fn ids(&self) -> (u16, u16) {
        let pid = (self.device_id >> 16) as u16;
        let vid = self.device_id as u16;
        if vid == 0x10de && self.subsystem_id != 0 {
            let spid = (self.subsystem_id >> 16) as u16;
            (
                self.subsystem_id as u16,
                if spid == 0 {
                    // Colorful and Inno3D
                    pid
                } else {
                    spid
                }
            )
        } else {
            (vid, pid)
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BusInfo {
    pub id: u32,
    pub slot_id: u32,
    pub irq: u32,
    pub bus: Bus,
}

impl BusInfo {
    pub fn vendor_id(&self) -> Option<NvValue<Vendor>> {
        self.bus.vendor_id()
    }
}

impl fmt::Display for BusInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({}:{} routed to IRQ {})", self.bus, self.id, self.slot_id, self.irq)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Bus {
    Pci {
        ids: PciIdentifiers,
    },
    PciExpress {
        ids: PciIdentifiers,
        lanes: u32,
    },
    Other(NvValue<BusType>),
}

impl Bus {
    pub fn bus_type(&self) -> NvValue<BusType> {
        match self {
            Bus::Pci { .. } => NvValue::<BusType>::Pci,
            Bus::PciExpress { .. } => NvValue::<BusType>::PciExpress,
            &Bus::Other(ty) => ty,
        }
    }

    pub fn pci_ids(&self) -> Option<&PciIdentifiers> {
        match self {
            Bus::Pci { ids } => Some(ids),
            Bus::PciExpress { ids, .. } => Some(ids),
            _ => None,
        }
    }

    pub fn vendor_id(&self) -> Option<NvValue<Vendor>> {
        self.pci_ids().map(|ids| ids.vendor_id())
    }
}

impl fmt::Display for Bus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bus::PciExpress { lanes, .. } => {
                fmt::Display::fmt(&BusType::PciExpress, f)?;
                if *lanes > 0 {
                    write!(f, " x{}", lanes)?;
                }
                Ok(())
            },
            Bus::Pci { .. } => fmt::Display::fmt(&BusType::Pci, f),
            Bus::Other(ty) => fmt::Display::fmt(ty, f),
        }
    }
}

impl Default for Bus {
    fn default() -> Self {
        Bus::Other(Default::default())
    }
}

nvwrap! {
    pub type MemoryInfoV3 = NvData<driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO_V3> {
        pub dedicated_evictions_size: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().dedicatedVideoMemoryEvictionsSize)
            },
        },
        pub dedicated_evictions: u32 {
            @get fn(&self) {
                self.sys().dedicatedVideoMemoryEvictionCount
            },
        },
    };

    impl @Deref(v2: MemoryInfoV2) for MemoryInfoV3 { }
}

nvwrap! {
    pub type MemoryInfoV2 = NvData<driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO_V2> {
        pub dedicated_available_current: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().curAvailableDedicatedVideoMemory)
            },
        },
    };

    impl @Deref(v1: MemoryInfoV1) for MemoryInfoV2 { }
}

nvwrap! {
    pub type MemoryInfoV1 = NvData<driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO_V1> {
        pub dedicated: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().dedicatedVideoMemory)
            },
        },
        pub dedicated_available: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().availableDedicatedVideoMemory)
            },
        },
        pub system: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().systemVideoMemory)
            },
        },
        pub shared: Kibibytes {
            @get fn(&self) {
                Kibibytes(self.sys().sharedSystemMemory)
            },
        },
    };
}

nvwrap! {
    pub enum MemoryInfo {
        V3(MemoryInfoV3),
        V2(MemoryInfoV2),
        V1(MemoryInfoV1),
    }

    impl @StructVersion for MemoryInfo { }
    impl @Deref(MemoryInfoV1) for MemoryInfo { }

    impl MemoryInfo {
    }
}

impl MemoryInfo {
    pub fn dedicated_available_current(&self) -> Option<Kibibytes> {
        match self {
            MemoryInfo::V3(mem) => Some(mem.dedicated_available_current()),
            MemoryInfo::V2(mem) => Some(mem.dedicated_available_current()),
            MemoryInfo::V1(..) => None,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Default)]
pub struct DriverModel {
    pub value: u32,
}

impl DriverModel {
    pub fn new(value: u32) -> Self {
        DriverModel {
            value,
        }
    }

    pub fn wddm(&self) -> (u8, u8) {
        // 2.0 or 1.(value >> 8)
        let major = ((self.value >> 12) & 0xf) as u8;
        (
            major,
            if major == 2 { 0 } else { (self.value >> 8) as u8 & 0xf }
        )
    }
}

impl fmt::Display for DriverModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let wddm = self.wddm();
        write!(f, "WDDM {}.{:02}", wddm.0, wddm.1)
    }
}

impl fmt::Debug for DriverModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:08x})", self, self.value)
    }
}

nvwrap! {
    pub enum DisplayId {
        V1(DisplayIdV1 {
            @type = NvData<display::NV_GPU_DISPLAYIDS> {
                pub connector: MonitorConnectorType {
                    @get fn(&self) {
                        self.sys().connectorType.get()
                    },
                },
                pub display_id: u32 {
                    @get fn(&self) {
                        self.sys().displayId
                    },
                },
                pub flags: DisplayIdFlags {
                    @get fn(&self) {
                        self.sys().flags.truncate()
                    },
                },
            },
        }),
    }

    impl DisplayId {
        pub fn connector(&self) -> MonitorConnectorType;
        pub fn display_id(&self) -> u32;
        pub fn flags(&self) -> DisplayIdFlags;
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Architecture {
    T2X(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_T2X),
    T3X(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_T3X),
    NV40(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_NV40),
    NV50(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_NV50),
    G78(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    G80(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_G80),
    G90(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_G90),
    GT200(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GT200),
    GF100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GF100),
    GK100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GK100),
    GK110(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GK110),
    GK200(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GK200),
    GM000(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    GM200(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GM200),
    GP100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GP100),
    GV100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GV100),
    GV110(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    TU100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_TU100),
    GA100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_GA100),
    AD100(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID_AD100),
    Unknown {
        id: sys::gpu::NV_GPU_ARCHITECTURE_ID,
        implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID,
    },
}

impl Default for Architecture {
    fn default() -> Self {
        Architecture::Unknown {
            id: Default::default(),
            implementation: Default::default(),
        }
    }
}

impl Architecture {
    pub fn new<I: Into<sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID>>(id: ArchitectureId, implementation: I) -> Self {
        Self::with_arch(id.into(), implementation.into())
    }

    pub fn with_arch(id: sys::gpu::NV_GPU_ARCHITECTURE_ID, implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID) -> Self {
        Self::try_with_arch(id, implementation)
            .unwrap_or_else(|_| Self::Unknown {
                id,
                implementation,
            })
    }

    pub fn try_with_arch(id: sys::gpu::NV_GPU_ARCHITECTURE_ID, implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID) -> Result<Self, ArgumentRangeError> {
        Ok(match id {
            sys::gpu::NV_GPU_ARCHITECTURE_T2X => Architecture::T2X(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_T3X => Architecture::T3X(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_NV40 => Architecture::NV40(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_NV50 => Architecture::NV50(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_G78 => Architecture::G78(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_G80 => Architecture::G80(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_G90 => Architecture::G90(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GT200 => Architecture::GT200(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GF100 => Architecture::GF100(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GK100 => Architecture::GK100(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GK110 => Architecture::GK110(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GK200 => Architecture::GK200(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GM000 => Architecture::GM000(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GM200 => Architecture::GM200(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GP100 => Architecture::GP100(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GV100 => Architecture::GV100(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GV110 => Architecture::GV110(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_TU100 => Architecture::TU100(implementation.cast()),
            sys::gpu::NV_GPU_ARCHITECTURE_GA100 => Architecture::GA100(implementation.cast()),
            _ => return Err(ArgumentRangeError),
        })
    }

    pub fn id(&self) -> NvValue<ArchitectureId> {
        match *self {
            Architecture::T2X(..) => ArchitectureId::T2X.value(),
            Architecture::T3X(..) => ArchitectureId::T3X.value(),
            Architecture::NV40(..) => ArchitectureId::NV40.value(),
            Architecture::NV50(..) => ArchitectureId::NV50.value(),
            Architecture::G78(..) => ArchitectureId::G78.value(),
            Architecture::G80(..) => ArchitectureId::G80.value(),
            Architecture::G90(..) => ArchitectureId::G90.value(),
            Architecture::GT200(..) => ArchitectureId::GT200.value(),
            Architecture::GF100(..) => ArchitectureId::GF100.value(),
            Architecture::GK100(..) => ArchitectureId::GK100.value(),
            Architecture::GK110(..) => ArchitectureId::GK110.value(),
            Architecture::GK200(..) => ArchitectureId::GK200.value(),
            Architecture::GM000(..) => ArchitectureId::GM000.value(),
            Architecture::GM200(..) => ArchitectureId::GM200.value(),
            Architecture::GP100(..) => ArchitectureId::GP100.value(),
            Architecture::GV100(..) => ArchitectureId::GV100.value(),
            Architecture::GV110(..) => ArchitectureId::GV110.value(),
            Architecture::TU100(..) => ArchitectureId::TU100.value(),
            Architecture::GA100(..) => ArchitectureId::GA100.value(),
            Architecture::AD100(..) => ArchitectureId::AD100.value(),
            Architecture::Unknown { id, .. } => id,
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Architecture::T2X(i) => fmt::Display::fmt(i.display(), f),
            Architecture::T3X(i) => fmt::Display::fmt(i.display(), f),
            Architecture::NV40(i) => fmt::Display::fmt(i.display(), f),
            Architecture::NV50(i) => fmt::Display::fmt(i.display(), f),
            Architecture::G80(i) => fmt::Display::fmt(i.display(), f),
            Architecture::G90(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GT200(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GF100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GK100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GK110(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GK200(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GM200(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GP100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GV100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::TU100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::GA100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::AD100(i) => fmt::Display::fmt(i.display(), f),
            Architecture::G78(implementation)
            | Architecture::GM000(implementation)
            | Architecture::GV110(implementation)
            => match self.id() {
                id if implementation == 0 => fmt::Display::fmt(&id, f),
                id => write!(f, "{}:{}", id, implementation.display()),
            },
            Architecture::Unknown { implementation, id } =>
                write!(f, "Unknown({}):{}", id.repr(), implementation.repr()),
        }
    }
}

nvwrap! {
    pub enum ArchInfo {
        V1(ArchInfoV1 {
            @type = NvData<gpu::NV_GPU_ARCH_INFO_V1> {
                pub architecture: gpu::NV_GPU_ARCHITECTURE_ID {
                    @get fn(&self) {
                        self.sys().architecture
                    },
                },
                pub implementation: gpu::NV_GPU_ARCH_IMPLEMENTATION_ID {
                    @get fn(&self) {
                        self.sys().implementation
                    },
                },
                pub revision: gpu::NV_GPU_CHIP_REVISION {
                    @get fn(&self) {
                        self.sys().revision
                    },
                },
            },
        }),
    }

    impl @StructVersion for ArchInfo { }

    impl ArchInfo {
        pub fn architecture(&self) -> gpu::NV_GPU_ARCHITECTURE_ID;
        pub fn implementation(&self) -> gpu::NV_GPU_ARCH_IMPLEMENTATION_ID;
        pub fn revision(&self) -> gpu::NV_GPU_CHIP_REVISION;
    }
}

impl ArchInfo {
    pub fn arch(&self) -> Architecture {
        Architecture::with_arch(self.architecture(), self.implementation())
    }
}

impl fmt::Display for ArchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.arch(), self.revision().display())
    }
}

#[cfg(never)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct VfpInfo {
    pub domains: ClockDomainInfo,
    pub mask: VfpMask,
}

#[cfg(never)]
impl VfpInfo {
    pub fn iter<'s>(&'s self, domain: ClockDomain) -> impl Iterator<Item=usize> + 's {
        self.domains.get(domain)
            .into_iter()
            .flat_map(|d| d.vfp_index.range().filter(|&i| self.mask.mask.get_bit(i)))
    }

    pub fn index<'s, 'a, T: 'static>(&'s self, domain: ClockDomain, entries: &'a [T]) -> impl Iterator<Item=(usize, &'a T)> + 's where 'a: 's {
        self.iter(domain).map(move |i| (i, &entries[i]))
    }

    pub fn index_mut<'s, 'a, T: 'static>(&'s self, domain: ClockDomain, entries: &'a mut [T]) -> impl Iterator<Item=(usize, &'a mut T)> + 's where 'a: 's {
        let mut entries = entries.iter_mut().enumerate();
        self.iter(domain).map(move |i| loop {
            match entries.next() {
                None => panic!("entries out of range of {:?}", self),
                Some((ei, _)) if ei < i => (),
                Some(t) => break t,
            }
        })
    }
}
