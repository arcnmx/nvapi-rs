use std::ops;
use std::mem::transmute;
use std::hash::{Hash, Hasher};
use std::{fmt, iter};
use zerocopy::{AsBytes, FromBytes};
use crate::ArgumentRangeError;

pub use nvapi_macros::{NvValueData, NvValueBits, NvValueEnum};

pub type NvEnum<T> = NvValue<T>;
pub type NvBits<T> = NvValue<T>;

pub trait NvValueData
    : Copy + PartialEq + Eq
    + Into<<Self as NvValueData>::Repr>
    + TryFrom<<Self as NvValueData>::Repr, Error = ArgumentRangeError>
{
    const NAME: &'static str;
    const C_NAME: &'static str;
    fn all_values() -> &'static [Self];

    type Repr: Copy + PartialEq + Eq + fmt::Display;

    fn values() -> iter::Copied<std::slice::Iter<'static, Self>> {
        Self::all_values().iter().copied()
    }

    fn repr(self) -> Self::Repr;
    fn repr_ref(&self) -> &Self::Repr;
    fn from_repr(value: Self::Repr) -> Result<Self, ArgumentRangeError>;
    fn from_repr_ref(value: &Self::Repr) -> Result<&Self, ArgumentRangeError>;
    fn from_repr_mut(value: &mut Self::Repr) -> Result<&mut Self, ArgumentRangeError>;

    fn from_value(value: NvValue<Self>) -> Result<Self, ArgumentRangeError> where
        Self: TryFrom<NvValue<Self>, Error = ArgumentRangeError>,
    {
        value.try_into()
    }

    fn value(self) -> NvValue<Self> {
        NvValue::new(self)
    }
}

pub trait NvValueEnum: NvValueData {
}

pub trait NvValueBits: NvValueData {
    fn from_repr_truncate(value: Self::Repr) -> Self;
}

#[derive(FromBytes, PartialEq, Eq)]
#[repr(transparent)]
pub struct NvValue<T: NvValueData> {
    pub value: T::Repr,
}

impl<T: NvValueData> NvValue<T> {
    pub fn new(value: T) -> Self {
        Self::with_repr(value.repr())
    }

    pub const fn with_repr(value: T::Repr) -> Self {
        Self {
            value,
        }
    }

    pub const fn with_repr_ref(value: &T::Repr) -> &Self {
        unsafe {
            transmute(value)
        }
    }

    pub fn with_repr_mut(value: &mut T::Repr) -> &mut Self {
        unsafe {
            transmute(value)
        }
    }

    pub fn cast<U: NvValueData<Repr=T::Repr>>(self) -> NvValue<U> {
        NvValue::with_repr(self.repr())
    }

    pub fn get(self) -> T {
        match self.try_get() {
            Ok(v) => v,
            Err(..) => panic!("nvapi unknown value {} for {}", self.value, T::NAME),
        }
    }

    pub fn try_get(self) -> Result<T, ArgumentRangeError> {
        T::try_from(self.value)
    }

    pub fn try_ref(&self) -> Result<&T, ArgumentRangeError> {
        T::from_repr_ref(self.repr_ref())
    }

    pub fn try_mut(&mut self) -> Result<&mut T, ArgumentRangeError> {
        T::from_repr_mut(self.repr_mut())
    }

    pub const fn repr(self) -> T::Repr {
        self.value
    }

    pub const fn repr_ref(&self) -> &T::Repr {
        &self.value
    }

    pub fn repr_mut(&mut self) -> &mut T::Repr {
        &mut self.value
    }

    pub fn display(&self) -> &dyn fmt::Display where
        T: fmt::Display,
    {
        match self.try_ref() {
            Ok(value) => value,
            Err(..) => self.repr_ref(),
        }
    }
}

impl<T: NvValueBits> NvValue<T> {
    pub fn truncate(&self) -> T {
        T::from_repr_truncate(self.repr())
    }
}

impl<T: NvValueData> From<T> for NvValue<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<'a, T: NvValueData> From<&'a T> for &'a NvValue<T> {
    fn from(value: &'a T) -> Self {
        NvValue::with_repr_ref(value.repr_ref())
    }
}

impl<'a, T: NvValueData> From<&'a NvValue<T>> for NvValue<T> {
    fn from(value: &'a NvValue<T>) -> Self {
        *value
    }
}

unsafe impl<T: NvValueData> AsBytes for NvValue<T> where
    T::Repr: AsBytes,
{
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

impl<T: NvValueData> Copy for NvValue<T> { }
impl<T: NvValueData> Clone for NvValue<T> where
    T::Repr: Clone,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: NvValueData> Default for NvValue<T> where
    T::Repr: Default,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

impl<T: NvValueData> PartialOrd for NvValue<T> where
    T::Repr: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T: NvValueData> Ord for NvValue<T> where
    T::Repr: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T: NvValueData> PartialEq<T> for NvValue<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other.repr_ref())
    }
}

