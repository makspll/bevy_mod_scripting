use std::marker::PhantomData;

use indexmap::IndexSet;
use proc_macro2::TokenStream;
use syn::{
    parenthesized,
    parse::{Parse, ParseBuffer, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token, Attribute, Block, DeriveInput, Field, Fields, Generics, Ident, Lit, LitStr, Meta,
    MetaList, MetaNameValue, Signature, Token,
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
    Methods(Punctuated<ProxyMethod, Token![,]>),
    UnaryOps,
    BinaryOps,
}

#[derive(Hash, Debug, PartialEq, Eq)]
/// A representation of a proxied method, has the potential to define a full blown method as well,
/// it is up to each individual language to interpret the meaning of the attributes and method signatures.
///
/// For example some function names may be reserved (`to_string` in lua for example) for operators.
pub struct ProxyMethod {
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub body: Option<Block>,
}

impl Parse for ProxyMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            sig: input.parse()?,
            body: {
                if input.peek(token::Brace) {
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

impl ProxyFlag {
    pub fn from_ident_and_tokens(
        ident: &Ident,
        tokens: Option<ParseStream>,
    ) -> Result<Self, syn::Error> {
        let name = ident.to_string();

        let parse_ident_only = |f: fn() -> Self| {
            if let Some(tokens) = &tokens {
                let span = {
                    if tokens.is_empty() {
                        ident.span()
                    } else {
                        tokens.span()
                    }
                };

                Err(syn::Error::new(
                    span,
                    format!("'{name}' does not expect any arguments. Remove `({tokens})`"),
                ))
            } else {
                Ok(f())
            }
        };

        let parse_with_body = |f: fn(ParseStream) -> Result<Self, _>| {
            if let Some(tokens) = tokens {
                f(tokens)
            } else {
                Err(syn::Error::new(
                    ident.span(),
                    format!("`{name}` expects arguments. Add `(<arguments>)`"),
                ))
            }
        };

        match name.as_str() {
            "Clone" => parse_ident_only(|| Self::Clone),
            "Debug" => parse_ident_only(|| Self::Debug),
            "Methods" => parse_with_body(|input| {
                Ok(Self::Methods(input.call(Punctuated::parse_terminated)?))
            }),
            _ => Err(syn::Error::new_spanned(ident, "Unknown proxy flag")),
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
    /// type docstring
    pub docstrings: Vec<LitStr>,
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
            .filter_map(ProxyFlags::from_attribure)
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(ProxyFlags::default(), |mut a, b| {
                a.merge(b);
                a
            });

        let docstrings = input
            .attrs
            .iter()
            .filter_map(|attr| {
                if attr.path.is_ident("doc") {
                    match attr.parse_meta().unwrap() {
                        Meta::NameValue(MetaNameValue {
                            lit: Lit::Str(str), ..
                        }) => Some(str),
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let meta = ProxyMeta {
            base_type_name: &input.ident,
            proxy_flags: flags,
            generics: &input.generics,
            docstrings,
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
    /// Parses a single proxy flag
    pub fn parse_one(input: ParseStream) -> Result<ProxyFlag, syn::Error> {
        let attr_ident = input.parse::<Ident>()?;

        // work out if there is a payload in the token
        if input.peek(token::Paren) {
            let tokens;
            parenthesized!(tokens in input);
            ProxyFlag::from_ident_and_tokens(&attr_ident, Some(&tokens))
        } else {
            ProxyFlag::from_ident_and_tokens(&attr_ident, None)
        }
    }

    /// Parses proxy flags separated by the given separator
    pub fn parse_separated<S: Parse>(
        input: ParseStream,
    ) -> Result<Punctuated<ProxyFlag, S>, syn::Error> {
        Punctuated::<_, S>::parse_terminated_with(input, Self::parse_one)
    }

    /// Parses a whole attribute with proxy flag annotations. Returns Some value if the attribute has a valid path, and None otherwise
    pub fn from_attribure(attr: &Attribute) -> Option<Result<Self, syn::Error>> {
        if !attr.path.is_ident(ATTRIBUTE_NAME) {
            return None;
        }

        Some(
            attr.parse_args_with(Self::parse_separated::<Token![,]>)
                .map(IntoIterator::into_iter)
                .map(Iterator::collect),
        )
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
