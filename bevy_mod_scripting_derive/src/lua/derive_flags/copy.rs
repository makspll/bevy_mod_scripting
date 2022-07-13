use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute, parse_quote, Ident};

use crate::{lua::{lua_method::LuaMethod, LuaImplementor}, common::{derive_flag::DeriveFlag, newtype::Newtype, ops::{OpName, OpExpr}, type_base_string}, EmptyToken};

pub(crate) fn make_copy<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>, functions_so_far : &IndexMap<String, Vec<LuaMethod>>) -> Result<(), syn::Error> {

    let newtype_name = &new_type.args.wrapper_type;


    let (ident, invocations) = match flag {
        DeriveFlag::Copy { ident, invocations, .. } => (ident, invocations),
        _ => panic!("Expected UnaryOps flag")
    };

    let mut new_methods = Vec::default();
                    for i in invocations{
                        let key = &i.target;
                        let key = quote_spanned!{key.span()=>#key}.to_string();

                        let methods = functions_so_far.get(&key).expect(&format!("Target lua wrapper type `{}` not found",key));

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
                    out.extend(new_methods);
  

    Ok(())

}
