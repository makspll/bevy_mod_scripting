#![allow(dead_code, unused_variables, unused_features)]

pub(crate) mod common;
pub(crate) mod lua;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{
    braced, bracketed, parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{Brace, Bracket, Paren},
    ItemFn, Result, Token, Type,
};

pub(crate) use {common::*, lua::*};

#[derive(Default,Debug,Clone)]
struct EmptyToken;

impl Parse for EmptyToken {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self)
    }
}
impl ToTokens for EmptyToken {
    fn to_tokens(&self, tokens: &mut TokenStream2) {}
}

struct NewtypeList {
    paren: Paren,
    module_headers: TokenStream2,
    sq_bracket1: Bracket,
    additional_types: Punctuated<Type, Token![,]>,
    sq_bracket2: Bracket,
    new_types: Punctuated<Newtype, Token![,]>,
}

impl Parse for NewtypeList {
    fn parse(input: ParseStream) -> Result<Self> {
        let h;
        let f;
        let g;
        Ok(Self {
            paren: parenthesized!(h in input),
            module_headers: h.parse()?,
            sq_bracket1: bracketed!(f in input),
            additional_types: f.parse_terminated(Type::parse)?,
            sq_bracket2: bracketed!(g in input),
            new_types: g.parse_terminated(Newtype::parse)?,
        })
    }
}

impl ToTokens for NewtypeList {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let module_headers = &self.module_headers;
        let external_types = &self.additional_types;
        let types = &self.new_types;
        tokens.extend(quote! {
            (#module_headers)
            [#external_types]
            [#types]
        })
    }
}

struct AdditionalImplBlock {
    impl_token: Token![impl],
    fn_token: Token![fn],
    impl_braces: Brace,
    functions: Punctuated<ItemFn, Token![;]>,
}

impl Parse for AdditionalImplBlock {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self {
            impl_token: input.parse()?,
            fn_token: input.parse()?,
            impl_braces: braced!(f in input),
            functions: f.parse_terminated(ItemFn::parse)?,
        })
    }
}

#[proc_macro]
pub fn impl_lua_newtype(input: TokenStream) -> TokenStream {
    let new_type = parse_macro_input!(input as Newtype);

    let mut lua = LuaImplementor::default();

    match lua.generate(&new_type) {
        Ok(v) => v.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
