use std::mem::transmute;
use std::ops::RangeInclusive;
use std::collections::BTreeMap;
use std::{fmt, ops, cmp, iter};
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};
use crate::sys::{ArgumentRangeError, tagged::TaggedData};
use crate::sys::version::{VersionedStructField, VersionedStruct, StructVersion, NvVersion};
use crate::sys::nvid::Api;

pub use crate::sys::{BoolU32, NvValue};
pub use crate::sys::value::{NvValueEnum, NvValueBits, NvValueData};

pub trait RawConversion {
    type Target;
    type Error;
}

macro_rules! unit_wrapper {
    (
        $(#[$meta:meta])*
        pub struct $name:ident($repr:ty);
    ) => {
        $(#[$meta])*
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
        pub struct $name(pub(crate) $repr);

        impl $name {
            pub const fn new(value: $repr) -> Self {
                Self(value)
            }

            pub const fn value(self) -> $repr {
                self.0
            }

            pub const fn value_ref(&self) -> &$repr {
                &self.0
            }

            pub fn value_mut(&mut self) -> &mut $repr {
                &mut self.0
            }
        }

        impl From<$repr> for $name {
            fn from(v: $repr) -> Self {
                $name(v)
            }
        }

        impl From<$name> for $repr {
            fn from(v: $name) -> Self {
                v.0
            }
        }

        impl<'a> From<&'a $name> for $name {
            fn from(v: &'a $name) -> Self {
                *v
            }
        }

        impl<'a> From<&'a $name> for &'a $repr {
            fn from(v: &'a $name) -> Self {
                v.value_ref()
            }
        }

        impl<'a> From<&'a mut $name> for &'a mut $repr {
            fn from(v: &'a mut $name) -> Self {
                v.value_mut()
            }
        }

        impl<'a> From<&'a $repr> for &'a $name {
            fn from(v: &'a $repr) -> Self {
                unsafe {
                    transmute(v)
                }
            }
        }

        impl<'a> From<&'a mut $repr> for &'a mut $name {
            fn from(v: &'a mut $repr) -> Self {
                unsafe {
                    transmute(v)
                }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Display::fmt(self, f)
            }
        }
    };
}

unit_wrapper! {
    pub struct Celsius(i32);
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}C", self.0)
    }
}

unit_wrapper! {
    pub struct Rpm(u32);
}

impl fmt::Display for Rpm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} RPM", self.0)
    }
}

unit_wrapper! {
    /// Nvidia encodes temperature as `<< 8` for some reason sometimes.
    pub struct CelsiusShifted(i32);
}

impl CelsiusShifted {
    pub fn get(&self) -> i32 {
        self.0 >> 8
    }
}

impl fmt::Display for CelsiusShifted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}C", self.get())
    }
}

impl From<CelsiusShifted> for Celsius {
    fn from(c: CelsiusShifted) -> Self {
        Celsius(c.get())
    }
}

impl From<Celsius> for CelsiusShifted {
    fn from(c: Celsius) -> Self {
        CelsiusShifted(c.0 << 8)
    }
}

impl From<u32> for CelsiusShifted {
    fn from(c: u32) -> Self {
        CelsiusShifted(c.try_into().expect("valid temperature value"))
    }
}

impl From<CelsiusShifted> for u32 {
    fn from(c: CelsiusShifted) -> Self {
        c.0.try_into().expect("positive temperature value")
    }
}

unit_wrapper! {
    pub struct Microvolts(u32);
}

impl fmt::Display for Microvolts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0 as f32 / 1000.0;
        if let Some(precision) = f.precision() {
            write!(f, "{:.*} mV", precision, value)
        } else {
            write!(f, "{} mV", value)
        }
    }
}

unit_wrapper! {
    pub struct MicrovoltsDelta(i32);
}

impl fmt::Display for MicrovoltsDelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0 as f32 / 1000.0;
        if let Some(precision) = f.precision() {
            write!(f, "{:.*} mV", precision, value)
        } else {
            write!(f, "{} mV", value)
        }
    }
}

unit_wrapper! {
    pub struct Kilohertz(u32);
}

impl fmt::Display for Kilohertz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 1000 {
            write!(f, "{} kHz", self.0)
        } else {
            let value = self.0 as f32 / 1000.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} MHz", precision, value)
            } else {
                write!(f, "{} MHz", value)
            }
        }
    }
}

impl ops::Sub for Kilohertz {
    type Output = KilohertzDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        KilohertzDelta(self.0 as i32 - rhs.0 as i32)
    }
}

impl ops::Sub<KilohertzDelta> for Kilohertz {
    type Output = Kilohertz;

