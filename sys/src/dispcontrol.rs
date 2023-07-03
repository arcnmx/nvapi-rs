#![allow(non_upper_case_globals)]

use crate::prelude_::*;
use std::os::raw::c_char;

pub(crate) type LUID = (u32, i32);

nvapi! {
    pub type EnumNvidiaDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvDispHandle@out: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns NVAPI_END_ENUMERATION.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub fn NvAPI_EnumNvidiaDisplayHandle;
}

nvapi! {
    pub type EnumNvidiaUnAttachedDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvUnAttachedDispHandle@out: *mut handles::NvUnAttachedDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA unattached display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns error.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub fn NvAPI_EnumNvidiaUnAttachedDisplayHandle;
}

nvapi! {
    pub type GetAssociatedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvDispHandle@out: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display that is associated
    /// with the given display "name" (such as "\\.\DISPLAY1").
    pub fn NvAPI_GetAssociatedNvidiaDisplayHandle;
}

nvapi! {
    pub type DISP_GetAssociatedUnAttachedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvUnAttachedDispHandle@out: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle;
}

nvenum! {
    pub enum NV_COLORSPACE_TYPE / ColorSpaceType {
        /// sRGB IEC 61966-2-1:1999 == DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709
        NV_COLORSPACE_sRGB / SRGB = 0,
        /// FP16 linear with sRGB color primaries == DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709
        NV_COLORSPACE_xRGB / XRGB = 1,
        /// ITU-R Rec BT.2100 (HDR10) == DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020
        NV_COLORSPACE_REC2100 / Rec2100 = 12,
    }
}

nvenum_display! {
    ColorSpaceType => _
}

nvapi! {
    /// This API sets colorspace of the source identified by the process id of the caller
    pub fn NvAPI_Disp_SetSourceColorSpace(displayId: u32, colorSpaceType: NV_COLORSPACE_TYPE) -> NvAPI_Status;
}

pub const NV_SOURCE_PID_CURRENT: u64 = 0;

nvapi! {
    /// This API gets colorspace of the source identified by the process id.
    ///
    /// Set `sourcePID` = [NV_SOURCE_PID_CURRENT] to use the process id of the caller.
    pub fn NvAPI_Disp_GetSourceColorSpace(displayId: u32, pColorSpaceType@out: *mut NV_COLORSPACE_TYPE, sourcePID: u64) -> NvAPI_Status;
}

nvenum! {
    pub enum NV_DISPLAY_OUTPUT_MODE / DisplayOutputMode {
        NV_DISPLAY_OUTPUT_MODE_SDR / SDR = 0,
        NV_DISPLAY_OUTPUT_MODE_HDR10 / HDR10 = 1,
        NV_DISPLAY_OUTPUT_MODE_HDR10PLUS_GAMING / HDR10PlusGaming = 2,
    }
}

nvenum_display! {
    DisplayOutputMode => _
}

nvapi! {
    /// This API sets display output mode and returns the display output mode used by the OS before the API call.
    ///
    /// Only one application at a time can override OS display output mode.
    pub fn NvAPI_Disp_SetOutputMode(displayId: u32, pDisplayMode: &mut NV_DISPLAY_OUTPUT_MODE) -> NvAPI_Status;
}

nvapi! {
    /// This API gets display output mode.
    pub fn NvAPI_Disp_GetOutputMode(displayId: u32, pDisplayMode@out: *mut NV_DISPLAY_OUTPUT_MODE) -> NvAPI_Status;
}

nvenum! {
    pub enum NV_HDR_TONEMAPPING_METHOD / HdrTonemappingMethod {
        NV_HDR_TONEMAPPING_APP / App = 0,
        NV_HDR_TONEMAPPING_GPU / Gpu = 1,
    }
}

nvenum_display! {
    HdrTonemappingMethod => _
}

nvapi! {
    /// This API sets HDR tonemapping method for the display
    pub fn NvAPI_Disp_SetHdrToneMapping(displayId: u32, hdrTonemapping: NV_HDR_TONEMAPPING_METHOD) -> NvAPI_Status;
}

