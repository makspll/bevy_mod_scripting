#![allow(unused_imports)] // TODO: Remove this later

use bevy_mod_scripting_common::{implementor::WrapperFunction, input::ProxyTypeNameMeta};
use proc_macro::Ident;
use proc_macro2::TokenStream;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Mut,
    Attribute, DeriveInput, Error, FnArg, Lit, Meta, MetaList, NestedMeta, Pat, PatType, Path,
    PathArguments, PathSegment, TraitItemMethod, Type, TypePath, PatIdent, visit_mut::VisitMut};

use proc_macro_error::{abort, emit_error, proc_macro_error};

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    *,
};
impl WrapperFunction for RhaiMethod {}

const PROXY_PREFIX: &str = "Rhai";

#[derive(Debug)]
struct FunctionMeta<'a> {
    public: bool,
    name: &'a Ident,
    body: &'a TraitItemMethod,
    fn_type: FunctionType,
    arg_meta: Vec<FunctionArgMeta>,
    output_meta: Option<FunctionArgMeta>,
}

impl FunctionMeta<'_> {
    fn new<'a>(
        proxy_type_name_meta: &ProxyTypeNameMeta,
        name: &'a Ident,
        body: &'a TraitItemMethod,
    ) -> Self {
         let meta = body
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("rhai"))
            .unwrap_or_else(|| {
                abort!(
                    body,
                    "Rhai proxy functions require `rhai` meta attribute like so: `#[rhai()]`"
                )
            })
            .parse_meta()
            .unwrap_or_else(|err| abort!(body, err));
        let mut output_attrs = Vec::default();

        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let mut fn_type = FunctionType::Function;
                
                nested
                    .iter()
                    .for_each(|attr|{
                        if let NestedMeta::Meta(Meta::Path(p)) = attr {
                            let attr_str = p.get_ident().map(Ident::to_string).unwrap_or_default();
                            if let Ok(_fn_type) = FunctionType::from_str(&attr_str) {
                                fn_type = _fn_type;
                                return;
                            }
                        } else if let NestedMeta::Meta(Meta::List(list)) = attr {
                            if list.path.is_ident("output") {
                                for attr in list.nested.iter() {
                                    output_attrs.push(parse_quote!(#[#attr]))
                                }
                                return;
                            }
                        }
    
                        emit_error!(attr, "unknown or malformed rhai proxy function attribute. Allowed attributes include: {}",
                            FunctionType::iterator()
                            .map(FunctionType::as_ident_str).collect::<Vec<_>>().join(","));
                    });
                let fn_meta = FunctionMeta {
                    name,
                    body,
                    fn_type,
                    arg_meta: body
                        .sig
                        .inputs
                        .iter()
                        .map(|arg| FunctionArgMeta::new_from_fn_arg(proxy_type_name_meta, arg))
                        .collect(),
                    output_meta: match &body.sig.output {
                        syn::ReturnType::Default => None,
                        syn::ReturnType::Type(_, t) => Some(FunctionArgMeta::new_from_type(
                            proxy_type_name_meta,
                            format_ident!("out"),
                            t,
                            output_attrs,
                        )),
                    },
                };
                // validate the function against it's meta
                fn_meta.validate_function_definition(body);
    
                fn_meta
            }
            _ => abort!(
                body,
                "`rhai` attribute must be a meta list of the form: `rhai(elem1,elem2)`"
            ),
        }


    }
}

pub(crate) struct RhaiMethod {
    pub method: ItemFn,
}

impl Parse for RhaiMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            method: input.parse()?,
        })
    }
}

impl ToTokens for RhaiMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let method = &self.method;
        tokens.extend(quote::quote! {
            #method
        })
    }
}


#[derive(Debug)]
struct FunctionArgMeta {
    is_receiver: bool,
    is_ref: bool,
    is_mut_ref: bool,
    is_a_rhai_proxy: bool,
    mutable: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    arg_type: Type,
    arg_name: Ident,
    span: Span,
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FunctionType {
    Function,
    MetaFunction,
    Method,
    MetaMethod,
    MutableFunction,
    MutableMetaFunction,
    MutatingMethod,
    MutatingMetaMethod,
}


