use crate::prelude_::*;

nvapi! {
    pub type SYS_GetDriverAndBranchVersionFn = extern "C" fn(pDriverVersion: *mut u32, szBuildBranchString: *mut NvAPI_ShortString) -> NvAPI_Status;

    /// This API returns display driver version and driver-branch string.
    pub unsafe fn NvAPI_SYS_GetDriverAndBranchVersion;
}

nvstruct! {
    /// Used in NvAPI_GPU_GetMemoryInfo().
    pub struct NV_DISPLAY_DRIVER_MEMORY_INFO_V1 {
        /// Version info
        pub version: NvVersion,
        /// Size(in kb) of the physical framebuffer.
        pub dedicatedVideoMemory: u32,
        /// Size(in kb) of the available physical framebuffer for allocating video memory surfaces.
        pub availableDedicatedVideoMemory: u32,
        /// Size(in kb) of system memory the driver allocates at load time.
        pub systemVideoMemory: u32,
        /// Size(in kb) of shared system memory that driver is allowed to commit for surfaces across all allocations.
        pub sharedSystemMemory: u32,
    }
}

nvstruct! {
    /// Used in NvAPI_GPU_GetMemoryInfo().
    pub struct NV_DISPLAY_DRIVER_MEMORY_INFO_V2 {
        pub v1: NV_DISPLAY_DRIVER_MEMORY_INFO_V1,
        /// Size(in kb) of the current available physical framebuffer for allocating video memory surfaces.
        pub curAvailableDedicatedVideoMemory: u32,
    }
}

nvstruct! {
    /// Used in NvAPI_GPU_GetMemoryInfo().
    pub struct NV_DISPLAY_DRIVER_MEMORY_INFO_V3 {
        pub v2: NV_DISPLAY_DRIVER_MEMORY_INFO_V2,
        /// Size(in kb) of the total size of memory released as a result of the evictions.
        pub dedicatedVideoMemoryEvictionsSize: u32,
        /// Indicates the number of eviction events that caused an allocation to be removed from dedicated video memory to free GPU
        /// video memory to make room for other allocations.
        pub dedicatedVideoMemoryEvictionCount: u32,
    }
}

nvversion! { NV_DISPLAY_DRIVER_MEMORY_INFO:
    NV_DISPLAY_DRIVER_MEMORY_INFO_V3(3; @inherit(v2: NV_DISPLAY_DRIVER_MEMORY_INFO_V2)),
    NV_DISPLAY_DRIVER_MEMORY_INFO_V2(2; @inherit(v1: NV_DISPLAY_DRIVER_MEMORY_INFO_V1)),
    NV_DISPLAY_DRIVER_MEMORY_INFO_V1(1)
}

nvapi! {
    pub type GPU_GetMemoryInfoFn = extern "C" fn(hPhysicalGpu: handles::NvPhysicalGpuHandle, pMemoryInfo: *mut NV_DISPLAY_DRIVER_MEMORY_INFO) -> NvAPI_Status;

    /// This function retrieves the available driver memory footprint for the specified GPU.
    /// If the GPU is in TCC Mode, only dedicatedVideoMemory will be returned in pMemoryInfo (NV_DISPLAY_DRIVER_MEMORY_INFO).
    #[deprecated = "Do not use this function - it is deprecated in release 520. Instead, use NvAPI_GPU_GetMemoryInfoEx"]
    pub unsafe fn NvAPI_GPU_GetMemoryInfo;
}

nvstruct! {
    /// Used in [NvAPI_GPU_GetMemoryInfoEx]\(\).
    pub struct NV_GPU_MEMORY_INFO_EX_V1 {
        /// Structure version
        pub version: NvVersion,
        /// Size(in bytes) of the physical framebuffer.
        pub dedicatedVideoMemory: u64,
        /// Size(in bytes) of the available physical framebuffer for allocating video memory surfaces.
        pub availableDedicatedVideoMemory: u64,
        /// Size(in bytes) of system memory the driver allocates at load time.
        pub systemVideoMemory: u64,
        /// Size(in bytes) of shared system memory that driver is allowed to commit for surfaces across all allocations.
        pub sharedSystemMemory: u64,
        /// Size(in bytes) of the current available physical framebuffer for allocating video memory surfaces.
        pub curAvailableDedicatedVideoMemory: u64,
        /// Size(in bytes) of the total size of memory released as a result of the evictions.
        pub dedicatedVideoMemoryEvictionsSize: u64,
        /// Indicates the number of eviction events that caused an allocation to be removed from dedicated video memory to free GPU
        /// video memory to make room for other allocations.
        pub dedicatedVideoMemoryEvictionCount: u64,
        /// Size(in bytes) of the total size of memory allocated as a result of the promotions.
        pub dedicatedVideoMemoryPromotionsSize: u64,
        /// Indicates the number of promotion events that caused an allocation to be promoted to dedicated video memory
        pub dedicatedVideoMemoryPromotionCount: u64,
    }
}

nvversion! { NV_GPU_MEMORY_INFO_EX:
    NV_GPU_MEMORY_INFO_EX_V1(1)
}

nvapi! {
    pub type GPU_GetMemoryInfoExFn = extern "C" fn(hPhysicalGpu: handles::NvPhysicalGpuHandle, pMemoryInfo: *mut NV_GPU_MEMORY_INFO_EX) -> NvAPI_Status;

    /// This function retrieves the available driver memory footprint for the specified GPU.
    ///
    /// If the GPU is in TCC Mode, only dedicatedVideoMemory will be returned in pMemoryInfo (NV_GPU_MEMORY_INFO_EX)
    pub unsafe fn NvAPI_GPU_GetMemoryInfoEx;
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;
    use super::NV_DISPLAY_DRIVER_MEMORY_INFO;

    nvapi! {
        /// This has a different offset than the NvAPI_GPU_GetMemoryInfo function despite both returning the same struct
        pub unsafe fn NvAPI_GetDisplayDriverMemoryInfo(hPhysicalGpu: handles::NvPhysicalGpuHandle, pMemoryInfo: *mut NV_DISPLAY_DRIVER_MEMORY_INFO) -> NvAPI_Status;
    }
}
