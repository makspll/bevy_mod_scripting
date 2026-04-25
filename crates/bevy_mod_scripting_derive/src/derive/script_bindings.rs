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
    let mut function_registrations = Vec::with_capacity(impl_block.items.len());
    for i in &impl_block.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                let fun = impl_fn_to_namespace_builder_registration(impl_item_fn);
                function_registrations.push(fun);
            }
            _ => continue,
        }
    }

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

            let registry = world.get_resource_or_init::<AppTypeRegistry>();
            let mut registry = registry.write();
            registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsGenerated>();
        }
    } else {
        Default::default()
    };

    let mark_as_core = if bms_bindings_path.is_ident("crate") || args.core {
        quote_spanned! {impl_span=>
            let registry = world.get_resource_or_init::<AppTypeRegistry>();
            let mut registry = registry.write();
            registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsCore>();
        }
    } else {
        Default::default()
    };

    let mark_as_significant = if args.significant {
        quote_spanned! {impl_span=>
            let registry = world.get_resource_or_init::<AppTypeRegistry>();
            let mut registry = registry.write();
            registry.register_type_data::<#type_ident_with_generics, #bms_bindings_path::MarkAsSignificant>();
        }
    } else {
        Default::default()
    };

    let use_dummy = if args.use_dummy_registry {
        quote_spanned! {impl_span=>
            .with_dummy_registry()
        }
    } else {
        Default::default()
    };

    let out = quote_spanned! {impl_span=>
        #visibility fn #function_name(world: &mut World) {
            #bms_bindings_path::function::namespace::NamespaceBuilder::<#type_ident_with_generics>::#builder_function_name(world)
                #use_dummy
                #(#function_registrations)*;

            #mark_as_generated
            #mark_as_core
            #mark_as_significant
        }

        #impl_block
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
    /// If true will register into the [`DummyScriptFunctionRegistry`] instead of the full one.
    /// This is useful for documenting functions without actually making them available, if you're exposing them another way.
    pub use_dummy_registry: bool,

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
            use_dummy_registry: Default::default(),
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

        if meta.path.is_ident("use_dummy_registry") {
            self.use_dummy_registry = true;
            return Ok(());
        }

        if self.shared_args.apply_nested_meta(meta)? {
            return Ok(());
        }

        Err(meta.error("Unknown argument to script_bindings"))
    }
}
