use {
    crate::prelude::*,
    proc_macro::TokenStream,
    proc_macro2::{TokenStream as TokenStream2, TokenTree},
    std::{
        borrow::Borrow,
        env::var_os,
        ffi::{OsStr, OsString},
        fmt::Display,
        sync::OnceLock,
    },
    syn::{punctuated::Punctuated, token::PathSep, AttrStyle, MacroDelimiter, Meta, MetaList},
};

pub mod attrs;
pub mod derive;

pub fn call_attr<M: Into<Meta>>(meta: M) -> Attribute {
    Attribute {
        pound_token: Default::default(),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: meta.into(),
    }
}

pub fn call_path<I: AsRef<str>, P: IntoIterator<Item = I>>(idents: P) -> Path {
    Path {
        leading_colon: None,
        segments: idents
            .into_iter()
            .map(call_ident)
            .map(|ident| PathSegment {
                ident,
                arguments: Default::default(),
            })
            .collect(),
    }
}

pub fn call_path_absolute<I: AsRef<str>, P: IntoIterator<Item = I>>(idents: P) -> Path {
    let mut path = call_path(idents);
    path.leading_colon = Some(Default::default());
    path
}

pub fn crate_name() -> Option<&'static OsStr> {
    static CARGO_PKG_NAME: OnceLock<Option<OsString>> = OnceLock::new();
    let cargo_pkg_name = CARGO_PKG_NAME.get_or_init(|| var_os("CARGO_PKG_NAME"));
    cargo_pkg_name.as_ref().map(|s| &s[..])
}

pub fn sys_crate() -> &'static str {
    if crate_name() == Some(OsStr::new("nvapi-sys")) {
        "crate"
    } else {
        "nvapi_sys"
    }
}

pub fn sys_path<I: AsRef<str>, P: IntoIterator<Item = I>>(idents: P) -> Path {
    let mut path = call_path([sys_crate()]);
    path
        .segments
        .extend(idents.into_iter().map(call_ident).map(|ident| PathSegment {
            ident,
            arguments: Default::default(),
        }));
    path
}

pub fn nvapi_path<I: AsRef<str>, P: IntoIterator<Item = I>>(idents: P) -> Path {
    let mut path = if crate_name() == Some(OsStr::new("nvapi")) {
        call_path(["crate"])
    } else {
        call_path(["nvapi"])
    };
    path
        .segments
        .extend(idents.into_iter().map(call_ident).map(|ident| PathSegment {
            ident,
            arguments: Default::default(),
        }));
    path
}

pub fn call_ident<I: AsRef<str>>(ident: I) -> Ident {
    Ident::new(ident.as_ref(), Span::call_site())
}

pub fn call_error<M: Display>(message: M) -> Error {
    error(None, message)
}

pub fn error<M: Display>(span: Option<Span>, message: M) -> Error {
    Error::new(span.unwrap_or(Span::call_site()), message)
}

pub fn result_stream<T: Into<TokenStream>>(res: Result<T>) -> TokenStream {
    match res {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}

pub fn result_stream2<T: Into<TokenStream2>>(res: Result<T>) -> TokenStream2 {
    match res {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error(),
    }
}

pub fn attr_derives(attr: &Attribute) -> Option<Punctuated<Path, Token![,]>> {
    match &attr.meta {
        Meta::List(MetaList {
            delimiter: MacroDelimiter::Paren(..),
            path,
            tokens,
        }) if path.is_ident("derive") =>
            Some(Parser::parse2(Punctuated::parse_terminated, tokens.clone()).expect("expected derive paths")),
        _ => None,
    }
}

pub fn attrs_repr<A: Borrow<Attribute>, I: IntoIterator<Item = A>>(attrs: I) -> Result<Option<Ident>> {
    let repr = match attrs.into_iter().find(|a| a.borrow().path().is_ident("repr")) {
        Some(attr) => attr,
        None => return Ok(None),
    };
    let repr = repr.borrow();
    let repr = repr.meta.require_list()?;
    match repr.tokens.clone().into_iter().next() {
        Some(TokenTree::Ident(ident)) => Ok(Some(ident)),
        _ => return Err(Error::new_spanned(repr, "invalid repr")),
    }
}

pub fn attrs_require_repr<A: Borrow<Attribute>, I: IntoIterator<Item = A>>(attrs: I) -> Result<Ident> {
    attrs_repr(attrs)
        .transpose()
        .ok_or_else(|| call_error("#[repr] required"))
        .and_then(|r| r)
}

pub fn path_tail_is<I: ?Sized>(ident: &I, segments: &Punctuated<PathSegment, PathSep>) -> bool
where
    Ident: PartialEq<I>,
{
    segments.last().map(|seg| &seg.ident == ident).unwrap_or(false)
}
