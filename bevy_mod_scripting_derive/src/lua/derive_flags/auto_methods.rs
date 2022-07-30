use std::iter::once;

use proc_macro2::Span;
use quote::{format_ident, quote_spanned};
use syn::{
    parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, LitInt, Token,
};

use crate::{
    common::{arg::SimpleType, derive_flag::DeriveFlag, newtype::Newtype},
    lua::lua_method::LuaMethod,
    EmptyToken,
};

pub(crate) fn make_methods<'a>(
    flag: &DeriveFlag,
    new_type: &'a Newtype,
    out: &mut Vec<LuaMethod>,
) {
    let wrapper_type = &new_type.args.wrapper_type;
    let wrapped_type = &new_type.args.base_type_ident;

    let (ident, paren, methods) = match flag {
        DeriveFlag::Methods {
            ident,
            paren,
            methods,
        } => (ident, paren, methods),
        _ => panic!("Expected Methods flag"),
    };
    
    out.extend(methods.iter()
    .map(|m| {

        // first go through each parameter and remember identifiers + types of each
        let mut parameter_identifiers = Vec::default();
        let mut parameter_types = Vec::default();

        let parameters : Punctuated<proc_macro2::TokenStream,Token![,]> = m.args.iter()
            .enumerate()
            .map(|(idx,arg_type)| {
                let lit = LitInt::new(&idx.to_string(),m.span());
                let lit = format_ident!("a_{lit}",span=m.span());

                // store the identifier and type
                parameter_identifiers.push(lit.clone());

                // the paramter type must be stripped of outermost references
                // and also a prefix for wrapper types must be addded
                let mut resolved_parameter_type = arg_type.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();
                if arg_type.is_wrapped() || arg_type.is_self() {
                    resolved_parameter_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{ident}"));
                }
                parameter_types.push(resolved_parameter_type.strip_outer_refs());
                
                // finally produce an expression to be used as parameter to the method/function call
                if (arg_type.is_wrapped() || arg_type.is_self()) && !arg_type.is_any_ref(){
                    quote_spanned!{m.span()=>
                        #lit.inner()?
                    }
                } else {
                    quote_spanned!{m.span()=>
                        #lit
                    }
                }
        }).collect();

        // we build final closure body in steps, first
        // create the function call, either static or from the receiver.
        let method_identifier = &m.ident;
        let base_ident = &new_type.args.base_type_ident;

        let receiver_argument_identifier = format_ident!("s");
        let static_;
        let fn_;
        let mut_;
        let mut body;
        if let Some((self_,_)) = &m.self_ {
         
            if self_.is_any_ref(){
                body = quote_spanned!(m.span()=>#receiver_argument_identifier.#method_identifier(#parameters));
                if self_.is_mut_ref(){
                    mut_ = Some(Token![mut](Span::call_site()));
                } else {
                    mut_ = None;
                }
            } else{
                body = quote_spanned!(m.span()=>#receiver_argument_identifier.inner()?.#method_identifier(#parameters));
                mut_ = None;
            }

            static_ = None;
            fn_ = None;  
        } else {
            body = quote_spanned!(m.span()=>#base_ident::#method_identifier(#parameters));
            static_ = Some(Token![static](Span::call_site()));
            fn_ = Some(Token![fn](Span::call_site()));
            mut_ = None;
        };

        // call wrapper constructor on produced value if necessary (if output is also wrapped)
        m.out.as_ref().map(|out_type| {
            if out_type.is_wrapped() || out_type.is_self(){
                let resolved_out_type = out_type.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                let wrapper_out_type = format_ident!("Lua{}",resolved_out_type.base_ident());
                body = quote_spanned!{m.span()=>
                    #wrapper_out_type::new(#body)
                };
            } 
        });

        // we must output a result so wrap in Ok.
        body = quote_spanned!(m.span()=>Ok(#body));

        // for every wrapper involved as a parameter (and possibly the `self` receiver) which is a reference,
        // wrap the expression in a val/val_mut call, to allow references as parameters
        m.args
            .iter()
            .zip(parameter_identifiers.iter())
            .map(|(a,b)| (a.is_any_ref().then_some(a),b))
            .chain(once((m.self_.as_ref().map(|(v,_)|v),&receiver_argument_identifier)))
            .filter_map(|(a,b)| Some((a?,b)))
            .for_each(|(arg,arg_ident)| {
                if arg.is_any_ref() {
                    let method_call = arg.is_mut_ref()
                    .then(|| format_ident!("val_mut",span=arg.span()))
                    .unwrap_or_else(|| format_ident!("val",span=arg.span()));
                    body = quote_spanned!{m.span()=>
                        #arg_ident.#method_call(|#arg_ident| #body)?
                    }            
                }
            });


        // finally generate the full method definition

        let docstrings = m.docstring.iter().collect::<Punctuated<_,EmptyToken>>();
        let method_identifier_string = method_identifier.to_string();
        let self_ident = m.self_.as_ref()
            .map(|_| quote_spanned!(m.span()=>#receiver_argument_identifier,))
            .unwrap_or_else(|| Default::default());
        
        parse_quote_spanned!{m.span()=>            
            #docstrings
            #static_ #mut_ #fn_ #method_identifier_string =>|_,#self_ident (#(#parameter_identifiers),*):(#(#parameter_types),*)| #body
        }
    }).collect::<Vec<_>>())
}
