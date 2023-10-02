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
    ($($tt:tt)*) => {
        nvapi_macros::nvenum_display! {
            $($tt)*
        }
    };
}

macro_rules! nvapi {
    ($($tt:tt)*) => {
        nvapi_macros::nvapi! {
            $($tt)*
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

