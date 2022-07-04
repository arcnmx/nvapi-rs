use std::borrow::Cow;
use std::mem::MaybeUninit;
use std::ops::{Deref, DerefMut};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::fmt;
use zerocopy::{AsBytes, FromBytes};
use crate::nvapi::NvVersion;

pub type NvBool = u8;

pub const NV_TRUE: NvBool = 1;
pub const NV_FALSE: NvBool = 0;

/// A boolean containing reserved bits
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, AsBytes, FromBytes)]
#[repr(transparent)]
pub struct BoolU32(pub u32);

impl BoolU32 {
    pub fn new(flag: bool, rest: u32) -> Self {
        let mut value = BoolU32(rest);
        value.set(flag);
        value
    }

    pub fn get(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn set(&mut self, value: bool) {
        self.0 = self.0 & 0xffffffe | if value { NV_TRUE } else { NV_FALSE } as u32
    }
}

impl From<bool> for BoolU32 {
    fn from(v: bool) -> Self {
        Self::new(v, 0)
    }
}

nvstruct! {
    pub struct NV_RECT {
        pub left: u32,
        pub top: u32,
        pub right: u32,
        pub bottom: u32,
    }
}

pub const NVAPI_GENERIC_STRING_MAX: usize = 4096;
pub const NVAPI_LONG_STRING_MAX: usize = 256;
pub const NVAPI_SHORT_STRING_MAX: usize = 64;

nvstruct! {
    pub struct NvSBox {
        pub sX: i32,
        pub sY: i32,
        pub sWidth: i32,
        pub sHeight: i32,
    }
}

nvstruct! {
    pub struct NvGUID {
        pub data1: u32,
        pub data2: u16,
        pub data3: u16,
        pub data4: [u8; 8],
    }
}

pub type NvLUID = NvGUID;

pub const NVAPI_MAX_PHYSICAL_GPUS: usize = 64;

pub const NVAPI_MAX_PHYSICAL_BRIDGES: usize = 100;
pub const NVAPI_PHYSICAL_GPUS: usize = 32;
pub const NVAPI_MAX_LOGICAL_GPUS: usize = 64;
pub const NVAPI_MAX_AVAILABLE_GPU_TOPOLOGIES: usize = 256;
pub const NVAPI_MAX_AVAILABLE_SLI_GROUPS: usize = 265;
pub const NVAPI_MAX_GPU_TOPOLOGIES: usize = NVAPI_MAX_PHYSICAL_GPUS;
pub const NVAPI_MAX_GPU_PER_TOPOLOGY: usize = 8;
pub const NVAPI_MAX_DISPLAY_HEADS: usize = 2;
pub const NVAPI_ADVANCED_DISPLAY_HEADS: usize = 4;
pub const NVAPI_MAX_DISPLAYS: usize = NVAPI_PHYSICAL_GPUS * NVAPI_ADVANCED_DISPLAY_HEADS;
pub const NVAPI_MAX_ACPI_IDS: usize = 16;
pub const NVAPI_MAX_VIEW_MODES: usize = 8;
pub const NVAPI_MAX_HEADS_PER_GPU: usize = 32;

/// Maximum heads, each with `NVAPI_DESKTOP_RES` resolution
pub const NV_MAX_HEADS: usize = 4;
/// Maximum number of input video streams, each with a `NVAPI_VIDEO_SRC_INFO`
pub const NV_MAX_VID_STREAMS: usize = 4;
/// Maximum number of output video profiles supported
pub const NV_MAX_VID_PROFILES: usize = 4;

pub const NVAPI_SYSTEM_MAX_DISPLAYS: usize = NVAPI_MAX_PHYSICAL_GPUS * NV_MAX_HEADS;

pub const NVAPI_SYSTEM_MAX_HWBCS: usize = 128;
pub const NVAPI_SYSTEM_HWBC_INVALID_ID: usize = 0xffffffff;
pub const NVAPI_MAX_AUDIO_DEVICES: usize = 16;

pub type NvAPI_String = NvString<NVAPI_GENERIC_STRING_MAX>;
pub type NvAPI_LongString = NvString<NVAPI_LONG_STRING_MAX>;
pub type NvAPI_ShortString = NvString<NVAPI_SHORT_STRING_MAX>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NvString<const N: usize>(pub [c_char; N]);

impl<const N: usize> NvString<N> {
    pub fn as_bytes(&self) -> &[u8; N] {
        let ptr = &self.0 as *const [c_char; N] as *const [u8; N];
        unsafe { &*ptr }
    }

    pub fn str_bytes(&self) -> &[u8] {
        let n = self.iter().take_while(|&&c| c != 0).count();
        &self.as_bytes()[..n]
    }

    pub fn as_cstr(&self) -> Result<&CStr, std::ffi::FromBytesWithNulError> {
        CStr::from_bytes_with_nul(self.str_bytes())
    }

    pub fn to_cstr(&self) -> Cow<CStr> {
        match self.as_cstr() {
            Ok(str) => Cow::Borrowed(str),
            Err(..) => Cow::Owned(unsafe {
                CString::from_vec_unchecked(self.str_bytes().into())
            }),
        }
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.str_bytes())
    }
}

