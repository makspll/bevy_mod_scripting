use proc_macro2::{Span, TokenStream};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
    Attribute, Ident, Member, Token,
};

use crate::{ops::*, utils::impl_parse_enum};

use quote::ToTokens;

use super::arg::ArgType;

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub enum DeriveFlag {

    /// Tells the implementors this type supports `Debug`
    Debug => {Ok(Self::Debug{ident})},
    /// Tells the implementors this type supports `Display`
    Display => {Ok(Self::Display{ident})},
    /// Tells the implementors this type supports `Clone`
    Clone{} => {Ok(Self::Clone{ident})},
    /// Tells the implementors what fields are available on this type
    Fields {
        paren: Paren,
        fields: Punctuated<AutoField,Token![,]>
    } => {
        let f;
        Ok(Self::Fields {
            ident,
            paren: parenthesized!(f in input),
            fields: f.parse_terminated(AutoField::parse)?
        })
    },
    /// Tells the implementors which methods are available on this type
    Methods {
        paren: Paren,
        methods: Punctuated<AutoMethod,Token![,]>
    } => {
        let f;
        Ok(Self::Methods{
            ident,
            paren: parenthesized!(f in input),
            methods: f.parse_terminated(AutoMethod::parse)?
        })
    },
    /// Tells the implementors which unary operations this type supports
    UnaryOps{
        paren : Paren,
        ops: Punctuated<OpExpr,Token![,]>

    } => {
        let f;
        Ok(Self::UnaryOps{
            ident,
            paren: parenthesized!(f in input),
            ops : f.parse_terminated(OpExpr::parse)?
        })
    },
    /// Tells the implementors which binary operations this type supports
    BinOps {
        paren: Paren,
        ops: Punctuated<OpExpr,Token![,]>
    } => {
        let f;
        Ok(Self::BinOps {
            ident,
            paren: parenthesized!(f in input),
            ops: f.parse_terminated(OpExpr::parse)?,
        })
    }
}
);

#[derive(PartialEq, Eq, Hash)]
pub struct AutoMethod {
    pub docstring: Vec<Attribute>,
    pub ident: Ident,
    pub paren: Paren,
    pub self_: Option<(ArgType, Token![:])>,
    pub args: Punctuated<ArgType, Token![,]>,
    pub out: Option<ArgType>,
}

impl ToTokens for AutoMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let docstring = self.docstring.iter();
        let id = &self.ident;
        let args = &self.args;
        let self_ = self.self_.as_ref().map(|(a, _)| quote::quote!(#a:));
        let out = self.out.as_ref().map(|t| quote::quote! {-> #t});
        tokens.extend(quote::quote! {
            #(#docstring)*
            #id(#self_ #args) #out
        })
    }
}

#[allow(clippy::mixed_read_write_in_expression)]
impl Parse for AutoMethod {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let f;
        let o = Ok(Self {
            docstring: Attribute::parse_outer(input)?,
            ident: input.parse()?,
            paren: parenthesized!(f in input),
            self_: {
                let parser = |p: ParseStream| {
                    Ok::<_, syn::Error>((p.parse::<ArgType>()?, p.parse::<Token![:]>()?))
                };
                let fork = f.fork();
                if parser(&fork).is_ok() {
                    Some(parser(&f).expect("Something went wrong"))
                } else {
                    None
                }
            },
            args: f.parse_terminated(ArgType::parse)?,
            out: if input.peek(Token![->]) {
                input.parse::<Token![->]>()?;
                Some(input.parse()?)
            } else {
                None
            },
        });
        o
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct AutoFieldAttributes {
    pub script_name: Option<Ident>,
}

impl TryFrom<&[Attribute]> for AutoFieldAttributes {
    type Error = syn::Error;

    fn try_from(value: &[Attribute]) -> Result<Self, Self::Error> {
        let mut out = Self { script_name: None };

        for v in value {
            let meta = v.parse_meta()?;

            if let Some(ident) = meta.path().get_ident() {
                if *ident == "rename" {
                    if let syn::Meta::List(l) = &meta {
                        for nested in &l.nested {
                            if let syn::NestedMeta::Lit(syn::Lit::Str(s)) = nested {
                                out.script_name = Some(s.parse()?)
                            }
                        }
                    }
                }
            }
        }

        Ok(out)
    }
}

#[derive(PartialEq, Eq, Hash)]
pub struct AutoField {
    pub docstring: Vec<Attribute>,
    pub attrs: Vec<Attribute>,
    pub parsed_attrs: AutoFieldAttributes,
    pub member: Member,
    pub colon: Token![:],
    pub type_: ArgType,
}

impl Parse for AutoField {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let attrs = Attribute::parse_outer(input)?;
        let split_idx = attrs.partition_point(|attr| *attr.path.get_ident().unwrap() == "doc");
        Ok(Self {
            docstring: attrs[0..split_idx].to_owned(),
            attrs: attrs[split_idx..].to_owned(),
            parsed_attrs: attrs[split_idx..].try_into()?,
            member: input.parse()?,
            colon: input.parse()?,
            type_: input.parse()?,
        })
    }
}

impl ToTokens for AutoField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let docstring = self.docstring.iter();
        let attrs = self.attrs.iter();
        let id = &self.member;
        let type_ = &self.type_;

        tokens.extend(quote::quote! {
            #(#docstring)*
            #(#attrs)*
            #id : #type_
        })
    }
}
