use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute, parse_quote, Ident};

use crate::{lua::{lua_method::LuaMethod, LuaImplementor}, common::{derive_flag::DeriveFlag, newtype::Newtype, ops::{OpName, OpExpr, Side}, type_base_string, stringify_token_group, arg::SimpleType}, EmptyToken};

pub(crate) fn make_bin_ops<'a>(implementor: &mut LuaImplementor,flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) -> Result<(), syn::Error> {

    let newtype_name = &new_type.args.wrapper_type;


    let (ident, ops) = match flag {
        DeriveFlag::BinOps { ident, ops, .. } => (ident, ops),
        _ => panic!("Expected BinOps flag")
    };

    let mut op_name_map : IndexMap<OpName,Vec<&OpExpr>> = Default::default();

    ops.iter().for_each(|v| op_name_map.entry(v.op.clone()).or_default().push(v));

    for (op_name,ops) in op_name_map.into_iter(){
        let metamethod_name = op_name.to_rlua_metamethod_path();

        let (lhs_union ,rhs_union) = ops.iter()
                                        .partition::<Vec<&&OpExpr>,_>(|t| !t.has_receiver_on_lhs());

        let return_union = ops.iter().map(|v| 
            v.map_return_type_with_default(parse_quote!(self), |t| {
               t.type_()
                .cloned() // meh
                .unwrap_or_else(|self_| self_.resolve_as(parse_quote!{#newtype_name}))
            })
        ).collect::<IndexSet<_>>();


        let return_arg_type = implementor.generate_register_union(return_union.iter().map(|t| t.base_ident().to_string()),
            &new_type.args.base_type_ident.to_string());

        let newtype = &new_type.args.wrapper_type;
        
        // makes match handlers for the case where v is the union side
        let mut make_handlers = |op_exprs : Vec<&&OpExpr>, receiver_side : Side| -> Result<(TokenStream,Ident),syn::Error> {

            // the types we are forming a union over are on the side opposite to the receiver,
            // collect them and later stringify them, then generate a teal union type
            let mut union = op_exprs
                                .iter()
                                .map(|v| 
                                    v.map_side(receiver_side.opposite(),|a| 
                                        a.type_()                
                                            .cloned() // meh
                                            .unwrap_or_else(|self_| self_.resolve_as(parse_quote!{#newtype_name}))
                                    ).expect("Expected binary expression!")
                                )
                                .collect::<IndexSet<_>>();
            let arg_type;

            arg_type = implementor.generate_register_union(union.iter().map(|t| t.base_ident().to_string()),
                &new_type.args.base_type_ident.to_string());
            

            let match_patterns = op_exprs.iter()
                .map(|op_expr| {
                    let mut type_opposite_receiver = None;
                    let (l_exp,r_exp) = op_expr.map_both(|t,side| {
                        // receiver is always a lua wrapper by definition
                        let is_wrapped = t.is_wrapped() || t.is_self();
                        let inner = is_wrapped
                            .then(|| quote_spanned!(op_expr.span()=>v.inner()?))
                            .unwrap_or_else(|| quote_spanned!(op_expr.span()=>v));

                        let type_ = t.type_()
                            .cloned()
                            .unwrap_or_else(|self_| self_.resolve_as(parse_quote!(#newtype)));
                        let type_ident = type_.base_ident();


                        let o =if let SimpleType::Ref{..} = type_ {
                            quote_spanned!(op_expr.span()=>&#inner)
                        } else {
                            quote_spanned!(op_expr.span()=>)    
                        };
                        
                        if side == receiver_side.opposite() {
                            type_opposite_receiver = Some(type_)
                        }

                        o
                    });
                    let operator = op_expr.op.to_rust_method_ident();
                    let mut body = quote_spanned!(op_expr.span()=>#l_exp.#operator(#r_exp));

                    op_expr.map_return_type_with_default(parse_quote!{self},|return_type| {
                        let is_wrapped = return_type.is_wrapped() || return_type.is_self();
                        let resolved_type = return_type.type_()
                            .cloned()
                            .unwrap_or_else(|self_| self_.resolve_as(parse_quote!(#newtype)));
                        let type_ident = resolved_type.base_ident();
                        if is_wrapped {
                            body = quote_spanned!{op_expr.span()=>#type_ident::new(#body)}
                        }

                        body = quote_spanned!{op_expr.span()=>#return_arg_type::#type_ident(#body)}
                    });

                    assert!(type_opposite_receiver.is_some(), "Something went wrong, we didn't see a type opposite the receiver");

                    Ok(quote_spanned!{op_expr.span()=>
                        #arg_type::#type_opposite_receiver(v) => Ok(#body),
                    })
                }).collect::<Result<TokenStream,syn::Error>>()?;

            Ok((quote_spanned!{newtype.span()=>
                match v {
                    #match_patterns
                    _ => Err(tealr::mlu::mlua::Error::RuntimeError(
                        format!("tried to `{}` `{}` with another argument on the `{}` side, but its type is not supported",
                            stringify!(#metamethod_name),
                            stringify!(#newtype_name),
                            receiver_side.opposite()
                        )
                    ))
                }
            },arg_type))

        };

        let (mut rhs_ud_handlers, rhs_arg_type) = make_handlers(rhs_union,Side::Left)?;

        let (mut lhs_ud_handlers, lhs_arg_type) = make_handlers(lhs_union,Side::Right)?;


        if lhs_arg_type.to_string().contains(&newtype_name.to_string()){
            rhs_ud_handlers = quote_spanned!{flag.span()=>
                (#lhs_arg_type::#newtype_name(ud),v) => {#rhs_ud_handlers},
            };
        } else {
            rhs_ud_handlers = Default::default();
        }

        if rhs_arg_type.to_string().contains(&newtype_name.to_string()){
            lhs_ud_handlers = quote_spanned!{flag.span()=>
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
