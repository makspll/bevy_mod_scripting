pub(crate) mod lua_method;

use std::collections::{HashSet};

use indexmap::{IndexMap, IndexSet};
pub(crate) use lua_method::*;
use proc_macro2::{TokenStream, Span, Ident};
use syn::{spanned::Spanned, parse_quote_spanned, punctuated::Punctuated, LitInt, Token, Attribute, parse_quote, Type, TypePath, Path};

use crate::{common::{WrapperImplementor, WrapperFunction, newtype::NewtypeVariation, attribute_to_string_lit, derive_flag::DeriveFlag, ops::{OpName,OpExpr}, stringify_type_path, type_base_string}, EmptyToken};
use quote::{quote, quote_spanned, ToTokens, format_ident};

impl WrapperFunction for LuaMethod {}

#[derive(Default)]
pub(crate) struct LuaImplementor{
    implemented_unions : IndexSet<Ident>,
    additional_globals : TokenStream
}

impl LuaImplementor {

    /// Generates a union registers it, and makes sure no two identical unions exist, while removing duplicate entries in the enum
    fn generate_register_union(&mut self, type_idents : &Vec<String>) -> Ident{

        let unique_idents : Vec<String> = type_idents.iter().cloned().collect::<IndexSet<_>>().into_iter().collect::<Vec<_>>();

        let return_arg_type = format_ident!("Union{}",unique_idents.join(""));

        if !self.implemented_unions.contains(&return_arg_type){
            self.implemented_unions.insert(return_arg_type.clone());
            let return_arg = unique_idents.iter().map(|v| format_ident!("{v}")).collect::<Punctuated<Ident,Token![|]>>();
            
            self.additional_globals.extend(quote!{
                create_union_mlua!(pub enum #return_arg_type = #return_arg);
            });
        }

        return_arg_type
    }

}

impl WrapperImplementor for LuaImplementor {
    type Function = LuaMethod;

    fn generate_newtype_definition(&mut self, newtype : &crate::common::newtype::Newtype) -> std::result::Result<TokenStream, syn::Error> {
        let name = &newtype.args.wrapper_type;
        let base_type = &newtype.args.base_type_ident;

        Ok(match &newtype.args.variation {
            NewtypeVariation::Reflect{..} => quote_spanned!{newtype.span()=>
                pub type #name = crate::LuaWrapper<#base_type>;
            },
            NewtypeVariation::NonReflect { field , ..} => quote_spanned!{newtype.span()=>
                pub type #name = crate::AnyLuaWrapper<#field>;
            },
            NewtypeVariation::Primitive{..} => quote_spanned!{newtype.span()=>}
        })
    }

    fn generate_newtype_implementation<'a,I : Iterator<Item=&'a Self::Function>>(&mut self, newtype: &'a crate::common::newtype::Newtype, functions : I) -> std::result::Result<TokenStream, syn::Error> {
        
        if newtype.args.variation.is_primitive(){
            return Ok(Default::default())
        }

        let name = &newtype.args.wrapper_type;

        // provide documentation generation implementations
        let tealr_implementations = quote_spanned!{newtype.span()=>
            impl_tealr_type!(#name);
        };

        // generate documentation calls on type level
        let type_documentator : TokenStream = newtype.args.docstring.iter()
                                                .map(attribute_to_string_lit)
                                                .map(|ts| quote_spanned!{ts.span()=>
                                                    methods.document_type(#ts);
                                                }).collect();
        
        let functions = functions
                        .filter(|f| !f.method_type.is_static)
                        .map(|f| f.to_call_expr("methods"));

        // expose to lua
        let user_data_implementation = quote_spanned!{newtype.span()=>
            impl tealr::mlu::TealData for #name {
                fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                    #type_documentator
                    #(#functions)*
                }
            }
        };

        // group everything together
        Ok(quote_spanned!{newtype.span()=>
            #user_data_implementation
            #tealr_implementations
        })
    }



