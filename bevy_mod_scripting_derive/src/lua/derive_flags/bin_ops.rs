use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, ToTokens, quote_spanned};
use syn::{punctuated::Punctuated, Token, LitInt, Type, spanned::Spanned, parse_quote_spanned, Attribute, parse_quote, Ident};

use crate::{lua::{lua_method::LuaMethod, LuaImplementor}, common::{derive_flag::DeriveFlag, newtype::Newtype, ops::{OpName, OpExpr}, type_base_string}, EmptyToken};

pub(crate) fn make_bin_ops<'a>(implementor: &mut LuaImplementor,flag: &DeriveFlag,new_type : &'a Newtype, out : &mut Vec<LuaMethod>) -> Result<(), syn::Error> {

    let newtype_name = &new_type.args.wrapper_type;


    let (ident, ops) = match flag {
        DeriveFlag::BinOps { ident, ops, .. } => (ident, ops),
        _ => panic!("Expected BinOps flag")
    };

    let mut op_name_map : IndexMap<OpName,Vec<&OpExpr>> = Default::default();

    ops.iter().for_each(|v| op_name_map.entry(v.op.clone()).or_default().push(v));

    for (op_name,ops) in op_name_map.into_iter(){
        // TODO: optimize this somehow if possible (the generated code)?

        let metamethod_name = op_name.to_rlua_metamethod_path();

        let (lhs_union ,rhs_union) = ops.iter()
                                        .partition::<Vec<&&OpExpr>,_>(|t| !t.has_receiver_on_lhs());

        let return_union = ops.iter().map(|v| 
            v.map_return_type_with_default(parse_quote!{#newtype_name}, |t| {
               t.clone()
            })
        ).collect::<IndexSet<_>>();
        let return_union_strings = return_union.iter().map(type_base_string).map(Option::unwrap).collect::<Vec<_>>();
        let return_arg_type = implementor.generate_register_union(&return_union_strings,&newtype_name.to_string()[3..]);

        let newtype = &new_type.args.wrapper_type;
        
        // makes match handlers for the case where v is the union side
        let mut make_handlers = |op_exprs : Vec<&&OpExpr>, side_name : &str| -> Result<(TokenStream,Ident),syn::Error> {

            let mut union_strings = op_exprs
                                .iter()
                                .map(|v| v.map_type_side(|v| type_base_string(v).expect("Unsopported rhs type")).expect("Expected at least one non implementor type"))
                                .collect::<Vec<_>>();
            let arg_type;
            let implementor_appears = ops.len() != union_strings.len();

            if implementor_appears {
                union_strings.push(newtype_name.to_string());
                arg_type = implementor.generate_register_union(&union_strings,&newtype_name.to_string()[3..]);
            } else {
                arg_type = implementor.generate_register_union(&union_strings,&newtype_name.to_string()[3..]);
            };

            let match_patterns = op_exprs.iter()
                .enumerate()
                .map(|(idx,v)| {
                    let type_ = format_ident!{"{}",union_strings[idx]};
                    let is_wrapper = union_strings[idx].starts_with("Lua");
                    let mut body = v.map_binary(|t| {
                        // unpack wrappers
                        let inner = if is_wrapper{
                            quote_spanned!{v.span()=>v.clone()}
                        } else {
                            quote_spanned!{v.span()=>v}
                        };
                        if let Type::Reference(r) = t{
                            quote_spanned!{v.span()=>(&#inner)}
                        } else {
                            inner
                        }
                    }, |s| {
                        if s.reference.is_some(){
                            quote_spanned!{v.span()=>&ud.clone()}
                        } else {
                            quote_spanned!{v.span()=>(ud.clone())}
                        }
                    })?;

                    let wrapped = v.map_return_type_with_default(parse_quote!{#newtype},|v| {
                        let str_type = type_base_string(v).expect("Expected simple return type");
                        let ident_type = format_ident!("{str_type}");

                        if str_type.starts_with("Lua") {
                            body = quote_spanned!{v.span()=>#ident_type::new(#body)}
                        };

                        quote_spanned!{v.span()=>#return_arg_type::#ident_type(#body)}
                    });

                    Ok(quote_spanned!{v.span()=>
                        #arg_type::#type_(v) => Ok(#wrapped),
                    })
                }).collect::<Result<TokenStream,syn::Error>>()?;

            Ok((quote_spanned!{newtype.span()=>
                match v {
                    #match_patterns
                    _ => Err(tealr::mlu::mlua::Error::RuntimeError(
                        format!("tried to `{}` `{}` with another argument on the `{}` side, but its type is not supported",
                            stringify!(#metamethod_name),
                            stringify!(#newtype_name),
                            #side_name
                        )
                    ))
                }
            },arg_type))

        };

        let (mut rhs_ud_handlers, rhs_arg_type) = make_handlers(rhs_union,"right")?;

        let (mut lhs_ud_handlers, lhs_arg_type) = make_handlers(lhs_union,"left")?;


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
