use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::derive::SharedArgs;
#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut shared_args = SharedArgs::default();

        for attr in attrs {
            if attr.path().is_ident("arg_meta") {
                attr.parse_nested_meta(|meta| {
                    // delegate everything to SharedArgs
                    if shared_args.apply_nested_meta(&meta)? {
                        return Ok(());
                    }

                    Err(meta.error("Unknown argument to get_type_dependencies"))
                })?;
            }
        }

        Ok(Self { shared_args })
    }
}
pub fn arg_meta(input: TokenStream) -> TokenStream {
    let (args, ident, generics) = match syn::parse2::<DeriveInput>(input) {
        Ok(derive_input) => {
            let args = match Args::parse(&derive_input.attrs) {
                Ok(args) => args,
                Err(error) => return error.to_compile_error(),
            };
            (args, derive_input.ident, derive_input.generics)
        }
        Err(err) => return err.to_compile_error(),
    };

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    let bms_bindings_path = args.shared_args.bms_bindings_path;
    quote! {
        impl #impl_generics #bms_bindings_path::ArgMeta for #ident #type_generics #where_clause {
        }
    }
}
