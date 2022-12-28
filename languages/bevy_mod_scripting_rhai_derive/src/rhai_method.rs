use bevy_mod_scripting_common::implementor::WrapperFunction;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    *,
};
impl WrapperFunction for RhaiMethod {}

pub(crate) struct RhaiMethod {
    pub method: ItemFn,
}

impl Parse for RhaiMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            method: input.parse()?,
        })
    }
}

impl ToTokens for RhaiMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let method = &self.method;
        tokens.extend(quote::quote! {
            #method
        })
    }
}
