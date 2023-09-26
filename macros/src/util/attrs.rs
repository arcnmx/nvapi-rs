use {crate::prelude::*, std::borrow::Borrow, syn::Meta};

pub trait ContextualAttr: Sized + Parse {
    const NAME: &'static str;
    const HAS_ARGS: bool = true;

    fn span(&self) -> Span;

    fn with_attr(attr: &Attribute) -> Result<Self> {
        if !attr.path().is_ident(Self::NAME) {
            return Err(Error::new_spanned(attr, format_args!("expected #[{}]", Self::NAME)))
        }

        match Self::HAS_ARGS {
            true => attr.parse_args(),
            false => Self::default_with_span(attr.span()),
        }
    }

    fn default_with_span(span: Span) -> Result<Self>;
}

pub fn try_parse_attr<T: ContextualAttr>(attr: &Attribute) -> Result<Option<T>> {
    if !attr.path().is_ident(T::NAME) {
        return Ok(None)
    }

    if !T::HAS_ARGS {
        return match &attr.meta {
            Meta::Path(..) => T::with_attr(attr).map(Some),
            _ => Err(Error::new_spanned(attr, "attribute does not take arguments")),
        }
    }

    attr.parse_args().map(Some)
}

pub fn filter_attrs<T: ContextualAttr, A: Borrow<Attribute>, I: IntoIterator<Item = A>>(
    attrs: I,
) -> impl Iterator<Item = Result<T>> {
    attrs
        .into_iter()
        .filter_map(|attr| try_parse_attr::<T>(attr.borrow()).transpose())
}

pub fn get_attr<T: ContextualAttr + AddAssign, A: Borrow<Attribute>, I: IntoIterator<Item = A>>(
    attrs: I,
) -> Result<Option<T>> {
    let mut res = None;
    for attr in filter_attrs(attrs) {
        let attr = attr?;
        res = Some(match res {
            Some(mut res) => {
                res += attr;
                res
            },
            None => attr,
        });
    }
    Ok(res)
}

pub fn get_field_attr<T: ContextualAttr + AddAssign>(field: &Field) -> Result<Option<T>> {
    get_attr(field.attrs.iter())
}
