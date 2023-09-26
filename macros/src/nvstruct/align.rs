use {crate::prelude::*, syn::LitInt};

pub struct NvAlignArgs {
    span: Span,
    pub bit_align: usize,
    pub ty: Option<Type>,
}

impl ContextualAttr for NvAlignArgs {
    const NAME: &'static str = "nv_align";
    const HAS_ARGS: bool = true;

    fn span(&self) -> Span {
        self.span
    }

    fn default_with_span(span: Span) -> Result<Self> {
        Ok(Self {
            span,
            bit_align: 0,
            ty: None,
        })
    }
}

impl Parse for NvAlignArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let align: LitInt = input.parse()?;
        let _comma: Option<Token![,]> = input.parse()?;
        let ty: Type = input.parse()?;
        let _: ParseEof = input.parse()?;
        Ok(Self {
            span,
            bit_align: align.base10_parse()?,
            ty: Some(ty),
        })
    }
}

impl AddAssign for NvAlignArgs {
    fn add_assign(&mut self, rhs: Self) {
        self.span = self.span.join(rhs.span).unwrap_or(self.span);
        self.bit_align = rhs.bit_align;
        self.ty = rhs.ty.or(self.ty.take());
    }
}
