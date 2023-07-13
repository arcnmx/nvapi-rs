use crate::prelude_::*;

/// The GPU cooler APIs are used to get and set the fan level or equivalent
/// cooler levels for various target devices associated with the GPU.
pub mod cooler;

/// The GPU performance state APIs are used to get and set various performance
/// levels on a per-GPU basis. P-States are GPU active/executing performance
/// capability and power consumption states.
///
/// P-States range from P0 to P15, with P0 being the highest performance/power
/// state, and P15 being the lowest performance/power state. Each P-State maps
/// to a performance level. Not all P-States are available on a given system.
/// The definition of each P-States are currently as follows:
///
/// - `P0`/`P1` Maximum 3D performance
/// - `P2`/`P3` Balanced 3D performance-power
/// - `P8` Basic HD video playback
/// - `P10` DVD playback
/// - `P12` Minimum idle power consumption
pub mod pstate;

/// The GPU clock control APIs are used to get and set individual clock domains
/// on a per-GPU basis.
pub mod clock;

/// The GPU thermal control APIs are used to get temperature levels from the
/// various thermal sensors associated with the GPU.
pub mod thermal;

pub mod power;

pub mod display;

/// ECC memory error information
pub mod ecc;

nvapi! {
    pub type EnumPhysicalGPUsFn = extern "C" fn(nvGPUHandle@out: *mut [NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS], pGpuCount@out: *mut u32) -> NvAPI_Status;

    /// This function returns an array of physical GPU handles.
    /// Each handle represents a physical GPU present in the system.
    /// That GPU may be part of an SLI configuration, or may not be visible to the OS directly.
    ///
    /// At least one GPU must be present in the system and running an NVIDIA display driver.
    ///
    /// The array nvGPUHandle will be filled with physical GPU handle values. The returned
    /// gpuCount determines how many entries in the array are valid.
    ///
    /// Note: In drivers older than 105.00, all physical GPU handles get invalidated on a
    /// modeset. So the calling applications need to renum the handles after every modeset.
    /// With drivers 105.00 and up, all physical GPU handles are constant.
    /// Physical GPU handles are constant as long as the GPUs are not physically moved and
    /// the SBIOS VGA order is unchanged.
    ///
    /// For GPU handles in TCC MODE please use NvAPI_EnumTCCPhysicalGPUs()
    pub fn NvAPI_EnumPhysicalGPUs;
}

nvapi! {
    pub type GPU_GetFullNameFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, szName@out: *mut NvAPI_ShortString) -> NvAPI_Status;

    /// This function retrieves the full GPU name as an ASCII string - for example, "Quadro FX 1400".
    pub fn NvAPI_GPU_GetFullName;

    impl self {
        pub fn GetFullName;
    }
}

nvapi! {
    pub type GPU_GetPhysicalFrameBufferSizeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pSize@out: *mut u32) -> NvAPI_Status;

    /// This function returns the physical size of framebuffer in KB.  This does NOT include any
    /// system RAM that may be dedicated for use by the GPU.
    pub fn NvAPI_GPU_GetPhysicalFrameBufferSize;

    impl self {
        pub fn GetPhysicalFrameBufferSize;
    }
}

nvapi! {
    pub type GPU_GetVirtualFrameBufferSizeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pSize@out: *mut u32) -> NvAPI_Status;

    /// This function returns the virtual size of framebuffer in KB.  This includes the physical RAM plus any
    /// system RAM that has been dedicated for use by the GPU.
    pub fn NvAPI_GPU_GetVirtualFrameBufferSize;

    impl self {
        pub fn GetVirtualFrameBufferSize;
    }
}

nvapi! {
    pub type GPU_GetVbiosRevision = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pBiosRevision@out: *mut u32) -> NvAPI_Status;

    /// This function returns the revision of the video BIOS associated with this GPU.
    pub fn NvAPI_GPU_GetVbiosRevision;

    impl self {
        pub fn GetVbiosRevision;
    }
}

nvapi! {
    pub type GPU_GetVbiosOEMRevision = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pBiosRevision@out: *mut u32) -> NvAPI_Status;

    /// This function returns the OEM revision of the video BIOS associated with this GPU.
    pub fn NvAPI_GPU_GetVbiosOEMRevision;

    impl self {
        pub fn GetVbiosOEMRevision;
    }
}

nvapi! {
    pub type GPU_GetVbiosVersionStringFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, szBiosRevision@out: *mut NvAPI_ShortString) -> NvAPI_Status;

    /// This function returns the full video BIOS version string in the form of xx.xx.xx.xx.yy where
    /// - xx numbers come from NvAPI_GPU_GetVbiosRevision() and
    /// - yy comes from NvAPI_GPU_GetVbiosOEMRevision().
    pub fn NvAPI_GPU_GetVbiosVersionString;

    impl self {
        pub fn GetVbiosVersionString;
    }
}

