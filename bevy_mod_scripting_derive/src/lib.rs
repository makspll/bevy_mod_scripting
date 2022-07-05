#![allow(dead_code,unused_variables,unused_features)]

pub(crate) mod lua;
pub(crate) mod common;


use bevy::prelude::default;
use indexmap::IndexMap;
use proc_macro::{TokenStream, token_stream::IntoIter, TokenTree, Delimiter, Group};
use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt, format_ident};
use syn::{parse::{ParseStream, Parse, ParseBuffer},Result, punctuated::Punctuated, Token, Ident, Error, parse_macro_input, Field, Visibility, ItemFn, braced, Type, token::{Brace, Token, self, Dot, Paren, Bracket, Bang}, ExprClosure, PatPath, LitStr, Path, ExprMethodCall, Expr, ExprPath, PathSegment, PathArguments, parse_quote, bracketed, parenthesized, parse_quote_spanned, UsePath, UseTree, ItemUse};

pub(crate) use {lua::*,common::*};



#[derive(Default)]
struct EmptyToken;

impl Parse for EmptyToken {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self)
    }
}
impl ToTokens for EmptyToken {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        
    }
}

struct NewtypeList {
    paren : Paren,
    module_headers : TokenStream2,
    sq_bracket1: Bracket,
    additional_types: Punctuated<Type, Token![,]>,
    sq_bracket2: Bracket,
    new_types: Punctuated<Newtype,Token![,]>,
}


impl Parse for NewtypeList {
    fn parse(input: ParseStream) -> Result<Self> {
        let h;
        let f;
        let g;
        Ok(Self {
            paren : parenthesized!(h in input),
            module_headers : h.parse()?,
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
        tokens.extend(quote!{
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
    functions: Punctuated<ItemFn,Token![;]>
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
pub fn impl_lua_newtypes(input: TokenStream) -> TokenStream {

    let new_types = parse_macro_input!(input as NewtypeList);

    let mut lua = LuaImplementor::default();

    match lua.generate_all(&new_types){
        Ok(v) => v.into(),
        Err(e) => e.into_compile_error().into(),
    }
}





pub(crate) struct ReplaceArgs {
    replacements: Punctuated<MethodMacroArg,Token![,]>,
}

impl Parse for ReplaceArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let replacements = Punctuated::<MethodMacroArg,Token![,]>::parse_separated_nonempty(input)?;

        Ok(Self{
            replacements,
        })
    }
}

impl Into<IndexMap<String,MethodMacroArg>> for &ReplaceArgs {
    fn into(self) -> IndexMap<String,MethodMacroArg> {
        self.replacements.iter().map(|i| (i.ident.to_string(),i.clone())).collect()
    }
}



#[proc_macro]
pub fn replace(input: TokenStream) -> TokenStream {
    // Get first parameters


    let mut found_colon = false;

    // skip past till end of replacements
    let (args,rest) = input.into_iter().partition(|tt| {
        match tt {
            TokenTree::Punct(p) => {
                if p.as_char() == ':' {
                    found_colon = true;
                }
            }
            _ => {}
        };

        !found_colon
    });

    
    let args = parse_macro_input!(args as ReplaceArgs);
    let replacements: IndexMap<String, MethodMacroArg> = (&args).into();
    // Return the remaining tokens, but replace identifiers.
    let mut remaining_iter = rest.into_iter();
    // skip colon
    remaining_iter.next();

    remaining_iter.map(|tt| {
        replace_helper(tt,&replacements)
    }).collect()
}


fn replace_helper(tt : TokenTree, replacements: &IndexMap<String, MethodMacroArg>) -> TokenTree{
    if let TokenTree::Group(g) = tt{
        let key = g.stream().to_string();

        if key.starts_with('$'){
            let key = &key[1..];

            if let Some(v) = replacements.get(key){
                    let r = &v.replacement;
                    return proc_macro::TokenTree::Ident(
                        proc_macro::Ident::new(&r.to_token_stream().to_string(),g.span()))
            }
        }
        

        let inner = g.stream().into_iter();

        let inner = inner.map(|t| {
            replace_helper(t,replacements)
        }).collect();

        proc_macro::TokenTree::Group(Group::new(g.delimiter(),inner))
    } else {
        tt
    }
}
