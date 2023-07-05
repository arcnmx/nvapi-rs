use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::{self, NonNull};
use std::fmt;
use crate::nvapi::nvapi_QueryInterface;
use crate::status::Status;
use crate::nvid::Api;

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

    pub unsafe fn query<F: NvapiInterface>(&self) -> Result<F::Fn, Status> {
        self.query_ptr(F::API.id())
            .map(|ptr| F::fn_from_ptr(ptr))
    }
}

impl fmt::Debug for NvapiFnCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NvapiFnCache")
            .field(&self.get())
            .finish()
    }
}

#[derive(Debug)]
pub struct NvapiFn<I> {
    cache: NvapiFnCache,
    _nvapi: PhantomData<I>,
}

impl<I> NvapiFn<I> {
    pub const fn empty() -> Self {
        Self {
            cache: NvapiFnCache::empty(),
            _nvapi: PhantomData,
        }
    }

    pub const unsafe fn cache_ref(&self) -> &NvapiFnCache {
        &self.cache
    }
}

impl<I: NvapiInterface> NvapiFn<I> {
    pub fn query(&self) -> Result<I::Fn, Status> {
        unsafe {
            self.cache.query::<I>()
        }
    }

    pub fn query_map<R, F: FnOnce(I::Fn) -> R>(&self, f: F) -> R where
        Status: Into<R>,
    {
        let interface = match self.query() {
            Ok(i) => i,
            Err(e) => return e.into(),
        };
        f(interface)
    }
}

impl<I> Default for NvapiFn<I> {
    fn default() -> Self {
        Self::empty()
    }
}

pub trait NvapiInterface {
    const API: Api;
    type Fn: Copy;

    unsafe fn fn_from_ptr(ptr: NonNull<()>) -> Self::Fn;
}
