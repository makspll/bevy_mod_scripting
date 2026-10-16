use proc_macro2::TokenStream;
use syn::DeriveInput;

/// Which conversions the type implements by hand
#[derive(Default)]
struct Args {
    custom_from: bool,
    custom_into: bool,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Self::default();
        for attr in attrs {
            if attr.path().is_ident("script_type") {
                attr.parse_nested_meta(|meta| {
                    let custom = |meta: &syn::meta::ParseNestedMeta| -> syn::Result<()> {
                        let value: syn::Ident = meta.value()?.parse()?;
                        if value == "custom" {
                            Ok(())
                        } else {
                            Err(meta.error("expected `custom`"))
                        }
                    };
                    if meta.path.is_ident("from") {
                        custom(&meta)?;
                        args.custom_from = true;
                        return Ok(());
                    }
                    if meta.path.is_ident("into") {
                        custom(&meta)?;
                        args.custom_into = true;
                        return Ok(());
                    }
                    Err(meta.error("Unknown argument to script_type, expected `from = custom` or `into = custom`"))
                })?;
            }
        }
        Ok(args)
    }
}

pub fn script_type(input: TokenStream) -> TokenStream {
    let args = match syn::parse2::<DeriveInput>(input.clone())
        .and_then(|derive_input| Args::parse(&derive_input.attrs))
    {
        Ok(args) => args,
        Err(error) => return error.to_compile_error(),
    };

    let mut out = TokenStream::new();
    if !args.custom_from {
        out.extend(super::from_script(input.clone()));
    }
    if !args.custom_into {
        out.extend(super::into_script(input.clone()));
    }
    out.extend(super::arg_meta(input.clone()));
    out.extend(super::get_type_dependencies(input.clone()));
    // derived conversions go through `V<T>`, so the type is documented as one, custom ones are opaque
    let value = !args.custom_from && !args.custom_into;
    out.extend(super::typed_through::typed_through_with(input, Some(value)));
    out
}
