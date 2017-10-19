use std::ptr;
use void::ResultVoidExt;
use sys::gpu::{self, pstate, clock, power, cooler, thermal};
use sys;
use types::RawConversion;

pub struct PhysicalGpu(sys::handles::NvPhysicalGpuHandle);

impl PhysicalGpu {
    pub fn handle(&self) -> &sys::handles::NvPhysicalGpuHandle {
        &self.0
    }

    pub fn enumerate() -> sys::Result<Vec<Self>> {
        let mut handles = [Default::default(); sys::types::NVAPI_MAX_PHYSICAL_GPUS];
        let mut len = 0;
        match unsafe { gpu::NvAPI_EnumPhysicalGPUs(&mut handles, &mut len) } {
            sys::status::NVAPI_NVIDIA_DEVICE_NOT_FOUND => Ok(Vec::new()),
            status => sys::status_result(status).map(move |_| handles[..len as usize].iter().cloned().map(PhysicalGpu).collect()),
        }
    }

    pub fn tachometer(&self) -> sys::Result<u32> {
        let mut out = 0;
        unsafe {
            sys::status_result(cooler::NvAPI_GPU_GetTachReading(self.0, &mut out))
                .map(move |_| out)
        }
    }

    pub fn full_name(&self) -> sys::Result<String> {
        let mut str = sys::types::short_string();
        unsafe {
            sys::status_result(gpu::NvAPI_GPU_GetFullName(self.0, &mut str))
                .map(|_| str.convert_raw().void_unwrap())
        }
    }

    pub fn clock_frequencies(&self, clock_type: clock::ClockFrequencyType) -> sys::Result<<clock::NV_GPU_CLOCK_FREQUENCIES as RawConversion>::Target> {
        let mut clocks = clock::NV_GPU_CLOCK_FREQUENCIES::zeroed();
        clocks.version = clock::NV_GPU_CLOCK_FREQUENCIES_VER;
        clocks.set_ClockType(clock_type.raw());

        sys::status_result(unsafe { clock::NvAPI_GPU_GetAllClockFrequencies(self.0, &mut clocks) })
            .map(|_| clocks.convert_raw().void_unwrap())
    }