nvapi! {
    pub type GPU_GetPCIIdentifiersFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pDeviceId@out: *mut u32, pSubSystemId@out: *mut u32, pRevisionId@out: *mut u32, pExtDeviceId@out: *mut u32) -> NvAPI_Status;

    /// This function returns the PCI identifiers associated with this GPU.
    pub fn NvAPI_GPU_GetPCIIdentifiers;

    impl self {
        pub fn GetPCIIdentifiers;
    }
}

nvenum! {
    /// Used in NvAPI_GPU_GetGPUType().
    pub enum NV_GPU_TYPE / GpuType {
        NV_SYSTEM_TYPE_GPU_UNKNOWN / Unknown = 0,
        /// Integrated GPU
        NV_SYSTEM_TYPE_IGPU / Integrated = 1,
        /// Discrete GPU
        NV_SYSTEM_TYPE_DGPU / Discrete = 2,
    }
}
nvenum_display! {
    GpuType => {
        Unknown = "Unknown",
        Integrated = "iGPU",
        Discrete = "dGPU",
    }
}

nvapi! {
    pub type GPU_GetGPUTypeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pGpuType@out: *mut NV_GPU_TYPE) -> NvAPI_Status;

    /// This function returns the GPU type (integrated or discrete).
    ///
    /// See [GpuType].
    pub fn NvAPI_GPU_GetGPUType;

    impl self {
        pub fn GetGPUType;
    }
}

nvenum! {
    /// Used in NvAPI_GPU_GetBusType()
    pub enum NV_GPU_BUS_TYPE / BusType {
        NVAPI_GPU_BUS_TYPE_UNDEFINED / Unknown = 0,
        NVAPI_GPU_BUS_TYPE_PCI / Pci = 1,
        NVAPI_GPU_BUS_TYPE_AGP / Agp = 2,
        NVAPI_GPU_BUS_TYPE_PCI_EXPRESS / PciExpress = 3,
        NVAPI_GPU_BUS_TYPE_FPCI / Fpci = 4,
        NVAPI_GPU_BUS_TYPE_AXI / Axi = 5,
    }
}

nvenum_display! {
    BusType => {
        Unknown = "Unknown",
        Pci = "PCI",
        Agp = "AGP",
        PciExpress = "PCIe",
        Fpci = "FPCI",
        Axi = "AXI",
    }
}

impl Default for BusType {
    fn default() -> Self {
        BusType::Unknown
    }
}

nvapi! {
    pub type GPU_GetBusTypeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pBusType@out: *mut NV_GPU_BUS_TYPE) -> NvAPI_Status;

    /// This function returns the type of bus associated with this GPU.
    ///
    /// See [BusType].
    pub fn NvAPI_GPU_GetBusType;

    impl self {
        pub fn GetBusType;
    }
}

nvapi! {
    pub type GPU_GetBusId = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pBusId@out: *mut u32) -> NvAPI_Status;

    /// Returns the ID of the bus associated with this GPU.
    pub fn NvAPI_GPU_GetBusId;

    impl self {
        pub fn GetBusId;
    }
}

nvapi! {
    pub type GPU_GetBusSlotId = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pBusSlotId@out: *mut u32) -> NvAPI_Status;

    /// Returns the ID of the bus slot associated with this GPU.
    pub fn NvAPI_GPU_GetBusSlotId;

    impl self {
        pub fn GetBusSlotId;
    }
}

nvapi! {
    pub type GPU_GetIRQ = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pIRQ@out: *mut u32) -> NvAPI_Status;

    /// This function returns the interrupt number associated with this GPU.
    pub fn NvAPI_GPU_GetIRQ;

    impl self {
        pub fn GetIRQ;
    }
}

nvapi! {
    pub type GPU_GetCurrentPCIEDownstreamWidth = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pWidth@out: *mut u32) -> NvAPI_Status;

    /// This function returns the number of PCIE lanes being used for the PCIE interface
    /// downstream from the GPU.
    pub fn NvAPI_GPU_GetCurrentPCIEDownstreamWidth;

    impl self {
        pub fn GetCurrentPCIEDownstreamWidth;
    }
}

nvenum! {
    /// Used in NvAPI_GPU_GetSystemType()
    pub enum NV_SYSTEM_TYPE / SystemType {
        NV_SYSTEM_TYPE_UNKNOWN / Unknown = 0,
        NV_SYSTEM_TYPE_LAPTOP / Laptop = 1,
        NV_SYSTEM_TYPE_DESKTOP / Desktop = 2,
    }
}

