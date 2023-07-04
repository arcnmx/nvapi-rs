use {
    super::NvValueSymbol,
    crate::prelude::*,
    syn::{
        braced, punctuated::Punctuated, token::Brace, ExprCall, ExprField, ExprPath, Fields, MacroDelimiter, MetaList,
        Variant, Visibility,
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
        let consts = self.variants.iter().map(|v| v.output_const(ident));
        let value_consts = self.variants.iter().map(|v| v.output_value_const(ident));

        let serde = call_path_absolute(["serde"]);
        let NvEnum = sys_path(["value", "NvEnum"]);
        let NvEnum = quote!(#NvEnum::<#ident>);
        let NvValueEnum = sys_path(["value", "NvValueEnum"]);
        let NvValueData = sys_path(["value", "NvValueData"]);
        let nv_value_symbol = call_ident(NvValueSymbol::NAME);
        quote! {
            #(#attrs)*
            #vis type #value_ident = #NvEnum;

            #(#consts)*

            #(#attrs)*
            #[cfg_attr(feature = "serde", derive(#serde::Serialize, #serde::Deserialize))]
            #[derive(Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
            #[derive(#NvValueEnum, #NvValueData)]
            #[non_exhaustive]
            #[repr(#repr)]
            #[#nv_value_symbol(#value_ident)]
            #vis #enum_token #ident {
                #(#variants,)*
            }

            impl #ident {
                pub const fn value(self) -> #NvEnum {
                    #NvEnum::with_repr(self.repr())
                }

                pub const fn repr(self) -> #repr {
                    self as #repr
                }
            }

            #[allow(non_upper_case_globals)]
            impl #NvEnum {
                #(#value_consts)*
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

        let nv_value_symbol = call_ident(NvValueSymbol::NAME);
        if !attrs.iter().any(|a| a.path().is_ident(&nv_value_symbol)) {
            let nv_value_symbol = call_attr(MetaList {
                path: nv_value_symbol.into(),
                delimiter: MacroDelimiter::Paren(Default::default()),
                tokens: self.symbol.to_token_stream(),
            });
            attrs.push(nv_value_symbol);
        }
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
            true => {
                let symbol_expr = ExprPath {
                    attrs: Default::default(),
                    qself: None,
                    path: self.symbol.into(),
                };
                let method = ExprField {
                    attrs: Default::default(),
                    base: Box::new(symbol_expr.into()),
                    dot_token: Default::default(),
                    member: call_ident("repr").into(),
                };
                ExprCall {
                    attrs: Default::default(),
                    func: Box::new(method.into()),
                    paren_token: Default::default(),
                    args: Punctuated::new(),
                }
                .into()
            },
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

        let NvEnum = sys_path(["value", "NvEnum"]);
        let sys = sys_crate();
        let doc = format!("[NvValue]({sys}::value::NvValue) wrapper of [`{enum_ident}::{ident}`]");
        quote! {
            #[doc = #doc]
            #[allow(overflowing_literals)]
            pub const #symbol: #NvEnum<#enum_ident> #eq_token #NvEnum::<#enum_ident>::with_repr(#value);
        }
    }

    pub fn output_value_const(&self, enum_ident: &Ident) -> TokenStream {
        let NvEnumValue {
            ident,
            symbol,
            eq_token,
            ..
        } = self;

        let NvEnum = sys_path(["value", "NvEnum"]);
        quote! {
            pub const #ident: #NvEnum<#enum_ident> #eq_token #symbol;
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

pub struct NvEnumDisplayBody {
    pub ident: Ident,
    pub arrow: Token![=>],
    pub brace_token: Option<Brace>,
    pub variants: Punctuated<NvEnumDisplayValue, Token![,]>,
}

impl Parse for NvEnumDisplayBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let arrow = input.parse()?;

        let res = if input.peek(Token![_]) {
            let _: Token![_] = input.parse()?;
            Self {
                ident,
                arrow,
                brace_token: None,
                variants: Punctuated::new(),
            }
        } else {
            let content;
            Self {
                ident,
                arrow,
                brace_token: Some(braced!(content in input)),
                variants: content.parse_terminated(NvEnumDisplayValue::parse, Token![,])?,
            }
        };
        let _: ParseEof = input.parse()?;
        Ok(res)
    }
}

impl NvEnumDisplayBody {
    pub fn output(&self) -> TokenStream {
        let NvEnumDisplayBody { ident, .. } = self;

        let fmt = quote!(::core::fmt);
        let f = call_ident("f");

        let body = if self.variants.is_empty() {
            quote! {
                #fmt::Debug::fmt(self, #f)
            }
        } else {
            let branches = self.variants.iter().map(|v| v.output_display_branch(ident, &f));
            quote! {
                match *self {
                    #(#branches,)*
                }
            }
        };

        quote! {
            impl #fmt::Display for #ident {
                fn fmt(&self, #f: &mut #fmt::Formatter) -> #fmt::Result {
                    #body
                }
            }
        }
    }
}

#[derive(Clone)]
pub enum NvEnumDisplayValue {
    Wildcard {
        underscore: Token![_],
        eq_token: Token![=],
        value: Option<Expr>,
    },
    Value {
        ident: Ident,
        eq_token: Token![=],
        value: Expr,
    },
}

impl NvEnumDisplayValue {
    pub fn output_display_branch(&self, enum_ident: &Ident, f: &Ident) -> TokenStream {
        let fmt = quote!(::core::fmt);
        match self {
            Self::Wildcard { value: None, .. } => quote! {
                ref v => #fmt::Debug::fmt(v, #f)
            },
            Self::Wildcard { value: Some(..), .. } => todo!(),
            Self::Value { ident, value, .. } => quote! {
                #enum_ident::#ident => write!(#f, "{}", #value)
            },
        }
    }
}

impl Parse for NvEnumDisplayValue {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(if input.peek(Token![_]) {
            let underscore = input.parse()?;
            let eq_token = input.parse()?;
            let value = if input.peek(Token![_]) {
                let _: Token![_] = input.parse()?;
                None
            } else {
                Some(input.parse()?)
            };
            Self::Wildcard {
                underscore,
                eq_token,
                value,
            }
        } else {
            let ident = input.parse()?;
            let eq_token = input.parse()?;
            let value = input.parse()?;

            Self::Value { ident, eq_token, value }
        })
    }
}

pub fn nvenum(input: TokenStream) -> Result<TokenStream> {
    let body: NvEnumBody = parse(input)?;
    Ok(body.output())
}

pub fn nvenum_display(input: TokenStream) -> Result<TokenStream> {
    let body: NvEnumDisplayBody = parse(input)?;
    Ok(body.output())
}

pub fn derive_value_enum(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveEnum = parse(input)?;

    let ident = &input.ident;
    let NvValueEnum = sys_path(["value", "NvValueEnum"]);

    Ok(quote! {
        impl #NvValueEnum for #ident {}
    })
}