    pub fn current_pstate(&self) -> sys::Result<::pstate::PState> {
        let mut pstate = 0;

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetCurrentPstate(self.0, &mut pstate) })?;

        ::pstate::PState::from_raw(pstate).map_err(From::from)
    }

    pub fn get_pstates(&self) -> sys::Result<<pstate::NV_GPU_PERF_PSTATES20_INFO as RawConversion>::Target> {
        let mut info = pstate::NV_GPU_PERF_PSTATES20_INFO::zeroed();
        info.version = pstate::NV_GPU_PERF_PSTATES20_INFO_VER;

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetPstates20(self.0, &mut info) })
            .and_then(|_| info.convert_raw().map_err(From::from))
    }

    pub fn set_pstates(&self, pstates: &<pstate::NV_GPU_PERF_PSTATES20_INFO as RawConversion>::Target) -> sys::Result<()> {
        let mut info = pstate::NV_GPU_PERF_PSTATES20_INFO::to_raw(pstates);
        info.version = pstate::NV_GPU_PERF_PSTATES20_INFO_VER;
        sys::status_result(unsafe { pstate::private::NvAPI_GPU_SetPstates20(self.0, &info) })
    }

    pub fn dynamic_pstates_info(&self) -> sys::Result<<pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX as RawConversion>::Target> {
        let mut info = pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX::zeroed();
        info.version = pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER;

        sys::status_result(unsafe { pstate::NvAPI_GPU_GetDynamicPstatesInfoEx(self.0, &mut info) })
            .and_then(|_| info.convert_raw().map_err(From::from))
    }

    /// Private and deprecated, use `dynamic_pstates_info()` instead.
    pub fn usages(&self) -> sys::Result<<clock::private::NV_USAGES_INFO as RawConversion>::Target> {
        let mut usages = clock::private::NV_USAGES_INFO::zeroed();
        usages.version = clock::private::NV_USAGES_INFO_VER;

        sys::status_result(unsafe { clock::private::NvAPI_GPU_GetUsages(self.0, &mut usages) })
            .and_then(|_| usages.convert_raw().map_err(From::from))
    }

    pub fn vfp_mask(&self) -> sys::Result<<clock::private::NV_CLOCK_MASKS as RawConversion>::Target> {
        let mut data = clock::private::NV_CLOCK_MASKS::zeroed();
        data.version = clock::private::NV_CLOCK_MASKS_VER;

        sys::status_result(unsafe { clock::private::NvAPI_GPU_GetClockBoostMask(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn vfp_table(&self, mask: &[u32; 4]) -> sys::Result<<clock::private::NV_CLOCK_TABLE as RawConversion>::Target> {
        let mut data = clock::private::NV_CLOCK_TABLE::zeroed();
        data.version = clock::private::NV_CLOCK_TABLE_VER;
        data.mask = *mask;

        sys::status_result(unsafe { clock::private::NvAPI_GPU_GetClockBoostTable(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn vfp_ranges(&self) -> sys::Result<<clock::private::NV_CLOCK_RANGES as RawConversion>::Target> {
        let mut data = clock::private::NV_CLOCK_RANGES::zeroed();
        data.version = clock::private::NV_CLOCK_RANGES_VER;

        sys::status_result(unsafe { clock::private::NvAPI_GPU_GetClockBoostRanges(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn vfp_curve(&self, mask: &[u32; 4]) -> sys::Result<<power::private::NV_VFP_CURVE as RawConversion>::Target> {
        let mut data = power::private::NV_VFP_CURVE::zeroed();
        data.version = power::private::NV_VFP_CURVE_VER;
        data.mask = *mask;

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetVFPCurve(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn core_voltage(&self) -> sys::Result<<power::private::NV_VOLTAGE_STATUS as RawConversion>::Target> {
        let mut data = power::private::NV_VOLTAGE_STATUS::zeroed();
        data.version = power::private::NV_VOLTAGE_STATUS_VER;

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetCurrentVoltage(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn core_voltage_boost(&self) -> sys::Result<<power::private::NV_VOLTAGE_BOOST_PERCENT as RawConversion>::Target> {
        let mut data = power::private::NV_VOLTAGE_BOOST_PERCENT::zeroed();
        data.version = power::private::NV_VOLTAGE_BOOST_PERCENT_VER;

        sys::status_result(unsafe { power::private::NvAPI_GPU_GetCoreVoltageBoostPercent(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_core_voltage_boost(&self, value: <power::private::NV_VOLTAGE_BOOST_PERCENT as RawConversion>::Target) -> sys::Result<()> {
        let mut data = power::private::NV_VOLTAGE_BOOST_PERCENT::to_raw(&value);
        data.version = power::private::NV_VOLTAGE_BOOST_PERCENT_VER;

        sys::status_result(unsafe { power::private::NvAPI_GPU_SetCoreVoltageBoostPercent(self.0, &data) })
    }

    pub fn thermal_settings(&self, index: thermal::ThermalTarget) -> sys::Result<<thermal::NV_GPU_THERMAL_SETTINGS as RawConversion>::Target> {
        let mut data = thermal::NV_GPU_THERMAL_SETTINGS::zeroed();
        data.version = thermal::NV_GPU_THERMAL_SETTINGS_VER;

        sys::status_result(unsafe { thermal::NvAPI_GPU_GetThermalSettings(self.0, index.raw() as u32, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn thermal_info(&self) -> sys::Result<<thermal::private::NV_GPU_THERMAL_INFO as RawConversion>::Target> {
        let mut data = thermal::private::NV_GPU_THERMAL_INFO::zeroed();
        data.version = thermal::private::NV_GPU_THERMAL_INFO_VER;

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesGetInfo(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn thermal_limit(&self) -> sys::Result<<thermal::private::NV_GPU_THERMAL_LIMIT as RawConversion>::Target> {
        let mut data = thermal::private::NV_GPU_THERMAL_LIMIT::zeroed();
        data.version = thermal::private::NV_GPU_THERMAL_LIMIT_VER;

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesGetLimit(self.0, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_thermal_limit(&self, value: &<thermal::private::NV_GPU_THERMAL_LIMIT as RawConversion>::Target) -> sys::Result<()> {
        let mut data = thermal::private::NV_GPU_THERMAL_LIMIT::to_raw(value);
        data.version = thermal::private::NV_GPU_THERMAL_LIMIT_VER;

        sys::status_result(unsafe { thermal::private::NvAPI_GPU_ClientThermalPoliciesSetLimit(self.0, &data) })
    }

    pub fn cooler_settings(&self, index: u32) -> sys::Result<<cooler::private::NV_GPU_COOLER_SETTINGS as RawConversion>::Target> {
        let mut data = cooler::private::NV_GPU_COOLER_SETTINGS::zeroed();
        data.version = cooler::private::NV_GPU_COOLER_SETTINGS_VER;

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_GetCoolerSettings(self.0, index as _, &mut data) })
            .and_then(|_| data.convert_raw().map_err(From::from))
    }

    pub fn set_cooler_levels(&self, index: u32, value: &<cooler::private::NV_GPU_SETCOOLER_LEVEL as RawConversion>::Target) -> sys::Result<()> {
        let mut data = cooler::private::NV_GPU_SETCOOLER_LEVEL::to_raw(value);
        data.version = cooler::private::NV_GPU_SETCOOLER_LEVEL_VER;

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_SetCoolerLevels(self.0, index as _, &mut data) })
    }

    pub fn restore_cooler_settings(&self, index: &[u32]) -> sys::Result<()> {
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        sys::status_result(unsafe { cooler::private::NvAPI_GPU_RestoreCoolerSettings(self.0, ptr, index.len() as u32) })
    }

    pub fn cooler_policy_table(&self, index: u32, policy: ::thermal::CoolerPolicy) -> sys::Result<<cooler::private::NV_GPU_COOLER_POLICY_TABLE as RawConversion>::Target> {
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::zeroed();
        data.version = cooler::private::NV_GPU_COOLER_POLICY_TABLE_VER;
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
        let mut data = cooler::private::NV_GPU_COOLER_POLICY_TABLE::to_raw(value);
        data.version = cooler::private::NV_GPU_COOLER_POLICY_TABLE_VER;

        sys::status_result(unsafe { cooler::private::NvAPI_GPU_SetCoolerPolicyTable(self.0, index, &data, value.levels.len() as u32) })
    }

    pub fn restore_cooler_policy_table(&self, index: &[u32], policy: ::thermal::CoolerPolicy) -> sys::Result<()> {
        let ptr = if index.is_empty() { ptr::null() } else { index.as_ptr() };
        sys::status_result(unsafe { cooler::private::NvAPI_GPU_RestoreCoolerPolicyTable(self.0, ptr, index.len() as u32, policy.raw()) })
    }
}
