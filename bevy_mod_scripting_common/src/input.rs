use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Ident, Span};
use quote::{format_ident, ToTokens, TokenStreamExt};
use syn::{
    bracketed, parenthesized,
    parse::{Nothing, Parse, ParseBuffer},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, DataStruct, DeriveInput, Fields, Meta, NestedMeta, TraitItemMethod, Type,
};

use crate::utils::attribute_to_string_lit;

/// Convenience structure for holding data relevant to proxy generation
pub struct DeriveMeta {}

#[derive(Debug)]
pub struct Language {
    pub name: String,
    pub on_feature: Option<String>,
}

/// Language settings for proxies
#[derive(Default, Debug)]
pub struct LanguageMeta {
    pub languages: Vec<Language>,
}

impl TryFrom<syn::MetaList> for LanguageMeta {
    type Error = syn::Error;

    fn try_from(list: syn::MetaList) -> Result<Self, Self::Error> {
        let mut languages: Vec<Language> = Default::default();

        for nested_meta in list.nested.into_iter() {
            match nested_meta {
                syn::NestedMeta::Lit(syn::Lit::Str(_str)) => {
                    let mut name = _str.value();
                    let mut on_feature = None;
                    if let Some(postfix) = name.strip_prefix("on_feature(") {
                        if let Some(middle) = postfix.strip_suffix(')') {
                            name = middle.to_owned();
                            on_feature = Some(name.clone());
                        }
                    }
                    languages.push(Language { name, on_feature })
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        nested_meta,
                        "Expected language name or wrapped language name",
                    ))
                }
            };
        }

        Ok(Self { languages })
    }
}

/// Flags which detail required functionality or additional derivation requirements
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum DeriveFlag {
    Debug,
    Display,
    Clone,
}

impl TryFrom<syn::NestedMeta> for DeriveFlag {
    type Error = syn::Error;

    fn try_from(value: syn::NestedMeta) -> Result<Self, Self::Error> {
        match value {
            NestedMeta::Meta(Meta::Path(p)) if p.is_ident("Clone") => Ok(Self::Clone),
            NestedMeta::Meta(Meta::Path(p)) if p.is_ident("Debug") => Ok(Self::Debug),
            NestedMeta::Meta(Meta::Path(p)) if p.is_ident("Display") => Ok(Self::Display),
            _ => Err(syn::Error::new_spanned(
                value,
                "Expected one of `Debug`, `Display`, `Clone`",
            )),
        }
    }
}

/// Container for proxy flags
#[derive(Debug, Default)]
pub struct ProxyFlags {
    pub flags: IndexSet<DeriveFlag>,
}

impl TryFrom<syn::MetaList> for ProxyFlags {
    type Error = syn::Error;

    fn try_from(meta_list: syn::MetaList) -> Result<Self, Self::Error> {
        let mut derive_flags: IndexSet<DeriveFlag> = Default::default();

        for nested_meta in meta_list.nested {
            let span = nested_meta.span();
            let flag: DeriveFlag = nested_meta.try_into()?;
            if derive_flags.contains(&flag) {
                return Err(syn::Error::new(
                    span,
                    "This flag was already defined, remove duplicate flag",
                ));
            } else {
                derive_flags.insert(flag);
            }
        }
        Ok(Self {
            flags: derive_flags,
        })
    }
}

pub(crate) struct ZeroOrManyTerminated<T: Parse, S: Parse>(Punctuated<T, S>);

impl<T: Parse, S: Parse> Parse for ZeroOrManyTerminated<T, S> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::<T, S>::parse_terminated(input)?))
    }
}

/// Detailed information about the proxy, here we can access fields/variants etc.
#[derive(Debug)]
pub enum ProxyData {
    Struct { fields: Fields },
}

pub struct ProxyTypeNameMeta {
    proxied_type: Type,
    proxied_type_ident: Ident,
    proxy_type_ident: Ident,
}

impl ProxyTypeNameMeta {
    pub fn new(proxied_type: Type, proxy_prefix: &'static str) -> Self {
        let proxied_type_ident = match &proxied_type {
            Type::Path(p) if p.path.get_ident().is_some() => p.path.get_ident().unwrap().clone(),
            _ => panic!(
                "Expected identifier for proxied type, instead got: {}",
                proxied_type.to_token_stream()
            ),
        };

        Self {
            proxy_type_ident: Self::proxy_type_to_ident(
                proxy_prefix,
                &proxied_type,
                &proxied_type_ident,
            )
            .unwrap(),
            proxied_type_ident,
            proxied_type,
        }
    }

    pub fn get_proxied_type_identifier(&self) -> &Ident {
        &self.proxied_type_ident
    }

    pub fn get_proxy_type_identifier(&self) -> &Ident {
        &self.proxy_type_ident
    }

    pub fn get_proxied_type(&self) -> &Type {
        &self.proxied_type
    }

