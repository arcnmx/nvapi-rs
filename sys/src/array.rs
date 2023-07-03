use std::ops::{Deref, DerefMut};
use std::fmt;
use zerocopy::{AsBytes, FromBytes};
use crate::ArgumentRangeError;

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Array<T> {
    pub data: T,
}

impl<T> Array<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data,
        }
    }

    pub fn into_data(self) -> T {
        self.data
    }
}

impl<T> From<T> for Array<T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

impl<T> Deref for Array<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: IntoIterator> IntoIterator for Array<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Array<T> where &'a T: IntoIterator {
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Array<T> where &'a mut T: IntoIterator {
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

unsafe impl<T: AsBytes> AsBytes for Array<T> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

unsafe impl<T: FromBytes> FromBytes for Array<T> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

impl<T: FromBytes, const N: usize> Default for Array<[T; N]> {
    fn default() -> Self {
        FromBytes::new_zeroed()
    }
}

fn all_zero<T: AsBytes>(v: &T) -> bool {
    v.as_bytes().iter().all(|&v| v == 0)
}

impl<T: AsBytes, const N: usize> Array<[T; N]> {
    pub fn all_zero(&self) -> bool {
        all_zero(self)
    }

    pub fn check_zero(&self) -> Result<(), ArgumentRangeError> {
        match self.all_zero() {
            true => Ok(()),
            false => Err(ArgumentRangeError),
        }
    }
}

impl<T: AsBytes + fmt::Debug, const N: usize> fmt::Debug for Array<[T; N]> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut it = self.data.iter();
        f.write_str("[")?;
        let mut prev: Option<&T> = None;
        let mut repeat: usize = 0;
        while let Some(v) = it.next() {
            match prev {
                Some(prev) if prev.as_bytes() == v.as_bytes() =>
                    repeat = repeat.saturating_add(1),
                _ => {
                    if repeat > 1 {
                        write!(f, ";{}, ", repeat)?;
                    } else if repeat == 1 {
                        f.write_str(", ")?;
                    }

                    if all_zero(v) {
                        f.write_str("0")?;
                    } else {
                        fmt::Debug::fmt(&v, f)?;
                    }

                    prev = Some(v);
                    repeat = 1;
                },
            }
        }
        if repeat > 1 {
            write!(f, ";{}", repeat)?;
        }
        f.write_str("]")
    }
}
