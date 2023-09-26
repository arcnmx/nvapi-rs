use {
    crate::prelude::*,
    std::ops::{Deref, DerefMut},
    syn::{DataEnum, DataStruct},
};

#[derive(Clone)]
pub struct DeriveStruct {
    pub input: DeriveInput,
}

#[derive(Clone)]
pub struct DeriveEnum {
    pub input: DeriveInput,
}

#[allow(dead_code)]
impl DeriveStruct {
    pub fn data(&self) -> &DataStruct {
        match &self.input.data {
            Data::Struct(s) => s,
            _ => unreachable!("expected struct"),
        }
    }

    pub fn data_mut(&mut self) -> &mut DataStruct {
        match &mut self.input.data {
            Data::Struct(s) => s,
            _ => unreachable!("expected struct"),
        }
    }
}

#[allow(dead_code)]
impl DeriveEnum {
    pub fn data(&self) -> &DataEnum {
        match &self.input.data {
            Data::Enum(s) => s,
            _ => unreachable!("expected enum"),
        }
    }

    pub fn data_mut(&mut self) -> &mut DataEnum {
        match &mut self.input.data {
            Data::Enum(s) => s,
            _ => unreachable!("expected enum"),
        }
    }
}

impl Parse for DeriveStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        input
            .parse()
            .and_then(|i| match i {
                DeriveInput {
                    data: Data::Struct(..), ..
                } => Ok(i),
                _ => Err(call_error("struct required")),
            })
            .map(|input| Self { input })
    }
}

impl Parse for DeriveEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        input
            .parse()
            .and_then(|i| match i {
                DeriveInput {
                    data: Data::Enum(..), ..
                } => Ok(i),
                _ => Err(call_error("enum required")),
            })
            .map(|input| Self { input })
    }
}

impl ToTokens for DeriveStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.input.to_tokens(tokens)
    }
}

impl ToTokens for DeriveEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.input.to_tokens(tokens)
    }
}

impl Deref for DeriveStruct {
    type Target = DeriveInput;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl DerefMut for DeriveStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}

impl Deref for DeriveEnum {
    type Target = DeriveInput;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}

impl DerefMut for DeriveEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.input
    }
}

pub struct ParseEof;

impl Parse for ParseEof {
    fn parse(input: ParseStream) -> Result<Self> {
        match input.is_empty() {
            true => Ok(Self),
            false => Err(input.error("unexpected trailing arguments")),
        }
    }
}
