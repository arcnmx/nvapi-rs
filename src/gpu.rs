use std::{ptr, fmt};
use std::convert::Infallible;
use log::trace;
use serde::{Serialize, Deserialize};
use crate::sys::gpu::{self, pstate, clock, power, cooler, thermal, display};
use crate::sys::{self, driverapi, i2c};
use crate::types::{Kibibytes, KilohertzDelta, Kilohertz2Delta, Microvolts, Percentage, Percentage1000, RawConversion};
use crate::thermal::CoolerLevel;
use crate::clock::{ClockDomain, VfpMask};
use crate::pstate::PState;

#[derive(Debug)]
pub struct PhysicalGpu(sys::handles::NvPhysicalGpuHandle);

unsafe impl Send for PhysicalGpu { }

pub use sys::gpu::{SystemType, PerformanceDecreaseReason};
pub use sys::gpu::private::{RamType, RamMaker, Foundry, VendorId as Vendor};
pub use sys::gpu::clock::ClockFrequencyType;
pub use sys::gpu::display::{ConnectedIdsFlags, DisplayIdsFlags, MonitorConnectorType};
pub type ClockFrequencies = <sys::gpu::clock::NV_GPU_CLOCK_FREQUENCIES as RawConversion>::Target;
pub type Utilizations = <pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX as RawConversion>::Target;

impl PhysicalGpu {
    pub fn handle(&self) -> &sys::handles::NvPhysicalGpuHandle {
        &self.0
    }

    pub fn enumerate() -> sys::Result<Vec<Self>> {
        trace!("gpu.enumerate()");
        let mut handles = [Default::default(); sys::types::NVAPI_MAX_PHYSICAL_GPUS];
        let mut len = 0;
        match unsafe { gpu::NvAPI_EnumPhysicalGPUs(&mut handles, &mut len) } {
            sys::status::NVAPI_NVIDIA_DEVICE_NOT_FOUND => Ok(Vec::new()),
            status => sys::status_result(status).map(move |_| handles[..len as usize].iter().cloned().map(PhysicalGpu).collect()),
        }
    }

    pub fn tachometer(&self) -> sys::Result<u32> {
        trace!("gpu.tachometer()");
        let mut out = 0;
        unsafe {
            sys::status_result(cooler::NvAPI_GPU_GetTachReading(self.0, &mut out))
                .map(move |_| out)
        }
    }

