use crate::prelude_::*;

pub const NVAPI_MAX_SIZEOF_I2C_DATA_BUFFER: usize = 4096;
pub const NVAPI_MAX_SIZEOF_I2C_REG_ADDRESS: usize = 4;
pub const NVAPI_DISPLAY_DEVICE_MASK_MAX: usize = 24;
pub const NVAPI_I2C_SPEED_DEPRECATED: u32 = 0xffff;

nvenum! {
    pub enum NV_I2C_SPEED / I2cSpeed {
        NVAPI_I2C_SPEED_DEFAULT / Default = 0,
        NVAPI_I2C_SPEED_3KHZ / _3Khz = 1,
        NVAPI_I2C_SPEED_10KHZ / _10Khz = 2,
        NVAPI_I2C_SPEED_33KHZ / _33Khz = 3,
        NVAPI_I2C_SPEED_100KHZ / _100Khz = 4,
        NVAPI_I2C_SPEED_200KHZ / _200Khz = 5,
        NVAPI_I2C_SPEED_400KHZ / _400Khz = 6,
    }
}

nvenum_display! {
    I2cSpeed => {
        _3Khz = "3 kHz",
        _10Khz = "10 kHz",
        _33Khz = "33 kHz",
        _100Khz = "100 kHz",
        _200Khz = "200 kHz",
        _400Khz = "400 kHz",
        _ = _,
    }
}

nvstruct! {
    /// Used in NvAPI_I2CRead() and NvAPI_I2CWrite()
    pub struct NV_I2C_INFO_V1 {
        /// The structure version.
        pub version: NvVersion,
        /// The Display Mask of the concerned display.
        pub displayMask: u32,
        /// This flag indicates either the DDC port (TRUE) or the communication port
        /// (FALSE) of the concerned display.
        pub bIsDDCPort: u8,
        /// The address of the I2C slave.  The address should be shifted left by one.  Fo
        /// example, the I2C address 0x50, often used for reading EDIDs, would be stored
        /// here as 0xA0.  This matches the position within the byte sent by the master,
        /// the last bit is reserved to specify the read or write direction.
        pub i2cDevAddress: u8,
        /// The I2C target register address.  May be NULL, which indicates no register
        /// address should be sent.
        pub pbI2cRegAddress: *mut u8,
        /// The size in bytes of target register address.  If pbI2cRegAddress is NULL, this
        /// field must be 0.
        pub regAddrSize: u32,
        /// The buffer of data which is to be read or written (depending on the command).
        pub pbData: *mut u8,
        /// The size of the data buffer, pbData, to be read or written.
        pub cbSize: u32,
        /// The target speed of the transaction (between 28Kbps to 40Kbps; not guaranteed).
        ///
        /// Deprecated in V2+. Must be set to `NVAPI_I2C_SPEED_DEPRECATED`.
        pub i2cSpeed: u32,
    }
}

#[cfg(target_pointer_width = "64")]
const NV_I2C_INFO_V1_SIZE: usize = 4 * 2 + (1 * 2) + 6 + 8 + 4 + 4 + 8 + 4 * 2;
#[cfg(target_pointer_width = "32")]
const NV_I2C_INFO_V1_SIZE: usize = 4 * 2 + (1 * 2) + 2 + 4 + 4 + 4 + 4 * 2;

nvstruct! {
    /// Used in NvAPI_I2CRead() and NvAPI_I2CWrite()
    pub struct NV_I2C_INFO_V2 {
        /*
        /// Must set `v1.i2cSpeed = NVAPI_I2C_SPEED_DEPRECATED`.
        pub v1: NV_I2C_INFO_V1,
        */
        /// The structure version.
        pub version: NvVersion,
        /// The Display Mask of the concerned display.
        pub displayMask: u32,
        /// This flag indicates either the DDC port (TRUE) or the communication port
        /// (FALSE) of the concerned display.
        pub bIsDDCPort: u8,
        /// The address of the I2C slave.  The address should be shifted left by one.  Fo
        /// example, the I2C address 0x50, often used for reading EDIDs, would be stored
        /// here as 0xA0.  This matches the position within the byte sent by the master,
        /// the last bit is reserved to specify the read or write direction.
        pub i2cDevAddress: u8,
        /// The I2C target register address.  May be NULL, which indicates no register
        /// address should be sent.
        pub pbI2cRegAddress: *mut u8,
        /// The size in bytes of target register address.  If pbI2cRegAddress is NULL, this
        /// field must be 0.
        pub regAddrSize: u32,
        /// The buffer of data which is to be read or written (depending on the command).
        pub pbData: *mut u8,
        /// The size of the data buffer, pbData, to be read or written.
        pub cbSize: u32,
        /// Deprecated - must be set to `NVAPI_I2C_SPEED_DEPRECATED`.
        pub i2cSpeed: u32,
        /// The target speed of the transaction in (kHz) (Chosen from the enum `NV_I2C_SPEED`).
        pub i2cSpeedKhz: NV_I2C_SPEED,
    }
}

