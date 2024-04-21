use std::collections::HashMap;

use bevy_mod_scripting_common::{input::*, utils::doc_attribute_to_string_lit};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, parse_quote_spanned, DeriveInput, ExprClosure, FnArg, Variant};
use syn::{
    parse_quote, spanned::Spanned, AttrStyle, Attribute, Field, Meta, Path, Token, TraitItemFn,
};

use darling::{FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro2::*;
use quote::*;
use vec1::{vec1, Vec1};

const SELF_ALIAS: &str = "_self";
const CTXT_ALIAS: &str = "lua";
const RAW_OUT_ALIAS: &str = "__proxied_out";
const PROXY_OUT_ALIAS: &str = "__proxy_out";
const PROXY_PREFIX: &str = "Lua";
const VALID_META_METHODS: [&str; 27] = [
    "Add", "Sub", "Mul", "Div", "Mod", "Pow", "Unm", "IDiv", "BAnd", "BOr", "BXor", "BNot", "Shl",
    "Shr", "Concat", "Len", "Eq", "Lt", "Le", "Index", "NewIndex", "Call", "ToString", "Pairs",
    "IPairs", "Iter", "Close",
];

/// Convert receiver to a standardised form, for example:
/// - instead o a `&self` receiver we have a `_self: LuaRefProxy<Self>`
/// - instead of a `&mut self` receiver we have a `_self: LuaRefMutProxy<Self>`
/// - instead of a `self` receiver we have a `_self: ValLuaProxy<Self>`
fn standardise_receiver<'a>(receiver: &mut FnArg, target_type_ident: &Ident, bms_core_path: &Path) {
    let replacement = if let FnArg::Receiver(receiver) = receiver {
        let ref_ = &receiver.reference.as_ref().map(|(amp, lifetime)| {
            quote_spanned! {receiver.span()=>
                #amp #lifetime
            }
        });

        let self_ident = syn::Ident::new(SELF_ALIAS, receiver.span());
        let unproxy_container_name = match (ref_.is_some(), receiver.mutability.is_some()) {
            (true, true) => "LuaReflectRefMutProxy",
            (true, false) => "LuaReflectRefProxy",
            (false, _) => "LuaReflectValProxy",
        };

        Some(syn::FnArg::Typed(parse_quote_spanned! {receiver.span()=>
            #self_ident: #bms_core_path::proxy::#unproxy_container_name<#target_type_ident>
        }))
    } else {
        None
    };
    if let Some(replacement) = replacement {
        *receiver = replacement;
    }
}

