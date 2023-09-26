macro_rules! nv_declare_handle {
    (
        $(#[$meta:meta])*
        $name:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        #[repr(transparent)]
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

        unsafe impl zerocopy::AsBytes for $name {
            fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
        }

        unsafe impl zerocopy::FromBytes for $name {
            fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
        }
    };
}

macro_rules! nvstruct {
    ($($tt:tt)*) => {
        #[$crate::types::NvStruct]
        $($tt)*
    };
}

macro_rules! nvenum {
    ($($tt:tt)*) => {
        nvapi_macros::nvenum! {
            $($tt)*
        }
    };
}

macro_rules! nvbits {
    ($($tt:tt)*) => {
        nvapi_macros::nvbits! {
            $($tt)*
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
    (@ $(=$name:ident)? $target:ident($ver:expr) $(= $sz:expr)?) => {
        nvversion! { $(=$name)? $target($ver) $(=$sz)? }

        impl crate::nvapi::StructVersion for $target {
            const NVAPI_VERSION: crate::nvapi::NvVersion = <$target as crate::nvapi::StructVersion<{$ver}>>::NVAPI_VERSION;

            fn versioned() -> Self {
                <$target as crate::nvapi::StructVersion<{$ver}>>::versioned()
            }
        }

        impl Default for $target {
            fn default() -> Self {
                crate::nvapi::StructVersion::<0>::versioned()
            }
        }
    };
    ($(=$name:ident)? $target:ident($ver:expr) $(= $sz:expr)?) => {
        $(
            pub type $name = $target;
        )?

        impl crate::nvapi::StructVersion<$ver> for $target {
            const NVAPI_VERSION: crate::nvapi::NvVersion = NvVersion::with_struct::<$target>($ver);
        }

        $(
            const _: () = assert!($sz == std::mem::size_of::<$target>());
        )?
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