    fn sub(self, rhs: KilohertzDelta) -> Self::Output {
        Kilohertz((self.0 as i32 - rhs.0 as i32) as u32)
    }
}

impl ops::Add<KilohertzDelta> for Kilohertz {
    type Output = Kilohertz;

    fn add(self, rhs: KilohertzDelta) -> Self::Output {
        Kilohertz((self.0 as i32 + rhs.0 as i32) as u32)
    }
}

unit_wrapper! {
    pub struct Kilohertz2(u32);
}

impl Kilohertz2 {
    pub fn get(&self) -> u32 {
        self.0 / 2
    }
}

impl From<Kilohertz2> for Kilohertz {
    fn from(p: Kilohertz2) -> Self {
        Kilohertz(p.get())
    }
}

impl From<Kilohertz> for Kilohertz2 {
    fn from(v: Kilohertz) -> Self {
        Kilohertz2(v.0 * 2)
    }
}

impl fmt::Display for Kilohertz2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.get();
        if v < 1000 {
            write!(f, "{} kHz", v)
        } else {
            let value = self.0 as f32 / 1000.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} MHz", precision, value)
            } else {
                write!(f, "{} MHz", value)
            }
        }
    }
}

unit_wrapper! {
    pub struct KilohertzDelta(i32);
}

impl fmt::Display for KilohertzDelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.0).abs() < 1000 {
            write!(f, "{} kHz", self.0)
        } else {
            let value = self.0 as f32 / 1000.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} MHz", precision, value)
            } else {
                write!(f, "{} MHz", value)
            }
        }
    }
}

impl ops::Add for KilohertzDelta {
    type Output = KilohertzDelta;

    fn add(self, rhs: Self) -> Self::Output {
        KilohertzDelta(self.0 + rhs.0)
    }
}

impl ops::Sub for KilohertzDelta {
    type Output = KilohertzDelta;

    fn sub(self, rhs: Self) -> Self::Output {
        KilohertzDelta(self.0 - rhs.0)
    }
}

impl ops::Mul<i32> for KilohertzDelta {
    type Output = KilohertzDelta;

    fn mul(self, rhs: i32) -> Self::Output {
        KilohertzDelta(self.0 * rhs)
    }
}

impl ops::Div<i32> for KilohertzDelta {
    type Output = KilohertzDelta;

    fn div(self, rhs: i32) -> Self::Output {
        KilohertzDelta(self.0 / rhs)
    }
}

unit_wrapper! {
    pub struct Kilohertz2Delta(i32);
}

impl Kilohertz2Delta {
    pub fn get(&self) -> i32 {
        self.0 / 2
    }
}

impl From<Kilohertz2Delta> for KilohertzDelta {
    fn from(p: Kilohertz2Delta) -> Self {
        KilohertzDelta(p.get())
    }
}

impl From<KilohertzDelta> for Kilohertz2Delta {
    fn from(v: KilohertzDelta) -> Self {
        Kilohertz2Delta(v.0 * 2)
    }
}

impl fmt::Display for Kilohertz2Delta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = self.get();
        if v.abs() < 1000 {
            write!(f, "{} kHz", v)
        } else {
            let value = self.0 as f32 / 1000.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} MHz", precision, value)
            } else {
                write!(f, "{} MHz", value)
            }
        }
    }
}

unit_wrapper! {
    pub struct Kibibytes(u32);
}

impl ops::Sub for Kibibytes {
    type Output = Kibibytes;

    fn sub(self, rhs: Self) -> Self::Output {
        Kibibytes(self.0 - rhs.0)
    }
}

impl fmt::Display for Kibibytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 1000 {
            write!(f, "{} KiB", self.0)
        } else if self.0 < 1000000 {
            let value = self.0 as f32 / 1024.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} MiB", precision, value)
            } else {
                write!(f, "{} MiB", value)
            }
        } else {
            let value = self.0 as f32 / 1048576.0;
            if let Some(precision) = f.precision() {
                write!(f, "{:.*} GiB", precision, value)
            } else {
                write!(f, "{} GiB", value)
            }
        }
    }
}

unit_wrapper! {
    pub struct Percentage(u32);
}

impl Percentage {
    pub fn from_raw(v: u32) -> Result<Self, ArgumentRangeError> {
        match v {
            v @ 0..=100 => Ok(Percentage(v)),
            _ => Err(ArgumentRangeError),
        }
    }
}

impl fmt::Display for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.0)
    }
}

unit_wrapper! {
    pub struct Percentage1000(u32);
}

