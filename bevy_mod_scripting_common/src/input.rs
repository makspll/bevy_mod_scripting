use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use indexmap::{IndexMap, IndexSet};
use proc_macro2::{Ident, Span};
use quote::{format_ident, ToTokens};
use strum::{Display, EnumString};
use syn::{
    parse::{Nothing, Parse},
    punctuated::Punctuated,
    spanned::Spanned,
    token::{And, Colon2, Gt, Lt, Mut},
    visit_mut::VisitMut,
    AngleBracketedGenericArguments, DataStruct, DeriveInput, Error, Fields, GenericArgument, Meta,
    NestedMeta, PatType, PathArguments, PathSegment, Receiver, TraitItemMethod, Type, TypePath,
    TypeReference, TypeTuple,
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

/// Enumeration of commonly encountered Rust standard library type identifiers which can be effectively proxied in Lua,
/// These types are `container` types, which wrap other types rather than standalone and literal types.
#[derive(EnumString, Debug, Clone, Copy, Display, PartialEq, Eq)]
pub enum StdTypeIdent {
    Option,
    Result,
    Vec,
    Box,
    Rc,
    Arc,
    Cow,
    Cell,
    RefCell,
    Mutex,
    RwLock,
    Pin,
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
    pub std_type_ident: Option<StdTypeIdent>,
    pub ident: Ident,
    pub colon2_token: Option<Colon2>,
    pub lt_token: Lt,
    pub gt_token: Gt,
    pub inner: Box<SimpleType>,
}

/// For types of the form `Result<L, R>` i.e. an outer identifier with two nested types inside angle brackets.
#[derive(Debug, Clone)]
pub struct DuoPath {
    pub std_type_ident: Option<StdTypeIdent>,
    pub ident: Ident,
    pub colon2_token: Option<Colon2>,
    pub lt_token: Lt,
    pub gt_token: Gt,
    pub left: Box<SimpleType>,
    pub right: Box<SimpleType>,
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
    /// A type of the form `Result<T, E>` a list containing two elements is referred to as a
    DuoPath(DuoPath),

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
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> Result<SimpleType, Error> {
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
                        proxied_to_proxy_ident_map,
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
                    proxied_to_proxy_ident_map,
                ),
            },
            syn::FnArg::Typed(PatType { ty, .. }) => Self::new_from_contextual_type(
                proxy_prefix,
                ty.as_ref(),
                proxied_type_identifier,
                proxied_to_proxy_ident_map,
            ),
        }
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will cause an error
    /// to be returned.
    pub fn new_from_fully_specified_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> Result<SimpleType, Error> {
        Self::new_from_type(proxy_prefix, proxied_type, None, proxied_to_proxy_ident_map)
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will be replaced
    /// with the given identifier prefixed with the proxy_prefix
    pub fn new_from_contextual_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_identifier: &Ident,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> Result<SimpleType, Error> {
        Self::new_from_type(
            proxy_prefix,
            proxied_type,
            Some(proxied_type_identifier),
            proxied_to_proxy_ident_map,
        )
    }

    /// Builds a SimpleType::ProxyType or SimpleType::Type depending on the passed data,
    /// - if the proxied type identifier has a Some value in the proxied_to_proxy_ident_map map then the proxy_ident will be set to the value in the map,
    /// - if it has a None value it will be set to the proxied type identifier prefixed with the proxy_prefix,
    /// - If it's not in the map it's built as a SimpleType::Type
    fn new_proxied_type_or_type(
        proxy_prefix: &'static str,
        proxied_ident: &Ident,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> SimpleType {
        if let Some((original_ident, replacement_ident)) =
            proxied_to_proxy_ident_map.get_key_value(proxied_ident)
        {
            let proxy_ident = replacement_ident
                .as_ref()
                .unwrap_or(&format_ident!("{proxy_prefix}{original_ident}"))
                .clone();

            SimpleType::ProxyType(ProxyType {
                proxied_ident: original_ident.clone(),
                proxy_ident,
            })
        } else {
            Self::Type(syn::Type::Path(ident_to_type_path(proxied_ident.clone())))
        }
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, if `proxied_type_identifier` is given then contextual
    /// receivers such as `Self` and `self` will be replaced with the given identifier prefixed with the proxy_prefix, otherwise an error will be returned.
    /// types with base identifiers not in the proxied_to_proxy_ident_map list are treated as non-proxy types and will be wrapped in a SimpleProxyType::Type
    fn new_from_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_identifier: Option<&Ident>,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> Result<SimpleType, Error> {
        match proxied_type {
            Type::Path(p) if p.path.is_ident("self") || p.path.is_ident("Self") => {
                let proxied_ident: &Ident = proxied_type_identifier.ok_or_else(|| {
                    Error::new_spanned(
                        proxied_type,
                        "Did not expect contextual receiver in constructing simple proxy type"
                            .to_owned(),
                    )
                })?;
                Ok(Self::new_proxied_type_or_type(proxy_prefix, proxied_ident, proxied_to_proxy_ident_map))
            }
            Type::Path(p) if !p.path.segments.is_empty() => {
                let last_segment = p.path.segments.last().unwrap();
                if last_segment.arguments.is_empty() {
                    return Ok(Self::new_proxied_type_or_type(proxy_prefix,&last_segment.ident, proxied_to_proxy_ident_map));
                } else if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(arg_type) = args.args.first().unwrap() {
                            let inner = Box::new(Self::new_from_type(
                                proxy_prefix,
                                arg_type,
                                proxied_type_identifier,
                                proxied_to_proxy_ident_map
                            )?);
                            return Ok(SimpleType::UnitPath(UnitPath {
                                std_type_ident: StdTypeIdent::from_str(&last_segment.ident.to_string()).ok(),
                                ident: last_segment.ident.clone(),
                                colon2_token: args.colon2_token,
                                lt_token: args.lt_token,
                                gt_token: args.gt_token,
                                inner,
                            }));
                        }
                    } else if args.args.len() == 2 {
                        let mut args_iter = args.args.iter();
                        if let (GenericArgument::Type(left), GenericArgument::Type(right)) =
                            (args_iter.next().unwrap(), args_iter.next().unwrap())
                        {
                            let left = Box::new(Self::new_from_type(
                                proxy_prefix,
                                left,
                                proxied_type_identifier,
                                proxied_to_proxy_ident_map
                            )?);
                            let right = Box::new(Self::new_from_type(
                                proxy_prefix,
                                right,
                                proxied_type_identifier,
                                proxied_to_proxy_ident_map
                            )?);
                            return Ok(SimpleType::DuoPath(DuoPath {
                                std_type_ident: StdTypeIdent::from_str(&last_segment.ident.to_string()).ok(),
                                ident: last_segment.ident.clone(),
                                colon2_token: args.colon2_token,
                                lt_token: args.lt_token,
                                gt_token: args.gt_token,
                                left,
                                right,
                            }));
                        }
                    }
                }
                Err(Error::new_spanned(proxied_type, "Unsupported type".to_owned()))
            }
            Type::Reference(tr) => Ok(SimpleType::Reference(Reference {
                and_token: tr.and_token,
                mutability: tr.mutability,
                inner: Box::new(Self::new_from_type(
                    proxy_prefix,
                    &tr.elem,
                    proxied_type_identifier,
                    proxied_to_proxy_ident_map
                )?),
            })),
            Type::Infer(_) => Ok(SimpleType::Type(proxied_type.clone())),
            Type::Tuple(TypeTuple { elems , ..}) if elems.is_empty() => {
                Ok(SimpleType::Unit)
            },
            _ => Err(Error::new_spanned(
                proxied_type,
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

    /// Returns true if the type has an inner reference, (e.g. `Type<&T>`)
    pub fn has_ref(&self) -> bool {
        match self {
            SimpleType::Unit => false,
            SimpleType::UnitPath(UnitPath { inner, .. }) => inner.has_ref(),
            SimpleType::DuoPath(DuoPath { left, right, .. }) => left.has_ref() || right.has_ref(),
            SimpleType::Reference(_) => true,
            SimpleType::ProxyType(ProxyType { .. }) => false,
            SimpleType::Type(_) => false,
        }
    }

    /// Returns true if the type has an inner proxy type
    pub fn contains_proxy_type(&self) -> bool {
        match self {
            SimpleType::Unit => false,
            SimpleType::UnitPath(UnitPath { inner, .. }) => inner.contains_proxy_type(),
            SimpleType::DuoPath(DuoPath { left, right, .. }) => {
                left.contains_proxy_type() || right.contains_proxy_type()
            }
            SimpleType::Reference(Reference { inner, .. }) => inner.contains_proxy_type(),
            SimpleType::ProxyType(_) => true,
            SimpleType::Type(_) => false,
        }
    }
}

pub trait VisitSimpleType<T>
where
    T: std::fmt::Debug,
{
    fn visit(&mut self, simple_type: &SimpleType) -> T {
        self.visit_simple_type(simple_type, false)
    }

    fn visit_simple_type(&mut self, simple_type: &SimpleType, is_child_of_reference: bool) -> T {
        match simple_type {
            SimpleType::Unit => self.visit_unit(is_child_of_reference),
            SimpleType::UnitPath(unit_path) => {
                self.visit_unit_path(unit_path, is_child_of_reference)
            }
            SimpleType::DuoPath(duo_path) => self.visit_duo_path(duo_path, is_child_of_reference),
            SimpleType::Reference(reference) => {
                self.visit_reference(reference, is_child_of_reference)
            }
            SimpleType::ProxyType(proxy_type) => {
                self.visit_proxy_type(proxy_type, is_child_of_reference)
            }
            SimpleType::Type(_type) => self.visit_type(_type, is_child_of_reference),
        }
    }
    fn visit_unit_path(&mut self, unit_path: &UnitPath, _is_child_of_reference: bool) -> T {
        self.visit_simple_type(&unit_path.inner, false)
    }

    fn visit_duo_path(&mut self, duo_path: &DuoPath, _is_child_of_reference: bool) -> T {
        self.visit_simple_type(&duo_path.left, false);
        self.visit_simple_type(&duo_path.right, false)
    }

    fn visit_reference(&mut self, reference: &Reference, _is_child_of_reference: bool) -> T {
        self.visit_simple_type(&reference.inner, true)
    }
    fn visit_unit(&mut self, is_child_of_reference: bool) -> T;
    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, is_child_of_reference: bool) -> T;
    fn visit_type(&mut self, _type: &Type, is_child_of_reference: bool) -> T;
}

pub struct TypeConstructorVisitor {
    /// if true then leaf proxies will be converted to their proxy type, otherwise they will be converted to their proxied type
    pub generate_proxy_type: bool,
    pub strip_outer_ref: bool,
}

impl TypeConstructorVisitor {
    pub fn new(generate_proxy_type: bool, strip_outer_ref: bool) -> Self {
        Self {
            generate_proxy_type,
            strip_outer_ref,
        }
    }
}

impl VisitSimpleType<Type> for TypeConstructorVisitor {
    fn visit_unit(&mut self, _: bool) -> Type {
        Type::Tuple(TypeTuple {
            paren_token: Default::default(),
            elems: Default::default(),
        })
    }

    fn visit_unit_path(&mut self, unit_path: &UnitPath, _: bool) -> Type {
        Type::Path(TypePath {
            qself: None,
            path: PathSegment {
                ident: unit_path.ident.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: unit_path.colon2_token,
                    lt_token: unit_path.lt_token,
                    args: Punctuated::from_iter([GenericArgument::Type(
                        self.visit_simple_type(&unit_path.inner, false),
                    )]),
                    gt_token: unit_path.gt_token,
                }),
            }
            .into(),
        })
    }

    fn visit_duo_path(&mut self, duo_path: &DuoPath, _: bool) -> Type {
        let left = self.visit_simple_type(&duo_path.left, false);
        let right = self.visit_simple_type(&duo_path.right, false);

        Type::Path(TypePath {
            qself: None,
            path: PathSegment {
                ident: duo_path.ident.clone(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: duo_path.colon2_token,
                    lt_token: duo_path.lt_token,
                    args: Punctuated::from_iter([
                        GenericArgument::Type(left),
                        GenericArgument::Type(right),
                    ]),
                    gt_token: duo_path.gt_token,
                }),
            }
            .into(),
        })
    }

    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, _: bool) -> Type {
        if self.generate_proxy_type {
            Type::Path(ident_to_type_path(proxy_type.proxy_ident.clone()))
        } else {
            Type::Path(ident_to_type_path(proxy_type.proxied_ident.clone()))
        }
    }

    fn visit_type(&mut self, _type: &Type, _: bool) -> Type {
        _type.clone()
    }

    fn visit_reference(&mut self, reference: &Reference, _: bool) -> Type {
        if self.strip_outer_ref {
            self.visit_simple_type(&reference.inner, false)
        } else {
            self.strip_outer_ref = false;
            Type::Reference(TypeReference {
                and_token: reference.and_token,
                lifetime: None,
                mutability: reference.mutability,
                elem: Box::new(self.visit_simple_type(&reference.inner, true)),
            })
        }
    }
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
            .filter(|attr| attr.path.is_ident("doc"))
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

