pub(crate) mod math;

use std::collections::{HashSet, HashMap};

use proc_macro2::Span;
use syn::{*, parse::*, punctuated::*, token::*};

use crate::{lua_block::{LuaBlock, LuaMethodType, MethodMacroArg, LuaMethod, LuaClosure},TokenStream2, newtype::NewtypeArgs, utils::impl_parse_enum};
use paste::paste;


use quote::{ToTokens,quote, quote_spanned};

pub(crate) use math::*;


#[derive(PartialEq,Eq,Hash,Debug)]
pub(crate) struct MethodMacroInvokation{
    target: TypePath,
    arrow: Token![->],
    identifier: LuaMethodType,
    paren: Paren,
    args : Punctuated<MethodMacroArg,Token![,]>
}

impl Parse for MethodMacroInvokation {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            target: input.parse()?,
            arrow: input.parse()?,
            identifier: input.parse()?,
            paren: parenthesized!(f in input),
            args: f.parse_terminated(MethodMacroArg::parse)?,
        })
    }
}

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum DeriveFlag {
    
    DebugToString => {Ok(Self::DebugToString{ident})},
    DisplayToString => {Ok(Self::DisplayToString{ident})},
    Copy{
        paren : Paren,
        invocations: Punctuated<MethodMacroInvokation,Token![,]>
    } => {
        let f;
        Ok(Self::Copy{
            ident,
            paren: parenthesized!(f in input),
            invocations: f.parse_terminated(MethodMacroInvokation::parse)?,
        })
    },
    MathUnaryOp{
        paren : Paren,
        name : MathOpName,
        colon: Token![:],
        op: MathUnaryOpArg
    } => {
        let f;
        Ok(Self::MathUnaryOp{ 
            ident,
            paren: parenthesized!(f in input), 
            name: f.parse()?, 
            colon: f.parse()?,
            op : f.parse()?,
        })
    },
    MathBinOp {
        paren : Paren,
        name: MathOpName,
        colon: Token![:],
        ops: Punctuated<MathBinOpGroup,Token![,]>
    } => {
        let f;
        Ok(Self::MathBinOp{
            ident,
            paren: parenthesized!(f in input),
            name: f.parse()?,
            colon: f.parse()?, 
            ops: f.parse_terminated(MathBinOpGroup::parse)? 
        })
    }
}
);
    

    
impl DeriveFlag {
    pub fn into_impl_block(&self, newtype_args : &NewtypeArgs) -> Option<ItemImpl>{

        let wrapper_type = &newtype_args.short_wrapper_type;

        match self {
            _ => None
        }
    }

