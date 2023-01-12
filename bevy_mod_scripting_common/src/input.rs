use indexmap::{IndexMap, IndexSet};
use proc_macro2::Ident;
use syn::{
    parse::{Nothing, Parse},
    punctuated::Punctuated,
    spanned::Spanned,
    DataStruct, DeriveInput, Fields, TraitItemMethod,
};

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
        todo!()
    }
}

/// Container for proxy flags
#[derive(Debug, Default)]
pub struct ProxyFlags {
    pub derive_flags: IndexSet<DeriveFlag>,
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
        Ok(Self { derive_flags })
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

/// Attributes relating to the proxy as a whole
#[derive(Debug)]
pub struct ProxyMeta {
    pub proxy_name: Ident,
    pub language_meta: LanguageMeta,
    pub proxy_flags: ProxyFlags,
    pub functions: IndexMap<Ident, TraitItemMethod>,
    pub data: ProxyData,
}

impl TryFrom<DeriveInput> for ProxyMeta {
    type Error = syn::Error;

    fn try_from(derive_input: DeriveInput) -> Result<Self, Self::Error> {
        let mut proxy_name = derive_input.ident.clone();

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
                syn::parse::<ZeroOrManyTerminated<TraitItemMethod, Nothing>>(attr.tokens.into())
                    .map(|fns| {
                        fns.0
                            .into_iter()
                            .map(|_fn| (_fn.sig.ident.clone(), _fn))
                            .collect::<IndexMap<_, _>>()
                    })
            })?;

        let mut language_meta = Default::default();
        let proxy_flags = Default::default();

        if let syn::Meta::List(list) = proxy_meta {
            for attr in list.nested.into_iter() {
                if let syn::NestedMeta::Meta(syn::Meta::NameValue(pair)) = attr {
                    let ident = pair.path.get_ident().ok_or_else(|| {
                        syn::Error::new_spanned(&pair, "Keys must be identifiers")
                    })?;

                    match (ident.to_string().as_str(), pair.lit) {
                        ("name", syn::Lit::Str(_str)) => {
                            proxy_name = Ident::new(&_str.value(), _str.span())
                        }
                        _ => return Err(syn::Error::new_spanned(ident, "Unrecognized argument")),
                    }
                } else if let syn::NestedMeta::Meta(syn::Meta::List(list)) = attr {
                    let ident = list
                        .path
                        .get_ident()
                        .ok_or_else(|| syn::Error::new_spanned(&list, "Expected identifier"))?;

                    match ident.to_string().as_str() {
                        "languages" => language_meta = list.try_into()?,
                        _ => return Err(syn::Error::new_spanned(list, "")),
                    }
                } else {
                    return Err(syn::Error::new_spanned(attr, "Expected key value pair"));
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

        Ok(ProxyMeta {
            proxy_name,
            proxy_flags,
            language_meta,
            functions,
            data,
        })
    }
}
