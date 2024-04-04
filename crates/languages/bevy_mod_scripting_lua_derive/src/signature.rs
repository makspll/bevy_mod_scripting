use std::collections::HashMap;

use bevy_mod_scripting_common::input::SimpleType;
use darling::FromAttributes;
use proc_macro2::{Ident, Span};
use syn::{
    spanned::Spanned, token::Mut, Attribute, FnArg, Pat, PatIdent, PatType, Path, ReturnType, Type,
    TypeTuple,
};

use crate::{
    arg::{Arg, ArgAttributes},
    PROXY_PREFIX, RAW_OUT_ALIAS, SELF_ALIAS,
};

/// Describes the functional signature of a function from the `functions[..]` list
#[derive(Debug)]
pub struct Signature {
    pub inputs: Vec<Arg>,
    pub output: Arg,
    pub span: Span,
}

impl Signature {
    /// Creates a new signature struct
    /// if in_raw_function will set is_raw on all arguments and outputs to true
    /// if is_field_setter, output_attrs will be applied to the third argument of the function if it exists (the first non self or ctxt arg)
    pub fn new(
        proxied_type_path: Path,
        sig: syn::Signature,
        in_raw_function: bool,
        output_attrs: Vec<Attribute>,
    ) -> darling::Result<Self> {
        // convert output to FnArg
        let output_arg_name = Ident::new(RAW_OUT_ALIAS, sig.output.span());
        let span = sig.span();
        // if no return type specified use `()`
        let output_type = match sig.output {
            ReturnType::Default => Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Default::default(),
            }),
            ReturnType::Type(_, ty) => *ty,
        };

        let inputs = sig
            .inputs
            .into_iter()
            .map(|arg| Self::convert_fn_arg(arg, &proxied_type_path, in_raw_function))
            .collect::<darling::Result<Vec<_>>>()?;

        // convert to Arg structs
        let output = Self::convert_type(
            output_type,
            &proxied_type_path,
            in_raw_function,
            output_attrs,
            output_arg_name,
            None,
        )?;

        Ok(Self {
            inputs,
            output,
            span,
        })
    }

    /// Convert a function argument into custom Arg struct by converting the type to SimpleType and parsing attributes
    fn convert_fn_arg(
        arg: FnArg,
        proxied_type_path: &Path,
        in_raw_function: bool,
    ) -> darling::Result<Arg> {
        let type_map = HashMap::from_iter([(
            proxied_type_path.segments.last().unwrap().clone().ident,
            None,
        )]);

        Ok(match arg {
            FnArg::Receiver(ref receiver) => {
                let type_ =
                    SimpleType::new_from_fn_arg(PROXY_PREFIX, &arg, proxied_type_path, &type_map)?;
                let attrs = ArgAttributes::from_attributes(&receiver.attrs)?;
                Arg::new(
                    attrs,
                    Ident::new(SELF_ALIAS, receiver.span()),
                    receiver.mutability,
                    type_,
                    in_raw_function,
                )
            }
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                let (mutability, arg_name) = match pat.as_ref() {
                    Pat::Ident(PatIdent {
                        mutability, ident, ..
                    }) => (mutability, ident),
                    _ => return Err(darling::Error::custom("Unsupported parameter pattern")),
                };

                Self::convert_type(
                    *ty,
                    proxied_type_path,
                    in_raw_function,
                    attrs,
                    arg_name.clone(),
                    *mutability,
                )?
            }
        })
    }

    /// Convert a type corresponding to an argument into an Arg struct by converting it to a Simple type and parsing the given attributes
    fn convert_type(
        ty: Type,
        proxied_type_path: &Path,
        in_raw_function: bool,
        attrs: Vec<Attribute>,
        arg_name: Ident,
        mutability: Option<Mut>,
    ) -> darling::Result<Arg> {
        let mut type_map = HashMap::from_iter([(
            proxied_type_path.segments.last().unwrap().clone().ident,
            None,
        )]);
        let is_proxy = attrs.iter().any(|a| a.path().is_ident("proxy"));
        let attrs = ArgAttributes::from_attributes(&attrs)?;
        let type_ = if is_proxy && attrs.map.is_empty() {
            SimpleType::new_from_contextual_type_proxy_all(PROXY_PREFIX, &ty, proxied_type_path)?
        } else {
            type_map.extend(attrs.map.iter().map(|(a, b)| (a.clone(), Some(b.clone()))));
            SimpleType::new_from_contextual_type(PROXY_PREFIX, &ty, proxied_type_path, &type_map)?
        };

        Ok(Arg::new(
            attrs,
            arg_name,
            mutability,
            type_,
            in_raw_function,
        ))
    }
}
