use {
    crate::prelude::*,
    syn::{
        braced,
        punctuated::Punctuated,
        token::{Brace, Struct},
        Visibility,
    },
};

pub struct NvBitsBody {
    pub repr: Ident,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub enum_token: Token![enum],
    pub ident: Ident,
    pub value_ident: Ident,
    pub brace_token: Brace,
    pub variants: Punctuated<NvBitsValue, Token![,]>,
}

impl Parse for NvBitsBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let enum_token = input.parse()?;
        let value_ident = input.parse()?;
        let _: Token![/] = input.parse()?;
        let ident = input.parse()?;
        let content;

        let repr = attrs_repr(&attrs)?.unwrap_or(call_ident(Self::DEFAULT_REPR));
        if let Some((i, _)) = attrs.iter().enumerate().find(|(_, a)| a.path().is_ident("repr")) {
            attrs.remove(i);
        }

        let res = Self {
            repr,
            attrs,
            vis,
            enum_token,
            ident,
            value_ident,
            brace_token: braced!(content in input),
            variants: content.parse_terminated(NvBitsValue::parse, Token![,])?,
        };
        let _: ParseEof = input.parse()?;
        Ok(res)
    }
}

impl NvBitsBody {
    pub const DEFAULT_REPR: &'static str = "u32";

    pub fn output(&self) -> TokenStream {
        let NvBitsBody {
            repr,
            attrs,
            vis,
            enum_token,
            ident,
            value_ident,
            ..
        } = self;
        let struct_token = Struct {
            span: enum_token.span(),
        };

        let flags = self.variants.iter().map(|v| v.clone().output_flag());
        let consts = self.variants.iter().map(|v| v.output_const(repr));

        let variant_names_0 = self.variants.iter().map(|v| &v.ident);

        let ArgumentRangeError = sys_path(["ArgumentRangeError"]);
        let Default = call_path_absolute(["core", "default", "Default"]);
        let Result = call_path_absolute(["core", "result", "Result"]);
        let serde = call_path_absolute(["serde"]);
        quote! {
            #(#attrs)*
            #vis type #value_ident = #repr;

            #(#consts)*

            bitflags::bitflags! {
                #(#attrs)*
                #[cfg_attr(feature = "serde", derive(#serde::Serialize, #serde::Deserialize))]
                #[derive(#Default)]
                #vis #struct_token #ident: #value_ident {
                    #(#flags)*
                }
            }

            impl TryFrom<#value_ident> for #ident {
                type Error = #ArgumentRangeError;

                fn try_from(value: #value_ident) -> #Result<Self, #ArgumentRangeError> {
                    #ident::from_bits(value).ok_or(#ArgumentRangeError)
                }
            }

            impl Iterator for #ident {
                type Item = Self;

                fn next(&mut self) -> Option<Self::Item> {
                    #(
                        if self.contains(#ident::#variant_names_0) {
                            self.remove(#ident::#variant_names_0);
                            Some(#ident::#variant_names_0)
                        } else
                     )*
                    { None }
                }
            }

            impl From<#ident> for #value_ident {
                fn from(v: #ident) -> #value_ident {
                    v.bits()
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct NvBitsValue {
    pub attrs: Vec<Attribute>,
    pub symbol: Ident,
    pub ident: Ident,
    pub eq_token: Token![=],
    pub value: Expr,
}

impl NvBitsValue {
    pub fn output_flag(&self) -> TokenStream {
        let NvBitsValue {
            attrs,
            ident,
            value,
            eq_token,
            ..
        } = self;

        quote! {
            #(#attrs)*
            const #ident #eq_token #value;
        }
    }

    pub fn output_const(&self, enum_repr: &Ident) -> TokenStream {
        let NvBitsValue {
            attrs,
            symbol,
            value,
            eq_token,
            ..
        } = self;

        quote! {
            #(#attrs)*
            #[allow(overflowing_literals)]
            pub const #symbol: #enum_repr #eq_token #value;
        }
    }
}

impl Parse for NvBitsValue {
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

pub fn nvbits(input: TokenStream) -> Result<TokenStream> {
    let body: NvBitsBody = parse(input)?;
    Ok(body.output())
}
