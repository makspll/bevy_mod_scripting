use indexmap::{IndexMap, IndexSet};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{parse_quote, parse_quote_spanned, spanned::Spanned};

use crate::{
    common::{
        arg::SimpleType,
        derive_flag::DeriveFlag,
        newtype::Newtype,
        ops::{OpExpr, OpName, Side},
    },
    lua::{lua_method::LuaMethod, LuaImplementor},
};

pub(crate) fn make_bin_ops<'a>(
    implementor: &mut LuaImplementor,
    flag: &DeriveFlag,
    new_type: &'a Newtype,
    out: &mut Vec<LuaMethod>,
) -> Result<(), syn::Error> {
    let newtype_name = &new_type.args.wrapper_type;
    let wrapped_type = &new_type.args.base_type_ident;

    let (ident, ops) = match flag {
        DeriveFlag::BinOps { ident, ops, .. } => (ident, ops),
        _ => panic!("Expected BinOps flag"),
    };

    let mut op_name_map: IndexMap<OpName, Vec<&OpExpr>> = Default::default();

    ops.iter()
        .for_each(|v| op_name_map.entry(v.op.clone()).or_default().push(v));

    for (op_name, ops) in op_name_map.into_iter() {
        let metamethod_name = op_name.to_rlua_metamethod_path();

        let return_union = ops
            .iter()
            .map(|v| {
                v.map_return_type_with_default(parse_quote!(self), |t| {
                    let type_ = t.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                    if t.is_wrapped() || t.is_self() {
                        format_ident!("Lua{}", type_.base_ident())
                    } else {
                        type_.base_ident().clone()
                    }
                })
            })
            .collect::<IndexSet<_>>();

        let return_arg_type = implementor.generate_register_union(
            return_union.iter().map(|t| t.to_string()),
            &new_type.args.base_type_ident.to_string(),
        );

        let newtype = &new_type.args.wrapper_type;

        // makes match handlers for the case where v is the union side
        // also returns if this union contains the receiver
        let mut make_handlers = |op_exprs: &Vec<&OpExpr>,
                                 receiver_side: Side|
         -> Result<(TokenStream, TokenStream, bool), syn::Error> {
            let mut union_has_receiver = false;
            // the types we are forming a union over are on the side opposite to the receiver,
            // collect them and later stringify them, then generate a teal union type
            let mut union = op_exprs
                .iter()
                .filter(|v| v.has_receiver_on_side(receiver_side))
                .map(|v| {
                    v.map_side(receiver_side.opposite(), |t| {
                        union_has_receiver = true;
                        let type_ =
                            t.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                        if t.is_wrapped() || t.is_self() {
                            format_ident!("Lua{}", type_.base_ident())
                        } else {
                            type_.base_ident().clone()
                        }
                    })
                    .expect("Expected binary expression!")
                })
                .collect::<IndexSet<_>>();

            // this happens when all the receivers sit only on one side
            if union.is_empty() {
                union.insert(newtype.clone());
            }

            let arg_type = implementor.generate_register_union(
                union.iter().map(|t| t.to_string()),
                &new_type.args.base_type_ident.to_string(),
            );

            let match_patterns = op_exprs
                .iter()
                .filter(|v| v.has_receiver_on_side(receiver_side))
                .map(|op_expr| {
                    let (l_exp, r_exp) = op_expr.map_both(|t, side| {
                        // receiver is always a lua wrapper by definition
                        let is_wrapped = t.is_wrapped() || t.is_self();
                        let arg_ident = t
                            .is_self()
                            .then(|| quote::quote!(ud))
                            .unwrap_or_else(|| quote::quote!(v));

                        let inner = is_wrapped
                            .then(|| quote_spanned!(op_expr.span()=>#arg_ident.inner()?))
                            .unwrap_or_else(|| quote_spanned!(op_expr.span()=>#arg_ident));

                        let is_ref = t.is_any_ref();
                        let o = if is_ref {
                            quote_spanned!(op_expr.span()=>&#inner)
                        } else {
                            quote_spanned!(op_expr.span()=>#inner)
                        };
                        o
                    });
                    let operator = op_expr.op.to_rust_method_ident();
                    let mut body = quote_spanned!(op_expr.span()=>#l_exp.#operator(#r_exp));
                    op_expr.map_return_type_with_default(parse_quote! {self}, |return_type| {
                        let is_wrapped = return_type.is_wrapped() || return_type.is_self();
                        let resolved_type = return_type
                            .type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                        let type_ident;
                        if return_type.is_wrapped() || return_type.is_self() {
                            type_ident = format_ident!("Lua{}", resolved_type.base_ident())
                        } else {
                            type_ident = resolved_type.base_ident().clone()
                        }

                        if is_wrapped {
                            body = quote_spanned! {op_expr.span()=>#type_ident::new(#body)}
                        }

                        body = quote_spanned! {op_expr.span()=>#return_arg_type::#type_ident(#body)}
                    });

                    let curr_arg_ident = op_expr
                        .map_side(receiver_side.opposite(), |t| {
                            let resolved_type =
                                t.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone()));
                            if t.is_wrapped() || t.is_self() {
                                format_ident!("Lua{}", resolved_type.base_ident())
                            } else {
                                resolved_type.base_ident().clone()
                            }
                        })
                        .unwrap();

                    Ok(quote_spanned! {op_expr.span()=>
                        #arg_type::#curr_arg_ident(v) => Ok(#body),
                    })
                })
                .collect::<Result<TokenStream, syn::Error>>()?;

            let receiver_side_string = receiver_side.to_string();
            Ok((
                quote_spanned! {newtype.span()=>
                    match v {
                        #match_patterns
                        _ => Err(tealr::mlu::mlua::Error::RuntimeError(
                            format!("tried to `{}` `{}` with another argument on the `{}` side, but its type is not supported",
                                stringify!(#metamethod_name),
                                stringify!(#newtype_name),
                                #receiver_side_string
                            )
                        ))
                    }
                },
                arg_type.to_token_stream(),
                union_has_receiver,
            ))
        };

        let (mut lhs_ud_handlers, lhs_arg_type, rhs_contains_receiver) =
            make_handlers(&ops, Side::Right)?;
        let (mut rhs_ud_handlers, rhs_arg_type, lhs_contains_receiver) =
            make_handlers(&ops, Side::Left)?;

        if lhs_contains_receiver {
            rhs_ud_handlers = quote_spanned! {flag.span()=>
                (#lhs_arg_type::#newtype_name(ud),v) => {#rhs_ud_handlers},
            };
        } else {
            rhs_ud_handlers = Default::default();
        }

        if rhs_contains_receiver {
            lhs_ud_handlers = quote_spanned! {flag.span()=>
                (v,#rhs_arg_type::#newtype_name(ud)) => {#lhs_ud_handlers},
            };
        } else {
            lhs_ud_handlers = Default::default();
        }
        let o = parse_quote_spanned! {ident.span()=>
            fn (mlua::MetaMethod::#metamethod_name) => |ctx, (lhs,rhs) :(#lhs_arg_type,#rhs_arg_type)| {

                match (lhs,rhs) {
                    // we always check implementor is on the left first
                    #rhs_ud_handlers
                    #lhs_ud_handlers
                    _ => Err(tealr::mlu::mlua::Error::RuntimeError(
                            format!("tried to `{}` two arguments, none of which are of type `{}` ",
                                stringify!(#metamethod_name),
                                stringify!(#newtype_name)
                            )
                        ))
                }
            }
        };
        out.push(o);
    }

    Ok(())
}
