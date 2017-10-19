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

        impl $name {
            pub fn zeroed() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
    };
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
            pub const $symbol: $enum = $value as _;
        )*

        $(#[$meta])*
        #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
        #[repr(i32)]
        pub enum $enum_name {
            $(
                $(#[$metai])*
                $name = $symbol as _,
            )*
        }

        impl $enum_name {
            pub fn from_raw(raw: $enum) -> ::std::result::Result<Self, ::ArgumentRangeError> {
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

            pub fn values() -> ::std::iter::Cloned<::std::slice::Iter<'static, Self>> {
                [
                    $(
                        $enum_name::$name
                    ),*
                ].into_iter().cloned()
            }
        }

        impl Into<$enum> for $enum_name {
            fn into(self) -> $enum {
                self as _
            }
        }
    };
}

macro_rules! nvapi {
    (
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident($($arg:ident: $arg_ty:ty),*) -> $ret:ty;
    ) => {
        $(#[$meta])*
        pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret {
            static CACHE: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::ATOMIC_USIZE_INIT;

            match ::nvapi::query_interface(::nvid::Api::$fn.id(), &CACHE) {
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

// No `const fn` yet :(
macro_rules! nvversion {
    ($name:ident($struct:ident = $sz:expr, $ver:expr)) => {
        pub const $name: u32 = ($sz) as u32 | ($ver as u32) << 16;
        /*pub fn $name() -> u32 {
            MAKE_NVAPI_VERSION::<$struct>($ver)
        }*/

        mod $name {
            #[test]
            fn $name() {
                assert_eq!(::types::GET_NVAPI_SIZE(super::$name), ::std::mem::size_of::<super::$struct>());
            }
        }
    };
    ($name:ident = $target:ident) => {
        pub const $name: u32 = $target;
        /*pub fn $name() -> u32 {
            $target()
        }*/
    };
}