/// Collect all arguments into a tuple, for example:
/// - `fn foo(a: i32, b: f32)` becomes `(name: (i32, f32))`
fn collect_args_in_tuple<'a, I: Iterator<Item = &'a FnArg>>(
    args: I,
    name: &Ident,
    outer_mut: bool,
) -> FnArg {
    let (arg_names, arg_types) = args
        .map(|arg| {
            if let FnArg::Typed(arg) = arg {
                (arg.pat.clone(), arg.ty.clone())
            } else {
                panic!("Function arguments must be typed")
            }
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();

    let outer_mut = if outer_mut {
        Some(Token![mut](name.span()))
    } else {
        None
    };

    parse_quote!( #outer_mut #name : (#(#arg_types),*) )
}

/// Convert a function definition to a closure, for example:
/// - `fn foo(a: i32, b: f32) -> f32 { a + b }` becomes `|a: i32, b: f32| { a + b} `
fn convert_function_def_to_closure(f: TraitItemFn) -> ExprClosure {
    let span = f.span();
    let sig = f.sig.inputs;
    let body = f
        .default
        .unwrap_or_else(|| panic!("Function {} must have a body", f.sig.ident));
    parse_quote_spanned! {span =>
        |#sig| #body
    }
}

#[proc_macro_derive(LuaProxy, attributes(lua, proxy))]
pub fn impl_lua_proxy(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let meta: ProxyInput = match ProxyInput::from_derive_input(&derive_input) {
        Ok(v) => v,
        Err(e) => return darling::Error::write_errors(e).into(),
    };
    if meta.proxy_name.is_some() {
        // throw error
        return syn::Error::new(
            derive_input.span(),
            "The `name` attribute is not supported for lua proxies",
        )
        .to_compile_error()
        .into();
    }

    let target_type_ident = &meta.ident;
    let target_type_path: syn::Path = meta.remote.unwrap_or(meta.ident.clone().into());
    let target_type_str = target_type_path.segments.last().unwrap().ident.to_string();
    let proxy_type_ident = meta.proxy_name.unwrap_or_else(|| {
        format_ident!("{PROXY_PREFIX}{}", &meta.ident, span = meta.ident.span())
    });

    let bms_core = meta.bms_core_path.0;
    let bms_lua = meta.bms_lua_path.0;
    let tealr = quote_spanned!(bms_lua.span()=>
        #bms_lua::tealr
    );
    let mlua = quote_spanned!(bms_core.span()=>
        #tealr::mlu::mlua
    );

    // generate type level tealr documentation calls
    let type_level_document_calls = meta
        .attrs
        .iter()
        .filter(|&a| a.meta.path().is_ident("doc"))
        .map(doc_attribute_to_string_lit)
        .map(|tkns| {
            quote_spanned!(meta.ident.span()=>
                    methods.document_type(#tkns);
            )
        });

    let add_function_stmts = meta.functions.0.into_iter().map(|mut f| {
        if let Some(first_arg) = f.sig.inputs.first_mut() {
            standardise_receiver(first_arg, target_type_ident, &bms_core)
        };
        // collect all args into tuple and add lua context arg
        let ctxt_alias = syn::Ident::new(CTXT_ALIAS, f.sig.inputs.span());
        let ctxt_arg = parse_quote_spanned! {f.span()=>
            #ctxt_alias: &#mlua::Lua
        };

        let func_name = &f.sig.ident;
        let (original_arg_idents, original_arg_types) = f
            .sig
            .inputs
            .iter()
            .map(|arg| {
                if let FnArg::Typed(arg) = arg {
                    (arg.pat.clone(), arg.ty.clone())
                } else {
                    panic!("Function arguments must be typed")
                }
            })
            .unzip::<_, _, Vec<_>, Vec<_>>();

        let span = f.span();
        let args_ident = format_ident!("args", span = f.sig.inputs.span());
        f.sig.inputs = Punctuated::from_iter(vec![
            ctxt_arg,
            collect_args_in_tuple(f.sig.inputs.iter(), &args_ident, true),
        ]);

        let out_type = match &f.sig.output {
            syn::ReturnType::Default => quote_spanned! {f.span()=>
                ()
            },
            syn::ReturnType::Type(_, ty) => ty.to_token_stream(),
        };

        // wrap function body in our unwrapping and wrapping logic, ignore pre-existing body
        f.default = Some(parse_quote_spanned! {span=>
            {
                let mut world: #bms_lua::bindings::proxy::LuaValProxy<#bms_core::bindings::WorldCallbackAccess> = lua.globals().get("world")?;
                let mut world = <#bms_lua::bindings::proxy::LuaValProxy<#bms_core::bindings::WorldCallbackAccess> as #bms_core::proxy::Unproxy>::unproxy(&mut world).map_err(#mlua::Error::external)?;
                let mut world = world.read().ok_or_else(|| #mlua::Error::external("World no longer exists"))?;

                // get allocator and type registry

                let cell = world.as_unsafe_world_cell();
                let allocator_resource_id = cell.components().resource_id::<#bms_core::allocator::ReflectAllocator>().expect("Reflect Allocator wasn't initialized");
                let type_registry_resource_id = cell.components().resource_id::<bevy::ecs::reflect::AppTypeRegistry>().expect("Type Registry wasn't initialized");

                let mut allocator_access = world.get_access(allocator_resource_id.into()).expect("Deadlock while accessing allocator");
                let type_registry_access = world.get_access(type_registry_resource_id.into()).expect("Deadlock while accessing type registry");

                let mut allocator = world.get_resource_mut::<#bms_core::allocator::ReflectAllocator>(&mut allocator_access).unwrap().unwrap();
                let type_registry = world.get_resource::<bevy::ecs::reflect::AppTypeRegistry>(&type_registry_access).unwrap().unwrap();
                let type_registry = type_registry.read();
                let mut world_acceses = Vec::default();
                // Safety: we pinky promise not to touch world_accessses after this point
                let (#( #original_arg_idents ),*) = unsafe { <(#(#original_arg_types),*) as #bms_core::proxy::Unproxy>::unproxy_with_world(&mut #args_ident, &world, &mut world_acceses, &type_registry, &allocator).map_err(#mlua::Error::external)? };
                
                // call proxy function 
                let out = #target_type_ident::#func_name(#(#original_arg_idents),*);
                let out = <#out_type as #bms_core::proxy::Proxy>::proxy_with_allocator(out, &mut allocator).map_err(#mlua::Error::external)?;

                // TODO: output proxies
                Ok(out)
            }
        });

        let name = f.sig.ident.to_string();
        let closure = convert_function_def_to_closure(f);
        quote_spanned! {span=>
            methods.add_function(#name, #closure);
        }
    });

    let vis = &meta.vis;
    quote_spanned! {meta.ident.span()=>

        #vis struct #proxy_type_ident (pub #bms_core::bindings::ReflectReference);

        impl #tealr::mlu::TealData for #proxy_type_ident {
            fn add_methods<'lua, M: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut M) {
                #(#type_level_document_calls)*
                #(#add_function_stmts)*
            }
        }

        impl #tealr::ToTypename for #proxy_type_ident {
            fn to_typename() -> #tealr::Type {
                #tealr::Type::Single(#tealr::SingleType {
                    name: #tealr::Name(#target_type_str.into()),
                    kind: #tealr::KindOfType::External,
                })
            }
        }


    }
    .into()
}

// test cases TODO:
// - pub/private wrapper being generated correctly
// - proxy name being generated correctly with custom name
// - proxy name being generated correctly with default name
