use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote_spanned};
use syn::{ItemImpl, spanned::Spanned};

use crate::derive::SharedArgs;

use super::{impl_fn_to_namespace_builder_registration, is_public_impl};

pub fn script_bindings(
    args_stream: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = Args::default();
    let parser = syn::meta::parser(|meta| args.apply_nested_meta(&meta));
    syn::parse_macro_input!(args_stream with parser);
    let mut impl_block = syn::parse_macro_input!(input as ItemImpl);
    let impl_span = impl_block.span();
    // let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let impl_block_self_ty = impl_block.self_ty.clone();
    let mut function_registrations = Vec::with_capacity(impl_block.items.len());
    let mut ffi_wrappers = Vec::new();
    for i in &mut impl_block.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                // `#[ffi]` functions also get an `extern "C"` wrapper, see `FfiFunction`: the attribute is ours only
                let attributes = impl_item_fn.attrs.len();
                impl_item_fn.attrs.retain(|a| !a.path().is_ident("ffi"));
                let fun = impl_fn_to_namespace_builder_registration(impl_item_fn);
                function_registrations.push(fun);
                if impl_item_fn.attrs.len() != attributes {
                    ffi_wrappers.push(
                        ffi_wrapper(
                            impl_item_fn,
                            &impl_block_self_ty,
                            &args.shared_args.bms_bindings_path,
                        )
                        .unwrap_or_else(syn::Error::into_compile_error),
                    );
                }
            }
            _ => continue,
        }
    }
    let type_ident_with_generics = &impl_block.self_ty;
    let ffi_registrations = match ffi_wrappers.is_empty() {
        true => TokenStream::default(),
        false => {
            let bms_bindings_path = &args.shared_args.bms_bindings_path;
            quote_spanned! {impl_span=>
                {
                    let mut registry = world
                        .get_resource_or_init::<#bms_bindings_path::function::script_function::AppScriptFunctionRegistry>()
                        .0
                        .clone();
                    let mut registry = registry.write();
                    #(#ffi_wrappers)*
                }
            }
        }
    };

    let visibility = match is_public_impl(&impl_block) {
        true => quote_spanned! {impl_span=>
            pub
        },
        false => quote_spanned! {impl_span=>
            pub(crate)
        },
    };

    let impl_block = match args.remote {
        true => TokenStream::default(),
        false => quote_spanned! {impl_span=>
            #impl_block
        },
    };

    let bms_bindings_path = &args.shared_args.bms_bindings_path;

    let function_name = format_ident!("register_{}", args.name);
    let builder_function_name = if args.unregistered {
        format_ident!("new_unregistered")
    } else {
        format_ident!("new")
    };

    let mark_as_generated = if args.generated {
        quote_spanned! {impl_span=>

            {
                let registry = world.get_resource_or_init::<AppTypeRegistry>();
                let mut registry = registry.write();
                registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsGenerated>();
            }
        }
    } else {
        Default::default()
    };

    let mark_as_core = if bms_bindings_path.is_ident("crate") || args.core {
        quote_spanned! {impl_span=>
            {
                let registry = world.get_resource_or_init::<AppTypeRegistry>();
                let mut registry = registry.write();
                registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsCore>();
            }
        }
    } else {
        Default::default()
    };

    let mark_as_significant = if args.significant {
        quote_spanned! {impl_span=>
            {
                let registry = world.get_resource_or_init::<AppTypeRegistry>();
                let mut registry = registry.write();
                registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsSignificant>();
            }
        }
    } else {
        Default::default()
    };

    let set_namespace = match &args.namespace {
        Some(namespace) => quote_spanned! {impl_span=>
            {
                let registry = world.get_resource_or_init::<AppTypeRegistry>();
                let mut registry = registry.write();
                #bms_bindings_path::set_script_namespace_if_unset::<#type_ident_with_generics>(&mut registry, #namespace);
            }
        },
        None => Default::default(),
    };

    // functions of `impl NamespaceMembers` are members of the namespace given, not functions on a type
    let is_namespace_members = matches!(
        &*impl_block_self_ty,
        syn::Type::Path(path) if path.path.segments.last().is_some_and(|s| s.ident == "NamespaceMembers")
    );

    let out = if is_namespace_members {
        let Some(namespace) = &args.namespace else {
            return syn::Error::new(
                impl_span,
                "`impl NamespaceMembers` registers functions into a script namespace, which needs `namespace = \"...\"`",
            )
            .to_compile_error()
            .into();
        };
        quote_spanned! {impl_span=>
            #visibility fn #function_name(world: &mut World) {
                #bms_bindings_path::function::namespace::NamespaceBuilder::<#type_ident_with_generics>::in_namespace(world, #namespace)
                    #(#function_registrations)*;
            }

            #impl_block
        }
    } else {
        quote_spanned! {impl_span=>
            #visibility fn #function_name(world: &mut World) {
                #bms_bindings_path::function::namespace::NamespaceBuilder::<#type_ident_with_generics>::#builder_function_name(world)
                    #(#function_registrations)*;

                #mark_as_generated
                #mark_as_core
                #mark_as_significant
                #set_namespace
                #ffi_registrations
            }

            #impl_block
        }
    };

    out.into()
}

