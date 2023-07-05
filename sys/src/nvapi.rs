use std::mem::transmute;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::os::raw::c_void;
use std::ffi::CStr;
use std::ptr::NonNull;
use std::io;
use crate::status::{Status, NvAPI_Status};
use crate::NvString;

pub const NVAPI_GENERIC_STRING_MAX: usize = 4096;
pub const NVAPI_LONG_STRING_MAX: usize = 256;
pub const NVAPI_SHORT_STRING_MAX: usize = 64;

pub type NvAPI_String = NvString<NVAPI_GENERIC_STRING_MAX>;
pub type NvAPI_LongString = NvString<NVAPI_LONG_STRING_MAX>;
pub type NvAPI_ShortString = NvString<NVAPI_SHORT_STRING_MAX>;

pub type NvBool = u8;

pub const NV_TRUE: NvBool = 1;
pub const NV_FALSE: NvBool = 0;

nvstruct! {
    pub struct NV_RECT {
        pub left: u32,
        pub top: u32,
        pub right: u32,
        pub bottom: u32,
    }
}

nvstruct! {
    pub struct NvSBox {
        pub sX: i32,
        pub sY: i32,
        pub sWidth: i32,
        pub sHeight: i32,
    }
}

nvstruct! {
    pub struct NvGUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
}

pub type NvLUID = NvGUID;

pub const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;

pub const NVAPI_MAX_PHYSICAL_BRIDGES: usize = 100;
pub const NVAPI_PHYSICAL_GPUS: usize = 32;
pub const NVAPI_MAX_LOGICAL_GPUS: usize = 64;
pub const NVAPI_MAX_AVAILABLE_GPU_TOPOLOGIES: usize = 256;
pub const NVAPI_MAX_AVAILABLE_SLI_GROUPS: usize = 265;
pub const NVAPI_MAX_GPU_TOPOLOGIES: usize = NVAPI_MAX_PHYSICAL_GPUS;
pub const NVAPI_MAX_GPU_PER_TOPOLOGY: usize = 8;
pub const NVAPI_MAX_DISPLAY_HEADS: usize = 2;
pub const NVAPI_ADVANCED_DISPLAY_HEADS: usize = 4;
pub const NVAPI_MAX_DISPLAYS: usize = NVAPI_PHYSICAL_GPUS * NVAPI_ADVANCED_DISPLAY_HEADS;
pub const NVAPI_MAX_ACPI_IDS: usize = 16;
pub const NVAPI_MAX_VIEW_MODES: usize = 8;
pub const NVAPI_MAX_HEADS_PER_GPU: usize = 32;

/// Maximum heads, each with `NVAPI_DESKTOP_RES` resolution
pub const NV_MAX_HEADS: usize = 4;
/// Maximum number of input video streams, each with a `NVAPI_VIDEO_SRC_INFO`
pub const NV_MAX_VID_STREAMS: usize = 4;
/// Maximum number of output video profiles supported
pub const NV_MAX_VID_PROFILES: usize = 4;

pub const NVAPI_SYSTEM_MAX_DISPLAYS: usize = NVAPI_MAX_PHYSICAL_GPUS * NV_MAX_HEADS;

pub const NVAPI_SYSTEM_MAX_HWBCS: usize = 128;
pub const NVAPI_SYSTEM_HWBC_INVALID_ID: usize = 0xffffffff;
pub const NVAPI_MAX_AUDIO_DEVICES: usize = 16;

pub type QueryInterfaceFn = extern "C" fn(id: u32) -> *const c_void;

macro_rules! cstr {
    ($s:expr) => {
        unsafe { CStr::from_bytes_with_nul_unchecked(concat!($s, "\0").as_bytes()) }
    };
}

#[cfg(all(windows, target_pointer_width = "32"))]
pub const LIBRARY_NAME: &'static CStr = cstr!("nvapi.dll");
#[cfg(all(windows, target_pointer_width = "64"))]
pub const LIBRARY_NAME: &'static CStr = cstr!("nvapi64.dll");
#[cfg(target_os = "linux")]
pub const LIBRARY_NAME: &'static CStr = cstr!("libnvidia-api.so.1");

pub const FN_NAME: &'static CStr = cstr!("nvapi_QueryInterface");

static QUERY_INTERFACE_CACHE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn set_query_interface(ptr: QueryInterfaceFn) {
    QUERY_INTERFACE_CACHE.store(ptr as usize, Ordering::Relaxed);
}

pub fn get_query_interface() -> Option<QueryInterfaceFn> {
    match QUERY_INTERFACE_CACHE.load(Ordering::Relaxed) {
        0 => None,
        ptr => Some(unsafe { transmute::<_, QueryInterfaceFn>(ptr) }),
    }
}

pub fn load_query_interface() -> io::Result<QueryInterfaceFn> {
    Ok(match get_query_interface() {
        Some(query_interface) => query_interface,
        None => {
            let query_interface = resolve_query_interface()?;
            unsafe { set_query_interface(query_interface) };
            query_interface
        },
    })
}

