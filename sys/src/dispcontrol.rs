#![allow(non_upper_case_globals)]

use crate::prelude_::*;
use std::os::raw::c_char;

nvapi! {
    pub type EnumNvidiaDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns NVAPI_END_ENUMERATION.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaDisplayHandle;
}

nvapi! {
    pub type EnumNvidiaUnAttachedDisplayHandleFn = extern "C" fn(thisEnum: u32, pNvUnAttachedDispHandle: *mut handles::NvUnAttachedDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA unattached display specified by the enum
    /// index (thisEnum). The client should keep enumerating until it
    /// returns error.
    ///
    /// Note: Display handles can get invalidated on a modeset, so the calling applications need to
    /// renum the handles after every modeset.
    pub unsafe fn NvAPI_EnumNvidiaUnAttachedDisplayHandle;
}

nvapi! {
    pub type GetAssociatedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of the NVIDIA display that is associated
    /// with the given display "name" (such as "\\.\DISPLAY1").
    pub unsafe fn NvAPI_GetAssociatedNvidiaDisplayHandle;
}

nvapi! {
    pub type DISP_GetAssociatedUnAttachedNvidiaDisplayHandleFn = extern "C" fn(szDisplayName: *const c_char, pNvUnAttachedDispHandle: *mut handles::NvDisplayHandle) -> NvAPI_Status;

    /// This function returns the handle of an unattached NVIDIA display that is
    /// associated with the given display name (such as "\\DISPLAY1").
    pub unsafe fn NvAPI_DISP_GetAssociatedUnAttachedNvidiaDisplayHandle;
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
    pub unsafe fn NvAPI_Disp_SetSourceColorSpace(displayId: u32, colorSpaceType: NV_COLORSPACE_TYPE) -> NvAPI_Status;
}

pub const NV_SOURCE_PID_CURRENT: u64 = 0;

nvapi! {
    /// This API gets colorspace of the source identified by the process id.
    ///
    /// Set `sourcePID` = [NV_SOURCE_PID_CURRENT] to use the process id of the caller.
    pub unsafe fn NvAPI_Disp_GetSourceColorSpace(displayId: u32, pColorSpaceType: *mut NV_COLORSPACE_TYPE, sourcePID: u64) -> NvAPI_Status;
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
    pub unsafe fn NvAPI_Disp_SetOutputMode(displayId: u32, pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE) -> NvAPI_Status;
}

nvapi! {
    /// This API gets display output mode.
    pub unsafe fn NvAPI_Disp_GetOutputMode(displayId: u32, pDisplayMode: *mut NV_DISPLAY_OUTPUT_MODE) -> NvAPI_Status;
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
    pub unsafe fn NvAPI_Disp_SetHdrToneMapping(displayId: u32, hdrTonemapping: NV_HDR_TONEMAPPING_METHOD) -> NvAPI_Status;
}

nvapi! {
    /// This API gets HDR tonemapping method for the display.
    pub unsafe fn NvAPI_Disp_GetHdrToneMapping(displayId: u32, pHdrTonemapping: *mut NV_HDR_TONEMAPPING_METHOD) -> NvAPI_Status;
}
