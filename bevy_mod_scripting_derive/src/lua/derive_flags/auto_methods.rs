use proc_macro2::Span;
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute};

use crate::{lua::lua_method::LuaMethod, common::{derive_flag::DeriveFlag, newtype::Newtype}, EmptyToken};



pub(crate) fn make_auto_methods<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) {
    let newtype_name = &new_type.args.wrapper_type;


    let (ident,paren,methods) = match flag {
        DeriveFlag::AutoMethods { ident, paren, methods } => (ident,paren,methods),
        _ => panic!("Expected AutoMethods flag")
    };
    
    out.extend(methods.iter()
    .map(|m| {
        let ident = &m.ident;
        let ident_str = ident.to_string();
        let mut arg_idents = Vec::default();
        let mut args_without_refs = Vec::default();
        let inner_args : Punctuated<proc_macro2::TokenStream,Token![,]> = m.args.iter()
            .enumerate()
            .map(|(idx,a)| {
                let lit = LitInt::new(&idx.to_string(),m.span());
                let lit = format_ident!("a_{lit}",span=m.span());
                arg_idents.push(lit.clone());
                let is_ref = if let Type::Reference(r) = a {
                    args_without_refs.push(r.elem.as_ref());
                    true
                } else {
                    args_without_refs.push(&a);
                    false
                };

                if a.to_token_stream().to_string().starts_with("Lua") && !is_ref{
                    quote_spanned!{m.span()=>
                        #lit.clone()
                    }
                } else {
                    quote_spanned!{m.span()=>
                        #lit
                    }
                }
        }).collect();

        let base_ident = new_type.args.base_type.path.segments.last().unwrap();

        let (mut inner_expr,
            static_,
            fn_,
            mut_,
            star) = 

            if let Some((r,v)) = &m.self_ {   
                
                let base = 
                    if r.reference.is_some() && r.mutability.is_some(){
                        quote_spanned!{m.span()=>s.val_mut(|s| s.#ident(#inner_args))}
                    } else if r.reference.is_some(){
                        quote_spanned!{m.span()=>s.val(|s| s.#ident(#inner_args))}
                    } else {
                        quote_spanned!{m.span()=>s.clone().#ident(#inner_args)}
                    };

                (base,
                    None,
                    None,
                    r.mutability,
                    r.reference.as_ref().map(|_| Token![*](Span::call_site())))
            } else {
                (quote_spanned!{m.span()=>#base_ident::#ident(#inner_args)},
                    Some(Token![static](Span::call_site())),
                    Some(Token![fn](Span::call_site())),
                    None,
                    None)
            };

        let out_ident = &m.out;
        inner_expr = out_ident.as_ref().map(|v| 
            if v.into_token_stream().to_string().starts_with("Lua"){
                quote_spanned!{m.span()=>
                    #out_ident::new(#inner_expr)
                }
            } else {
                inner_expr.clone()
            }
        ).unwrap_or(quote_spanned!{m.span()=>
            #newtype_name::new(#inner_expr)
        });

        // wrap reference variables with val and val mut calls
        for (idx,arg) in m.args.iter().enumerate(){
            if let Type::Reference(r) = arg {
                let method_call = r.mutability
                    .map(|v| format_ident!("val_mut",span=arg.span()))
                    .unwrap_or_else(|| format_ident!("val",span=arg.span()));
                let arg_ident = &arg_idents[idx];
                inner_expr = quote_spanned!{m.span()=>
                    #arg_ident.#method_call(|#arg_ident| #inner_expr)
                }
            }                            
        }


        let self_ident = static_.map(|_| quote::quote!{}).unwrap_or(quote_spanned!{m.span()=>s,});
        let ds : Punctuated<Attribute,EmptyToken> = m.docstring.iter().cloned().collect();


        parse_quote_spanned!{m.span()=>
            #ds
            #static_ #mut_ #fn_ #ident_str =>|_,#self_ident (#(#arg_idents),*):(#(#args_without_refs),*)| Ok(#inner_expr)
        }
    }).collect::<Vec<_>>())
}
