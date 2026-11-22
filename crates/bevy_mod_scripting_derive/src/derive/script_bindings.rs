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

/// The registration of the FFI form of an `#[ffi]` function: its body as a closure, which `FfiFn` turns into an
/// `extern "C"` function (a compile error if a parameter or the result can't be passed, i.e. a `Union`: those functions
/// register a form for each alternative by hand).
///
/// ```rust,ignore
/// registry.register_ffi(TypeId::of::<Vec3>(), "dot", |_self: R<Vec3>, other: R<Vec3>| -> f32 { _self.dot(*other) });
/// ```
fn ffi_wrapper(
    fun: &syn::ImplItemFn,
    self_ty: &syn::Type,
    _bms: &syn::Path,
) -> syn::Result<TokenStream> {
    let span = fun.sig.ident.span();
    if !fun.sig.generics.params.is_empty() || fun.sig.asyncness.is_some() {
        return Err(syn::Error::new(
            span,
            "`#[ffi]` functions can't be generic or async",
        ));
    }
    if let Some(receiver) = fun.sig.receiver() {
        return Err(syn::Error::new(
            receiver.span(),
            "`#[ffi]` functions take no `self`, use `_self: R<Self>`",
        ));
    }
    let syn::ReturnType::Type(_, out_type) = &fun.sig.output else {
        return Err(syn::Error::new(span, "`#[ffi]` functions return a value"));
    };
    let args = &fun.sig.inputs;
    let body = &fun.block;
    let fun_name = syn::LitStr::new(&fun.sig.ident.to_string(), Span::call_site());
    Ok(quote_spanned! {span=>
        registry.register_ffi(
            ::std::any::TypeId::of::<#self_ty>(),
            #fun_name,
            |#args| -> #out_type #body,
        );
    })
}
