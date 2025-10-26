use proc_macro2::{Span, TokenStream};
use quote::format_ident;
use syn::{DeriveInput, Ident, parse_macro_input, parse_quote};

struct Args {
    bms_display_path: syn::Path,
    remote: bool,
}

impl Args {
    fn parse(input: &[syn::Attribute]) -> syn::Result<Self> {
        let mut args = Args {
            bms_display_path: parse_quote!(::bevy_mod_scripting::display),
            remote: false,
        };

        for attr in input {
            if attr.path().is_ident("debug_with_type_info") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("bms_display_path") {
                        let value = meta.value()?;
                        let string: syn::LitStr = value.parse()?;
                        args.bms_display_path = string.parse()?;
                        Ok(())
                    } else if meta.path.is_ident("remote") {
                        args.remote = true;
                        Ok(())
                    } else {
                        Err(syn::Error::new_spanned(
                            meta.path,
                            "unknown attribute, allowed: bms_display_path,remote",
                        ))
                    }
                })?
            }
        }

        Ok(args)
    }
}

pub fn debug_with_type_info(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = parse_macro_input!(input as DeriveInput);

    // parse the args
    let args = match Args::parse(&derive_input.attrs) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    let bms_display_path = &args.bms_display_path;

    // use DebugStruct, DebugTuple, DebugMap, DebugList etc from bevy_mod_scripting_display
    // from the trait DebugWithTypeInfoBuilder trait on the formatter in implementing Debug i.e.:
    // Extension methods for `std::fmt::Formatter` which create builders that
    // / format values using `DebugWithTypeInfo`. These helpers mirror the
    // / standard formatter builders (`debug_struct`, `debug_tuple`, etc.) but
    // / ensure that fields, entries and keys/values are displayed through the
    // / `DebugWithTypeInfo` adapter.
    // pub trait DebugWithTypeInfoBuilder<'a, 'b: 'a> {
    //     /// Start formatting a struct with the given name using type-aware
    //     /// field formatting.
    //     fn debug_struct_with_type_info(&'a mut self, name: &str) -> DebugStruct<'a, 'b>;

    //     /// Start formatting a tuple-like value with the given name using
    //     /// type-aware element formatting.
    //     fn debug_tuple_with_type_info(&'a mut self, name: &str) -> DebugTuple<'a, 'b>;

    //     /// Start formatting a list using type-aware entry formatting.
    //     fn debug_list_with_type_info(&'a mut self) -> DebugList<'a, 'b>;

    //     /// Start formatting a set using type-aware entry formatting.
    //     fn debug_set_with_type_info(&'a mut self) -> DebugSet<'a, 'b>;

    //     /// Start formatting a map using type-aware key/value formatting.
    //     fn debug_map_with_type_info(&'a mut self) -> DebugMap<'a, 'b>;
    // }

    let name = derive_input.ident;
    let name_str = name.to_string();
    let placeholder_ident = syn::Ident::new("_", Span::call_site());

    let builder = match derive_input.data {
        syn::Data::Struct(data_struct) => {
            let self_ident = Ident::new("self", Span::call_site());
            build_from_fields(
                Some(&self_ident),
                &name_str,
                bms_display_path,
                &data_struct.fields,
            )
        }
        syn::Data::Enum(data_enum) => {
            // we emit a match statement
            let match_arms = data_enum.variants.into_iter().map(|variant| {
                let variant_ident = &variant.ident;
                let variant_name = variant.ident.to_string();
                let inner_builder =
                    build_from_fields(None, &variant_name, bms_display_path, &variant.fields);

                let destructuring = match &variant.fields {
                    syn::Fields::Named(fields) => {
                        let field_idents = fields
                            .named
                            .iter()
                            .map(|f| f.ident.as_ref().unwrap_or(&placeholder_ident));
                        quote::quote! { { #(#field_idents),* } }
                    }
                    syn::Fields::Unit => quote::quote! {},
                    syn::Fields::Unnamed(fields) => {
                        let field_idents = fields
                            .unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| format_ident!("_{}", i));
                        quote::quote! { ( #(#field_idents),* ) }
                    }
                };
                quote::quote! {
                    #name::#variant_ident #destructuring => #inner_builder
                }
            });
            quote::quote! {
                match self {
                    #(#match_arms),*
                }
            }
        }
        syn::Data::Union(data_union) => {
            return syn::Error::new_spanned(
                data_union.union_token,
                "DebugWithTypeInfo cannot be derived for unions",
            )
            .to_compile_error()
            .into();
        }
    };
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics #bms_display_path::DebugWithTypeInfo for #name #ty_generics #where_clause {
            fn to_string_with_type_info(&self, f: &mut std::fmt::Formatter<'_>, type_info_provider: Option<&dyn #bms_display_path::GetTypeInfo>) -> std::fmt::Result {
                #builder
            }
        }
        impl #impl_generics std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Debug::fmt(&#bms_display_path::WithTypeInfo::new(self), f)
            }
        }
    }
    .into()
}

fn build_from_fields(
    self_ident: Option<&Ident>,
    variant_name: &str,
    bms_display_path: &syn::Path,
    fields: &syn::Fields,
) -> TokenStream {
    let member_args: Vec<_> = match fields {
        syn::Fields::Named(fields_named) => fields_named.named.iter().map(|f| &f.attrs).collect(),
        syn::Fields::Unnamed(fields_unnamed) => {
            fields_unnamed.unnamed.iter().map(|f| &f.attrs).collect()
        }
        syn::Fields::Unit => Vec::new(),
    };

    let members = fields
        .members()
        .enumerate()
        .filter(|(i, _)| {
            // exclude fields with #[debug_with_type_info(skip)]
            member_args
                .get(*i)
                .map(|attrs| {
                    !attrs.iter().any(|attr| {
                        let mut is_skipped = false;
                        attr.path().is_ident("debug_with_type_info")
                            && attr
                                .parse_nested_meta(|meta| {
                                    is_skipped = meta.path.is_ident("skip");
                                    Ok(())
                                })
                                .is_ok()
                            && is_skipped
                    })
                })
                .unwrap_or(true)
        })
        .map(|(_, m)| match m {
            syn::Member::Named(ident) => {
                let field_name = ident.to_string();
                if let Some(self_ident) = self_ident {
                    quote::quote! {
                        .field(#field_name, &#self_ident.#ident)
                    }
                } else {
                    quote::quote! {
                        .field(#field_name, #ident)
                    }
                }
            }
            syn::Member::Unnamed(index) => {
                if let Some(self_ident) = self_ident {
                    quote::quote! {
                        .field(&#self_ident.#index)
                    }
                } else {
                    let underscore_index = format_ident!("_{}", index);
                    quote::quote! {
                        .field(#underscore_index)
                    }
                }
            }
        });

    let builder_method = Ident::new(
        match fields {
            syn::Fields::Named(_) => "debug_struct_with_type_info",
            syn::Fields::Unnamed(_) => "debug_tuple_with_type_info",
            syn::Fields::Unit => "debug_tuple_with_type_info",
        },
        Span::call_site(),
    );

    quote::quote! {
        #bms_display_path::DebugWithTypeInfoBuilder::#builder_method(f, #variant_name, type_info_provider)
            #(#members)*
            .finish()
    }
}
