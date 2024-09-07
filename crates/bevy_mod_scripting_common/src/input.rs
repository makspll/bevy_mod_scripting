#![allow(clippy::manual_unwrap_or_default)] // from darling
use darling::{util::Flag, FromDeriveInput, FromMeta};
use proc_macro2::Ident;
use quote::format_ident;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    str::FromStr,
};
use strum::{Display, EnumString};
use syn::{
    punctuated::Punctuated,
    spanned::Spanned,
    token::{And, Gt, Lt, Mut, PathSep},
    visit_mut::VisitMut,
    AngleBracketedGenericArguments, Attribute, Error, Field, Fields, GenericArgument, PatType,
    Path, PathArguments, PathSegment, Receiver, TraitItemFn, Type, TypePath, TypeReference,
    TypeTuple, Variant,
};

use crate::utils::ident_to_type_path;

/// Flags which detail required functionality or additional derivation requirements
#[derive(Debug, FromMeta, Default)]
pub struct ProxyFlags {
    pub clone: Flag,
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
    pub colon2_token: Option<PathSep>,
    pub lt_token: Lt,
    pub gt_token: Gt,
    pub inner: Box<SimpleType>,
}

/// For types of the form `Result<L, R>` i.e. an outer identifier with two nested types inside angle brackets.
#[derive(Debug, Clone)]
pub struct DuoPath {
    pub std_type_ident: Option<StdTypeIdent>,
    pub ident: Ident,
    pub colon2_token: Option<PathSep>,
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
    pub proxied_path: Path,
    pub proxy_ident: Ident,
}