#[cfg(test)]
mod test {
    use super::VisitSimpleType;

    struct TestVisitor;
    impl VisitSimpleType<bool> for TestVisitor {
        fn visit_unit(&mut self, is_child_of_reference: bool) -> bool {
            is_child_of_reference
        }
        fn visit_proxy_type(&mut self, _: &super::ProxyType, is_child_of_reference: bool) -> bool {
            is_child_of_reference
        }
        fn visit_type(&mut self, _: &syn::Type, is_child_of_reference: bool) -> bool {
            is_child_of_reference
        }
    }

    #[test]
    pub fn test_child_of_reference() {
        let mut visitor = TestVisitor;
        assert!(!visitor.visit(&super::SimpleType::Unit));
        assert!(
            !visitor.visit(&super::SimpleType::ProxyType(super::ProxyType {
                proxied_ident: syn::Ident::new("T", proc_macro2::Span::call_site()),
                proxy_ident: syn::Ident::new("LuaT", proc_macro2::Span::call_site()),
            }))
        );
        assert!(
            !visitor.visit(&super::SimpleType::Type(syn::Type::Path(syn::TypePath {
                qself: None,
                path: syn::Path::from(syn::Ident::new("T", proc_macro2::Span::call_site())),
            })))
        );
        assert!(
            visitor.visit(&super::SimpleType::Reference(super::Reference {
                and_token: syn::Token![&](proc_macro2::Span::call_site()),
                mutability: None,
                inner: Box::new(super::SimpleType::Unit),
            }))
        );
        assert!(
            visitor.visit(&super::SimpleType::Reference(super::Reference {
                and_token: syn::Token![&](proc_macro2::Span::call_site()),
                mutability: None,
                inner: Box::new(super::SimpleType::ProxyType(super::ProxyType {
                    proxied_ident: syn::Ident::new("T", proc_macro2::Span::call_site()),
                    proxy_ident: syn::Ident::new("LuaT", proc_macro2::Span::call_site()),
                })),
            }))
        );
        assert!(
            visitor.visit(&super::SimpleType::Reference(super::Reference {
                and_token: syn::Token![&](proc_macro2::Span::call_site()),
                mutability: None,
                inner: Box::new(super::SimpleType::Type(syn::Type::Path(syn::TypePath {
                    qself: None,
                    path: syn::Path::from(syn::Ident::new("T", proc_macro2::Span::call_site())),
                }))),
            }))
        );
    }
}