    fn generate_derive_flag_functions<'a, I : Iterator<Item=&'a crate::common::derive_flag::DeriveFlag>>(&mut self, new_type : &'a crate::common::newtype::Newtype, mut derive_flags : I,functions_so_far : & IndexMap<String, Vec<Self::Function>>) -> Result<Vec<LuaMethod>, syn::Error> {
        
        let mut out : Vec<Self::Function> = Default::default();
        let newtype_name = &new_type.args.wrapper_type;

        derive_flags.try_for_each(|v| {
            Ok::<(),syn::Error>(match v {
                DeriveFlag::DebugToString{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s))
                }),
                DeriveFlag::DisplayToString{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s))
                }),
                DeriveFlag::AutoMethods { ident,methods , ..} => {
                    
                    out.extend(methods.iter()
                        .map(|m| {
                            let ident = &m.ident;
                            let ident_str = ident.to_string();
                            let inner_args : Punctuated<proc_macro2::TokenStream,Token![,]> = m.args.iter().enumerate().map(|(idx,a)| {
                                let lit = LitInt::new(&idx.to_string(),ident.span());
                                let lit = if m.args.len() == 1 {
                                    quote_spanned!{ident.span()=>
                                        a
                                    }
                                } else {
                                    quote_spanned!{ident.span()=>
                                        a.#lit
                                    }
                                };

                                if a.to_token_stream().to_string().starts_with("Lua"){
                                    quote_spanned!{ident.span()=>
                                        #lit.inner()
                                    }
                                } else {
                                    quote_spanned!{ident.span()=>
                                        #lit
                                    }
                                }
                            }).collect();

                            let base_ident = new_type.args.base_type.path.segments.last().unwrap();

                            let (call_expr,static_,fn_,mut_, star) = if let Some((r,v)) = &m.self_ {
                                (quote_spanned!{ident.span()=>s.inner().#ident},
                                    None,
                                    None,
                                    r.mutability,
                                    r.reference.as_ref().map(|_| Token![*](Span::call_site())))
                            } else {
                                (quote_spanned!{ident.span()=>#base_ident::#ident},
                                    Some(Token![static](Span::call_site())),
                                    Some(Token![fn](Span::call_site())),
                                    None,
                                    None)
                            };


                            let mut inner_expr = quote_spanned!{ident.span()=> #call_expr(#inner_args)};
                            let out_ident = &m.out;
                            inner_expr = out_ident.as_ref().map(|v| 
                                if v.into_token_stream().to_string().starts_with("Lua"){
                                    quote_spanned!{ident.span()=>
                                        #out_ident::new(#inner_expr)
                                    }
                                } else {
                                    inner_expr.clone()
                                }
                            ).unwrap_or(quote_spanned!{ident.span()=>
                                #newtype_name::new(#inner_expr)
                            });

                            let self_ident = static_.map(|_| quote_spanned!{ident.span()=>}).unwrap_or(quote!{s,});
                            let ds : Punctuated<Attribute,EmptyToken> = m.docstring.iter().cloned().collect();

                            // we remove references
                            let args = &m.args;

                            parse_quote_spanned!{ident.span()=>
                                #ds
                                #static_ #mut_ #fn_ #ident_str =>|_,#self_ident a:(#args)| Ok(#inner_expr)
                            }
                        }).collect::<Vec<_>>())
                },
                DeriveFlag::Copy { ident, paren, invocations } =>{ 
                    let mut new_methods = Vec::default();
                    for i in invocations{
                        let key = &i.target;
                        let key = quote!{#key}.to_string();

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
                },
                DeriveFlag::BinOps {ident ,ops, .. } =>  {  

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
                        let return_arg_type = self.generate_register_union(&return_union_strings);

                        let newtype = &new_type.args.wrapper_type;
                        
                        // makes match handlers for the case where v is the union side
                        let mut make_handlers = |op_exprs : Vec<&&OpExpr>, side_name : &str| -> Result<(TokenStream,Ident),syn::Error> {

                            let mut union_strings = op_exprs
                                                .iter()
                                                .map(|v| v.map_type_side(|v| type_base_string(v).expect("Unsopported rhs type")).expect("Expected at least one non self type"))
                                                .collect::<Vec<_>>();
                            let arg_type;
                            let self_appears = ops.len() != union_strings.len();

                            if self_appears {
                                union_strings.push(newtype_name.to_string());
                                arg_type = self.generate_register_union(&union_strings);
                            } else {
                                arg_type = self.generate_register_union(&union_strings);
                            };

                            let match_patterns = op_exprs.iter()
                            .enumerate()
                            .map(|(idx,v)| {
                                let type_ = format_ident!{"{}",union_strings[idx]};
                                let is_wrapper = union_strings[idx].starts_with("Lua");
                                let mut body = v.map_binary(|t| {
                                    // unpack wrappers
                                    let inner = if is_wrapper{
                                        quote!{v.inner()}
                                    } else {
                                        quote!{v}
                                    };
                                    if let Type::Reference(r) = t{
                                        quote!{(&#inner)}
                                    } else {
                                        inner
                                    }
                                }, |s| {
                                    if s.reference.is_some(){
                                        quote!{&ud.inner()}
                                    } else {
                                        quote!{(ud.inner())}
                                    }
                                })?;

                                let wrapped = v.map_return_type_with_default(parse_quote!{#newtype},|v| {
                                    let str_type = type_base_string(v).expect("Expected simple return type");
                                    let ident_type = format_ident!("{str_type}");

                                    if str_type.starts_with("Lua") {
                                        body = quote!{#ident_type::new(#body)}
                                    };

                                    quote!{#return_arg_type::#ident_type(#body)}
                                });

                                Ok(quote!{
                                    #arg_type::#type_(v) => Ok(#wrapped),
                                })
                            }).collect::<Result<TokenStream,syn::Error>>()?;

                            Ok((quote!{
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
                            rhs_ud_handlers = quote!{
                                (#lhs_arg_type::#newtype_name(ud),v) => {#rhs_ud_handlers},
                            };
                        } else {
                            rhs_ud_handlers = Default::default();
                        }

                        if rhs_arg_type.to_string().contains(&newtype_name.to_string()){
                            lhs_ud_handlers = quote!{
                                (v,#rhs_arg_type::#newtype_name(ud)) => {#lhs_ud_handlers},
                            };
                        } else {
                            lhs_ud_handlers = Default::default();
                        }

                        let o = parse_quote_spanned! {ident.span()=>
                            fn (mlua::MetaMethod::#metamethod_name) => |ctx, (lhs,rhs) :(#lhs_arg_type,#rhs_arg_type)| {
                            
                                match (lhs,rhs) {
                                    // we always check self is on the left first 
                                    #rhs_ud_handlers
                                    #lhs_ud_handlers
                                    _ => Err(tealr::mlu::mlua::Error::RuntimeError(
                                            format!("tried to `{}` two arguments, none of which are of type `{}` ",
                                                stringify!(#metamethod_name),
                                                stringify!(#newtype_name)
                                            )
                                        ))
                                }

                                // return Err(tealr::mlu::mlua::Error::RuntimeError(format!("`{}` not supported between {:?} and {:?}",&metamethod_name,&lhs,&rhs)))
                            }
                        };
                        // panic!("{o}")
                        out.push(o);


                    }
                }
                DeriveFlag::UnaryOps { ident, ops, ..} => {
                    
                    ops.iter().for_each(|op| {

                        let meta = op.op.to_rlua_metamethod_path();
                        let mut body = op.map_unary(|s| {
                            if s.reference.is_some(){
                                quote!{(&ud.inner())}
                            } else {
                                quote!{ud.inner()}

                            }
                        }).expect("Expected unary expression");

                        op.map_return_type_with_default(parse_quote!{#newtype_name},|v| {
                            let str_type = type_base_string(v).expect("Expected simple return type");
                            let ident_type = format_ident!("{str_type}");

                            if str_type.starts_with("Lua") {
                                body = quote!{#ident_type::new(#body)}
                            };
                        });

                        out.push(parse_quote_spanned! {ident.span()=>
                            (mlua::MetaMethod::#meta) => |_,ud,()|{
                                return Ok(#body)
                            }
                        });
                    })

                }

            })
        })?;


        Ok(out)

    }

    fn generate_newtype_functions(&mut self, new_type : &crate::common::newtype::Newtype) -> Result<Vec<LuaMethod>, syn::Error> {
        
        if new_type.args.variation.is_primitive() {
            return Ok(Vec::default())
        };

        Ok(new_type.additional_lua_functions
            .as_ref()
            .map(|v| v.functions.iter().cloned().collect())
            .unwrap_or_default())    
    }

    fn generate_globals(&mut self, new_types: &crate::NewtypeList, all_functions : IndexMap<String, Vec<Self::Function>>) -> Result<TokenStream, syn::Error> {
        let from_lua : Punctuated<TokenStream,Token![,]> = new_types.new_types
            .iter()
            .filter(|base| !base.args.variation.is_non_reflect())
            .filter_map(|base| {
                let key = stringify_type_path(&base.args.base_type);
                let wrapper_type = &base.args.wrapper_type;

                let value = 
                    if base.args.variation.is_reflect(){
                        quote_spanned!{base.span()=>
                            |r,c,n| {
                                if let Value::UserData(v) = n {
                                    let mut v = v.borrow_mut::<#wrapper_type>()?;
                                    #wrapper_type::apply_self_to_base(v.deref_mut(),r);
                                    Ok(())
                                } else {
                                    Err(Error::RuntimeError("Invalid type".to_owned()))
                                }
                            }
                        }
                    } else if base.args.variation.is_primitive(){
                        base.additional_lua_functions.as_ref().unwrap().functions.iter().find(|f| 
                            f.method_type.get_inner_tokens().to_string() == "\"from\""
                        ).expect("").closure.to_applied_closure()
                    } else {
                        return None
                    };

                Some(quote!{#key => #value})
            }).collect();

        let to_lua : Punctuated<TokenStream,Token![,]> = new_types.new_types
            .iter()
            .filter(|base| !base.args.variation.is_non_reflect())
            .filter_map(|base| {
                let key = stringify_type_path(&base.args.base_type);
                let wrapper_type = &base.args.wrapper_type;

                let value = 
                    if base.args.variation.is_reflect(){
                        quote_spanned!{base.span()=>
                            |r,c| {
                                let usr = c.create_userdata(#wrapper_type::base_to_self(r)).unwrap();
                                Value::UserData(usr)
                            }
                        }
                    } else if base.args.variation.is_primitive(){
                        base.additional_lua_functions.as_ref().unwrap().functions.iter().find(|f| 
                            f.method_type.get_inner_tokens().to_string() == "\"to\""
                        ).expect("").closure.to_applied_closure()
                    } else {
                        return None
                    };

                Some(quote_spanned!{base.span()=> #key => #value})
            }).collect();


        let lookup_tables = quote_spanned!{new_types.span()=>

            pub static BEVY_TO_LUA: Map<&'static str,
                for<'l> fn(&crate::LuaRef,&'l Lua) -> tealr::mlu::mlua::Value<'l>
                > = phf_map!{
                    #to_lua,
                };

            pub static APPLY_LUA_TO_BEVY: Map<&'static str,
                for<'l> fn(&mut crate::LuaRef,&'l Lua, tealr::mlu::mlua::Value<'l>) -> Result<(),tealr::mlu::mlua::Error>
                > = phf_map!{
                    #from_lua,
                };
        };
        let (mut global_proxies,mut global_proxy_keys) = (Vec::default(), Vec::default());
        
        let global_modules : TokenStream = all_functions.iter()
                .map(|(newtype_name,methods)| {
                    let static_methods = methods.iter()
                        .filter(|f| f.method_type.is_static)
                        .map(|f| f.to_call_expr("methods"))
                        .collect::<Punctuated<TokenStream,EmptyToken>>();
    

                    if !static_methods.is_empty(){
                        let ident = format_ident!{"{}Globals",newtype_name};
                        let key = format_ident!{"{}",newtype_name.starts_with("Lua").then(|| &newtype_name[3..]).unwrap_or(&newtype_name)};
    
                        global_proxies.push(ident.clone());
                        global_proxy_keys.push(key);


                        let global_key = &newtype_name[3..];
    
                        return quote_spanned!{new_types.span()=>
                            struct #ident;
                            impl tealr::mlu::TealData for #ident {
                                fn add_methods<'lua,T: tealr::mlu::TealDataMethods<'lua,Self>>(methods: &mut T) {
                                    methods.document_type(concat!("Global methods for ", #global_key));
                                    #static_methods
                                }
                            }
    
                            impl_tealr_type!(#ident);
                        }
                    } 
    
                    Default::default()
                }).collect();
    
        let userdata_newtype_names : Vec<&Ident> = new_types.new_types
            .iter()
            .filter(|v| (!v.args.variation.is_primitive()).into())
            .map(|v| &v.args.wrapper_type)
            .collect();
                
            let external_types = new_types.additional_types.iter();

            let api_provider = quote_spanned!{new_types.span()=>
            
            struct BevyAPIGlobals;
            impl tealr::mlu::ExportInstances for BevyAPIGlobals {
                fn add_instances<'lua, T: tealr::mlu::InstanceCollector<'lua>>(
                    instance_collector: &mut T,
                ) -> mlua::Result<()> {
                    #(
                        instance_collector.document_instance(concat!("Global methods for ", stringify!(#global_proxy_keys)));
                        instance_collector.add_instance(stringify!(#global_proxy_keys).into(), |_| Ok(#global_proxies))?;
                    )*
    
                    Ok(())
                }
            }
    
            #global_modules
    
            #[derive(Default)]
            pub struct LuaBevyAPIProvider;
    
            impl crate::APIProvider for LuaBevyAPIProvider{
                type Target = ::std::sync::Mutex<Lua>;
                type DocTarget = crate::LuaDocFragment;
    
                fn attach_api(&mut self, c: &mut <Self as crate::APIProvider>::Target) -> Result<(),crate::ScriptError> {
                    let lua_ctx = c.lock().expect("Could not get lock on script context");
    
                    tealr::mlu::set_global_env::<BevyAPIGlobals>(&lua_ctx)?;
    
                    Ok(())
                }
    
                fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
                    Some(crate::LuaDocFragment::new(|tw|
                                tw.document_global_instance::<BevyAPIGlobals>().unwrap()
                                #(
                                    .process_type::<#userdata_newtype_names>()
                                )*
                                #(
                                    .process_type::<#global_proxies>()  
                                )*
                                #(
                                    .process_type::<#external_types>()
                                )*
                            )
                        )
                }
            }
        };
    
    
        let asserts : proc_macro2::TokenStream = new_types.new_types.iter().map(|x| {
            let ident = &x.args.base_type.path.segments.last().unwrap().ident;
            let mut full_key = x.args.base_type.to_token_stream().to_string();
            full_key.retain(|c| !c.is_whitespace());
    
            quote_spanned!{x.span()=>
                assert_eq!(std::any::type_name::<#ident>(),#full_key);
            }
        }).collect();
    
        let custom_tests : Punctuated<proc_macro2::TokenStream,EmptyToken> = all_functions.iter()
            .flat_map(|(n,v)| v.iter().filter_map(|v| v.gen_tests(n)))
            .collect();
    
        let imports : Punctuated<proc_macro2::TokenStream,EmptyToken> = new_types.new_types.iter()
            .filter(|v| &v.args.base_type.path.segments.first().unwrap().ident.to_string() == "bevy")
            .map(|v| {
                let p = &v.args.base_type;
                quote_spanned!(v.span()=> use #p;)
            }).collect();
    
        let tests = quote_spanned!{new_types.span()=>
            #[cfg(test)]
            mod gen_test {
                use bevy::prelude::*;
                use bevy::math::*;
    
                #imports
                #[test]
                pub fn test_wrapper_keys(){
                    #asserts
                }
    
                #custom_tests
            }
        };    

        let additional_globals = &self.additional_globals;

        Ok(quote_spanned!{new_types.span()=>
            #imports
            #api_provider
            #lookup_tables
            #tests
            #additional_globals
        })
    }
    


}