use indexmap::IndexMap;
use proc_macro2::Span;
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute};

use crate::{lua::lua_method::LuaMethod, common::{derive_flag::DeriveFlag, newtype::Newtype}, EmptyToken};



pub(crate) fn make_fields<'a>(flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) -> Result<(),syn::Error> {
    let newtype_name = &new_type.args.wrapper_type;


    let (ident,fields) = match flag {
        DeriveFlag::Fields { ident, fields, ..  } => (ident,fields),
        _ => panic!("Expected Fields flag")
    };

    for f in fields {
        if let Type::Reference(r) = &f.type_ {
            return Err(syn::Error::new_spanned(f, "Reference fields are not supported"))
        }
        let ds : Punctuated<Attribute,EmptyToken> = f.docstring.iter().cloned().collect();
        let id = &f.member;
        let id_string = &f.member.to_token_stream().to_string();
        let type_ = &f.type_;
        
        let type_string = type_.to_token_stream()
            .to_string();

        let expr_getter = type_string.starts_with("Lua") 
            .then(|| {quote_spanned!{f.span()=>
                Ok(#type_::new_ref(&s.script_ref().index(std::borrow::Cow::Borrowed(#id_string))?))
            }}).unwrap_or_else(|| {
                if type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        s.script_ref().index(std::borrow::Cow::Borrowed(#id_string))
                    }
                }
                quote_spanned!{f.span()=>{}
                    s.val(|s| Ok(s.#id))
                }
            });
        

        out.push(parse_quote_spanned! {f.span()=>
            #ds
            get #id_string => |_,s : &#newtype_name| {
                #expr_getter
            }
        });

        let expr_setter = (type_string.starts_with("Lua") || type_string == "reflect_only")
            .then(|| {quote_spanned!{f.span()=>
                Ok(o.apply_self_to_base(&mut s.script_ref().index(std::borrow::Cow::Borrowed(#id_string))?))
            }}).unwrap_or_else(|| {
                if type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        Ok(s.script_ref().index(std::borrow::Cow::Borrowed(#id_string))?.apply(&o.ref_))
                    }
                }
                quote_spanned!{f.span()=>
                    s.val_mut(|s| Ok(s.#id = o))
                }
            });

        out.push(parse_quote_spanned! {f.span()=>
            set #id_string => |_,s: &mut #newtype_name, o: #type_| {
                #expr_setter
            }
        });
    }

    Ok(())
}
