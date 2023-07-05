#![allow(non_upper_case_globals)]

use crate::prelude_::*;

nvstruct! {
    pub struct NV_CHIPSET_INFO_v1 {
        /// structure version
        pub version: NvVersion,
        /// vendor ID
        pub vendorId: u32,
        /// device ID
        pub deviceId: u32,
        /// vendor Name
        pub szVendorName: NvAPI_ShortString,
        /// device Name
        pub szChipsetName: NvAPI_ShortString,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v2 {
        pub v1: NV_CHIPSET_INFO_v1,
        /// Chipset info flags - obsolete
        #[deprecated]
        pub flags: u32,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v3 {
        pub v2: NV_CHIPSET_INFO_v2,
        /// subsystem vendor ID
        pub subSysVendorId: u32,
        /// subsystem device ID
        pub subSysDeviceId: u32,
        /// subsystem vendor Name
        pub szSubSysVendorName: NvAPI_ShortString,
    }
}

nvstruct! {
    pub struct NV_CHIPSET_INFO_v4 {
        pub v3: NV_CHIPSET_INFO_v3,
        /// Host bridge vendor identification
        pub HBvendorId: u32,
        /// Host bridge device identification
        pub HBdeviceId: u32,
        /// Host bridge subsystem vendor identification
        pub HBsubSysVendorId: u32,
        /// Host bridge subsystem device identification
        pub HBsubSysDeviceId: u32,
    }
}

nvversion! { NV_CHIPSET_INFO:
    NV_CHIPSET_INFO_v4(4; @inherit(v3: NV_CHIPSET_INFO_v3)),
    NV_CHIPSET_INFO_v3(3; @inherit(v2: NV_CHIPSET_INFO_v2)),
    NV_CHIPSET_INFO_v2(2; @inherit(v1: NV_CHIPSET_INFO_v1)),
    NV_CHIPSET_INFO_v1(1)
}

nvapi! {
    pub type SYS_GetChipSetInfoFn = extern "C" fn(pChipSetInfo@StructVersionOut: *mut NV_CHIPSET_INFO) -> NvAPI_Status;

    /// This API returns display driver version and driver-branch string.
    pub fn NvAPI_SYS_GetChipSetInfo;
}

nvapi! {
    /// This API converts a Physical GPU handle and output ID to a display ID.
    pub fn NvAPI_SYS_GetDisplayIdFromGpuAndOutputId(hPhysicalGpu: NvPhysicalGpuHandle, outputId: u32, displayId@out: *mut u32) -> NvAPI_Status;
}

nvapi! {
    /// This API converts a display ID to a Physical GPU handle and output ID.
    pub fn NvAPI_SYS_GetGpuAndOutputIdFromDisplayId(displayId: u32, hPhysicalGpu@out: *mut NvPhysicalGpuHandle, outputId: *mut u32) -> NvAPI_Status;
}

nvapi! {
    /// This API retrieves the Physical GPU handle of the connected display
    pub fn NvAPI_SYS_GetPhysicalGpuFromDisplayId(displayId: u32, hPhysicalGpu@out: *mut NvPhysicalGpuHandle) -> NvAPI_Status;
}

nvbits! {
    /// Bitfield in [NV_DISPLAY_DRIVER_INFO_V1]
    pub enum NV_DISPLAY_DRIVER_INFO_FLAGS / DisplayDriverInfoFlags {
        /// Contains the driver DCH status after successful return.
        ///
        /// Value of 1 means that this is DCH driver.
        ///
        /// Value of 0 means that this is not a DCH driver
        /// (NVAPI may be unable to query the DCH status of the driver due to some registry API errors, in that case the API will return with NVAPI_ERROR)
        NV_DISPLAY_DRIVER_INFO_FLAGS_IS_DCH_DRIVER / IsDCHDriver = 0b00000001,
        /// this field provides information about whether the installed driver is from an NVIDIA Studio Driver package.
        ///
        /// Value of 1 means that this driver is from the NVIDIA Studio Driver package.
        NV_DISPLAY_DRIVER_INFO_FLAGS_IS_NVIDIA_STUDIO_PACKAGE / IsNVIDIAStudioPackage = 0b00000010,
        /// this field provides information about whether the installed driver is from an NVIDIA Game Ready Driver package.
        ///
        /// Value of 1 means that this driver is from the NVIDIA Game Ready Driver package.
        NV_DISPLAY_DRIVER_INFO_FLAGS_IS_GAME_READY_PACKAGE / IsNVIDIAGameReadyPackage = 0b00000100,
        /// this field confirms whether the installed driver package is from an NVIDIA RTX Enterprise Production Branch which offers ISV certifications, long life-cycle support, regular security updates, and access to the same functionality as corresponding NVIDIA Studio Driver Packages (i.e., of the same driver version number).
        ///
        /// Value of 1 means that this driver is from the NVIDIA RTX Enterprise Production Branch package.
        NV_DISPLAY_DRIVER_INFO_FLAGS_IS_NVIDIA_RTX_PRODUCTION_BRANCH_PACKAGE / IsNVIDIARTXProductionBranchPackage = 0b00001000,
        /// this field confirms whether the installed driver package is from an NVIDIA RTX New Feature Branch.
        ///
        /// This driver typically gives access to new features, bug fixes, new operating system support, and other driver enhancements offered between NVIDIA RTX Enterprise Production Branch releases. Support duration for NVIDIA RTX New Feature Branches is shorter than that for NVIDIA RTX Enterprise Production Branches.
        ///
        /// Value of 1 means that this driver is from the NVIDIA RTX New Feature Branch package.
        NV_DISPLAY_DRIVER_INFO_FLAGS_IS_NVIDIA_RTX_NEW_FEATURE_BRANCH_PACKAGE / IsNVIDIARTXNewFeatureBranchPackage = 0b00010000,
    }
}

nvstruct! {
    pub struct NV_DISPLAY_DRIVER_INFO_V1 {
        /// Structure Version.
        pub version: NvVersion,
        /// the driver version
        pub driverVersion: u32,
        /// the driver-branch string
        pub szBuildBranch: NvAPI_ShortString,
        pub flags: NV_DISPLAY_DRIVER_INFO_FLAGS,
    }
}

nvstruct! {
    pub struct NV_DISPLAY_DRIVER_INFO_V2 {
        pub v1: NV_DISPLAY_DRIVER_INFO_V1,
        /// the driver base branch string
        pub szBuildBaseBranch: NvAPI_ShortString,
        /// Reserved for future use
        pub reservedEx: u32,
    }
}

nvversion! { NV_DISPLAY_DRIVER_INFO:
    NV_DISPLAY_DRIVER_INFO_V2(2; @inherit(v1: NV_DISPLAY_DRIVER_INFO_V1)),
    NV_DISPLAY_DRIVER_INFO_V1(1)
}

nvapi! {
    /// This API will return information related to the NVIDIA Display Driver.
    ///
    /// Note that out of the driver types - Studio, Game Ready, RTX Production Branch, RTX New Feature Branch -
    /// only one driver type can be available in system.
    ///
    /// If NVAPI is unable to get the information of particular driver type, we report all flags as 0 (Unknown).
    pub fn NvAPI_SYS_GetDisplayDriverInfo(pDriverInfo@StructVersionOut: *mut NV_DISPLAY_DRIVER_INFO) -> NvAPI_Status;
}
