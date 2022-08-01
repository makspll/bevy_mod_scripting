pub(crate) mod derive_flags;
pub(crate) mod lua_method;

pub(crate) use {derive_flags::*, lua_method::*};

use indexmap::IndexSet;
use proc_macro2::{Ident, Span, TokenStream};
use syn::{parse_quote_spanned, punctuated::Punctuated, spanned::Spanned, Token};

use crate::common::{
    attribute_to_string_lit, derive_flag::DeriveFlag, WrapperFunction, WrapperImplementor,
};
use quote::{format_ident, quote, quote_spanned};

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
    fn generate_register_union<'a, I: Iterator<Item = String>>(
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
                impl_tealr_any_union!(pub enum #return_arg_type = #return_arg);
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
        newtype: &crate::common::newtype::Newtype,
    ) -> std::result::Result<TokenStream, syn::Error> {
        let newtype_name = &newtype.args.wrapper_type;
        let base_type = &newtype.args.base_type_ident;

        let mut definition = Default::default();
        if newtype.args.flags.contains(&DeriveFlag::Clone {
            ident: Ident::new("Clone", Span::call_site()),
        }) {
            definition = quote_spanned! {newtype.span()=>
                #definition
                bevy_mod_scripting::make_script_wrapper!(#base_type as #newtype_name with Clone);
            };
        } else {
            definition = quote_spanned! {newtype.span()=>
                #definition
                bevy_mod_scripting::make_script_wrapper!(#base_type as #newtype_name);
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
        newtype: &'a crate::common::newtype::Newtype,
        functions: I,
    ) -> std::result::Result<TokenStream, syn::Error> {
        let wrapper_type = &newtype.args.wrapper_type;
        let wrapped_type = &newtype.args.base_type_ident;

        // provide documentation generation implementations
        let tealr_implementations = quote_spanned! {newtype.span()=>
            bevy_mod_scripting::impl_tealr_type!(#wrapper_type);
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

            impl bevy_mod_scripting::LuaProxyable for #wrapped_type {
                fn ref_to_lua<'lua>(self_ : bevy_mod_scripting::ScriptRef, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
                    <#wrapper_type as mlua::ToLua>::to_lua(#wrapper_type::new_ref(self_),lua)
                }

                fn apply_lua<'lua>(self_ : &mut bevy_mod_scripting::ScriptRef, lua: &'lua mlua::Lua, new_val: mlua::Value<'lua>) -> mlua::Result<()> {
                    if let mlua::Value::UserData(v) = new_val {
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
        Ok(quote_spanned! {newtype.span()=>
            #user_data_implementation
            #tealr_implementations
            #additional_globals
        })
    }

    fn generate_derive_flag_functions<
        'a,
        I: Iterator<Item = &'a crate::common::derive_flag::DeriveFlag>,
    >(
        &mut self,
        new_type: &'a crate::common::newtype::Newtype,
        mut derive_flags: I,
    ) -> Result<Vec<LuaMethod>, syn::Error> {
        let mut out: Vec<Self::Function> = Default::default();
        let wrapper_type = &new_type.args.wrapper_type;
        let wrapped_type = &new_type.args.base_type_ident;

        derive_flags.try_for_each(|v| {
            Ok::<(),syn::Error>(match v {
                DeriveFlag::Debug{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{:?}",s))
                }),
                DeriveFlag::Display{ident} => out.push(parse_quote_spanned!{ident.span()=>
                    (mlua::MetaMethod::ToString) => |_,s,()| Ok(format!("{}",s))
                }),
                DeriveFlag::Clone{ident} => {
                    self.additional_globals.extend(
                        quote_spanned!{ident.span()=>
                            impl bevy_mod_scripting::FromLuaProxy<'_> for #wrapped_type {
                                fn from_lua_proxy<'lua>(lua_value: mlua::Value<'lua>, _: &'lua mlua::Lua) -> mlua::Result<Self> {
                                    if let mlua::Value::UserData(ud) = lua_value{
                                        let wrapper = ud.borrow::<#wrapper_type>()?;
                                        Ok(std::ops::Deref::deref(&wrapper).inner()?)
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

            })
        })?;

        Ok(out)
    }

    fn generate_newtype_functions(
        &mut self,
        new_type: &crate::common::newtype::Newtype,
    ) -> Result<Vec<LuaMethod>, syn::Error> {
        Ok(new_type
            .additional_lua_functions
            .as_ref()
            .map(|v| v.functions.iter().cloned().collect())
            .unwrap_or_default())
    }
}
