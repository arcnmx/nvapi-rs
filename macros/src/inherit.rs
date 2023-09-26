use crate::prelude::*;

pub struct NvInheritArgs {
    span: Span,
}

impl ContextualAttr for NvInheritArgs {
    const NAME: &'static str = "nv_inherit";
    const HAS_ARGS: bool = false;

    fn span(&self) -> Span {
        self.span
    }

    fn default_with_span(span: Span) -> Result<Self> {
        Ok(Self { span })
    }
}

impl Parse for NvInheritArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let _: ParseEof = input.parse()?;
        Ok(Self { span })
    }
}

impl AddAssign for NvInheritArgs {
    fn add_assign(&mut self, rhs: Self) {
        self.span = self.span.join(rhs.span).unwrap_or(self.span);
    }
}

pub fn derive_inherit(input: TokenStream) -> Result<TokenStream> {
    let input: DeriveStruct = parse(input)?;

    let inherit_fields: Result<Vec<_>> = input
        .data()
        .fields
        .iter()
        .enumerate()
        .flat_map(|(field_index, field)| {
            get_field_attr::<NvInheritArgs>(field)
                .transpose()
                .map(move |attr| attr.map(move |attr| (attr, field, field_index)))
        })
        .collect();

    let (field, field_id) = match inherit_fields {
        Err(e) => Err(e),
        Ok(fields) if fields.len() > 1 => {
            let last = fields.iter().last();
            Err(error(last.map(|(attr, ..)| attr.span()), "multiple fields specified"))
        },
        Ok(fields) => {
            let field = match fields.into_iter().next() {
                Some((_a, f, i)) => Some((f, i)),
                None if input.data().fields.len() == 1 => input.data().fields.iter().next().map(|field| (field, 0)),
                None => None,
            };
            match field.map(|(f, i)| (f, f.ident.as_ref().ok_or(i))) {
                Some((f, Ok(id))) => Ok((f, id.to_token_stream())),
                Some((f, Err(i))) => Ok((f, i.into_token_stream())),
                None => Err(call_error(format_args!("#[{}] field missing", NvInheritArgs::NAME))),
            }
        },
    }?;

    let name = &input.ident;
    let field_ty = &field.ty;

    let expanded = quote! {
        impl ::core::ops::Deref for #name {
            type Target = #field_ty;

            fn deref(&self) -> &Self::Target {
                &self.#field_id
            }
        }

        impl ::core::ops::DerefMut for #name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.#field_id
            }
        }
    };

    Ok(expanded)
}
