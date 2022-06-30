macro_rules! nv_declare_handle {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $name(*const ::std::os::raw::c_void);

        impl Default for $name {
            fn default() -> Self {
                $name(::std::ptr::null())
            }
        }
    };
}

macro_rules! nvinherit {
    (
        $v2:ident($id:ident: $v1:ty)
    ) => {
        impl ::std::ops::Deref for $v2 {
            type Target = $v1;

            fn deref(&self) -> &Self::Target {
                &self.$id
            }
        }

        impl ::std::ops::DerefMut for $v2 {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$id
            }
        }

        impl crate::nvapi::VersionedStruct for $v2 {
            fn nvapi_version_mut(&mut self) -> &mut crate::nvapi::NvVersion {
                self.$id.nvapi_version_mut()
            }

            fn nvapi_version(&self) -> crate::nvapi::NvVersion {
                self.$id.nvapi_version()
            }
        }
    };
}

macro_rules! nvstruct {
    (
        $(#[$meta:meta])*
        pub struct $name:ident {
            $($tt:tt)*
        }
    ) => {
        $(#[$meta])*
        #[repr(C)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name {
            $($tt)*
        }

        nvstruct! { @int fields $name ($($tt)*) }
    };
    (@int fields $name:ident (
            $(#[$meta:meta])*
            pub $id:ident: NvVersion,
            $($tt:tt)*)
        ) => {
        impl crate::nvapi::VersionedStruct for $name {
            fn nvapi_version_mut(&mut self) -> &mut NvVersion {
                &mut self.$id
            }

            fn nvapi_version(&self) -> NvVersion {
                self.$id
            }
        }
    };
    (@int fields $name:ident ($($tt:tt)*)) => { };
}

macro_rules! nvenum {
    (
        $(#[$meta:meta])*
        pub enum $enum:ident / $enum_name:ident {
            $(
                $(#[$metai:meta])*
                $symbol:ident / $name:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$meta])*
        pub type $enum = ::std::os::raw::c_int;
        $(
            $(#[$metai])*
            #[allow(overflowing_literals)]
            pub const $symbol: $enum = $value as _;
        )*

        $(#[$meta])*
        #[allow(overflowing_literals)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum $enum_name {
            $(
                $(#[$metai])*
                $name = $symbol as _,
            )*
        }

        impl $enum_name {
            #[allow(overflowing_literals)]
            pub fn from_raw(raw: $enum) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                match raw {
                    $(
                        $symbol
                    )|* => Ok(unsafe { ::std::mem::transmute(raw) }),
                    _ => Err(Default::default()),
                }
            }

            pub fn raw(&self) -> $enum {
                *self as _
            }

            pub fn values() -> impl Iterator<Item=Self> {
                [
                    $(
                        $enum_name::$name
                    ),*
                ].into_iter()
            }
        }

        impl Into<$enum> for $enum_name {
            fn into(self) -> $enum {
                self as _
            }
        }

        impl TryFrom<$enum> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(raw: $enum) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                Self::from_raw(raw)
            }
        }
    };
}

macro_rules! nvbits {
    (
        $(#[$meta:meta])*
        pub enum $enum:ident / $enum_name:ident {
            $(
                $(#[$($metai:tt)*])*
                $symbol:ident / $name:ident = $value:expr,
            )*
        }
    ) => {
        $(#[$meta])*
        pub type $enum = u32;
        $(
            $(#[$($metai)*])*
            pub const $symbol: $enum = $value as _;
        )*

        bitflags::bitflags! {
            $(#[$meta])*
            #[derive(Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $enum_name: $enum {
            $(
                $(#[$($metai)*])*
                const $name = $value;
            )*
            }
        }

        impl Iterator for $enum_name {
            type Item = Self;

            fn next(&mut self) -> Option<Self::Item> {
                $(
                    if self.contains($enum_name::$name) {
                        self.remove($enum_name::$name);
                        Some($enum_name::$name)
                    } else
                 )*
                { None }
            }
        }

        impl TryFrom<$enum> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(v: $enum) -> Result<Self, Self::Error> {
                Self::from_bits(v).ok_or(crate::ArgumentRangeError)
            }
        }

        impl From<$enum_name> for $enum {
            fn from(v: $enum_name) -> $enum {
                v.bits()
            }
        }
    };
}

macro_rules! nvenum_display {
    ($enum:ident => _) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Debug::fmt(self, f)
            }
        }
    };
    ($enum:ident => {
        $(
            $name:tt = $value:tt,
        )*
    }) => {
        impl ::std::fmt::Display for $enum {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                $(
                    nvenum_display!(@q $enum $name) => nvenum_display!(@expr self f $value),
                    //$enum::$name => nvenum_display!(@expr self f $value),
                )*
                }
            }
        }
    };
    (@q $enum:ident _) => {
        _
    };
    (@q $enum:ident $name:ident) => {
        $enum::$name
    };
    (@expr $this:tt $fmt:ident _) => {
        ::std::fmt::Debug::fmt($this, $fmt)
    };
    (@expr $this:tt $fmt:ident $expr:expr) => {
        write!($fmt, "{}", $expr)
    };
}

macro_rules! nvapi {
    (
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
    ) => {
        $(#[$meta])*
        pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret {
            static CACHE: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::AtomicUsize::new(0);

            match crate::nvapi::query_interface(crate::nvid::Api::$fn.id(), &CACHE) {
                Ok(ptr) => ::std::mem::transmute::<_, extern "C" fn($($arg: $arg_ty),*) -> $ret>(ptr)($($arg),*),
                Err(e) => e.raw(),
            }
        }
    };
    (
        pub type $name:ident = extern "C" fn($($arg:ident: $arg_ty:ty),*) -> $ret:ty;

        $(#[$meta:meta])*
        pub unsafe fn $fn:ident;
    ) => {
        pub type $name = extern "C" fn($($arg: $arg_ty),*) -> $ret;

        nvapi! {
            $(#[$meta])*
            pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret;
        }
    };
}

macro_rules! nvversion {
    ($name:ident($struct:ident = $sz:expr, $ver:expr)) => {
        pub const $name: NvVersion = NvVersion::with_struct::<$struct>($ver);

        mod $name {
            pub(crate) type Argument = super::$struct;
        }

        impl crate::nvapi::StructVersion<$ver> for $struct {
            const NVAPI_VERSION: crate::nvapi::NvVersion = $name;
        }

        const _: () = assert!($sz == std::mem::size_of::<$struct>());
    };
    ($name:ident = $target:ident) => {
        pub const $name: NvVersion = $target;

        impl crate::nvapi::StructVersion for $target::Argument {
            const NVAPI_VERSION: crate::nvapi::NvVersion = <$target::Argument as crate::nvapi::StructVersion<{$target.version()}>>::NVAPI_VERSION;

            fn versioned() -> Self {
                <$target::Argument as crate::nvapi::StructVersion<{$target.version()}>>::versioned()
            }
        }

        impl Default for $target::Argument {
            fn default() -> Self {
                crate::nvapi::StructVersion::<0>::versioned()
            }
        }
    };
    ($struct:ident(@.$id:ident)) => {
        impl crate::nvapi::VersionedStruct for $v2 {
            fn nvapi_version_mut(&mut self) -> &mut crate::nvapi::NvVersion {
                &mut self.$id
            }

            fn nvapi_version(&self) -> crate::nvapi::NvVersion {
                self.$id
            }
        }
    };
}

