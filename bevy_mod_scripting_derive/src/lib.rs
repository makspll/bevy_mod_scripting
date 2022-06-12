#![allow(dead_code,unused_variables,unused_features)]

pub(crate) mod lua_method;
pub(crate) mod impls;
pub(crate) mod newtype;
pub(crate) mod utils;

use std::{collections::{HashSet, HashMap}};

use bevy::prelude::default;
use proc_macro::{TokenStream, token_stream::IntoIter, TokenTree, Delimiter, Group};
use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt};
use rlua::MetaMethod;
use syn::{parse::{ParseStream, Parse, ParseBuffer},Result, punctuated::Punctuated, Token, Ident, Error, parse_macro_input, Field, Visibility, ItemFn, braced, Type, token::{Brace, Token, self, Dot, Paren, Bracket, Bang}, ExprClosure, PatPath, LitStr, Path, ExprMethodCall, Expr, ExprPath, PathSegment, PathArguments, parse_quote, bracketed, parenthesized, parse_quote_spanned, UsePath, UseTree, ItemUse};

pub(crate) use {impls::*,lua_method::*,newtype::*,utils::*};



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
    paren: Paren,
    imports: Punctuated<ItemUse,EmptyToken>,
    sq_brackets: Bracket,
    new_types: Punctuated<Newtype,Token![,]>
}


impl Parse for NewtypeList {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        let g;
        Ok(Self {
            paren: parenthesized!(f in input),
            imports: f.parse_terminated(ItemUse::parse)?,
            sq_brackets: bracketed!(g in input),
            new_types: g.parse_terminated(Newtype::parse)?,
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


    let to_lua : Punctuated<proc_macro2::TokenStream,Token![,]> = new_types.new_types
        .iter()
        .filter(|base| base.args.variation.is_full())
        .map(|base|{
        let k = &mut base.args.full_base_type.clone().into_token_stream().to_string();
        k.retain(|c| !c.is_whitespace());
        let wrapper_type = &base.args.short_wrapper_type;


        quote!{
            #k => |r,c| {
                let usr = c.create_userdata(#wrapper_type::base_to_self(r)).unwrap();
                Value::UserData(usr)
            }
        }
    }).collect();

    let from_lua : Punctuated<proc_macro2::TokenStream,Token![,]> = new_types.new_types
        .iter()
        .filter(|base| base.args.variation.is_full())
        .map(|base|{
        let k = &mut base.args.full_base_type.clone().into_token_stream().to_string();
        k.retain(|c| !c.is_whitespace());
        let wrapper_type = &base.args.short_wrapper_type;

        quote!{
            #k => |r,c,n| {
                if let Value::UserData(v) = n {
                    let mut v = v.borrow_mut::<#wrapper_type>()?;
                    #wrapper_type::apply_self_to_base(v.deref_mut(),r);
                    Ok(())
                } else {
                    Err(Error::RuntimeError("Invalid type".to_owned()))
                }
            }
        }
    }).collect();


    let lookup_tables = quote!{

        pub static BEVY_TO_LUA: Map<&'static str,
            for<'l> fn(&LuaRef,Context<'l>) -> Value<'l>
            > = phf_map!{
                #to_lua
            };

        pub static APPLY_LUA_TO_BEVY: Map<&'static str,
            for<'l> fn(&mut LuaRef, Context<'l>, Value<'l>) -> Result<(),Error>
            > = phf_map!{
                #from_lua
            };
    };

    let global_method_calls : Punctuated<TokenStream2, Token![;]> = new_types.new_types.iter().flat_map(|base|  
        if let Some(ref b) = base.additional_lua_functions {
            b.functions.iter()
                .filter_map(|f| f.to_create_global_expr("g", "lua_ctx"))
                .collect::<Vec<TokenStream2>>()
                .into_iter()
        } else {
            Vec::default().into_iter()
        }
    ).collect();


    let api_provider = quote!{

        #[derive(Default)]
        pub struct LuaBevyAPI;

        impl APIProvider for LuaBevyAPI{
            type Ctx = Mutex<Lua>;


            fn attach_api(c: &mut <Self as APIProvider>::Ctx) {
                c.lock()
                .expect("Could not get lock on script context")
                .context::<_, Result<(), ScriptError>>(|lua_ctx| {
                    let g = lua_ctx.globals();
                    #global_method_calls;
                    Ok(())
                }).unwrap();

            }
        }
    };

    let mut methods_so_far = HashMap::default();
    let impls : proc_macro2::TokenStream = new_types.new_types.iter().map(|base| {
        let applied_tokens = base.to_applied_tokens(&mut methods_so_far);
        quote!{#applied_tokens}
    }).collect();

    let asserts : proc_macro2::TokenStream = new_types.new_types.iter().map(|x| {
        let ident = &x.args.full_base_type.path.segments.last().unwrap().ident;
        let mut full_key = x.args.full_base_type.to_token_stream().to_string();
        full_key.retain(|c| !c.is_whitespace());

        quote!{
            assert_eq!(std::any::type_name::<#ident>(),#full_key);
        }
    }).collect();

    let imports = &new_types.imports;

    let tests = quote!{
        #[cfg(test)]
        mod test {
            #imports

            #[test]
            pub fn test_wrapper_keys(){
                #asserts
            }
        }
    };


    TokenStream::from(quote!{
        #api_provider
        #lookup_tables
        #impls
        #tests
    })
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

impl Into<HashMap<String,MethodMacroArg>> for &ReplaceArgs {
    fn into(self) -> HashMap<String,MethodMacroArg> {
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
    let replacements: HashMap<String, MethodMacroArg> = (&args).into();
    // Return the remaining tokens, but replace identifiers.
    let mut remaining_iter = rest.into_iter();
    // skip colon
    remaining_iter.next();

    remaining_iter.map(|tt| {
        replace_helper(tt,&replacements)
    }).collect()
}


fn replace_helper(tt : TokenTree, replacements: &HashMap<String, MethodMacroArg>) -> TokenTree{
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
