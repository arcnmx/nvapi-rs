
#[derive(Debug)]
pub struct GSyncDevice(sys::handles::NvGSyncDeviceHandle);

impl GSyncDevice {
    pub fn handle(&self) -> &sys::handles::NvGSyncDeviceHandle {
        &self.0
    }
}