/*macro_rules! nvcall {
    ($nvapi:ident@get_value($($arg:expr),*$(,)?) $($tt:tt)*) => {
        {
            let mut out = Default::default();
            nvcall!($nvapi($($arg,)* &mut out) => map(move |()| out) $($tt)*)
        }
    };
    ($nvapi:ident@get{$init:expr}($($arg:expr),*$(,)?) $($tt:tt)*) => {
        {
            let mut out = $init;
            nvcall!($nvapi($($arg,)* crate::sys::version::StructVersion::as_storage_ptr_mut(&mut out)) => map(move |()| out) $($tt)*)
        }
    };
    ($nvapi:ident@get2{$init:expr}($($arg:expr),*$(,)?) $($tt:tt)*) => {
        {
            let mut out = $init;
            nvcall!($nvapi($($arg,)* &mut out.0, &mut out.1) => map(move |()| out) $($tt)*)
        }
    };
    ($nvapi:ident@get($($arg:expr),*$(,)?) $($tt:tt)*) => {
        nvcall!($nvapi@get{Default::default()}($($arg),*) $($tt)*)
    };
    ($nvapi:ident@get2($($arg:expr),*$(,)?) $($tt:tt)*) => {
        nvcall!($nvapi@get2{(Default::default(), Default::default())}($($arg),*) $($tt)*)
    };
    ($nvapi:ident($($arg:expr),*$(,)?) $($tt:tt)*) => {
        {
            nvcall!(@post ($crate::status_result($crate::sys::Api::$nvapi, $crate::sys::api::all::$nvapi($($arg,)*))) $($tt)*)
        }
    };
    (@post ($res:expr) => into $($tt:tt)*) => {
        nvcall!(@post (
            ($res).map(Into::into)
        ) $($tt)*)
    };
    (@post ($res:expr) => err $($tt:tt)*) => {
        nvcall!(@post (
            ($res).map_err(Into::into)
        ) $($tt)*)
    };
    (@post ($res:expr) => map($map:expr) $($tt:tt)*) => {
        nvcall!(@post (
            ($res).map($map)
        ) $($tt)*)
    };
    (@post ($res:expr) => raw $($tt:tt)*) => {
        nvcall!(@post (
            ($res)
                .map_err(Into::into)
                .and_then(|v| TryInto::try_into(&v).map_err(Into::into))
        ) $($tt)*)
    };
    (@post ($res:expr) => try $($tt:tt)*) => {
        nvcall!(@post (
            ($res)
                .map_err(Into::into)
                .and_then(|v| TryInto::try_into(v).map_err(Into::into))
        ) $($tt)*)
    };
    (@post ($res:expr)) => {
        $res
    };
}*/

/*macro_rules! nvconv {
    (@impl_raw $sys:ty as $nvapi:ty) => {
        impl crate::RawConversion for $sys {
            type Target = $nvapi;
            type Error = <$sys as TryInto<$nvapi>>::Error;
        }
    };
    (fn try_from($this:ident: &$life:lifetime $sys:ty) -> Result<$nvapi:ty, $err:ty> $try_from:block) => {
        nvconv! { $sys as $nvapi }

        impl<$life> TryFrom<&$life $sys> for $nvapi {
            type Error = $err;

            #[allow(non_snake_case)] // TODO: shouldn't need this...
            fn try_from($this: &$life $sys) -> Result<Self, Self::Error>
                $try_from
        }
    };
    (fn try_from($this:ident: &$sys:ty) -> Result<$nvapi:ty, $err:ty> $try_from:block) => {
        nvconv! { fn try_from($this: &'a $sys) -> Result<$nvapi, $err> $try_from }
    };
    (fn from($this:ident: &$life:lifetime $sys:ty) -> $nvapi:ty $try_from:block) => {
        nvconv! { $sys as $nvapi | @From }

        impl<$life> From<&$life $sys> for $nvapi {
            #[allow(non_snake_case)] // TODO: shouldn't need this...
            fn from($this: &$life $sys) -> Self
                $try_from
        }
    };
    (fn from($this:ident: &$sys:ty) -> $nvapi:ty $try_from:block) => {
        nvconv! { fn from($this: &'a $sys) -> $nvapi $try_from }
    };
    ($sys:ty as $nvapi:ty | @From) => {
        nvconv! { @impl_raw $sys as $nvapi }

        impl From<$sys> for $nvapi {
            fn from(data: $sys) -> $nvapi {
                From::from(&data)
            }
        }
    };
    ($sys:ty as $nvapi:ty) => {
        nvconv! { @impl_raw $sys as $nvapi }

        impl TryFrom<$sys> for $nvapi {
            type Error = <&'static $sys as TryInto<$nvapi>>::Error;

            fn try_from(data: $sys) -> Result<$nvapi, Self::Error> {
                TryFrom::try_from(&data)
            }
        }
    };
}*/

