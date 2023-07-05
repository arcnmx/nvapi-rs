use std::ops::{self, Deref, DerefMut, RangeBounds, Range, Bound};
use std::{fmt, iter};
use zerocopy::{AsBytes, FromBytes};
use crate::{ClockMask, ArgumentRangeError};

pub trait ArrayLike
    : ops::Index<usize, Output=<Self as ArrayLike>::Element>
{
    type Element;

    fn len(&self) -> usize;
}

impl<T, const N: usize> ArrayLike for [T; N] {
    type Element = T;

    #[inline]
    fn len(&self) -> usize {
        N
    }
}

impl<'a, A: ArrayLike> ArrayLike for &'a A where
    &'a A: ops::Index<usize, Output=A::Element>
{
    type Element = A::Element;

    #[inline]
    fn len(&self) -> usize { ArrayLike::len(*self) }
}

impl<'a, A: ArrayLike> ArrayLike for &'a mut A where
    &'a mut A: ops::Index<usize, Output=A::Element>
{
    type Element = A::Element;

    #[inline]
    fn len(&self) -> usize { ArrayLike::len(*self) }
}

#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Truncated<T, C = ops::RangeTo<usize>> {
    pub data: T,
    pub count: C,
}

impl<T, C> Truncated<T, C> {
    pub const fn new(data: T, count: C) -> Self {
        Self {
            data,
            count,
        }
    }
}

fn count_range_<C: RangeBounds<usize>>(count: &C, data_len: usize) -> Range<usize> {
    Range {
        start: match count.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.saturating_add(1),
        },
        end: match count.end_bound() {
            Bound::Unbounded => data_len,
            Bound::Excluded(&i) => i,
            Bound::Included(&i) => i.saturating_add(1),
        },
    }
}

impl<T: ArrayLike, C: RangeBounds<usize>> Truncated<T, C> {
    pub fn new_with(data: T, count: C) -> Self where
        C: Clone,
    {
        let data = match () {
            #[cfg(feature = "log")]
            () if log::Level::Warn <= log::max_level() => {
                match Self::try_with(data, count.clone()) {
                    Ok(data) => return data,
                    Err(data) => {
                        log::warn!("invalid Truncated<[T; {}]> count {:?} ~ {:?}", data.len(), count.start_bound(), count.end_bound());
                        data
                    },
                }
            },
            _ => data,
        };
        Self::new(data, count)
    }

    pub fn try_with(data: T, count: C) -> Result<Self, T> {
        let data_len = data.len();
        let range = count_range_(&count, data_len);
        if range.start > range.end || range.end > data_len {
            Err(data)
        } else {
            Ok(Self::new(data, count))
        }
    }

    pub fn as_slice(&self) -> &[T::Element] where
        T: ops::Index<Range<usize>, Output=[T::Element]>,
    {
        self.data.index(self.range())
    }

    pub fn as_slice_mut(&mut self) -> &mut [T::Element] where
        T: ops::IndexMut<Range<usize>, Output=[T::Element]>,
    {
        let range = self.range();
        self.data.index_mut(range)
    }

    fn count_range(&self) -> Range<usize> where
        C: RangeBounds<usize>,
    {
        count_range_(&self.count, self.data.len())
    }

    pub fn range(&self) -> Range<usize> where
        C: RangeBounds<usize>,
    {
        let data_len = self.data.len();
        let Range { start, end } = self.count_range();
        Range {
            start: start.min(data_len),
            end: end.min(data_len),
        }
    }

    pub fn len(&self) -> usize where
    {
        self.range().len()
    }
}

impl<I, T: ArrayLike, C: RangeBounds<usize>> ops::Index<I> for Truncated<T, C> where
    T: ops::Index<Range<usize>, Output=[T::Element]>,
    [T::Element]: ops::Index<I>,
{
    type Output = <[T::Element] as ops::Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.as_slice().index(index)
    }
}

impl<I, T: ArrayLike, C: RangeBounds<usize>> ops::IndexMut<I> for Truncated<T, C> where
    T: ops::IndexMut<Range<usize>, Output=[T::Element]>,
    [T::Element]: ops::IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.as_slice_mut().index_mut(index)
    }
}

impl<T: ArrayLike, C: RangeBounds<usize>> ArrayLike for Truncated<T, C> where
    T: ops::IndexMut<Range<usize>, Output=[T::Element]>,
{
    type Element = T::Element;

    fn len(&self) -> usize {
        Truncated::len(self)
    }
}

