use std::marker::PhantomData;

use indexmap::IndexSet;
use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    token, Attribute, DeriveInput, Field, Fields, Generics, Ident, Meta, MetaList, Token,
};

pub const ATTRIBUTE_NAME: &str = "scripting";

pub enum ProxyData<'a> {
    Struct(StructData<'a>),
    TupleStruct(StructData<'a>),
    UnitStruct(StructData<'a>),
    Enum(EnumData<'a>),
}

#[derive(Default, Debug)]
pub struct ProxyFlags {
    flags: IndexSet<ProxyFlag>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub enum ProxyFlag {
    Debug,
    Display,
    Clone,
    Fields,
    Methods,
    UnaryOps,
    BinaryOps,
}
impl ProxyFlag {
    pub fn from_name_and_tokens(
        name: &str,
        tokens: Option<TokenStream>,
    ) -> Result<Self, syn::Error> {
        match name {
            "Clone" => Ok(Self::Clone),
            _ => Err(syn::Error::new_spanned(tokens, "Unknown proxy flag")),
        }
    }
}

pub struct ProxyMeta<'a> {
    /// The name of the type being wrapped by this proxy
    pub base_type_name: &'a Ident,
    /// Flags representing additional required functionality
    pub proxy_flags: ProxyFlags,
    /// The generics defined on the base type
    pub generics: &'a Generics,
}

pub struct StructData<'a> {
    pub meta: ProxyMeta<'a>,
    pub fields: Vec<StructField<'a>>,
}

pub struct StructField<'a> {
    pub data: &'a Field,
    pub index: usize,
}

pub struct EnumData<'a> {
    pub meta: ProxyMeta<'a>,
}

impl<'a> TryFrom<&'a DeriveInput> for ProxyData<'a> {
    type Error = syn::Error;

    fn try_from(input: &'a DeriveInput) -> Result<Self, Self::Error> {
        let flags = input
            .attrs
            .iter()
            .filter_map(|attr| ProxyFlags::from_attribure(attr).ok())
            .fold(ProxyFlags::default(), |mut a, b| {
                a.merge(b);
                a
            });

        let meta = ProxyMeta {
            base_type_name: &input.ident,
            proxy_flags: flags,
            generics: &input.generics,
        };

        match &input.data {
            syn::Data::Struct(data) => {
                let fields = Self::collect_struct_fields(&data.fields)?;
                let struct_data = StructData { meta, fields };

                match data.fields {
                    Fields::Named(..) => Ok(Self::Struct(struct_data)),
                    Fields::Unnamed(..) => Ok(Self::TupleStruct(struct_data)),
                    Fields::Unit => Ok(Self::UnitStruct(struct_data)),
                }
            }
            syn::Data::Enum(_) => todo!(),
            syn::Data::Union(_) => todo!(),
        }
    }
}

impl ProxyData<'_> {
    pub fn collect_struct_fields(fields: &Fields) -> Result<Vec<StructField>, syn::Error> {
        fields
            .iter()
            .enumerate()
            .map(|(index, field)| Ok(StructField { data: field, index }))
            .collect()
    }
}

impl ProxyFlags {
    pub fn from_nested_metas(list: MetaList) -> Self {
        let mut flags = Self::default();
        for nested_meta in list.nested.iter() {
            match nested_meta {
                syn::NestedMeta::Meta(Meta::Path(path)) => {
                    let ident = if let Some(segment) = path.segments.first() {
                        segment.ident.to_string()
                    } else {
                        continue;
                    };

                    let flag = match ident.as_str() {
                        "Clone" => ProxyFlag::Clone,
                        _ => continue,
                    };

                    flags.flags.insert(flag);
                }
                _ => continue,
            }
        }
        flags
    }

    /// Parses a single proxy flag
    pub fn parse_one(input: ParseStream) -> Result<ProxyFlag, syn::Error> {
        let attr_name: String = input.parse::<Ident>()?.to_string();

        // work out if there is a payload in the token
        if input.peek(token::Paren) {
            let tokens;
            parenthesized!(tokens in input);
            ProxyFlag::from_name_and_tokens(&attr_name, Some(tokens.parse()?))
        } else {
            ProxyFlag::from_name_and_tokens(&attr_name, None)
        }
    }

    /// Parses proxy flags separated by the given separator
    pub fn parse_separated<S: Parse>(
        input: ParseStream,
    ) -> Result<Punctuated<ProxyFlag, S>, syn::Error> {
        Punctuated::<_, S>::parse_terminated_with(input, Self::parse_one)
    }

    /// Parses a whole attribute with proxy flag annotations
    pub fn from_attribure(attr: &Attribute) -> Result<Self, syn::Error> {
        attr.parse_args_with(Self::parse_separated::<Token![,]>)
            .map(IntoIterator::into_iter)
            .map(Iterator::collect)
    }

    pub fn contains(&self, flag: &ProxyFlag) -> bool {
        self.flags.contains(flag)
    }

    pub fn merge(&mut self, o: Self) {
        self.flags.extend(o.flags.into_iter())
    }
}

impl FromIterator<ProxyFlag> for ProxyFlags {
    fn from_iter<T: IntoIterator<Item = ProxyFlag>>(iter: T) -> Self {
        Self {
            flags: FromIterator::from_iter(iter),
        }
    }
}
