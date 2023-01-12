use indexmap::IndexSet;
use proc_macro2::Ident;
use syn::{spanned::Spanned, DeriveInput};

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

impl TryFrom<(&DeriveInput, syn::MetaList)> for LanguageMeta {
    type Error = syn::Error;

    fn try_from(value: (&DeriveInput, syn::MetaList)) -> Result<Self, Self::Error> {
        let (_, list) = value;
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

impl<'a> TryFrom<(&'a DeriveInput, syn::NestedMeta)> for DeriveFlag {
    type Error = syn::Error;

    fn try_from(value: (&'a DeriveInput, syn::NestedMeta)) -> Result<Self, Self::Error> {
        todo!()
    }
}

/// Container for proxy flags
#[derive(Debug, Default)]
pub struct ProxyFlags {
    pub derive_flags: IndexSet<DeriveFlag>,
    pub functions: 
}

impl<'a> TryFrom<(&'a DeriveInput, syn::MetaList)> for ProxyFlags {
    type Error = syn::Error;

    fn try_from(value: (&'a DeriveInput, syn::MetaList)) -> Result<Self, Self::Error> {
        let (derive_input, meta_list) = value;
        let mut flags: IndexSet<DeriveFlag> = Default::default();
        for nested_meta in meta_list.nested {
            let span = nested_meta.span();
            let flag: DeriveFlag = (derive_input, nested_meta).try_into()?;
            if flags.contains(&flag) {
                return Err(syn::Error::new(
                    span,
                    "This flag was already defined, remove duplicate flag",
                ));
            } else {
                flags.insert(flag);
            }
        }
        Ok(Self { flags })
    }
}

/// Attributes relating to the proxy as a whole
#[derive(Debug)]
pub struct ProxyMeta<'a> {
    pub proxy_name: Ident,
    pub language_meta: LanguageMeta,
    pub proxy_flags: ProxyFlags,
    pub derive_data: &'a DeriveInput,
}

impl<'a> TryFrom<(&'a DeriveInput, syn::Meta)> for ProxyMeta<'a> {
    type Error = syn::Error;

    fn try_from(value: (&'a DeriveInput, syn::Meta)) -> Result<Self, Self::Error> {
        let (derive_data, meta) = value;
        if let syn::Meta::List(list) = meta {
            let mut proxy_name = derive_data.ident.clone();
            let mut language_meta = Default::default();
            let mut proxy_flags = Default::default();

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
                        "languages" => language_meta = (derive_data, list).try_into()?,
                        _ => return Err(syn::Error::new_spanned(list, "")),
                    }
                } else {
                    return Err(syn::Error::new_spanned(attr, "Expected key value pair"));
                }
            }
            Ok(ProxyMeta {
                proxy_name,
                proxy_flags,
                language_meta,
                derive_data,
            })
        } else {
            Err(syn::Error::new_spanned(
                meta,
                "Expected list of key value pairs",
            ))
        }
    }
}
