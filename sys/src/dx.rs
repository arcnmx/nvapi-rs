use winapi::um::unknwnbase::IUnknown;

use status::NvAPI_Status;

nv_declare_handle! { NVDX_ObjectHandle }
pub const NVDX_OBJECT_NONE: NVDX_ObjectHandle = NVDX_ObjectHandle(0 as *const _);

nvapi! {
    pub type D3D_GetObjectHandleForResourceFn = extern "C" fn(pDevice: *const IUnknown, pResource: *const IUnknown, pHandle: *mut NVDX_ObjectHandle) -> NvAPI_Status;

    /// This API gets a handle to a resource.
    pub unsafe fn NvAPI_D3D_GetObjectHandleForResource;
}

