use crate::prelude::*;

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
    let VersionedStructField = sys_path(["nvapi", "VersionedStruct"]);
    let NvVersion = sys_path(["nvapi", "NvVersion"]);

    let (body, body_mut) = match version_field_id {
        Ok(version_field_id) => (
            quote! {
                #VersionedStructField::nvapi_version(&self.#version_field_id)
            },
            quote! {
                #VersionedStructField::nvapi_version_mut(&mut self.#version_field_id)
            },
        ),
        Err(err) => (err.to_compile_error(), err.into_compile_error()),
    };

    Ok(quote! {
        impl #VersionedStructField for #name {
            fn nvapi_version(&self) -> #NvVersion {
                #body
            }

            fn nvapi_version_mut(&mut self) -> &mut #NvVersion {
                #body_mut
            }
        }
    })
}
