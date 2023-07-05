macro_rules! nv_declare_handle {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $name(*const ::std::os::raw::c_void);

        impl $name {
            pub fn as_ptr(&self) -> *const ::std::os::raw::c_void {
                self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name(::std::ptr::null())
            }
        }
    };
}

macro_rules! nvinherit {
    (
        struct $v2:ident($id:ident: $v1:ty)
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
    };
    (
        $v2:ident($id:ident: $v1:ty)
    ) => {
        nvinherit! { struct $v2($id: $v1) }

        impl VersionedStructField for $v2 {
            fn nvapi_version_mut(&mut self) -> &mut NvVersion {
                VersionedStructField::nvapi_version_mut(&mut self.$id)
            }

            fn nvapi_version_ref(&self) -> &NvVersion {
                VersionedStructField::nvapi_version_ref(&self.$id)
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

        unsafe impl zerocopy::AsBytes for $name {
            fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
        }

        unsafe impl zerocopy::FromBytes for $name {
            fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
        }

        nvstruct! { @int fields $name ($($tt)*) }
    };
    (@int fields $name:ident (
            $(#[$meta:meta])*
            pub $id:ident: NvVersion,
            $($tt:tt)*)
        ) => {
        impl VersionedStructField for $name {
            fn nvapi_version_mut(&mut self) -> &mut NvVersion {
                &mut self.$id
            }

            fn nvapi_version_ref(&self) -> &NvVersion {
                &self.$id
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
        pub type $enum = crate::value::NvEnum<$enum_name>;
        $(
            $(#[$metai])*
            #[allow(overflowing_literals)]
            pub const $symbol: $enum = $enum::with_repr($value as _);
        )*

        #[allow(non_upper_case_globals)]
        impl $enum {
            $(
                $(#[$metai])*
                pub const $name: $enum = $symbol;
            )*
        }

        $(#[$meta])*
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        #[non_exhaustive]
        #[repr(i32)]
        pub enum $enum_name {
            $(
                $(#[$metai])*
                $name = $symbol.repr(),
            )*
        }

        impl $enum_name {
            pub const fn value(self) -> $enum {
                $enum::with_repr(self.repr())
            }

            pub const fn repr(self) -> i32 {
                self as _
            }

            pub fn values() -> impl Iterator<Item=Self> {
                <Self as crate::value::NvValueData>::all_values().iter().copied()
            }
        }

        impl crate::value::NvValueEnum for $enum_name {
        }

        impl crate::value::NvValueData for $enum_name {
            const NAME: &'static str = stringify!($enum_name);
            const C_NAME: &'static str = stringify!($enum);

            type Repr = i32;

            fn from_repr(raw: Self::Repr) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                match $enum::with_repr(raw) {
                    $(
                        $symbol
                    )|* => Ok(unsafe { ::std::mem::transmute(raw) }),
                    _ => Err(Default::default()),
                }
            }

            fn from_repr_ref(raw: &Self::Repr) -> ::std::result::Result<&Self, crate::ArgumentRangeError> {
                Self::from_repr(*raw).map(|_| unsafe {
                    std::mem::transmute(raw)
                })
            }

            fn from_repr_mut(raw: &mut Self::Repr) -> ::std::result::Result<&mut Self, crate::ArgumentRangeError> {
                Self::from_repr(*raw).map(|_| unsafe {
                    std::mem::transmute(raw)
                })
            }

            fn all_values() -> &'static [Self] {
                &[
                    $(
                        $enum_name::$name
                    ),*
                ]
            }

            fn repr(self) -> Self::Repr {
                self as _
            }

            fn repr_ref(&self) -> &Self::Repr {
                unsafe {
                    std::mem::transmute(self)
                }
            }
        }

        impl Into<i32> for $enum_name {
            fn into(self) -> i32 {
                crate::value::NvValueData::repr(self)
            }
        }

        impl TryFrom<i32> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(raw: i32) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                crate::value::NvValueData::from_repr(raw)
            }
        }

        impl TryFrom<$enum> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(value: $enum) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                Self::try_from(value.value)
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
        pub type $enum = crate::value::NvBits<$enum_name>;
        $(
            $(#[$($metai)*])*
            pub const $symbol: u32 = $value as _;
        )*

        impl TryFrom<$enum> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(value: $enum) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                Self::try_from(value.value)
            }
        }

        bitflags::bitflags! {
            $(#[$meta])*
            #[derive(Default)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct $enum_name: u32 {
            $(
                $(#[$($metai)*])*
                const $name = $value;
            )*
            }
        }

        impl $enum_name {
            pub const fn value(self) -> $enum {
                $enum::with_repr(self.bits())
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

        impl crate::value::NvValueBits for $enum_name {
            fn from_repr_truncate(value: Self::Repr) -> Self {
                Self::from_bits_truncate(value)
            }
        }

        impl crate::value::NvValueData for $enum_name {
            const NAME: &'static str = stringify!($enum_name);
            const C_NAME: &'static str = stringify!($enum);

            type Repr = u32;

            fn from_repr(raw: Self::Repr) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                Self::from_bits(raw).ok_or(crate::ArgumentRangeError)
            }

            fn from_repr_ref(raw: &Self::Repr) -> ::std::result::Result<&Self, crate::ArgumentRangeError> {
                Self::from_repr(*raw).map(|_| unsafe {
                    std::mem::transmute(raw)
                })
            }

            fn from_repr_mut(raw: &mut Self::Repr) -> ::std::result::Result<&mut Self, crate::ArgumentRangeError> {
                Self::from_repr(*raw).map(|_| unsafe {
                    std::mem::transmute(raw)
                })
            }

            fn all_values() -> &'static [Self] {
                &[
                    $(
                        $enum_name::$name
                    ),*
                ]
            }

            fn repr(self) -> Self::Repr {
                self.bits()
            }

            fn repr_ref(&self) -> &Self::Repr {
                unsafe {
                    std::mem::transmute(self)
                }
            }
        }

        impl Into<u32> for $enum_name {
            fn into(self) -> u32 {
                crate::value::NvValueData::repr(self)
            }
        }

        impl TryFrom<u32> for $enum_name {
            type Error = crate::ArgumentRangeError;

            fn try_from(raw: u32) -> ::std::result::Result<Self, crate::ArgumentRangeError> {
                crate::value::NvValueData::from_repr(raw)
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
            static CACHE: crate::apifn::NvapiFnCache = crate::apifn::NvapiFnCache::empty();

            match CACHE.query_ptr(crate::nvid::Api::$fn.id()) {
                Ok(ptr) => ::std::mem::transmute::<_, extern "C" fn($($arg: $arg_ty),*) -> $ret>(ptr)($($arg),*),
                Err(e) => e.value(),
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
    (_: $latest:ident($latest_ver:literal$($rest:tt)*) $($tt:tt)*) => {
        nvversion! { @cont(_) $latest $latest($latest_ver) $latest($latest_ver$($rest)*) $($tt)* }
    };
    ($name:ident: $latest:ident($latest_ver:literal$($rest:tt)*) $($tt:tt)*) => {
        nvversion! { @cont($name) $name $latest($latest_ver) $latest($latest_ver$($rest)*) $($tt)* }
    };
    (@cont($defname:tt) $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?) => {
        nvversion! { @impl StructVersion $target($ver) }
        nvversion! { @rest($defname) $name $latest($latest_ver) $latest($latest_ver) $target($ver)($($($rest)*)?) }
        nvversion! { @type($defname) $latest $target }
    };
    (@cont($defname:tt) $name:ident $latest:ident($latest_ver:literal) $($target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?),+) => {
        nvversion! { @find_oldest($defname)() $name $latest($latest_ver) $($target($ver$(;$($rest)+)?)$(= $sz)?),+ }
    };
    (@find_oldest($defname:tt)($($older:tt)*) $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?, $($rest_:tt)*) => {
        nvversion! { @find_oldest($defname)($($older)* $target($ver$(;$($rest)+)?)$(= $sz)?,) $name $latest($latest_ver) $($rest_)* }
    };
    (@find_oldest($defname:tt)($($older:tt)*) $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?) => {
        nvversion! { @find_oldest($defname)($($older)* $target($ver$(;$($rest)+)?)$(= $sz)?,)($target($ver)) $name $latest($latest_ver) }
    };
    (@find_oldest($defname:tt)($($target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?,)+)($oldest:ident($oldest_ver:literal)) $name:ident $latest:ident($latest_ver:literal)) => {
        $(
            nvversion! { @impl StructVersion $target($ver) $latest($latest_ver) $oldest($oldest_ver) }
            nvversion! { @rest($defname) $name $latest($latest_ver) $oldest($oldest_ver) $target($ver)($($($rest)*)?) }
        )*
        $(
            impl StructVersionInfo<$ver> for $oldest {
                type Struct = $target;
                type Storage = $oldest;
            }
        )*
        nvversion! { @type($defname) $latest $oldest }
    };
    (@rest($defname:tt) $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)()) => {
        nvversion! { @latest($defname) $target($ver) $oldest($oldest_ver) }
    };
    (@rest($defname:tt) $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)(@inherit($id:ident: $v1:ty)$($rest:tt)*)) => {
        nvinherit! { $target($id: $v1) }
        nvversion! { @rest($defname) $name $latest($latest_ver) $oldest($oldest_ver) $target($ver)($($rest)*) }
    };
    (@rest($defname:tt) $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)(@old)) => {
        nvversion! { @old($defname) $target($ver) $latest($latest_ver) $oldest($oldest_ver) }
    };
    // @rest base case for latest version
    (@latest($defname:tt) $target:ident($ver:literal) $oldest:ident($oldest_ver:literal)) => {
        nvversion! { @impl Default $target($ver) }
        //nvversion! { @impl StructVersion $target($ver) }

        /*impl StructVersionInfo<$ver> for $target {
            type Oldest = $oldest;
            type Storage = $oldest;
        }*/
    };
    // @rest base case for old versions
    (@old($defname:tt) $target:ident($ver:literal) $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal)) => {
    };
    // this target is the latest version
    /*(@impl StructVersion $target:ident($ver:literal)) => {
        nvversion! { @impl StructVersion($ver)(NvVersion) $target(NvVersion::with_struct::<$target>($ver)) }
    };*/
    // this version is one of many
    (@impl StructVersion $target:ident($ver:literal) $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal)) => {
        nvversion! { @impl StructVersion($ver)($oldest) $target(NvVersion::with_struct::<$target>($ver)) }
    };
    /* this version is not the latest
    (@impl StructVersion($t:literal) $target:ident($ver:literal) $latest:ident($latest_ver:literal)) => {
        /*impl StructVersion<0> for $target {
            const NVAPI_VERSION: crate::nvapi::NvVersion = <Self as StructVersion<$ver>>::NVAPI_VERSION;
            type Storage = <Self as StructVersion<$ver>>::Storage;

            /*
            #[inline]
            fn storage_ref(nv: &Self::Storage) -> Option<&Self> {
                <Self as StructVersion<$ver>>::storage_ref(nv)
            }

            #[inline]
            fn storage_mut(nv: &mut Self::Storage) -> Option<&mut Self> {
                <Self as StructVersion<$ver>>::storage_mut(nv)
            }*/
        }*/
    };*/
    // there is just one known version, and this is it
    (@impl StructVersion $target:ident($ver:literal)) => {
        nvversion! { @impl StructVersion($ver)($target) $target(NvVersion::with_struct::<$target>($ver)) }

        impl StructVersionInfo<$ver> for $target {
            type Struct = Self;
            type Storage = Self;
        }
    };
    (@impl StructVersion($t:literal)($storage:ty) $target:ident($ver:expr)) => {
        impl StructVersion<$t> for $target {
            const NVAPI_VERSION: NvVersion = $ver;
            type Storage = $storage;

            /*
            fn storage_ref(nv: &Self::Storage) -> Option<&Self> {
                match VersionedStruct::nvapi_version(nv) {
                    <Self as StructVersion<$t>>::NVAPI_VERSION => Some(unsafe {
                        ::std::mem::transmute(nv)
                    }),
                    _ => None,
                }
            }

            fn storage_mut(nv: &mut Self::Storage) -> Option<&mut Self> {
                match VersionedStruct::nvapi_version(nv) {
                    <Self as StructVersion<$t>>::NVAPI_VERSION => Some(unsafe {
                        ::std::mem::transmute(nv)
                    }),
                    _ => None,
                }
            }*/
        }
    };
    (@impl Default $target:ident($ver:literal)) => {
        impl Default for $target {
            #[inline]
            fn default() -> Self {
                StructVersion::<$ver>::versioned()
            }
        }
    };
    (@type(_) $($rest:tt)*) => {
    };
    (@type($name:ident) $latest:ident $oldest:ident) => {
        pub type $name = $latest;
    };
}