/// Proxies can also be returned in "container" types, such as:
/// - Option
/// - Result
/// - Vec
/// - Tuple
///
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
        proxied_type_path: &Path,
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
                        &Type::Path(TypePath {
                            qself: None,
                            path: proxied_type_path.clone(),
                        }),
                        proxied_type_path,
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
                    &Type::Path(TypePath {
                        qself: None,
                        path: proxied_type_path.clone(),
                    }),
                    proxied_type_path,
                    proxied_to_proxy_ident_map,
                ),
            },
            syn::FnArg::Typed(PatType { ty, .. }) => Self::new_from_contextual_type(
                proxy_prefix,
                ty.as_ref(),
                proxied_type_path,
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
        Self::new_from_type(
            proxy_prefix,
            proxied_type,
            None,
            proxied_to_proxy_ident_map,
            false,
        )
    }

    pub fn new_from_fully_specified_type_proxy_all(
        proxy_prefix: &'static str,
        proxied_type: &Type,
    ) -> Result<SimpleType, Error> {
        Self::new_from_type(proxy_prefix, proxied_type, None, &Default::default(), true)
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will be replaced
    /// with the given identifier prefixed with the proxy_prefix
    pub fn new_from_contextual_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_path: &Path,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
    ) -> Result<SimpleType, Error> {
        Self::new_from_type(
            proxy_prefix,
            proxied_type,
            Some(proxied_type_path),
            proxied_to_proxy_ident_map,
            false,
        )
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, contextual receivers such as `Self` and `self` will be replaced
    /// with the given identifier prefixed with the proxy_prefix
    /// All types will be proxied with the given proxy prefix
    pub fn new_from_contextual_type_proxy_all(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_path: &Path,
    ) -> Result<SimpleType, Error> {
        Self::new_from_type(
            proxy_prefix,
            proxied_type,
            Some(proxied_type_path),
            &Default::default(),
            true,
        )
    }

    /// Builds a SimpleType::ProxyType or SimpleType::Type depending on the passed data,
    /// - if the proxied type identifier has a Some value in the proxied_to_proxy_ident_map map then the proxy_ident will be set to the value in the map,
    /// - if it has a None value it will be set to the proxied type identifier prefixed with the proxy_prefix,
    /// - If it's not in the map it's built as a SimpleType::Type
    fn new_proxied_type_or_type(
        proxy_prefix: &'static str,
        proxied_path: Path,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
        proxy_prefix_all: bool,
    ) -> SimpleType {
        let last_segment = &proxied_path.segments.last().unwrap().ident;
        let replacement_ident = proxied_to_proxy_ident_map.get(last_segment);

        if proxy_prefix_all || replacement_ident.is_some() {
            let proxy_ident = replacement_ident
                .cloned()
                .flatten()
                .unwrap_or_else(|| format_ident!("{proxy_prefix}{}", last_segment));

            SimpleType::ProxyType(ProxyType {
                proxied_path,
                proxy_ident,
            })
        } else {
            Self::Type(syn::Type::Path(TypePath {
                qself: None,
                path: proxied_path.clone(),
            }))
        }
    }

    /// Constructs a new SimpleProxyType from a `syn::Type`, if `proxied_type_identifier` is given then contextual
    /// receivers such as `Self` and `self` will be replaced with the given identifier prefixed with the proxy_prefix, otherwise an error will be returned.
    /// types with base identifiers not in the proxied_to_proxy_ident_map list are treated as non-proxy types and will be wrapped in a SimpleProxyType::Type.
    /// If the proxy_prefix_all option is passed, the ident map will be ignored and EVERY type inside will be treated as a default proxy (prefixed with the proxy prefix as well)
    fn new_from_type(
        proxy_prefix: &'static str,
        proxied_type: &Type,
        proxied_type_path: Option<&Path>,
        proxied_to_proxy_ident_map: &HashMap<Ident, Option<Ident>>,
        proxy_prefix_all: bool,
    ) -> Result<SimpleType, Error> {
        match proxied_type {
            Type::Path(p) if p.path.is_ident("self") || p.path.is_ident("Self") => {
                let proxied_path: &Path = proxied_type_path.ok_or_else(|| {
                    Error::new_spanned(
                        proxied_type,
                        "Did not expect contextual receiver in constructing simple proxy type"
                            .to_owned(),
                    )
                })?;
                Ok(Self::new_proxied_type_or_type(
                    proxy_prefix,
                    proxied_path.clone(),
                    proxied_to_proxy_ident_map,
                    proxy_prefix_all,
                ))
            }
            Type::Path(p) if !p.path.segments.is_empty() => {
                let last_segment = p.path.segments.last().unwrap();
                if last_segment.arguments.is_empty() {
                    return Ok(Self::new_proxied_type_or_type(
                        proxy_prefix,
                        p.path.clone(),
                        proxied_to_proxy_ident_map,
                        proxy_prefix_all,
                    ));
                } else if let PathArguments::AngleBracketed(args) = &last_segment.arguments {
                    if args.args.len() == 1 {
                        if let GenericArgument::Type(arg_type) = args.args.first().unwrap() {
                            let inner = Box::new(Self::new_from_type(
                                proxy_prefix,
                                arg_type,
                                proxied_type_path,
                                proxied_to_proxy_ident_map,
                                proxy_prefix_all,
                            )?);
                            return Ok(SimpleType::UnitPath(UnitPath {
                                std_type_ident: StdTypeIdent::from_str(
                                    &last_segment.ident.to_string(),
                                )
                                .ok(),
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
                                proxied_type_path,
                                proxied_to_proxy_ident_map,
                                proxy_prefix_all,
                            )?);
                            let right = Box::new(Self::new_from_type(
                                proxy_prefix,
                                right,
                                proxied_type_path,
                                proxied_to_proxy_ident_map,
                                proxy_prefix_all,
                            )?);
                            return Ok(SimpleType::DuoPath(DuoPath {
                                std_type_ident: StdTypeIdent::from_str(
                                    &last_segment.ident.to_string(),
                                )
                                .ok(),
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
                Err(Error::new_spanned(
                    proxied_type,
                    "Unsupported type".to_owned(),
                ))
            }
            Type::Reference(tr) => Ok(SimpleType::Reference(Reference {
                and_token: tr.and_token,
                mutability: tr.mutability,
                inner: Box::new(Self::new_from_type(
                    proxy_prefix,
                    &tr.elem,
                    proxied_type_path,
                    proxied_to_proxy_ident_map,
                    proxy_prefix_all,
                )?),
            })),
            Type::Infer(_) => Ok(SimpleType::Type(proxied_type.clone())),
            Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty() => Ok(SimpleType::Unit),
            t => Ok(SimpleType::Type(t.clone())),
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
            Type::Path(TypePath {
                qself: None,
                path: proxy_type.proxied_path.clone(),
            })
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

#[derive(FromDeriveInput)]
#[darling(attributes(proxy), forward_attrs(allow, doc, cfg))]
#[allow(clippy::manual_unwrap_or_default)]

pub struct ProxyInput {
    /// The name of the type for which we are generating a proxy
    pub ident: syn::Ident,
    pub attrs: Vec<Attribute>,

    pub remote: Option<syn::Path>,
    /// The name to use for the proxy type, if not provided the language derive macro
    /// will generate one using a standard prefix.
    #[darling(rename = "name")]
    pub proxy_name: Option<Ident>,

    /// The body of the type for which we are generating a proxy
    pub data: darling::ast::Data<Variant, Field>,

    /// Flags signifying which additional trait implementation should be generated on the proxy type
    #[darling(default)]
    pub derive: ProxyFlags,

    /// A list of multi-lang function definitions to be generated on the proxy type
    #[darling(default)]
    pub functions: TraitItemFnsWrapper,
}

#[derive(Default)]
pub struct TraitItemFnsWrapper(pub Vec<TraitItemFn>);

impl FromMeta for TraitItemFnsWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        let token_stream: proc_macro2::TokenStream = value.parse().map_err(syn::Error::from)?;
        let trait_items_vec = vec![syn::parse2(token_stream)?];
        Ok(TraitItemFnsWrapper(trait_items_vec))
    }

    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(TraitItemFnsWrapper(
            items
                .iter()
                .map(Self::from_nested_meta)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flat_map(|v| v.0.into_iter())
                .collect::<Vec<_>>(),
        ))
    }
}

impl Deref for TraitItemFnsWrapper {
    type Target = Vec<TraitItemFn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TraitItemFnsWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
                proxied_path: syn::Ident::new("T", proc_macro2::Span::call_site()).into(),
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
                    proxied_path: syn::Ident::new("T", proc_macro2::Span::call_site()).into(),
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