    /// Converts a type representing a proxy (simple type with possible outer reference) without generics
    /// into the identifier representing the type. in addition, replaces Self and self occurences with the given ident
    /// uses `generate_automatic_proxy_name` in the case that the identifier is anything but `self` or `Self`.
    ///
    /// For example, `MyType` will be translated to `LuaMyType` if given the "Lua" prefix
    /// `Self` will be translated to the proxied_type_identifier with the given prefix
    /// etc.
    pub fn proxy_type_to_ident(
        proxy_prefix: &'static str,
        proxy_type: &Type,
        proxied_type_identifier: &Ident,
    ) -> Result<Ident, (Span, String)> {
        match proxy_type {
            Type::Path(p) if p.path.is_ident("self") || p.path.is_ident("Self") => Ok(
                format_ident!("{}{}", proxy_prefix, proxied_type_identifier.clone()),
            ),
            Type::Path(p) if p.path.get_ident().is_some() => Ok(format_ident!(
                "{}{}",
                proxy_prefix,
                p.path.get_ident().unwrap()
            )),
            Type::Reference(tr) => {
                Self::proxy_type_to_ident(proxy_prefix, &tr.elem, proxied_type_identifier)
            }
            _ => Err((
                proxy_type.span(),
                "Expected simple type with one identifier and possible reference for proxy type"
                    .to_string(),
            )),
        }
    }
}

/// Attributes relating to the proxy as a whole
#[derive(Debug)]
pub struct ProxyMeta {
    /// the identifier of the proxied type
    pub proxied_name: Ident,
    /// the identifier for the proxy type
    pub proxy_name: Ident,
    /// language derivation settings
    pub language_meta: LanguageMeta,
    /// additional flags for the proxy
    pub proxy_flags: ProxyFlags,
    /// functions to be proxied
    pub functions: IndexMap<Ident, TraitItemMethod>,
    /// the inner type data
    pub data: ProxyData,
    /// the derive input span
    pub span: Span,
    /// docstrings
    pub docstrings: Vec<proc_macro2::TokenStream>,
}

impl TryFrom<DeriveInput> for ProxyMeta {
    type Error = syn::Error;

    fn try_from(derive_input: DeriveInput) -> Result<Self, Self::Error> {
        let mut proxy_name = derive_input.ident.clone();
        let span = derive_input.span();

        // helper for collecting errors which are not fatal to the logic flow
        // simplifies logical flow
        let mut accumulated_errors = VecDeque::<Self::Error>::default();

        let docstrings = derive_input
            .attrs
            .iter()
            .map(attribute_to_string_lit)
            .filter(|s| !s.is_empty())
            .collect();

        let proxy_meta = derive_input
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("proxy"))
            .ok_or_else(|| syn::Error::new_spanned(&derive_input, "`proxy` meta missing"))
            .and_then(|attr| attr.parse_meta())?;

        let functions = derive_input
            .attrs
            .into_iter()
            .find(|attr| attr.path.is_ident("functions"))
            .map_or(Ok(IndexMap::default()), |attr| {
                attr.parse_args::<ZeroOrManyTerminated<TraitItemMethod, Nothing>>()
                    .map(|fns| {
                        let mut fn_map = IndexMap::default();
                        fns.0
                            .into_iter()
                            .map(|_fn| (_fn.sig.ident.clone(), _fn))
                            .for_each(|(name, body)| {
                                if let Some(old_val) = fn_map.insert(name, body) {
                                    accumulated_errors.push_back(syn::Error::new_spanned(
                                        old_val.sig.ident, // old == new ident
                                        "duplicate Lua proxy function, re-name this function",
                                    ))
                                }
                            });
                        fn_map
                    })
            })?;

        let mut language_meta = Default::default();
        let mut proxy_flags: ProxyFlags = Default::default();

        if let syn::Meta::List(list) = proxy_meta {
            for attr in list.nested.into_iter() {
                match attr {
                    NestedMeta::Meta(Meta::NameValue(pair)) => {
                        let ident = pair.path.get_ident().ok_or_else(|| {
                            syn::Error::new_spanned(&pair, "Keys must be identifiers")
                        })?;

                        match (ident.to_string().as_str(), pair.lit) {
                            ("name", syn::Lit::Str(_str)) => {
                                proxy_name = Ident::new(&_str.value(), _str.span())
                            }
                            _ => {
                                return Err(syn::Error::new_spanned(ident, "Unrecognized argument"))
                            }
                        }
                    }
                    NestedMeta::Meta(Meta::List(list)) => {
                        let ident = list
                            .path
                            .get_ident()
                            .ok_or_else(|| syn::Error::new_spanned(&list, "Expected identifier"))?;

                        match ident.to_string().as_str() {
                            "languages" => language_meta = list.try_into()?,
                            "derive" => proxy_flags = list.try_into()?,
                            _ => return Err(syn::Error::new_spanned(list, "")),
                        }
                    }
                    _ => return Err(syn::Error::new_spanned(attr, "Expected key value pair")),
                }
            }
        } else {
            return Err(syn::Error::new_spanned(
                proxy_meta,
                "Expected list of key value pairs",
            ));
        }

        let data = match derive_input.data {
            syn::Data::Struct(DataStruct { fields, .. }) => ProxyData::Struct { fields },
            syn::Data::Enum(_) => todo!(),
            syn::Data::Union(_) => todo!(),
        };

        let proxied_name = derive_input.ident;

        if let Some(first_err) = accumulated_errors.pop_front() {
            return Err(first_err);
        }

        Ok(ProxyMeta {
            proxied_name,
            proxy_name,
            proxy_flags,
            language_meta,
            functions,
            data,
            span,
            docstrings,
        })
    }
}