nvenum_display! {
    SystemType => _
}

nvapi! {
    pub type GPU_GetSystemTypeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pSystemType@out: *mut NV_SYSTEM_TYPE) -> NvAPI_Status;

    /// This function identifies whether the GPU is a notebook GPU or a desktop GPU.
    pub fn NvAPI_GPU_GetSystemType;

    impl self {
        pub fn GetSystemType;
    }
}

nvapi! {
    pub type GPU_GetShaderSubPipeCountFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pCount@out: *mut u32) -> NvAPI_Status;

    /// This function retrieves the number of Shader SubPipes on the GPU
    /// On newer architectures, this corresponds to the number of SM units
    pub fn NvAPI_GPU_GetShaderSubPipeCount;

    impl self {
        pub fn GetShaderSubPipeCount;
    }
}

nvapi! {
    pub type GPU_GetGpuCoreCountFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pCount@out: *mut u32) -> NvAPI_Status;

    /// Retrieves the total number of cores defined for a GPU.
    /// Returns 0 on architectures that don't define GPU cores.
    pub fn NvAPI_GPU_GetGpuCoreCount;

    impl self {
        pub fn GetGpuCoreCount;
    }
}

nvstruct! {
    pub struct NV_BOARD_INFO_V1 {
        /// structure version
        pub version: NvVersion,
        /// Board Serial Number
        pub BoardNum: Array<[u8; 16]>,
    }
}

impl Into<[u8; 16]> for NV_BOARD_INFO_V1 {
    fn into(self) -> [u8; 16] {
        self.BoardNum.data
    }
}

nvversion! { NV_BOARD_INFO(NvAPI_GPU_GetBoardInfo):
    NV_BOARD_INFO_V1(1)
}

nvapi! {
    /// This API Retrieves the Board information (a unique GPU Board Serial Number) stored in the InfoROM.
    pub fn NvAPI_GPU_GetBoardInfo(hPhysicalGpu@self: NvPhysicalGpuHandle, pBoardInfo@StructVersionOut: *mut NV_BOARD_INFO) -> NvAPI_Status;

    impl self {
        pub fn GetBoardInfo;
    }
}

nvapi! {
    pub type GPU_GetRamBusWidthFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pRamBusWidth@out: *mut u32) -> NvAPI_Status;

    /// This function returns the width of the GPU's RAM memory bus.
    pub fn NvAPI_GPU_GetRamBusWidth;

    impl self {
        pub fn GetRamBusWidth;
    }
}


nvbits! {
    /// Bit masks for knowing the exact reason for performance decrease
    ///
    /// Used in `NvAPI_GPU_GetPerfDecreaseInfo`
    pub enum NVAPI_GPU_PERF_DECREASE / PerformanceDecreaseReason {
        NV_GPU_PERF_DECREASE_NONE / NONE = 0x00,
        NV_GPU_PERF_DECREASE_REASON_THERMAL_PROTECTION / THERMAL_PROTECTION = 0x01,
        NV_GPU_PERF_DECREASE_REASON_POWER_CONTROL / POWER_CONTROL = 0x02,
        NV_GPU_PERF_DECREASE_REASON_AC_BATT / AC_BATTERY = 0x04,
        NV_GPU_PERF_DECREASE_REASON_API_TRIGGERED / API_TRIGGERED = 0x08,
        NV_GPU_PERF_DECREASE_REASON_INSUFFICIENT_POWER / INSUFFICIENT_POWER = 0x10,
        NV_GPU_PERF_DECREASE_REASON_UNKNOWN / UNKNOWN = 0x00,
    }
}

nvapi! {
    /// This function retrieves reasons for the current performance decrease.
    pub fn NvAPI_GPU_GetPerfDecreaseInfo(hPhysicalGpu@self: NvPhysicalGpuHandle, pPerfDecrInfo@out: *mut NVAPI_GPU_PERF_DECREASE) -> NvAPI_Status;

    impl self {
        pub fn GetPerfDecreaseInfo;
    }
}

nvbits! {
    pub enum NVAPI_GPU_WORKSTATION_FEATURE_MASK / WorkstationFeatureMask {
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_SWAPGROUP / SWAPGROUP = 0x01,
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_STEREO / STEREO = 0x10,
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_WARPING / WARPING = 0x100,
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_PIXINTENSITY / PIXINTENSITY = 0x200,
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_GRAYSCALE / GRAYSCALE = 0x400,
        NVAPI_GPU_WORKSTATION_FEATURE_MASK_BPC10 / BPC10 = 0x1000,
    }
}

