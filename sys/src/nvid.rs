#![allow(non_camel_case_types)]

use std::mem;

macro_rules! nvapis {
    ($(
        $(#[$($meta:meta)*])*
        $name:ident = $id:expr,
    )*) => {
        #[repr(u32)]
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        pub enum Api {
        $(
            $(#[$($meta)*])*
            $name = $id,
        )*
        }

        impl Api {
            pub fn from_id(id: u32) -> Result<Self, ()> {
                match id {
                $(
                    $id
                )|* => Ok(unsafe { mem::transmute(id) }),
                    _ => Err(()),
                }
            }

            pub fn id(&self) -> u32 {
                *self as _
            }
        }
    };
}

nvapis! {

// source: https://stackoverflow.com/a/16497265 (full dump as of May 2013)

NvAPI_Initialize = 0x0150e828,
NvAPI_Unload = 0xd22bdd7e,
NvAPI_GetErrorMessage = 0x6c2d048c,
NvAPI_GetInterfaceVersionString = 0x01053fa5,
NvAPI_GetDisplayDriverVersion = 0xf951a4d1,
NvAPI_SYS_GetDriverAndBranchVersion = 0x2926aaad,
NvAPI_EnumNvidiaDisplayHandle = 0x9abdd40d,
NvAPI_EnumNvidiaUnAttachedDisplayHandle = 0x20de9260,
NvAPI_EnumPhysicalGPUs = 0xe5ac921f,
NvAPI_EnumTCCPhysicalGPUs = 0xd9930b07,
NvAPI_EnumLogicalGPUs = 0x48b3ea59,
NvAPI_GetPhysicalGPUsFromDisplay = 0x34ef9506,
NvAPI_GetPhysicalGPUFromUnAttachedDisplay = 0x5018ed61,
NvAPI_CreateDisplayFromUnAttachedDisplay = 0x63f9799e,
NvAPI_GetLogicalGPUFromDisplay = 0xee1370cf,
NvAPI_GetLogicalGPUFromPhysicalGPU = 0xadd604d1,
NvAPI_GetPhysicalGPUsFromLogicalGPU = 0xaea3fa32,
NvAPI_GetAssociatedNvidiaDisplayHandle = 0x35c29134,
NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle = 0xa70503b2,
NvAPI_GetAssociatedNvidiaDisplayName = 0x22a78b05,
NvAPI_GetUnAttachedAssociatedDisplayName = 0x4888d790,
NvAPI_EnableHWCursor = 0x2863148d,
NvAPI_DisableHWCursor = 0xab163097,
NvAPI_GetVBlankCounter = 0x67b5db55,
NvAPI_SetRefreshRateOverride = 0x3092ac32,
NvAPI_GetAssociatedDisplayOutputId = 0xd995937e,
NvAPI_GetDisplayPortInfo = 0xc64ff367,
NvAPI_SetDisplayPort = 0xfa13e65a,
NvAPI_GetHDMISupportInfo = 0x6ae16ec3,
NvAPI_DISP_EnumHDMIStereoModes = 0xd2ccf5d6,
NvAPI_GetInfoFrame = 0x09734f1d,
NvAPI_SetInfoFrame = 0x69c6f365,
NvAPI_SetInfoFrameState = 0x67efd887,
NvAPI_GetInfoFrameState = 0x41511594,
NvAPI_Disp_InfoFrameControl = 0x6067af3f,
NvAPI_Disp_ColorControl = 0x92f9d80d,
NvAPI_Disp_GetHdrCapabilities = 0x84f2a8df,
NvAPI_Disp_HdrColorControl = 0x351da224,
NvAPI_DISP_GetVirtualModeData = 0x3230d69a,
NvAPI_DISP_OverrideDisplayModeList = 0x0291bff2,
NvAPI_GetDisplayDriverMemoryInfo = 0x774aa982,
NvAPI_GetDriverMemoryInfo = 0x2dc95125,
NvAPI_GetDVCInfo = 0x4085de45,
NvAPI_SetDVCLevel = 0x172409b4,
NvAPI_GetDVCInfoEx = 0x0e45002d,
NvAPI_SetDVCLevelEx = 0x4a82c2b1,
NvAPI_GetHUEInfo = 0x95b64341,
NvAPI_SetHUEAngle = 0xf5a0f22c,
NvAPI_GetImageSharpeningInfo = 0x9fb063df,
NvAPI_SetImageSharpeningLevel = 0x3fc9a59c,
NvAPI_D3D_GetCurrentSLIState = 0x4b708b54,
NvAPI_D3D9_RegisterResource = 0xa064bdfc,
NvAPI_D3D9_UnregisterResource = 0xbb2b17aa,
NvAPI_D3D9_AliasSurfaceAsTexture = 0xe5ceae41,
NvAPI_D3D9_StretchRectEx = 0x22de03aa,
NvAPI_D3D9_ClearRT = 0x332d3942,
NvAPI_D3D_CreateQuery = 0x5d19bca4,
NvAPI_D3D_DestroyQuery = 0xc8ff7258,
NvAPI_D3D_Query_Begin = 0xe5a9aae0,
NvAPI_D3D_Query_End = 0x2ac084fa,
NvAPI_D3D_Query_GetData = 0xf8b53c69,
NvAPI_D3D_Query_GetDataSize = 0xf2a54796,
NvAPI_D3D_Query_GetType = 0x4aceeaf7,
NvAPI_D3D_RegisterApp = 0xd44d3c4e,
NvAPI_D3D9_CreatePathContextNV = 0xa342f682,
NvAPI_D3D9_DestroyPathContextNV = 0x667c2929,
NvAPI_D3D9_CreatePathNV = 0x71329df3,
NvAPI_D3D9_DeletePathNV = 0x73e0019a,
NvAPI_D3D9_PathVerticesNV = 0xc23df926,
NvAPI_D3D9_PathParameterfNV = 0xf7ff00c1,
NvAPI_D3D9_PathParameteriNV = 0xfc31236c,
NvAPI_D3D9_PathMatrixNV = 0xd2f6c499,
NvAPI_D3D9_PathDepthNV = 0xfcb16330,
NvAPI_D3D9_PathClearDepthNV = 0x157e45c4,
NvAPI_D3D9_PathEnableDepthTestNV = 0xe99ba7f3,
NvAPI_D3D9_PathEnableColorWriteNV = 0x3e2804a2,
NvAPI_D3D9_DrawPathNV = 0x13199b3d,
NvAPI_D3D9_GetSurfaceHandle = 0x0f2dd3f2,
NvAPI_D3D9_GetOverlaySurfaceHandles = 0x6800f5fc,
NvAPI_D3D9_GetTextureHandle = 0xc7985ed5,
NvAPI_D3D9_GpuSyncGetHandleSize = 0x80c9fd3b,
NvAPI_D3D9_GpuSyncInit = 0x6d6fdad4,
NvAPI_D3D9_GpuSyncEnd = 0x754033f0,
NvAPI_D3D9_GpuSyncMapTexBuffer = 0xcde4a28a,
NvAPI_D3D9_GpuSyncMapSurfaceBuffer = 0x2ab714ab,
NvAPI_D3D9_GpuSyncMapVertexBuffer = 0xdbc803ec,
NvAPI_D3D9_GpuSyncMapIndexBuffer = 0x12ee68f2,
NvAPI_D3D9_SetPitchSurfaceCreation = 0x18cdf365,
NvAPI_D3D9_GpuSyncAcquire = 0xd00b8317,
NvAPI_D3D9_GpuSyncRelease = 0x3d7a86bb,
NvAPI_D3D9_GetCurrentRenderTargetHandle = 0x022cad61,
NvAPI_D3D9_GetCurrentZBufferHandle = 0xb380f218,
NvAPI_D3D9_GetIndexBufferHandle = 0xfc5a155b,
NvAPI_D3D9_GetVertexBufferHandle = 0x72b19155,
NvAPI_D3D9_CreateTexture = 0xd5e13573,
NvAPI_D3D9_AliasPrimaryAsTexture = 0x13c7112e,
NvAPI_D3D9_PresentSurfaceToDesktop = 0x0f7029c5,
NvAPI_D3D9_CreateVideoBegin = 0x84c9d553,
NvAPI_D3D9_CreateVideoEnd = 0xb476bf61,
NvAPI_D3D9_CreateVideo = 0x89ffd9a3,
NvAPI_D3D9_FreeVideo = 0x3111bed1,
NvAPI_D3D9_PresentVideo = 0x5cf7f862,
NvAPI_D3D9_VideoSetStereoInfo = 0xb852f4db,
NvAPI_D3D9_SetGamutData = 0x2bbda32e,
NvAPI_D3D9_SetSurfaceCreationLayout = 0x5609b86a,
NvAPI_D3D9_GetVideoCapabilities = 0x3d596b93,
NvAPI_D3D9_QueryVideoInfo = 0x1e6634b3,
NvAPI_D3D9_AliasPrimaryFromDevice = 0x7c20c5be,
NvAPI_D3D9_SetResourceHint = 0x905f5c27,
NvAPI_D3D9_Lock = 0x6317345c,
NvAPI_D3D9_Unlock = 0xc182027e,
NvAPI_D3D9_GetVideoState = 0xa4527bf8,
NvAPI_D3D9_SetVideoState = 0xbd4bc56f,
NvAPI_D3D9_EnumVideoFeatures = 0x1db7c52c,
NvAPI_D3D9_GetSLIInfo = 0x694bff4d,
NvAPI_D3D9_SetSLIMode = 0xbfdc062c,
NvAPI_D3D9_QueryAAOverrideMode = 0xddf5643c,
NvAPI_D3D9_VideoSurfaceEncryptionControl = 0x9d2509ef,
NvAPI_D3D9_DMA = 0x962b8af6,
NvAPI_D3D9_EnableStereo = 0x492a6954,
NvAPI_D3D9_StretchRect = 0xaeaecd41,
NvAPI_D3D9_CreateRenderTarget = 0x0b3827c8,
NvAPI_D3D9_NVFBC_GetStatus = 0xbd3eb475,
NvAPI_D3D9_IFR_SetUpTargetBufferToSys = 0x55255d05,
NvAPI_D3D9_GPUBasedCPUSleep = 0xd504dda7,
NvAPI_D3D9_IFR_TransferRenderTarget = 0x0ab7c2dc,
NvAPI_D3D9_IFR_SetUpTargetBufferToNV12BLVideoSurface = 0xcfc92c15,
NvAPI_D3D9_IFR_TransferRenderTargetToNV12BLVideoSurface = 0x5fe72f64,
NvAPI_D3D10_AliasPrimaryAsTexture = 0x8aac133d,
NvAPI_D3D10_SetPrimaryFlipChainCallbacks = 0x73eb9329,
NvAPI_D3D10_ProcessCallbacks = 0xae9c2019,
NvAPI_D3D10_GetRenderedCursorAsBitmap = 0xcac3ce5d,
NvAPI_D3D10_BeginShareResource = 0x35233210,
NvAPI_D3D10_BeginShareResourceEx = 0xef303a9d,
NvAPI_D3D10_EndShareResource = 0x0e9c5853,
NvAPI_D3D10_SetDepthBoundsTest = 0x4eadf5d2,
NvAPI_D3D10_CreateDevice = 0x2de11d61,
NvAPI_D3D10_CreateDeviceAndSwapChain = 0x5b803daf,
NvAPI_D3D11_CreateDevice = 0x6a16d3a0,
NvAPI_D3D11_CreateDeviceAndSwapChain = 0xbb939ee5,
NvAPI_D3D11_BeginShareResource = 0x0121bdc6,
NvAPI_D3D11_EndShareResource = 0x8ffb8e26,
NvAPI_D3D11_SetDepthBoundsTest = 0x7aaf7a04,
NvAPI_D3D11_IsNvShaderExtnOpCodeSupported = 0x5f68da40,
NvAPI_D3D11_SetNvShaderExtnSlot = 0x8e90bb9f,
NvAPI_D3D12_SetNvShaderExtnSlotSpace = 0xac2dfeb5,
NvAPI_D3D12_SetNvShaderExtnSlotSpaceLocalThread = 0x43d867c0,
NvAPI_D3D11_SetNvShaderExtnSlotLocalThread = 0x0e6482a0,
NvAPI_D3D11_BeginUAVOverlapEx = 0xba08208a,
NvAPI_D3D11_BeginUAVOverlap = 0x65b93ca8,
NvAPI_D3D11_EndUAVOverlap = 0x2216a357,
NvAPI_D3D11_GetResourceHandle = 0x09d52986,
NvAPI_GPU_GetShaderPipeCount = 0x63e2f56f,
NvAPI_GPU_GetShaderSubPipeCount = 0x0be17923,
NvAPI_GPU_GetPartitionCount = 0x86f05d7a,
NvAPI_GPU_GetMemPartitionMask = 0x329d77cd,
NvAPI_GPU_GetTPCMask = 0x4a35df54,
NvAPI_GPU_GetSMMask = 0xeb7af173,
NvAPI_GPU_GetTotalTPCCount = 0x4e2f76a8,
NvAPI_GPU_GetTotalSMCount = 0xae5fbcfe,
NvAPI_GPU_GetTotalSPCount = 0xb6d62591,
NvAPI_GPU_GetGpuCoreCount = 0xc7026a87,
NvAPI_GPU_GetAllOutputs = 0x7d554f8e,
NvAPI_GPU_GetConnectedOutputs = 0x1730bfc9,
NvAPI_GPU_GetConnectedSLIOutputs = 0x0680de09,
NvAPI_GPU_GetConnectedDisplayIds = 0x0078dba2,
NvAPI_GPU_GetAllDisplayIds = 0x785210a2,
NvAPI_GPU_GetConnectedOutputsWithLidState = 0xcf8caf39,
NvAPI_GPU_GetConnectedSLIOutputsWithLidState = 0x96043cc7,
NvAPI_GPU_GetSystemType = 0xbaaabfcc,
NvAPI_GPU_GetActiveOutputs = 0xe3e89b6f,
NvAPI_GPU_GetEDID = 0x37d32e69,
NvAPI_GPU_SetEDID = 0xe83d6456,
NvAPI_GPU_GetOutputType = 0x40a505e4,
NvAPI_GPU_GetDeviceDisplayMode = 0xd2277e3a,
NvAPI_GPU_GetFlatPanelInfo = 0x36cff969,
NvAPI_GPU_ValidateOutputCombination = 0x34c9c2d4,
NvAPI_GPU_GetConnectorInfo = 0x4eca2c10,
NvAPI_GPU_GetFullName = 0xceee8e9f,
NvAPI_GPU_GetPCIIdentifiers = 0x2ddfb66e,
NvAPI_GPU_GetGPUType = 0xc33baeb1,
NvAPI_GPU_GetBusType = 0x1bb18724,
NvAPI_GPU_GetBusId = 0x1be0b8e5,
NvAPI_GPU_GetBusSlotId = 0x2a0a350f,
NvAPI_GPU_GetIRQ = 0xe4715417,
NvAPI_GPU_GetVbiosRevision = 0xacc3da0a,
NvAPI_GPU_GetVbiosOEMRevision = 0x2d43fb31,
NvAPI_GPU_GetVbiosVersionString = 0xa561fd7d,
NvAPI_GPU_GetAGPAperture = 0x6e042794,
NvAPI_GPU_GetCurrentAGPRate = 0xc74925a0,
NvAPI_GPU_GetCurrentPCIEDownstreamWidth = 0xd048c3b1,
NvAPI_GPU_GetPhysicalFrameBufferSize = 0x46fbeb03,
NvAPI_GPU_GetVirtualFrameBufferSize = 0x5a04b644,
NvAPI_GPU_GetQuadroStatus = 0xe332fa47,
NvAPI_GPU_GetBoardInfo = 0x22d54523,
NvAPI_GPU_GetRamType = 0x57f7caac,
NvAPI_GPU_GetFBWidthAndLocation = 0x11104158,
NvAPI_GPU_GetAllClockFrequencies = 0xdcb616c3,
NvAPI_GPU_GetPerfClocks = 0x1ea54a3b,
NvAPI_GPU_SetPerfClocks = 0x07bcf4ac,
NvAPI_GPU_GetCoolerSettings = 0xda141340,
NvAPI_GPU_SetCoolerLevels = 0x891fa0ae,
NvAPI_GPU_RestoreCoolerSettings = 0x8f6ed0fb,
NvAPI_GPU_GetCoolerPolicyTable = 0x0518a32c,
NvAPI_GPU_SetCoolerPolicyTable = 0x987947cd,
NvAPI_GPU_RestoreCoolerPolicyTable = 0xd8c4fe63,
NvAPI_GPU_GetPstatesInfo = 0xba94c56e,
NvAPI_GPU_GetPstatesInfoEx = 0x843c0256,
NvAPI_GPU_SetPstatesInfo = 0xcdf27911,
NvAPI_GPU_GetPstates20 = 0x6ff81213,
NvAPI_GPU_SetPstates20 = 0x0f4dae6b,
NvAPI_GPU_GetCurrentPstate = 0x927da4f6,
NvAPI_GPU_GetPstateClientLimits = 0x88c82104,
NvAPI_GPU_SetPstateClientLimits = 0xfdfc7d49,
NvAPI_GPU_EnableOverclockedPstates = 0xb23b70ee,
NvAPI_GPU_EnableDynamicPstates = 0xfa579a0f,
NvAPI_GPU_GetDynamicPstatesInfoEx = 0x60ded2ed,
NvAPI_GPU_GetVoltages = 0x7d656244,
NvAPI_GPU_GetThermalSettings = 0xe3640a56,
NvAPI_GPU_SetDitherControl = 0xdf0dfcdd,
NvAPI_GPU_GetDitherControl = 0x932ac8fb,
NvAPI_GPU_GetColorSpaceConversion = 0x8159e87a,
NvAPI_GPU_SetColorSpaceConversion = 0xfcabd23a,
NvAPI_GetTVOutputInfo = 0x30c805d5,
NvAPI_GetTVEncoderControls = 0x5757474a,
NvAPI_SetTVEncoderControls = 0xca36a3ab,
NvAPI_GetTVOutputBorderColor = 0x6dfd1c8c,
NvAPI_SetTVOutputBorderColor = 0xaed02700,
NvAPI_GetDisplayPosition = 0x6bb1ee5d,
NvAPI_SetDisplayPosition = 0x57d9060f,
NvAPI_GetValidGpuTopologies = 0x5dfab48a,
NvAPI_GetInvalidGpuTopologies = 0x15658be6,
NvAPI_SetGpuTopologies = 0x25201f3d,
NvAPI_GPU_GetPerGpuTopologyStatus = 0xa81f8992,
NvAPI_SYS_GetChipSetTopologyStatus = 0x8a50f126,
NvAPI_GPU_Get_DisplayPort_DongleInfo = 0x76a70e8d,
NvAPI_I2CRead = 0x2fde12c5,
NvAPI_I2CWrite = 0xe812eb07,
NvAPI_I2CWriteEx = 0x283ac65a,
NvAPI_I2CReadEx = 0x4d7b0709,
NvAPI_GPU_GetPowerMizerInfo = 0x76bfa16b,
NvAPI_GPU_SetPowerMizerInfo = 0x50016c78,
NvAPI_GPU_GetVoltageDomainsStatus = 0xc16c7e2c,
NvAPI_GPU_ClientPowerTopologyGetInfo = 0xa4dfd3f2,
NvAPI_GPU_ClientPowerTopologyGetStatus = 0xedcf624e,
NvAPI_GPU_ClientPowerPoliciesGetInfo = 0x34206d86,
NvAPI_GPU_ClientPowerPoliciesGetStatus = 0x70916171,
NvAPI_GPU_ClientPowerPoliciesSetStatus = 0xad95f5ed,
NvAPI_GPU_WorkstationFeatureSetup = 0x6c1f3fe4,
NvAPI_GPU_WorkstationFeatureQuery = 0x004537df,
NvAPI_GPU_QueryWorkstationFeatureSupport = 0x80b1abb9,
NvAPI_SYS_GetChipSetInfo = 0x53dabbca,
NvAPI_SYS_GetLidAndDockInfo = 0xcda14d8a,
NvAPI_OGL_ExpertModeSet = 0x3805ef7a,
NvAPI_OGL_ExpertModeGet = 0x22ed9516,
NvAPI_OGL_ExpertModeDefaultsSet = 0xb47a657e,
NvAPI_OGL_ExpertModeDefaultsGet = 0xae921f12,
NvAPI_SetDisplaySettings = 0xe04f3d86,
NvAPI_GetDisplaySettings = 0xdc27d5d4,
NvAPI_GetTiming = 0xafc4833e,
NvAPI_DISP_GetTiming = 0x175167e9,
NvAPI_DISP_GetMonitorCapabilities = 0x3b05c7e1,
NvAPI_DISP_GetMonitorColorCapabilities = 0x6ae4cfb5,
NvAPI_DISP_EnumCustomDisplay = 0xa2072d59,
NvAPI_DISP_TryCustomDisplay = 0x1f7db630,
NvAPI_DISP_DeleteCustomDisplay = 0x552e5b9b,
NvAPI_DISP_SaveCustomDisplay = 0x49882876,
NvAPI_DISP_RevertCustomDisplayTrial = 0xcbbd40f0,
NvAPI_EnumCustomDisplay = 0x42892957,
NvAPI_TryCustomDisplay = 0xbf6c1762,
NvAPI_RevertCustomDisplayTrial = 0x854ba405,
NvAPI_DeleteCustomDisplay = 0xe7cb998d,
NvAPI_SaveCustomDisplay = 0xa9062c78,
NvAPI_QueryUnderscanCap = 0x61d7b624,
NvAPI_EnumUnderscanConfig = 0x4144111a,
NvAPI_DeleteUnderscanConfig = 0xf98854c8,
NvAPI_SetUnderscanConfig = 0x3efada1d,
NvAPI_GetDisplayFeatureConfig = 0x8e985ccd,
NvAPI_SetDisplayFeatureConfig = 0xf36a668d,
NvAPI_GetDisplayFeatureConfigDefaults = 0x0f5f4d01,
NvAPI_SetView = 0x0957d7b6,
NvAPI_GetView = 0xd6b99d89,
NvAPI_SetViewEx = 0x06b89e68,
NvAPI_GetViewEx = 0xdbbc0af4,
NvAPI_GetSupportedViews = 0x66fb7fc0,
NvAPI_GetHDCPLinkParameters = 0xb3bb0772,
NvAPI_Disp_DpAuxChannelControl = 0x8eb56969,
NvAPI_SetHybridMode = 0xfb22d656,
NvAPI_GetHybridMode = 0xe23b68c1,
NvAPI_Coproc_GetCoprocStatus = 0x1efc3957,
NvAPI_Coproc_SetCoprocInfoFlagsEx = 0xf4c863ac,
NvAPI_Coproc_GetCoprocInfoFlagsEx = 0x69a9874d,
NvAPI_Coproc_NotifyCoprocPowerState = 0xcadcb956,
NvAPI_Coproc_GetApplicationCoprocInfo = 0x79232685,
NvAPI_GetVideoState = 0x1c5659cd,
NvAPI_SetVideoState = 0x054fe75a,
NvAPI_SetFrameRateNotify = 0x18919887,
NvAPI_SetPVExtName = 0x4feeb498,
NvAPI_GetPVExtName = 0x2f5b08e0,
NvAPI_SetPVExtProfile = 0x8354a8f4,
NvAPI_GetPVExtProfile = 0x1b1b9a16,
NvAPI_VideoSetStereoInfo = 0x97063269,
NvAPI_VideoGetStereoInfo = 0x8e1f8cfe,
NvAPI_Mosaic_GetSupportedTopoInfo = 0xfdb63c81,
NvAPI_Mosaic_GetTopoGroup = 0xcb89381d,
NvAPI_Mosaic_GetOverlapLimits = 0x989685f0,
NvAPI_Mosaic_SetCurrentTopo = 0x9b542831,
NvAPI_Mosaic_GetCurrentTopo = 0xec32944e,
NvAPI_Mosaic_EnableCurrentTopo = 0x5f1aa66c,
NvAPI_Mosaic_SetGridTopology = 0x3f113c77,
NvAPI_Mosaic_GetMosaicCapabilities = 0xda97071e,
NvAPI_Mosaic_GetDisplayCapabilities = 0xd58026b9,
NvAPI_Mosaic_EnumGridTopologies = 0xa3c55220,
NvAPI_Mosaic_GetDisplayViewportsByResolution = 0xdc6dc8d3,
NvAPI_Mosaic_GetMosaicViewports = 0x07eba036,
NvAPI_Mosaic_SetDisplayGrids = 0x4d959a89,
NvAPI_Mosaic_ValidateDisplayGridsWithSLI = 0x1ecfd263,
NvAPI_Mosaic_ValidateDisplayGrids = 0xcf43903d,
NvAPI_Mosaic_EnumDisplayModes = 0x78db97d7,
NvAPI_Mosaic_ChooseGpuTopologies = 0xb033b140,
NvAPI_Mosaic_EnumDisplayGrids = 0xdf2887af,
NvAPI_GetSupportedMosaicTopologies = 0x410b5c25,
NvAPI_GetCurrentMosaicTopology = 0xf60852bd,
NvAPI_SetCurrentMosaicTopology = 0xd54b8989,
NvAPI_EnableCurrentMosaicTopology = 0x74073cc9,
NvAPI_GSync_EnumSyncDevices = 0xd9639601,
NvAPI_GSync_QueryCapabilities = 0x44a3f1d1,
NvAPI_GSync_GetTopology = 0x4562bc38,
NvAPI_GSync_SetSyncStateSettings = 0x60acdfdd,
NvAPI_GSync_GetControlParameters = 0x16de1c6a,
NvAPI_GSync_SetControlParameters = 0x8bbff88b,
NvAPI_GSync_AdjustSyncDelay = 0x2d11ff51,
NvAPI_GSync_GetSyncStatus = 0xf1f5b434,
NvAPI_GSync_GetStatusParameters = 0x70d404ec,
NvAPI_QueryNonMigratableApps = 0xbb9ef1c3,
NvAPI_GPU_QueryActiveApps = 0x65b1c5f5,
NvAPI_Hybrid_QueryUnblockedNonMigratableApps = 0x5f35bcb5,
NvAPI_Hybrid_QueryBlockedMigratableApps = 0xf4c2f8cc,
NvAPI_Hybrid_SetAppMigrationState = 0xfa0b9a59,
NvAPI_Hybrid_IsAppMigrationStateChangeable = 0x584cb0b6,
NvAPI_GPU_GPIOQueryLegalPins = 0xfab69565,
NvAPI_GPU_GPIOReadFromPin = 0xf5e10439,
NvAPI_GPU_GPIOWriteToPin = 0xf3b11e68,
NvAPI_GPU_GetHDCPSupportStatus = 0xf089eef5,
NvAPI_SetTopologyFocusDisplayAndView = 0x0a8064f9,
NvAPI_Stereo_CreateConfigurationProfileRegistryKey = 0xbe7692ec,
NvAPI_Stereo_DeleteConfigurationProfileRegistryKey = 0xf117b834,
NvAPI_Stereo_SetConfigurationProfileValue = 0x24409f48,
NvAPI_Stereo_DeleteConfigurationProfileValue = 0x49bceecf,
NvAPI_Stereo_Enable = 0x239c4545,
NvAPI_Stereo_Disable = 0x2ec50c2b,
NvAPI_Stereo_IsEnabled = 0x348ff8e1,
NvAPI_Stereo_GetStereoCaps = 0xdfc063b7,
NvAPI_Stereo_GetStereoSupport = 0x296c434d,
NvAPI_Stereo_CreateHandleFromIUnknown = 0xac7e37f4,
NvAPI_Stereo_DestroyHandle = 0x3a153134,
NvAPI_Stereo_Activate = 0xf6a1ad68,
NvAPI_Stereo_Deactivate = 0x2d68de96,
NvAPI_Stereo_IsActivated = 0x1fb0bc30,
NvAPI_Stereo_GetSeparation = 0x451f2134,
NvAPI_Stereo_SetSeparation = 0x5c069fa3,
NvAPI_Stereo_DecreaseSeparation = 0xda044458,
NvAPI_Stereo_IncreaseSeparation = 0xc9a8ecec,
NvAPI_Stereo_GetConvergence = 0x4ab00934,
NvAPI_Stereo_SetConvergence = 0x3dd6b54b,
NvAPI_Stereo_DecreaseConvergence = 0x4c87e317,
NvAPI_Stereo_IncreaseConvergence = 0xa17daabe,
NvAPI_Stereo_GetFrustumAdjustMode = 0xe6839b43,
NvAPI_Stereo_SetFrustumAdjustMode = 0x7be27fa2,
NvAPI_Stereo_CaptureJpegImage = 0x932cb140,
NvAPI_Stereo_InitActivation = 0xc7177702,
NvAPI_Stereo_Trigger_Activation = 0x0d6c6cd2,
NvAPI_Stereo_CapturePngImage = 0x8b7e99b5,
NvAPI_Stereo_ReverseStereoBlitControl = 0x3cd58f89,
NvAPI_Stereo_SetNotificationMessage = 0x6b9b409e,
NvAPI_Stereo_SetActiveEye = 0x96eea9f8,
NvAPI_Stereo_SetDriverMode = 0x5e8f0bec,
NvAPI_Stereo_GetEyeSeparation = 0xce653127,
NvAPI_Stereo_IsWindowedModeSupported = 0x40c8ed5e,
NvAPI_Stereo_AppHandShake = 0x8c610bda,
NvAPI_Stereo_HandShake_Trigger_Activation = 0xb30cd1a7,
NvAPI_Stereo_HandShake_Message_Control = 0x315e0ef0,
NvAPI_Stereo_SetSurfaceCreationMode = 0xf5dcfcba,
NvAPI_Stereo_GetSurfaceCreationMode = 0x36f1c736,
NvAPI_Stereo_Debug_WasLastDrawStereoized = 0xed4416c5,
NvAPI_Stereo_ForceToScreenDepth = 0x2d495758,
NvAPI_Stereo_SetVertexShaderConstantF = 0x416c07b3,
NvAPI_Stereo_SetVertexShaderConstantB = 0x5268716f,
NvAPI_Stereo_SetVertexShaderConstantI = 0x7923ba0e,
NvAPI_Stereo_GetVertexShaderConstantF = 0x622fdc87,
NvAPI_Stereo_GetVertexShaderConstantB = 0x712baa5b,
NvAPI_Stereo_GetVertexShaderConstantI = 0x5a60613a,
NvAPI_Stereo_SetPixelShaderConstantF = 0xa9657f32,
NvAPI_Stereo_SetPixelShaderConstantB = 0xba6109ee,
NvAPI_Stereo_SetPixelShaderConstantI = 0x912ac28f,
NvAPI_Stereo_GetPixelShaderConstantF = 0xd4974572,
NvAPI_Stereo_GetPixelShaderConstantB = 0xc79333ae,
NvAPI_Stereo_GetPixelShaderConstantI = 0xecd8f8cf,
NvAPI_Stereo_SetDefaultProfile = 0x44f0ecd1,
NvAPI_Stereo_GetDefaultProfile = 0x624e21c2,
NvAPI_Stereo_Is3DCursorSupported = 0xd7c9ec09,
NvAPI_Stereo_GetCursorSeparation = 0x72162b35,
NvAPI_Stereo_SetCursorSeparation = 0xfbc08fc1,
NvAPI_VIO_GetCapabilities = 0x1dc91303,
NvAPI_VIO_Open = 0x44ee4841,
NvAPI_VIO_Close = 0xd01bd237,
NvAPI_VIO_Status = 0x0e6ce4f1,
NvAPI_VIO_SyncFormatDetect = 0x118d48a3,
NvAPI_VIO_GetConfig = 0xd34a789b,
NvAPI_VIO_SetConfig = 0x0e4eec07,
NvAPI_VIO_SetCSC = 0xa1ec8d74,
NvAPI_VIO_GetCSC = 0x7b0d72a3,
NvAPI_VIO_SetGamma = 0x964bf452,
NvAPI_VIO_GetGamma = 0x51d53d06,
NvAPI_VIO_SetSyncDelay = 0x2697a8d1,
NvAPI_VIO_GetSyncDelay = 0x462214a9,
NvAPI_VIO_GetPCIInfo = 0xb981d935,
NvAPI_VIO_IsRunning = 0x96bd040e,
NvAPI_VIO_Start = 0xcde8e1a3,
NvAPI_VIO_Stop = 0x6ba2a5d6,
NvAPI_VIO_IsFrameLockModeCompatible = 0x7bf0a94d,
NvAPI_VIO_EnumDevices = 0xfd7c5557,
NvAPI_VIO_QueryTopology = 0x869534e2,
NvAPI_VIO_EnumSignalFormats = 0xead72fe4,
NvAPI_VIO_EnumDataFormats = 0x221fa8e8,
NvAPI_GPU_GetTachReading = 0x5f608315,
NvAPI_3D_GetProperty = 0x8061a4b1,
NvAPI_3D_SetProperty = 0xc9175e8d,
NvAPI_3D_GetPropertyRange = 0xb85de27c,
NvAPI_GPS_GetPowerSteeringStatus = 0x540ee82e,
NvAPI_GPS_SetPowerSteeringStatus = 0x9723d3a2,
NvAPI_GPS_SetVPStateCap = 0x68888eb4,
NvAPI_GPS_GetVPStateCap = 0x71913023,
NvAPI_GPS_GetThermalLimit = 0x583113ed,
NvAPI_GPS_SetThermalLimit = 0xc07e210f,
NvAPI_GPS_GetPerfSensors = 0x271c1109,
NvAPI_SYS_GetDisplayIdFromGpuAndOutputId = 0x08f2bab4,
NvAPI_SYS_GetGpuAndOutputIdFromDisplayId = 0x112ba1a5,
NvAPI_GPU_ClientRegisterForUtilizationSampleUpdates = 0xadeeaf67,
NvAPI_SYS_GetDisplayDriverInfo = 0x721faceb,
NvAPI_SYS_GetPhysicalGpuFromDisplayId = 0x9ea74659,
NvAPI_DISP_GetDisplayIdByDisplayName = 0xae457190,
NvAPI_DISP_GetGDIPrimaryDisplayId = 0x1e9d8a31,
NvAPI_DISP_GetDisplayConfig = 0x11abccf8,
NvAPI_DISP_SetDisplayConfig = 0x5d8cf8de,
NvAPI_DISP_GetAdaptiveSyncData = 0xb73d1ee9,
NvAPI_DISP_SetAdaptiveSyncData = 0x3eebba1d,
NvAPI_DISP_GetVirtualRefreshRateData = 0x8c00429a,
NvAPI_DISP_SetVirtualRefreshRateData = 0x5abbe6a3,
NvAPI_DISP_SetPreferredStereoDisplay = 0xc9d0e25f,
NvAPI_DISP_GetPreferredStereoDisplay = 0x1f6b4666,
NvAPI_DISP_GetNvManagedDedicatedDisplays = 0xdbdf0cb2,
NvAPI_DISP_AcquireDedicatedDisplay = 0x47c917ba,
NvAPI_DISP_ReleaseDedicatedDisplay = 0x1247825f,
NvAPI_GPU_GetPixelClockRange = 0x66af10b7,
NvAPI_GPU_SetPixelClockRange = 0x5ac7f8e5,
NvAPI_GPU_GetECCStatusInfo = 0xca1ddaf3,
NvAPI_GPU_GetECCErrorInfo = 0xc71f85a6,
NvAPI_GPU_ResetECCErrorInfo = 0xc02eec20,
NvAPI_GPU_GetECCConfigurationInfo = 0x77a796f3,
NvAPI_GPU_SetECCConfiguration = 0x1cf639d9,
NvAPI_D3D1x_CreateSwapChain = 0x1bc21b66,
NvAPI_D3D9_CreateSwapChain = 0x1a131e09,
NvAPI_D3D_SetFPSIndicatorState = 0xa776e8db,
NvAPI_D3D9_Present = 0x05650beb,
NvAPI_D3D9_QueryFrameCount = 0x9083e53a,
NvAPI_D3D9_ResetFrameCount = 0xfa6a0675,
NvAPI_D3D9_QueryMaxSwapGroup = 0x5995410d,
NvAPI_D3D9_QuerySwapGroup = 0xeba4d232,
NvAPI_D3D9_JoinSwapGroup = 0x7d44bb54,
NvAPI_D3D9_BindSwapBarrier = 0x9c39c246,
NvAPI_D3D1x_Present = 0x03b845a1,
NvAPI_D3D1x_QueryFrameCount = 0x9152e055,
NvAPI_D3D1x_ResetFrameCount = 0xfbbb031a,
NvAPI_D3D1x_QueryMaxSwapGroup = 0x9bb9d68f,
NvAPI_D3D1x_QuerySwapGroup = 0x407f67aa,
NvAPI_D3D1x_JoinSwapGroup = 0x14610cd7,
NvAPI_D3D1x_BindSwapBarrier = 0x9de8c729,
NvAPI_SYS_VenturaGetState = 0xcb7c208d,
NvAPI_SYS_VenturaSetState = 0x0ce2e9d9,
NvAPI_SYS_VenturaGetCoolingBudget = 0xc9d86e33,
NvAPI_SYS_VenturaSetCoolingBudget = 0x85ff5a15,
NvAPI_SYS_VenturaGetPowerReading = 0x63685979,
NvAPI_DISP_GetDisplayBlankingState = 0x63e5d8db,
NvAPI_DISP_SetDisplayBlankingState = 0x1e17e29b,
NvAPI_DRS_CreateSession = 0x0694d52e,
NvAPI_DRS_DestroySession = 0xdad9cff8,
NvAPI_DRS_LoadSettings = 0x375dbd6b,
NvAPI_DRS_SaveSettings = 0xfcbc7e14,
NvAPI_DRS_LoadSettingsFromFile = 0xd3ede889,
NvAPI_DRS_SaveSettingsToFile = 0x2be25df8,
NvAPI_DRS_CreateProfile = 0xcc176068,
NvAPI_DRS_DeleteProfile = 0x17093206,
NvAPI_DRS_SetCurrentGlobalProfile = 0x1c89c5df,
NvAPI_DRS_GetCurrentGlobalProfile = 0x617bff9f,
NvAPI_DRS_GetProfileInfo = 0x61cd6fd6,
NvAPI_DRS_SetProfileInfo = 0x16abd3a9,
NvAPI_DRS_FindProfileByName = 0x7e4a9a0b,
NvAPI_DRS_EnumProfiles = 0xbc371ee0,
NvAPI_DRS_GetNumProfiles = 0x1dae4fbc,
NvAPI_DRS_CreateApplication = 0x4347a9de,
NvAPI_DRS_DeleteApplicationEx = 0xc5ea85a1,
NvAPI_DRS_DeleteApplication = 0x2c694bc6,
NvAPI_DRS_GetApplicationInfo = 0xed1f8c69,
NvAPI_DRS_EnumApplications = 0x7fa2173a,
NvAPI_DRS_FindApplicationByName = 0xeee566b2,
NvAPI_DRS_SetSetting = 0x577dd202,
NvAPI_DRS_GetSetting = 0x73bf8338,
NvAPI_DRS_EnumSettings = 0xae3039da,
NvAPI_DRS_EnumAvailableSettingIds = 0xf020614a,
NvAPI_DRS_EnumAvailableSettingValues = 0x2ec39f90,
NvAPI_DRS_GetSettingIdFromName = 0xcb7309cd,
NvAPI_DRS_GetSettingNameFromId = 0xd61cbe6e,
NvAPI_DRS_DeleteProfileSetting = 0xe4a26362,
NvAPI_DRS_RestoreAllDefaults = 0x5927b094,
NvAPI_DRS_RestoreProfileDefault = 0xfa5f6134,
NvAPI_DRS_RestoreProfileDefaultSetting = 0x53f0381e,
NvAPI_DRS_GetBaseProfile = 0xda8466a0,
NvAPI_Event_RegisterCallback = 0xe6dbea69,
NvAPI_Event_UnregisterCallback = 0xde1f9b45,
NvAPI_GPU_GetCurrentThermalLevel = 0xd2488b79,
NvAPI_GPU_GetCurrentFanSpeedLevel = 0xbd71f0c9,
NvAPI_GPU_SetScanoutIntensity = 0xa57457a4,
NvAPI_GPU_GetScanoutIntensityState = 0xe81ce836,
NvAPI_GPU_SetScanoutWarping = 0xb34bab4f,
NvAPI_GPU_GetScanoutWarpingState = 0x6f5435af,
NvAPI_GPU_SetScanoutCompositionParameter = 0xf898247d,
NvAPI_GPU_GetScanoutCompositionParameter = 0x58fe51e6,
NvAPI_GPU_GetScanoutConfiguration = 0x6a9f5b63,
NvAPI_GPU_GetScanoutConfigurationEx = 0xe2e1e6f0,
NvAPI_DISP_SetHCloneTopology = 0x61041c24,
NvAPI_DISP_GetHCloneTopology = 0x47bad137,
NvAPI_DISP_ValidateHCloneTopology = 0x5f4c2664,
NvAPI_GPU_GetAdapterIdFromPhysicalGpu = 0x0ff07fde,
NvAPI_GPU_GetVirtualizationInfo = 0x44e022a9,
NvAPI_GPU_GetLogicalGpuInfo = 0x842b066e,
NvAPI_GPU_GetLicensableFeatures = 0x3fc596aa,
NvAPI_GPU_GetVRReadyData = 0x81d629c5,
NvAPI_GPU_GetPerfDecreaseInfo = 0x7f7f4600,
NvAPI_GPU_QueryIlluminationSupport = 0xa629da31,
NvAPI_GPU_GetIllumination = 0x9a1b9365,
NvAPI_GPU_SetIllumination = 0x0254a187,
NvAPI_D3D1x_IFR_SetUpTargetBufferToSys = 0x473f7828,
NvAPI_D3D1x_IFR_TransferRenderTarget = 0x9fbae4eb,

// source: https://github.com/Kaldaien/BMT/blob/master/BMT/dxgi.cpp

NvAPI_GetPhysicalGPUFromDisplay = 0x1890e8da,
NvAPI_GetPhysicalGPUFromGPUID = 0x5380ad1a,
NvAPI_GetGPUIDfromPhysicalGPU = 0x6533ea3e,

NvAPI_GetInfoFrameStatePvt = 0x7fc17574,
NvAPI_GPU_GetMemoryInfo = 0x07f9b368,
NvAPI_GPU_GetMemoryInfoEx = 0xc0599498,

NvAPI_LoadMicrocode = 0x3119f36e,
NvAPI_GetLoadedMicrocodePrograms = 0x919b3136,
NvAPI_GetDisplayDriverBuildTitle = 0x7562e947,
NvAPI_GetDisplayDriverCompileType = 0x988aea78,
NvAPI_GetDisplayDriverSecurityLevel = 0x9d772bba,
NvAPI_AccessDisplayDriverRegistry = 0xf5579360,
NvAPI_GetDisplayDriverRegistryPath = 0x0e24ceee,
NvAPI_GetUnAttachedDisplayDriverRegistryPath = 0x633252d8,
NvAPI_GPU_GetRawFuseData = 0xe0b1dce9,
NvAPI_GPU_GetFoundry = 0x5d857a00,
NvAPI_GPU_GetVPECount = 0xd8cbf37b,

NvAPI_GPU_GetTargetID = 0x35b5fd2f,

NvAPI_GPU_GetShortName = 0xd988f0f3,

NvAPI_GPU_GetVbiosMxmVersion = 0xe1d5daba,
NvAPI_GPU_GetVbiosImage = 0xfc13ee11,
NvAPI_GPU_GetMXMBlock = 0xb7ab19b9,

NvAPI_GPU_SetCurrentPCIEWidth = 0x3f28e1b9,
NvAPI_GPU_SetCurrentPCIESpeed = 0x3bd32008,
NvAPI_GPU_GetPCIEInfo = 0xe3795199,
NvAPI_GPU_ClearPCIELinkErrorInfo = 0x8456ff3d,
NvAPI_GPU_ClearPCIELinkAERInfo = 0x521566bb,
NvAPI_GPU_GetFrameBufferCalibrationLockFailures = 0x524b9773,
NvAPI_GPU_SetDisplayUnderflowMode = 0x387b2e41,
NvAPI_GPU_GetDisplayUnderflowStatus = 0xed9e8057,

NvAPI_GPU_GetBarInfo = 0xe4b701e3,

NvAPI_GPU_GetPSFloorSweepStatus = 0xdee047ab,
NvAPI_GPU_GetVSFloorSweepStatus = 0xd4f3944c,
NvAPI_GPU_GetSerialNumber = 0x14b83a5f,
NvAPI_GPU_GetManufacturingInfo = 0xa4218928,

NvAPI_GPU_GetRamConfigStrap = 0x51ccdb2a,
NvAPI_GPU_GetRamBusWidth = 0x7975c581,

NvAPI_GPU_GetRamBankCount = 0x17073a3c,
NvAPI_GPU_GetArchInfo = 0xd8265d24,
NvAPI_GPU_GetExtendedMinorRevision = 0x25f17421,
NvAPI_GPU_GetSampleType = 0x32e1d697,
NvAPI_GPU_GetHardwareQualType = 0xf91e777b,
NvAPI_GPU_GetAllClocks = 0x1bd69f49,
NvAPI_GPU_SetClocks = 0x6f151055,
NvAPI_GPU_SetPerfHybridMode = 0x7bc207f8,
NvAPI_GPU_GetPerfHybridMode = 0x5d7ccaeb,
NvAPI_GPU_GetHybridControllerInfo = 0xd26b8a58,

NvAPI_RestartDisplayDriver = 0xb4b26b65,
NvAPI_GPU_GetAllGpusOnSameBoard = 0x4db019e6,

NvAPI_SetTopologyDisplayGPU = 0xf409d5e5,
NvAPI_GetTopologyDisplayGPU = 0x813d89a8,
NvAPI_SYS_GetSliApprovalCookie = 0xb539a26e,

NvAPI_CreateUnAttachedDisplayFromDisplay = 0xa0c72ee4,
NvAPI_GetDriverModel = 0x25eeb2c4,
NvAPI_GPU_CudaEnumComputeCapableGpus = 0x5786cc6e,
NvAPI_GPU_PhysxSetState = 0x4071b85e,
NvAPI_GPU_PhysxQueryRecommendedState = 0x7a4174f4,
NvAPI_GPU_GetDeepIdleState = 0x1aad16b4,
NvAPI_GPU_SetDeepIdleState = 0x568a2292,

NvAPI_GetScalingCaps = 0x8e875cf9,
NvAPI_GPU_GetThermalTable = 0xc729203c,
NvAPI_SYS_SetPostOutput = 0xd3a092b1,

// source: PX18 ManagedNvApi.dll (see also: ccminer/nvapi.cpp)

NvAPI_GPU_PerfPoliciesGetInfo = 0x409d9841,
NvAPI_GPU_PerfPoliciesGetStatus = 0x3d358a0c,
NvAPI_GPU_ClientThermalPoliciesGetInfo = 0x0d258bb5,
NvAPI_GPU_ClientThermalPoliciesGetStatus = 0xe9c425a1,
NvAPI_GPU_ClientThermalPoliciesSetStatus = 0x34c0b13d,
NvAPI_GPU_ClientVoltRailsGetStatus = 0x465f9bcf, // aka NVAPI_ID_VOLTAGE_GET / NvAPI_{DLL,GPU}_GetCurrentVoltage
NvAPI_GPU_GetVoltageStep = 0x28766157, // unsure of the name
NvAPI_GPU_ClockClientClkDomainsGetInfo = 0x64b43a6a, // aka NVAPI_ID_CLK_RANGE_GET / NvAPI_{DLL,GPU}_GetClockBoostRanges
NvAPI_GPU_ClockClientClkVfPointsGetInfo = 0x507b4b59, // aka NVAPI_ID_CLK_BOOST_MASK / NvAPI_{DLL,GPU}_GetClockBoostMask
NvAPI_GPU_ClockClientClkVfPointsGetControl = 0x23f1b133, // aka NVAPI_ID_CLK_BOOST_TABLE_GET / NvAPI_{DLL,GPU}_GetClockBoostTable
NvAPI_GPU_ClockClientClkVfPointsSetControl = 0x0733e009, // aka NVAPI_ID_CLK_BOOST_TABLE_SET / NvAPI_{DLL,GPU}_SetClockBoostTable
NvAPI_GPU_ClockClientClkVfPointsGetStatus = 0x21537ad4, // aka NVAPI_ID_VFP_CURVE_GET / NvAPI_{DLL,GPU}_GetVFPCurve
NvAPI_GPU_PerfClientLimitsGetStatus = 0xe440b867, // aka NVAPI_ID_CURVE_GET / NvAPI_GPU_GetClockBoostLock
NvAPI_GPU_PerfClientLimitsSetStatus = 0x39442cfb, // aka NVAPI_ID_CURVE_SET / NvAPI_GPU_SetClockBoostLock
NvAPI_GPU_ClientVoltRailsGetControl = 0x9df23ca1, // aka NVAPI_ID_VOLTBOOST_GET / NvAPI_{DLL,GPU}_GetCoreVoltageBoostPercent
NvAPI_GPU_ClientVoltRailsSetControl = 0xb9306d9b, // aka NVAPI_ID_VOLTBOOST_SET / NvAPI_{DLL,GPU}_SetCoreVoltageBoostPercent

NvAPI_GPU_ClientFanArbitersGetControl = 0x600f612e,
NvAPI_GPU_ClientFanArbitersGetInfo = 0xdddfda38,
NvAPI_GPU_ClientFanArbitersGetStatus = 0xcde021b9,
NvAPI_GPU_ClientFanArbitersSetControl = 0x44cd3014,
NvAPI_GPU_ClientFanCoolersGetControl = 0x814b209f,
NvAPI_GPU_ClientFanCoolersGetInfo = 0xfb85b01e,
NvAPI_GPU_ClientFanCoolersGetStatus = 0x35aed5e8,
NvAPI_GPU_ClientFanCoolersSetControl = 0xa58971a5,
NvAPI_GPU_ClientFanPoliciesGetControl = 0xe543c540,
NvAPI_GPU_ClientFanPoliciesGetInfo = 0x52b76d12,
NvAPI_GPU_ClientFanPoliciesSetControl = 0xc181947a,
NvAPI_GPU_ClientGetLastOcScannerResults = 0x593e8e72,
NvAPI_GPU_ClientGetOcConfig = 0x210f1841,
NvAPI_GPU_ClientIllumDevicesGetInfo = 0xd4100e58,
NvAPI_GPU_ClientIllumDevicesGetControl = 0x73c01d58,
NvAPI_GPU_ClientIllumDevicesSetControl = 0x57024c62,
NvAPI_GPU_ClientIllumZonesGetControl = 0x3dbf5764,
NvAPI_GPU_ClientIllumZonesGetInfo = 0x4b81241b,
NvAPI_GPU_ClientIllumZonesSetControl = 0x197d065e,
NvAPI_GPU_ClientRegisterForOcConfigChangedUpdates = 0xf627074f,
NvAPI_GPU_ClientRegisterForOcScannerStatusUpdates = 0x1cb41116,
NvAPI_GPU_ClientRevertOc = 0xcc727b22,
NvAPI_GPU_ClientStartOcScanner = 0xbc4aee25,
NvAPI_GPU_ClientStopOcScanner = 0xc28b73de,

// source: https://github.com/processhacker2/plugins-extra/blob/master/NvGpuPlugin/nvidia.c

NvAPI_GPU_GetUsages = 0x189a1fdf,

NvAPI_GPU_GetRamMaker = 0x42aea16a,

// source: nvapi.lib

NvAPI_D3D_GetObjectHandleForResource = 0xfceac864,
NvAPI_D3D_SetResourceHint = 0x6c0ed98c,
NvAPI_D3D_BeginResourceRendering = 0x91123d6a,
NvAPI_D3D_EndResourceRendering = 0x37e7191c,
NvAPI_D3D12_QueryPresentBarrierSupport = 0xa15faef7,
NvAPI_D3D12_CreatePresentBarrierClient = 0x4d815de9,
NvAPI_D3D12_RegisterPresentBarrierResources = 0xd53c9ef0,
NvAPI_DestroyPresentBarrierClient = 0x3c5c351b,
NvAPI_JoinPresentBarrier = 0x17f6bf82,
NvAPI_LeavePresentBarrier = 0xc3ec5a7f,
NvAPI_QueryPresentBarrierFrameStatistics = 0x61b844a1,
NvAPI_D3D12_CreateDDisplayPresentBarrierClient = 0xb5a21987,
NvAPI_D3D11_CreateRasterizerState = 0xdb8d28af,
NvAPI_D3D_ConfigureAnsel = 0x341c6c7f,
NvAPI_D3D11_CreateTiledTexture2DArray = 0x7886981a,
NvAPI_D3D11_CheckFeatureSupport = 0x106a487e,
NvAPI_D3D11_CreateImplicitMSAATexture2D = 0xb8f79632,
NvAPI_D3D12_CreateCommittedImplicitMSAATexture2D = 0x24c6a07b,
NvAPI_D3D11_ResolveSubresourceRegion = 0xe6bfedd6,
NvAPI_D3D12_ResolveSubresourceRegion = 0xc24a15bf,
NvAPI_D3D11_TiledTexture2DArrayGetDesc = 0xf1a2b9d5,
NvAPI_D3D11_UpdateTileMappings = 0x9a06ea07,
NvAPI_D3D11_CopyTileMappings = 0xc09ee6bc,
NvAPI_D3D11_TiledResourceBarrier = 0xd6839099,
NvAPI_D3D11_AliasMSAATexture2DAsNonMSAA = 0xf1c54fc9,
NvAPI_D3D11_CreateGeometryShaderEx_2 = 0x99ed5c1c,
NvAPI_D3D11_CreateVertexShaderEx = 0x0beaa0b2,
NvAPI_D3D11_CreateHullShaderEx = 0xb53cab00,
NvAPI_D3D11_CreateDomainShaderEx = 0xa0d7180d,
NvAPI_D3D11_CreatePixelShaderEx_2 = 0x4162822b,
NvAPI_D3D11_CreateFastGeometryShaderExplicit = 0x71ab7c9c,
NvAPI_D3D11_CreateFastGeometryShader = 0x525d43be,
NvAPI_D3D11_DecompressView = 0x3a94e822,
NvAPI_D3D12_CreateGraphicsPipelineState = 0x2fc28856,
NvAPI_D3D12_CreateComputePipelineState = 0x2762deac,
NvAPI_D3D12_SetDepthBoundsTestValues = 0xb9333fe9,
NvAPI_D3D12_CreateReservedResource = 0x2c85f101,
NvAPI_D3D12_CreateHeap = 0x5cb397cf,
NvAPI_D3D12_CreateHeap2 = 0x924be9d6,
NvAPI_D3D12_QueryCpuVisibleVidmem = 0x26322bc3,
NvAPI_D3D12_ReservedResourceGetDesc = 0x9aa2aabb,
NvAPI_D3D12_UpdateTileMappings = 0xc6017a7d,
NvAPI_D3D12_CopyTileMappings = 0x47f78194,
NvAPI_D3D12_ResourceAliasingBarrier = 0xb942bab7,
NvAPI_D3D12_CaptureUAVInfo = 0x6e5ea9db,
NvAPI_D3D11_GetResourceGPUVirtualAddressEx = 0xaf6d14da,
NvAPI_D3D11_EnumerateMetaCommands = 0xc7453ba8,
NvAPI_D3D11_CreateMetaCommand = 0xf505fba0,
NvAPI_D3D11_InitializeMetaCommand = 0xaec629e9,
NvAPI_D3D11_ExecuteMetaCommand = 0x82236c47,
NvAPI_D3D12_EnumerateMetaCommands = 0xcd9141d8,
NvAPI_D3D12_CreateMetaCommand = 0xeb29634b,
NvAPI_D3D12_InitializeMetaCommand = 0xa4125399,
NvAPI_D3D12_ExecuteMetaCommand = 0xde24fc3d,
NvAPI_D3D12_CreateCommittedResource = 0x027e98ae,
NvAPI_D3D12_GetCopyableFootprints = 0xf6305eb5,
NvAPI_D3D12_CopyTextureRegion = 0x82b91b25,
NvAPI_D3D12_IsNvShaderExtnOpCodeSupported = 0x3dfacec8,
NvAPI_D3D12_GetOptimalThreadCountForMesh = 0xb43995cb,
NvAPI_D3D_IsGSyncCapable = 0x9c1eed78,
NvAPI_D3D_IsGSyncActive = 0xe942b0ff,
NvAPI_D3D1x_DisableShaderDiskCache = 0xd0cbca7d,
NvAPI_D3D11_MultiGPU_GetCaps = 0xd2d25687,
NvAPI_D3D11_MultiGPU_Init = 0x017be49e,
NvAPI_D3D11_CreateMultiGPUDevice = 0xbdb20007,
NvAPI_D3D_QuerySinglePassStereoSupport = 0x6f5f0a6d,
NvAPI_D3D_SetSinglePassStereoMode = 0xa39e6e6e,
NvAPI_D3D12_QuerySinglePassStereoSupport = 0x3b03791b,
NvAPI_D3D12_SetSinglePassStereoMode = 0x83556d87,
NvAPI_D3D_QueryMultiViewSupport = 0xb6e0a41c,
NvAPI_D3D_SetMultiViewMode = 0x8285c8da,
NvAPI_D3D_QueryModifiedWSupport = 0xcbf9f4f5,
NvAPI_D3D_SetModifiedWMode = 0x06ea4bf4,
NvAPI_D3D12_QueryModifiedWSupport = 0x51235248,
NvAPI_D3D12_SetModifiedWMode = 0xe1fdaba7,
NvAPI_D3D_CreateLateLatchObject = 0x2db27d09,
NvAPI_D3D_QueryLateLatchSupport = 0x8ceca0ec,
NvAPI_D3D_RegisterDevice = 0x8c02c4d0,
NvAPI_D3D11_MultiDrawInstancedIndirect = 0xd4e26bbf,
NvAPI_D3D11_MultiDrawIndexedInstancedIndirect = 0x59e890f9,
NvAPI_D3D_ImplicitSLIControl = 0x2aede111,
NvAPI_D3D12_UseDriverHeapPriorities = 0xf0d978a8,
NvAPI_D3D12_Mosaic_GetCompanionAllocations = 0xa46022c7,
NvAPI_D3D12_Mosaic_GetViewportAndGpuPartitions = 0xb092b818,
NvAPI_D3D1x_GetGraphicsCapabilities = 0x52b1499a,
NvAPI_D3D12_GetGraphicsCapabilities = 0x01e87354,
NvAPI_D3D11_RSSetExclusiveScissorRects = 0xae4d73ef,
NvAPI_D3D11_RSSetViewportsPixelShadingRates = 0x34f7938f,
NvAPI_D3D11_CreateShadingRateResourceView = 0x99ca2dff,
NvAPI_D3D11_RSSetShadingRateResourceView = 0x1b0c2f83,
NvAPI_D3D11_RSGetPixelShadingRateSampleOrder = 0x092442a1,
NvAPI_D3D11_RSSetPixelShadingRateSampleOrder = 0xa942373a,
NvAPI_D3D_InitializeVRSHelper = 0x4780d70b,
NvAPI_D3D_InitializeNvGazeHandler = 0x5b3b7479,
NvAPI_D3D_InitializeSMPAssist = 0x42763d0c,
NvAPI_D3D_QuerySMPAssistSupport = 0xc57921de,
NvAPI_D3D_GetSleepStatus = 0xaef96ca1,
NvAPI_D3D_SetSleepMode = 0xac1ca9e0,
NvAPI_D3D_Sleep = 0x852cd1d2,
NvAPI_D3D_GetLatency = 0x1a587f9c,
NvAPI_D3D_SetLatencyMarker = 0xd9984c05,
NvAPI_D3D12_SetAsyncFrameMarker = 0x13c98f73,
NvAPI_D3D12_NotifyOutOfBandCommandQueue = 0x03d6e8cb,
NvAPI_D3D12_CreateCubinComputeShader = 0x2a2c79e8,
NvAPI_D3D12_CreateCubinComputeShaderEx = 0x3151211b,
NvAPI_D3D12_CreateCubinComputeShaderWithName = 0x1dc7261f,
NvAPI_D3D12_LaunchCubinShader = 0x5c52bb86,
NvAPI_D3D12_DestroyCubinComputeShader = 0x7fb785ba,
NvAPI_D3D12_GetCudaTextureObject = 0x80403fc9,
NvAPI_D3D12_GetCudaSurfaceObject = 0x48f5b2ee,
NvAPI_D3D12_IsFatbinPTXSupported = 0x70c07832,
NvAPI_D3D12_CreateCuModule = 0xad1a677d,
NvAPI_D3D12_EnumFunctionsInModule = 0x7ab88d88,
NvAPI_D3D12_CreateCuFunction = 0xe2436e22,
NvAPI_D3D12_LaunchCuKernelChain = 0x24973538,
NvAPI_D3D12_DestroyCuModule = 0x41c65285,
NvAPI_D3D12_DestroyCuFunction = 0xdf295ea6,
NvAPI_D3D11_CreateCubinComputeShader = 0x0ed98181,
NvAPI_D3D11_CreateCubinComputeShaderEx = 0x32c2a0f6,
NvAPI_D3D11_CreateCubinComputeShaderWithName = 0xb672be19,
NvAPI_D3D11_LaunchCubinShader = 0x427e236d,
NvAPI_D3D11_DestroyCubinComputeShader = 0x01682c86,
NvAPI_D3D11_IsFatbinPTXSupported = 0x6086bd93,
NvAPI_D3D11_CreateUnorderedAccessView = 0x74a497a1,
NvAPI_D3D11_CreateShaderResourceView = 0x65cb431e,
NvAPI_D3D11_CreateSamplerState = 0x89eca416,
NvAPI_D3D11_GetCudaTextureObject = 0x9006fa68,
NvAPI_D3D11_GetResourceGPUVirtualAddress = 0x1819b423,
NvAPI_D3D12_GetRaytracingCaps = 0x85a6c2a0,
NvAPI_D3D12_GetRaytracingOpacityMicromapArrayPrebuildInfo = 0x4726d180,
NvAPI_D3D12_SetCreatePipelineStateOptions = 0x5c607a27,
NvAPI_D3D12_CheckDriverMatchingIdentifierEx = 0xafb237d4,
NvAPI_D3D12_GetRaytracingAccelerationStructurePrebuildInfoEx = 0x8d025b77,
NvAPI_D3D12_BuildRaytracingOpacityMicromapArray = 0x814f8d11,
NvAPI_D3D12_RelocateRaytracingOpacityMicromapArray = 0x0425c538,
NvAPI_D3D12_EmitRaytracingOpacityMicromapArrayPostbuildInfo = 0x1d9a39b6,
NvAPI_D3D12_BuildRaytracingAccelerationStructureEx = 0xe24ead45,

// source: gpu-z

/// `Unknown(*mut { version = 0x00030038, count, .. })`
Unknown_1629A173 = 0x1629a173,
/// `Unknown(hDisplayHandle, *mut hGpu)` maybe?
Unknown_F1D2777B = 0xf1d2777b,
/// `Unknown(hGpu, *mut u32, *mut u32)`
Unknown_8EFC0978 = 0x8efc0978,
/// `Unknown(hGpu, *mut { version = 0x00010008, value })` seen `value = 0x703`
Unknown_B7BCF50D = 0xb7bcf50d,
/// `Unknown(*mut { version = 0x0002000c, count, ... })` might be handles?
Unknown_36E39E6B = 0x36e39e6b,
/// `GPU_GetRasterOperators(hGpu, *mut u32)`
Unknown_GetROPCount = 0xfdc129fa,

}
