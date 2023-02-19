use std::sync::atomic::{AtomicUsize, Ordering};
use std::os::raw::c_void;
use crate::status::{Status, NvAPI_Status};
use crate::types;

pub type QueryInterfaceFn = extern "C" fn(id: u32) -> *const c_void;

#[cfg(all(windows, target_pointer_width = "32"))]
pub const LIBRARY_NAME: &'static [u8; 10] = b"nvapi.dll\0";
#[cfg(all(windows, target_pointer_width = "64"))]
pub const LIBRARY_NAME: &'static [u8; 12] = b"nvapi64.dll\0";
#[cfg(target_os = "linux")]
pub const LIBRARY_NAME: &'static [u8; 19] = b"libnvidia-api.so.1\0";

pub const FN_NAME: &'static [u8; 21] = b"nvapi_QueryInterface\0";

static QUERY_INTERFACE_CACHE: AtomicUsize = AtomicUsize::new(0);

pub unsafe fn set_query_interface(ptr: QueryInterfaceFn) {
    QUERY_INTERFACE_CACHE.store(ptr as usize, Ordering::Relaxed);
}

// Since v525 NVIDIA drivers have libnvidia-api.so.1 which implements NVAPI but the implementation is still poor
// (many functions are not there, like it's impossible to identify physical handler by pci slot etc)
#[cfg(target_os = "linux")]
pub fn nvapi_QueryInterface(id: u32) -> crate::Result<usize> {
    use libc::{ dlopen, dlsym };
    use std::os::raw::{ c_char, c_int};
    use std::mem;

    const RTLD_LAZY: c_int = 0x00001;
    const RTLD_LOCAL: c_int = 0;

    unsafe {
        let ptr = match QUERY_INTERFACE_CACHE.load(Ordering::Relaxed) {
            0 => {
                let lib = dlopen(LIBRARY_NAME.as_ptr() as *const c_char,RTLD_LAZY | RTLD_LOCAL);
                if lib.is_null() {
                    Err(Status::LibraryNotFound)
                } else {
                    let ptr = dlsym(lib, FN_NAME.as_ptr() as *const c_char);
                    if ptr.is_null() {
                        Err(Status::LibraryNotFound)
                    } else {
                        QUERY_INTERFACE_CACHE.store(ptr as usize, Ordering::Relaxed);
                        Ok(ptr as usize)
                    }
                }
            },
            ptr => Ok(ptr),
        }?;

        match mem::transmute::<_, QueryInterfaceFn>(ptr)(id) as usize {
            0 => Err(Status::NoImplementation),
            ptr => Ok(ptr),
        }
    }
}

#[cfg(macos)]
pub fn nvapi_QueryInterface(id: u32) -> crate::Result<usize> {
    // TODO: Apparently nvapi is available for macOS?
    Err(Status::LibraryNotFound)
}



#[cfg(windows)]
pub fn nvapi_QueryInterface(id: u32) -> crate::Result<usize> {
    use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
    use std::mem;
    use std::os::raw::c_char;

    unsafe {
        let ptr = match QUERY_INTERFACE_CACHE.load(Ordering::Relaxed) {
            0 => {
                let lib = LoadLibraryA(LIBRARY_NAME.as_ptr() as *const c_char);
                if lib.is_null() {
                    Err(Status::LibraryNotFound)
                } else {
                    let ptr = GetProcAddress(lib, FN_NAME.as_ptr() as *const c_char);
                    if ptr.is_null() {
                        Err(Status::LibraryNotFound)
                    } else {
                        QUERY_INTERFACE_CACHE.store(ptr as usize, Ordering::Relaxed);
                        Ok(ptr as usize)
                    }
                }
            },
            ptr => Ok(ptr),
        }?;

        match mem::transmute::<_, QueryInterfaceFn>(ptr)(id) as usize {
            0 => Err(Status::NoImplementation),
            ptr => Ok(ptr),
        }
    }
}

pub(crate) fn query_interface(id: u32, cache: &AtomicUsize) -> crate::Result<usize> {
    match cache.load(Ordering::Relaxed) {
        0 => {
            let value = nvapi_QueryInterface(id)?;
            cache.store(value, Ordering::Relaxed);
            Ok(value)
        },
        value => Ok(value),
    }
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
    pub type GetErrorMessageFn = extern "C" fn(nr: NvAPI_Status, szDesc: *mut types::NvAPI_ShortString) -> NvAPI_Status;

    /// This function converts an NvAPI error code into a null terminated string.
    pub unsafe fn NvAPI_GetErrorMessage;
}

nvapi! {
    pub type GetInterfaceVersionStringFn = extern "C" fn(szDesc: *mut types::NvAPI_ShortString) -> NvAPI_Status;

    /// This function returns a string describing the version of the NvAPI library.
    /// The contents of the string are human readable.  Do not assume a fixed format.
    pub unsafe fn NvAPI_GetInterfaceVersionString;
}

