#![allow(dead_code,unused_variables,unused_features)]

pub(crate) mod lua;

use std::{collections::{HashSet, HashMap}};

use bevy::prelude::default;
use proc_macro::{TokenStream, token_stream::IntoIter, TokenTree, Delimiter, Group};
use proc_macro2::{Span,TokenStream as TokenStream2};
use quote::{quote, ToTokens, TokenStreamExt, format_ident};
use rlua::MetaMethod;
use syn::{parse::{ParseStream, Parse, ParseBuffer},Result, punctuated::Punctuated, Token, Ident, Error, parse_macro_input, Field, Visibility, ItemFn, braced, Type, token::{Brace, Token, self, Dot, Paren, Bracket, Bang}, ExprClosure, PatPath, LitStr, Path, ExprMethodCall, Expr, ExprPath, PathSegment, PathArguments, parse_quote, bracketed, parenthesized, parse_quote_spanned, UsePath, UseTree, ItemUse};

pub(crate) use lua::*;



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
        .filter(|base| !base.args.variation.is_non_assignable())
        .filter_map(|base| base.to_to_lua_entry()).collect();

    let from_lua : Punctuated<proc_macro2::TokenStream,Token![,]> = new_types.new_types
        .iter()
        .filter(|base| !base.args.variation.is_non_assignable())
        .filter_map(|base| base.to_from_lua_entry()).collect();


    let lookup_tables = quote!{

        pub static BEVY_TO_LUA: Map<&'static str,
            for<'l> fn(&LuaRef,&'l Lua) -> Value<'l>
            > = phf_map!{
                #to_lua,
            };

        pub static APPLY_LUA_TO_BEVY: Map<&'static str,
            for<'l> fn(&mut LuaRef,&'l Lua, Value<'l>) -> Result<(),Error>
            > = phf_map!{
                #from_lua,
            };
    };


    let mut userdata_newtype_global_names = Vec::default();
    let global_modules : TokenStream2 = new_types.new_types.iter()
        .filter(|v| (!v.args.variation.is_primitive()).into())
        .flat_map(|base|{
        if let Some(ref b) = base.additional_lua_functions {
            let static_methods = b.functions.iter()
                .filter(|f| f.method_type.is_static)
                .map(|f| f.to_call_expr("methods"))
                .collect::<Punctuated<TokenStream2,EmptyToken>>();

            let ident = format_ident!{"{}Globals",base.args.short_wrapper_type.path.get_ident().unwrap()};

            if !static_methods.is_empty(){
                userdata_newtype_global_names.push(ident.clone());

                let global_key = base.args.short_base_type.path.get_ident().unwrap().to_string();

                return quote!{
                    struct #ident;
                    impl tealr::mlu::TealData for #ident {
                        fn add_methods<'lua,T: tealr::mlu::TealDataMethods<'lua,Self>>(methods: &mut T) {
                            methods.document_type(concat!("Global methods for ", #global_key));
                            #static_methods
                        }
                    }

                    impl_tealr_type!(#ident);
                }
            } 
        };
        default()}
    ).collect();

    let userdata_newtype_names : Vec<&Ident> = new_types.new_types
        .iter()
        .filter(|v| (!v.args.variation.is_primitive()).into())
        .map(|v| v.args.short_wrapper_type.path.get_ident().unwrap())
        .collect();

        let api_provider = quote!{

        struct BevyAPIGlobals;
        impl tealr::mlu::ExportInstances for BevyAPIGlobals {
            fn add_instances<'lua, T: tealr::mlu::InstanceCollector<'lua>>(
                instance_collector: &mut T,
            ) -> mlua::Result<()> {
                #(
                    instance_collector.document_instance(concat!("Global methods for ", stringify!(#userdata_newtype_global_names)));
                    instance_collector.add_instance(stringify!(#userdata_newtype_global_names).into(), |_| Ok(#userdata_newtype_global_names))?;
                )*

                Ok(())
            }
        }

        #global_modules

        #[derive(Default)]
        pub struct LuaBevyAPIProvider;

        impl APIProvider for LuaBevyAPIProvider{
            type Target = Mutex<Lua>;
            type DocTarget = crate::LuaDocFragment;

            fn attach_api(&mut self, c: &mut <Self as APIProvider>::Target) -> Result<(),ScriptError> {
                let lua_ctx = c.lock().expect("Could not get lock on script context");

                tealr::mlu::set_global_env::<BevyAPIGlobals>(&lua_ctx)?;

                Ok(())
            }

            fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
                Some(crate::LuaDocFragment::new(|tw|
                            tw.document_global_instance::<BevyAPIGlobals>().unwrap()
                            #(
                                .process_type::<#userdata_newtype_names>()
                            )*
                            #(
                                .process_type::<#userdata_newtype_global_names>()  
                            )*
                        )
                    )
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
    let custom_tests : Punctuated<proc_macro2::TokenStream,EmptyToken> = methods_so_far.iter()
        .flat_map(|(n,v)| v.iter().filter_map(|v| v.gen_tests(n)))
        .collect();

    let tests = quote!{
        #[cfg(test)]
        mod gen_test {
            #imports

            #[test]
            pub fn test_wrapper_keys(){
                #asserts
            }

            #custom_tests
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