impl Percentage1000 {
    pub fn get(&self) -> u32 {
        self.0 / 1000
    }
}

impl fmt::Display for Percentage1000 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0 as f32 / 1000.0;
        if let Some(precision) = f.precision() {
            write!(f, "{:.*}%", precision, value)
        } else {
            write!(f, "{}%", value)
        }
    }
}

impl From<Percentage1000> for Percentage {
    fn from(p: Percentage1000) -> Self {
        Percentage(p.get())
    }
}

impl From<Percentage> for Percentage1000 {
    fn from(p: Percentage) -> Self {
        Percentage1000(p.0 * 1000)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}

impl<T> Range<T> {
    pub const fn new(min: T, max: T) -> Self {
        Self {
            min,
            max,
        }
    }

    pub const fn with_ref(minmax: &[T; 2]) -> &Self {
        unsafe {
            transmute(minmax)
        }
    }

    pub fn with_mut(minmax: &mut [T; 2]) -> &mut Self {
        unsafe {
            transmute(minmax)
        }
    }

    pub fn map<O, F: Fn(T) -> O>(self, f: F) -> Range<O> {
        Range::new(f(self.min), f(self.max))
    }
}

impl<T: fmt::Display> fmt::Display for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ~ {}", self.min, self.max)
    }
}

impl<T: fmt::Debug> fmt::Debug for Range<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} ~ {:?}", self.min, self.max)
    }
}

impl<T> Range<T> {
    pub fn range_from<U>(r: Range<U>) -> Self where T: From<U> {
        Range {
            min: r.min.into(),
            max: r.max.into(),
        }
    }

    pub fn from_scalar(v: T) -> Self where T: Clone {
        Range {
            min: v.clone(),
            max: v,
        }
    }

    pub fn range(&self) -> RangeInclusive<T> where T: Clone{
        self.min.clone()..=self.max.clone()
    }
}

/*#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Delta<T> {
    pub value: T,
    pub range: Range<T>,
}*/

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Tagged<I, T> {
    pub tag: I,
    pub value: T,
}

impl<I, T> Tagged<I, T> {
    pub const fn new(tag: I, value: T) -> Self {
        Self {
            tag,
            value,
        }
    }

    pub fn into_tuple(self) -> (I, T) {
        (self.tag, self.value)
    }

    pub fn into_value(self) -> T {
        self.value
    }
}

impl<I, T> FromIterator<Tagged<I, T>> for BTreeMap<I, T> where
    BTreeMap<I, T>: FromIterator<(I, T)>,
{
    fn from_iter<II: IntoIterator<Item = Tagged<I, T>>>(iter: II) -> Self {
        Self::from_iter(iter.into_iter().map(Tagged::into_tuple))
    }
}

impl<I, T> FromIterator<Tagged<I, T>> for Vec<T> where
    Vec<T>: FromIterator<T>,
{
    fn from_iter<II: IntoIterator<Item = Tagged<I, T>>>(iter: II) -> Self {
        Self::from_iter(iter.into_iter().map(Tagged::into_value))
    }
}

impl<T: TaggedData> Tagged<T::Repr, T> {
    pub fn with_value(value: T) -> Self {
        Self {
            tag: value.tag(),
            value,
        }
    }
}

impl<T: TaggedData> Tagged<T::Id, T> {
    pub fn try_with_value(value: T) -> Result<Self, <T::Repr as TryInto<T::Id>>::Error> {
        value.tag().try_into().map(|tag| Self {
            tag,
            value,
        })
    }
}

impl<I: Copy + cmp::Ord, T> TaggedData for Tagged<I, T> {
    type Repr = I;
    type Id = I;

    fn tag(&self) -> Self::Repr {
        self.tag
    }
}

impl<I, T> Into<(I, T)> for Tagged<I, T> {
    fn into(self) -> (I, T) {
        self.into_tuple()
    }
}

impl<I, II: Into<I>, T, TI: Into<T>> From<(II, TI)> for Tagged<I, T> {
    fn from(tuple: (II, TI)) -> Self {
        Self {
            tag: tuple.0.into(),
            value: tuple.1.into(),
        }
    }
}

impl<I, T> ops::Deref for Tagged<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<I, T> ops::DerefMut for Tagged<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct Map<I: Ord, T> {
    pub values: BTreeMap<I, T>,
}