    pub fn into_lua_block(&self, newtype_args : &NewtypeArgs, all_methods: &HashMap<String,Vec<LuaMethod>>) -> Option<LuaBlock> {

        match self {
            DeriveFlag::DebugToString{ident} => Some(parse_quote_spanned!{ident.span()=>
                impl {
                    (rlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s));
                }
            }),
            DeriveFlag::DisplayToString{ident} => Some(parse_quote_spanned!{ident.span()=>
                impl {
                    (rlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s));
                }
            }),
            DeriveFlag::Copy { ident, paren, invocations } =>{ 
                let mut new_methods = Vec::default();
                for i in invocations{
                    let key = &i.target;
                    let key = quote!{#key}.to_string();

                    let methods = all_methods.get(&key).expect(&format!("Target lua wrapper type `{}` not found",key));

                    let mut found = false;
                    for m in methods {
                        if i.identifier == m.method_type {
                            found = true;
                            // hit apply replacements
                            let mut new_method = m.clone();

                            new_method.rebind_macro_args(i.args.iter()).unwrap();

                            new_methods.push(new_method);
                        }
                    }
                    if !found {
                        panic!("Could not find Method `{}` in target `{}`",i.identifier.to_token_stream(), i.target.to_token_stream());
                    }
                };
                let new_methods: Punctuated<LuaMethod,Token![;]> = new_methods.into_iter().collect();
                Some(parse_quote_spanned!{ident.span()=>
                    impl {
                        #new_methods
                    }
                })
            },
            DeriveFlag::MathBinOp {ident ,name , ops, .. } =>  {  
                let (lua_wrapper_group,rest)  = ops.into_iter()
                                                                        .partition::<Vec<_>,_>(|x| x.kind.is_lua_wrapper());
                let (numbers_group,integer_group) = rest.into_iter()
                                                .partition::<Vec<_>,_>(|x| x.kind.is_number());
                
                if lua_wrapper_group.len() > 1 || numbers_group.len() > 1 || integer_group.len() > 1 {
                    panic!("Expected at most one instance of LuaWrapper, Number and Integer argument groups")
                };

                let lua_wrapper_group = lua_wrapper_group.first();
                let numbers_group = numbers_group.first();
                let integer_group = integer_group.first();

                let newtype = &newtype_args.short_wrapper_type;

                let out_ident = Ident::new("out",Span::call_site());

                let body_ud_borrow = quote_spanned! {ident.span()=>
                    // figure out which side is the newtype

                    let (ud,op_on_rhs) : (&rlua::AnyUserData,bool) = match (&lhs,&rhs) {
                        (Value::UserData(v),_) => (v,true),
                        (_,Value::UserData(v)) => (v,false),
                        _ => panic!("Something went wrong"),   
                    };

                    // borrow the newtype
                    let ud: &#newtype = &ud.borrow::<#newtype>().unwrap();
                };

                let out_needs_wrapped_in_value = ops.iter().flat_map(|v| {
                    v.arg_sides.iter().map(|i| i.out_kind.as_ref())
                }).collect::<HashSet<_>>().len() > 1;


                // we check in order of: userdata -> integer -> number in each case
                let (userdata_handlers_lhs,userdata_handlers_rhs) = lua_wrapper_group
                    .map(|x| {
                        let lhs = x.gen_luawrapper_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,
                            &OpSide::Lhs { ident: Ident::new("Lhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        let rhs = x.gen_luawrapper_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,
                            &OpSide::Rhs { ident: Ident::new("Rhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        (lhs,rhs)
                    })
                    .unwrap_or((proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new()));
                
                let (integer_handlers_lhs,integer_handlers_rhs) = integer_group
                    .map(|x| {
                        let lhs = x.gen_integer_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,
                            &OpSide::Lhs { ident: Ident::new("Lhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        let rhs = x.gen_integer_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,
                            &OpSide::Rhs { ident: Ident::new("Rhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        (lhs,rhs)
                    })
                    .unwrap_or((proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new()));
                
                let (number_handlers_lhs,number_handlers_rhs) = numbers_group
                    .map(|x| {
                        let lhs = x.gen_number_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,&OpSide::Lhs { ident: Ident::new("Lhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        let rhs = x.gen_number_exprs(
                            out_needs_wrapped_in_value,
                            &newtype_args.short_wrapper_type,
                            &name,
                            &OpSide::Rhs { ident: Ident::new("Rhs",Span::call_site()) },
                            "ctx",
                            "ud",
                            "op");
                        (lhs,rhs)
                    })
                    .unwrap_or((proc_macro2::TokenStream::new(),proc_macro2::TokenStream::new()));


                    
                let op_handling = quote_spanned!{ident.span()=>
                    // userdata handling
                    if op_on_rhs {
                        let op = &rhs;
                        #userdata_handlers_rhs
                        #integer_handlers_rhs
                        #number_handlers_rhs
                    } else {
                        let op = &lhs;
                        #userdata_handlers_lhs
                        #integer_handlers_lhs
                        #number_handlers_lhs
                    };

                    Err(rlua::Error::RuntimeError("Attempted to perform invalid arithmetic with userdata".to_owned()))
                };


                Some(parse_quote_spanned! {ident.span()=>
                    impl {
                        fn (rlua::MetaMethod::#name) => |ctx,(lhs,rhs) :(Value,Value)| {
                            #body_ud_borrow
                            #op_handling
                        };
                    }
                })
                
            }
            DeriveFlag::MathUnaryOp { ident, name, op , ..} => {
                let body = op.gen_unary_call_expr(&newtype_args.short_wrapper_type, name, "ud");
                Some(parse_quote_spanned! {ident.span()=>
                    impl {
                        (rlua::MetaMethod::#name) => |_,ud,()|{
                            #body
                        }
                    }
                })
            },

        }
    }
}