nvapi! {
    pub fn NvAPI_GPU_WorkstationFeatureSetup(hPhysicalGpu@self: NvPhysicalGpuHandle, featureEnableMask: NVAPI_GPU_WORKSTATION_FEATURE_MASK, featureDisableMask: NVAPI_GPU_WORKSTATION_FEATURE_MASK) -> NvAPI_Status;

    impl self {
        pub fn WorkstationFeatureSetup;
    }
}

nvapi! {
    /// This API queries the current set of workstation features.
    pub fn NvAPI_GPU_WorkstationFeatureQuery(hPhysicalGpu@self: NvPhysicalGpuHandle, pConfiguredFeatureMask@out: *mut NVAPI_GPU_WORKSTATION_FEATURE_MASK, pConsistentFeatureMask@out: *mut NVAPI_GPU_WORKSTATION_FEATURE_MASK) -> NvAPI_Status;

    impl self {
        pub fn WorkstationFeatureQuery;
    }
}

nvstruct! {
    /// Used in NvAPI_GPU_GetArchInfo()
    pub struct NV_GPU_ARCH_INFO_V1 {
        pub version: NvVersion,
        pub architecture: NV_GPU_ARCHITECTURE_ID,
        pub implementation: NV_GPU_ARCH_IMPLEMENTATION_ID,
        pub revision: NV_GPU_CHIP_REVISION,
    }
}

nvversion! { NV_GPU_ARCH_INFO(NvAPI_GPU_GetArchInfo):
    NV_GPU_ARCH_INFO_V1(2),
    NV_GPU_ARCH_INFO_V1(1; @old)
}

nvapi! {
    pub type GPU_GetArchInfo = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pGpuArchInfo@StructVersionOut: *mut NV_GPU_ARCH_INFO) -> NvAPI_Status;

    pub fn NvAPI_GPU_GetArchInfo;

    impl self {
        pub fn GetArchInfo;
    }
}

nvenum! {
    /// NV_GPU_ARCH_INFO() values to identify Architecture level for the GPU.
    pub enum NV_GPU_ARCHITECTURE_ID / ArchitectureId {
        NV_GPU_ARCHITECTURE_T2X / T2X = 0xE0000020,
        NV_GPU_ARCHITECTURE_T3X / T3X = 0xE0000030,
        NV_GPU_ARCHITECTURE_NV40 / NV40 = 0x00000040,
        NV_GPU_ARCHITECTURE_NV50 / NV50 = 0x00000050,
        NV_GPU_ARCHITECTURE_G78 / G78 = 0x00000060,
        NV_GPU_ARCHITECTURE_G80 / G80 = 0x00000080,
        NV_GPU_ARCHITECTURE_G90 / G90 = 0x00000090,
        NV_GPU_ARCHITECTURE_GT200 / GT200 = 0x000000A0,
        NV_GPU_ARCHITECTURE_GF100 / GF100 = 0x000000C0,
        NV_GPU_ARCHITECTURE_GF110 / GF110 = 0x000000D0,
        NV_GPU_ARCHITECTURE_GK100 / GK100 = 0x000000E0,
        NV_GPU_ARCHITECTURE_GK110 / GK110 = 0x000000F0,
        NV_GPU_ARCHITECTURE_GK200 / GK200 = 0x00000100,
        NV_GPU_ARCHITECTURE_GM000 / GM000 = 0x00000110,
        NV_GPU_ARCHITECTURE_GM200 / GM200 = 0x00000120,
        NV_GPU_ARCHITECTURE_GP100 / GP100 = 0x00000130,
        NV_GPU_ARCHITECTURE_GV100 / GV100 = 0x00000140,
        NV_GPU_ARCHITECTURE_GV110 / GV110 = 0x00000150,
        NV_GPU_ARCHITECTURE_TU100 / TU100 = 0x00000160,
        NV_GPU_ARCHITECTURE_GA100 / GA100 = 0x00000170,
        NV_GPU_ARCHITECTURE_AD100 / AD100 = 0x00000180,
    }
}

pub const NV_GPU_ARCHITECTURE_T4X: NV_GPU_ARCHITECTURE_ID = NV_GPU_ARCHITECTURE_NV40;
pub const NV_GPU_ARCHITECTURE_T12X: NV_GPU_ARCHITECTURE_ID = NV_GPU_ARCHITECTURE_NV40;

