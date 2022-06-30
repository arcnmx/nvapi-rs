use std::{ptr, fmt};
use std::convert::Infallible;
use log::trace;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use crate::sys::gpu::{pstate, clock, power, cooler, thermal, display, ecc};
use crate::sys::{self, driverapi, i2c};
use crate::types::{Kibibytes, KilohertzDelta, Kilohertz2Delta, Microvolts, Percentage, Percentage1000, RawConversion};
use crate::thermal::CoolerLevel;
use crate::clock::{ClockDomain, VfpMask};
use crate::pstate::PState;

#[derive(Debug)]
pub struct PhysicalGpu(sys::handles::NvPhysicalGpuHandle);

unsafe impl Send for PhysicalGpu { }

pub use sys::gpu::{SystemType, GpuType, BusType, PerformanceDecreaseReason, WorkstationFeatureMask, ArchitectureId, ChipRevision};
pub use sys::gpu::private::{RamType, RamMaker, Foundry, VendorId as Vendor};
pub use sys::gpu::clock::ClockFrequencyType;
pub use sys::gpu::display::{ConnectedIdsFlags, DisplayIdsFlags, MonitorConnectorType};
pub type ClockFrequencies = <sys::gpu::clock::NV_GPU_CLOCK_FREQUENCIES as RawConversion>::Target;
pub type Utilizations = <pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX as RawConversion>::Target;

impl PhysicalGpu {
    pub fn handle(&self) -> &sys::handles::NvPhysicalGpuHandle {
        &self.0
    }

    pub fn enumerate() -> crate::NvapiResult<Vec<Self>> {
        trace!("gpu.enumerate()");
        let mut handles = [Default::default(); sys::types::NVAPI_MAX_PHYSICAL_GPUS];
        match unsafe { nvcall!(NvAPI_EnumPhysicalGPUs@get(&mut handles)) } {
            Err(crate::NvapiError { status: crate::Status::NvidiaDeviceNotFound, .. }) => Ok(Vec::new()),
            Ok(len) => Ok(handles[..len as usize].iter().cloned().map(PhysicalGpu).collect()),
            Err(e) => Err(e),
        }
    }

    pub fn tachometer(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.tachometer()");
        unsafe {
            nvcall!(NvAPI_GPU_GetTachReading@get(self.0))
        }
    }

    pub fn short_name(&self) -> crate::NvapiResult<String> {
        trace!("gpu.short_name()");
        unsafe {
            nvcall!(NvAPI_GPU_GetShortName@get(self.0) => into)
        }
    }

    pub fn full_name(&self) -> crate::NvapiResult<String> {
        trace!("gpu.full_name()");
        unsafe {
            nvcall!(NvAPI_GPU_GetFullName@get(self.0) => into)
        }
    }

    pub fn vbios_version(&self) -> crate::NvapiResult<(u32, u32)> {
        trace!("gpu.vbios_revision()");
        Ok(unsafe {
            (nvcall!(NvAPI_GPU_GetVbiosRevision@get(self.0))?, nvcall!(NvAPI_GPU_GetVbiosOEMRevision@get(self.0))?)
        })
    }

    pub fn vbios_version_string(&self) -> crate::NvapiResult<String> {
        trace!("gpu.vbios_version_string()");
        unsafe {
            nvcall!(NvAPI_GPU_GetVbiosVersionString@get(self.0) => into)
        }
    }

    pub fn driver_model(&self) -> crate::NvapiResult<DriverModel> {
        trace!("gpu.driver_model()");
        unsafe {
            nvcall!(NvAPI_GetDriverModel@get(self.0))
                .map(DriverModel::new)
        }
    }

    pub fn gpu_id(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.gpu_id()");
        unsafe {
            nvcall!(NvAPI_GetGPUIDFromPhysicalGPU@get(self.0))
        }
    }

    pub fn pci_identifiers(&self) -> crate::NvapiResult<PciIdentifiers> {
        trace!("gpu.pci_identifiers()");
        let mut pci = PciIdentifiers::default();
        unsafe {
            nvcall!(NvAPI_GPU_GetPCIIdentifiers(self.0, &mut pci.device_id, &mut pci.subsystem_id, &mut pci.revision_id, &mut pci.ext_device_id))
                .map(|()| pci)
        }
    }

    pub fn bus_info(&self) -> crate::Result<BusInfo> {
        trace!("gpu.bus_info()");
        let bus_type = self.bus_type()?;
        Ok(BusInfo {
            irq: self.irq()?,
            id: self.bus_id()?,
            slot_id: self.bus_slot_id()?,
            bus: match bus_type {
                BusType::Pci => Bus::Pci {
                    ids: self.pci_identifiers()?,
                },
                BusType::PciExpress => Bus::PciExpress {
                    ids: self.pci_identifiers()?,
                    lanes: self.pcie_lanes()?,
                },
                ty => Bus::Other(ty),
            },
        })
    }

    pub fn gpu_type(&self) -> crate::Result<GpuType> {
        trace!("gpu.gpu_type()");
        unsafe {
            nvcall!(NvAPI_GPU_GetGPUType@get(self.0) => try)
        }
    }

