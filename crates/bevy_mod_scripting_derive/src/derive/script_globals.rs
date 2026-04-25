use proc_macro2::Span;
use quote::{format_ident, quote_spanned};
use syn::{ItemImpl, spanned::Spanned};

use crate::derive::SharedArgs;

use super::{impl_fn_to_global_registry_registration, is_public_impl};

pub fn script_globals(
    args_stream: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = Args::default();
    let parser = syn::meta::parser(|meta| args.apply_nested_meta(&meta));
    syn::parse_macro_input!(args_stream with parser);

    let impl_block = syn::parse_macro_input!(input as ItemImpl);
    let impl_span = impl_block.span();
    let mut function_registrations = Vec::with_capacity(impl_block.items.len());

    for i in &impl_block.items {
        match i {
            syn::ImplItem::Fn(impl_item_fn) => {
                let fun = impl_fn_to_global_registry_registration(impl_item_fn);
                function_registrations.push(fun);
            }
            _ => continue,
        }
    }

    let function_name = format_ident!("register_{}", args.name);
    let bms_bindings_path = &args.shared_args.bms_bindings_path;

    let visibility = match is_public_impl(&impl_block) {
        true => quote_spanned! {impl_span=>
            pub
        },
        false => quote_spanned! {impl_span=>
            pub(crate)
        },
    };

    let out = quote_spanned! {impl_span=>
        #visibility fn #function_name(world: &mut World) {

            let registry = world.get_resource_or_init::<#bms_bindings_path::globals::AppScriptGlobalsRegistry>();
            let mut registry = registry.write();

            #(
                if (registry #function_registrations).is_some() {
                    warn!("conflicting global registration under name: {}. This might cause confusing problems, use `CoreScriptGlobalsPlugin.filter` to filter out uneeded duplicate types.", stringify!(#function_name))
                }
            )*;
        }
    };

    out.into()
}

struct Args {
    /// The name to use to suffix the generated function, i.e. `test_fn` will generate `register_test_fn
    pub name: syn::Ident,
    pub shared_args: SharedArgs,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            name: syn::Ident::new("functions", Span::call_site()),
            shared_args: SharedArgs::default(),
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

        // delegate shared args
        if self.shared_args.apply_nested_meta(meta)? {
            return Ok(());
        }

        Err(meta.error("Unknown argument to script_globals"))
    }
}