impl<const N: usize> Default for NvString<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Deref for NvString<N> {
    type Target = [c_char; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for NvString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<NvString<N>> for String {
    fn from(str: NvString<N>) -> String {
        str.to_string_lossy().into_owned()
    }
}

unsafe impl<const N: usize> AsBytes for NvString<N> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

unsafe impl<const N: usize> FromBytes for NvString<N> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

/// NvAPI Version Definition
///
/// Maintain per structure specific version
pub const fn MAKE_NVAPI_VERSION<T>(ver: u16) -> u32 {
    NvVersion::with_struct::<T>(ver).data
}

pub const fn GET_NVAPI_VERSION(ver: u32) -> u16 {
    NvVersion::with_version(ver).version()
}

pub const fn GET_NVAPI_SIZE(ver: u32) -> usize {
    NvVersion::with_version(ver).size()
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Padding<T> {
    pub data: T,
}

impl<T> Deref for Padding<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Padding<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: IntoIterator> IntoIterator for Padding<T> {
    type Item = T::Item;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Padding<T> where &'a T: IntoIterator {
    type Item = <&'a T as IntoIterator>::Item;
    type IntoIter = <&'a T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Padding<T> where &'a mut T: IntoIterator {
    type Item = <&'a mut T as IntoIterator>::Item;
    type IntoIter = <&'a mut T as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

unsafe impl<T: AsBytes> AsBytes for Padding<T> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

unsafe impl<T: FromBytes> FromBytes for Padding<T> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

impl<T: FromBytes, const N: usize> Default for Padding<[T; N]> {
    fn default() -> Self {
        unsafe {
            MaybeUninit::zeroed().assume_init()
        }
    }
}

fn all_zero<T: AsBytes>(v: &T) -> bool {
    v.as_bytes().iter().all(|&v| v == 0)
}

impl<T: AsBytes, const N: usize> Padding<[T; N]> {
    pub fn all_zero(&self) -> bool {
        all_zero(self)
    }

    pub fn check_zero(&self) -> Result<(), crate::ArgumentRangeError> {
        match self.all_zero() {
            true => Ok(()),
            false => Err(crate::ArgumentRangeError),
        }
    }
}

impl<T: AsBytes + fmt::Debug, const N: usize> fmt::Debug for Padding<[T; N]> {
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

pub type ClockMaskData<const N: usize = 8> = [u32; N];
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ClockMask<const N: usize = 8> {
    pub mask: ClockMaskData<N>,
}

impl<const N: usize> ClockMask<N> {
    pub fn get_bit(&self, mut bit: usize) -> bool {
        let mut mask = &self.mask[..];
        while bit >= 32 {
            mask = &mask[1..];
            bit -= 32;
        }
        mask[0] & (1u32 << bit) != 0
    }

    pub fn set_bit(&mut self, mut bit: usize) {
        let mut mask = &mut self.mask[..];
        while bit >= 32 {
            mask = &mut { mask }[1..];
            bit -= 32;
        }
        mask[0] |= 1u32 << bit;
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn index<'s, 'a, T: 'static>(&'s self, entries: &'a [T]) -> impl Iterator<Item=(usize, &'a T)> + 's where 'a: 's {
        self.iter().map(move |i| (i, &entries[i]))
    }

    pub fn index_mut<'s, 'a, T: 'static>(&'s self, entries: &'a mut [T]) -> impl Iterator<Item=(usize, &'a mut T)> + 's where 'a: 's {
        let mut entries = entries.iter_mut().enumerate();
        self.iter().map(move |i| loop {
            match entries.next() {
                None => panic!("entries out of range of {:?}", self),
                Some((ei, _)) if ei < i => (),
                Some(t) => break t,
            }
        })
    }
}

impl<const N: usize> Default for ClockMask<N> {
    fn default() -> Self {
        Self {
            mask: [0u32; N],
        }
    }
}

unsafe impl<const N: usize> AsBytes for ClockMask<N> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

unsafe impl<const N: usize> FromBytes for ClockMask<N> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

impl<'a, const N: usize> IntoIterator for &'a ClockMask<N> {
    type Item = usize;
    type IntoIter = ClockMaskIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ClockMaskIter::new(&self.mask)
    }
}

#[cfg(feature = "serde")]
mod serde_impl_clock_mask {
    use serde::{Serialize, Serializer, Deserialize, Deserializer};
    use super::ClockMask;

    impl<'de, const N: usize> Deserialize<'de> for ClockMask<N> where [u32; N]: Deserialize<'de> {
        fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
            Deserialize::deserialize(de)
                .map(|mask| Self {
                    mask,
                })
        }
    }

    impl<const N: usize> Serialize for ClockMask<N> {
        fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            self.mask.serialize(ser)
        }
    }
}

#[derive(Copy, Clone)]
pub struct ClockMaskIter<'a> {
    mask: &'a [u32],
    offset: usize,
}

impl<'a> ClockMaskIter<'a> {
    pub fn new(mask: &'a [u32]) -> Self {
        ClockMaskIter {
            mask,
            offset: 0,
        }
    }
}

impl<'a> Iterator for ClockMaskIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.mask.len() > 0 {
            let offset = self.offset;
            let bit = offset % 32;
            let set = self.mask[0] & (1u32 << bit) != 0;

            self.offset += 1;
            if bit == 31 {
                self.mask = &self.mask[1..]
            }

            if set {
                return Some(offset)
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.mask.len() * 32 - (self.offset % 32)))
    }
}
