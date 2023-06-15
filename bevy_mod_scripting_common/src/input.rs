use std::collections::VecDeque;

use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Ident, Span};
use quote::{format_ident, ToTokens};
use syn::{
    parse::{Nothing, Parse},
    parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{And, Colon2, Gt, Lt, Mut},
    visit_mut::VisitMut,
    AngleBracketedGenericArguments, DataStruct, DeriveInput, Fields, GenericArgument, Meta,
    NestedMeta, PatType, Path, PathArguments, PathSegment, Receiver, Token, TraitItemMethod, Type,
    TypePath, TypeReference, TypeTuple,
};

use crate::utils::{attribute_to_string_lit, ident_to_type_path};

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

/// For types of the form `Option<T>` i.e. an outer identifier with a nested type inside angle brackets.
/// This type helps us destructure these patterns and unwrap/wrap proxies fully without dealing with the complicated full syn::Type enum
#[derive(Debug, Clone)]
pub struct UnitPath {
    pub ident: Ident,
    pub colon2_token: Option<Colon2>,
    pub lt_token: Lt,
    pub gt_token: Gt,
    pub inner: Box<SimpleType>,
}

/// Represents a type prefixed with an outer reference, e.g. `&T` or `&mut T`
#[derive(Debug, Clone)]
pub struct Reference {
    pub and_token: And,
    pub mutability: Option<Mut>,
    pub inner: Box<SimpleType>,
}

/// Represents the identifier part of a type which doubles as a proxy type, e.g. `T` in `Option<T>`
/// Stores both the proxied and proxy identifier i.e. `T` and `LuaT`
#[derive(Debug, Clone)]
pub struct ProxyType {
    pub proxied_ident: Ident,
    pub proxy_ident: Ident,
}

/// Proxies can also be returned in "container" types, such as:
/// - Option
/// - Result
/// - Vec
/// - Tuple
/// This type helps us destructure these patterns and unwrap/wrap proxies fully without dealing with the full syn::Type enum
#[derive(Debug, Clone)]
pub enum SimpleType {
    /// The unit type `()`
    Unit,
    /// A type of the form `Option<T>`
    UnitPath(UnitPath),
    /// A type with an outer reference, e.g. `&T` or `&mut T`
    Reference(Reference),
    /// A type which doubles as a proxy type, e.g. `T` in `Option<T>`
    ProxyType(ProxyType),
    /// A type which is not a proxy type, e.g. `i32`, required for composite types which can contain both proxy and non-proxy types
    /// like tuples: `(i32, T)`
    Type(syn::Type),
}