#[cfg(target_pointer_width = "64")]
const NV_I2C_INFO_V2_SIZE: usize = NV_I2C_INFO_V1_SIZE + 4 + 4;
#[cfg(target_pointer_width = "32")]
const NV_I2C_INFO_V2_SIZE: usize = NV_I2C_INFO_V1_SIZE + 4;

nvstruct! {
    /// Used in NvAPI_I2CRead() and NvAPI_I2CWrite()
    pub struct NV_I2C_INFO_V3 {
        //pub v2: NV_I2C_INFO_V2,
        /// The structure version.
        pub version: NvVersion,
        /// The Display Mask of the concerned display.
        pub displayMask: u32,
        /// This flag indicates either the DDC port (TRUE) or the communication port
        /// (FALSE) of the concerned display.
        pub bIsDDCPort: u8,
        /// The address of the I2C slave.  The address should be shifted left by one.  Fo
        /// example, the I2C address 0x50, often used for reading EDIDs, would be stored
        /// here as 0xA0.  This matches the position within the byte sent by the master,
        /// the last bit is reserved to specify the read or write direction.
        pub i2cDevAddress: u8,
        /// The I2C target register address.  May be NULL, which indicates no register
        /// address should be sent.
        pub pbI2cRegAddress: *mut u8,
        /// The size in bytes of target register address.  If pbI2cRegAddress is NULL, this
        /// field must be 0.
        pub regAddrSize: u32,
        /// The buffer of data which is to be read or written (depending on the command).
        pub pbData: *mut u8,
        /// The size of the data buffer, pbData, to be read or written.
        pub cbSize: u32,
        /// Deprecated - must be set to `NVAPI_I2C_SPEED_DEPRECATED`.
        pub i2cSpeed: u32,
        /// The target speed of the transaction in (kHz) (Chosen from the enum `NV_I2C_SPEED`).
        pub i2cSpeedKhz: NV_I2C_SPEED,
        /// The portid on which device is connected (remember to set bIsPortIdSet if this value is set)
        ///
        /// Optional for pre-Kepler
        pub portId: u8,
        /// set this flag on if and only if portid value is set
        pub bIsPortIdSet: u32,
    }
}

const NV_I2C_INFO_V3_SIZE: usize = NV_I2C_INFO_V2_SIZE + 1 + 3 + 4;

pub type NV_I2C_INFO = NV_I2C_INFO_V3;

nvversion! { NV_I2C_INFO_VER1(NV_I2C_INFO_V1 = NV_I2C_INFO_V1_SIZE, 1) }
nvversion! { NV_I2C_INFO_VER2(NV_I2C_INFO_V2 = NV_I2C_INFO_V2_SIZE, 2) }
nvversion! { NV_I2C_INFO_VER3(NV_I2C_INFO_V3 = NV_I2C_INFO_V3_SIZE, 3) }
nvversion! { NV_I2C_INFO_VER = NV_I2C_INFO_VER3 }

nvapi! {
    pub type NvAPI_I2CReadFn = extern "C" fn(hPhysicalGpu: NvPhysicalGpuHandle, pI2cInfo: *mut NV_I2C_INFO) -> NvAPI_Status;

    /// This function reads the data buffer from the I2C port.
    /// The I2C request must be for a DDC port: pI2cInfo->bIsDDCPort = 1.
    ///
    /// A data buffer size larger than 16 bytes may be rejected if a register address is specified.  In such a case,
    /// NVAPI_ARGUMENT_EXCEED_MAX_SIZE would be returned.
    ///
    /// If a register address is specified (i.e. regAddrSize is positive), then the transaction will be performed in
    /// the combined format described in the I2C specification.  The register address will be written, followed by
    /// reading into the data buffer.
    ///
    /// # Returns
    ///
    /// - `NVAPI_OK`: Completed request
    /// - `NVAPI_ERROR`: Miscellaneous error occurred.
    /// - `NVAPI_HANDLE_INVALIDATED`: Handle passed has been invalidated (see user guide).
    /// - `NVAPI_EXPECTED_PHYSICAL_GPU_HANDLE`: Handle passed is not a physical GPU handle.
    /// - `NVAPI_INCOMPATIBLE_STRUCT_VERSION`: Structure version is not supported.
    /// - `NVAPI_INVALID_ARGUMENT`: argument does not meet specified requirements
    /// - `NVAPI_ARGUMENT_EXCEED_MAX_SIZE`: an argument exceeds the maximum
    pub unsafe fn NvAPI_I2CRead;
}