nvapi! {
    /// This API gets HDR tonemapping method for the display.
    pub fn NvAPI_Disp_GetHdrToneMapping(displayId: u32, pHdrTonemapping@out: *mut NV_HDR_TONEMAPPING_METHOD) -> NvAPI_Status;
}

nvstruct! {
    pub struct NV_DISPLAY_ID_INFO_DATA_V1 {
        /// Structure version
        pub version: NvVersion,
        /// Locally unique ID (LUID) of the display adapter on which the given display is present.
        pub adapterId: LUID,
        /// The target identifier of the given display. This is also called AdapterRelativeId.
        pub targetId: u32,
        /// Reserved for future use.
        pub reserved: Padding<[u32; 4]>,
    }
}

nvversion! { NV_DISPLAY_ID_INFO_DATA(NvAPI_Disp_GetDisplayIdInfo):
    NV_DISPLAY_ID_INFO_DATA_V1(1)
}

nvapi! {
    /// This API returns information related to the given displayId.
    ///
    /// It returns adapterId and targetId (AdapterRelativeId) corresponding to the given displayId.
    ///
    /// If the displayId is part of a display grid (Mosaic/Surround), then every displayId that is part of the same display grid
    /// outputs the same (adapterId, targetId) pair, and no other displayId outputs this pair.
    /// Otherwise, the (adapterId, targetId) pair is unique to this displayId.
    pub fn NvAPI_Disp_GetDisplayIdInfo(displayId: u32, pDisplayIdInfoData@StructVersionOut: *mut NV_DISPLAY_ID_INFO_DATA) -> NvAPI_Status;
}

nvstruct! {
    pub struct NV_TARGET_INFO_DATA_V1 {
        /// Structure version
        pub version: NvVersion,
        /// Locally unique ID (LUID) of the display adapter on which the target is presnt.
        pub adapterId: LUID,
        /// The target identifier. This is also called AdapterRelativeId.
        pub targetId: u32,
        /// An array of displayIds corresponding to the input adapterId and targetId.
        ///
        /// If the input (targetId, adapterId) pair is a display grid (Mosaic/Surround)
        /// then the output contains the displayId of every display that is part of the display grid.
        /// Otherwise, it contains exactly one displayId.
        ///
        /// These displayId values are unique to this (targetId, adapterId) pair.
        pub displayId: Array<[u32; NVAPI_MAX_DISPLAYS]>,
        /// The number of displays returned in displayId array.
        pub displayIdCount: u32,
        /// Reserved for future use.
        pub reserved: Padding<[u32; 4]>,
    }
}

nvversion! { NV_TARGET_INFO_DATA(NvAPI_Disp_GetDisplayIdsFromTarget):
    NV_TARGET_INFO_DATA_V1(1)
}

nvapi! {
    /// This API returns displayId(s) corresponding to the given target.
    ///
    /// If the input (targetId, adapterId) pair is a display grid (Mosaic/Surround), then the output contains the displayId of every display
    /// that is part of the display grid. Otherwise, it contains exactly one displayId.
    ///
    /// These displayId values are unique to this (targetId, adapterId) pair.
    pub fn NvAPI_Disp_GetDisplayIdsFromTarget(displayId: u32, pTargetInfoData@StructVersionOut: *mut NV_TARGET_INFO_DATA) -> NvAPI_Status;
}

nvstruct! {
    pub struct NV_GET_VRR_INFO_V1 {
        /// Structure version
        pub version: NvVersion,
        /// Set if VRR Mode is currently enabled on given display.
        pub bIsVRREnabled: BoolU32,
        pub reservedEx: Padding<[u32; 4]>,
    }
}

nvversion! { NV_GET_VRR_INFO(NvAPI_Disp_GetVRRInfo):
    NV_GET_VRR_INFO_V1(1)
}

nvapi! {
    /// This API returns Variable Refresh Rate(VRR) information for the given display ID.
    pub fn NvAPI_Disp_GetVRRInfo(displayId: u32, pVrrInfo@StructVersionOut: *mut NV_GET_VRR_INFO) -> NvAPI_Status;
}
