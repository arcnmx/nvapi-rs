#![allow(non_snake_case)]

use {crate::util::result_stream, proc_macro::TokenStream};

pub(crate) mod api;
pub(crate) mod inherit;
pub(crate) mod nvstruct;
pub(crate) mod util;
pub(crate) mod value;
pub(crate) mod version;

pub(crate) mod prelude {
    pub(crate) use {
        crate::util::{
            attr_derives,
            attrs::{get_attr, get_field_attr, try_parse_attr, ContextualAttr},
            attrs_repr, attrs_require_repr, call_attr, call_error, call_ident, call_path_absolute,
            derive::{DeriveEnum, DeriveStruct, ParseEof},
            error, nvapi_path, path_tail_is, result_stream2, sys_crate, sys_path,
        },
        proc_macro2::{Span, TokenStream},
        quote::{quote, quote_spanned, ToTokens},
        std::{ops::AddAssign, result::Result as StdResult},
        syn::{
            parse::{Parse, ParseStream, Parser},
            parse2 as parse, parse_quote,
            spanned::Spanned,
            Attribute, Data, DeriveInput, Error, Expr, Field, Ident, Path, PathSegment, Result, Token, Type,
        },
    };
}

#[proc_macro_derive(VersionedStructField, attributes(nv_version_field))]
pub fn derive_versioned_struct(input: TokenStream) -> TokenStream {
    result_stream(self::version::derive_versioned_struct(input.into()))
}

#[proc_macro]
pub fn nvversion(input: TokenStream) -> TokenStream {
    result_stream(self::version::nvversion(input.into()))
}

#[proc_macro_derive(NvInherit, attributes(nv_inherit))]
pub fn derive_inherit(input: TokenStream) -> TokenStream {
    result_stream(self::inherit::derive_inherit(input.into()))
}

#[proc_macro_attribute]
pub fn NvStruct(attr: TokenStream, input: TokenStream) -> TokenStream {
    result_stream(self::nvstruct::NvStruct(attr.into(), input.into()))
}

#[proc_macro]
pub fn nvenum(input: TokenStream) -> TokenStream {
    result_stream(self::value::nvenum(input.into()))
}

#[proc_macro]
pub fn nvenum_display(input: TokenStream) -> TokenStream {
    result_stream(self::value::nvenum_display(input.into()))
}

#[proc_macro_derive(NvValueEnum)]
pub fn derive_value_enum(input: TokenStream) -> TokenStream {
    result_stream(self::value::derive_value_enum(input.into()))
}

#[proc_macro]
pub fn nvbits(input: TokenStream) -> TokenStream {
    result_stream(self::value::nvbits(input.into()))
}

#[proc_macro_derive(NvValueBits)]
pub fn derive_value_bits(input: TokenStream) -> TokenStream {
    result_stream(self::value::derive_value_bits(input.into()))
}

#[proc_macro_derive(NvValueData, attributes(nv_value_symbol))]
pub fn derive_value_data(input: TokenStream) -> TokenStream {
    result_stream(self::value::derive_value_data(input.into()))
}

#[proc_macro]
pub fn nvapi(input: TokenStream) -> TokenStream {
    result_stream(self::api::nvapi(input.into()))
}
