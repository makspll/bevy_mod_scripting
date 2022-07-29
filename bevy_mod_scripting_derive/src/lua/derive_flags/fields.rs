use indexmap::IndexMap;
use proc_macro2::Span;
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute, parse_quote};

use crate::{lua::lua_method::LuaMethod, common::{derive_flag::DeriveFlag, newtype::Newtype, arg::SimpleType}, EmptyToken};



pub(crate) fn make_fields<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) -> Result<(),syn::Error> {
    let newtype_name = &new_type.args.wrapper_type;


    let (ident,fields) = match flag {
        DeriveFlag::Fields { ident, fields, ..  } => (ident,fields),
        _ => panic!("Expected Fields flag")
    };

    for f in fields {
        // resolve the type of this field
        let field_type = f.type_
            .type_()
            .cloned()
            .unwrap_or_else(|self_|  self_.resolve_as(parse_quote!(#newtype_name)));

        if let SimpleType::Ref{..} = field_type{
            return Err(syn::Error::new_spanned(f, "Reference fields are not supported"))
        }

        let ds : Punctuated<Attribute,EmptyToken> = f.docstring.iter().cloned().collect();
        let id = &f.member;
        let id_string = &f.member.to_token_stream().to_string();
        let type_string = field_type.base_ident().to_string();
        
        let field_type_ident = f.type_.is_wrapped()
            .then(|| format_ident!("Lua{}",field_type.base_ident()))
            .unwrap_or_else(|| field_type.base_ident().clone());


        let expr_getter = f.type_.is_wrapped()
            .then(|| {
                let field_type_ident = format_ident!("Lua{}",field_type.base_ident());
                quote_spanned!{f.span()=>
                    Ok(#field_type_ident::new_ref(s.script_ref().index(std::borrow::Cow::Borrowed(#id_string))))
                }
            }).unwrap_or_else(|| {
                if type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        Ok(s.script_ref().index(std::borrow::Cow::Borrowed(#id_string)))
                    }
                }
                quote_spanned!{f.span()=>{}
                    s.val(|s| Ok(s.#id.clone()))?
                }
            });
        

        out.push(parse_quote_spanned! {f.span()=>
            #ds
            get #id_string => |_,s : &#newtype_name| {
                #expr_getter
            }
        });

        let expr_setter = f.type_.is_wrapped()
            .then(|| {quote_spanned!{f.span()=>
                Ok(o.apply_self_to_base(&mut s.script_ref().index(std::borrow::Cow::Borrowed(#id_string)))?)
            }}).unwrap_or_else(|| {
                if type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        Ok(s.script_ref().index(std::borrow::Cow::Borrowed(#id_string)).apply(&o.ref_)?)
                    }
                }
                quote_spanned!{f.span()=>
                    s.val_mut(|s| Ok(s.#id = o))?
                }
            });

        out.push(parse_quote_spanned! {f.span()=>
            set #id_string => |_,s: &mut #newtype_name, o: #field_type_ident| {
                #expr_setter
            }
        });
    }

    Ok(())
}