    pub fn short_name(&self) -> sys::Result<String> {
        trace!("gpu.short_name()");
        let mut str = Default::default();
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetShortName(self.0, &mut str))
                .and_then(|_| str.convert_raw().map_err(Into::into))
        }
    }

    pub fn full_name(&self) -> sys::Result<String> {
        trace!("gpu.full_name()");
        let mut str = Default::default();
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetFullName(self.0, &mut str))
                .and_then(|_| str.convert_raw().map_err(Into::into))
        }
    }

    pub fn vbios_version_string(&self) -> sys::Result<String> {
        trace!("gpu.vbios_version_string()");
        let mut str = Default::default();
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetVbiosVersionString(self.0, &mut str))
                .and_then(|_| str.convert_raw().map_err(Into::into))
        }
    }

    pub fn driver_model(&self) -> sys::Result<DriverModel> {
        trace!("gpu.driver_model()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GetDriverModel(self.0, &mut value))
                .map(|_| DriverModel::new(value))
        }
    }

    pub fn gpu_id(&self) -> sys::Result<u32> {
        trace!("gpu.gpu_id()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GetGPUIDFromPhysicalGPU(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn pci_identifiers(&self) -> sys::Result<PciIdentifiers> {
        trace!("gpu.pci_identifiers()");
        let mut pci = PciIdentifiers::default();
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetPCIIdentifiers(self.0, &mut pci.device_id, &mut pci.subsystem_id, &mut pci.revision_id, &mut pci.ext_device_id))
                .map(|_| pci)
        }
    }

    pub fn board_number(&self) -> sys::Result<[u8; 0x10]> {
        trace!("gpu.board_number()");
        let mut data = gpu::NV_BOARD_INFO::default();
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetBoardInfo(self.0, &mut data))
                .map(|_| data.BoardNum)
        }
    }

    pub fn system_type(&self) -> sys::Result<SystemType> {
        trace!("gpu.system_type()");
        let mut ty = gpu::NV_SYSTEM_TYPE_UNKNOWN;
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetSystemType(self.0, &mut ty))
                .and_then(|_| gpu::SystemType::from_raw(ty).map_err(From::from))
        }
    }

    pub fn core_count(&self) -> sys::Result<u32> {
        trace!("gpu.core_count()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetGpuCoreCount(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn shader_pipe_count(&self) -> sys::Result<u32> {
        trace!("gpu.shader_pipe_count()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetShaderPipeCount(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn shader_sub_pipe_count(&self) -> sys::Result<u32> {
        trace!("gpu.shader_sub_pipe_count()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetShaderSubPipeCount(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn ram_type(&self) -> sys::Result<RamType> {
        trace!("gpu.ram_type()");
        let mut value = gpu::private::NV_GPU_RAM_UNKNOWN;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetRamType(self.0, &mut value))
                .and_then(|_| gpu::private::RamType::from_raw(value).map_err(From::from))
        }
    }

    pub fn ram_maker(&self) -> sys::Result<RamMaker> {
        trace!("gpu.ram_maker()");
        let mut value = gpu::private::NV_GPU_RAM_MAKER_UNKNOWN;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetRamMaker(self.0, &mut value))
                .and_then(|_| gpu::private::RamMaker::from_raw(value).map_err(From::from))
        }
    }

    pub fn ram_bus_width(&self) -> sys::Result<u32> {
        trace!("gpu.ram_bus_width()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetRamBusWidth(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn ram_bank_count(&self) -> sys::Result<u32> {
        trace!("gpu.ram_bank_count()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetRamBankCount(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn ram_partition_count(&self) -> sys::Result<u32> {
        trace!("gpu.ram_partition_count()");
        let mut value = 0;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetPartitionCount(self.0, &mut value))
                .map(|_| value)
        }
    }

    pub fn foundry(&self) -> sys::Result<Foundry> {
        trace!("gpu.foundry()");
        let mut value = gpu::private::NV_GPU_FOUNDRY_UNKNOWN;
        unsafe {
            sys::status_result(gpu::private::NvAPI_GPU_GetFoundry(self.0, &mut value))
                .and_then(|_| gpu::private::Foundry::from_raw(value).map_err(From::from))
        }
    }

    pub fn memory_info(&self) -> sys::Result<MemoryInfo> {
        trace!("gpu.memory_info()");
        let mut data = driverapi::NV_DISPLAY_DRIVER_MEMORY_INFO::default();

        sys::status_result(unsafe { driverapi::NvAPI_GPU_GetMemoryInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(Into::into))
    }

    pub fn clock_frequencies(&self, clock_type: ClockFrequencyType) -> sys::Result<ClockFrequencies> {
        trace!("gpu.clock_frequencies({:?})", clock_type);
        let mut clocks = clock::NV_GPU_CLOCK_FREQUENCIES::default();
        clocks.set_ClockType(clock_type.raw());

        sys::status_result(unsafe { clock::NvAPI_GPU_GetAllClockFrequencies(self.0, &mut clocks) })
            .and_then(|_| clocks.convert_raw().map_err(Into::into))
    }

    pub fn current_pstate(&self) -> sys::Result<PState> {
        trace!("gpu.current_pstate()");
        let mut pstate = 0;

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetCurrentPstate(self.0, &mut pstate) })?;

        PState::from_raw(pstate).map_err(From::from)
    }

    pub fn pstates(&self) -> sys::Result<<pstate::NV_GPU_PERF_PSTATES20_INFO as RawConversion>::Target> {
        trace!("gpu.pstates()");
        let mut info = pstate::NV_GPU_PERF_PSTATES20_INFO::default();

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetPstates20(self.0, &mut info) })
            .and_then(|_| info.convert_raw().map_err(From::from))
    }

    pub fn set_pstates<I: Iterator<Item=(PState, ClockDomain, KilohertzDelta)>>(&self, deltas: I) -> sys::Result<()> {
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

        sys::status_result(unsafe { pstate::private::NvAPI_GPU_SetPstates20(self.0, &info) })
    }

    pub fn dynamic_pstates_info(&self) -> sys::Result<Utilizations> {
        trace!("gpu.dynamic_pstates_info()");
        let mut info = pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX::default();

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetDynamicPstatesInfoEx(self.0, &mut info) })
            .and_then(|_| info.convert_raw().map_err(From::from))
    }

    /// Private and deprecated, use `dynamic_pstates_info()` instead.
    pub fn usages(&self) -> sys::Result<<clock::private::NV_USAGES_INFO as RawConversion>::Target> {
        trace!("gpu.usages()");
        let mut usages = clock::private::NV_USAGES_INFO::default();

        sys::status_result(unsafe { clock::private::NvAPI_GPU_GetUsages(self.0, &mut usages) })
            .and_then(|_| usages.convert_raw().map_err(From::from))
    }

    pub fn vfp_mask(&self) -> sys::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO as RawConversion>::Target> {
        trace!("gpu.vfp_mask()");
        let mut data = clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_INFO::default();

        sys::status_result(unsafe { clock::private::NvAPI_GPU_ClockClientClkVfPointsGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn vfp_table(&self, mask: [u32; 4]) -> sys::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL as RawConversion>::Target> {
        trace!("gpu.vfp_table({:?})", mask);
        let mut data = clock::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_CONTROL::default();
        data.mask = mask;

        sys::status_result(unsafe { clock::private::NvAPI_GPU_ClockClientClkVfPointsGetControl(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_vfp_table<I: Iterator<Item=(usize, Kilohertz2Delta)>, M: Iterator<Item=(usize, Kilohertz2Delta)>>(&self, mask: [u32; 4], clocks: I, memory: M) -> sys::Result<()> {
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

        sys::status_result(unsafe { clock::private::NvAPI_GPU_ClockClientClkVfPointsSetControl(self.0, &data) })
    }

    pub fn vfp_ranges(&self) -> sys::Result<<clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO as RawConversion>::Target> {
        trace!("gpu.vfp_ranges()");
        let mut data = clock::private::NV_GPU_CLOCK_CLIENT_CLK_DOMAINS_INFO::default();

        sys::status_result(unsafe { clock::private::NvAPI_GPU_ClockClientClkDomainsGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn vfp_locks(&self) -> sys::Result<<clock::private::NV_GPU_PERF_CLIENT_LIMITS as RawConversion>::Target> {
        trace!("gpu.vfp_locks()");
        let mut data = clock::private::NV_GPU_PERF_CLIENT_LIMITS::default();

        sys::status_result(unsafe { clock::private::NvAPI_GPU_PerfClientLimitsGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_vfp_locks<I: Iterator<Item=(usize, Option<Microvolts>)>>(&self, values: I) -> sys::Result<()> {
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

        sys::status_result(unsafe { clock::private::NvAPI_GPU_PerfClientLimitsSetStatus(self.0, &data) })
    }

    pub fn vfp_curve(&self, mask: [u32; 4]) -> sys::Result<<power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS as RawConversion>::Target> {
        trace!("gpu.vfp_curve({:?})", mask);
        let mut data = power::private::NV_GPU_CLOCK_CLIENT_CLK_VF_POINTS_STATUS::default();
        data.mask = mask;

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClockClientClkVfPointsGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn core_voltage(&self) -> sys::Result<<power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS as RawConversion>::Target> {
        trace!("gpu.core_voltage()");
        let mut data = power::private::NV_GPU_CLIENT_VOLT_RAILS_STATUS::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientVoltRailsGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn core_voltage_boost(&self) -> sys::Result<<power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL as RawConversion>::Target> {
        trace!("gpu.core_voltage_boost()");
        let mut data = power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientVoltRailsGetControl(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_core_voltage_boost(&self, value: Percentage) -> sys::Result<()> {
        trace!("gpu.set_core_voltage_boost({:?})", value);
        let mut data = power::private::NV_GPU_CLIENT_VOLT_RAILS_CONTROL::default();
        data.percent = value.0;

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientVoltRailsSetControl(self.0, &data) })
    }

    pub fn power_usage(&self) -> sys::Result<<power::private::NV_GPU_POWER_TOPO as RawConversion>::Target> {
        trace!("gpu.power_usage()");
        let mut data = power::private::NV_GPU_POWER_TOPO::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientPowerTopologyGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn power_limit_info(&self) -> sys::Result<<power::private::NV_GPU_POWER_INFO as RawConversion>::Target> {
        trace!("gpu.power_limit_info()");
        let mut data = power::private::NV_GPU_POWER_INFO::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientPowerPoliciesGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn power_limit(&self) -> sys::Result<<power::private::NV_GPU_POWER_STATUS as RawConversion>::Target> {
        trace!("gpu.power_limit()");
        let mut data = power::private::NV_GPU_POWER_STATUS::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientPowerPoliciesGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_power_limit<I: Iterator<Item=Percentage1000>>(&self, values: I) -> sys::Result<()> {
        trace!("gpu.set_power_limit()");
        let mut data = power::private::NV_GPU_POWER_STATUS::default();
        //data.valid = 1;
        for (entry, v) in data.entries.iter_mut().zip(values) {
            trace!("gpu.set_power_limit({:?})", v);
            entry.power = v.0;
            data.count += 1;
        }

        sys::status_result(unsafe { power::private::NvAPI_GPU_ClientPowerPoliciesSetStatus(self.0, &data) })
    }

    pub fn thermal_settings(&self, index: Option<u32>) -> sys::Result<<thermal::NV_GPU_THERMAL_SETTINGS as RawConversion>::Target> {
        trace!("gpu.thermal_settings({:?})", index);
        let mut data = thermal::NV_GPU_THERMAL_SETTINGS::default();

        sys::status_result(unsafe { thermal::NvAPI_GPU_GetThermalSettings(self.0, index.unwrap_or(thermal::NVAPI_THERMAL_TARGET_ALL as _), &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn thermal_limit_info(&self) -> sys::Result<<thermal::private::NV_GPU_THERMAL_INFO as RawConversion>::Target> {
        trace!("gpu.thermal_limit_info()");
        let mut data = thermal::private::NV_GPU_THERMAL_INFO::default();

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn thermal_limit(&self) -> sys::Result<<thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS as RawConversion>::Target> {
        trace!("gpu.thermal_limit()");
        let mut data = thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS::default();

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_thermal_limit<I: Iterator<Item=crate::thermal::ThermalLimit>>(&self, value: I) -> sys::Result<()> {
        trace!("gpu.set_thermal_limit()");
        let mut data = thermal::private::NV_GPU_CLIENT_THERMAL_POLICIES_STATUS::default();
        for (entry, v) in data.entries.iter_mut().zip(value) {
            trace!("gpu.set_thermal_limit({:?})", v);
            entry.controller = v.controller.raw();
            entry.value = v.value.0 as _;
            entry.flags = v.flags;
            data.flags += 1;
        }

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesSetStatus(self.0, &data) })
    }

    pub fn cooler_settings(&self, index: Option<u32>) -> sys::Result<<cooler::private::NV_GPU_COOLER_SETTINGS as RawConversion>::Target> {
        trace!("gpu.cooler_settings({:?})", index);
        let mut data = cooler::private::NV_GPU_COOLER_SETTINGS::default();

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_GetCoolerSettings(self.0, index.unwrap_or(cooler::private::NVAPI_COOLER_TARGET_ALL as _), &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_cooler_levels<I: Iterator<Item=CoolerLevel>>(&self, index: Option<u32>, values: I) -> sys::Result<()> {
        trace!("gpu.set_cooler_levels({:?})", index);
        let mut data = cooler::private::NV_GPU_SETCOOLER_LEVEL::default();
        for (entry, level) in data.cooler.iter_mut().zip(values) {
            trace!("gpu.set_cooler_level({:?})", level);
            entry.currentLevel = level.level.0;
            entry.currentPolicy = level.policy.raw();
        }

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_SetCoolerLevels(self.0, index.unwrap_or(cooler::private::NVAPI_COOLER_TARGET_ALL as _), &data) })
    }

    pub fn restore_cooler_settings(&self, index: &[u32]) -> sys::Result<()> {
        trace!("gpu.restore_cooler_settings({:?})", index);
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        sys::status_result(unsafe { cooler::private::NvAPI_GPU_RestoreCoolerSettings(self.0, ptr, index.len() as u32) })
    }

    pub fn cooler_policy_table(&self, index: u32, policy: crate::thermal::CoolerPolicy) -> sys::Result<<cooler::private::NV_GPU_COOLER_POLICY_TABLE as RawConversion>::Target> {
        trace!("gpu.cooler_policy_table({:?})", index);
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::default();
        data.policy = policy.raw();
        let mut count = 0;

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_GetCoolerPolicyTable(self.0, index, &mut data, &mut count) })
            .and_then(|_| data.convert_raw().map_err(From::from)).map(|mut c| {
                c.levels.truncate(count as usize);
                // TODO: ensure remaining levels are null?
                c
            })
    }

    pub fn set_cooler_policy_table(&self, index: u32, value: &<cooler::private::NV_GPU_COOLER_POLICY_TABLE as RawConversion>::Target) -> sys::Result<()> {
        trace!("gpu.set_cooler_policy_table({:?}, {:?})", index, value);
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::default();
        data.policy = value.policy.raw();
        // TODO: data.policyCoolerLevel

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_SetCoolerPolicyTable(self.0, index, &data, value.levels.len() as u32) })
    }

    pub fn restore_cooler_policy_table(&self, index: &[u32], policy: crate::thermal::CoolerPolicy) -> sys::Result<()> {
        trace!("gpu.restore_cooler_policy_table({:?}, {:?})", index, policy);
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        sys::status_result(unsafe { cooler::private::NvAPI_GPU_RestoreCoolerPolicyTable(self.0, ptr, index.len() as u32, policy.raw()) })
    }

    pub fn perf_info(&self) -> sys::Result<<power::private::NV_GPU_PERF_INFO as RawConversion>::Target> {
        trace!("gpu.perf_info()");
        let mut data = power::private::NV_GPU_PERF_INFO::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_PerfPoliciesGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn perf_status(&self) -> sys::Result<<power::private::NV_GPU_PERF_STATUS as RawConversion>::Target> {
        trace!("gpu.perf_status()");
        let mut data = power::private::NV_GPU_PERF_STATUS::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_PerfPoliciesGetStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn voltage_domains_status(&self) -> sys::Result<<power::private::NV_VOLT_STATUS as RawConversion>::Target> {
        trace!("gpu.voltage_domains_status()");
        let mut data = power::private::NV_VOLT_STATUS::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetVoltageDomainsStatus(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn voltage_step(&self) -> sys::Result<<power::private::NV_VOLT_STATUS as RawConversion>::Target> {
        trace!("gpu.voltage_step()");
        let mut data = power::private::NV_VOLT_STATUS::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetVoltageStep(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn voltage_table(&self) -> sys::Result<<power::private::NV_VOLT_TABLE as RawConversion>::Target> {
        trace!("gpu.voltage_table()");
        let mut data = power::private::NV_VOLT_TABLE::default();

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetVoltages(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn performance_decrease(&self) -> sys::Result<PerformanceDecreaseReason> {
        trace!("gpu.performance_decrease()");

        let mut data = gpu::NV_GPU_PERF_DECREASE_NONE;

        sys::status_result(unsafe { gpu::NvAPI_GPU_GetPerfDecreaseInfo(self.0, &mut data) })
            .map(|_| PerformanceDecreaseReason::from_bits_truncate(data))
    }

    pub fn display_ids_all(&self) -> sys::Result<Vec<<display::NV_GPU_DISPLAYIDS as RawConversion>::Target>> {
        trace!("gpu.display_ids_all()");
        let mut count = 0;
        sys::status_result(unsafe { display::NvAPI_GPU_GetAllDisplayIds(self.0, ptr::null_mut(), &mut count) })?;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut data = vec![display::NV_GPU_DISPLAYIDS::default(); count as usize];

        sys::status_result(unsafe { display::NvAPI_GPU_GetAllDisplayIds(self.0, data.as_mut_ptr(), &mut count) })
            .and_then(|_| data.into_iter().map(|v| v.convert_raw().map_err(From::from)).collect())
    }

    pub fn display_ids_connected(&self, flags: ConnectedIdsFlags) -> sys::Result<Vec<<display::NV_GPU_DISPLAYIDS as RawConversion>::Target>> {
        trace!("gpu.display_ids_connected({:?})", flags);
        let mut count = 0;
        sys::status_result(unsafe { display::NvAPI_GPU_GetConnectedDisplayIds(self.0, ptr::null_mut(), &mut count, flags.bits()) })?;
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut data = vec![display::NV_GPU_DISPLAYIDS::default(); count as usize];

        sys::status_result(unsafe { display::NvAPI_GPU_GetConnectedDisplayIds(self.0, data.as_mut_ptr(), &mut count, flags.bits()) })
            .and_then(|_| data.into_iter().map(|v| v.convert_raw().map_err(From::from)).collect())
    }

    pub fn i2c_read(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &mut [u8], speed: i2c::I2cSpeed) -> sys::Result<usize> {
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

        sys::status_result(unsafe { i2c::NvAPI_I2CRead(self.0, &mut data) })
            .map(|_| data.cbSize as usize) // TODO: not actually sure if this ever changes?
    }

    pub fn i2c_write(&self, display_mask: u32, port: Option<u8>, port_is_ddc: bool, address: u8, register: &[u8], bytes: &[u8], speed: i2c::I2cSpeed) -> sys::Result<()> {
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

        sys::status_result(unsafe { i2c::NvAPI_I2CWrite(self.0, &mut data) })
            .map(drop)
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
