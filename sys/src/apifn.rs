use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::{self, NonNull};
use std::fmt;
use crate::nvapi::nvapi_QueryInterface;
use crate::status::Status;

#[derive(Default)]
#[repr(transparent)]
pub struct NvapiFnCache {
    cache: AtomicUsize,
}

impl NvapiFnCache {
    pub const fn new_usize(value: usize) -> Self {
        Self {
            cache: AtomicUsize::new(value),
        }
    }

    pub const fn empty() -> Self {
        Self::new_usize(0)
    }

    pub fn get(&self) -> Option<NonNull<()>> {
        NonNull::new(self.cache.load(Ordering::Relaxed) as *mut _)
    }

    pub fn set(&self, ptr: Option<NonNull<()>>) {
        let ptr = ptr.map(|p| p.as_ptr()).unwrap_or(ptr::null_mut());
        self.cache.store(ptr as usize, Ordering::Relaxed)
    }

    pub fn query_ptr(&self, interface: u32) -> Result<NonNull<()>, Status> {
        match self.get() {
            None => {
                let ptr = nvapi_QueryInterface(interface)?;
                self.set(Some(ptr));
                Ok(ptr)
            },
            Some(v) => Ok(v),
        }
    }
}

impl fmt::Debug for NvapiFnCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NvapiFnCache")
            .field(&self.get())
            .finish()
    }
}
