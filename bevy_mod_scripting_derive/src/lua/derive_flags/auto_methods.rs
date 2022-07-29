use std::{iter::{once}};

use proc_macro2::Span;
use quote::{format_ident, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, spanned::Spanned, parse_quote_spanned, Attribute};

use crate::{lua::lua_method::LuaMethod, common::{derive_flag::DeriveFlag, newtype::Newtype, arg::SimpleType}, EmptyToken};



pub(crate) fn make_auto_methods<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) {
    let newtype_name = &new_type.args.wrapper_type;
    let wrapped_type = &new_type.args.base_type_ident;

    let (ident,paren,methods) = match flag {
        DeriveFlag::Methods { ident, paren, methods } => (ident,paren,methods),
        _ => panic!("Expected AutoMethods flag")
    };
    out.extend(methods.iter()
    .map(|m| {
        let ident = &m.ident;
        let ident_str = ident.to_string();

        let self_arg_ident = format_ident!("s");
        let has_self_arg;
        let static_;
        let fn_;
        let mut_;
        let star;

        let self_type = if let Some((self_,_)) = &m.self_ {
            has_self_arg = true;
            static_ = None;
            fn_ = None;            
            if self_.is_any_ref(){
                star = Some(Token![*](Span::call_site()));
                if self_.is_mut_ref(){
                    mut_ = Some(Token![mut](Span::call_site()));
                } else {
                    mut_ = None;
                }
            } else {
                star = None;
                mut_ = None;
            }
            Some(self_)
        } else {
            has_self_arg = false;
            static_ = Some(Token![static](Span::call_site()));
            fn_ = Some(Token![fn](Span::call_site()));
            mut_ = None;
            star = None;
            None
        };

        let mut arg_idents = Vec::default();
        let mut arg_types = Vec::default();

        let inner_args : Punctuated<proc_macro2::TokenStream,Token![,]> = m.args.iter()
            .enumerate()
            .map(|(idx,arg_type)| {
                let lit = LitInt::new(&idx.to_string(),m.span());
                let lit = format_ident!("a_{lit}",span=m.span());

                arg_idents.push(lit.clone());
                arg_types.push(arg_type.clone());

                let is_ref = arg_type.is_any_ref();

                if (arg_type.is_wrapped() || arg_type.is_self()) && !is_ref{
                    quote_spanned!{m.span()=>
                        #lit.inner()?
                    }
                } else {
                    quote_spanned!{m.span()=>
                        #lit
                    }
                }
        }).collect();

        let base_ident = &new_type.args.base_type_ident;

        let out_type = &m.out;

        // create function call first
        let mut inner_expr =  if let Some(self_) = self_type {        
            if self_.is_any_ref(){
                // the s will come from a val or val_mut call
                quote_spanned!(m.span()=>#self_arg_ident.#ident(#inner_args))
            } else  {
                quote_spanned!(m.span()=>#self_arg_ident.inner()?.#ident(#inner_args))
            }
        } else {
            quote_spanned!(m.span()=>#base_ident::#ident(#inner_args))
        };

        // then wrap it in constructor if necessary
        if let Some(out_type) = out_type{
            if out_type.is_wrapped() || out_type.is_self(){
                let resolved_out_type = out_type.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                let wrapper_out_type = format_ident!("Lua{}",resolved_out_type.base_ident());
                inner_expr = quote_spanned!{m.span()=>
                    #wrapper_out_type::new(#inner_expr)
                };
            } 
        }

        // wrap in ok 
        inner_expr = quote_spanned!(m.span()=>Ok(#inner_expr));


        // and then wrap in getters for every argument which is a reference including self if it exists
        let all_ref_iter = m.args
                .iter()
                .zip(arg_idents.iter())
                .map(|(a,b)| (a.is_any_ref().then_some(a),b))
                .chain(once((self_type,&self_arg_ident)))
                .filter_map(|(a,b)| Some((a?,b)));
        
        for (arg,arg_ident) in all_ref_iter
        {
            if arg.is_any_ref() {
                let method_call = arg.is_mut_ref()
                .then(|| format_ident!("val_mut",span=arg.span()))
                .unwrap_or_else(|| format_ident!("val",span=arg.span()));
                inner_expr = quote_spanned!{m.span()=>
                    #arg_ident.#method_call(|#arg_ident| #inner_expr)?
                }            
            }
         
        }

        let self_ident = static_.map(|_| quote::quote!{})
            .unwrap_or(quote_spanned!{m.span()=>#self_arg_ident,});
        let ds : Punctuated<Attribute,EmptyToken> = m.docstring.iter().cloned().collect();

        let args_without_refs = arg_types.iter().map(|type_| {
            let mut t = type_.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();
            
            if type_.is_wrapped() || type_.is_self() {
                t.mutate_base_ident(|i| *i = format_ident!("Lua{i}"));
            }

            if let SimpleType::Ref{ type_, .. } = t{
                *type_
            } else {
                t
            }
        });

        // now if our output is &self or &mut self, and the only other self reference is the receiver
        // we can just return the wrapper


        parse_quote_spanned!{m.span()=>            
            #ds
            #static_ #mut_ #fn_ #ident_str =>|_,#self_ident (#(#arg_idents),*):(#(#args_without_refs),*)| #inner_expr
        }
    }).collect::<Vec<_>>())
}