pub fn resolve_query_interface() -> io::Result<QueryInterfaceFn> {
    #[allow(unreachable_patterns)]
    match () {
        // Since NVIDIA driver version 525 libnvidia-api.so.1 implements NVAPI
        // but the implementation is still poor (many functions are not implemented)
        #[cfg(target_os = "linux")]
        () => {
            use libc::{RTLD_LAZY, RTLD_LOCAL, dlopen, dlsym};
            use std::os::raw::c_char;

            fn dlerror() -> io::Error {
                use libc::dlerror;

                let message = unsafe { CStr::from_ptr(dlerror()) };
                let message: String = message.to_string_lossy().into();
                io::Error::new(io::ErrorKind::NotFound, message)
            }

            let lib = unsafe { dlopen(LIBRARY_NAME.as_ptr() as *const c_char, RTLD_LAZY | RTLD_LOCAL) };
            let lib = NonNull::new(lib).ok_or_else(|| dlerror())?;
            let ptr = unsafe { dlsym(lib.as_ptr(), FN_NAME.as_ptr() as *const c_char) };
            NonNull::new(ptr)
                .ok_or_else(|| dlerror())
                .map(|ptr| unsafe { transmute(ptr) })
        },
        #[cfg(windows)]
        () => {
            use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
            use std::os::raw::c_char;

            let lib = unsafe { LoadLibraryA(LIBRARY_NAME.as_ptr() as *const c_char) };
            let lib = NonNull::new(lib).ok_or_else(|| io::Error::last_os_error())?;
            let ptr = unsafe { GetProcAddress(lib.as_ptr(), FN_NAME.as_ptr() as *const c_char) };
            NonNull::new(ptr)
                .ok_or_else(|| io::Error::last_os_error())
                .map(|ptr| unsafe { transmute(ptr) })
        },
        // TODO: Apparently nvapi is available for macOS?
        () => Err(io::Error::new(io::ErrorKind::Unsupported, Status::LibraryNotFound)),
    }
}

pub fn nvapi_QueryInterface(id: u32) -> Result<NonNull<()>, Status> {
    let query_interface = load_query_interface().map_err(|_e| {
        #[cfg(feature = "log")] {
            let fn_name = FN_NAME.to_str().unwrap();
            let library_name = LIBRARY_NAME.to_str().unwrap();
            log::warn!("failed to load {fn_name} from {library_name}: {_e:?}");
        }
        Status::LibraryNotFound
    })?;

    NonNull::new(query_interface(id) as *mut ())
        .ok_or(Status::NoImplementation)
}

nvapi! {
    pub type InitializeFn = extern "C" fn() -> NvAPI_Status;

    /// This function initializes the NvAPI library (if not already initialized) but always increments the ref-counter.
    /// This must be called before calling other NvAPI_ functions.
    pub unsafe fn NvAPI_Initialize;
}

nvapi! {
    pub type UnloadFn = extern "C" fn() -> NvAPI_Status;

    /// Decrements the ref-counter and when it reaches ZERO, unloads NVAPI library.
    /// This must be called in pairs with NvAPI_Initialize.
    ///
    /// Unloading NvAPI library is not supported when the library is in a resource locked state.
    /// Some functions in the NvAPI library initiates an operation or allocates certain resources
    /// and there are corresponding functions available, to complete the operation or free the
    /// allocated resources. All such function pairs are designed to prevent unloading NvAPI library.
    ///
    /// For example, if NvAPI_Unload is called after NvAPI_XXX which locks a resource, it fails with
    /// NVAPI_ERROR. Developers need to call the corresponding NvAPI_YYY to unlock the resources,
    /// before calling NvAPI_Unload again.
    ///
    /// Note: By design, it is not mandatory to call NvAPI_Initialize before calling any NvAPI.
    /// When any NvAPI is called without first calling NvAPI_Initialize, the internal refcounter
    /// will be implicitly incremented. In such cases, calling NvAPI_Initialize from a different thread will
    /// result in incrementing the refcount again and the user has to call NvAPI_Unload twice to
    /// unload the library. However, note that the implicit increment of the refcounter happens only once.
    /// If the client wants unload functionality, it is recommended to always call NvAPI_Initialize and NvAPI_Unload in pairs.
    pub unsafe fn NvAPI_Unload;
}

nvapi! {
    pub type GetErrorMessageFn = extern "C" fn(nr: NvAPI_Status, szDesc: *mut NvAPI_ShortString) -> NvAPI_Status;

    /// This function converts an NvAPI error code into a null terminated string.
    pub unsafe fn NvAPI_GetErrorMessage;
}

nvapi! {
    pub type GetInterfaceVersionStringFn = extern "C" fn(szDesc: *mut NvAPI_ShortString) -> NvAPI_Status;

    /// This function returns a string describing the version of the NvAPI library.
    /// The contents of the string are human readable.  Do not assume a fixed format.
    pub unsafe fn NvAPI_GetInterfaceVersionString;
}
