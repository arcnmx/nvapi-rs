use zerocopy::{AsBytes, FromBytes};

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
