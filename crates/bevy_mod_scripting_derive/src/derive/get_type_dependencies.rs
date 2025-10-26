use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::{DeriveInput, WhereClause, parse_quote, parse_quote_spanned};

/// Generate a GetTypeDependencies impl like below:
/// For type:
///
/// ```rust,ignore
/// #[derive(GetTypeDependencies)]
/// #[get_type_dependencies(remote)]
/// struct TargetType<T1: CustomBoundsT1, T2: CustomBoundsT2>{
///     ...
/// }
/// ```
///
/// ```rust,ignore
/// impl <T1,T2> GetTypeDependencies for TargetType
/// where
///     T1: GetTypeDependencies,
///     T2: GetTypeDependencies,
///     T1::Underlying: bevy::reflect::GetTypeRegistration + CustomBoundsT1,
///     T2::Underlying: bevy::reflect::GetTypeRegistration + CustomBoundsT2,    
/// {
///     type Underlying = TargetType<T1::Underlying, T2::Underlying>;
///     pub fn get_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {
///         T1::get_type_dependencies(registry);
///         T2::get_type_dependencies(registry);
///
///         registry.register::<TargetType<T1::Underlying, T2::Underlying>>();
///     }  
/// }
/// ```
fn get_type_dependencies_from_input(derive_input: DeriveInput) -> TokenStream {
    let args = match Args::parse(&derive_input.attrs) {
        Ok(args) => args,
        Err(error) => return error.to_compile_error(),
    };

    let bms_core = &args.bms_bindings_path;

    let (impl_generics, type_generics, impl_where) = derive_input.generics.split_for_impl();

    let name = &derive_input.ident;

    let generic_names = derive_input
        .generics
        .type_params()
        .map(|param| &param.ident)
        .collect::<Vec<_>>();

    let type_generics_underlying = if generic_names.is_empty() {
        Default::default()
    } else {
        quote_spanned! {derive_input.ident.span()=>
            <#( #generic_names::Underlying ),*>
        }
    };

    let underlying = if let Some(underlying) = args.underlying {
        underlying.to_token_stream()
    } else {
        quote_spanned! {derive_input.ident.span()=>
            #name #type_generics_underlying
        }
    };

    let mut impl_where: WhereClause = impl_where.cloned().unwrap_or_else(|| parse_quote! {where});
    let mut recursive_registrations = Vec::default();
    for param in derive_input.generics.type_params() {
        let param_name = &param.ident;
        if !args.dont_recurse {
            impl_where
                .predicates
                .push(parse_quote_spanned!(param.ident.span()=> #param_name: GetTypeDependencies));
            recursive_registrations.push(quote_spanned! {param.ident.span()=>
                <#param_name as GetTypeDependencies>::register_type_dependencies(registry);
            });

            impl_where.predicates.push(
                parse_quote_spanned!(param.ident.span()=> #param_name::Underlying: GetTypeRegistration),
            );
        } else {
            impl_where
                .predicates
                .push(parse_quote_spanned!(param.ident.span()=> #param_name: GetTypeRegistration))
        }
    }

    quote_spanned! {derive_input.ident.span()=>
        #[automatically_derived]
        #[allow(clippy::needless_lifetimes)]
        impl #impl_generics #bms_core::GetTypeDependencies for #name #type_generics #impl_where
        {
            type Underlying = #underlying;
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                #(#recursive_registrations)*

                registry.register::<#underlying>();
            }
        }
    }
}

pub fn get_type_dependencies(input: TokenStream) -> TokenStream {
    let derive_input: DeriveInput = match syn::parse2(input) {
        Ok(input) => input,
        Err(e) => return e.into_compile_error(),
    };

    get_type_dependencies_from_input(derive_input)
}

struct Args {
    bms_bindings_path: syn::Path,
    underlying: Option<syn::Type>,
    dont_recurse: bool,
    // bounds: Vec<syn::TypeParamBound>,
}

impl Args {
    fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut bms_bindings_path = parse_quote!(::bevy_mod_scripting::bindings);
        let mut underlying = None;
        let mut dont_recurse = false;

        for attr in attrs {
            // find attr with name `get_type_dependencies`
            // then parse its meta
            if attr.path().is_ident("get_type_dependencies") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("bms_bindings_path") {
                        let value = meta.value()?;
                        let string: syn::LitStr = value.parse()?;
                        bms_bindings_path = string.parse()?;
                        Ok(())
                    } else if meta.path.is_ident("underlying") {
                        let value = meta.value()?;
                        let string: syn::LitStr = value.parse()?;
                        underlying = Some(string.parse()?);
                        Ok(())
                    } else if meta.path.is_ident("dont_recurse") {
                        dont_recurse = true;
                        Ok(())
                    } else {
                        Err(syn::Error::new_spanned(
                            meta.path,
                            "unknown attribute, allowed: bms_bindings_path, underlying",
                        ))
                    }
                })?;
            }
        }

        Ok(Self {
            bms_bindings_path,
            underlying,
            dont_recurse,
        })
    }
}
