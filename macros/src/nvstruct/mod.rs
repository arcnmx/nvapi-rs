use {
    crate::{inherit::NvInheritArgs, prelude::*, version::NvVersionArgs},
    syn::{punctuated::Punctuated, TypeArray},
};

mod align;

pub use self::align::NvAlignArgs;

pub fn NvStruct(attr: TokenStream, input: TokenStream) -> Result<TokenStream> {
    assert!(attr.is_empty());
    let item = input.clone();
    let mut item: DeriveStruct = parse(item)?;

    let repr = match item.attrs.iter().any(|a| a.path().is_ident("repr")) {
        true => quote!(),
        false => quote! {
            #[repr(C)]
        },
    };

    let has_derive = |derive: &'static str| {
        |a: &Attribute| {
            attr_derives(a)
                .map(|derives| derives.iter().any(|path| path_tail_is(derive, &path.segments)))
                .unwrap_or(false)
        }
    };

    let has_version = item.data().fields.iter().any(|f| match &f.ty {
        Type::Path(ty) if path_tail_is("NvVersion", &ty.path.segments) => true,
        _ => false,
    });
    let derives = match has_version {
        true if !item.attrs.iter().any(has_derive("VersionedStructField")) => Some("VersionedStructField"),
        _ => None,
    };

    let derives: Punctuated<Path, Token![,]> = {
        // add missing derives if their related field attributes are found
        const ATTR_DERIVES: [(&'static str, &'static str); 2] = [
            (NvVersionArgs::NAME, "VersionedStructField"),
            (NvInheritArgs::NAME, "NvInherit"),
        ];

        ATTR_DERIVES
            .iter()
            .filter(|(attr_ident, derive)| {
                let has_attr = item
                    .data()
                    .fields
                    .iter()
                    .any(|f| f.attrs.iter().any(|a| a.path().is_ident(attr_ident)));
                has_attr && !item.attrs.iter().any(has_derive(derive))
            })
            .map(|&(_, derive)| derive)
            .chain(derives)
            .map(|derive| Path::from(call_ident(derive)))
            .collect()
    };

    let name = &item.ident;
    let AsBytes = call_path_absolute(["zerocopy", "AsBytes"]);
    let FromBytes = call_path_absolute(["zerocopy", "FromBytes"]);

    let struct_attrs = quote! {
        #[derive(Copy, Clone, Debug, #AsBytes, #FromBytes, #derives)]
        #repr
    };

    let expanded = quote! {
        impl #name {
            pub fn zeroed() -> Self {
                #FromBytes::new_zeroed()
            }
        }
    };

    let padding_fields = item
        .data_mut()
        .fields
        .iter_mut()
        .enumerate()
        .flat_map(|(field_index, field)| {
            field.attrs.iter().enumerate().filter_map(move |(attr_index, attr)| {
                try_parse_attr::<NvAlignArgs>(attr)
                    .transpose()
                    .map(|a| a.map(|attr| (field_index, attr_index, attr)))
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let requires_rewrite = !padding_fields.is_empty();

    let input = match requires_rewrite {
        false => input,
        true => {
            for (field_index, attr_index, align) in padding_fields {
                let field = item.data_mut().fields.iter_mut().nth(field_index).unwrap();
                field.attrs.remove(attr_index);
                match &mut field.ty {
                    Type::Array(TypeArray { len, elem, .. }) => {
                        // TODO: just assert that elem is u8
                        let bit_align = align.bit_align;
                        let next_ty = align.ty.as_ref().unwrap();
                        let mem = quote! { ::core::mem };
                        *len = parse_quote! {
                            #mem::align_of::<#next_ty>().saturating_sub(#bit_align / #mem::size_of::<#elem>() / 8)
                        };
                    },
                    _ => return Err(Error::new_spanned(field, "alignment padding must be a byte array")),
                }
            }

            item.into_token_stream()
        },
    };

    Ok(struct_attrs.into_iter().chain(input).chain(expanded).collect())
}
