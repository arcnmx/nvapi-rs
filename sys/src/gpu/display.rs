use crate::prelude_::*;

nvenum! {
    pub enum NV_MONITOR_CONN_TYPE / MonitorConnectorType {
        NV_MONITOR_CONN_TYPE_UNINITIALIZED / Uninitialized = 0,
        NV_MONITOR_CONN_TYPE_VGA / Vga = 2,
        NV_MONITOR_CONN_TYPE_COMPONENT / Component = 3,
        NV_MONITOR_CONN_TYPE_SVIDEO / SVideo = 4,
        NV_MONITOR_CONN_TYPE_HDMI / Hdmi = 5,
        NV_MONITOR_CONN_TYPE_DVI / Dvi = 6,
        NV_MONITOR_CONN_TYPE_LVDS / Lvds = 7,
        NV_MONITOR_CONN_TYPE_DP / DisplayPort = 8,
        NV_MONITOR_CONN_TYPE_COMPOSITE / Composite = 9,
        NV_MONITOR_CONN_TYPE_UNKNOWN / Unknown = -1,
    }
}

nvenum_display! {
    MonitorConnectorType => _
}

nvbits! {
    /// Argument to `NvAPI_GPU_GetConnectedDisplayIds`
    pub enum NV_GPU_CONNECTED_IDS_FLAG / ConnectedIdsFlags {
        /// Get uncached connected devices
        NV_GPU_CONNECTED_IDS_FLAG_UNCACHED / UNCACHED = 0x01,
        /// Get devices such that those can be selected in an SLI configuration
        NV_GPU_CONNECTED_IDS_FLAG_SLI / SLI = 0x02,
        /// Get devices such that to reflect the Lid State
        NV_GPU_CONNECTED_IDS_FLAG_LIDSTATE / LID_STATE = 0x04,
        /// Get devices that includes the fake connected monitors
        NV_GPU_CONNECTED_IDS_FLAG_FAKE / FAKE = 0x08,
        /// Excludes devices that are part of the multi stream topology.
        NV_GPU_CONNECTED_IDS_FLAG_EXCLUDE_MST / EXCLUDE_MST = 0x10,
    }
}

nvbits! {
    /// Bitfield in `NV_GPU_DISPLAYIDS`
    pub enum NV_GPU_DISPLAYIDS_FLAGS / DisplayIdsFlags {
        /// This display is part of MST topology and it's a dynamic
        NV_GPU_DISPLAYIDS_FLAGS_DYNAMIC / DYNAMIC = 0x01,
        /// This displayID belongs to a multi stream enabled connector(root node).
        /// Note that when multi stream is enabled and
        /// a single multi stream capable monitor is connected to it, the monitor will share the
        /// display id with the RootNode.
        NV_GPU_DISPLAYIDS_FLAGS_MST_ROOT_NODE / MST_ROOT_NODE = 0x02,
        /// This display is being actively driven
        NV_GPU_DISPLAYIDS_FLAGS_ACTIVE / ACTIVE = 0x04,
        /// This display is the representative display
        NV_GPU_DISPLAYIDS_FLAGS_CLUSTER / CLUSTER = 0x08,
        /// This display is reported to the OS
        NV_GPU_DISPLAYIDS_FLAGS_OS_VISIBLE / OS_VISIBLE = 0x10,
        /// This display is wireless
        NV_GPU_DISPLAYIDS_FLAGS_WIRELESS / WIRELESS = 0x20,
        /// This display is connected
        NV_GPU_DISPLAYIDS_FLAGS_CONNECTED / CONNECTED = 0x40,
        /// Do not use
        NV_GPU_DISPLAYIDS_FLAGS_RESERVED_INTERNAL / RESERVED_INTERNAL = 0x1ff80,
        /// this display is a phycially connected display; Valid only when is
        /// Connected bit is set
        NV_GPU_DISPLAYIDS_FLAGS_PHYSICALLY_CONNECTED / PHYSICALLY_CONNECTED = 0x20000,
        /// must be zero
        NV_GPU_DISPLAYIDS_FLAGS_RESERVED / RESERVED = 0xfffc0000u32,
    }
}

nvstruct! {
    pub struct NV_GPU_DISPLAYIDS {
        pub version: NvVersion,
        /// out: vga, tv, dvi, hdmi and dp. This is reserved for future use and clients should not
        /// rely on this information.
        pub connectorType: NV_MONITOR_CONN_TYPE,
        /// this is a unique identifier for each device
        pub displayId: u32,
        /// if bit is set then this display is part of MST topology and it's a dynamic
        pub flags: NV_GPU_DISPLAYIDS_FLAGS,
    }
}

nvversion! { _:
    NV_GPU_DISPLAYIDS(3),
    NV_GPU_DISPLAYIDS(1; @old)
}

