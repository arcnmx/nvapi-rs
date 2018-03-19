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

    pub fn into_inner(self) -> T {
        self.0
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

impl Default<T: SliceDefault> for Array<T> {
    fn default() -> Self {
        Array(SliceDefault::default())
    }
}

pub trait Slice {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
    fn as_slice_mut(&mut self) -> &mut [Self::Item];
}

pub trait SliceDefault: Sized + Slice where Self::Item: Default {
    fn default() -> Self;
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
    ([$($tt:tt)*] @nodefault) => {
        debug_array_impl! { @slice [$($tt)*] }
        debug_array_impl! { @impl [$($tt)*] }
    };
    ([$($tt:tt)*]) => {
        debug_array_impl! { @default [$($tt)*] }
        debug_array_impl! { @slice [$($tt)*] }
        debug_array_impl! { @impl [$($tt)*] }
    };
    (@default [$ty: ty; $v: expr]) => {
        impl ::debug_array::SliceDefault for [$ty; $v] {
            //fn default() -> Self where Self::Item: Default, Self: Sized {
            fn default() -> Self {
                [Self::Item::default(); $v]
            }
        }
    };
    (@slice [$ty: ty; $v: expr]) => {
        impl ::debug_array::Slice for [$ty; $v] {
            type Item = $ty;

            fn as_slice(&self) -> &[Self::Item] {
                &self[..]
            }

            fn as_slice_mut(&mut self) -> &mut [Self::Item] {
                &mut self[..]
            }
        }
    };
    (@impl $array:ty) => {
        impl ::debug_array::DebugArray for $array {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(&self[..], f)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for ::debug_array::Array<$array> {
            fn deserialize<D: ::serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                unimplemented!()
            }
        }

        impl ::serde::Serialize for ::debug_array::Array<$array> {
            fn serialize<S: ::serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                unimplemented!()
            }
        }
    };
}
