use quote::{format_ident, quote_spanned};
use syn::{parse_quote_spanned, spanned::Spanned};

use crate::{
    common::{arg::SimpleType, derive_flag::DeriveFlag, newtype::Newtype},
    lua::lua_method::LuaMethod,
};

pub(crate) fn make_fields<'a>(
    flag: &DeriveFlag,
    new_type: &'a Newtype,
    out: &mut Vec<LuaMethod>,
) -> Result<(), syn::Error> {
    let newtype_name = &new_type.args.wrapper_type;

    let (ident, fields) = match flag {
        DeriveFlag::Fields { ident, fields, .. } => (ident, fields),
        _ => panic!("Expected Fields flag"),
    };

    // each field is mapped to a getter + setter function
    for f in fields {
        if f.type_.is_any_ref() {
            return Err(syn::Error::new_spanned(
                f,
                "Reference fields are not supported",
            ));
        }

        // resolve the type of this field
        let mut resolved_field_type = f
            .type_
            .type_or_resolve(|| SimpleType::BaseIdent(new_type.args.base_type_ident.clone()))
            .into_owned();

        if f.type_.is_wrapped() || f.type_.is_self() {
            resolved_field_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{ident}"));
        }

        let id = &f.member;
        let (mut lua_id_string, rust_id_string) = match id {
            syn::Member::Named(string_id) => (string_id.to_string(), string_id.to_string()),
            syn::Member::Unnamed(index) => (format!("_{}", index.index), index.index.to_string()),
        };

        if let Some(new_name) = &f.parsed_attrs.script_name {
            lua_id_string = new_name.to_string();
        }

        let docstring = f.docstring.iter();
        let field_type_ident = resolved_field_type.base_ident();
        let field_type_string = field_type_ident.to_string();

        // make the getter method
        let expr_getter = f.type_.is_wrapped()
            .then(|| {
                quote_spanned!{f.span()=>
                    Ok(#field_type_ident::new_ref(s.script_ref().index(std::borrow::Cow::Borrowed(#rust_id_string))))
                }
            }).unwrap_or_else(|| {
                if field_type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        Ok(s.script_ref().index(std::borrow::Cow::Borrowed(#rust_id_string)))
                    }
                }
                quote_spanned!{f.span()=>{}
                    s.val(|s| Ok(s.#id.clone()))?
                }
            });

        out.push(parse_quote_spanned! {f.span()=>
            #(#docstring)*
            get #lua_id_string => |_,s : &#newtype_name| {
                #expr_getter
            }
        });

        // make the setter method
        let expr_setter = f.type_.is_wrapped()
            .then(|| {quote_spanned!{f.span()=>
                Ok(o.apply_self_to_base(&mut s.script_ref().index(std::borrow::Cow::Borrowed(#rust_id_string)))?)
            }}).unwrap_or_else(|| {
                if field_type_string == "ReflectedValue" {
                    return quote_spanned!{f.span()=>
                        Ok(s.script_ref().index(std::borrow::Cow::Borrowed(#rust_id_string)).apply(&o.ref_)?)
                    }
                }
                quote_spanned!{f.span()=>
                    s.val_mut(|s| Ok(s.#id = o))?
                }
            });

        out.push(parse_quote_spanned! {f.span()=>
            set #lua_id_string => |_,s: &mut #newtype_name, o: #field_type_ident| {
                #expr_setter
            }
        });
    }

    Ok(())
}
