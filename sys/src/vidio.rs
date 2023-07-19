use crate::prelude_::*;
use crate::handles::NvVioHandle;

pub type NVVIOOWNERID = u32;

/// NVVIOOWNERID_NONE
pub const NVVIOOWNERID_NONE: NVVIOOWNERID = 0;

nvenum! {
    /// Owner type for device
    pub enum NVVIOOWNERTYPE / VioOwnerType {
        /// No owner for the device
        NVVIOOWNERTYPE_NONE / None = 0,
        /// Application owns the device
        NVVIOOWNERTYPE_APPLICATION / Application = 1,
        /// Desktop transparent mode owns the device
        ///
        /// (not applicable for video input)
        NVVIOOWNERTYPE_DESKTOP / Desktop = 2,
    }
}

nvenum_display! {
    VioOwnerType => _
}

/// Read access rights for [NvAPI_VIO_Open]
///
/// (not applicable for video output)
pub const NVVIO_O_READ: u32 = 0x00000000;

/// Write exclusive access rights for [NvAPI_VIO_Open]
///
/// (not applicable for video input)
pub const NVVIO_O_WRITE_EXCLUSIVE: u32 = 0x00010001;

pub const NVVIO_VALID_ACCESSRIGHTS: u32 = NVVIO_O_READ | NVVIO_O_WRITE_EXCLUSIVE;

/// `VIO_DATA.ulOwnerID` high-bit is set only if device has been initialized by VIOAPI
///
/// examined at NvAPI_GetCapabilities|[NvAPI_VIO_Open] to determine
/// if settings need to be applied from registry or POR state read
pub const NVVIO_OWNERID_INITIALIZED: u32 = 0x80000000;

/// VIO_DATA.ulOwnerID next-bit is set only if device is currently in exclusive write access mode from [NvAPI_VIO_Open]\(\)
pub const NVVIO_OWNERID_EXCLUSIVE: u32 = 0x40000000;

/// mask for NVVIOOWNERTYPE_xxx
///
/// `VIO_DATA.ulOwnerID` lower bits are: NVGVOOWNERTYPE_xxx enumerations indicating use context
pub const NVVIO_OWNERID_TYPEMASK: u32 = 0x0FFFFFFF;

nvapi! {
    /// This API opens the graphics adapter for video I/O operations
    /// using the OpenGL application interface.
    ///
    /// Read operations are permitted in this mode by multiple clients, but Write operations are application exclusive.
    #[deprecated = "Do not use this function - it is deprecated in release 440"]
    pub unsafe fn NvAPI_VIO_Open(hVioHandle: NvVioHandle, vioClass: u32, ownerType: NVVIOOWNERTYPE) -> NvAPI_Status;
}

nvapi! {
    /// This API closes the graphics adapter for graphics-to-video operations
    /// using the OpenGL application interface.
    ///
    /// Closing an OpenGL handle releases the device.
    #[deprecated = "Do not use this function - it is deprecated in release 440"]
    pub unsafe fn NvAPI_VIO_Close(hVioHandle: NvVioHandle, bRelease: BoolU32) -> NvAPI_Status;
}
