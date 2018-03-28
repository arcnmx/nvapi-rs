use std::{fmt, error};

nvenum! {
    /// NvAPI Status Values
    ///
    /// All NvAPI functions return one of these codes.
    pub enum NvAPI_Status / Status {
        /// Success. Request is completed.
        NVAPI_OK / Ok = 0,
        /// Generic error
        NVAPI_ERROR / Error = -1,
        /// NVAPI support library cannot be loaded.
        NVAPI_LIBRARY_NOT_FOUND / LibraryNotFound = -2,
        /// not implemented in current driver installation
        NVAPI_NO_IMPLEMENTATION / NoImplementation = -3,
        /// NvAPI_Initialize has not been called (successfully)
        NVAPI_API_NOT_INITIALIZED / ApiNotInitialized = -4,
        /// The argument/parameter value is not valid or NULL.
        NVAPI_INVALID_ARGUMENT / InvalidArgument = -5,
        /// No NVIDIA display driver, or NVIDIA GPU driving a display, was found.
        NVAPI_NVIDIA_DEVICE_NOT_FOUND / NvidiaDeviceNotFound = -6,
        /// No more items to enumerate
        NVAPI_END_ENUMERATION / EndEnumeration = -7,
        /// Invalid handle
        NVAPI_INVALID_HANDLE / InvalidHandle = -8,
        /// An argument's structure version is not supported
        NVAPI_INCOMPATIBLE_STRUCT_VERSION / IncompatibleStructVersion = -9,
        /// The handle is no longer valid (likely due to GPU or display re-configuration)
        NVAPI_HANDLE_INVALIDATED / HandleInvalidated = -10,
        /// No NVIDIA OpenGL context is current (but needs to be)
        NVAPI_OPENGL_CONTEXT_NOT_CURRENT / OpenglContextNotCurrent = -11,
        /// An invalid pointer, usually NULL, was passed as a parameter
        NVAPI_INVALID_POINTER / InvalidPointer = -14,
        /// OpenGL Expert is not supported by the current drivers
        NVAPI_NO_GL_EXPERT / NoGlExpert = -12,
        /// OpenGL Expert is supported, but driver instrumentation is currently disabled
        NVAPI_INSTRUMENTATION_DISABLED / InstrumentationDisabled = -13,
        /// OpenGL does not support Nsight
        NVAPI_NO_GL_NSIGHT / NoGlNsight = -15,

        /// Expected a logical GPU handle for one or more parameters
        NVAPI_EXPECTED_LOGICAL_GPU_HANDLE / ExpectedLogicalGpuHandle = -100,
        /// Expected a physical GPU handle for one or more parameters
        NVAPI_EXPECTED_PHYSICAL_GPU_HANDLE / ExpectedPhysicalGpuHandle = -101,
        /// Expected an NV display handle for one or more parameters
        NVAPI_EXPECTED_DISPLAY_HANDLE / ExpectedDisplayHandle = -102,
        /// The combination of parameters is not valid. 
        NVAPI_INVALID_COMBINATION / InvalidCombination = -103,
        /// Requested feature is not supported in the selected GPU
        NVAPI_NOT_SUPPORTED / NotSupported = -104,
        /// No port ID was found for the I2C transaction
        NVAPI_PORTID_NOT_FOUND / PortidNotFound = -105,
        /// Expected an unattached display handle as one of the input parameters.
        NVAPI_EXPECTED_UNATTACHED_DISPLAY_HANDLE / ExpectedUnattachedDisplayHandle = -106,
        /// Invalid perf level 
        NVAPI_INVALID_PERF_LEVEL / InvalidPerfLevel = -107,
        /// Device is busy; request not fulfilled
        NVAPI_DEVICE_BUSY / DeviceBusy = -108,
        /// NV persist file is not found
        NVAPI_NV_PERSIST_FILE_NOT_FOUND / NvPersistFileNotFound = -109,
        /// NV persist data is not found
        NVAPI_PERSIST_DATA_NOT_FOUND / PersistDataNotFound = -110,
        /// Expected a TV output display
        NVAPI_EXPECTED_TV_DISPLAY / ExpectedTvDisplay = -111,
        /// Expected a TV output on the D Connector - HDTV_EIAJ4120.
        NVAPI_EXPECTED_TV_DISPLAY_ON_DCONNECTOR / ExpectedTvDisplayOnDconnector = -112,
        /// SLI is not active on this device.
        NVAPI_NO_ACTIVE_SLI_TOPOLOGY / NoActiveSliTopology = -113,
        /// Setup of SLI rendering mode is not possible right now.
        NVAPI_SLI_RENDERING_MODE_NOTALLOWED / SliRenderingModeNotallowed = -114,
        /// Expected a digital flat panel.
        NVAPI_EXPECTED_DIGITAL_FLAT_PANEL / ExpectedDigitalFlatPanel = -115,
        /// Argument exceeds the expected size.
        NVAPI_ARGUMENT_EXCEED_MAX_SIZE / ArgumentExceedMaxSize = -116,
        /// Inhibit is ON due to one of the flags in NV_GPU_DISPLAY_CHANGE_INHIBIT or SLI active.
        NVAPI_DEVICE_SWITCHING_NOT_ALLOWED / DeviceSwitchingNotAllowed = -117,
        /// Testing of clocks is not supported.
        NVAPI_TESTING_CLOCKS_NOT_SUPPORTED / TestingClocksNotSupported = -118,
        /// The specified underscan config is from an unknown source (e.g. INF)
        NVAPI_UNKNOWN_UNDERSCAN_CONFIG / UnknownUnderscanConfig = -119,
        /// Timeout while reconfiguring GPUs
        NVAPI_TIMEOUT_RECONFIGURING_GPU_TOPO / TimeoutReconfiguringGpuTopo = -120,
        /// Requested data was not found
        NVAPI_DATA_NOT_FOUND / DataNotFound = -121,
        /// Expected an analog display
        NVAPI_EXPECTED_ANALOG_DISPLAY / ExpectedAnalogDisplay = -122,
        /// No SLI video bridge is present
        NVAPI_NO_VIDLINK / NoVidlink = -123,
        /// NVAPI requires a reboot for the settings to take effect
        NVAPI_REQUIRES_REBOOT / RequiresReboot = -124,
        /// The function is not supported with the current Hybrid mode.
        NVAPI_INVALID_HYBRID_MODE / InvalidHybridMode = -125,
        /// The target types are not all the same
        NVAPI_MIXED_TARGET_TYPES / MixedTargetTypes = -126,
        /// The function is not supported from 32-bit on a 64-bit system.
        NVAPI_SYSWOW64_NOT_SUPPORTED / Syswow64NotSupported = -127,
        /// There is no implicit GPU topology active. Use NVAPI_SetHybridMode to change topology.
        NVAPI_IMPLICIT_SET_GPU_TOPOLOGY_CHANGE_NOT_ALLOWED / ImplicitSetGpuTopologyChangeNotAllowed = -128,
        /// Prompt the user to close all non-migratable applications. 
        NVAPI_REQUEST_USER_TO_CLOSE_NON_MIGRATABLE_APPS / RequestUserToCloseNonMigratableApps = -129,
        /// Could not allocate sufficient memory to complete the call.
        NVAPI_OUT_OF_MEMORY / OutOfMemory = -130,
        /// The previous operation that is transferring information to or from this surface is incomplete.
        NVAPI_WAS_STILL_DRAWING / WasStillDrawing = -131,
        /// The file was not found.
        NVAPI_FILE_NOT_FOUND / FileNotFound = -132,
        /// There are too many unique instances of a particular type of state object.
        NVAPI_TOO_MANY_UNIQUE_STATE_OBJECTS / TooManyUniqueStateObjects = -133,
        /// The method call is invalid. For example, a method's parameter may not be a valid pointer.
        NVAPI_INVALID_CALL / InvalidCall = -134,
        /// d3d10_1.dll cannot be loaded.
        NVAPI_D3D10_1_LIBRARY_NOT_FOUND / D3d101LibraryNotFound = -135,
        /// Couldn't find the function in the loaded DLL.
        NVAPI_FUNCTION_NOT_FOUND / FunctionNotFound = -136,
        /// Current User is not Admin.
        NVAPI_INVALID_USER_PRIVILEGE / InvalidUserPrivilege = -137,
        /// The handle corresponds to GDIPrimary.
        NVAPI_EXPECTED_NON_PRIMARY_DISPLAY_HANDLE / ExpectedNonPrimaryDisplayHandle = -138,
        /// Setting Physx GPU requires that the GPU is compute-capable.
        NVAPI_EXPECTED_COMPUTE_GPU_HANDLE / ExpectedComputeGpuHandle = -139,
        /// The Stereo part of NVAPI failed to initialize completely. Check if the stereo driver is installed.
        NVAPI_STEREO_NOT_INITIALIZED / StereoNotInitialized = -140,
        /// Access to stereo-related registry keys or values has failed.
        NVAPI_STEREO_REGISTRY_ACCESS_FAILED / StereoRegistryAccessFailed = -141,
        /// The given registry profile type is not supported.
        NVAPI_STEREO_REGISTRY_PROFILE_TYPE_NOT_SUPPORTED / StereoRegistryProfileTypeNotSupported = -142,
        /// The given registry value is not supported.
        NVAPI_STEREO_REGISTRY_VALUE_NOT_SUPPORTED / StereoRegistryValueNotSupported = -143,
        /// Stereo is not enabled and the function needed it to execute completely.
        NVAPI_STEREO_NOT_ENABLED / StereoNotEnabled = -144,
        /// Stereo is not turned on and the function needed it to execute completely.
        NVAPI_STEREO_NOT_TURNED_ON / StereoNotTurnedOn = -145,
        /// Invalid device interface.
        NVAPI_STEREO_INVALID_DEVICE_INTERFACE / StereoInvalidDeviceInterface = -146,
        /// Separation percentage or JPEG image capture quality is out of [0-100] range.
        NVAPI_STEREO_PARAMETER_OUT_OF_RANGE / StereoParameterOutOfRange = -147,
        /// The given frustum adjust mode is not supported.
        NVAPI_STEREO_FRUSTUM_ADJUST_MODE_NOT_SUPPORTED / StereoFrustumAdjustModeNotSupported = -148,
        /// The mosaic topology is not possible given the current state of the hardware.
        NVAPI_TOPO_NOT_POSSIBLE / TopoNotPossible = -149,
        /// An attempt to do a display resolution mode change has failed. 
        NVAPI_MODE_CHANGE_FAILED / ModeChangeFailed = -150,
        /// d3d11.dll/d3d11_beta.dll cannot be loaded.
        NVAPI_D3D11_LIBRARY_NOT_FOUND / D3d11LibraryNotFound = -151,
        /// Address is outside of valid range.
        NVAPI_INVALID_ADDRESS / InvalidAddress = -152,
        /// The pre-allocated string is too small to hold the result.
        NVAPI_STRING_TOO_SMALL / StringTooSmall = -153,
        /// The input does not match any of the available devices.
        NVAPI_MATCHING_DEVICE_NOT_FOUND / MatchingDeviceNotFound = -154,
        /// Driver is running.
        NVAPI_DRIVER_RUNNING / DriverRunning = -155,
        /// Driver is not running.
        NVAPI_DRIVER_NOTRUNNING / DriverNotrunning = -156,
        /// A driver reload is required to apply these settings.
        NVAPI_ERROR_DRIVER_RELOAD_REQUIRED / ErrorDriverReloadRequired = -157,
        /// Intended setting is not allowed.
        NVAPI_SET_NOT_ALLOWED / SetNotAllowed = -158,
        /// Information can't be returned due to "advanced display topology".
        NVAPI_ADVANCED_DISPLAY_TOPOLOGY_REQUIRED / AdvancedDisplayTopologyRequired = -159,
        /// Setting is not found.
        NVAPI_SETTING_NOT_FOUND / SettingNotFound = -160,
        /// Setting size is too large.
        NVAPI_SETTING_SIZE_TOO_LARGE / SettingSizeTooLarge = -161,
        /// There are too many settings for a profile. 
        NVAPI_TOO_MANY_SETTINGS_IN_PROFILE / TooManySettingsInProfile = -162,
        /// Profile is not found.
        NVAPI_PROFILE_NOT_FOUND / ProfileNotFound = -163,
        /// Profile name is duplicated.
        NVAPI_PROFILE_NAME_IN_USE / ProfileNameInUse = -164,
        /// Profile name is empty.
        NVAPI_PROFILE_NAME_EMPTY / ProfileNameEmpty = -165,
        /// Application not found in the Profile.
        NVAPI_EXECUTABLE_NOT_FOUND / ExecutableNotFound = -166,
        /// Application already exists in the other profile.
        NVAPI_EXECUTABLE_ALREADY_IN_USE / ExecutableAlreadyInUse = -167,
        /// Data Type mismatch 
        NVAPI_DATATYPE_MISMATCH / DatatypeMismatch = -168,
        /// The profile passed as parameter has been removed and is no longer valid.
        NVAPI_PROFILE_REMOVED / ProfileRemoved = -169,
        /// An unregistered resource was passed as a parameter. 
        NVAPI_UNREGISTERED_RESOURCE / UnregisteredResource = -170,
        /// The DisplayId corresponds to a display which is not within the normal outputId range.
        NVAPI_ID_OUT_OF_RANGE / IdOutOfRange = -171,
        /// Display topology is not valid so the driver cannot do a mode set on this configuration.
        NVAPI_DISPLAYCONFIG_VALIDATION_FAILED / DisplayconfigValidationFailed = -172,
        /// Display Port Multi-Stream topology has been changed.
        NVAPI_DPMST_CHANGED / DpmstChanged = -173,
        /// Input buffer is insufficient to hold the contents. 
        NVAPI_INSUFFICIENT_BUFFER / InsufficientBuffer = -174,
        /// No access to the caller.
        NVAPI_ACCESS_DENIED / AccessDenied = -175,
        /// The requested action cannot be performed without Mosaic being enabled.
        NVAPI_MOSAIC_NOT_ACTIVE / MosaicNotActive = -176,
        /// The surface is relocated away from video memory.
        NVAPI_SHARE_RESOURCE_RELOCATED / ShareResourceRelocated = -177,
        /// The user should disable DWM before calling NvAPI.
        NVAPI_REQUEST_USER_TO_DISABLE_DWM / RequestUserToDisableDwm = -178,
        /// D3D device status is D3DERR_DEVICELOST or D3DERR_DEVICENOTRESET - the user has to reset the device.
        NVAPI_D3D_DEVICE_LOST / D3dDeviceLost = -179,
        /// The requested action cannot be performed in the current state.
        NVAPI_INVALID_CONFIGURATION / InvalidConfiguration = -180,
        /// Call failed as stereo handshake not completed.
        NVAPI_STEREO_HANDSHAKE_NOT_DONE / StereoHandshakeNotDone = -181,
        /// The path provided was too short to determine the correct NVDRS_APPLICATION
        NVAPI_EXECUTABLE_PATH_IS_AMBIGUOUS / ExecutablePathIsAmbiguous = -182,
        /// Default stereo profile is not currently defined
        NVAPI_DEFAULT_STEREO_PROFILE_IS_NOT_DEFINED / DefaultStereoProfileIsNotDefined = -183,
        /// Default stereo profile does not exist
        NVAPI_DEFAULT_STEREO_PROFILE_DOES_NOT_EXIST / DefaultStereoProfileDoesNotExist = -184,
        /// A cluster is already defined with the given configuration.
        NVAPI_CLUSTER_ALREADY_EXISTS / ClusterAlreadyExists = -185,
        /// The input display id is not that of a multi stream enabled connector or a display device in a multi stream topology 
        NVAPI_DPMST_DISPLAY_ID_EXPECTED / DpmstDisplayIdExpected = -186,
        /// The input display id is not valid or the monitor associated to it does not support the current operation
        NVAPI_INVALID_DISPLAY_ID / InvalidDisplayId = -187,
        /// While playing secure audio stream, stream goes out of sync
        NVAPI_STREAM_IS_OUT_OF_SYNC / StreamIsOutOfSync = -188,
        /// Older audio driver version than required
        NVAPI_INCOMPATIBLE_AUDIO_DRIVER / IncompatibleAudioDriver = -189,
        /// Value already set, setting again not allowed.
        NVAPI_VALUE_ALREADY_SET / ValueAlreadySet = -190,
        /// Requested operation timed out 
        NVAPI_TIMEOUT / Timeout = -191,
        /// The requested workstation feature set has incomplete driver internal allocation resources
        NVAPI_GPU_WORKSTATION_FEATURE_INCOMPLETE / GpuWorkstationFeatureIncomplete = -192,
        /// Call failed because InitActivation was not called.
        NVAPI_STEREO_INIT_ACTIVATION_NOT_DONE / StereoInitActivationNotDone = -193,
        /// The requested action cannot be performed without Sync being enabled. 
        NVAPI_SYNC_NOT_ACTIVE / SyncNotActive = -194,
        /// The requested action cannot be performed without Sync Master being enabled.
        NVAPI_SYNC_MASTER_NOT_FOUND / SyncMasterNotFound = -195,
        /// Invalid displays passed in the NV_GSYNC_DISPLAY pointer.
        NVAPI_INVALID_SYNC_TOPOLOGY / InvalidSyncTopology = -196,
        /// The specified signing algorithm is not supported.
        /// Either an incorrect value was entered or the current installed driver/hardware does not support the input value.
        NVAPI_ECID_SIGN_ALGO_UNSUPPORTED / EcidSignAlgoUnsupported = -197,
        /// The encrypted public key verification has failed.
        NVAPI_ECID_KEY_VERIFICATION_FAILED / EcidKeyVerificationFailed = -198,
        /// The device's firmware is out of date.
        NVAPI_FIRMWARE_OUT_OF_DATE / FirmwareOutOfDate = -199,
        /// The device's firmware is not supported.
        NVAPI_FIRMWARE_REVISION_NOT_SUPPORTED / FirmwareRevisionNotSupported = -200,
        /// The caller is not authorized to modify the License.
        NVAPI_LICENSE_CALLER_AUTHENTICATION_FAILED / LicenseCallerAuthenticationFailed = -201,
        /// The user tried to use a deferred context without registering the device first 	 
        NVAPI_D3D_DEVICE_NOT_REGISTERED / D3dDeviceNotRegistered = -202,
        /// Head or SourceId was not reserved for the VR Display before doing the Modeset.
        NVAPI_RESOURCE_NOT_ACQUIRED / ResourceNotAcquired = -203,
        /// Provided timing is not supported.
        NVAPI_TIMING_NOT_SUPPORTED / TimingNotSupported = -204,
        /// HDCP Encryption Failed for the device. Would be applicable when the device is HDCP Capable.
        NVAPI_HDCP_ENCRYPTION_FAILED / HdcpEncryptionFailed = -205,
        /// Provided mode is over sink device pclk limitation.
        NVAPI_PCLK_LIMITATION_FAILED / PclkLimitationFailed = -206,
        /// No connector on GPU found. 
        NVAPI_NO_CONNECTOR_FOUND / NoConnectorFound = -207,
        /// When a non-HDCP capable HMD is connected, we would inform user by this code.
        NVAPI_HDCP_DISABLED / HdcpDisabled = -208,
        /// Atleast an API is still being called
        NVAPI_API_IN_USE / ApiInUse = -209,
        /// No display found on Nvidia GPU(s).
        NVAPI_NVIDIA_DISPLAY_NOT_FOUND / NvidiaDisplayNotFound = -210,
        /// Priv security violation, improper access to a secured register.
        NVAPI_PRIV_SEC_VIOLATION / PrivSecViolation = -211,
        /// NVAPI cannot be called by this vendor
        NVAPI_INCORRECT_VENDOR / IncorrectVendor = -212,
        /// DirectMode Display is already in use
        NVAPI_DISPLAY_IN_USE / DisplayInUse = -213,
        /// The Config is having Non-NVidia GPU with Non-HDCP HMD connected
        NVAPI_UNSUPPORTED_CONFIG_NON_HDCP_HMD / UnsupportedConfigNonHdcpHmd = -214,
        /// GPU's Max Display Limit has Reached
        NVAPI_MAX_DISPLAY_LIMIT_REACHED / MaxDisplayLimitReached = -215,
        /// DirectMode not Enabled on the Display
        NVAPI_INVALID_DIRECT_MODE_DISPLAY / InvalidDirectModeDisplay = -216,
        /// GPU is in debug mode, OC is NOT allowed.
        NVAPI_GPU_IN_DEBUG_MODE / GpuInDebugMode = -217,
    }
}

impl error::Error for Status {
    fn description(&self) -> &str {
        "NVAPI Error"
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