/*impl<I, T> ops::Deref for Mapped<I, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<I, T> ops::DerefMut for Tagged<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}*/
impl<I: Ord, T> FromIterator<Tagged<I, T>> for Map<I, T> {
    fn from_iter<II: IntoIterator<Item = Tagged<I, T>>>(iter: II) -> Self {
        Self {
            values: iter.into_iter().map(|tag| (tag.tag, tag.value)).collect(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Hash, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct List<T> {
    pub values: Vec<T>,
}

pub trait TaggedIterator: Sized + IntoIterator {
    fn tagged(self) -> iter::Map<<Self as IntoIterator>::IntoIter, fn(<Self as IntoIterator>::Item) -> (<<Self as IntoIterator>::Item as TaggedData>::Id, <Self as IntoIterator>::Item)> where
        <Self as IntoIterator>::Item: TaggedData,
        <<Self as IntoIterator>::Item as TaggedData>::Repr: fmt::Debug,
    {
        fn map_tagged<T: TaggedData>(value: T) -> (T::Id, T) where
            T::Repr: fmt::Debug,
        {
            let tag = value.tag();
            let id = match tag.try_into() {
                Ok(id) => id,
                Err(..) => panic!("unknown nvapi enum value {:?}", tag),
            };
            (id, value)
        }
        self.into_iter().map(map_tagged::<<Self as IntoIterator>::Item>)
    }

    fn into_vec(self) -> Vec<<Self as IntoIterator>::Item> {
        self.into_iter().collect()
    }

    fn into_map(self) -> BTreeMap<<<Self as IntoIterator>::Item as TaggedData>::Id, <Self as IntoIterator>::Item> where
        <Self as IntoIterator>::Item: TaggedData,
        <<Self as IntoIterator>::Item as TaggedData>::Repr: fmt::Debug,
    {
        self.tagged().into_iter().collect()
    }
}

impl<T: IntoIterator> TaggedIterator for T { }

pub(crate) type MappedList<T> = Vec<T>;

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, zerocopy::FromBytes)]
#[repr(transparent)]
pub struct NvData<T> {
    sys: T,
}

impl<T> NvData<T> {
    pub const fn with_sys(sys: T) -> Self {
        Self {
            sys,
        }
    }

    pub const fn with_sys_ref(sys: &T) -> &Self {
        unsafe {
            transmute(sys)
        }
    }

    pub fn with_sys_mut(sys: &mut T) -> &mut Self {
        unsafe {
            transmute(sys)
        }
    }

    pub const fn sys(&self) -> &T {
        &self.sys
    }

    pub fn sys_mut(&mut self) -> &mut T {
        &mut self.sys
    }

    pub fn into_sys(self) -> T {
        self.sys
    }
}

impl<T> TaggedData for NvData<T> where T: TaggedData {
    type Repr = <T as TaggedData>::Repr;
    type Id = <T as TaggedData>::Id;

    fn tag(&self) -> Self::Repr {
        <T as TaggedData>::tag(self.sys())
    }
}

impl<T: VersionedStructField> VersionedStructField for NvData<T> {
    fn nvapi_version_ref(&self) -> &NvVersion {
        self.sys().nvapi_version_ref()
    }

    fn nvapi_version_mut(&mut self) -> &mut NvVersion {
        self.sys_mut().nvapi_version_mut()
    }
}

impl<const N: u16, T: StructVersion<N>> StructVersion<N> for NvData<T> where
    Self: VersionedStruct,
{
    const NVAPI_VERSION: NvVersion = T::NVAPI_VERSION;
    const API: Api = T::API;
    const API_SET: Option<Api> = T::API_SET;
    type Storage = T::Storage;

    /*fn storage_ref(storage: &Self::Storage) -> Option<&Self> {
        T::storage_ref(storage).map(Self::with_sys_ref)
    }

    fn storage_mut(storage: &mut Self::Storage) -> Option<&mut Self> {
        T::storage_mut(storage).map(Self::with_sys_mut)
    }*/
}

impl<T> From<T> for NvData<T> {
    fn from(sys: T) -> Self {
        Self::with_sys(sys)
    }
}

impl<'a, T> From<&'a T> for &'a NvData<T> {
    fn from(sys: &'a T) -> Self {
        NvData::with_sys_ref(sys)
    }
}

impl<'a, T> From<&'a mut T> for &'a mut NvData<T> {
    fn from(sys: &'a mut T) -> Self {
        NvData::with_sys_mut(sys)
    }
}

pub trait IsNvData: self::sealed::Sealed {
    type Data;

    fn sys(&self) -> &Self::Data;
}

impl<T> IsNvData for NvData<T> {
    type Data = T;

    fn sys(&self) -> &Self::Data {
        NvData::<T>::sys(self)
    }
}

impl<T> self::sealed::Sealed for NvData<T> { }

pub(crate) mod sealed {
    pub trait Sealed { }
}
