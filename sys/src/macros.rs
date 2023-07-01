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
        #[$crate::nvapi::NvStruct]
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
    ($($tt:tt)*) => {
        nvapi_macros::nvversion! {
            $($tt)*
        }
    };
}
