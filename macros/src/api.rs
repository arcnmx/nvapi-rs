use {
    crate::prelude::*,
    syn::{
        braced, parenthesized,
        punctuated::Punctuated,
        token::{Brace, Paren},
        LitStr, Visibility,
    },
};

pub struct NvApiBody {
    pub fn_type: Option<(
        Visibility,
        Token![type],
        Ident,
        Token![=],
        Token![extern],
        LitStr,
        Token![fn],
        Token![;],
    )>,
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub unsafe_: Option<Token![unsafe]>,
    pub fn_: Token![fn],
    pub ident: Ident,
    pub paren: Paren,
    pub args: Punctuated<NvApiArg, Token![,]>,
    pub result: Option<(Token![->], Type)>,
    pub semi: Token![;],
    pub self_ident: Option<(
        Ident,
        Token![impl],
        Token![self],
        Brace,
        Visibility,
        Token![fn],
        Token![;],
    )>,
}

impl Parse for NvApiBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let fn_type = input.peek(Token![pub]) && input.peek2(Token![type]);
        let (fn_type, attrs, vis, unsafe_, fn_, ident, paren, args, result) = if fn_type {
            let vis = input.parse()?;
            let type_ = input.parse()?;
            let ident = input.parse()?;
            let eq = input.parse()?;
            let extern_ = input.parse()?;
            let c = input.parse()?;
            let fn_ = input.parse()?;

            let args;
            let paren = parenthesized!(args in input);
            let result = if input.peek(Token![->]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            };
            let semi = input.parse()?;
            let fn_type = (vis, type_, ident, eq, extern_, c, fn_, semi);

            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse()?;
            let unsafe_ = if input.peek(Token![unsafe]) {
                Some(input.parse()?)
            } else {
                None
            };
            let fn_ = input.parse()?;
            let ident = input.parse()?;
            (Some(fn_type), attrs, vis, unsafe_, fn_, ident, paren, args, result)
        } else {
            let attrs = input.call(Attribute::parse_outer)?;
            let vis = input.parse()?;
            let unsafe_ = if input.peek(Token![unsafe]) {
                Some(input.parse()?)
            } else {
                None
            };
            let fn_ = input.parse()?;
            let ident = input.parse()?;
            let args;
            let paren = parenthesized!(args in input);
            let result = if input.peek(Token![->]) {
                Some((input.parse()?, input.parse()?))
            } else {
                None
            };
            (None, attrs, vis, unsafe_, fn_, ident, paren, args, result)
        };
        let semi = input.parse()?;

        let self_ident = if input.peek(Token![impl]) && input.peek2(Token![self]) {
            let impl_ = input.parse()?;
            let self_ = input.parse()?;
            let content;
            let brace = braced!(content in input);
            let vis = content.parse()?;
            let fn_ = content.parse()?;
            let ident = content.parse()?;
            let semi = content.parse()?;
            let _: ParseEof = content.parse()?;
            Some((ident, impl_, self_, brace, vis, fn_, semi))
        } else {
            None
        };

        let args = args.parse_terminated(NvApiArg::parse, Token![,])?;

        let res = Self {
            fn_type,
            attrs,
            vis,
            unsafe_,
            fn_,
            ident,
            paren,
            args,
            result,
            semi,
            self_ident,
        };
        let _: ParseEof = input.parse()?;
        Ok(res)
    }
}

