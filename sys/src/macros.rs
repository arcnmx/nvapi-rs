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

        unsafe impl zerocopy::FromBytes for $name {
            fn only_derive_is_allowed_to_implement_this_trait() where Self: Sized { }
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

        impl $name {
            pub fn zeroed() -> Self {
                zerocopy::FromBytes::new_zeroed()
            }
        }
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
    // declare NvAPI fn
    (
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident($($args:tt)*) -> $ret:ty;

        $($rest_body:tt)*
    ) => {
        nvapi! { @entry
            $(#[$meta])*
            pub fn(unsafe) $fn($($args)*) -> $ret;

            $($rest_body)*
        }
    };
    (
        $(#[$meta:meta])*
        pub fn $fn:ident($($args:tt)*) -> $ret:ty;

        $($rest_body:tt)*
    ) => {
        nvapi! { @entry
            $(#[$meta])*
            pub fn() $fn($($args)*) -> $ret;

            $($rest_body)*
        }
    };
    (@entry
        $(#[$meta:meta])*
        pub fn($($unsafe:ident)?) $fn:ident($($args:tt)*) -> $ret:ty;

        $($rest_body:tt)*
    ) => {
        nvapi! { @fn
            $(#[$meta])*
            pub unsafe fn $fn($($args)*) -> $ret;
        }

        nvapi! { @scan_args
            $(#[$meta])*
            (( // $rest (double-wrapped to avoid ambiguity when passing around
                ($($rest_body)*) // body extra config provided by caller, to be parsed later as needed
                ($($unsafe)?) // whether the fn (once processed for out/SV args) is safe or not
            ))
            fn $fn($($args)*) -> $ret;
        }
    };
    // declare NvAPI fn with type alias
    (
        pub type $name:ident = extern "C" fn($($args:tt)*) -> $ret:ty;

        $(#[$meta:meta])*
        pub unsafe fn $fn:ident;

        $($rest_body:tt)*
    ) => {
        nvapi! {
            $(#[$meta])*
            pub unsafe fn $fn($($args)*) -> $ret;

            $($rest_body)*
            pub type fn $name;
        }
    };
    (
        pub type $name:ident = extern "C" fn($($args:tt)*) -> $ret:ty;

        $(#[$meta:meta])*
        pub fn $fn:ident;

        $($rest_body:tt)*
    ) => {
        nvapi! { @entry
            $(#[$meta])*
            pub fn() $fn($($args)*) -> $ret;

            $($rest_body)*
            pub type fn $name;
        }
    };
    // implementation of a specific NvAPI fn
    (@fn
        $(#[$meta:meta])*
        pub unsafe fn $fn:ident($($arg:ident$(@$($_:ident)*)?: $arg_ty:ty),*) -> $ret:ty;
    ) => {
        impl crate::apifn::NvapiInterface for crate::nvid::api::$fn {
            const API: crate::nvid::Api = crate::nvid::Api::$fn;
            type Fn = extern "C" fn($($arg: $arg_ty),*) -> $ret;

            unsafe fn fn_from_ptr(ptr: std::ptr::NonNull<()>) -> Self::Fn {
                std::mem::transmute(ptr)
            }
        }

        $(#[$meta])*
        #[inline]
        pub unsafe fn $fn($($arg: $arg_ty),*) -> $ret {
            $crate::nvid::$fn.nvapi($($arg),*)
        }

        impl crate::nvid::api::$fn {
            $(#[$meta])*
            #[inline]
            pub unsafe fn nvapi($($arg: $arg_ty),*) -> $ret {
                $fn($($arg),*)
            }
        }

        impl crate::nvid::interface::$fn {
            $(#[$meta])*
            pub unsafe fn nvapi(&self $(, $arg: $arg_ty)*) -> $ret {
                self.query_map(|interface| interface($($arg),*))
            }
        }
    };
    // scan through args for special tags on the argument ident:
    // * @self exposes the api on a specific type
    // * @out indicates that the input data isn't important
    // * @StructVersion marks the argument as implementing `StructVersion`
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($($args:tt)*) -> $ret:ty;
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($args)*) -> $ret;
            () // all (extern "C" args)
            () // input args (omit outputs)
            () // out args
            () // cstr args
            (
                () // non-self args
                () // self arg
            )
            (
                () // non-StructVersion args
                () // StructVersion arg
            )
        }
    };
    // @self
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident@self: $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) (/*args_self*/))
        (($($args_notsv:tt)*) $($args_sv:tt)*)
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: $arg_ty,)
            ($($args_input)* $arg: $arg_ty,)
            ($($args_out)*)
            ($($args_cstr)*)
            (($($args_notself)*) ($arg: $arg_ty))
            (($($args_notsv)* $arg: $arg_ty,) $($args_sv)*)
        }
    };
    // @out
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident@out: *mut $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) $($args_sv:tt)*)
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: *mut $arg_ty,)
            ($($args_input)*)
            ($($args_out)* $arg: *mut $arg_ty,)
            ($($args_cstr)*)
            (($($args_notself)* /*$arg: *mut $arg_ty,*/) $($args_self)*)
            (($($args_notsv)* /*$arg: *mut $arg_ty,*/) $($args_sv)*)
        }
    };
    // @StructVersion: *const (input)
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident@StructVersion: *const $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) (/*args_sv*/))
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: *const $arg_ty,)
            ($($args_input)* $arg: &SV,)
            ($($args_out)*)
            ($($args_cstr)*)
            (($($args_notself)* $arg: &SV,) $($args_self)*)
            (($($args_notsv)* $arg: &SV,) ($arg: *const $arg_ty))
        }
    };
    // @StructVersion: *mut (in/out)
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident@StructVersion: *mut $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) (/*args_sv*/))
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: *mut $arg_ty,)
            ($($args_input)* $arg: &mut SV,)
            ($($args_out)*)
            ($($args_cstr)*)
            (($($args_notself)* $arg: &mut SV,) $($args_self)*)
            (($($args_notsv)* $arg: &mut SV,) ($arg: *mut $arg_ty))
        }
    };
    // @StructVersionOut
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident@StructVersionOut: *mut $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) (/*args_sv*/))
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: *mut $arg_ty,)
            ($($args_input)*)
            ($($args_out)* $arg: *mut SV,)
            ($($args_cstr)*)
            (($($args_notself)*) $($args_self)*)
            (($($args_notsv)*) ($arg@SV: *mut $arg_ty))
        }
    };
    // C-strings
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident: *const c_char $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) $($args_sv:tt)*)
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: *const c_char,)
            ($($args_input)* $arg: &std::ffi::CStr,)
            ($($args_out)*)
            ($($args_cstr)* $arg,)
            (($($args_notself)* $arg: &std::ffi::CStr,) $($args_self)*)
            (($($args_notsv)* $arg: &std::ffi::CStr,) $($args_sv)*)
        }
    };
    // fallback regular args
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident($arg:ident: $arg_ty:ty $(,$($args:tt)*)?) -> $ret:ty;
        ($($args_all:tt)*) ($($args_input:tt)*)
        ($($args_out:tt)*)
        ($($args_cstr:tt)*)
        (($($args_notself:tt)*) $($args_self:tt)*)
        (($($args_notsv:tt)*) $($args_sv:tt)*)
    ) => {
        nvapi! { @scan_args
            $(#[$meta])* ($rest)
            fn $fn($($($args)*)?) -> $ret;
            ($($args_all)* $arg: $arg_ty,)
            ($($args_input)* $arg: $arg_ty,)
            ($($args_out)*)
            ($($args_cstr)*)
            (($($args_notself)* $arg: $arg_ty,) $($args_self)*)
            (($($args_notsv)* $arg: $arg_ty,) $($args_sv)*)
        }
    };
    // base case once all arguments are parsed
    (@scan_args
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident() -> $ret:ty;
        $args_all:tt $args_input:tt
        $args_out:tt
        $args_cstr:tt
        $args_self:tt
        $args_sv:tt
    ) => {
        nvapi! { @parse_args(self)$args_self
            $(#[$meta])* ($rest)
            fn $fn() -> $ret;
            $args_all $args_input $args_out $args_cstr $args_self $args_sv
        }
        nvapi! { @parse_args(StructVersion)$args_sv
            $(#[$meta])* ($rest)
            fn $fn() -> $ret;
            $args_all $args_input $args_out $args_cstr $args_self $args_sv
        }
        nvapi! { @parse_args(rest)($rest)
            $(#[$meta])* ($rest)
            fn $fn() -> $ret;
            $args_all $args_input $args_out $args_cstr $args_self $args_sv
        }
    };
    // parse special args that have been separated via @scan_args
    (@parse_args(self)($args_notself:tt ()) $($rest:tt)*) => { };
    (@parse_args(self)($args_notself:tt $args_self:tt)
        $(#[$meta:meta])* ((
            (
                impl self {
                    pub fn $self_fn:ident;
                }
                $($rest_body:tt)*
            )
            $($rest:tt)*
        )) $($rest_:tt)*
    ) => {
        nvapi! { @parse_args(self)($args_notself $args_self ($self_fn))
            $(#[$meta])* ((
                (
                    $($rest_body)*
                )
              $($rest)*
            ))
            $($rest_)*
        }
    };
    (@parse_args(self)($args_notself:tt $args_self:tt)
        $(#[$meta:meta])* ($rest:tt)
        fn $fn:ident() $($rest_:tt)*
    ) => {
        nvapi! { @parse_args(self)($args_notself $args_self ($fn))
            $(#[$meta])* ($rest)
            fn $fn()
            $($rest_)*
        }
    };
    (@parse_args(self)
        (
            ($($arg_notself:ident: $arg_notself_ty:ty,)*)
            ($arg_self:ident: $self_ty:ty)
            ($self_fn:ident)
        )
        $(#[$meta:meta])*
        (($rest_body:tt ($($unsafe:ident)?) $($rest:tt)*))
        fn $fn:ident() -> $ret:ty;
        ($($arg:ident: $arg_ty:ty,)*)
        ($($arg_input:ident: $arg_input_ty:ty,)*)
        $args_out:tt
        $args_cstr:tt
        $args_self:tt
        (($($arg_notsv:ident: $arg_notsv_ty:ty,)*) ($($arg_sv:ident$(@$arg_sv_out:ident)?: *$arg_sv_mut:tt $arg_sv_ty:ty)?))
    ) => {
        impl $self_ty {
            $(#[$meta])*
            #[allow(unused_unsafe)]
            pub $($unsafe)? fn $self_fn$(<const VER: u16, SV: StructVersion<VER, Storage=$arg_sv_ty>>)?(self $(, $arg_notself: $arg_notself_ty)* /*$(, $arg_sv: &$arg_sv_mut SV)?*/)
            -> $crate::Result<nvapi! { @out(return) $args_out }> where
                $($($arg_sv_out: VersionedStructField + zerocopy::FromBytes,)?)?
                $($arg_sv_ty: StructVersionInfo<VER, Struct=SV>,)?
            {
                let $arg_self = self;
                unsafe {
                    $crate::nvid::$fn.call($($arg_input),*)
                }
            }
        }
    };
    (@storage_ptr($arg_sv:ident: *mut)) => {
        $arg_sv.as_storage_ptr_mut()
    };
    (@storage_ptr($arg_sv:ident: *const)) => {
        $arg_sv.as_storage_ptr()
    };
    (@out(let $out:ident)() $args_sv:tt) => {
        let $out = ();
    };
    (@out(let $out:ident)($($arg_out:ident: *mut $arg_out_ty:ty,)*) $args_sv:tt) => {
        #[allow(unused_parens)]
        let mut $out: ($($arg_out_ty),*) = (
            $(
                <$arg_out_ty as zerocopy::FromBytes>::new_zeroed()
            ),*
        );
        #[allow(unused_parens)]
        let ($($arg_out),*) = &mut $out;

        nvapi! { @out(sv) $args_sv }
    };
    (@out(sv)($args_notsv:tt ($arg_sv:ident@SV: *mut $arg_sv_ty:ty))) => {
        //StructVersion::<VER>::$arg_sv.init_version();
        $arg_sv.init_version();
    };
    (@out(sv)($args_notsv:tt ($arg_sv:ident: *mut $arg_sv_ty:ty))) => {
        //$crate::nvapi::StructVersion::<VER>::$arg_sv.init_version();
        // TODO: assert version matches expected!!!
    };
    (@out(sv)($args_notsv:tt ())) => {
    };
    (@out(return)($arg_out:ident: *mut $arg_out_ty:ty,)) => {
        $arg_out_ty
    };
    (@out(return)($($arg_out:ident: *mut $arg_out_ty:ty,)*)) => {
        ($($arg_out_ty),*)
    };
    (@cstr($($arg_cstr:ident,)*)) => {
        $(
            let $arg_cstr = $arg_cstr.as_ptr();
        )*
    };
    (@parse_args(StructVersion)($args_notsv:tt ())
        $(#[$meta:meta])*
        (($rest_body:tt ($($unsafe:ident)?) $($rest:tt)*))
        fn $fn:ident() -> $ret:ty;
        ($($arg:ident: $arg_ty:ty,)*)
        ($($arg_input:ident: $arg_input_ty:ty,)*)
        $args_out:tt
        $args_cstr:tt
        $args_self:tt $args_sv:tt
    ) => {
        impl crate::nvid::interface::$fn {
            #[allow(unused_unsafe)]
            pub $($unsafe)? fn call(&self $(, $arg_input: $arg_input_ty)*) -> $crate::Result<nvapi! { @out(return) $args_out }> {
                nvapi! { @out(let out) $args_out $args_sv }
                nvapi! { @cstr $args_cstr }
                unsafe {
                    self.nvapi($($arg),*)
                        //.to_error_result(Api::$fn)
                        .to_result()
                        .map(|()| out)
                }
            }
        }
    };
    (@parse_args(StructVersion)
        (
            ($($arg_notsv:ident: $arg_notsv_ty:ty,)*)
            ($sv_arg:ident$(@$sv_out:ident)?: *$sv_mut:tt $sv_ty:ty)
        )
        $(#[$meta:meta])*
        (($rest_body:tt ($($unsafe:ident)?) $($rest:tt)*))
        fn $fn:ident() -> $ret:ty;
        ($($arg:ident: $arg_ty:ty,)*)
        ($($arg_input:ident: $arg_input_ty:ty,)*)
        $args_out:tt
        $args_cstr:tt
        ($args_self:tt ($($arg_self:ident: $arg_self_ty:ty)?)) $args_sv:tt
    ) => {
        impl crate::nvid::interface::$fn {
            #[allow(unused_unsafe)]
            pub $($unsafe)? fn call<const VER: u16, SV: StructVersion<VER, Storage=$sv_ty>>(&self $(, $arg_input: $arg_input_ty)*) -> $crate::Result<nvapi! { @out(return) $args_out }> where
                $($sv_out: VersionedStructField + zerocopy::FromBytes)?
            {
                nvapi! { @out(let out) $args_out $args_sv }
                nvapi! { @cstr $args_cstr }
                let $sv_arg = nvapi!(@storage_ptr($sv_arg: *$sv_mut));
                unsafe {
                    self.nvapi($($arg),*)
                        //.to_error_result(Api::$fn)
                        .to_result()
                        .map(|()| out)
                }
            }
        }
    };
    (@parse_args(rest)
        ((
            (
                $(
                    impl self { $($self_fn:tt)* }
                )?
                $(
                    pub type fn $fn_typealias:ident;
                )?
            )
            $($rest:tt)*
        ))
        $(#[$meta:meta])* ($rest_:tt)
        fn $fn:ident() -> $ret:ty;
        $args:tt $args_input:tt $args_out:tt $args_cstr:tt $args_self:tt $args_sv:tt
    ) => {
        nvapi! { @type fn $($fn_typealias)?$args -> $ret }
    };
    (@type fn ($($args:tt)*) $($rest:tt)*) => { };
    (@type fn $fn_typealias:ident($($arg:ident: $arg_ty:ty,)*) -> $ret:ty) => {
        pub type $fn_typealias = extern "C" fn($($arg: $arg_ty),*) -> $ret;
    };
}

macro_rules! nvversion {
    (_$(($($api:tt)*))?: $latest:ident($latest_ver:literal$($rest:tt)*) $($tt:tt)*) => {
        nvversion! { @cont(_)($($($api)*)?) $latest $latest($latest_ver) $latest($latest_ver$($rest)*) $($tt)* }
    };
    ($name:ident$(($($api:tt)*))?: $latest:ident($latest_ver:literal$($rest:tt)*) $($tt:tt)*) => {
        nvversion! { @cont($name)($($($api)*)?) $name $latest($latest_ver) $latest($latest_ver$($rest)*) $($tt)* }
    };
    (@cont($defname:tt)$api:tt $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?) => {
        nvversion! { @impl StructVersion $target($ver)$api }
        nvversion! { @rest($defname)$api $name $latest($latest_ver) $latest($latest_ver) $target($ver)($($($rest)*)?) }
        nvversion! { @type($defname) $latest $target }
    };
    (@cont($defname:tt)$api:tt $name:ident $latest:ident($latest_ver:literal) $($target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?),+) => {
        nvversion! { @find_oldest($defname)$api() $name $latest($latest_ver) $($target($ver$(;$($rest)+)?)$(= $sz)?),+ }
    };
    (@find_oldest($defname:tt)$api:tt($($older:tt)*) $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?, $($rest_:tt)*) => {
        nvversion! { @find_oldest($defname)$api($($older)* $target($ver$(;$($rest)+)?)$(= $sz)?,) $name $latest($latest_ver) $($rest_)* }
    };
    (@find_oldest($defname:tt)$api:tt($($older:tt)*) $name:ident $latest:ident($latest_ver:literal) $target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?) => {
        nvversion! { @find_oldest($defname)$api($($older)* $target($ver$(;$($rest)+)?)$(= $sz)?,)($target($ver)) $name $latest($latest_ver) }
    };
    (@find_oldest($defname:tt)$api:tt($($target:ident($ver:literal$(;$($rest:tt)+)?)$(= $sz:expr)?,)+)($oldest:ident($oldest_ver:literal)) $name:ident $latest:ident($latest_ver:literal)) => {
        $(
            nvversion! { @impl StructVersion $target($ver)$api $latest($latest_ver) $oldest($oldest_ver) }
            nvversion! { @rest($defname)$api $name $latest($latest_ver) $oldest($oldest_ver) $target($ver)($($($rest)*)?) }
        )*
        $(
            impl StructVersionInfo<$ver> for $oldest {
                type Struct = $target;
                type Storage = $oldest;
            }
        )*
        nvversion! { @type($defname) $latest $oldest }
    };
    (@rest($defname:tt)$api:tt $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)()) => {
        nvversion! { @latest($defname)$api $target($ver) $oldest($oldest_ver) }
    };
    (@rest($defname:tt)$api:tt $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)(@inherit($id:ident: $v1:ty)$($rest:tt)*)) => {
        nvinherit! { $target($id: $v1) }
        nvversion! { @rest($defname)$api $name $latest($latest_ver) $oldest($oldest_ver) $target($ver)($($rest)*) }
    };
    (@rest($defname:tt)$api:tt $name:ident $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal) $target:ident($ver:literal)(@old)) => {
        nvversion! { @old($defname)$api $target($ver) $latest($latest_ver) $oldest($oldest_ver) }
    };
    // @rest base case for latest version
    (@latest($defname:tt)$api:tt $target:ident($ver:literal) $oldest:ident($oldest_ver:literal)) => {
        nvversion! { @impl Default $target($ver) }
        //nvversion! { @impl StructVersion $target($ver) }

        /*impl StructVersionInfo<$ver> for $target {
            type Oldest = $oldest;
            type Storage = $oldest;
        }*/
    };
    // @rest base case for old versions
    (@old($defname:tt)$api:tt $target:ident($ver:literal) $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal)) => {
    };
    // this target is the latest version
    /*(@impl StructVersion $target:ident($ver:literal)) => {
        nvversion! { @impl StructVersion($ver)(NvVersion) $target(NvVersion::with_struct::<$target>($ver)) }
    };*/
    // this version is one of many
    (@impl StructVersion $target:ident($ver:literal)$api:tt $latest:ident($latest_ver:literal) $oldest:ident($oldest_ver:literal)) => {
        nvversion! { @impl StructVersion($ver)($oldest)$api $target(NvVersion::with_struct::<$target>($ver)) }
    };
    /* this version is not the latest
    (@impl StructVersion($t:literal) $target:ident($ver:literal) $latest:ident($latest_ver:literal)) => {
        /*impl StructVersion<0> for $target {
            const NVAPI_VERSION: NvVersion = <Self as StructVersion<$ver>>::NVAPI_VERSION;
            const API: Option<Api> = <Self as StructVersion<$ver>>::API;
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
    (@impl StructVersion $target:ident($ver:literal)$api:tt) => {
        nvversion! { @impl StructVersion($ver)($target)$api $target(NvVersion::with_struct::<$target>($ver)) }

        impl StructVersionInfo<$ver> for $target {
            type Struct = Self;
            type Storage = Self;
        }
    };
    (@impl StructVersion($t:literal)($storage:ty)() $target:ident($ver:expr)) => {
        compile_error!("expected associated NvAPI")
    };
    (@impl StructVersion::API_SET($t:literal)($storage:ty)($api:ident, $api_set:ident) $target:ident($ver:expr)) => {
        const API_SET: Option<Api> = Some(Api::$api_set);
    };
    (@impl StructVersion::API_SET($t:literal)($storage:ty)($api:ident) $target:ident($ver:expr)) => {
        const API_SET: Option<Api> = None;
    };
    (@impl StructVersion($t:literal)($storage:ty)($api:ident$(, $api_:ident)*) $target:ident($ver:expr)) => {
        impl StructVersion<$t> for $target {
            const NVAPI_VERSION: NvVersion = $ver;
            const API: Api = Api::$api;
            nvversion! { @impl StructVersion::API_SET($t)($storage)($api$(, $api_)*) $target($ver) }
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
macro_rules! nventries {
    ($name:ident.$field:ident[..$count:ident]@($get:ident/$set:ident/$mut_:ident): [$item:ty; $len:expr]) => {
        impl $name {
            pub fn $get(self) -> Truncated<Array<[$item; $len]>, std::ops::RangeTo<usize>> {
                self.$field.truncate_to(self.$count as usize)
            }

            pub fn $field(&self) -> Truncated<&Array<[$item; $len]>, std::ops::RangeTo<usize>> {
                self.$field.truncate_to_ref(self.$count as usize)
            }

            pub fn $mut_(&mut self) -> Truncated<&mut Array<[$item; $len]>, std::ops::RangeTo<usize>> {
                self.$field.truncate_to_mut(self.$count as usize)
            }

            pub fn $set<I: IntoIterator<Item=$item>>(&mut self, iter: I) {
                self.$count = 0;
                for (entry, item) in self.$field.iter_mut().zip(iter) {
                    *entry = item;
                    self.$count += 1;
                }
            }
        }
    };
    ($name:ident.$field:ident@filter($($filter:expr)?)($get:ident/$set:ident/$mut_:ident): [($tag:ty, $item:ty); $len:expr]) => {
        impl $name {
            pub fn $field(&self) -> impl Iterator<Item = ($tag, $item)> {
                let iter = self.$field.into_iter().enumerate();
                $(let iter = iter.filter(|(_, item)| ($filter)(item));)?
                iter.map(|(tag, item)| ((tag as i32).into(), item))
            }
        }
    };
    ($name:ident.$field:ident@masked(.$mask:ident$(@$masked:expr)?)($get:ident/$set:ident/$mut_:ident): [$item:ty; $len:expr]) => {
        impl $name {
            pub fn $field(&self) -> Truncated<&Array<[$item; $len]>, &ClockMask$(<$masked>)?> {
                self.$field.truncated_ref(&self.$mask)
            }

            pub fn $get(self) -> impl Iterator<Item = $item> {
                let $field = self.$field;
                let $mask: Vec<_> = self.$mask.into_iter().collect();
                $mask.into_iter()
                    .map(move |i| *$field.get(i).unwrap())
            }
        }
    };
}

macro_rules! nvtag {
    (@rest $name:ident.$field:ident: $repr:ident / $id:ident) => { };
    (@rest $name:ident.$field:ident: $repr:ident / $id:ident @TaggedData) => {
        impl crate::tagged::TaggedData for $name {
            type Repr = $repr;
            type Id = $id;

            fn tag(&self) -> Self::Repr {
                self.$field
            }
        }
    };
    ($name:ident.$field:ident@$accessor:ident: $repr:ident / $id:ident $($rest:tt)*) => {
        nvtag! { @rest $name.$field: $repr / $id $($rest)* }
    };
    ($name:ident.$field:ident: $repr:ident / $id:ident $($rest:tt)*) => {
        nvtag! { $name.$field@$field: $repr / $id $($rest)* }
    };
}