nvenum_display! {
    ArchitectureId => {
        NV40 = "NV40 / T12X / T4X",
        _ = _,
    }
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_T2X / ArchitectureImplementationT2X {
        NV_GPU_ARCH_IMPLEMENTATION_T20 / T20 = 0x00000000,
    }
}
nvenum_display! {
    ArchitectureImplementationT2X => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_T3X / ArchitectureImplementationT3X {
        NV_GPU_ARCH_IMPLEMENTATION_T30 / T30 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_T35 / T35 = 0x00000005,
    }
}
nvenum_display! {
    ArchitectureImplementationT3X => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_T4X / ArchitectureImplementationT4X {
        NV_GPU_ARCH_IMPLEMENTATION_T40 / T40 = 0x00000000,
    }
}
nvenum_display! {
    ArchitectureImplementationT4X => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_T12X / ArchitectureImplementationT12X {
        NV_GPU_ARCH_IMPLEMENTATION_T124 / T124 = 0x00000000,
    }
}
nvenum_display! {
    ArchitectureImplementationT12X => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_NV40 / ArchitectureImplementationNV40 {
        NV_GPU_ARCH_IMPLEMENTATION_NV40 / NV40 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_NV41 / NV41 = 0x00000001,
        NV_GPU_ARCH_IMPLEMENTATION_NV42 / NV42 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_NV43 / NV43 = 0x00000003,
        NV_GPU_ARCH_IMPLEMENTATION_NV44 / NV44 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_NV44A / NV44A = 0x0000000A,
        NV_GPU_ARCH_IMPLEMENTATION_NV46 / NV46 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_NV47 / NV47 = 0x00000007,
        NV_GPU_ARCH_IMPLEMENTATION_NV49 / NV49 = 0x00000009,
        NV_GPU_ARCH_IMPLEMENTATION_NV4B / NV4B = 0x0000000B,
        NV_GPU_ARCH_IMPLEMENTATION_NV4C / NV4C = 0x0000000C,
        NV_GPU_ARCH_IMPLEMENTATION_NV4E / NV4E = 0x0000000E,
    }
}
nvenum_display! {
    ArchitectureImplementationNV40 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_NV50 / ArchitectureImplementationNV50 {
        NV_GPU_ARCH_IMPLEMENTATION_NV50 / NV50 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_NV63 / NV63 = 0x00000003,
        NV_GPU_ARCH_IMPLEMENTATION_NV67 / NV67 = 0x00000007,
    }
}
nvenum_display! {
    ArchitectureImplementationNV50 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_G80 / ArchitectureImplementationG80 {
        NV_GPU_ARCH_IMPLEMENTATION_G84 / G84 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_G86 / G86 = 0x00000006,
    }
}
nvenum_display! {
    ArchitectureImplementationG80 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_G90 / ArchitectureImplementationG90 {
        NV_GPU_ARCH_IMPLEMENTATION_G92 / G92 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_G94 / G94 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_G96 / G96 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_G98 / G98 = 0x00000008,
    }
}
nvenum_display! {
    ArchitectureImplementationG90 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GT200 / ArchitectureImplementationGT200 {
        NV_GPU_ARCH_IMPLEMENTATION_GT200 / GT200 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GT212 / GT212 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_GT214 / GT214 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_GT215 / GT215 = 0x00000003,
        NV_GPU_ARCH_IMPLEMENTATION_GT216 / GT216 = 0x00000005,
        NV_GPU_ARCH_IMPLEMENTATION_GT218 / GT218 = 0x00000008,
        NV_GPU_ARCH_IMPLEMENTATION_MCP77 / MCP77 = 0x0000000A,
        NV_GPU_ARCH_IMPLEMENTATION_GT21C / GT21C = 0x0000000B,
        NV_GPU_ARCH_IMPLEMENTATION_MCP79 / MCP79 = 0x0000000C,
        NV_GPU_ARCH_IMPLEMENTATION_GT21A / GT21A = 0x0000000D,
        NV_GPU_ARCH_IMPLEMENTATION_MCP89 / MCP89 = 0x0000000F,
    }
}
nvenum_display! {
    ArchitectureImplementationGT200 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GF100 / ArchitectureImplementationGF100 {
        NV_GPU_ARCH_IMPLEMENTATION_GF100 / GF100 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GF104 / GF104 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_GF106 / GF106 = 0x00000003,
        NV_GPU_ARCH_IMPLEMENTATION_GF108 / GF108 = 0x00000001,
    }
}
nvenum_display! {
    ArchitectureImplementationGF100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GF110 / ArchitectureImplementationGF110 {
        NV_GPU_ARCH_IMPLEMENTATION_GF110 / GF110 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GF116 / GF116 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_GF117 / GF117 = 0x00000007,
        NV_GPU_ARCH_IMPLEMENTATION_GF118 / GF118 = 0x00000008,
        NV_GPU_ARCH_IMPLEMENTATION_GF119 / GF119 = 0x00000009,
    }
}
nvenum_display! {
    ArchitectureImplementationGF110 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GK100 / ArchitectureImplementationGK100 {
        NV_GPU_ARCH_IMPLEMENTATION_GK104 / GK104 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_GK106 / GK106 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_GK107 / GK107 = 0x00000007,
        NV_GPU_ARCH_IMPLEMENTATION_GK20A / GK20A = 0x0000000A,
    }
}
nvenum_display! {
    ArchitectureImplementationGK100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GK110 / ArchitectureImplementationGK110 {
        NV_GPU_ARCH_IMPLEMENTATION_GK110 / GK110 = 0x00000000,
    }
}
nvenum_display! {
    ArchitectureImplementationGK110 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GK200 / ArchitectureImplementationGK200 {
        NV_GPU_ARCH_IMPLEMENTATION_GK208 / GK208 = 0x00000008,
    }
}
nvenum_display! {
    ArchitectureImplementationGK200 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GM200 / ArchitectureImplementationGM200 {
        NV_GPU_ARCH_IMPLEMENTATION_GM204 / GM204 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_GM206 / GM206 = 0x00000006,
    }
}
nvenum_display! {
    ArchitectureImplementationGM200 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GP100 / ArchitectureImplementationGP100 {
        NV_GPU_ARCH_IMPLEMENTATION_GP100 / GP100 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GP000 / GP000 = 0x00000001,
        NV_GPU_ARCH_IMPLEMENTATION_GP102 / GP102 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_GP104 / GP104 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_GP106 / GP106 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_GP107 / GP107 = 0x00000007,
        NV_GPU_ARCH_IMPLEMENTATION_GP108 / GP108 = 0x00000008,
    }
}
nvenum_display! {
    ArchitectureImplementationGP100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GV100 / ArchitectureImplementationGV100 {
        NV_GPU_ARCH_IMPLEMENTATION_GV100 / GV100 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GV10B / GV10B = 0x0000000B,
    }
}
nvenum_display! {
    ArchitectureImplementationGV100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_TU100 / ArchitectureImplementationTU100 {
        NV_GPU_ARCH_IMPLEMENTATION_TU100 / TU100 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_TU102 / TU102 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_TU104 / TU104 = 0x00000004,
        NV_GPU_ARCH_IMPLEMENTATION_TU106 / TU106 = 0x00000006,
        NV_GPU_ARCH_IMPLEMENTATION_TU116 / TU116 = 0x00000008,
        NV_GPU_ARCH_IMPLEMENTATION_TU117 / TU117 = 0x00000007,
        NV_GPU_ARCH_IMPLEMENTATION_TU000 / TU000 = 0x00000001,
    }
}
nvenum_display! {
    ArchitectureImplementationTU100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_GA100 / ArchitectureImplementationGA100 {
        NV_GPU_ARCH_IMPLEMENTATION_GA100 / GA100 = 0x00000000,
        NV_GPU_ARCH_IMPLEMENTATION_GA102 / GA102 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_GA104 / GA104 = 0x00000004,
    }
}
nvenum_display! {
    ArchitectureImplementationGA100 => _
}

