use proc_macro2::Span;
use quote::{format_ident, quote_spanned};
use syn::{ItemImpl, spanned::Spanned};

use super::{impl_fn_to_global_registry_registration, is_public_impl};

pub fn script_globals(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = syn::parse_macro_input!(args as Args);

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
    let bms_bindings_path = &args.bms_bindings_path;

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
    /// If set the path to override bms imports
    pub bms_bindings_path: syn::Path,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse separated key-value pairs
        let pairs =
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?;

        let mut name = syn::Ident::new("functions", Span::call_site());
        let mut bms_bindings_path =
            syn::Path::from(syn::Ident::new("bevy_mod_scripting", Span::call_site()));
        bms_bindings_path.segments.push(syn::PathSegment {
            ident: syn::Ident::new("core", Span::call_site()),
            arguments: syn::PathArguments::None,
        });
        let mut unknown_spans = Vec::default();
        for pair in pairs {
            match &pair {
                syn::Meta::NameValue(name_value) => {
                    if name_value.path.is_ident("bms_bindings_path")
                        && let syn::Expr::Lit(path) = &name_value.value
                        && let syn::Lit::Str(lit_str) = &path.lit
                    {
                        bms_bindings_path = syn::parse_str(&lit_str.value())?;
                        continue;
                    } else if name_value.path.is_ident("name")
                        && let syn::Expr::Lit(path) = &name_value.value
                        && let syn::Lit::Str(lit_str) = &path.lit
                    {
                        name = syn::parse_str(&lit_str.value())?;
                        continue;
                    }
                }
                _ => {
                    unknown_spans.push((pair.span(), "Unsupported meta kind for script_globals"));
                    continue;
                }
            }

            unknown_spans.push((pair.span(), "Unknown argument to script_globals"));
        }

        if !unknown_spans.is_empty() {
            return Err(syn::Error::new(unknown_spans[0].0, unknown_spans[0].1));
        }

        Ok(Self {
            bms_bindings_path,
            name,
        })
    }
}