nvapi! {
    pub type NvAPI_I2CWriteFn = extern "C" fn(hPhysicalGpu: NvPhysicalGpuHandle, pI2cInfo: *mut NV_I2C_INFO) -> NvAPI_Status;

    /// This function writes the data buffer to the I2C port.
    ///
    /// The I2C request must be for a DDC port: pI2cInfo->bIsDDCPort = 1.
    ///
    /// A data buffer size larger than 16 bytes may be rejected if a register address is specified.  In such a case,
    /// NVAPI_ARGUMENT_EXCEED_MAX_SIZE would be returned.
    ///
    /// If a register address is specified (i.e. regAddrSize is positive), then the register address will be written
    /// and the data buffer will immediately follow without a restart.
    ///
    /// # Returns
    ///
    /// - `NVAPI_OK`: Completed request
    /// - `NVAPI_ERROR`: Miscellaneous error occurred.
    /// - `NVAPI_HANDLE_INVALIDATED`: Handle passed has been invalidated (see user guide).
    /// - `NVAPI_EXPECTED_PHYSICAL_GPU_HANDLE`: Handle passed is not a physical GPU handle.
    /// - `NVAPI_INCOMPATIBLE_STRUCT_VERSION`: Structure version is not supported.
    /// - `NVAPI_INVALID_ARGUMENT`: Argument does not meet specified requirements
    /// - `NVAPI_ARGUMENT_EXCEED_MAX_SIZE`: exceeds the maximum
    pub unsafe fn NvAPI_I2CWrite;
}

/// Undocumented API
pub mod private {
    use crate::prelude_::*;
    use super::NV_I2C_SPEED;

    nvstruct! {
        /// Used in NvAPI_I2CRead() and NvAPI_I2CWrite()
        pub struct NV_I2C_INFO_EX_V3 {
            /// The structure version.
            pub version: NvVersion,
            /// The Display Mask of the concerned display.
            pub displayMask: u32,
            /// This flag indicates either the DDC port (TRUE) or the communication port
            /// (FALSE) of the concerned display.
            pub bIsDDCPort: u8,
            /// The address of the I2C slave.  The address should be shifted left by one.  Fo
            /// example, the I2C address 0x50, often used for reading EDIDs, would be stored
            /// here as 0xA0.  This matches the position within the byte sent by the master,
            /// the last bit is reserved to specify the read or write direction.
            pub i2cDevAddress: u8,
            /// The I2C target register address.  May be NULL, which indicates no register
            /// address should be sent.
            pub pbI2cRegAddress: *mut u8,
            /// The size in bytes of target register address.  If pbI2cRegAddress is NULL, this
            /// field must be 0.
            pub regAddrSize: u32,
            /// The buffer of data which is to be read or written (depending on the command).
            pub pbData: *mut u8,
            /// bytes to read ??? seems required on write too
            pub pbRead: u32,
            /// The size of the data buffer, pbData, to be read or written.
            pub cbSize: u32,
            /// The target speed of the transaction in (kHz) (Chosen from the enum `NV_I2C_SPEED`).
            pub i2cSpeedKhz: NV_I2C_SPEED,
            /// The portid on which device is connected (remember to set bIsPortIdSet if this value is set)
            ///
            /// Optional for pre-Kepler
            pub portId: u8,
            /// set this flag on if and only if portid value is set
            pub bIsPortIdSet: u32,
        }
    }

    #[cfg(target_pointer_width = "64")]
    const NV_I2C_INFO_EX_V3_SIZE: usize = 4 * 2 + (1 * 2) + 6 + 8 + 4 + 4 + 8 + 4 * 3 + 1 + 3 + 4 + 4;
    #[cfg(target_pointer_width = "32")]
    const NV_I2C_INFO_EX_V3_SIZE: usize = 4 * 2 + (1 * 2) + 2 + 4 + 4 + 4 + 4 * 3 + 1 + 3 + 4;

    pub type NV_I2C_INFO_EX = NV_I2C_INFO_EX_V3;

    nvversion! { NV_I2C_INFO_EX_VER3(NV_I2C_INFO_EX_V3 = NV_I2C_INFO_EX_V3_SIZE, 3) }
    nvversion! { NV_I2C_INFO_EX_VER = NV_I2C_INFO_EX_VER3 }

    nvapi! {
        pub type NvAPI_I2CReadExFn = extern "C" fn(hPhysicalGpu: NvPhysicalGpuHandle, pI2cInfo: *mut NV_I2C_INFO_EX, pData: *mut u32) -> NvAPI_Status;

        /// Undocumented function. `pData` is often `{ 1, 0 }`?
        pub unsafe fn NvAPI_I2CReadEx;
    }

    nvapi! {
        pub type NvAPI_I2CWriteExFn = extern "C" fn(hPhysicalGpu: NvPhysicalGpuHandle, pI2cInfo: *mut NV_I2C_INFO_EX, pData: *mut u32) -> NvAPI_Status;

        /// Undocumented function. `pData` is often `{ 1, 0 }`?
        pub unsafe fn NvAPI_I2CWriteEx;
    }
}
