use crate::prelude::*;

pub mod nvbits;
pub mod nvenum;

pub use {
    nvbits::{derive_value_bits, nvbits},
    nvenum::{derive_value_enum, nvenum, nvenum_display},
};

pub fn derive_value_data(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveEnum = parse(input)?;

    let repr = attrs_require_repr(&input.attrs)?;
    let symbol = get_attr::<NvValueSymbol, _, _>(&input.attrs)
        .and_then(|a| a.ok_or_else(|| call_error(format_args!("#[{}] required", NvValueSymbol::NAME))))?;
    let symbol = symbol.ident().to_string();

    let ident = &input.ident;
    let ident_str = ident.to_string();

    let Result = call_path_absolute(["core", "result", "Result"]);
    let Default = call_path_absolute(["core", "default", "Default"]);
    let Into = call_path_absolute(["core", "convert", "Into"]);
    let TryFrom = call_path_absolute(["core", "convert", "TryFrom"]);
    let transmute = call_path_absolute(["core", "mem", "transmute"]);
    let error = sys_path(["ArgumentRangeError"]);
    let NvValueData = sys_path(["value", "NvValueData"]);
    let NvValue = sys_path(["value", "NvValue"]);

    let value_names = input.data().variants.iter().map(|variant| &variant.ident);
    let symbols = input
        .data()
        .variants
        .iter()
        .map(|variant| {
            get_attr::<NvValueSymbol, _, _>(&variant.attrs)
                .and_then(|a| a.ok_or_else(|| call_error(format_args!("#[{}] required", NvValueSymbol::NAME))))
        })
        .collect::<Result<Vec<_>>>()?;
    let symbols = symbols.iter().map(|symbol| symbol.ident());

    let impl_into_repr = quote! {
        impl #Into<#repr> for #ident {
            fn into(self) -> #repr {
                #NvValueData::repr(self)
            }
        }
    };

    let impl_tryfrom_repr = quote! {
        impl #TryFrom<#repr> for #ident {
            type Error = #error;

            fn try_from(raw: #repr) -> #Result<Self, #error> {
                #NvValueData::from_repr(raw)
            }
        }
    };

    let impl_tryfrom_value = quote! {
        impl #TryFrom<#NvValue<#ident>> for #ident {
            type Error = #error;

            fn try_from(value: #NvValue<#ident>) -> #Result<Self, #error> {
                Self::try_from(value.value)
            }
        }
    };

    Ok(quote! {
        impl #NvValueData for #ident {
            const NAME: &'static str = #ident_str;
            const C_NAME: &'static str = #symbol;
            type Repr = #repr;

            fn from_repr(raw: Self::Repr) -> #Result<Self, #error> {
                match #NvValue::with_repr(raw) {
                    #(
                        #symbols
                    )|* => Ok(unsafe { #transmute(raw) }),
                    _ => Err(#Default::default()),
                }
            }

            fn from_repr_ref(raw: &Self::Repr) -> #Result<&Self, #error> {
                Self::from_repr(*raw).map(|_| unsafe {
                    #transmute(raw)
                })
            }

            fn from_repr_mut(raw: &mut Self::Repr) -> #Result<&mut Self, #error> {
                Self::from_repr(*raw).map(|_| unsafe {
                    #transmute(raw)
                })
            }

            fn all_values() -> &'static [Self] {
                &[
                    #(#ident::#value_names),*
                ]
            }

            fn repr(self) -> Self::Repr {
                self as #repr
            }

            fn repr_ref(&self) -> &Self::Repr {
                unsafe {
                    #transmute(self)
                }
            }
        }

        #impl_into_repr
        #impl_tryfrom_repr
        #impl_tryfrom_value
    })
}

pub struct NvValueSymbol {
    span: Span,
    pub ident: Option<Ident>,
}

impl NvValueSymbol {
    pub fn ident(&self) -> &Ident {
        self.ident.as_ref().unwrap()
    }
}

impl ContextualAttr for NvValueSymbol {
    const NAME: &'static str = "nv_value_symbol";
    const HAS_ARGS: bool = true;

    fn span(&self) -> Span {
        self.span
    }

    fn default_with_span(span: Span) -> Result<Self> {
        Ok(Self { span, ident: None })
    }
}

impl Parse for NvValueSymbol {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let ident = input.parse()?;
        let _: ParseEof = input.parse()?;
        Ok(Self {
            span,
            ident: Some(ident),
        })
    }
}

impl AddAssign for NvValueSymbol {
    fn add_assign(&mut self, rhs: Self) {
        self.span = self.span.join(rhs.span).unwrap_or(self.span);
        self.ident = rhs.ident.or(self.ident.take());
    }
}
