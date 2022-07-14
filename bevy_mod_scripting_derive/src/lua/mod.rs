pub(crate) mod lua_method;
pub(crate) mod derive_flags;



pub(crate) use {lua_method::*, derive_flags::*};


use indexmap::{IndexMap, IndexSet};
use proc_macro2::{TokenStream, Span, Ident};
use syn::{spanned::Spanned, parse_quote_spanned, punctuated::Punctuated, LitInt, Token, Attribute, parse_quote, Type};

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

    fn module_name() -> &'static str {
        "lua"
    }

    fn generate_newtype_definition(&mut self, newtype : &crate::common::newtype::Newtype) -> std::result::Result<TokenStream, syn::Error> {
        let name = &newtype.args.wrapper_type;
        let base_type = &newtype.args.base_type_ident;

        Ok(match &newtype.args.variation {
            NewtypeVariation::Value{..} => quote_spanned!{newtype.span()=>
                pub type #name = crate::LuaWrapper<#base_type>;
            },
            NewtypeVariation::Ref {..} => quote_spanned!{newtype.span()=>
                // TODO these don't support clone so gotta do a special FromLua impl 
                pub type #name = crate::LuaWrapper<#base_type>;
            },
            NewtypeVariation::Primitive{..} => quote_spanned!{newtype.span()=>},
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
        
        let (fields,methods) = functions
            .filter(|f| !f.method_type.is_static)
            .partition::<Vec<_>,_>(|f| f.method_type.is_field());

        let methods = methods
            .iter()
            .map(|f| f.to_call_expr("methods"));

        let fields = fields
            .iter()
            .map(|f| f.to_call_expr("fields"));

        // expose to lua
        let user_data_implementation = quote_spanned!{newtype.span()=>
            impl tealr::mlu::TealData for #name {
                fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                    #type_documentator
                    #(#methods)*
                }

                fn add_fields<'lua, T: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
                    #(#fields)*
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
                flag @ DeriveFlag::AutoMethods {..} => {
                    make_auto_methods(flag,new_type,&mut out);
                },
                flag @ DeriveFlag::Copy {..} =>{ 
                    make_copy(flag, new_type, &mut out, &functions_so_far)?;
                },
                flag @ DeriveFlag::BinOps {..} =>  {  
                    make_bin_ops(self, flag, new_type, &mut out)?;
                },
                flag @ DeriveFlag::UnaryOps {..} => {
                    make_unary_ops(flag, new_type, &mut out)?;
                },
                flag @ DeriveFlag::Fields {..} => {
                    make_fields(flag,new_type,&mut out)?;
                },

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
            .filter_map(|base| {
                let key = stringify_type_path(&base.args.base_type);
                let wrapper_type = &base.args.wrapper_type;

                let value = 
                    if base.args.variation.is_value(){
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

                Some(quote_spanned!{base.span()=>#key => #value})
            }).collect();

        let to_lua : Punctuated<TokenStream,Token![,]> = new_types.new_types
            .iter()
            .filter_map(|base| {
                let key = stringify_type_path(&base.args.base_type);
                let wrapper_type = &base.args.wrapper_type;

                let value = 
                    if base.args.variation.is_value(){
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
                for<'l> fn(&crate::ScriptRef,&'l Lua) -> tealr::mlu::mlua::Value<'l>
                > = phf_map!{
                    #to_lua,
                };

            pub static APPLY_LUA_TO_BEVY: Map<&'static str,
                for<'l> fn(&mut crate::ScriptRef,&'l Lua, tealr::mlu::mlua::Value<'l>) -> Result<(),tealr::mlu::mlua::Error>
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