nvenum! {
    pub enum NV_GPU_ARCH_IMPLEMENTATION_ID_AD100 / ArchitectureImplementationAD100 {
        NV_GPU_ARCH_IMPLEMENTATION_AD102 / AD102 = 0x00000002,
        NV_GPU_ARCH_IMPLEMENTATION_AD103 / AD103 = 0x00000003,
        NV_GPU_ARCH_IMPLEMENTATION_AD104 / AD104 = 0x00000004,
    }
}
nvenum_display! {
    ArchitectureImplementationAD100 => _
}

pub type NV_GPU_ARCH_IMPLEMENTATION_ID = NV_GPU_ARCH_IMPLEMENTATION_ID_AD100;

nvenum! {
    pub enum NV_GPU_CHIP_REVISION / ChipRevision {
        /// QT chip
        NV_GPU_CHIP_REV_EMULATION_QT / QT = 0x00000000,
        /// FPGA implementation of the chipset
        NV_GPU_CHIP_REV_EMULATION_FPGA / FPGA = 0x00000001,
        /// First silicon chipset revision
        NV_GPU_CHIP_REV_A01 / A01 = 0x00000011,
        /// Second Silicon chipset revision
        NV_GPU_CHIP_REV_A02 / A02 = 0x00000012,
        /// Third Silicon chipset revision
        NV_GPU_CHIP_REV_A03 / A03 = 0x00000013,
        /// Unknown chip revision
        NV_GPU_CHIP_REV_UNKNOWN / Unknown = 0xffffffff,
    }
}

