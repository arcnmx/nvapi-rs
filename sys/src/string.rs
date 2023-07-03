use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::c_char;
use zerocopy::{AsBytes, FromBytes};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NvString<const N: usize, C = c_char>(pub [C; N]);

impl<const N: usize, C> NvString<N, C> {
    pub fn data(&self) -> &[C; N] {
        &self.0
    }

    pub fn data_mut(&self) -> &[C; N] {
        &self.0
    }
}

impl<const N: usize, C> NvString<N, C> where
    C: Default + PartialEq,
{
    pub fn len(&self) -> usize {
        let zero = C::default();
        self.data().iter().take_while(|&c| c != &zero).count()
    }

    pub fn to_slice(&self) -> &[C] {
        &self.data()[..self.len()]
    }

    pub fn to_bytes(&self) -> &[u8] where
        C: AsBytes,
    {
        self.to_slice().as_bytes()
    }
}

impl<const N: usize> NvString<N, c_char> {
    /// May fail if buffer is full and not nul-terminated
    ///
    /// Typically [`NvString::to_cstr()`] should be preferred over this.
    pub fn as_cstr(&self) -> Option<&CStr> {
        match self.len() {
            len if len == N => None,
            len => Some(unsafe {
                CStr::from_bytes_with_nul_unchecked(self.data()[..len + 1].as_bytes())
            }),
        }
    }

    pub fn to_cstr(&self) -> Cow<CStr> {
        self.as_cstr().map(Cow::Borrowed)
            .unwrap_or_else(|| Cow::Owned(unsafe {
                CString::from_vec_unchecked(self.to_bytes()[..].into())
            }))
    }

    pub fn to_string_lossy(&self) -> Cow<str> {
        String::from_utf8_lossy(self.to_bytes())
    }
}

impl<const N: usize, C: FromBytes> Default for NvString<N, C> {
    fn default() -> Self {
        FromBytes::new_zeroed()
    }
}

impl<const N: usize, C> Deref for NvString<N, C> {
    type Target = [C; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize, C> DerefMut for NvString<N, C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<NvString<N, c_char>> for String {
    fn from(str: NvString<N, c_char>) -> String {
        str.to_string_lossy().into_owned()
    }
}

impl<const N: usize> fmt::Debug for NvString<N, c_char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NvString")
            .field(&self.to_cstr())
            .finish()
    }
}

impl<const N: usize> fmt::Display for NvString<N, c_char> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.to_string_lossy(), f)
    }
}

unsafe impl<const N: usize, C: AsBytes> AsBytes for NvString<N, C> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}

unsafe impl<const N: usize, C: FromBytes> FromBytes for NvString<N, C> {
    fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
}
