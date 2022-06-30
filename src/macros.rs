#[macro_export]
macro_rules! nvcall {
    ($nvapi:ident@get{$init:expr}($($arg:expr),*$(,)?) $($tt:tt)*) => {
        {
            let mut out = $init;
            nvcall!($nvapi($($arg,)* &mut out) => map(move |()| out) $($tt)*)
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
            nvcall!(@post ($crate::status_result($crate::sys::Api::$nvapi, $crate::sys::api::$nvapi($($arg,)*))) $($tt)*)
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
                .and_then(|v| v.convert_raw().map_err(Into::into))
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
}
