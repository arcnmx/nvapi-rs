use status::NvAPI_Status;
use handles::NvPhysicalGpuHandle;
use types;

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

nvapi! {
    pub type EnumPhysicalGPUsFn = extern "C" fn(nvGPUHandle: *mut [NvPhysicalGpuHandle; types::NVAPI_MAX_PHYSICAL_GPUS], pGpuCount: *mut u32) -> NvAPI_Status;

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
    pub unsafe fn NvAPI_EnumPhysicalGPUs;
}

nvapi! {
    pub type GPU_GetFullNameFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, szName: *mut types::NvAPI_ShortString) -> NvAPI_Status;

    /// This function retrieves the full GPU name as an ASCII string - for example, "Quadro FX 1400".
    pub unsafe fn NvAPI_GPU_GetFullName;
}

nvapi! {
    pub type GPU_GetPhysicalFrameBufferSizeFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pSize: *mut u32) -> NvAPI_Status;

    /// This function returns the physical size of framebuffer in KB.  This does NOT include any
    /// system RAM that may be dedicated for use by the GPU.
    pub unsafe fn NvAPI_GPU_GetPhysicalFrameBufferSize;
}

nvapi! {
    pub type GPU_GetVbiosVersionStringFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, szBiosRevision: *mut types::NvAPI_ShortString) -> NvAPI_Status;

    /// This function returns the full video BIOS version string in the form of xx.xx.xx.xx.yy where
    /// - xx numbers come from NvAPI_GPU_GetVbiosRevision() and
    /// - yy comes from NvAPI_GPU_GetVbiosOEMRevision().
    pub unsafe fn NvAPI_GPU_GetVbiosVersionString;
}

nvapi! {
    pub type GPU_GetPCIIdentifiersFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pDeviceId: *mut u32, pSubSystemId: *mut u32, pRevisionId: *mut u32, pExtDeviceId: *mut u32) -> NvAPI_Status;

    /// This function returns the PCI identifiers associated with this GPU.
    pub unsafe fn NvAPI_GPU_GetPCIIdentifiers;
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
    pub type GPU_GetSystemTypeFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pSystemType: *mut NV_SYSTEM_TYPE) -> NvAPI_Status;

    /// This function identifies whether the GPU is a notebook GPU or a desktop GPU.
    pub unsafe fn NvAPI_GPU_GetSystemType;
}

nvapi! {
    pub type GPU_GetShaderSubPipeCountFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pCount: *mut u32) -> NvAPI_Status;

    /// This function retrieves the number of Shader SubPipes on the GPU
    /// On newer architectures, this corresponds to the number of SM units
    pub unsafe fn NvAPI_GPU_GetShaderSubPipeCount;
}

nvapi! {
    pub type GPU_GetGpuCoreCountFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pCount: *mut u32) -> NvAPI_Status;

    /// Retrieves the total number of cores defined for a GPU.
    /// Returns 0 on architectures that don't define GPU cores.
    pub unsafe fn NvAPI_GPU_GetGpuCoreCount;
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
    pub unsafe fn NvAPI_GPU_GetPerfDecreaseInfo(hPhysicalGpu: NvPhysicalGpuHandle, pPerfDecrInfo: *mut NVAPI_GPU_PERF_DECREASE) -> NvAPI_Status;
}

/// Undocumented API
pub mod private {
    use status::NvAPI_Status;
    use handles::NvPhysicalGpuHandle;
    use types;

    pub const NVAPI_MAX_PROCESSES: usize = 128;

    nvapi! {
        pub type GPU_GetShaderPipeCountFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pCount: *mut u32) -> NvAPI_Status;

        pub unsafe fn NvAPI_GPU_GetShaderPipeCount;
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
        }
    }

    nvenum_display! {
        RamType => _
    }

    nvapi! {
        pub type GPU_GetRamTypeFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pMemType: *mut NV_GPU_RAM_TYPE) -> NvAPI_Status;

        /// Undocumented function.
        pub unsafe fn NvAPI_GPU_GetRamType;
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
        pub type GPU_GetRamMakerFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pRamMaker: *mut NV_GPU_RAM_MAKER) -> NvAPI_Status;

        /// Undocumented function.
        pub unsafe fn NvAPI_GPU_GetRamMaker;
    }

    nvapi! {
        pub type GPU_GetRamBusWidthFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pRamBusWidth: *mut u32) -> NvAPI_Status;

        /// Undocumented function.
        pub unsafe fn NvAPI_GPU_GetRamBusWidth;
    }

    nvapi! {
        pub type GPU_GetRamBankCountFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pRamBankCount: *mut u32) -> NvAPI_Status;

        /// Undocumented function.
        pub unsafe fn NvAPI_GPU_GetRamBankCount;
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
        pub type GPU_GetFoundryFn = extern "C" fn(hPhysicalGPU: NvPhysicalGpuHandle, pFoundry: *mut NV_GPU_FOUNDRY) -> NvAPI_Status;

        /// Undocumented function.
        pub unsafe fn NvAPI_GPU_GetFoundry;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_GetFBWidthAndLocation(hPhysicalGpu: NvPhysicalGpuHandle, pWidth: *mut u32, pLocation: *mut u32) -> NvAPI_Status;
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

    nvapi! {
        pub unsafe fn NvAPI_GetGPUIDFromPhysicalGPU(hPhysicalGpu: NvPhysicalGpuHandle, gpuid: *mut u32) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_GetShortName(hPhysicalGpu: NvPhysicalGpuHandle, pName: *mut types::NvAPI_ShortString) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GPU_GetPartitionCount(hPhysicalGpu: NvPhysicalGpuHandle, pPartitionCount: *mut u32) -> NvAPI_Status;
    }

    nvapi! {
        pub unsafe fn NvAPI_GetDriverModel(hPhysicalGpu: NvPhysicalGpuHandle, pDriverModel: *mut u32) -> NvAPI_Status;
    }
}
