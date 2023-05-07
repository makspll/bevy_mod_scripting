use bevy_mod_scripting_common::{derive_flag::DeriveFlag, newtype::Newtype, ops::Side};
use quote::quote_spanned;
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned};

use crate::lua_method::LuaMethod;

pub(crate) fn make_unary_ops(
    flag: &DeriveFlag,
    new_type: &Newtype,
    out: &mut Vec<LuaMethod>,
) -> Result<(), syn::Error> {
    let newtype = &new_type.args.wrapper_type;

    let (ident, ops) = match flag {
        DeriveFlag::UnaryOps { ident, ops, .. } => (ident, ops),
        _ => panic!("Expected UnaryOps flag"),
    };

    ops.iter().for_each(|op| {
        let meta = op.op.to_rlua_metamethod_path();
        let mut body = op
            .map_side(Side::Right, |s| {
                if s.is_any_ref() {
                    quote_spanned! {op.span()=>(&ud.inner()?)}
                } else {
                    quote_spanned! {op.span()=>ud.inner()?}
                }
            })
            .expect("Expected unary expression");

        // return has to be self due to how OpExpr works
        // wrap in constructor
        let resolved_type = op.right.self_().unwrap().resolve_as(parse_quote!(#newtype));
        body = quote_spanned! {op.span()=>#resolved_type::new(#body)};

        out.push(parse_quote_spanned! {ident.span()=>
            (bevy_mod_scripting_lua::tealr::mlu::mlua::MetaMethod::#meta) => |_,ud,()|{
                return Ok(#body)
            }
        });
    });

    Ok(())
}
