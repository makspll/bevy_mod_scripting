mod input; 
mod utils;

use crate::{input::*, utils::doc_attribute_to_string_lit};

use std::collections::HashMap;
use darling::util::Flag;
use syn::{
    spanned::Spanned, Path, Token, TraitItemFn, parse_quote,
    parse_macro_input, parse_quote_spanned, DeriveInput, ExprClosure, FnArg,
    punctuated::Punctuated
};

use darling::{FromAttributes, FromDeriveInput};
use proc_macro::TokenStream;
use proc_macro2::*;
use quote::*;


const SELF_ALIAS: &str = "_self";
const CTXT_ALIAS: &str = "lua";
const PROXY_PREFIX: &str = "Lua";

/// Convert receiver to a standardised form, for example:
/// - instead o a `&self` receiver we have a `_self: LuaRefProxy<Self>`
/// - instead of a `&mut self` receiver we have a `_self: LuaRefMutProxy<Self>`
/// - instead of a `self` receiver we have a `_self: ValLuaProxy<Self>`
fn standardise_receiver(receiver: &mut FnArg, target_type: &Path, bms_lua_path: &Path) {
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
        let unproxy_ident = syn::Ident::new(unproxy_container_name, receiver.span());

        Some(syn::FnArg::Typed(parse_quote_spanned! {receiver.span()=>
            #self_ident: #bms_lua_path::bindings::proxy::#unproxy_ident::<#target_type>
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
    let (_, arg_types) = args
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
fn convert_function_def_to_closure(f: &TraitItemFn) -> ExprClosure {
    let span = f.span();
    let sig = &f.sig.inputs;
    let body = f
        .default
        .as_ref()
        .unwrap_or_else(|| panic!("Function {} must have a body", f.sig.ident));
    parse_quote_spanned! {span =>
        |#sig| #body
    }
}

/// Processes the function def to wrap it in the necessary proxying logic
/// Will convert the function signature to take in two arguments:
/// - a context argument
/// - a tuple of all arguments passed to the underlying function
fn proxy_wrap_function_def(
    f: &mut TraitItemFn,
    target_type: &Path,
    bms_core: &Path,
    bms_lua: &Path,
    mlua: &Path,
    attrs: &FunctionAttrs,
) {
    if let Some(first_arg) = f.sig.inputs.first_mut() {
        standardise_receiver(first_arg, target_type, bms_lua)
    };

    // collect all args into tuple and add lua context arg
    let ctxt_alias = syn::Ident::new(CTXT_ALIAS, f.sig.inputs.span());

    let ctxt_arg = if attrs.with_context.is_present() {
        f.sig
            .inputs
            .pop()
            .expect("Expected at least one argument for the context")
            .into_value()
    } else {
        parse_quote_spanned! {f.span()=>
            #ctxt_alias: &#mlua::Lua
        }
    };
    let ctxt_arg_ident = match &ctxt_arg {
        FnArg::Typed(arg) => arg.pat.clone(),
        _ => panic!("Expected a typed argument, not a receiver for the context argument"),
    };

    let func_name = &f.sig.ident;
    let (original_arg_idents, _) = f
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

    // change signature to take in a single args tuple instead of multiple arguments (on top of a context arg)
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
    let mut fn_call = std::panic::catch_unwind(|| {
        match (&f.default, &attrs.as_trait) {
            (Some(body), _) => quote_spanned!(span=>
                (||{ #body })()
            ),
            (_, None) => quote_spanned!(span=>
                #target_type::#func_name(#(#original_arg_idents),*)
            ),
            (_, Some(trait_path)) => {
                let trait_path = quote_spanned!(span=> #trait_path);
                quote_spanned!(span=>
                    <#target_type as #trait_path>::#func_name(#(#original_arg_idents),*)
                )
            }
        }
    })
    .unwrap(); // todo: handle the error nicer

    if f.sig.unsafety.is_some() {
        fn_call = quote_spanned!(span=>
            unsafe { #fn_call }
        );
    }

    if attrs.no_proxy.is_present() {
        f.default = Some(parse_quote_spanned! {span=>
            {
                #fn_call
            }
        });
    } else {
        f.default = Some(parse_quote_spanned! {span=>
            {
                let mut world: #bms_lua::bindings::proxy::LuaValProxy<#bms_core::bindings::WorldCallbackAccess> = #ctxt_arg_ident.globals().get("world")?;
                let mut world = <#bms_lua::bindings::proxy::LuaValProxy<#bms_core::bindings::WorldCallbackAccess> as #bms_core::bindings::Unproxy>::unproxy(&mut world).map_err(#mlua::Error::external)?;
                let mut world = world.read().ok_or_else(|| #mlua::Error::external("World no longer exists"))?;
                let out: #out_type = world.proxy_call(#args_ident, |(#(#original_arg_idents),*)| {
                    #fn_call
                }).map_err(|e| #mlua::Error::external(e))?;
                Ok(out)
            }
        });
    }
}

fn generate_methods_registration(
    attrs: &FunctionAttrs,
    span: Span,
    name: proc_macro2::TokenStream,
    closure: ExprClosure,
) -> proc_macro2::TokenStream {
    let registration_method = if attrs.metamethod.is_some() {
        quote_spanned!(span=>add_meta_function)
    } else {
        quote_spanned!(span=>add_function)
    };
    let docs = attrs.doc.iter().map(|doc| {
        quote_spanned! {span=>
            methods.document(#doc);
        }
    });
    quote_spanned! {span=>
        #(#docs)*
        methods.#registration_method(#name, #closure);
    }
}

#[derive(FromAttributes, Clone)]
#[darling(attributes(lua))]
struct FunctionAttrs {
    #[darling(multiple)]
    pub doc: Vec<String>,

    /// Marks the function as a composite with the given ID, at least one another function with the same composite
    /// ID must exist resulting in a combined function being generated. The actual function to dispatch to will be decided based on
    /// the types of arguments. If the signature is invalid (i.e. doesn't allow us to dispatch) an error will be thrown
    #[darling(default)]
    pub composite: Option<String>,

    /// Marks this to be ignored, only used for fields as functions are opt-in
    pub skip: Flag,

    /// If passed will generate <T as Trait> statement before calling the method
    /// on the type
    pub as_trait: Option<Path>,

    /// If passed will generate a metamethod call instead of using the function name
    pub metamethod: Option<Ident>,

    /// If true will pass in the context as the last argument,
    /// i.e. will remove that argument from the function signature and use it's name as the context alias
    pub with_context: Flag,

    /// Skips the unproxying & proxying call, useful for functions that don't need to access the world
    pub no_proxy: Flag,
}

#[proc_macro_derive(LuaProxy, attributes(lua, proxy))]
pub fn impl_lua_proxy(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let mut meta: ProxyInput = match ProxyInput::from_derive_input(&derive_input) {
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

    let target_type = meta.remote.unwrap_or(meta.ident.clone().into());
    let target_type_str = target_type.segments.last().unwrap().ident.to_string();
    let proxy_type_ident = meta.proxy_name.unwrap_or_else(|| {
        format_ident!("{PROXY_PREFIX}{}", &meta.ident, span = meta.ident.span())
    });

    let bms_core = meta.bms_core_path.0;
    let bms_lua = meta.bms_lua_path.0;
    let tealr: Path = parse_quote_spanned!(bms_lua.span()=>
        #bms_lua::tealr
    );
    let mlua: Path = parse_quote_spanned!(bms_core.span()=>
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

    // extract composites first
    let mut composites: HashMap<String, Vec<(TraitItemFn, FunctionAttrs)>> = HashMap::new();
    meta.functions.0.retain(|f| {
        let attrs = FunctionAttrs::from_attributes(&f.attrs).unwrap();
        if let Some(composite_id) = &attrs.composite {
            composites
                .entry(composite_id.to_owned())
                .or_default()
                .push((f.clone(), attrs));
            false
        } else {
            true
        }
    });

    let add_composite_function_stmts = composites.into_values().map(|functions| {
        let (first_function, first_function_attrs) = functions
            .first()
            .cloned()
            .expect("At least one function must be a composite for this code to be reached");

        let name = match &first_function_attrs.metamethod {
            Some(metamethod) => quote_spanned!(metamethod.span()=>
                #mlua::MetaMethod::#metamethod
            ),
            None => first_function.sig.ident.to_string().to_token_stream(),
        };

        let value_arg_types = (0..first_function.sig.inputs.len())
            .map(|_| {
                quote_spanned!(first_function.span()=>
                    #mlua::Value
                )
            })
            .collect::<Vec<_>>();

        let value_arg_names = (0..first_function.sig.inputs.len()).map(|i| {
            format_ident!("arg{}", i, span = first_function.span())
        }).collect::<Vec<_>>(); 

        let closures = functions
            .into_iter()
            .map(|(mut f, attrs)| {
                proxy_wrap_function_def(&mut f, &target_type, &bms_core, &bms_lua, &mlua, &attrs);
                convert_function_def_to_closure(&f)
            })
            .collect::<Vec<_>>();

        let closure_args_types = closures.iter().map(|closure| {
            let last = closure.inputs.last().unwrap();
            if let syn::Pat::Type(pat_type) = last {
                &pat_type.ty
            } else {
                panic!("Closure must have a single argument tuple as its last argument")
            }
        });

        let closure = parse_quote_spanned! {first_function.span()=>
            |ctxt, (#(#value_arg_names,)*): (#(#value_arg_types,)*)| {
                let args = #mlua::MultiValue::from_vec(vec![#(#value_arg_names,)*]);
                #(
                    if let Ok(args) = <#closure_args_types as #mlua::FromLuaMulti>::from_lua_multi(args.clone(), ctxt) {
                        let out : Result<_, #mlua::Error> = (#closures)(ctxt, args);
                        return out?.into_lua(ctxt)
                    }
                )*
                Err(#mlua::Error::external("Invalid arguments for composite function"))
            }
        };

        generate_methods_registration(&first_function_attrs, first_function.span(), name, closure)
    });

    let add_function_stmts = meta.functions.0.into_iter().filter_map(|mut f| {
        let attrs = FunctionAttrs::from_attributes(&f.attrs).unwrap();

        if attrs.skip.is_present() {
            return None;
        }

        proxy_wrap_function_def(&mut f, &target_type, &bms_core, &bms_lua, &mlua, &attrs);

        let name = match &attrs.metamethod {
            Some(metamethod) => quote_spanned!(metamethod.span()=>
                #mlua::MetaMethod::#metamethod
            ),
            None => f.sig.ident.to_string().to_token_stream(),
        };
        let span = f.span();

        let closure = convert_function_def_to_closure(&f);

        Some(generate_methods_registration(&attrs, span, name, closure))
    });

    let vis = &meta.vis;

    let definition = if meta.proxy_as_self.is_present() {
        quote_spanned!(derive_input.span()=>
            #[derive(Clone, Debug, #tealr::mlu::UserData, #tealr::ToTypename)]
            #vis struct #proxy_type_ident (pub #target_type);
        )
    } else {
        quote_spanned!(derive_input.span()=>
            #[derive(Clone, Debug, #tealr::mlu::UserData, #tealr::ToTypename)]
            #vis struct #proxy_type_ident (pub #bms_core::bindings::ReflectReference);
        )
   };

   let conversions = if meta.proxy_as_self.is_present() {
        quote!(
            impl <'a>From<&'a #proxy_type_ident> for #target_type {
                fn from(r: &'a #proxy_type_ident) -> Self {
                    r.0.clone()
                }
            }
        )
    } else {
        quote_spanned!(derive_input.span()=>
            impl AsRef<#bms_core::bindings::ReflectReference> for #proxy_type_ident {
                fn as_ref(&self) -> &#bms_core::bindings::ReflectReference {
                    &self.0
                }
            }

            impl From<#bms_core::bindings::ReflectReference> for #proxy_type_ident {
                fn from(r: #bms_core::bindings::ReflectReference) -> Self {
                    Self(r)
                }
            }
        )
    };
    quote_spanned! {meta.ident.span()=>

        #definition

        impl #bms_lua::bindings::proxy::LuaProxied for #target_type {
            type Proxy = #proxy_type_ident;
        }

        impl #tealr::mlu::TealData for #proxy_type_ident {
            fn add_methods<'lua, M: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut M) {
                #(#type_level_document_calls)*
                #(#add_composite_function_stmts)*
                #(#add_function_stmts)*
            }
        }


        impl<'lua> #tealr::mlu::mlua::FromLua<'lua> for #proxy_type_ident {
            fn from_lua(
                value: #tealr::mlu::mlua::Value<'lua>,
                _lua: &#tealr::mlu::mlua::Lua,
            ) -> Result<Self, #tealr::mlu::mlua::Error> {
                match value {
                    tealr::mlu::mlua::Value::UserData(ud) => Ok(ud.borrow::<Self>()?.clone()),
                    _ => {
                        return Err(#tealr::mlu::mlua::Error::FromLuaConversionError {
                            from: value.type_name(),
                            to: stringify!(#proxy_type_ident),
                            message: None,
                        })
                    }
                }
            }
        }

        #conversions
    }
    .into()
}

