use std::ffi::CStr;
use std::fmt;
use void::Void;
use sys;

pub trait RawConversion {
    type Target;
    type Error;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error>;

    fn to_raw(_s: &Self::Target) -> Self where Self: Sized { unimplemented!() }
}

impl RawConversion for sys::types::NvAPI_ShortString {
    type Target = String;
    type Error = Void;

    fn convert_raw(&self) -> Result<Self::Target, Self::Error> {
        unsafe {
            Ok(CStr::from_ptr(self.as_ptr()).to_string_lossy().into_owned())
        }
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Celsius(pub i32);

impl fmt::Debug for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} C", self.0)
    }
}

/// Nvidia encodes temperature as `<< 8` for some reason sometimes.
#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct CelsiusShifted(pub i32);

impl fmt::Debug for CelsiusShifted {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} C", self.get())
    }
}

impl CelsiusShifted {
    pub fn get(&self) -> i32 {
        self.0 >> 8
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Microvolts(pub u32);

impl fmt::Debug for Microvolts {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mV", self.0 as f32 / 1000.0)
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct MicrovoltsDelta(pub i32);

impl fmt::Debug for MicrovoltsDelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mV", self.0 as f32 / 1000.0)
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Kilohertz(pub u32);

impl fmt::Debug for Kilohertz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 < 1000 {
            write!(f, "{} kHz", self.0)
        } else {
            write!(f, "{} MHz", self.0 as f32 / 1000.0)
        }
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct KilohertzDelta(pub i32);

impl fmt::Debug for KilohertzDelta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if (self.0).abs() < 1000 {
            write!(f, "{} kHz", self.0)
        } else {
            write!(f, "{} MHz", self.0 as f32 / 1000.0)
        }
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct Percentage(pub u32);

impl fmt::Debug for Percentage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} %", self.0)
    }
}

impl Percentage {
    pub fn from_raw(v: u32) -> Result<Self, sys::ArgumentRangeError> {
        match v {
            v @ 0...100 => Ok(Percentage(v)),
            _ => Err(sys::ArgumentRangeError),
        }
    }
}