    pub fn bus_type(&self) -> crate::Result<BusType> {
        trace!("gpu.bus_type()");
        unsafe {
            nvcall!(NvAPI_GPU_GetBusType@get(self.0) => try)
        }
    }

    pub fn bus_id(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.bus_id()");
        unsafe {
            nvcall!(NvAPI_GPU_GetBusId@get(self.0))
        }
    }

    pub fn bus_slot_id(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.bus_slot_id()");
        unsafe {
            nvcall!(NvAPI_GPU_GetBusSlotId@get(self.0))
        }
    }

    pub fn irq(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.irq()");
        unsafe {
            nvcall!(NvAPI_GPU_GetIRQ@get(self.0))
        }
    }

    pub fn pcie_lanes(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.pcie_lanes()");
        unsafe {
            nvcall!(NvAPI_GPU_GetCurrentPCIEDownstreamWidth@get(self.0))
        }
    }

    pub fn board_number(&self) -> crate::NvapiResult<[u8; 0x10]> {
        trace!("gpu.board_number()");
        unsafe {
            nvcall!(NvAPI_GPU_GetBoardInfo@get(self.0))
                .map(|data| data.BoardNum)
        }
    }

    pub fn system_type(&self) -> crate::Result<SystemType> {
        trace!("gpu.system_type()");
        unsafe {
            nvcall!(NvAPI_GPU_GetSystemType@get(self.0) => try)
        }
    }

    pub fn core_count(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.core_count()");
        unsafe {
            nvcall!(NvAPI_GPU_GetGpuCoreCount@get(self.0))
        }
    }

    pub fn shader_pipe_count(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.shader_pipe_count()");
        unsafe {
            nvcall!(NvAPI_GPU_GetShaderPipeCount@get(self.0))
        }
    }

    pub fn shader_sub_pipe_count(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.shader_sub_pipe_count()");
        unsafe {
            nvcall!(NvAPI_GPU_GetShaderSubPipeCount@get(self.0))
        }
    }

    pub fn ram_type(&self) -> crate::Result<RamType> {
        trace!("gpu.ram_type()");
        unsafe {
            nvcall!(NvAPI_GPU_GetRamType@get(self.0) => try)
        }
    }

    pub fn ram_maker(&self) -> crate::Result<RamMaker> {
        trace!("gpu.ram_maker()");
        unsafe {
            nvcall!(NvAPI_GPU_GetRamMaker@get(self.0) => try)
        }
    }

    pub fn ram_bus_width(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.ram_bus_width()");
        unsafe {
            nvcall!(NvAPI_GPU_GetRamBusWidth@get(self.0))
        }
    }

    pub fn ram_bank_count(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.ram_bank_count()");
        unsafe {
            nvcall!(NvAPI_GPU_GetRamBankCount@get(self.0))
        }
    }

    pub fn ram_partition_count(&self) -> crate::NvapiResult<u32> {
        trace!("gpu.ram_partition_count()");
        unsafe {
            nvcall!(NvAPI_GPU_GetPartitionCount@get(self.0))
        }
    }

    pub fn foundry(&self) -> crate::Result<Foundry> {
        trace!("gpu.foundry()");
        unsafe {
            nvcall!(NvAPI_GPU_GetFoundry@get(self.0) => try)
        }
    }

    pub fn memory_info(&self) -> crate::NvapiResult<MemoryInfo> {
        trace!("gpu.memory_info()");

        unsafe {
            nvcall!(NvAPI_GPU_GetMemoryInfo@get(self.0) => raw)
        }
    }

    pub fn architecture(&self) -> crate::NvapiResult<ArchInfo> {
        trace!("gpu.architecture()");

        unsafe {
            nvcall!(NvAPI_GPU_GetArchInfo@get(self.0) => raw)
        }
    }

    pub fn workstation_features(&self) -> crate::NvapiResult<(WorkstationFeatureMask, WorkstationFeatureMask)> {
        trace!("gpu.workstation_features()");

        unsafe {
            nvcall!(NvAPI_GPU_WorkstationFeatureQuery@get2(self.0))
                .map(|(configured, consistent)| (
                    WorkstationFeatureMask::from_bits_truncate(configured),
                    WorkstationFeatureMask::from_bits_truncate(consistent),
                ))
        }
    }

    pub fn ecc_status(&self) -> crate::Result<<ecc::NV_GPU_ECC_STATUS_INFO as RawConversion>::Target> {
        trace!("gpu.ecc_status()");

        unsafe {
            nvcall!(NvAPI_GPU_GetECCStatusInfo@get(self.0) => raw)
        }
    }

    pub fn ecc_errors(&self) -> crate::NvapiResult<<ecc::NV_GPU_ECC_ERROR_INFO as RawConversion>::Target> {
        trace!("gpu.ecc_errors()");

        unsafe {
            nvcall!(NvAPI_GPU_GetECCErrorInfo@get(self.0) => raw)
        }
    }

