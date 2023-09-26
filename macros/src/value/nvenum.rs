use {
    crate::prelude::*,
    syn::{
        braced, punctuated::Punctuated, token::Brace, ExprPath, Fields, MacroDelimiter, MetaList, Variant, Visibility,
    },
};

pub struct NvEnumBody {
    pub repr: Ident,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub value_ident: Ident,
    pub brace_token: Brace,
    pub variants: Punctuated<NvEnumValue, Token![,]>,
}

impl Parse for NvEnumBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;

        let repr = attrs_repr(&attrs)?.unwrap_or(call_ident(Self::DEFAULT_REPR));
        if let Some((i, _)) = attrs.iter().enumerate().find(|(_, a)| a.path().is_ident("repr")) {
            attrs.remove(i);
        }

        let vis = input.parse()?;
        let enum_token = input.parse()?;
        let value_ident = input.parse()?;
        let _: Token![/] = input.parse()?;
        let ident = input.parse()?;
        let content;

        let res = Self {
            repr,
            attrs,
            vis,
            enum_token,
            ident,
            value_ident,
            brace_token: braced!(content in input),
            variants: content.parse_terminated(NvEnumValue::parse, Token![,])?,
        };
        let _: ParseEof = input.parse()?;
        Ok(res)
    }
}

impl NvEnumBody {
    pub const DEFAULT_REPR: &'static str = "i32";

    pub fn output(&self) -> TokenStream {
        let NvEnumBody {
            attrs,
            repr,
            vis,
            enum_token,
            ident,
            value_ident,
            ..
        } = self;

        let variants = self.variants.iter().map(|v| v.clone().into_variant());
        let consts = self.variants.iter().map(|v| v.output_const(value_ident));
        let variant_idents = self.variants.iter().map(|v| &v.ident);
        let variant_symbols = self.variants.iter().map(|v| &v.symbol);

        let transmute = call_path_absolute(["core", "mem", "transmute"]);
        let serde = call_path_absolute(["serde"]);
        let Iterator = call_path_absolute(["core", "iter", "Iterator"]);
        let Into = call_path_absolute(["core", "convert", "Into"]);
        let TryFrom = call_path_absolute(["core", "convert", "TryFrom"]);
        let Result = call_path_absolute(["core", "result", "Result"]);
        let ArgumentRangeError = sys_path(["ArgumentRangeError"]);
        quote! {
            #(#attrs)*
            #vis type #value_ident = #repr;

            #(#consts)*

            #(#attrs)*
            #[cfg_attr(feature = "serde", derive(#serde::Serialize, #serde::Deserialize))]
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            #[non_exhaustive]
            #[repr(#repr)]
            #vis #enum_token #ident {
                #(#variants,)*
            }

            impl #ident {
                pub fn from_raw(raw: #value_ident) -> #Result<Self, #ArgumentRangeError> {
                    match raw {
                        #(
                            #variant_symbols
                        )|* => Ok(unsafe { #transmute(raw) }),
                        _ => Err(#ArgumentRangeError),
                    }
                }

                pub fn raw(&self) -> #value_ident {
                    *self as _
                }

                pub fn values() -> impl #Iterator<Item=Self> {
                    [
                        #(
                            #ident::#variant_idents
                        ),*
                    ].into_iter()
                }
            }

            impl #Into<#value_ident> for #ident {
                fn into(self) -> #value_ident {
                    self as _
                }
            }

            impl #TryFrom<#value_ident> for #ident {
                type Error = #ArgumentRangeError;

                fn try_from(raw: #value_ident) -> #Result<Self, #ArgumentRangeError> {
                    Self::from_raw(raw)
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct NvEnumValue {
    pub attrs: Vec<Attribute>,
    pub symbol: Ident,
    pub ident: Ident,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl NvEnumValue {
    pub fn into_variant(self) -> Variant {
        let mut attrs = self.attrs;

        let has_doc_alias = false;
        if !has_doc_alias {
            let symbol = self.symbol.to_string();
            attrs.push(call_attr(MetaList {
                path: call_ident("doc").into(),
                delimiter: MacroDelimiter::Paren(Default::default()),
                tokens: quote!(alias = #symbol),
            }));
        }
        let has_symbol = true;
        let value = match has_symbol {
            true => ExprPath {
                attrs: Default::default(),
                qself: None,
                path: self.symbol.into(),
            }
            .into(),
            false => self.value,
        };
        Variant {
            attrs,
            ident: self.ident,
            fields: Fields::Unit,
            discriminant: Some((self.eq_token, value)),
        }
    }

    pub fn output_const(&self, enum_ident: &Ident) -> TokenStream {
        let NvEnumValue {
            symbol,
            ident,
            value,
            eq_token,
            ..
        } = self;

        let doc = format!("[`{enum_ident}::{ident}`]");
        quote! {
            #[doc = #doc]
            #[allow(overflowing_literals)]
            pub const #symbol: #enum_ident #eq_token #value;
        }
    }
}

impl Parse for NvEnumValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let symbol = input.parse()?;
        let _: Token![/] = input.parse()?;
        let ident = input.parse()?;
        let eq_token = input.parse()?;
        let value = input.parse()?;

        Ok(Self {
            attrs,
            symbol,
            ident,
            eq_token,
            value,
        })
    }
}

pub fn nvenum(input: TokenStream) -> Result<TokenStream> {
    let body: NvEnumBody = parse(input)?;
    Ok(body.output())
}
