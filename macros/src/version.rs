use {
    crate::prelude::*,
    syn::{parenthesized, punctuated::Punctuated, token::Paren, LitInt},
};

pub struct NvVersionArgs {
    span: Span,
}

impl ContextualAttr for NvVersionArgs {
    const NAME: &'static str = "nv_version_field";
    const HAS_ARGS: bool = false;

    fn span(&self) -> Span {
        self.span
    }

    fn default_with_span(span: Span) -> Result<Self> {
        Ok(Self { span })
    }
}

impl Parse for NvVersionArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let _: ParseEof = input.parse()?;
        Ok(Self { span })
    }
}

impl AddAssign for NvVersionArgs {
    fn add_assign(&mut self, rhs: Self) {
        self.span = self.span.join(rhs.span).unwrap_or(self.span);
    }
}

pub fn derive_versioned_struct(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveStruct = parse(input)?;

    let version_fields: Result<Vec<_>> = input
        .data()
        .fields
        .iter()
        .enumerate()
        .flat_map(|(field_index, field)| {
            let attr = get_field_attr::<NvVersionArgs>(field)
                .transpose()
                .map(move |attr| attr.map(move |attr| (Some(attr), field_index, field)));
            let implicit = match &field.ty {
                Type::Path(ty)
                    if ty
                        .path
                        .segments
                        .last()
                        .map(|id| id.ident == "NvVersion")
                        .unwrap_or(false) =>
                    Some(Ok((None, field_index, field))),
                _ => None,
            };
            attr.into_iter().chain(implicit)
        })
        .collect();

    let version_field_id = match version_fields {
        Err(e) => Err(e),
        Ok(version_fields) if version_fields.iter().filter(|(attr, ..)| attr.is_some()).count() > 1 => {
            let last = version_fields.iter().filter_map(|(attr, ..)| attr.as_ref()).last();
            Err(error(last.map(|attr| attr.span()), "multiple fields specified"))
        },
        Ok(mut version_fields) => {
            version_fields.sort_by_key(|(attr, ..)| attr.is_none());
            match version_fields.first().map(|(_attr, i, f)| (i, f.ident.as_ref())) {
                Some((_, Some(id))) => Ok(id.to_token_stream()),
                Some((i, None)) => Ok(i.into_token_stream()),
                None => Err(call_error(format_args!("#[{}] missing", NvVersionArgs::NAME))),
            }
        },
    };

    let name = &input.ident;
    let VersionedStructField = sys_path(["version", "VersionedStructField"]);
    let NvVersion = sys_path(["version", "NvVersion"]);

    let (body, body_mut) = match version_field_id {
        Ok(version_field_id) => (
            quote! {
                #VersionedStructField::nvapi_version_ref(&self.#version_field_id)
            },
            quote! {
                #VersionedStructField::nvapi_version_mut(&mut self.#version_field_id)
            },
        ),
        Err(err) => (err.to_compile_error(), err.into_compile_error()),
    };

    Ok(quote! {
        impl #VersionedStructField for #name {
            fn nvapi_version_ref(&self) -> &#NvVersion {
                #body
            }

            fn nvapi_version_mut(&mut self) -> &mut #NvVersion {
                #body_mut
            }
        }
    })
}

pub struct NvVersionBody {
    pub ident: StdResult<Ident, Token![_]>,
    pub colon: Token![:],
    pub versions: Punctuated<NvVersionInstance, Token![,]>,
}

impl Parse for NvVersionBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = if input.peek(Token![_]) {
            Err(input.parse()?)
        } else {
            Ok(input.parse()?)
        };

        let colon = input.parse()?;

        let res = Self {
            ident,
            colon,
            versions: input.parse_terminated(NvVersionInstance::parse, Token![,])?,
        };
        if res.versions.is_empty() {
            return Err(input.error("at least one version is required"))
        }
        let _: ParseEof = input.parse()?;
        Ok(res)
    }
}

impl NvVersionBody {
    pub fn output(&self) -> TokenStream {
        let StructVersion = sys_path(["version", "StructVersion"]);
        let Default = call_path_absolute(["core", "default", "Default"]);

        let mut versions: Vec<_> = self.versions.iter().collect();
        versions.sort_by_key(|v| v.version());

        let mut expanded = TokenStream::new();
        for (i, instance) in versions.iter().enumerate().rev() {
            let NvVersionInstance {
                ident: instance_ident,
                version,
                ..
            } = instance;
            expanded.extend(instance.output_struct_version_impl());

            let mut prior = versions[0..i].iter().rev();
            if !prior.any(|prior| &prior.ident == instance_ident) {
                // this is the oldest version supported by this specific struct
                expanded.extend(quote! {
                    impl #Default for #instance_ident {
                        #[inline]
                        fn default() -> Self {
                            #StructVersion::<#version>::versioned()
                        }
                    }
                })
            }
        }

        if let Ok(ident) = &self.ident {
            let latest = versions.last().map(|v| &v.ident).expect("version");
            expanded.extend(quote! {
                pub type #ident = #latest;
            });
        }

        expanded
    }
}

#[derive(Clone)]
pub struct NvVersionInstance {
    pub ident: Ident,
    pub version: LitInt,
    pub paren: Paren,
    /// `@old` and `@inherit(field: Ty)`, shouldn't be necessary anymore
    pub trailing: Option<(Token![;], TokenStream)>,
    pub size: Option<(Token![=], LitInt)>,
}

impl NvVersionInstance {
    pub fn version(&self) -> u16 {
        self.version.base10_parse().expect("u16 version")
    }

    pub fn output_struct_version_impl(&self) -> TokenStream {
        let Self { ident, version, .. } = self;

        let StructVersion = sys_path(["version", "StructVersion"]);
        let NvVersion = sys_path(["version", "NvVersion"]);

        let expanded_size_assertion = match &self.size {
            Some((eq, size)) => {
                let message = format!("{} NvVersion size expected {}", ident, size);
                quote_spanned! { size.span() =>
                    const _: () #eq if <#ident as #StructVersion::<#version>>::NVAPI_VERSION.size() != #size & 0xffff {
                        panic!(#message)
                    } else {
                        ()
                    };
                }
            },
            None => Default::default(),
        };

        quote_spanned! { ident.span() =>
            impl #StructVersion<#version> for #ident {
                const NVAPI_VERSION: #NvVersion = #NvVersion::with_struct::<#ident>(#version);
            }

            #expanded_size_assertion
        }
    }
}

impl Parse for NvVersionInstance {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;

        let content;
        let paren = parenthesized!(content in input);
        let version: LitInt = content.parse()?;
        let _: u16 = version.base10_parse()?;
        let semi: Option<Token![;]> = content.parse()?;
        let trailing = match semi {
            Some(semi) => Some((semi, content.parse()?)),
            None => None,
        };
        let size = if input.peek(Token![=]) {
            let eq = input.parse()?;
            let size = input.parse()?;
            Some((eq, size))
        } else {
            None
        };
        Ok(Self {
            ident,
            version,
            paren,
            trailing,
            size,
        })
    }
}

pub fn nvversion(input: TokenStream) -> Result<TokenStream> {
    let body: NvVersionBody = parse(input)?;
    Ok(body.output())
}