    pub fn ecc_reset(&self, current: bool, aggregate: bool) -> crate::NvapiResult<()> {
        trace!("gpu.ecc_reset({:?}, {:?})", current, aggregate);

        unsafe {
            nvcall!(NvAPI_GPU_ResetECCErrorInfo(self.0, current.into(), aggregate.into()))
        }
    }

    pub fn ecc_configuration(&self) -> crate::NvapiResult<(bool, bool)> {
        trace!("gpu.ecc_configuration()");

        unsafe {
            nvcall!(NvAPI_GPU_GetECCConfigurationInfo@get(self.0))
                .map(|raw| (raw.isEnabled(), raw.isEnabledByDefault()))
        }
    }

    pub fn ecc_configure(&self, enable: bool, immediately: bool) -> crate::NvapiResult<()> {
        trace!("gpu.ecc_configure()");

        unsafe {
            nvcall!(NvAPI_GPU_SetECCConfiguration(self.0, enable.into(), immediately.into()))
        }
    }

    pub fn clock_frequencies(&self, clock_type: ClockFrequencyType) -> crate::NvapiResult<ClockFrequencies> {
        trace!("gpu.clock_frequencies({:?})", clock_type);
        let mut clocks = clock::NV_GPU_CLOCK_FREQUENCIES::default();
        clocks.set_ClockType(clock_type.raw());

        unsafe {
            nvcall!(NvAPI_GPU_GetAllClockFrequencies@get{clocks}(self.0) => raw)
        }
    }

    pub fn current_pstate(&self) -> crate::Result<PState> {
        trace!("gpu.current_pstate()");

        unsafe {
            nvcall!(NvAPI_GPU_GetCurrentPstate@get(self.0) => try)
        }
    }

    pub fn pstates(&self) -> crate::Result<<pstate::NV_GPU_PERF_PSTATES20_INFO as RawConversion>::Target> {
        trace!("gpu.pstates()");

        unsafe {
            nvcall!(NvAPI_GPU_GetPstates20@get(self.0) => raw)
        }
    }

