use indexmap::IndexSet;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Token};

use bevy_mod_scripting_common::{
    derive_flag::DeriveFlag,
    implementor::{WrapperFunction, WrapperImplementor},
    newtype::Newtype,
    utils::attribute_to_string_lit,
};
use quote::{format_ident, quote, quote_spanned};

use crate::{
    derive_flags::{make_bin_ops, make_fields, make_methods, make_unary_ops},
    lua_method::LuaMethod,
};

impl WrapperFunction for LuaMethod {}

#[derive(Default)]
pub(crate) struct LuaImplementor {
    implemented_unions: IndexSet<Ident>,
    additional_globals: TokenStream,
}

impl LuaImplementor {
    /// Generates a union registers it, and makes sure no two identical unions exist, while removing duplicate entries in the enum
    ///
    /// The given unique type idents must be unique (i.e. come from a HashSet)
    pub(crate) fn generate_register_union<I: Iterator<Item = String>>(
        &mut self,
        unique_type_idents: I,
        unique_ident: &str,
    ) -> Ident {
        let unique_idents: Vec<String> = unique_type_idents.collect::<Vec<_>>();

        let return_arg_type = format_ident!("{unique_ident}Union{}", unique_idents.join(""));

        if !self.implemented_unions.contains(&return_arg_type) {
            self.implemented_unions.insert(return_arg_type.clone());
            let return_arg = unique_idents
                .iter()
                .map(|v| format_ident!("{v}"))
                .collect::<Punctuated<Ident, Token![|]>>();

            self.additional_globals.extend(quote! {
                bevy_script_api::impl_tealr_any_union!(pub enum #return_arg_type = #return_arg);
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

    fn generate_newtype_definition(
        &mut self,
        newtype: &Newtype,
    ) -> std::result::Result<TokenStream, syn::Error> {
        let newtype_name = &newtype.args.wrapper_type;
        let base_type = &newtype.args.base_type_ident;

        let mut definition = Default::default();
        if newtype.args.flags.contains(&DeriveFlag::Clone {
            ident: Ident::new("Clone", Span::call_site()),
        }) {
            definition = quote_spanned! {newtype.span()=>
                #definition
                bevy_script_api::make_script_wrapper!(#base_type as #newtype_name with Clone);
            };
        } else {
            definition = quote_spanned! {newtype.span()=>
                #definition
                bevy_script_api::make_script_wrapper!(#base_type as #newtype_name);
            };
        }

        if newtype.args.flags.contains(&DeriveFlag::Debug {
            ident: Ident::new("Debug", Span::call_site()),
        }) {
            definition = quote_spanned! {newtype.span()=>
                #definition
                impl std::fmt::Debug for #newtype_name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                        self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))                    }
                }
            }
        }

        Ok(definition)
    }

    fn generate_newtype_implementation<'a, I: Iterator<Item = &'a Self::Function>>(
        &mut self,
        newtype: &'a Newtype,
        functions: I,
    ) -> std::result::Result<TokenStream, syn::Error> {
        let wrapper_type = &newtype.args.wrapper_type;
        let wrapped_type = &newtype.args.base_type_ident;
        let tealr = quote::quote!(bevy_mod_scripting_lua::tealr);

        // provide documentation generation implementations
        let tealr_implementations = quote_spanned! {newtype.span()=>
            bevy_script_api::impl_tealr_type!(#wrapper_type);
        };

        // generate documentation calls on type level
        let type_documentator: TokenStream = newtype
            .args
            .docstring
            .iter()
            .map(attribute_to_string_lit)
            .map(|ts| {
                quote_spanned! {ts.span()=>
                    methods.document_type(#ts);
                }
            })
            .collect();
        let (fields, methods) = functions.partition::<Vec<_>, _>(|f| f.method_type.is_field());

        let methods = methods.iter().map(|f| f.to_call_expr("methods"));

        let fields = fields.iter().map(|f| f.to_call_expr("fields"));

        // expose to lua
        let user_data_implementation = quote_spanned! {newtype.span()=>
            #[allow(unused_parens,unreachable_patterns,unused_variables,clippy::all)]
            impl #tealr::mlu::TealData for #wrapper_type {
                fn add_methods<'lua, T: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                    #type_documentator
                    #(#methods)*
                }

                fn add_fields<'lua, T: #tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
                    #(#fields)*
                }
            }

            impl bevy_script_api::lua::LuaProxyable for #wrapped_type {
                fn ref_to_lua<'lua>(self_ : bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>> {
                    <#wrapper_type as #tealr::mlu::mlua::IntoLua>::into_lua(#wrapper_type::new_ref(self_),lua)
                }

                fn apply_lua<'lua>(self_ : &mut bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua, new_val: #tealr::mlu::mlua::Value<'lua>) -> #tealr::mlu::mlua::Result<()> {
                    if let #tealr::mlu::mlua::Value::UserData(v) = new_val {
                        let other = v.borrow::<#wrapper_type>()?;
                        let other = &other;

                        other.apply_self_to_base(self_)?;
                        Ok(())
                    } else {
                        Err(#tealr::mlu::mlua::Error::RuntimeError(
                            "Error in assigning to custom user data".to_owned(),
                        ))
                    }
                }
            }

            impl bevy_script_api::lua::ToLuaProxy<'_> for #wrapped_type {
                fn to_lua_proxy<'lua>(self, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>>{
                    <#wrapper_type as #tealr::mlu::mlua::IntoLua>::into_lua(#wrapper_type::new(self),lua)
                }
            }
        };

        let additional_globals = &self.additional_globals;

        // group everything together
        Ok(quote_spanned! {newtype.span()=>
            #user_data_implementation
            #tealr_implementations
            #additional_globals
        })
    }

    fn generate_derive_flag_functions<'a, I: Iterator<Item = &'a DeriveFlag>>(
        &mut self,
        new_type: &'a Newtype,
        mut derive_flags: I,
    ) -> Result<Vec<LuaMethod>, syn::Error> {
        let mut out: Vec<Self::Function> = Default::default();
        let wrapper_type = &new_type.args.wrapper_type;
        let wrapped_type = &new_type.args.base_type_ident;
        let tealr = quote::quote!(bevy_mod_scripting_lua::tealr);

        derive_flags.try_for_each(|v| {
            match v {
                DeriveFlag::Debug{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (#tealr::mlu::mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s))
                }),
                DeriveFlag::Display{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (#tealr::mlu::mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s))
                }),
                DeriveFlag::Clone{ident} => {
                    self.additional_globals.extend(
                        quote_spanned!{ident.span()=>
                            impl bevy_script_api::lua::FromLuaProxy<'_> for #wrapped_type {
                                fn from_lua_proxy<'lua>(lua_value: #tealr::mlu::mlua::Value<'lua>, _: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<Self> {
                                    if let #tealr::mlu::mlua::Value::UserData(ud) = lua_value{
                                        let wrapper = ud.borrow::<#wrapper_type>()?;
                                        Ok(std::ops::Deref::deref(&wrapper).inner()?)
                                    } else {
                                        Err(#tealr::mlu::mlua::Error::FromLuaConversionError{
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
                flag @ DeriveFlag::Methods {..} => {
                    make_methods(flag,new_type,&mut out);
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
            };
            Ok::<(),syn::Error>(())
        })?;

        Ok(out)
    }

    fn generate_newtype_functions(
        &mut self,
        new_type: &Newtype,
    ) -> Result<Vec<LuaMethod>, syn::Error> {
        let lua_block = new_type
            .impl_blocks
            .iter()
            .find(|block| block.label == "lua");

        if let Some(block) = lua_block {
            let functions = &block.functions;
            let tokens: Punctuated<LuaMethod, Token![;]> =
                parse_quote_spanned!(block.span()=>#functions);

            Ok(tokens.into_iter().collect())
        } else {
            Ok(Default::default())
        }
    }
}