impl<T: ArrayLike> Truncated<T, ClockMask> {
    pub fn masked_iter<'a>(&'a self) -> impl Iterator<Item = &'a T::Element> + 'a {
        #[cfg(feature = "log")] {
            log::error!("TODO: data needs bounds checking!!!");
        }
        self.count.iter().map(|i| &self.data[i])
    }
}

impl<T: ArrayLike, C: RangeBounds<usize>> Deref for Truncated<T, C> where
    T: ops::Index<Range<usize>, Output=[T::Element]>,
{
    type Target = [T::Element];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: ArrayLike, C: RangeBounds<usize>> DerefMut for Truncated<T, C> where
    T: ops::IndexMut<Range<usize>, Output=[T::Element]>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_slice_mut()
    }
}

impl<T: ArrayLike, C: RangeBounds<usize>> IntoIterator for Truncated<T, C> where
    T: IntoIterator,
{
    type Item = <T as IntoIterator>::Item;
    type IntoIter = iter::Take<iter::Skip<<T as IntoIterator>::IntoIter>>;

    fn into_iter(self) -> Self::IntoIter {
        let range = self.range();
        self.data.into_iter().skip(range.start).take(range.end)
    }
}

impl<'a, T: ArrayLike> IntoIterator for &'a Truncated<T, ClockMask> {
    type Item = &'a T::Element;
    type IntoIter = Box<dyn Iterator<Item=Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.masked_iter())
    }
}

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

    pub fn truncated<C>(self, count: C) -> Truncated<Self, C> {
        Truncated::new(self, count)
    }

    pub fn truncated_ref<C>(&self, count: C) -> Truncated<&Self, C> {
        Truncated::new(self, count)
    }

    pub fn truncated_mut<C>(&mut self, count: C) -> Truncated<&mut Self, C> {
        Truncated::new(self, count)
    }

    pub fn truncate_to(self, count: usize) -> Truncated<Self, std::ops::RangeTo<usize>> where
        T: ArrayLike,
    {
        Truncated::new_with(self, ..count)
    }

    pub fn truncate_to_ref(&self, count: usize) -> Truncated<&Self, std::ops::RangeTo<usize>> where
        T: ArrayLike,
    {
        Truncated::new_with(self, ..count)
    }

    pub fn truncate_to_mut(&mut self, count: usize) -> Truncated<&mut Self, std::ops::RangeTo<usize>> where
        T: ArrayLike,
    {
        Truncated::new_with(self, ..count)
    }

    pub fn get_iter<R: RangeBounds<usize>>(self, range: R) -> std::iter::Take<std::iter::Skip<T::IntoIter>> where
        T: IntoIterator,
        T: AsRef<[<T as IntoIterator>::Item]>,
    {
        let skip = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i.saturating_add(1),
        };
        let take = match range.end_bound() {
            Bound::Unbounded => usize::MAX,
            Bound::Excluded(&i) => i.saturating_add(1).saturating_sub(skip),
            Bound::Included(&i) => i.saturating_sub(skip),
        };
        debug_assert!(self.data.as_ref().len() < skip);
        debug_assert!(self.data.as_ref().len() <= take);
        self.data.into_iter().skip(skip).take(take)
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

impl<I, T: ArrayLike> ops::Index<I> for Array<T> where
    T: ops::Index<I>,
{
    type Output = <T as ops::Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index)
    }
}

impl<I, T: ArrayLike> ops::IndexMut<I> for Array<T> where
    T: ops::IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<'a, I, T: ArrayLike> ops::Index<I> for &'a Array<T> where
    T: ops::Index<I>,
{
    type Output = <T as ops::Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index)
    }
}

impl<'a, I, T: ArrayLike> ops::Index<I> for &'a mut Array<T> where
    T: ops::Index<I>,
{
    type Output = <T as ops::Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.data.index(index)
    }
}

impl<'a, I, T: ArrayLike> ops::IndexMut<I> for &'a mut Array<T> where
    T: ops::IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.data.index_mut(index)
    }
}

impl<T: ArrayLike> ArrayLike for Array<T> {
    type Element = T::Element;

    fn len(&self) -> usize {
        ArrayLike::len(&self.data)
    }
}