impl NvApiBody {
    pub fn result(&self) -> TokenStream {
        match &self.result {
            Some((arrow, res)) => quote! { #arrow #res },
            None => TokenStream::new(),
        }
    }

    pub fn output(&self) -> TokenStream {
        let expanded_ffi = self.output_ffi_fn();
        let expanded_ty = self.output_fn_type();
        quote! {
            #expanded_ffi

            #expanded_ty
        }
    }

    pub fn output_ffi_fn(&self) -> TokenStream {
        let Self { attrs, ident, .. } = self;

        let res = self.result();

        let arg_idents = self.args.iter().map(|arg| &arg.ident);
        let arg_idents_1 = arg_idents.clone();
        let arg_idents_2 = arg_idents.clone();
        let arg_idents_3 = arg_idents.clone();
        let arg_types = self.args.iter().map(|arg| &arg.ty);
        let arg_types_1 = arg_types.clone();
        let arg_types_2 = arg_types.clone();

        let query_interface = sys_path(["nvapi", "query_interface"]);
        let Api = sys_path(["nvid", "Api"]);
        let AtomicUsize = call_path_absolute(["core", "sync", "atomic", "AtomicUsize"]);
        let transmute = call_path_absolute(["core", "mem", "transmute"]);

        let res_ident = call_ident("nvapi_res");
        let log = self.output_log_result(&res_ident);

        quote! {
            #(#attrs)*
            pub unsafe fn #ident(#(#arg_idents_1: #arg_types_1),*) #res {
                static CACHE: #AtomicUsize = #AtomicUsize::new(0);

                let #res_ident = match #query_interface(#Api::#ident.id(), &CACHE) {
                    Ok(ptr) => #transmute::<_, extern "C" fn(#(#arg_idents_2: #arg_types_2),*) #res>(ptr)(#(#arg_idents_3),*),
                    Err(e) => e.raw(),
                };
                #log
                #res_ident
            }
        }
    }

    pub fn output_fn_type(&self) -> TokenStream {
        let (vis, type_, ident, eq, extern_, c, fn_, semi) = match &self.fn_type {
            Some(t) => t,
            None => return TokenStream::new(),
        };

        let res = self.result();

        let arg_idents = self.args.iter().map(|arg| &arg.ident);
        let arg_types = self.args.iter().map(|arg| &arg.ty);

        quote! {
            #vis #type_ #ident #eq #extern_ #c #fn_(#(#arg_idents: #arg_types),*) #res #semi
        }
    }

    pub fn output_log_result(&self, res: &Ident) -> TokenStream {
        let NvApiBody { ident, .. } = self;
        let ident_str = ident.to_string();

        let log = call_path_absolute(["log"]);
        let status_result = sys_path(["status_result"]);

        let arg_inputs = self.args.iter();
        let arg_inputs_key = arg_inputs.clone().map(|NvApiArg { ident, .. }| ident);

        let arg_inputs_value = arg_inputs
            .clone()
            .map(|NvApiArg { ident, .. }| quote! { #log::as_debug!(&#ident) });

        let arg_inputs_debug = arg_inputs.clone().map(|NvApiArg { ident, .. }| quote! { &#ident });

        let arg_inputs_key_1 = arg_inputs_key.clone();
        let arg_inputs_key_2 = arg_inputs_key.clone();
        let arg_inputs_value_1 = arg_inputs_value.clone();
        let arg_inputs_value_2 = arg_inputs_value.clone();
        let arg_inputs_debug_1 = arg_inputs_debug.clone();
        let arg_inputs_debug_2 = arg_inputs_debug.clone();
        let arg_inputs_debug_3 = arg_inputs_debug.clone();
        let arg_inputs_debug_4 = arg_inputs_debug.clone();

        let format_inputs = arg_inputs
            .clone()
            .fold(String::new(), |mut s, _arg| match s.is_empty() {
                true => "{:?}".into(),
                false => {
                    s.push_str(", {:?}");
                    s
                },
            });
        let format_call = format!("{}({})", ident, format_inputs);
        let format_succ = format!("{} = {{:?}}", format_call);
        let format_err = format!("{} = error: {{}}", format_call);

        quote! {
            #[cfg(feature = "log")]
            match #status_result(#res) {
                #[cfg(feature = "log-kv")]
                &Err(e) => #log::trace!(target: "nvapi_sys::api",
                    api = #log::as_display(#ident_str),
                    err = #log::as_error!(e)
                    #(, #arg_inputs_key_1 = #arg_inputs_value_1)*;
                    #format_err #(, #arg_inputs_debug_1)*, e
                ),
                #[cfg(feature = "log-kv")]
                Ok(res) => #log::trace!(target: "nvapi_sys::api",
                    api = #log::as_display(#ident_str),
                    out = #log::as_debug!(res)
                    #(, #arg_inputs_key_2 = #arg_inputs_value_2)*;
                    #format_succ #(, #arg_inputs_debug_2)*, res
                ),
                #[cfg(not(feature = "log-kv"))]
                &Err(e) => #log::trace!(target: "nvapi_sys::api",
                    #format_err #(, #arg_inputs_debug_3)*, e
                ),
                #[cfg(not(feature = "log-kv"))]
                Ok(res) => #log::trace!(target: "nvapi_sys::api",
                    #format_succ #(, #arg_inputs_debug_4)*, res
                ),
            }
        }
    }
}

pub struct NvApiArg {
    pub ident: Ident,
    pub colon: Token![:],
    pub ty: Type,
}

impl Parse for NvApiArg {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse()?;
        let colon = input.parse()?;
        let ty = input.parse()?;

        Ok(Self { ident, colon, ty })
    }
}

pub fn nvapi(input: TokenStream) -> Result<TokenStream> {
    let body: NvApiBody = parse(input)?;
    Ok(body.output())
}