nvapi! {
    pub type GPU_GetConnectedDisplayIds = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pDisplayIds: *mut NV_GPU_DISPLAYIDS, pDisplayIdCount: *mut u32, flags: NV_GPU_CONNECTED_IDS_FLAG) -> NvAPI_Status;

    /// Due to space limitation NvAPI_GPU_GetConnectedOutputs can return maximum 32 devices, but
    /// this is no longer true for DPMST. NvAPI_GPU_GetConnectedDisplayIds will return all
    /// the connected display devices in the form of displayIds for the associated hPhysicalGpu.
    /// This function can accept set of flags to request cached, uncached, sli and lid to get the connected devices.
    /// Default value for flags will be cached.
    ///
    /// # HOW TO USE
    ///
    /// 1. for each PhysicalGpu, make a call to get the number of connected displayId's
    ///    using NvAPI_GPU_GetConnectedDisplayIds by passing the pDisplayIds as NULL
    /// 2. On call success:
    ///    Allocate memory based on pDisplayIdCount then make a call NvAPI_GPU_GetConnectedDisplayIds to populate DisplayIds
    ///
    /// # RETURN STATUS
    ///
    /// - `NVAPI_INVALID_ARGUMENT`: hPhysicalGpu or pDisplayIds or pDisplayIdCount is NULL
    /// - `NVAPI_OK`: *pDisplayIds contains a set of GPU-output identifiers
    /// - `NVAPI_NVIDIA_DEVICE_NOT_FOUND`: no NVIDIA GPU driving a display was found
    /// - `NVAPI_EXPECTED_PHYSICAL_GPU_HANDLE`: hPhysicalGpu was not a physical GPU handle
    pub unsafe fn NvAPI_GPU_GetConnectedDisplayIds;
}

nvapi! {
    pub type GPU_GetAllDisplayIds = extern "C" fn(hPhysicalGPU@self: NvPhysicalGpuHandle, pDisplayIds: *mut NV_GPU_DISPLAYIDS, pDisplayIdCount: *mut u32) -> NvAPI_Status;

    /// This API returns display IDs for all possible outputs on the GPU.
    /// For DPMST connector, it will return display IDs for all the video sinks in the topology.
    ///
    /// # Returns
    ///
    /// - `NVAPI_INSUFFICIENT_BUFFER`: When the input buffer(pDisplayIds) is less than the actual number of display IDs
    pub unsafe fn NvAPI_GPU_GetAllDisplayIds;
}

fn check_display_id_version(display_ids: &mut [NV_GPU_DISPLAYIDS]) -> crate::Result<()> {
    match display_ids {
        [] => Ok(()),
        [display_id, ..] => match display_id.version {
            <NV_GPU_DISPLAYIDS as StructVersion<1>>::NVAPI_VERSION | <NV_GPU_DISPLAYIDS as StructVersion<3>>::NVAPI_VERSION => Ok(()),
            _version => {
                #[cfg(feature = "log")] {
                    log::warn!("incorrectly initialized version for GetConnectedDisplayIds: {:?}", _version);
                }
                Err(NvAPI_Status::IncompatibleStructVersion)
            },
        },
    }
}

impl NvPhysicalGpuHandle {
    pub fn GetConnectedDisplayIds(self, display_ids: &mut [NV_GPU_DISPLAYIDS], flags: NV_GPU_CONNECTED_IDS_FLAG) -> crate::Result<usize> {
        check_display_id_version(display_ids)?;
        let mut count = display_ids.len() as u32;
        unsafe {
            self.NvAPI_GPU_GetConnectedDisplayIds(display_ids.as_mut_ptr(), &mut count, flags)
        }.map(move |()| count as _)
    }

    pub fn GetConnectedDisplayIds3(self, flags: NV_GPU_CONNECTED_IDS_FLAG) -> crate::Result<Vec<NV_GPU_DISPLAYIDS>> {
        let count = self.GetConnectedDisplayIds(&mut [], flags)?;
        let mut data: Vec<NV_GPU_DISPLAYIDS> = vec![StructVersion::<3>::versioned(); count];
        self.GetConnectedDisplayIds(&mut data, flags).map(move |count| {
            data.truncate(count);
            data
        })
    }

    pub fn GetConnectedDisplayIds1(self, flags: NV_GPU_CONNECTED_IDS_FLAG) -> crate::Result<Vec<NV_GPU_DISPLAYIDS>> {
        let count = self.GetConnectedDisplayIds(&mut [], flags)?;
        let mut data: Vec<NV_GPU_DISPLAYIDS> = vec![StructVersion::<1>::versioned(); count];
        self.GetConnectedDisplayIds(&mut data, flags).map(move |count| {
            data.truncate(count);
            data
        })
    }

    pub fn GetAllDisplayIds(self, display_ids: &mut [NV_GPU_DISPLAYIDS]) -> crate::Result<usize> {
        check_display_id_version(display_ids)?;
        let mut count = display_ids.len() as u32;
        unsafe {
            self.NvAPI_GPU_GetAllDisplayIds(display_ids.as_mut_ptr(), &mut count)
        }.map(move |()| count as _)
    }

    pub fn GetAllDisplayIds3(self) -> crate::Result<Vec<NV_GPU_DISPLAYIDS>> {
        let count = self.GetAllDisplayIds(&mut [])?;
        let mut data: Vec<NV_GPU_DISPLAYIDS> = vec![StructVersion::<3>::versioned(); count];
        self.GetAllDisplayIds(&mut data).map(move |count| {
            data.truncate(count);
            data
        })
    }

    pub fn GetAllDisplayIds1(self) -> crate::Result<Vec<NV_GPU_DISPLAYIDS>> {
        let count = self.GetAllDisplayIds(&mut [])?;
        let mut data: Vec<NV_GPU_DISPLAYIDS> = vec![StructVersion::<1>::versioned(); count];
        self.GetAllDisplayIds(&mut data).map(move |count| {
            data.truncate(count);
            data
        })
    }
}