macro_rules! allow_version_compat {
    (try $res:expr) => {
        match allow_version_compat!($res) {
            Ok(None) => (),
            Ok(Some(res)) => return Ok(res),
            Err(e) => return Err(e),
        }
    };
    ($res:expr) => {
        $res.map(Some)
            .or_else(|e| e.allow_version_incompat().map(|()| None))
    };
}

macro_rules! nvwrap {
    (impl Deref() for ) => { };
    (impl Deref() for @ $($tt:tt)*) => { };
    (impl Deref($field:ident: $target:ty) for $name:ident) => {
        impl std::ops::Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                (&self.sys().$field).into()
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                (&mut self.sys_mut().$field).into()
            }
        }
    };
    (impl Deref($target:ty) for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
        impl std::ops::Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                match self { $(
                    Self::$variant(data) => &*data,
                )* }
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self { $(
                    Self::$variant(data) => &mut *data,
                )* }
            }
        }
    };
    (impl TaggedFrom() for $($tt:tt)*) => { };
    (impl TaggedFrom($tag:ty) for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
        $(
            impl From<crate::types::Tagged<$tag, $data>> for crate::types::Tagged<$tag, $name> {
                fn from(tagged: crate::types::Tagged<$tag, $data>) -> Self {
                    Self::new(tagged.tag, tagged.value.into())
                }
            }
        )*
    };
    (impl TaggedData $tagged:tt for @ $($tt:tt)*) => { };
    (impl TaggedData($($tagged:tt)*) for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
        impl<R: Copy + Ord, I: Copy + Ord + TryFrom<R>> crate::sys::tagged::TaggedData for $name where $(
            $data: crate::sys::tagged::TaggedData<Repr=R, Id=I>,
        )* {
            type Repr = R;
            type Id = I;

            fn tag(&self) -> Self::Repr {
                match self { $(
                    Self::$variant(data) => crate::sys::tagged::TaggedData::tag(data),
                )* }
            }
        }

        impl<I> From<crate::types::Tagged<I, $name>> for $name {
            fn from(tag: crate::types::Tagged<I, $name>) -> Self {
                tag.value
            }
        }

        impl From<$name> for crate::types::Tagged<<$name as crate::sys::tagged::TaggedData>::Repr, $name> {
            fn from(value: $name) -> Self {
                crate::types::Tagged::with_value(value)
            }
        }

        nvwrap! {
            impl TaggedDataTryFrom($($tagged)*) for $name @{ $($variant($data)),+ }
        }
    };
    (impl TaggedDataTryFrom(@$repr:ty) for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
    };
    (impl TaggedDataTryFrom() for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
        /*impl TryFrom<$name> for crate::types::Tagged<<$name as crate::sys::TaggedData>::Id, $name> {
        //impl TryInto<crate::types::Tagged<<$name as crate::sys::TaggedData>::Id, $name>> for $name
            type Error = <<$name as crate::sys::TaggedData>::Repr as TryInto<<$name as crate::sys::TaggedData>::Id>>::Error;
        }*/
    };
    (impl IntoIterator() for ) => { };
    (impl IntoIterator($into_iter:ident() -> $item:ty) for $name:ident ) => {
        impl IntoIterator for $name {
            type Item = $item;
            type IntoIter = Box<dyn Iterator<Item=Self::Item>>;

            fn into_iter(self) -> Self::IntoIter {
                Box::new(self.$into_iter())
            }
        }
    };
    (impl StructVersion($name:ident)($latest:ident($latest_data:ident)) for @ $($tt:tt)*) => {
        impl Default for $name {
            fn default() -> Self {
                Self::$latest(zerocopy::FromBytes::new_zeroed())
            }
        }
    };
    (impl StructVersion($name_:ident)($latest:ident($latest_data:ty)) for $name:ident @{ $($variant:ident($data:ty)),+ }) => {
        impl<const VER: u16> crate::sys::version::StructVersion<VER> for $name where
            $latest_data: crate::sys::version::StructVersion<VER>,
        {
            const NVAPI_VERSION: crate::sys::version::NvVersion = <$latest_data as crate::sys::version::StructVersion<VER>>::NVAPI_VERSION;
            const API: crate::sys::nvid::Api = <$latest_data as crate::sys::version::StructVersion<VER>>::API;
            const API_SET: Option<crate::sys::nvid::Api> = <$latest_data as crate::sys::version::StructVersion<VER>>::API_SET;
            type Storage = <$latest_data as crate::sys::version::StructVersion<VER>>::Storage;
/*
            fn storage_ref(storage: &Self::Storage) -> Option<&Self> {
                /* TODO!!!
                match crate::sys::version::VersionedStruct::nvapi_version(storage) {
                    $(
                        <$data as crate::sys::version::StructVersion>::NVAPI_VERSION =>
                            <$data as crate::sys::version::StructVersion>::storage_ref(storage)
                                .map(|data| data.sys()),
                    )*
                    _ => None,
                }*/
                None
            }*/

            /*fn storage_mut(storage: &mut Self::Storage) -> Option<&mut Self> {
                /* TODO!!
                match crate::sys::version::VersionedStruct::nvapi_version(storage) {
                    $(
                        <$data as crate::sys::version::StructVersion>::NVAPI_VERSION =>
                            <$data as crate::sys::version::StructVersion>::storage_mut(storage)
                                .map(Self::$variant),
                    )*
                    _ => None,
                }*/
                None
            }*/
        }
        impl crate::sys::version::VersionedStruct for $name {
            fn nvapi_version(&self) -> crate::sys::version::NvVersion {
                match self { $(
                    Self::$variant(data) => crate::sys::version::VersionedStruct::nvapi_version(data),
                )* }
            }
        }
        /*$(
            impl crate::sys::version::StructVersion for $data {
                const NVAPI_VERSION: crate::sys::version::NvVersion = <<$data as crate::types::IsNvData>::Data as crate::sys::version::StructVersion>::NVAPI_VERSION;
                type Storage = $name;

                fn storage_ref(storage: &Self::Storage) -> Option<&Self> {
                    #[allow(unreachable_patterns)]
                    match storage {
                        $name::$variant(data) => Some(data),
                        _ => None,
                    }
                }

                fn storage_mut(storage: &mut Self::Storage) -> Option<&mut Self> {
                    #[allow(unreachable_patterns)]
                    match storage {
                        $name::$variant(data) => Some(data),
                        _ => None,
                    }
                }
            }
        )**/

        impl Default for $name {
            fn default() -> Self {
                Self::$latest(Default::default())
            }
        }
    };
    (impl fn @{ $($var:tt)* } $($tt:tt)* ) => { };
    (impl fn $name:ident @{ $($var:tt)* } { } ) => { };
    (@impl_item fn $name:ident @{ $($variant:ident($data:ty)),+ } ($impl_fn:ident)(&self)($impl_ret:ty)) => {
        pub fn $impl_fn(&self) -> $impl_ret {
            match self { $(
                Self::$variant(data) => data.$impl_fn().into(),
            )* }
        }
    };
    (@impl_item fn $name:ident @{ $($variant:ident($data:ty)),+ } ($impl_fn:ident)(&mut self $(, $arg:ident: $arg_ty:ty)*)($($impl_ret:ty)?)) => {
        pub fn $impl_fn(&mut self $(, $arg: $arg_ty)*) $(-> $impl_ret)? {
            todo!()
            /*match self { $(
                Self::$variant(data) => data.$impl_fn($($arg),*).into(),
            )* }*/
        }
    };
    (@impl_item fn $name:ident @{ $($variant:ident($data:ty)),+ } ($impl_fn:ident)(@iter $($args:tt)*)($item:ty)) => {
        nvwrap! { @impl_item fn $name @{ $($variant($data)),+ } ($impl_fn)(@map(
            |iter| Box::new(iter.map(Into::into)) as Box<dyn Iterator<Item=$item>>
        ) $($args)*)(impl Iterator<Item=$item>) }
    };
    (@impl_item fn $name:ident @{ $($variant:ident($data:ty)),+ } ($impl_fn:ident)(&self)($impl_ret:ty)) => {
        pub fn $impl_fn(&self) -> $impl_ret {
            match self { $(
                Self::$variant(data) => data.$impl_fn().into(),
            )* }
        }
    };
    (@impl_item fn $name:ident @{ $($variant:ident($data:ty)),+ } ($impl_fn:ident)(@map(|$map_id:ident|$map:expr) self)($impl_ret:ty)) => {
        pub fn $impl_fn(self) -> $impl_ret {
            match self { $(
                Self::$variant(data) => {
                    let $map_id = data.$impl_fn();
                    $map
                },
            )* }
        }
    };
    (impl fn $name:ident @{ $($variant:ident($data:ty)),+ } { ($(#[$($impl_meta:meta)*])*)($($impl_fn:tt)*); $($rest:tt)* }) => {
        impl $name {
            $(#[$($impl_meta)*])*
            nvwrap! { @impl_item fn $name @{ $($variant($data)),+ } $($impl_fn)* }
        }
        nvwrap! { impl fn $name @{ $($variant($data)),+ } { $($rest)* } }
    };
    (@variant_extra $name:ident $variant:ident($data:ident) {}) => { };
    (@variant_extra $name:ident $variant:ident($data:ident) {
        @type = NvData<$data_ty:ty> { $($fields:tt)* },
        $($rest:tt)*
    }) => {
        nvwrap! { pub type $data = NvData<$data_ty> { $($fields)* }; }
        nvwrap! { @variant_extra $name $variant($data) {$($rest)*} }
    };
    (@enum($latest:ident($latest_data:ident))
        $(#[$($meta:meta)*])*
        pub enum $name:ident {
            $($variant:ident($data:ident $({$($extra:tt)*})?),)*
        }

        $(impl @StructVersion for $name_impl_sv:ident { })?
        $(impl @TaggedData$((@$repr:ty))? for $name_impl_tag:ident { })?
        $(impl @TaggedFrom($taggedfrom_tag:ty) for $name_impl_taggedfrom:ident { })?
        $(impl @IntoIterator($into_iter:ident() -> $item:ty) for $name_impl_intoiter:ident { })?
        $(impl @Deref($deref_target:ty) for $name_impl_deref:ident { })?
        $(
            impl $name_impl:ident { $(
                $(#[$($impl_meta:meta)*])*
                pub fn $impl_fn:ident($($args:tt)*) -> $impl_ret:ty;
            )* }
        )?
    ) => {
        $(#[$($meta)*])*
        #[derive(Clone, Debug)]
        #[non_exhaustive]
        pub enum $name { $(
            $variant($data),
        )* }

        $(
            nvwrap! { @variant_extra $name $variant($data) {$($($extra)*)?} }
        )*

        nvwrap! { impl StructVersion($name)($latest($latest_data)) for $($name_impl_sv)? @{ $($variant($data)),* } }
        nvwrap! { impl TaggedData($($(@$repr)?)?) for $($name_impl_tag)? @{ $($variant($data)),* } }
        nvwrap! { impl TaggedFrom($($taggedfrom_tag)?) for $($name_impl_taggedfrom)? @{ $($variant($data)),* } }
        nvwrap! { impl IntoIterator($($into_iter() -> $item)?) for $($name_impl_intoiter)? }
        nvwrap! { impl Deref($($deref_target)?) for $($name_impl_deref)? @{ $($variant($data)),* } }
        nvwrap! { impl fn $($name_impl)? @{ $($variant($data)),* } { $($(
            ($(#[$($impl_meta)*])*)(($impl_fn)($($args)*)($impl_ret));
        )*)? } }

        $(
            impl From<$data> for $name {
                fn from(data: $data) -> Self {
                    Self::$variant(data)
                }
            }

            // TODO: $sys to enum
            /*impl From<<$data as crate::types::IsNvData>::Data> for $name {
                fn from(data: <$data as crate::types::IsNvData>::Data) -> Self {
                    Self::$variant(data.into())
                }
            }*/

            impl TryFrom<$name> for $data {
                type Error = crate::ArgumentRangeError;

                fn try_from(data: $name) -> std::result::Result<Self, Self::Error> {
                    #[allow(unreachable_patterns)]
                    match data {
                        $name::$variant(data) => Ok(data),
                        _ => Err(crate::ArgumentRangeError),
                    }
                }
            }

            // TODO: try enum to $sys
            /*impl TryFrom<$name> for <$data as crate::types::IsNvData>::Data {
                type Error = crate::ArgumentRangeError;

                fn try_from(data: $name) -> std::result::Result<Self, Self::Error> {
                    #[allow(unreachable_patterns)]
                    match data {
                        $name::$variant(data) => Ok(data.into_sys()),
                        _ => Err(crate::ArgumentRangeError),
                    }
                }
            }*/
        )*
    };
    (
        $(#[$($meta:meta)*])*
        pub enum $name:ident {
            $latest:ident($latest_data:ident $({$($latest_extra:tt)*})?)
            $($rest:tt)*
        }

        $($tt:tt)*
    ) => {
        nvwrap! { @enum($latest($latest_data))
            $(#[$($meta)*])*
            pub enum $name {
                $latest($latest_data $({$($latest_extra)*})?)
                $($rest)*
            }
            $($tt)*
        }
    };
    (@type field $name:ident $field:ident($($field_:tt)*)($field_ty:ty) () () () ()) => {
    };
    (@type field $name:ident $field:ident($($field_:tt)*)($field_ty:ty) () () (@mut($($set:ident)?)($mut_name:ident)($($mut_args:tt)*)($mut_body:block)) $($rest:tt)*) => {
        pub fn $mut_name($($mut_args)*) -> &mut $field_ty
            $mut_body

        $(
            pub fn $set(&mut self, value: $field_ty) {
                *self.$mut_name() = value;
            }
        )?

        nvwrap! { @type field $name $field($($field_)*)($field_ty) () () () $($rest)* }
    };
    (@type field $name:ident $field:ident(($($field_mut:ident)?)($($field_set:ident)?))($field_ty:ty) () () () (@sys@BoolU32($sys_field:ident $($sys_field_:ident)*)) $($rest:tt)*) => {
        pub fn $field(&self) -> $field_ty {
            self.sys().$sys_field.get()
        }

        $(
            pub fn $field_mut(&mut self) -> &mut crate::types::BoolU32 {
                &mut self.sys_mut().$sys_field
            }
        )?
        $(
            pub fn $field_set(&mut self, value: $field_ty) {
                self.sys_mut().$sys_field.set(value.into())
            }
        )?

        nvwrap! { @type field $name $field(($($field_mut)?)($($field_set)?))($field_ty) () () () () $($rest)* }
    };
    (@type field $name:ident $field:ident(($($field_mut:ident)?)($($field_set:ident)?))($field_ty:ty) () () () (@sys($sys_field:ident $($sys_field_:ident)*)) $($rest:tt)*) => {
        pub fn $field(&self) -> $field_ty {
            self.sys().$sys_field.into()
        }

        $(
            pub fn $field_mut(&mut self) -> &mut $field_ty {
                Into::into(&mut self.sys_mut().$sys_field)
            }
        )?
        $(
            pub fn $field_set(&mut self, value: $field_ty) {
                self.sys_mut().$sys_field = value.into()
            }
        )?

        nvwrap! { @type field $name $field(($($field_mut)?)($($field_set)?))($field_ty) () () () () $($rest)* }
    };
    (@type field $name:ident $field:ident(($($field_mut:ident)?)($($field_set:ident)?))($field_ty:ty) () () () (@sys@ref($sys_field:ident $($sys_field_:ident)*)) $($rest:tt)*) => {
        pub fn $field(&self) -> &$field_ty {
            Into::into(&self.sys().$sys_field)
        }

        $(
            pub fn $field_mut(&mut self) -> &mut $field_ty {
                Into::into(&mut self.sys_mut().$sys_field)
            }
        )?
        $(
            pub fn $field_set(&mut self, value: $field_ty) {
                self.sys_mut().$sys_field = value.into()
            }
        )?

        nvwrap! { @type field $name $field(($($field_mut)?)($($field_set)?))($field_ty) () () () () $($rest)* }
    };
    (@type field $name:ident $field:ident($field_mut:tt ($field_set:ident))($field_ty:ty) () (@set(($($set_args:tt)*) $($set_args_:tt)*)($set_body:block)) $($rest:tt)*) => {
        pub fn $field_set($($set_args)*)
            $set_body
        nvwrap! { @type field $name $field($field_mut ($field_set))($field_ty) () () $($rest)* }
    };
    (@type field $name:ident $field:ident($($field_:tt)*)($field_ty:ty) (@get($($get_args:tt)*)($get_body:block)) $($rest:tt)*) => {
        pub fn $field($($get_args)*) -> $field_ty
            $get_body

        nvwrap! { @type field $name $field($($field_)*)($field_ty) () $($rest)* }
    };
    (@type field_iter $name:ident $field:ident($($field_:tt)*)($item:ty) ()) => { };
    (@type field_iter $name:ident $field:ident($($field_:tt)*)($item:ty) (@into($into_name:ident)($($into_args:tt)*)($into_body:block)) $($rest:tt)*) => {
        pub fn $into_name($($into_args)*) -> impl Iterator<Item = $item>
            $into_body

        nvwrap! { @type field_iter $name $field($($field_)*)($item) () $($rest)* }
    };
    (@type fields $name:ident {}) => { };
    (@type fields $name:ident {
        pub $field:ident$(@mut($field_mut:ident))?$(@set($field_set:ident))?
            : @iter($item:ty)
        {
            $(@into fn $into_name:ident($($into_args:tt)*) $into_body:block,)?
        },
        $($rest:tt)*
    }) => {
        impl $name {
            nvwrap! { @type field_iter $name $field(($($field_mut)?)($($field_set)?))($item)
                ($(@into($into_name)($($into_args)*)($into_body))?)
            }
        }
        nvwrap! { @type fields $name {$($rest)*} }
    };
    (@type fields $name:ident {
        pub $field:ident$(@mut($field_mut:ident))?$(@set($field_set:ident))?
            : $field_ty:ty
        {
            $(@get fn($($get_args:tt)*) $get_body:block,)?
            $(@set fn$(($($set_args:tt)*))?$($set_self:ident $set_value_id:ident)? $set_body:block,)?
            $(@mut$(($mut_set:ident))? fn $mut_name:ident($($mut_args:tt)*) $mut_body:block,)?
            $(@sys$(@$sys_opt:ident)*$(($sys_field:ident))?,)?
        },
        $($rest:tt)*
    }) => {
        impl $name {
            nvwrap! { @type field $name $field(($($field_mut)?)($($field_set)?))($field_ty)
                ($(@get($($get_args)*)($get_body))?)
                ($(@set($(($($set_args)*))? $((&mut $set_self, $set_value_id: $field_ty))?)($set_body))?)
                ($(@mut($($mut_set)?)($mut_name)($($mut_args)*)($mut_body))?)
                ($(@sys$(@$sys_opt)*($($sys_field)? $field))?)
            }
        }
        nvwrap! { @type fields $name {$($rest)*} }
    };
    (
        $(#[$($meta:meta)*])*
        pub type $name:ident = NvData<$sys:ty> $({$($fields:tt)*})?;

        $(impl @Deref($deref_field:ident: $deref_target:ty) for $name_impl_deref:ident { })?

        $(
            impl $name_impl:ident { $(
                $(#[$($impl_meta:meta)*])*
                pub fn $impl_fn:ident($($args:tt)*) -> $impl_ret:ty;
            )* }
        )?
    ) => {
        $(#[$($meta)*])*
        pub type $name = crate::types::NvData<$sys>;

        impl Into<$sys> for NvData<$sys> {
            fn into(self) -> $sys {
                self.into_sys()
            }
        }

        $(
            nvwrap! { @type fields $name { $($fields)* } }
        )?

        nvwrap! { impl Deref($($deref_field: $deref_target)?) for $($name_impl_deref)? }
    };
    /*(pub struct $name:ident($sys:ty);) => {
        #[derive(Clone, Debug)]
        pub struct $name {
            sys: $sys,
        }

        impl $name {
            pub const fn with_sys(sys: $sys) -> Self {
                Self { sys }
            }

            pub const fn sys(&self) -> &$sys {
                &self.sys
            }
        }

        impl crate::sys::TaggedData for $name where $sys: crate::sys::TaggedData {
            type Repr = <$sys as crate::sys::TaggedData>::Repr;
            type Id = <$sys as crate::sys::TaggedData>::Id;

            fn tag(&self) -> Self::Repr {
                <$sys as crate::sys::TaggedData>::tag(self.sys())
            }
        }

        impl From<$sys> for $name {
            fn from(sys: $sys) -> Self {
                Self::with_sys(sys)
            }
        }

        impl Into<$sys> for $name {
            fn into(self) -> $sys {
                self.sys
            }
        }
    };*/
}