nvenum_display! {
    ChipRevision => _
}

impl Default for ChipRevision {
    fn default() -> Self {
        ChipRevision::Unknown
    }
}

impl NvPhysicalGpuHandle {
    pub fn EnumPhysicalGPUs() -> crate::Result<Truncated<[NvPhysicalGpuHandle; NVAPI_MAX_PHYSICAL_GPUS]>> {
        use crate::nvid::NvAPI_EnumPhysicalGPUs;

        let res = match NvAPI_EnumPhysicalGPUs.call() {
            Err(NvAPI_Status::NvidiaDeviceNotFound) =>
                Ok(([Default::default(); NVAPI_MAX_PHYSICAL_GPUS], 0)),
            res => res,
        };
        res.map(move |(handles, count)| Truncated::new_with(handles, ..count as usize))
    }
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;

    pub const NVAPI_MAX_PROCESSES: usize = 128;

    nvapi! {
        pub type GPU_GetShaderPipeCountFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pCount@out: *mut u32) -> NvAPI_Status;

        pub fn NvAPI_GPU_GetShaderPipeCount;

        impl self {
            pub fn GetShaderPipeCount;
        }
    }

    nvenum! {
        /// Undocumented function NvAPI_GPU_GetRamType()
        pub enum NV_GPU_RAM_TYPE / RamType {
            NV_GPU_RAM_UNKNOWN / Unknown = 0,
            NV_GPU_RAM_SDRAM / SDRAM = 1,
            NV_GPU_RAM_DDR1 / DDR1 = 2,
            NV_GPU_RAM_DDR2 / DDR2 = 3,
            NV_GPU_RAM_GDDR2 / GDDR2 = 4,
            NV_GPU_RAM_GDDR3 / GDDR3 = 5,
            NV_GPU_RAM_GDDR4 / GDDR4 = 6,
            NV_GPU_RAM_DDR3 / DDR3 = 7,
            NV_GPU_RAM_GDDR5 / GDDR5 = 8,
            NV_GPU_RAM_LPDDR2 / LPDDR2 = 9,
            NV_GPU_RAM_GDDR5X / GDDR5X = 10,
            NV_GPU_RAM_GDDR6X / GDDR6X = 15,
        }
    }

    nvenum_display! {
        RamType => _
    }

    nvapi! {
        pub type GPU_GetRamTypeFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pMemType@out: *mut NV_GPU_RAM_TYPE) -> NvAPI_Status;

        /// Undocumented function.
        pub fn NvAPI_GPU_GetRamType;

        impl self {
            pub fn GetRamType;
        }
    }


    nvenum! {
        /// Undocumented function NvAPI_GPU_GetRamMaker()
        pub enum NV_GPU_RAM_MAKER / RamMaker {
            NV_GPU_RAM_MAKER_UNKNOWN / Unknown = 0,
            NV_GPU_RAM_MAKER_SAMSUNG / Samsung = 1,
            NV_GPU_RAM_MAKER_QIMONDA / Qimonda = 2,
            NV_GPU_RAM_MAKER_ELPIDA / Elpida = 3,
            NV_GPU_RAM_MAKER_ETRON / Etron = 4,
            NV_GPU_RAM_MAKER_NANYA / Nanya = 5,
            NV_GPU_RAM_MAKER_HYNIX / Hynix = 6,
            NV_GPU_RAM_MAKER_MOSEL / Mosel = 7,
            NV_GPU_RAM_MAKER_WINBOND / Winbond = 8,
            NV_GPU_RAM_MAKER_ELITE / Elite = 9,
            NV_GPU_RAM_MAKER_MICRON / Micron = 10,
        }
    }

    nvenum_display! {
        RamMaker => _
    }

    nvapi! {
        pub type GPU_GetRamMakerFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pRamMaker@out: *mut NV_GPU_RAM_MAKER) -> NvAPI_Status;

        /// Undocumented function.
        pub fn NvAPI_GPU_GetRamMaker;

        impl self {
            pub fn GetRamMaker;
        }
    }

    nvapi! {
        pub type GPU_GetRamBankCountFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pRamBankCount@out: *mut u32) -> NvAPI_Status;

        /// Undocumented function.
        pub fn NvAPI_GPU_GetRamBankCount;

        impl self {
            pub fn GetRamBankCount;
        }
    }

    nvenum! {
        /// Undocumented function NvAPI_GPU_GetFoundry()
        pub enum NV_GPU_FOUNDRY / Foundry {
            NV_GPU_FOUNDRY_UNKNOWN / Unknown = 0,
            NV_GPU_FOUNDRY_TSMC / TSMC = 1,
            NV_GPU_FOUNDRY_UMC / UMC = 2,
            NV_GPU_FOUNDRY_IBM / IBM = 3,
            NV_GPU_FOUNDRY_SMIC / SMIC = 4,
            NV_GPU_FOUNDRY_CSM / CSM = 5,
            NV_GPU_FOUNDRY_TOSHIBA / Toshiba = 6,
        }
    }

    nvenum_display! {
        Foundry => {
            TSMC = "Taiwan Semiconductor Manufacturing Company (TSMC)",
            UMC = "United Microelectronics Corporation (UMC)",
            IBM = "IBM Microelectronics",
            SMIC = "Semiconductor Manufacturing International Corporation (SMIC)",
            CSM = "Chartered Semiconductor Manufacturing (CSM)",
            Toshiba = "Toshiba Corporation",
            _ = _,
        }
    }

    nvapi! {
        pub type GPU_GetFoundryFn = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pFoundry@out: *mut NV_GPU_FOUNDRY) -> NvAPI_Status;

        /// Undocumented function.
        pub fn NvAPI_GPU_GetFoundry;

        impl self {
            pub fn GetFoundry;
        }
    }

    nvapi! {
        pub fn NvAPI_GPU_GetFBWidthAndLocation(hPhysicalGpu@self: NvPhysicalGpuHandle, pWidth@out: *mut u32, pLocation@out: *mut u32) -> NvAPI_Status;

        impl self {
            pub fn GetFBWidthAndLocation;
        }
    }

    nvenum! {
        pub enum NV_GPU_VENDOR / VendorId {
            NV_GPU_VENDOR_UNKNOWN / Unknown = 0,
            NV_GPU_VENDOR_ASUS / ASUS = 0x1043,
            NV_GPU_VENDOR_ELSA / Elsa = 0x1048,
            NV_GPU_VENDOR_LEADTEK / Leadtek = 0x107d,
            NV_GPU_VENDOR_GAINWARD / Gainward = 0x10b0,
            NV_GPU_VENDOR_NVIDIA / NVIDIA = 0x10de,
            NV_GPU_VENDOR_GIGABYTE / Gigabyte = 0x1458,
            NV_GPU_VENDOR_MSI / MSI = 0x1462,
            NV_GPU_VENDOR_PNY_ / PNY_ = 0x154b, // maybe storage devices
            NV_GPU_VENDOR_PALIT / Palit = 0x1569,
            NV_GPU_VENDOR_XFX / XFX = 0x1682,
            NV_GPU_VENDOR_CLUB3D / Club3D = 0x196d,
            NV_GPU_VENDOR_PNY / PNY = 0x196e,
            NV_GPU_VENDOR_ZOTAC / Zotac = 0x19da,
            NV_GPU_VENDOR_BFG / BFG = 0x19f1,
            NV_GPU_VENDOR_POV / PoV = 0x1acc,
            NV_GPU_VENDOR_GALAX / Galax = 0x1b4c, // KFA2 in EU
            NV_GPU_VENDOR_EVGA / EVGA = 0x3842,
            NV_GPU_VENDOR_COLORFUL / Colorful = 0x7377,
        }
    }

    nvenum_display! {
        VendorId => {
            ASUS = "ASUSTeK Computer Inc.",
            Gigabyte = "Gigabyte Technology",
            MSI = "Micro-Star International",
            PNY_ = "PNY",
            Galax = "Galax / KFA2",
            _ = _,
        }
    }

    impl Default for VendorId {
        fn default() -> Self {
            VendorId::Unknown
        }
    }

    nvapi! {
        pub fn NvAPI_GetGPUIDfromPhysicalGPU(hPhysicalGpu@self: NvPhysicalGpuHandle, gpuid@out: *mut u32) -> NvAPI_Status;

        impl self {
            pub fn GetGPUID;
        }
    }

    nvapi! {
        pub fn NvAPI_GPU_GetShortName(hPhysicalGpu@self: NvPhysicalGpuHandle, pName@out: *mut NvAPI_ShortString) -> NvAPI_Status;

        impl self {
            pub fn GetShortName;
        }
    }

    nvapi! {
        pub fn NvAPI_GPU_GetPartitionCount(hPhysicalGpu@self: NvPhysicalGpuHandle, pPartitionCount@out: *mut u32) -> NvAPI_Status;

        impl self {
            pub fn GetPartitionCount;
        }
    }

    nvapi! {
        pub fn NvAPI_GetDriverModel(hPhysicalGpu@self: NvPhysicalGpuHandle, pDriverModel@out: *mut u32) -> NvAPI_Status;

        impl self {
            pub fn GetDriverModel;
        }
    }
}
