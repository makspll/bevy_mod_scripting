use indexmap::{IndexMap, IndexSet};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{parse_quote_spanned, spanned::Spanned, Token};

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

    // begin by collecting the expressions for the same operator
    // I.e. [L Add self,self Add R],[L Mul self, self Add R] etc.
    let mut op_expressions: IndexMap<OpName, Vec<&OpExpr>> = Default::default();

    ops.iter()
        .for_each(|v| op_expressions.entry(v.op.clone()).or_default().push(v));

    // each operator maps to a single metamethod so we must do some runtime thinking
    for (op_name, ops) in op_expressions.into_iter() {

        let metamethod_name = op_name.to_rlua_metamethod_path();

        // the return type is the union of expression return types
        // collect it and produce an enumeration type which we can use to return this union as a single type
        let return_union = ops
            .iter()
            .map(|v| {
                let mut resolved_return_type = v.return_type.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();
                if v.return_type.is_wrapped() || v.return_type.is_self() {
                    resolved_return_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{}", ident));
                } 
                resolved_return_type.into_base_ident()
                
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
                    v.map_side(receiver_side.opposite(), |arg_type| {
                        union_has_receiver = true;
                        let mut resolved_arg_type = arg_type.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();
                        if arg_type.is_wrapped() || arg_type.is_self() {
                            resolved_arg_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{}", ident));
                        } 
                        resolved_arg_type.into_base_ident()
                    })
                    .expect("Expected binary expression!")
                })
                .collect::<IndexSet<_>>();

            // this happens when all the receivers sit only on one side
            if union.is_empty() {
                union.insert(newtype.clone());
            }

            let union_parameter_type = implementor.generate_register_union(
                union.iter().map(|t| t.to_string()),
                &new_type.args.base_type_ident.to_string(),
            );

            // now we generate a pattern statement for each type appearing on this side of the expression
            // i.e. we are operating in one of (v,right_arg::LuaMyWrapper) or (left_arg::LuaMyWrapper,v)
            // generate a match statement: match v {}, where each arm corresponds to a type in this parameter type's union
            let match_patterns = op_exprs
                .iter()
                .filter(|v| v.has_receiver_on_side(receiver_side))
                .map(|op_expr| {
                    // first of all resolve both sides of the expression 
                    let (l_exp, r_exp) = op_expr.map_both(|arg_type, side| {
                        // receiver is always a lua wrapper by definition
                        let arg_ident = arg_type
                            .is_self()
                            .then(|| quote::quote!(ud))
                            .unwrap_or_else(|| quote::quote!(v));

                        let ampersand = arg_type.is_any_ref()
                            .then_some(Token![&](arg_type.span()));

                        if arg_type.is_wrapped() || arg_type.is_self() {
                            quote_spanned!(op_expr.span()=>#ampersand #arg_ident.inner()?)
                        } else {
                            quote_spanned!(op_expr.span()=>#ampersand #arg_ident)
                        }
                    });

                    // combine into one expression
                    let rust_operator = op_expr.op.to_rust_method_ident();
                    let mut expression_body = quote_spanned!(op_expr.span()=>#l_exp.#rust_operator(#r_exp));
                    
                    // then resolve return type and wrap in constructor if need be
                    let mut resolved_type = op_expr.return_type
                        .type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();

                    if op_expr.return_type.is_wrapped() || op_expr.return_type.is_self() {
                        resolved_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{ident}"));
                        expression_body = quote_spanned! {op_expr.span()=>#resolved_type::new(#expression_body)}
                    } 
                    
                    let type_ident = resolved_type.base_ident();
                    expression_body = quote_spanned! {op_expr.span()=>#return_arg_type::#type_ident(#expression_body)};
                    
                    // resolve the type opposite to the receiver side (still could be another receiver)
                    let matched_identifier = op_expr
                        .map_side(receiver_side.opposite(), |t| {
                            let mut resolved_type =
                                t.type_or_resolve(|| SimpleType::BaseIdent(wrapped_type.clone())).into_owned();
                            if t.is_wrapped() || t.is_self() {
                                resolved_type.mutate_base_ident(|ident| *ident = format_ident!("Lua{ident}"))
                            } 
                            resolved_type.into_base_ident()
                        })
                        .unwrap();
                    Ok(quote_spanned! {op_expr.span()=>
                        #union_parameter_type::#matched_identifier(v) => Ok(#expression_body),
                    })
                })
                .collect::<Result<TokenStream, syn::Error>>()?;

            // generate match statement containing these patterns + appropriate error message
            let receiver_side_string = receiver_side.to_string();
            Ok((
                quote_spanned! {op_name.span()=>
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
                union_parameter_type.to_token_stream(),
                union_has_receiver,
            ))
        };

        // generate these match statements for cases where self is on left and right side
        let (mut rhs_union_handlers, rhs_arg_type, lhs_contains_receiver) =
            make_handlers(&ops, Side::Left)?;
        let (mut lhs_union_handlers, lhs_arg_type, rhs_contains_receiver) =
            make_handlers(&ops, Side::Right)?;

        if lhs_contains_receiver {
            rhs_union_handlers = quote_spanned! {flag.span()=>
                (#lhs_arg_type::#newtype_name(ud),v) => {#rhs_union_handlers},
            };
        } else {
            // in the case self never appears on that side we don't check it, since a union type won't exist for that case
            rhs_union_handlers = Default::default();
        }

        if rhs_contains_receiver {
            lhs_union_handlers = quote_spanned! {flag.span()=>
                (v,#rhs_arg_type::#newtype_name(ud)) => {#lhs_union_handlers},
            };
        } else {
            lhs_union_handlers = Default::default();
        }
        let o = parse_quote_spanned! {ident.span()=>
            fn (mlua::MetaMethod::#metamethod_name) => |ctx, (lhs,rhs) :(#lhs_arg_type,#rhs_arg_type)| {

                match (lhs,rhs) {
                    // we always check implementor is on the left first
                    #rhs_union_handlers
                    #lhs_union_handlers
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
