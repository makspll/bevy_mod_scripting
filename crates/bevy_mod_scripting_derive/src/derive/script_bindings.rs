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
    let impl_block = syn::parse_macro_input!(input as ItemImpl);
    let impl_span = impl_block.span();
    // let (impl_generics, ty_generics, where_clause) = impl_block.generics.split_for_impl();

    let type_ident_with_generics = &impl_block.self_ty;
    let impl_block_self_ty = impl_block.self_ty.clone();
    let mut function_registrations = Vec::with_capacity(impl_block.items.len());
    let mut ffi_wrappers = Vec::new();
    for i in &impl_block.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                let fun = impl_fn_to_namespace_builder_registration(impl_item_fn);
                function_registrations.push(fun);
                if args.ffi
                    && let Some(wrapper) = ffi_wrapper(
                        impl_item_fn,
                        &impl_block.self_ty,
                        &args.shared_args.bms_bindings_path,
                    )
                {
                    ffi_wrappers.push(wrapper);
                }
            }
            _ => continue,
        }
    }
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
    /// If true, functions whose parameters and result have C types also get an `extern "C"` wrapper, which languages call
    /// through their FFI, see `bevy_mod_scripting_bindings::native::FfiFunction`. A panic in a wrapper aborts the process:
    /// only for impl blocks whose functions don't panic
    pub ffi: bool,

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
            ffi: Default::default(),
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

        if meta.path.is_ident("ffi") {
            self.ffi = true;
            return Ok(());
        }

        if self.shared_args.apply_nested_meta(meta)? {
            return Ok(());
        }

        Err(meta.error("Unknown argument to script_bindings"))
    }
}

/// How a parameter or the result of a function is passed to its `extern "C"` wrapper, see [`ffi_wrapper`].
enum FfiKind {
    /// By value, with this C type
    Scalar(&'static str),
    /// A pointer to the value, `R<T>`
    Ref(syn::Type),
    /// A pointer to the value, `M<T>`
    Mut(syn::Type),
    /// A pointer to the value, `V<T>`, copied (results: an out-pointer)
    Value(syn::Type),
}

/// The FFI kind of a type as written: scalars, and `R`, `M` and `V` of any type (types without an FFI layout get wrappers
/// languages never call, see `FfiFunction`). `None` for anything else.
fn ffi_kind(ty: &syn::Type) -> Option<FfiKind> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let scalar = match segment.ident.to_string().as_str() {
        "f32" => Some("float"),
        "f64" => Some("double"),
        "i32" => Some("int32_t"),
        "u32" => Some("uint32_t"),
        "i64" => Some("int64_t"),
        "u64" => Some("uint64_t"),
        "bool" => Some("bool"),
        "usize" => Some("size_t"),
        _ => None,
    };
    if let Some(scalar) = scalar {
        return segment
            .arguments
            .is_none()
            .then_some(FfiKind::Scalar(scalar));
    }
    let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
        return None;
    };
    let inner = arguments.args.iter().find_map(|a| match a {
        syn::GenericArgument::Type(ty) => Some(ty.clone()),
        _ => None,
    })?;
    match segment.ident.to_string().as_str() {
        "R" => Some(FfiKind::Ref(inner)),
        "M" => Some(FfiKind::Mut(inner)),
        "V" => Some(FfiKind::Value(inner)),
        _ => None,
    }
}

