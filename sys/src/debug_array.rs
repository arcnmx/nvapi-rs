use std::{ops, fmt};

/// Hacks a `Debug` impl onto large arrays.
#[derive(Copy, Clone)]
pub struct Array<T>(pub T);

impl<T> Array<T> {
    pub fn inner(&self) -> &T {
        &self.0
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> ops::Deref for Array<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait DebugArray {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

impl<T: DebugArray> fmt::Debug for Array<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        T::fmt(self, f)
    }
}

macro_rules! debug_array_impl {
    ($array:ty) => {
        impl ::debug_array::DebugArray for $array {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self[..], f)
            }
        }
    };
}
