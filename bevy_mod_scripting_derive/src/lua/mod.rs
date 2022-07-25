pub(crate) mod lua_method;
pub(crate) mod derive_flags;



pub(crate) use {lua_method::*, derive_flags::*};


use indexmap::{IndexMap, IndexSet};
use proc_macro2::{TokenStream, Span, Ident};
use syn::{spanned::Spanned, parse_quote_spanned, punctuated::Punctuated, LitInt, Token, Attribute, parse_quote, Type};

use crate::{common::{WrapperImplementor, WrapperFunction, attribute_to_string_lit, derive_flag::DeriveFlag, ops::{OpName,OpExpr}, stringify_type_path, type_base_string}, EmptyToken};
use quote::{quote, quote_spanned, ToTokens, format_ident};

impl WrapperFunction for LuaMethod {}

#[derive(Default)]
pub(crate) struct LuaImplementor{
    implemented_unions : IndexSet<Ident>,
    additional_globals : TokenStream
}

impl LuaImplementor {

    /// Generates a union registers it, and makes sure no two identical unions exist, while removing duplicate entries in the enum
    fn generate_register_union(&mut self, type_idents : &Vec<String>, unique_ident : &str) -> Ident{

        let unique_idents : Vec<String> = type_idents.iter().cloned().collect::<IndexSet<_>>().into_iter().collect::<Vec<_>>();

        let return_arg_type = format_ident!("{unique_ident}Union{}",unique_idents.join(""));

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

        Ok(quote_spanned!{newtype.span()=>
                pub type #name = crate::LuaWrapper<#base_type>;
        })
    }

    fn generate_newtype_implementation<'a,I : Iterator<Item=&'a Self::Function>>(&mut self, newtype: &'a crate::common::newtype::Newtype, functions : I) -> std::result::Result<TokenStream, syn::Error> {

        let wrapper_type = &newtype.args.wrapper_type;
        let wrapped_type = &newtype.args.base_type_ident;

        // provide documentation generation implementations
        let tealr_implementations = quote_spanned!{newtype.span()=>
            impl_tealr_type!(#wrapper_type);
        };

        // generate documentation calls on type level
        let type_documentator : TokenStream = newtype.args.docstring.iter()
                                                .map(attribute_to_string_lit)
                                                .map(|ts| quote_spanned!{ts.span()=>
                                                    methods.document_type(#ts);
                                                }).collect();
        
        let (fields,methods) = functions
            .partition::<Vec<_>,_>(|f| f.method_type.is_field());

        let methods = methods
            .iter()
            .map(|f| f.to_call_expr("methods"));

        let fields = fields
            .iter()
            .map(|f| f.to_call_expr("fields"));

        // expose to lua
        let user_data_implementation = quote_spanned!{newtype.span()=>
            #[allow(unused_parens,unreachable_patterns,unused_variables)]
            impl tealr::mlu::TealData for #wrapper_type {
                fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                    #type_documentator
                    #(#methods)*
                }

                fn add_fields<'lua, T: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
                    #(#fields)*
                }
            }

            impl LuaProxyable for #wrapped_type {
                fn ref_to_lua<'lua>(self_ : ScriptRef, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
                    #wrapper_type::new_ref(self_).to_lua(lua)
                }

                fn apply_lua<'lua>(self_ : &mut ScriptRef, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()> {
                    if let Value::UserData(v) = new_val {
                        let other = v.borrow::<#wrapper_type>()?;
                        let other = &other;

                        other.apply_self_to_base(self_)?;
                        Ok(()) 
                    } else {
                        Err(mlua::Error::RuntimeError(
                            "Error in assigning to custom user data".to_owned(),
                        ))
                    }
                }
            }
        };


        let additional_globals = &self.additional_globals;

        // group everything together
        Ok(quote_spanned!{newtype.span()=>
            #user_data_implementation
            #tealr_implementations
            #additional_globals
        })
    }



    fn generate_derive_flag_functions<'a, I : Iterator<Item=&'a crate::common::derive_flag::DeriveFlag>>(&mut self, new_type : &'a crate::common::newtype::Newtype, mut derive_flags : I) -> Result<Vec<LuaMethod>, syn::Error> {
        
        let mut out : Vec<Self::Function> = Default::default();
        let wrapper_type = &new_type.args.wrapper_type;
        let wrapped_type = &new_type.args.base_type_ident;

        derive_flags.try_for_each(|v| {
            Ok::<(),syn::Error>(match v {
                DeriveFlag::DebugToString{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s))
                }),
                DeriveFlag::DisplayToString{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s))
                }),
                DeriveFlag::FromLuaProxy{ident} => {
                    self.additional_globals.extend(
                        quote_spanned!{ident.span()=>
                            impl FromLuaProxy<'_> for #wrapped_type {
                                fn from_lua_proxy<'lua>(lua_value: Value<'lua>, _: &'lua Lua) -> mlua::Result<Self> {
                                    if let mlua::Value::UserData(ud) = lua_value{
                                        let wrapper = ud.borrow::<#wrapper_type>()?;
                                        Ok(wrapper.deref().inner()?)
                                    } else {
                                        Err(mlua::Error::FromLuaConversionError{
                                            from: "",
                                            to: "",
                                            message: None
                                        })
                                    }
                                }
                            }
                        }
                    );

                }
                flag @ DeriveFlag::AutoMethods {..} => {
                    make_auto_methods(flag,new_type,&mut out);
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

        Ok(new_type.additional_lua_functions
            .as_ref()
            .map(|v| v.functions.iter().cloned().collect())
            .unwrap_or_default())    
    }

}