/// The registration of the `extern "C"` wrapper of a function whose parameters and result have FFI kinds (see
/// [`ffi_kind`]), `None` for other functions. The wrapper runs the function's body like its registration does.
///
/// ```rust,ignore
/// extern "C" fn wrapper(a0: *const Vec3, a1: *const Vec3, out: *mut Vec3) {
///     let f = |a: V<Vec3>, b: V<Vec3>| -> V<Vec3> { .. };
///     unsafe { out.write(f(V(*a0), V(*a1)).into_inner()) }
/// }
/// ```
fn ffi_wrapper(fun: &syn::ImplItemFn, self_ty: &syn::Type, bms: &syn::Path) -> Option<TokenStream> {
    let span = fun.sig.ident.span();
    if !fun.sig.generics.params.is_empty() || fun.sig.asyncness.is_some() {
        return None;
    }
    let mut params = Vec::new();
    let mut conversions = Vec::new();
    let mut kinds = Vec::new();
    for (i, arg) in fun.sig.inputs.iter().enumerate() {
        let syn::FnArg::Typed(arg) = arg else {
            return None;
        };
        let name = format_ident!("a{i}");
        let (param, conversion, kind) = match ffi_kind(&arg.ty)? {
            FfiKind::Scalar(c) => {
                let ty = &arg.ty;
                (
                    quote_spanned! {span=> #name: #ty },
                    quote_spanned! {span=> #name },
                    quote_spanned! {span=> #bms::native::FfiType::Scalar(#c) },
                )
            }
            FfiKind::Ref(inner) => (
                quote_spanned! {span=> #name: *const #inner },
                quote_spanned! {span=> #bms::R::from(unsafe { &*#name }) },
                quote_spanned! {span=> #bms::native::FfiType::Value(::std::any::TypeId::of::<#inner>()) },
            ),
            FfiKind::Mut(inner) => (
                quote_spanned! {span=> #name: *mut #inner },
                quote_spanned! {span=> #bms::M::from(unsafe { &mut *#name }) },
                quote_spanned! {span=> #bms::native::FfiType::Mut(::std::any::TypeId::of::<#inner>()) },
            ),
            FfiKind::Value(inner) => (
                quote_spanned! {span=> #name: *const #inner },
                quote_spanned! {span=> #bms::V(unsafe { *#name }) },
                quote_spanned! {span=> #bms::native::FfiType::Value(::std::any::TypeId::of::<#inner>()) },
            ),
        };
        params.push(param);
        conversions.push(conversion);
        kinds.push(kind);
    }
    let (out_param, ret_type, finish, result) = match &fun.sig.output {
        syn::ReturnType::Default => (
            None,
            quote_spanned! {span=> () },
            quote_spanned! {span=> output },
            quote_spanned! {span=> #bms::native::FfiType::Unit },
        ),
        syn::ReturnType::Type(_, ty) => match ffi_kind(ty)? {
            FfiKind::Scalar(c) => (
                None,
                quote_spanned! {span=> #ty },
                quote_spanned! {span=> output },
                quote_spanned! {span=> #bms::native::FfiType::Scalar(#c) },
            ),
            FfiKind::Value(inner) => (
                Some(quote_spanned! {span=> out: *mut #inner }),
                quote_spanned! {span=> () },
                quote_spanned! {span=> unsafe { out.write(output.into_inner()) } },
                quote_spanned! {span=> #bms::native::FfiType::Value(::std::any::TypeId::of::<#inner>()) },
            ),
            FfiKind::Ref(_) | FfiKind::Mut(_) => return None,
        },
    };
    let args = &fun.sig.inputs;
    let body = &fun.block;
    let out_type = match &fun.sig.output {
        syn::ReturnType::Default => quote_spanned! {span=> () },
        syn::ReturnType::Type(_, ty) => quote_spanned! {span=> #ty },
    };
    let fun_name = syn::LitStr::new(&fun.sig.ident.to_string(), Span::call_site());
    let params = params.into_iter().chain(out_param);
    Some(quote_spanned! {span=>
        {
            // Safety (callers): pointers to valid values of their types, the out-pointer to memory for one; a panic aborts
            #[allow(unused_unsafe, clippy::not_unsafe_ptr_arg_deref, clippy::unused_unit)]
            extern "C" fn wrapper(#(#params),*) -> #ret_type {
                let function = |#args| -> #out_type { #body };
                let output = function(#(#conversions),*);
                #finish
            }
            registry.register_ffi_function(
                ::std::any::TypeId::of::<#self_ty>(),
                #bms::native::FfiFunction {
                    name: #fun_name,
                    function: wrapper as *const (),
                    params: vec![#(#kinds),*],
                    result: #result,
                },
            );
        }
    })
}
