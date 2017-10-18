use std::os::raw::c_void;

nv_declare_handle! {
    /// One or more physical GPUs acting in concert (SLI)
    NvLogicalGpuHandle
}

nv_declare_handle! {
    /// A single physical GPU
    NvPhysicalGpuHandle
}

nv_declare_handle! {
    /// Display Device driven by NVIDIA GPU(s) (an attached display)
    NvDisplayHandle
}

nv_declare_handle! {
    /// Monitor handle
    NvMonitorHandle
}

nv_declare_handle! {
    /// Unattached Display Device driven by NVIDIA GPU(s)
    NvUnAttachedDisplayHandle
}

nv_declare_handle! {
    /// A handle to an event registration instance
    NvEventHandle
}

nv_declare_handle! {
    /// A handle to a Visual Computing Device
    NvVisualComputingDeviceHandle
}

nv_declare_handle! {
    /// A handle to a Host Interface Card
    NvHICHandle
}

nv_declare_handle! {
    /// A handle to a Sync device
    NvGSyncDeviceHandle
}

nv_declare_handle! {
    /// A handle to an SDI device
    NvVioHandle
}

nv_declare_handle! {
    /// A handle to address a single transition request
    NvTransitionHandle
}

nv_declare_handle! {
    /// NVIDIA HD Audio Device
    NvAudioHandle
}

nv_declare_handle! {
    /// A handle for a 3D Vision Pro (3DVP) context
    Nv3DVPContextHandle
}

nv_declare_handle! {
    /// A handle for a 3DVP RF transceiver
    Nv3DVPTransceiverHandle
}

nv_declare_handle! {
    /// A handle for a pair of 3DVP RF shutter glasses
    Nv3DVPGlassesHandle
}

/// A stereo handle, that corresponds to the device interface
pub type StereoHandle = *const c_void;

nv_declare_handle! {
    /// Unique source handle on the system
    NvSourceHandle
}

nv_declare_handle! {
    /// Unique target handle on the system
    NvTargetHandle
}

nv_declare_handle! {
    /// DirectX SwapChain objects
    NVDX_SwapChainHandle
}

pub const NVDX_SWAPCHAIN_NONE: NVDX_SwapChainHandle = NVDX_SwapChainHandle(0 as *const _);

pub const NVAPI_DEFAULT_HANDLE: usize = 0;
