mod lua_method;


use std::{collections::HashSet};

use bevy::prelude::default;
use proc_macro::{TokenStream};
use proc_macro2::{Span};
use quote::{quote, ToTokens, TokenStreamExt};
use rlua::MetaMethod;
use syn::{parse::{ParseStream, Parse, ParseBuffer},Result, punctuated::Punctuated, Token, Ident, Error, parse_macro_input, Field, Visibility, ItemFn, braced, Type, token::{Brace, Token, self, Dot, Paren}, ExprClosure, PatPath, LitStr, Path, ExprMethodCall, Expr, ExprPath, PathSegment, PathArguments, parse_quote};
use lua_method::*;

macro_rules! enum_variants_str {
    (
        #[$($meta:meta)*]
        enum $name:ident {
            $(
                $field:ident
            ),*
            $(,)?
        }
    ) => { 
        #[$($meta)*]
        enum $name {
            $(
                $field
            ),*
            
        }

        impl $name {
            fn variants() -> &'static str{
                concat!($(
                    stringify!($field),", "
                ),*
                )
                
            }
        }
    };
}




enum_variants_str!(
#[derive(PartialEq,Eq,Hash)]
enum DeriveFlag {
    Standard,
    Vector,
    Matrix,
}
);

impl TryFrom<Ident> for DeriveFlag {
    type Error = Error;

    fn try_from(value: Ident) -> Result<Self> {
        match value.to_string().as_str() {
            "Vector" => Ok(Self::Vector),
            "Matrix" => Ok(Self::Matrix),
            _ => Err(Error::new_spanned(value, format!("Invalid derive flag, try one of `{}`",DeriveFlag::variants()))),
        }
    }
}

struct NewtypeArgs {
    base_type: Ident,
    colon: Token![:],
    flags: HashSet<DeriveFlag>
}


impl Parse for NewtypeArgs {
    fn parse(input: ParseStream) -> Result<Self>{
        Ok(Self {
            base_type: input.parse()?,
            colon: input.parse()?,
            flags: Punctuated::<Ident, Token![+]>::parse_separated_nonempty(input)?
                .into_iter()
                .map(|i| i.try_into())
                .collect::<Result<HashSet<DeriveFlag>>>()?,
        })
    }
}




struct BaseNewtype {
    args: NewtypeArgs,
    additional_functions: Option<AdditionalImplBlock>,
    additional_lua_functions: Option<LuaBlock>
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

impl Parse for BaseNewtype {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            args: input.parse()?,
            additional_lua_functions: if input.peek(Token![impl]) && !input.peek2(Token![fn]){
                    Some(input.parse()?)
                } else {
                    None
                },
            additional_functions:  if input.peek(Token![impl]) && input.peek2(Token![fn]) {
                    Some(input.parse()?)
                } else {
                    None
                },

        })
    }
}


#[proc_macro]
pub fn impl_lua_newtype(input: TokenStream) -> TokenStream {

    let base = parse_macro_input!(input as BaseNewtype);

    let args = base.args;

    let name : Ident = Ident::new(&format!("Lua{}",args.base_type.to_string()), Span::call_site());

    let functions = match base.additional_functions {
        Some(v) => {
            let fns = v.functions;
            quote!{#fns}
        },
        None => quote!{},
    };

    let additional_lua_functions = match base.additional_lua_functions {
        Some(v) => {
            let fns : Punctuated<ExprMethodCall,Token![;]> = v.functions
                .into_iter()
                .map(|m| m.to_call_expr("methods"))
                .collect();

            quote!{
                #fns;
            }
        },
        None => quote!{},
    };

    TokenStream::from(quote!{
        pub struct #name {

        }

        impl #name {
            #functions
        }

        impl rlua::UserData for #name {
            fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
                #additional_lua_functions
            }
        }

    })
}




