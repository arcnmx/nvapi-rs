use zerocopy::{AsBytes, FromBytes};
use std::fmt;
use crate::nvapi::{NV_TRUE, NV_FALSE};

/// A boolean containing reserved bits
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, AsBytes, FromBytes)]
#[repr(transparent)]
pub struct BoolU32(pub u32);

impl BoolU32 {
    pub const FLAG_MASK: u32 = 1;
    pub const FLAGS_MASK: u32 = 0xffffffe;

    pub const fn with_value(value: u32) -> Self {
        Self(value)
    }

    pub const fn new(flag: bool, flags: u32) -> Self {
        Self::with_value(
            flags & Self::FLAGS_MASK
            | if flag { NV_TRUE } else { NV_FALSE } as u32
        )
    }

    pub const fn get(self) -> bool {
        self.value() & Self::FLAG_MASK == NV_TRUE as u32
    }

    /// Remaining bits
    pub const fn flags(self) -> u32 {
        self.value() & Self::FLAGS_MASK
    }

    #[inline]
    pub const fn value(self) -> u32 {
        self.0
    }

    pub fn set(&mut self, value: bool) {
        self.0 &= Self::FLAGS_MASK;
        self.0 |= if value { NV_TRUE } else { NV_FALSE } as u32
    }
}

impl From<bool> for BoolU32 {
    fn from(v: bool) -> Self {
        Self::new(v, 0)
    }
}

impl From<BoolU32> for bool {
    fn from(v: BoolU32) -> Self {
        v.get()
    }
}

impl From<u32> for BoolU32 {
    fn from(v: u32) -> Self {
        BoolU32::with_value(v)
    }
}

impl From<BoolU32> for u32 {
    fn from(v: BoolU32) -> Self {
        v.value()
    }
}

impl fmt::Display for BoolU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value(), f)
    }
}

impl fmt::Debug for BoolU32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.flags() {
            0 => f.debug_tuple("BoolU32")
                .field(&self.get())
                .finish(),
            flags => f.debug_struct("BoolU32")
                .field("flag", &self.get())
                .field("flags", &flags)
                .finish(),
        }
    }
}
