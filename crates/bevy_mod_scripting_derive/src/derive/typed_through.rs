use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::derive::SharedArgs;
#[derive(Default)]
struct Args {
    shared_args: SharedArgs,
    /// Whether the type is documented as a value, i.e. like `V<T>`
    value: bool,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Self::default();

        for attr in attrs {
            if attr.path().is_ident("typed_through") {
                attr.parse_nested_meta(|meta| {
                    if args.shared_args.apply_nested_meta(&meta)? {
                        return Ok(());
                    }
                    if meta.path.is_ident("value") {
                        args.value = true;
                        return Ok(());
                    }

                    Err(meta.error("Unknown argument to typed_through"))
                })?;
            }
        }

        Ok(args)
    }
}

pub fn typed_through(input: TokenStream) -> TokenStream {
    typed_through_with(input, None)
}

/// The `TypedThrough` implementation, documented as a value if `value` is set, otherwise as the attributes say.
pub(crate) fn typed_through_with(input: TokenStream, value: Option<bool>) -> TokenStream {
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

    let turbofish = type_generics.as_turbofish();
    let bms_bindings_path = args.shared_args.bms_bindings_path;
    let variant = if value.unwrap_or(args.value) {
        quote!(Val)
    } else {
        quote!(Opaque)
    };
    quote! {
        impl #impl_generics #bms_bindings_path::TypedThrough for #ident #type_generics #where_clause {
            fn through_type_info() -> #bms_bindings_path::ThroughTypeInfo {
                #bms_bindings_path::ThroughTypeInfo::#variant(#ident #turbofish ::type_info())
            }
        }
    }
}
