use bevy_mod_scripting_common::{
    implementor::WrapperFunction,
    utils::{attribute_to_string_lit, EmptyToken},
};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
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