struct Args {
    /// The name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn
    pub name: syn::Ident,
    /// If true the original impl block will be ignored, and only the function registrations will be generated
    pub remote: bool,
    /// If true will use `new_unregistered` instead of `new` for the namespace builder
    pub unregistered: bool,
    /// If true registers a marker type against the type registry to state that the type is generated (if unregistered is not set)
    pub generated: bool,
    /// If true registers a marker type against the type registry to state that the type is core to BMS (if unregistered is not set)
    pub core: bool,
    /// If true registers a marker type against the type registry to state that the type is significant (if unregistered is not set)
    pub significant: bool,
    /// If set, puts the type in the given script namespace (if it is not in one already), instead of the one named after its crate
    pub namespace: Option<syn::LitStr>,

    pub shared_args: SharedArgs,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            name: syn::Ident::new("functions", Span::call_site()),
            remote: Default::default(),
            unregistered: Default::default(),
            generated: Default::default(),
            core: Default::default(),
            significant: Default::default(),
            namespace: Default::default(),
            shared_args: Default::default(),
        }
    }
}

impl Args {
    pub fn apply_nested_meta(&mut self, meta: &syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("name") {
            let value = meta.value()?;
            let lit: syn::LitStr = value.parse()?;
            self.name = syn::parse_str(&lit.value())?;
            return Ok(());
        }

        if meta.path.is_ident("remote") {
            self.remote = true;
            return Ok(());
        }

        if meta.path.is_ident("unregistered") {
            self.unregistered = true;
            return Ok(());
        }

        if meta.path.is_ident("generated") {
            self.generated = true;
            return Ok(());
        }

        if meta.path.is_ident("core") {
            self.core = true;
            return Ok(());
        }

        if meta.path.is_ident("significant") {
            self.significant = true;
            return Ok(());
        }

        if meta.path.is_ident("namespace") {
            self.namespace = Some(meta.value()?.parse()?);
            return Ok(());
        }

        if self.shared_args.apply_nested_meta(meta)? {
            return Ok(());
        }

        Err(meta.error("Unknown argument to script_bindings"))
    }
}

/// How a parameter is passed to an `extern "C"` wrapper: its declaration, its conversion to the function's parameter and
/// its `FfiType`.
type FfiParam = (TokenStream, TokenStream, TokenStream);