impl SimpleType {
    /// Constructs a new SimpleProxyType from a `syn::FnArg` using contextual resolution for receivers such as `Self` and `self` with the proxied type identifier given.
    pub fn new_from_fn_arg(
        proxy_prefix: &'static str,
        arg: &syn::FnArg,
        proxied_type_identifier: &Ident,
    ) -> Result<SimpleType, (Span, String)> {
        match arg {
            syn::FnArg::Receiver(Receiver {
                reference,
                mutability,
                ..
            }) => match reference {
                Some((and, ..)) => {
                    let mutability = mutability.as_ref().copied();
                    let inner = Box::new(SimpleType::new_from_contextual_type(
                        proxy_prefix,
                        &Type::Path(ident_to_type_path(proxied_type_identifier.clone())),
                        proxied_type_identifier,
                    )?);
                    Ok(Self::Reference(Reference {
                        and_token: *and,
                        mutability,
                        inner,
                    }))
                }
                None => Self::new_from_contextual_type(
                    proxy_prefix,
                    &Type::Path(ident_to_type_path(proxied_type_identifier.clone())),
                    proxied_type_identifier,
                ),
            },
            syn::FnArg::Typed(PatType { ty, .. }) => {
                Self::new_from_contextual_type(proxy_prefix, ty.as_ref(), proxied_type_identifier)
            }
        }
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will cause an error
    /// to be returned.
    pub fn new_from_fully_specified_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
    ) -> Result<SimpleType, (Span, String)> {
        Self::new_from_type(proxy_prefix, proxied_type, None)
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will be replaced
    /// with the given identifier prefixed with the proxy_prefix
    pub fn new_from_contextual_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_identifier: &Ident,
    ) -> Result<SimpleType, (Span, String)> {
        Self::new_from_type(proxy_prefix, proxied_type, Some(proxied_type_identifier))
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, if `proxied_type_identifier` is given then contextual
    /// receivers such as `Self` and `self` will be replaced with the given identifier prefixed with the proxy_prefix, otherwise an error will be returned.
    fn new_from_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_identifier: Option<&Ident>,
    ) -> Result<SimpleType, (Span, String)> {
        match proxied_type {
            Type::Path(p) if p.path.is_ident("self") || p.path.is_ident("Self") => {
                let proxied_ident = proxied_type_identifier.ok_or_else(|| {
                    (
                        proxied_type.span(),
                        "Did not expect contextual receiver in constructing simple proxy type"
                            .to_owned(),
                    )
                })?;

                Ok(SimpleType::ProxyType(ProxyType {
                    proxied_ident: proxied_ident.clone(),
                    proxy_ident: format_ident!("{}{}", proxy_prefix, proxied_ident),
                }))
            }
            Type::Path(p) if !p.path.segments.is_empty() => {
                let last_segment = p.path.segments.last().unwrap();
                if last_segment.arguments.is_empty() {
                    return Ok(SimpleType::ProxyType(ProxyType {
                        proxied_ident: last_segment.ident.clone(),
                        proxy_ident: format_ident!("{}{}", proxy_prefix, last_segment.ident),
                    }));
                } else if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(arg_type) = args.args.first().unwrap() {
                            let inner = Box::new(Self::new_from_type(
                                proxy_prefix,
                                arg_type,
                                proxied_type_identifier,
                            )?);
                            return Ok(SimpleType::UnitPath(UnitPath {
                                ident: last_segment.ident.clone(),
                                colon2_token: args.colon2_token,
                                lt_token: args.lt_token,
                                gt_token: args.gt_token,
                                inner,
                            }));
                        }
                    }
                }
                Err((proxied_type.span(), "Unsupported type".to_owned()))
            }
            Type::Reference(tr) => Ok(SimpleType::Reference(Reference {
                and_token: tr.and_token,
                mutability: tr.mutability,
                inner: Box::new(Self::new_from_type(
                    proxy_prefix,
                    &tr.elem,
                    proxied_type_identifier,
                )?),
            })),
            Type::Tuple(TypeTuple { elems , ..}) if elems.is_empty() => {
                Ok(SimpleType::Unit)
            },
            _ => Err((
                proxied_type.span(),
                format!("Expected simple type with one identifier and possible reference for proxy type, got {}", proxied_type.to_token_stream()),
            )),
        }
    }

    /// Returns true if the type has an outer reference, (e.g. `&Type`)
    pub fn has_outer_ref(&self) -> bool {
        matches!(self, SimpleType::Reference { .. })
    }

    pub fn has_outer_mut_ref(&self) -> bool {
        matches!(self, SimpleType::Reference (Reference{ mutability, .. }) if mutability.is_some())
    }

    /// Strips outer references and returns the type if any are present
    pub fn construct_proxy_type_without_outer_ref(&self) -> Type {
        match self {
            SimpleType::Reference(Reference { inner, .. }) => inner.construct_proxy_type(),
            other => other.construct_proxy_type(),
        }
    }

    /// Constructs a syn::Type from this SimpleProxyType, representing the proxied type
    pub fn construct_proxied_type(&self) -> Type {
        self.construct_type_with_proxy_conversion(|proxied_ident, _| {
            Type::Path(ident_to_type_path(proxied_ident.clone()))
        })
    }

    /// Constructs a syn::Type from this SimpleProxyType, representing the proxy type
    pub fn construct_proxy_type(&self) -> Type {
        self.construct_type_with_proxy_conversion(|_, proxy_ident| {
            Type::Path(ident_to_type_path(proxy_ident.clone()))
        })
    }

    /// A helper function for constructing a syn::Type from this SimpleProxyType, using the given function to convert
    /// the proxied type identifier and proxy type identifier into a syn::Type
    fn construct_type_with_proxy_conversion<F: Fn(&Ident, &Ident) -> Type>(
        &self,
        proxy_conversion: F,
    ) -> Type {
        match self {
            SimpleType::UnitPath(UnitPath {
                ident,
                colon2_token,
                lt_token,
                gt_token,
                inner,
            }) => Type::Path(TypePath {
                qself: None,
                path: PathSegment {
                    ident: ident.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: *colon2_token,
                        lt_token: *lt_token,
                        args: Punctuated::from_iter(
                            [GenericArgument::Type(
                                inner.construct_type_with_proxy_conversion(proxy_conversion),
                            )]
                            .into_iter(),
                        ),
                        gt_token: *gt_token,
                    }),
                }
                .into(),
            }),
            SimpleType::Reference(Reference {
                and_token,
                mutability,
                inner,
            }) => Type::Reference(TypeReference {
                and_token: *and_token,
                lifetime: None,
                mutability: *mutability,
                elem: Box::new(inner.construct_type_with_proxy_conversion(proxy_conversion)),
            }),
            SimpleType::Unit => Type::Tuple(TypeTuple {
                paren_token: Default::default(),
                elems: Default::default(),
            }),
            SimpleType::Type(_) => todo!(),
            SimpleType::ProxyType(ProxyType {
                proxied_ident,
                proxy_ident,
            }) => proxy_conversion(proxied_ident, proxy_ident),
        }
    }
}

pub trait VisitSimpleType<T> {
    fn visit_simple_type(&self, simple_type: &SimpleType) -> T {
        match simple_type {
            SimpleType::Unit => self.visit_unit(),
            SimpleType::UnitPath(unit_path) => self.visit_unit_path(unit_path),
            SimpleType::Reference(reference) => self.visit_reference(reference),
            SimpleType::ProxyType(proxy_type) => self.visit_proxy_type(proxy_type),
            SimpleType::Type(_type) => self.visit_type(_type),
        }
    }
    fn visit_unit_path(&self, unit_path: &UnitPath) -> T {
        self.visit_simple_type(&unit_path.inner)
    }
    fn visit_reference(&self, reference: &Reference) -> T {
        self.visit_simple_type(&reference.inner)
    }
    fn visit_unit(&self) -> T;
    fn visit_proxy_type(&self, proxy_type: &ProxyType) -> T;
    fn visit_type(&self, _type: &Type) -> T;
}

/// Attributes relating to the proxy as a whole
#[derive(Debug)]
pub struct ProxyMeta {
    /// the identifier of the proxied type
    pub proxied_name: Ident,
    /// the identifier for the proxy type
    pub proxy_name: Option<Ident>,
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
        let mut proxy_name = None;
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
                                proxy_name = Some(Ident::new(&_str.value(), _str.span()))
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

/// Replaces every occurence of an identifier with
/// the given string while preserving the original span
pub struct IdentifierRenamingVisitor<'a> {
    pub target: &'a str,
    pub replacement: &'a str,
}

impl VisitMut for IdentifierRenamingVisitor<'_> {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        if *i == self.target {
            *i = Ident::new(self.replacement, i.span());
        }
    }
}
