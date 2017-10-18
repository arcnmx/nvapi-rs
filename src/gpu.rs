use std::collections::BTreeMap;
use std::ffi::CStr;
use sys;
use pstate;

pub struct PhysicalGpu(sys::handles::NvPhysicalGpuHandle);

impl PhysicalGpu {
    pub fn handle(&self) -> &sys::handles::NvPhysicalGpuHandle {
        &self.0
    }

    pub fn enumerate() -> sys::Result<Vec<Self>> {
        let mut handles = [Default::default(); sys::types::NVAPI_MAX_PHYSICAL_GPUS];
        let mut len = 0;
        match unsafe { sys::gpu::NvAPI_EnumPhysicalGPUs(&mut handles, &mut len) } {
            sys::status::NVAPI_NVIDIA_DEVICE_NOT_FOUND => Ok(Vec::new()),
            status => sys::status_result(status).map(move |_| handles[..len as usize].iter().cloned().map(PhysicalGpu).collect()),
        }
    }

    pub fn tachometer(&self) -> sys::Result<u32> {
        let mut out = 0;
        unsafe {
            sys::status_result(sys::gpu::cooler::NvAPI_GPU_GetTachReading(self.0, &mut out))
                .map(move |_| out)
        }
    }

    pub fn full_name(&self) -> sys::Result<String> {
        let mut str = sys::types::short_string();
        unsafe {
            sys::status_result(sys::gpu::NvAPI_GPU_GetFullName(self.0, &mut str))
                .map(move |_| CStr::from_ptr(str.as_ptr()).to_string_lossy().into_owned())
        }
    }

    pub fn clock_frequencies(&self, clock_type: sys::gpu::clock::ClockFrequencyType) -> sys::Result<BTreeMap<sys::gpu::clock::PublicClockId, pstate::Kilohertz>> {
        let mut clocks = sys::gpu::clock::NV_GPU_CLOCK_FREQUENCIES::zeroed();
        clocks.version = sys::gpu::clock::NV_GPU_CLOCK_FREQUENCIES_VER;
        clocks.set_ClockType(clock_type.raw());

        sys::status_result(unsafe { sys::gpu::clock::NvAPI_GPU_GetAllClockFrequencies(self.0, &mut clocks) })?;

        Ok([
            sys::gpu::clock::PublicClockId::Graphics,
            sys::gpu::clock::PublicClockId::Memory,
            sys::gpu::clock::PublicClockId::Processor,
            sys::gpu::clock::PublicClockId::Video,
        ].iter()
            .cloned().map(|id| (id, &clocks.domain[id.raw() as usize]))
            .filter(|&(_, clock)| clock.bIsPresent.get())
            .map(|(id, clock)| (id, pstate::Kilohertz(clock.frequency)))
            .collect()
        )
    }

    pub fn current_pstate(&self) -> sys::Result<pstate::PState> {
        let mut pstate = 0;

        sys::status_result(unsafe { sys::gpu::pstate::NvAPI_GPU_GetCurrentPstate(self.0, &mut pstate) })?;

        pstate::PState::from_raw(pstate).map_err(From::from)
    }

    pub fn get_pstates(&self) -> sys::Result<pstate::PStates> {
        let mut info = sys::gpu::pstate::NV_GPU_PERF_PSTATES20_INFO::zeroed();
        info.version = sys::gpu::pstate::NV_GPU_PERF_PSTATES20_INFO_VER;

        sys::status_result(unsafe { sys::gpu::pstate::NvAPI_GPU_GetPstates20(self.0, &mut info) })
            .and_then(|_| pstate::PStates::from_raw(&info))
    }

    pub fn dynamic_pstates_info(&self) -> sys::Result<BTreeMap<sys::gpu::pstate::UtilizationDomain, pstate::Percentage>> {
        let mut info = sys::gpu::pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX::zeroed();
        info.version = sys::gpu::pstate::NV_GPU_DYNAMIC_PSTATES_INFO_EX_VER;

        sys::status_result(unsafe { sys::gpu::pstate::NvAPI_GPU_GetDynamicPstatesInfoEx(self.0, &mut info) })?;

        if info.flag_enabled() {
            Ok(BTreeMap::new())
        } else {
            Ok([
               sys::gpu::pstate::UtilizationDomain::Graphics,
               sys::gpu::pstate::UtilizationDomain::FrameBuffer,
               sys::gpu::pstate::UtilizationDomain::VideoEngine,
               sys::gpu::pstate::UtilizationDomain::BusInterface,
            ].iter()
                .cloned().map(|domain| (domain, &info.utilization[domain.raw() as usize]))
                .filter(|&(_, util)| util.bIsPresent.get())
                .map(|(id, util)| (id, pstate::Percentage(util.percentage)))
                .collect()
            )
        }
    }
}