/// The ways the parameter `name` of the type `ty` is passed: one for most types, through `FfiArg` (which errors clearly for
/// types that can't be), one for each alternative of a `Union` as written, which get a wrapper each.
fn ffi_params(name: &syn::Ident, ty: &syn::Type, bms: &syn::Path) -> Vec<FfiParam> {
    if let Some((left, right)) = union_alternatives(ty) {
        let mut params = Vec::new();
        for (alternative, side) in [(left, "new_left"), (right, "new_right")] {
            let side = format_ident!("{side}");
            for (param, conversion, kind) in ffi_params(name, alternative, bms) {
                let span = alternative.span();
                let conversion = quote_spanned! {span=> #bms::Union::#side(#conversion) };
                params.push((param, conversion, kind));
            }
        }
        return params;
    }
    let span = ty.span();
    let arg = quote_spanned! {span=> <#ty as #bms::native::FfiArg> };
    vec![(
        quote_spanned! {span=> #name: #arg::C },
        // Safety (callers): see `FfiArg::from_c`
        quote_spanned! {span=> unsafe { #arg::from_c(#name) } },
        quote_spanned! {span=> #arg::kind() },
    )]
}

/// The two alternatives of a `Union<L, R>`, `None` for other types.
fn union_alternatives(ty: &syn::Type) -> Option<(&syn::Type, &syn::Type)> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    let mut types = arguments.args.iter().filter_map(|a| match a {
        syn::GenericArgument::Type(ty) => Some(ty),
        _ => None,
    });
    match (segment.ident == "Union", types.next(), types.next()) {
        (true, Some(left), Some(right)) => Some((left, right)),
        _ => None,
    }
}

/// The registrations of the `extern "C"` wrappers of an `#[ffi]` function: its parameters through `FfiArg`, its result
/// through `FfiReturn` (compile errors for types that can't be passed). A wrapper runs the function's body like its
/// registration does. A function with `Union` parameters gets a wrapper for each combination of their alternatives (i.e.
/// `add(R<Vec3>, R<Vec3>)` and `add(R<Vec3>, f32)`), so operators are FFI calls too.
///
/// ```rust,ignore
/// extern "C" fn wrapper(a0: *const Vec3, a1: *const Vec3, out: *mut u8) -> () {
///     let f = |a: V<Vec3>, b: V<Vec3>| -> V<Vec3> { .. };
///     let output = f(unsafe { <V<Vec3> as FfiArg>::from_c(a0) }, ..);
///     unsafe { FfiReturn::into_c(output, out) }
/// }
/// ```
fn ffi_wrapper(
    fun: &syn::ImplItemFn,
    self_ty: &syn::Type,
    bms: &syn::Path,
) -> syn::Result<TokenStream> {
    let span = fun.sig.ident.span();
    if !fun.sig.generics.params.is_empty() || fun.sig.asyncness.is_some() {
        return Err(syn::Error::new(
            span,
            "`#[ffi]` functions can't be generic or async",
        ));
    }
    // every combination of the ways the parameters are passed
    let mut overloads: Vec<Vec<FfiParam>> = vec![Vec::new()];
    for (i, arg) in fun.sig.inputs.iter().enumerate() {
        let syn::FnArg::Typed(arg) = arg else {
            return Err(syn::Error::new(
                arg.span(),
                "`#[ffi]` functions take no `self`, use `_self: R<Self>`",
            ));
        };
        let alternatives = ffi_params(&format_ident!("a{i}"), &arg.ty, bms);
        overloads = overloads
            .into_iter()
            .flat_map(|overload| {
                alternatives.iter().map(move |alternative| {
                    let mut overload = overload.clone();
                    overload.push(alternative.clone());
                    overload
                })
            })
            .collect();
    }
    let syn::ReturnType::Type(_, out_type) = &fun.sig.output else {
        return Err(syn::Error::new(span, "`#[ffi]` functions return a value"));
    };
    let ret = quote_spanned! {out_type.span()=> <#out_type as #bms::native::FfiReturn> };
    let args = &fun.sig.inputs;
    let body = &fun.block;
    let fun_name = syn::LitStr::new(&fun.sig.ident.to_string(), Span::call_site());
    let wrappers = overloads.into_iter().map(|overload| {
        let params = overload.iter().map(|(param, _, _)| param);
        let conversions = overload.iter().map(|(_, conversion, _)| conversion);
        let kinds = overload.iter().map(|(_, _, kind)| kind);
        quote_spanned! {span=>
            {
                // Safety (callers): arguments as `FfiArg::from_c` says, the out-pointer as `FfiReturn::into_c` says; a
                // panic aborts
                #[allow(unused_unsafe, clippy::not_unsafe_ptr_arg_deref, clippy::unused_unit)]
                extern "C" fn wrapper(#(#params,)* out: *mut u8) -> #ret::C {
                    let function = |#args| -> #out_type { #body };
                    let output = function(#(#conversions),*);
                    unsafe { #ret::into_c(output, out) }
                }
                registry.register_ffi_function(
                    ::std::any::TypeId::of::<#self_ty>(),
                    #bms::native::FfiFunction {
                        name: #fun_name,
                        function: wrapper as *const (),
                        params: vec![#(#kinds),*],
                        result: #ret::result(),
                    },
                );
            }
        }
    });
    Ok(quote_spanned! {span=> #(#wrappers)* })
}