    pub fn set_pstates<I: Iterator<Item=(PState, ClockDomain, KilohertzDelta)>>(&self, deltas: I) -> crate::NvapiResult<()> {
        trace!("gpu.set_pstates()");
        use std::collections::BTreeMap;

        let mut info = pstate::NV_GPU_PERF_PSTATES20_INFO::default();

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
            nvcall!(NvAPI_GPU_SetPstates20(self.0, &info))
        }
    }

    pub fn dynamic_pstates_info(&self) -> crate::Result<Utilizations> {
        trace!("gpu.dynamic_pstates_info()");

        unsafe {
            nvcall!(NvAPI_GPU_GetDynamicPstatesInfoEx@get(self.0) => raw)
        }
    }

    /// Private and deprecated, use `dynamic_pstates_info()` instead.
    pub fn usages(&self) -> crate::Result<<clock::private::NV_USAGES_INFO as RawConversion>::Target> {
        trace!("gpu.usages()");

        unsafe {
            nvcall!(NvAPI_GPU_GetUsages@get(self.0) => raw)
        }
    }

    pub fn vfp_mask(&self) -> crate::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO as RawConversion>::Target> {
        trace!("gpu.vfp_mask()");

        unsafe {
            nvcall!(NvAPI_GPU_ClockClientClkVfPointsGetInfo@get(self.0) => raw)
        }
    }

    pub fn vfp_table(&self, mask: [u32; 4]) -> crate::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL as RawConversion>::Target> {
        trace!("gpu.vfp_table({:?})", mask);
        let mut data = clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL::default();
        data.mask = mask;

        unsafe {
            nvcall!(NvAPI_GPU_ClockClientClkVfPointsGetControl@get{data}(self.0) => raw)
        }
    }

    pub fn set_vfp_table<I: Iterator<Item=(usize, Kilohertz2Delta)>, M: Iterator<Item=(usize, Kilohertz2Delta)>>(&self, mask: [u32; 4], clocks: I, memory: M) -> crate::NvapiResult<()> {
        trace!("gpu.set_vfp_table({:?})", mask);
        let mut data = clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL::default();
        data.mask = mask;
        for (i, delta) in clocks {
            trace!("gpu.set_vfp_table({:?}, {:?})", i, delta);
            data.gpuDeltas[i].freqDeltaKHz = delta.0;
            VfpMask::set_bit(&mut data.mask, i);
        }
        for (i, delta) in memory {
            data.memFilled[i] = 1;
            data.memDeltas[i] = delta.0;
        }

        unsafe {
            nvcall!(NvAPI_GPU_ClockClientClkVfPointsSetControl(self.0, &data))
        }
    }

    pub fn vfp_ranges(&self) -> crate::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO as RawConversion>::Target> {
        trace!("gpu.vfp_ranges()");

        unsafe {
            nvcall!(NvAPI_GPU_ClockClientClkDomainsGetInfo@get(self.0) => raw)
        }
    }

    pub fn vfp_locks(&self) -> crate::Result<<clock::private::NV_GPU_PERF_CLIENT_LIMITS as RawConversion>::Target> {
        trace!("gpu.vfp_locks()");

        unsafe {
            nvcall!(NvAPI_GPU_PerfClientLimitsGetStatus@get(self.0) => raw)
        }
    }

    pub fn set_vfp_locks<I: Iterator<Item=(usize, Option<Microvolts>)>>(&self, values: I) -> crate::NvapiResult<()> {
        trace!("gpu.set_vfp_locks()");
        use crate::clock::ClockLockMode;

        let mut data = clock::private::NV_GPU_PERF_CLIENT_LIMITS::default();
        for (i, (id, voltage)) in values.enumerate() {
            trace!("gpu.set_vfp_lock({:?}, {:?})", id, voltage);
            data.count += 1;
            let entry = &mut data.entries[i];
            entry.id = id as _;
            if let Some(voltage) = voltage {
                entry.mode = ClockLockMode::Manual.raw();
                entry.voltage_uV = voltage.0;
            } else {
                // these are already 0
                //entry.mode = ClockLockMode::None.raw();
                //entry.voltage_uV = 0;
            }
        }

        unsafe {
            nvcall!(NvAPI_GPU_PerfClientLimitsSetStatus(self.0, &data))
        }
    }

    pub fn vfp_curve(&self, mask: [u32; 4]) -> crate::Result<<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS as RawConversion>::Target> {
        trace!("gpu.vfp_curve({:?})", mask);
        let mut data = power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS::default();
        data.mask = mask;

        unsafe {
            nvcall!(NvAPI_GPU_ClockClientClkVfPointsGetStatus@get{data}(self.0) => raw)
        }
    }

    pub fn core_voltage(&self) -> crate::Result<<power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS as RawConversion>::Target> {
        trace!("gpu.core_voltage()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientVoltRailsGetStatus@get(self.0) => raw)
        }
    }

    pub fn core_voltage_boost(&self) -> crate::Result<<power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL as RawConversion>::Target> {
        trace!("gpu.core_voltage_boost()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientVoltRailsGetControl@get(self.0) => raw)
        }
    }

    pub fn set_core_voltage_boost(&self, value: Percentage) -> crate::NvapiResult<()> {
        trace!("gpu.set_core_voltage_boost({:?})", value);
        let mut data = power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL::default();
        data.percent = value.0;

        unsafe {
            nvcall!(NvAPI_GPU_ClientVoltRailsSetControl(self.0, &data))
        }
    }

    pub fn power_usage(&self) -> crate::Result<<power::private::NV_GPU_POWER_TOPO as RawConversion>::Target> {
        trace!("gpu.power_usage()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientPowerTopologyGetStatus@get(self.0) => raw)
        }
    }

    pub fn power_limit_info(&self) -> crate::Result<<power::private::NV_GPU_POWER_INFO as RawConversion>::Target> {
        trace!("gpu.power_limit_info()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientPowerPoliciesGetInfo@get(self.0) => raw)
        }
    }

    pub fn power_limit(&self) -> crate::Result<<power::private::NV_GPU_POWER_STATUS as RawConversion>::Target> {
        trace!("gpu.power_limit()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientPowerPoliciesGetStatus@get(self.0) => raw)
        }
    }

    pub fn set_power_limit<I: Iterator<Item=Percentage1000>>(&self, values: I) -> crate::NvapiResult<()> {
        trace!("gpu.set_power_limit()");
        let mut data = power::private::NV_GPU_POWER_STATUS::default();
        //data.valid = 1;
        for (entry, v) in data.entries.iter_mut().zip(values) {
            trace!("gpu.set_power_limit({:?})", v);
            entry.power = v.0;
            data.count += 1;
        }

        unsafe {
            nvcall!(NvAPI_GPU_ClientPowerPoliciesSetStatus(self.0, &data))
        }
    }

    pub fn thermal_settings(&self, index: Option<u32>) -> crate::Result<<thermal::NV_GPU_THERMAL_SETTINGS as RawConversion>::Target> {
        trace!("gpu.thermal_settings({:?})", index);

        unsafe {
            nvcall!(NvAPI_GPU_GetThermalSettings@get(self.0, index.unwrap_or(thermal::NVAPI_THERMAL_TARGET_ALL as _)) => raw)
        }
    }

    pub fn thermal_limit_info(&self) -> crate::Result<<thermal::private::NV_GPU_THERMAL_INFO as RawConversion>::Target> {
        trace!("gpu.thermal_limit_info()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientThermalPoliciesGetInfo@get(self.0) => raw)
        }
    }

    pub fn thermal_limit(&self) -> crate::Result<<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS as RawConversion>::Target> {
        trace!("gpu.thermal_limit()");

        unsafe {
            nvcall!(NvAPI_GPU_ClientThermalPoliciesGetStatus@get(self.0) => raw)
        }
    }

    pub fn set_thermal_limit<I: Iterator<Item=crate::thermal::ThermalLimit>>(&self, value: I) -> crate::NvapiResult<()> {
        trace!("gpu.set_thermal_limit()");
        let mut data = thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS::default();
        for (entry, v) in data.entries.iter_mut().zip(value) {
            trace!("gpu.set_thermal_limit({:?})", v);
            entry.controller = v.controller.raw();
            entry.value = v.value.0 as _;
            entry.flags = v.flags;
            data.flags += 1;
        }

        unsafe {
            nvcall!(NvAPI_GPU_ClientThermalPoliciesSetStatus(self.0, &data))
        }
    }

    pub fn cooler_settings(&self, index: Option<u32>) -> crate::Result<<cooler::private::NV_GPU_COOLER_SETTINGS as RawConversion>::Target> {
        trace!("gpu.cooler_settings({:?})", index);

        unsafe {
            nvcall!(NvAPI_GPU_GetCoolerSettings@get(self.0, index.unwrap_or(cooler::private::NVAPI_COOLER_TARGET_ALL as _)) => raw)
        }
    }

    pub fn set_cooler_levels<I: Iterator<Item=CoolerLevel>>(&self, index: Option<u32>, values: I) -> crate::NvapiResult<()> {
        trace!("gpu.set_cooler_levels({:?})", index);
        let mut data = cooler::private::NV_GPU_SETCOOLER_LEVEL::default();
        for (entry, level) in data.cooler.iter_mut().zip(values) {
            trace!("gpu.set_cooler_level({:?})", level);
            entry.currentLevel = level.level.0;
            entry.currentPolicy = level.policy.raw();
        }

        unsafe {
            nvcall!(NvAPI_GPU_SetCoolerLevels(self.0, index.unwrap_or(cooler::private::NVAPI_COOLER_TARGET_ALL as _), &data))
        }
    }

    pub fn restore_cooler_settings(&self, index: &[u32]) -> crate::NvapiResult<()> {
        trace!("gpu.restore_cooler_settings({:?})", index);
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        unsafe {
            nvcall!(NvAPI_GPU_RestoreCoolerSettings(self.0, ptr, index.len() as u32))
        }
    }

    pub fn cooler_policy_table(&self, index: u32, policy: crate::thermal::CoolerPolicy) -> crate::Result<<cooler::private::NV_GPU_COOLER_POLICY_TABLE as RawConversion>::Target> {
        trace!("gpu.cooler_policy_table({:?})", index);
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::default();
        data.policy = policy.raw();

        unsafe {
            nvcall!(NvAPI_GPU_GetCoolerPolicyTable@get(self.0, index, &mut data) => err)
                .and_then(|count| data.convert_raw().map_err(From::from).map(|mut c| {
                    c.levels.truncate(count as usize);
                    // TODO: ensure remaining levels are null?
                    c
                }))
        }
    }

    pub fn set_cooler_policy_table(&self, index: u32, value: &<cooler::private::NV_GPU_COOLER_POLICY_TABLE as RawConversion>::Target) -> crate::NvapiResult<()> {
        trace!("gpu.set_cooler_policy_table({:?}, {:?})", index, value);
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::default();
        data.policy = value.policy.raw();
        // TODO: data.policyCoolerLevel

        unsafe {
            nvcall!(NvAPI_GPU_SetCoolerPolicyTable(self.0, index, &data, value.levels.len() as u32))
        }
    }

    pub fn restore_cooler_policy_table(&self, index: &[u32], policy: crate::thermal::CoolerPolicy) -> crate::NvapiResult<()> {
        trace!("gpu.restore_cooler_policy_table({:?}, {:?})", index, policy);
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        unsafe {
            nvcall!(NvAPI_GPU_RestoreCoolerPolicyTable(self.0, ptr, index.len() as u32, policy.raw()))
        }
    }

    pub fn perf_info(&self) -> crate::Result<<power::private::NV_GPU_PERF_INFO as RawConversion>::Target> {
        trace!("gpu.perf_info()");

        unsafe {
            nvcall!(NvAPI_GPU_PerfPoliciesGetInfo@get(self.0) => raw)
        }
    }

    pub fn perf_status(&self) -> crate::Result<<power::private::NV_GPU_PERF_STATUS as RawConversion>::Target> {
        trace!("gpu.perf_status()");

        unsafe {
            nvcall!(NvAPI_GPU_PerfPoliciesGetStatus@get(self.0) => raw)
        }
    }

    pub fn voltage_domains_status(&self) -> crate::Result<<power::private::NV_VOLT_STATUS as RawConversion>::Target> {
        trace!("gpu.voltage_domains_status()");

        unsafe {
            nvcall!(NvAPI_GPU_GetVoltageDomainsStatus@get(self.0) => raw)
        }
    }

    pub fn voltage_step(&self) -> crate::Result<<power::private::NV_VOLT_STATUS as RawConversion>::Target> {
        trace!("gpu.voltage_step()");

        unsafe {
            nvcall!(NvAPI_GPU_GetVoltageStep@get(self.0) => raw)
        }
    }

    pub fn voltage_table(&self) -> crate::Result<<power::private::NV_VOLT_TABLE as RawConversion>::Target> {
        trace!("gpu.voltage_table()");

        unsafe {
            nvcall!(NvAPI_GPU_GetVoltages@get(self.0) => raw)
        }
    }

    pub fn performance_decrease(&self) -> crate::NvapiResult<PerformanceDecreaseReason> {
        trace!("gpu.performance_decrease()");

        unsafe {
            nvcall!(NvAPI_GPU_GetPerfDecreaseInfo@get(self.0))
                .map(|data| PerformanceDecreaseReason::from_bits_truncate(data))
        }
    }

    pub fn display_ids_all(&self) -> crate::Result<Vec<<display::NV_GPU_DISPLAYIDS as RawConversion>::Target>> {
        trace!("gpu.display_ids_all()");
        let mut count = unsafe {
            nvcall!(NvAPI_GPU_GetAllDisplayIds@get(self.0, ptr::null_mut()))
        }?;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut data = vec![display::NV_GPU_DISPLAYIDS::default(); count as usize];

        unsafe {
            nvcall!(NvAPI_GPU_GetAllDisplayIds(self.0, data.as_mut_ptr(), &mut count) => err)
                .and_then(|()| data.into_iter().map(|v| v.convert_raw().map_err(From::from)).collect())
        }
    }

    pub fn display_ids_connected(&self, flags: ConnectedIdsFlags) -> crate::Result<Vec<<display::NV_GPU_DISPLAYIDS as RawConversion>::Target>> {
        trace!("gpu.display_ids_connected({:?})", flags);
        let mut count = unsafe {
            let mut count = 0;
            nvcall!(NvAPI_GPU_GetConnectedDisplayIds(self.0, ptr::null_mut(), &mut count, flags.bits()))
                .map(|()| count)
        }?;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut data = vec![display::NV_GPU_DISPLAYIDS::default(); count as usize];

        unsafe {
            nvcall!(NvAPI_GPU_GetConnectedDisplayIds(self.0, data.as_mut_ptr(), &mut count, flags.bits()) => err)
                .and_then(|()| data.into_iter().map(|v| v.convert_raw().map_err(From::from)).collect())
        }
    }

    pub fn i2c_read(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &mut [u8], speed: i2c::I2cSpeed) -> crate::NvapiResult<usize> {
        trace!("i2c_read({}, {:?}, {:?}, 0x{:02x}, {:?}, {:?})", display_mask, port, port_is_ddc, address, register, speed);
        let mut data = i2c::NV_I2C_INFO::default();
        data.displayMask = display_mask;
        data.bIsDDCPort = if port_is_ddc { sys::NV_TRUE } else { sys::NV_FALSE } as _;
        data.i2cDevAddress = address << 1;
        data.pbI2cRegAddress = if register.is_empty() { ptr::null_mut() } else { register.as_ptr() as *mut _ };
        data.regAddrSize = register.len() as _;
        data.pbData = bytes.as_mut_ptr();
        data.cbSize = bytes.len() as _;
        data.i2cSpeed = i2c::NVAPI_I2C_SPEED_DEPRECATED;
        data.i2cSpeedKhz = speed.raw();
        if let Some(port) = port {
            data.portId = port;
            data.bIsPortIdSet = sys::NV_TRUE as _;
        }

        unsafe {
            nvcall!(NvAPI_I2CRead(self.0, &mut data))
                .map(|()| data.cbSize as usize) // TODO: not actually sure if this ever changes?
        }
    }

    pub fn i2c_write(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &[u8], speed: i2c::I2cSpeed) -> crate::NvapiResult<()> {
        trace!("i2c_write({}, {:?}, {:?}, 0x{:02x}, {:?}, {:?})", display_mask, port, port_is_ddc, address, register, speed);
        let mut data = i2c::NV_I2C_INFO::default();
        data.displayMask = display_mask;
        data.bIsDDCPort = if port_is_ddc { sys::NV_TRUE } else { sys::NV_FALSE } as _;
        data.i2cDevAddress = address << 1;
        data.pbI2cRegAddress = if register.is_empty() { ptr::null_mut() } else { register.as_ptr() as *mut _ };
        data.regAddrSize = register.len() as _;
        data.pbData = bytes.as_ptr() as *mut _;
        data.cbSize = bytes.len() as _;
        data.i2cSpeed = i2c::NVAPI_I2C_SPEED_DEPRECATED;
        data.i2cSpeedKhz = speed.raw();
        if let Some(port) = port {
            data.portId = port;
            data.bIsPortIdSet = sys::NV_TRUE as _;
        }

        unsafe {
            nvcall!(NvAPI_I2CWrite(self.0, &mut data))
        }
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
    pub fn vendor_id(&self) -> u16 {
        self.ids().0
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

    pub fn vendor(&self) -> Result<Vendor, sys::ArgumentRangeError> {
        Vendor::from_raw(self.vendor_id() as _)
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
    pub fn vendor(&self) -> Result<Option<Vendor>, sys::ArgumentRangeError> {
        self.bus.vendor()
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
    Other(BusType),
}

impl Bus {
    pub fn bus_type(&self) -> BusType {
        match self {
            Bus::Pci { .. } => BusType::Pci,
            Bus::PciExpress { .. } => BusType::PciExpress,
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

    pub fn vendor(&self) -> Result<Option<Vendor>, sys::ArgumentRangeError> {
        match self.pci_ids() {
            Some(ids) => ids.vendor().map(Some),
            None => Ok(None),
        }
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MemoryInfo {
    pub dedicated: Kibibytes,
    pub dedicated_available: Kibibytes,
    pub system: Kibibytes,
    pub shared: Kibibytes,
    pub dedicated_available_current: Kibibytes,
    pub dedicated_evictions_size: Kibibytes,
    pub dedicated_evictions: u32,
}

impl RawConversion for driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO {
    type Target = MemoryInfo;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(MemoryInfo {
            dedicated: Kibibytes(self.dedicatedVideoMemory),
            dedicated_available: Kibibytes(self.availableDedicatedVideoMemory),
            system: Kibibytes(self.systemVideoMemory),
            shared: Kibibytes(self.sharedSystemMemory),
            dedicated_available_current: Kibibytes(self.curAvailableDedicatedVideoMemory),
            dedicated_evictions_size: Kibibytes(self.dedicatedVideoMemoryEvictionsSize),
            dedicated_evictions: self.dedicatedVideoMemoryEvictionCount,
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct DisplayId {
    pub connector: MonitorConnectorType,
    pub display_id: u32,
    pub flags: DisplayIdsFlags,
}

impl RawConversion for display::NV_GPU_DISPLAYIDS {
    type Target = DisplayId;
    type Error = sys::ArgumentRangeError;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        Ok(DisplayId {
            connector: MonitorConnectorType::from_raw(self.connectorType)?,
            display_id: self.displayId,
            flags: DisplayIdsFlags::from_bits_truncate(self.flags),
        })
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Architecture {
    T2X(sys::gpu::ArchitectureImplementationT2X),
    T3X(sys::gpu::ArchitectureImplementationT3X),
    NV40(sys::gpu::ArchitectureImplementationNV40),
    NV50(sys::gpu::ArchitectureImplementationNV50),
    G78(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    G80(sys::gpu::ArchitectureImplementationG80),
    G90(sys::gpu::ArchitectureImplementationG90),
    GT200(sys::gpu::ArchitectureImplementationGT200),
    GF100(sys::gpu::ArchitectureImplementationGF100),
    GK100(sys::gpu::ArchitectureImplementationGK100),
    GK110(sys::gpu::ArchitectureImplementationGK110),
    GK200(sys::gpu::ArchitectureImplementationGK200),
    GM000(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    GM200(sys::gpu::ArchitectureImplementationGM200),
    GP100(sys::gpu::ArchitectureImplementationGP100),
    GV100(sys::gpu::ArchitectureImplementationGV100),
    GV110(sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID),
    TU100(sys::gpu::ArchitectureImplementationTU100),
    GA100(sys::gpu::ArchitectureImplementationGA100),
    Unknown {
        id: sys::gpu::NV_GPU_ARCHITECTURE_ID,
        implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID,
    },
}

impl Default for Architecture {
    fn default() -> Self {
        Architecture::Unknown {
            id: 0,
            implementation: 0,
        }
    }
}

impl Architecture {
    pub fn new<I: Into<sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID>>(id: ArchitectureId, implementation: I) -> Self {
        Self::from_raw(id.into(), implementation.into())
    }

    pub fn from_raw(id: sys::gpu::NV_GPU_ARCHITECTURE_ID, implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID) -> Self {
        Self::from_raw_inner(id, implementation)
            .unwrap_or_else(|_| Self::Unknown {
                id,
                implementation,
            })
    }

    fn from_raw_inner(id: sys::gpu::NV_GPU_ARCHITECTURE_ID, implementation: sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID) -> Result<Self, sys::ArgumentRangeError> {
        Ok(match id {
            sys::gpu::NV_GPU_ARCHITECTURE_T2X => Architecture::T2X(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_T3X => Architecture::T3X(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_NV40 => Architecture::NV40(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_NV50 => Architecture::NV50(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_G78 => Architecture::G78(implementation),
            sys::gpu::NV_GPU_ARCHITECTURE_G80 => Architecture::G80(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_G90 => Architecture::G90(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GT200 => Architecture::GT200(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GF100 => Architecture::GF100(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GK100 => Architecture::GK100(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GK110 => Architecture::GK110(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GK200 => Architecture::GK200(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GM000 => Architecture::GM000(implementation),
            sys::gpu::NV_GPU_ARCHITECTURE_GM200 => Architecture::GM200(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GP100 => Architecture::GP100(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GV100 => Architecture::GV100(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GV110 => Architecture::GV110(implementation),
            sys::gpu::NV_GPU_ARCHITECTURE_TU100 => Architecture::TU100(implementation.try_into()?),
            sys::gpu::NV_GPU_ARCHITECTURE_GA100 => Architecture::GA100(implementation.try_into()?),
            _ => return Err(Default::default()),
        })
    }

    pub fn id(&self) -> Result<ArchitectureId, sys::gpu::NV_GPU_ARCHITECTURE_ID> {
        Ok(match *self {
            Architecture::T2X(..) => ArchitectureId::T2X,
            Architecture::T3X(..) => ArchitectureId::T3X,
            Architecture::NV40(..) => ArchitectureId::NV40,
            Architecture::NV50(..) => ArchitectureId::NV50,
            Architecture::G78(..) => ArchitectureId::G78,
            Architecture::G80(..) => ArchitectureId::G80,
            Architecture::G90(..) => ArchitectureId::G90,
            Architecture::GT200(..) => ArchitectureId::GT200,
            Architecture::GF100(..) => ArchitectureId::GF100,
            Architecture::GK100(..) => ArchitectureId::GK100,
            Architecture::GK110(..) => ArchitectureId::GK110,
            Architecture::GK200(..) => ArchitectureId::GK200,
            Architecture::GM000(..) => ArchitectureId::GM000,
            Architecture::GM200(..) => ArchitectureId::GM200,
            Architecture::GP100(..) => ArchitectureId::GP100,
            Architecture::GV100(..) => ArchitectureId::GV100,
            Architecture::GV110(..) => ArchitectureId::GV110,
            Architecture::TU100(..) => ArchitectureId::TU100,
            Architecture::GA100(..) => ArchitectureId::GA100,
            Architecture::Unknown { id, .. } => return id.try_into().map_err(|_| id),
        })
    }

    pub fn raw_id(&self) -> sys::gpu::NV_GPU_ARCHITECTURE_ID {
        self.id().map(|id| id.into()).unwrap_or_else(|id| id)
    }

    pub fn raw_implementation(&self) -> sys::gpu::NV_GPU_ARCH_IMPLEMENTATION_ID {
        match *self {
            Architecture::T2X(i) => i.into(),
            Architecture::T3X(i) => i.into(),
            Architecture::NV40(i) => i.into(),
            Architecture::NV50(i) => i.into(),
            Architecture::G78(i) => i,
            Architecture::G80(i) => i.into(),
            Architecture::G90(i) => i.into(),
            Architecture::GT200(i) => i.into(),
            Architecture::GF100(i) => i.into(),
            Architecture::GK100(i) => i.into(),
            Architecture::GK110(i) => i.into(),
            Architecture::GK200(i) => i.into(),
            Architecture::GM000(i) => i,
            Architecture::GM200(i) => i.into(),
            Architecture::GP100(i) => i.into(),
            Architecture::GV100(i) => i.into(),
            Architecture::GV110(i) => i,
            Architecture::TU100(i) => i.into(),
            Architecture::GA100(i) => i.into(),
            Architecture::Unknown { implementation, .. } => implementation,
        }
    }
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Architecture::T2X(i) => fmt::Display::fmt(i, f),
            Architecture::T3X(i) => fmt::Display::fmt(i, f),
            Architecture::NV40(i) => fmt::Display::fmt(i, f),
            Architecture::NV50(i) => fmt::Display::fmt(i, f),
            Architecture::G80(i) => fmt::Display::fmt(i, f),
            Architecture::G90(i) => fmt::Display::fmt(i, f),
            Architecture::GT200(i) => fmt::Display::fmt(i, f),
            Architecture::GF100(i) => fmt::Display::fmt(i, f),
            Architecture::GK100(i) => fmt::Display::fmt(i, f),
            Architecture::GK110(i) => fmt::Display::fmt(i, f),
            Architecture::GK200(i) => fmt::Display::fmt(i, f),
            Architecture::GM200(i) => fmt::Display::fmt(i, f),
            Architecture::GP100(i) => fmt::Display::fmt(i, f),
            Architecture::GV100(i) => fmt::Display::fmt(i, f),
            Architecture::TU100(i) => fmt::Display::fmt(i, f),
            Architecture::GA100(i) => fmt::Display::fmt(i, f),
            Architecture::G78(implementation)
                | Architecture::GM000(implementation)
                | Architecture::GV110(implementation)
                | Architecture::Unknown { implementation, .. }
                => match self.id() {
                    Ok(ref id) if *implementation == 0 => fmt::Display::fmt(id, f),
                    Ok(id) => write!(f, "{}:{}", id, implementation),
                    Err(id) => write!(f, "Unknown:{}:{}", id, implementation),
                },
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ArchInfo {
    pub arch: Architecture,
    pub revision: sys::gpu::NV_GPU_CHIP_REVISION,
}

impl ArchInfo {
    pub fn revision(&self) -> Result<ChipRevision, sys::gpu::NV_GPU_CHIP_REVISION> {
        self.revision.try_into().map_err(|_| self.revision)
    }
}

impl fmt::Display for ArchInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.arch, f)?;
        match self.revision() {
            Ok(rev) => write!(f, ":{}", rev),
            Err(rev) => write!(f, ":{}", rev),
        }
    }
}

impl RawConversion for sys::gpu::NV_GPU_ARCH_INFO_V2 {
    type Target = ArchInfo;
    type Error = Infallible;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        trace!("convert_raw({:?})", self);
        Ok(ArchInfo {
            arch: Architecture::from_raw(self.architecture, self.implementation),
            revision: self.revision,
        })
    }
}