impl<T: NvValueData> Hash for NvValue<T> where
    T::Repr: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T: NvValueData> fmt::Display for NvValue<T> where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match T::try_from(self.value) {
            Ok(value) => fmt::Display::fmt(&value, f),
            Err(..) => fmt::Display::fmt(&self.value, f),
        }
    }
}

impl<T: NvValueData> fmt::Debug for NvValue<T> where
    T: fmt::Debug,
    T::Repr: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut debug;
        match T::try_from(self.value) {
            Ok(value) => {
                debug = f.debug_tuple(T::NAME);
                debug.field(&value);
            },
            Err(..) => {
                debug = f.debug_tuple(T::C_NAME);
            },
        }
        debug.field(&self.value).finish()
    }
}

impl<T: NvValueData> fmt::LowerHex for NvValue<T> where
    T::Repr: fmt::LowerHex,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.repr(), f)
    }
}

impl<T: NvValueData> fmt::UpperHex for NvValue<T> where
    T::Repr: fmt::UpperHex,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.repr(), f)
    }
}

impl<T: NvValueData> ops::Not for NvValue<T> where
    T::Repr: ops::Not,
    <T::Repr as ops::Not>::Output: Into<T::Repr>,
{
    type Output = Self;
    fn not(self) -> Self::Output {
        let value = !self.repr();
        Self::with_repr(value.into())
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitOr<Rhs> for NvValue<T> where
    T::Repr: ops::BitOr,
    <T::Repr as ops::BitOr>::Output: Into<T::Repr>,
{
    type Output = Self;
    fn bitor(self, rhs: Rhs) -> Self::Output {
        let value = self.repr() | rhs.into();
        Self::with_repr(value.into())
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitAnd<Rhs> for NvValue<T> where
    T::Repr: ops::BitAnd,
    <T::Repr as ops::BitAnd>::Output: Into<T::Repr>,
{
    type Output = Self;
    fn bitand(self, rhs: Rhs) -> Self::Output {
        let value = self.repr() & rhs.into();
        Self::with_repr(value.into())
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitXor<Rhs> for NvValue<T> where
    T::Repr: ops::BitXor,
    <T::Repr as ops::BitXor>::Output: Into<T::Repr>,
{
    type Output = Self;
    fn bitxor(self, rhs: Rhs) -> Self::Output {
        let value = self.repr() ^ rhs.into();
        Self::with_repr(value.into())
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitOrAssign<Rhs> for NvValue<T> where
    T::Repr: ops::BitOrAssign,
{
    fn bitor_assign(&mut self, rhs: Rhs) {
        self.value |= rhs.into();
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitAndAssign<Rhs> for NvValue<T> where
    T::Repr: ops::BitAndAssign,
{
    fn bitand_assign(&mut self, rhs: Rhs) {
        self.value &= rhs.into();
    }
}

impl<T: NvValueData, Rhs: Into<T::Repr>> ops::BitXorAssign<Rhs> for NvValue<T> where
    T::Repr: ops::BitXorAssign,
{
    fn bitxor_assign(&mut self, rhs: Rhs) {
        self.value ^= rhs.into();
    }
}

#[cfg(feature = "serde")]
mod serde_impl_nvenum {
    use serde::{Serialize, Serializer, Deserialize, Deserializer};
    use super::{NvValue, NvValueData};

    impl<'de, T: NvValueData> Deserialize<'de> for NvValue<T> where
        T::Repr: Deserialize<'de>,
    {
        fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
            Deserialize::deserialize(de)
                .map(Self::with_repr)
        }
    }

    impl<T: NvValueData> Serialize for NvValue<T> where
        T::Repr: Serialize,
    {
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            self.value.serialize(ser)
        }
    }
}

macro_rules! nvvalue_reprs {
    ($($repr:ty,)*) => { $(
        impl<T: NvValueData<Repr=$repr>> Into<$repr> for NvValue<T> {
            fn into(self) -> $repr {
                self.repr()
            }
        }

        impl<T: NvValueData<Repr=$repr>> From<$repr> for NvValue<T> {
            fn from(value: T::Repr) -> Self {
                Self::with_repr(value)
            }
        }

        impl<T: NvValueData<Repr=$repr>> PartialEq<$repr> for NvValue<T> {
            fn eq(&self, other: &T::Repr) -> bool {
                self.value.eq(other)
            }
        }
    )* };
}

nvvalue_reprs! {
    u32, i32,
}
