use std::{borrow::Cow, result};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    token::Paren,
    Ident, Token,
};

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SelfType {
    Self_,
    RefSelf,
    MutSelf,
}

impl SelfType {
    /// converts this SelfType to a SimpleType given the value to substituted to self, preserves the references from self
    pub fn resolve_as(self, as_: SimpleType) -> SimpleType {
        match self {
            SelfType::Self_ => SimpleType::BaseIdent(as_.base_ident().clone()),
            SelfType::RefSelf => SimpleType::Ref {
                ampersand: Some(Default::default()),
                mut_: None,
                type_: Box::new(as_),
            },
            SelfType::MutSelf => SimpleType::Ref {
                ampersand: Some(Default::default()),
                mut_: Some(Default::default()),
                type_: Box::new(as_),
            },
        }
    }

    /// Returns true if either a reference or mutable reference
    pub fn is_any_ref(self) -> bool {
        match self {
            SelfType::Self_ => false,
            SelfType::RefSelf => true,
            SelfType::MutSelf => true,
        }
    }

    /// Returns true if mutable reference
    pub fn is_mut_ref(self) -> bool {
        match self {
            SelfType::Self_ => false,
            SelfType::RefSelf => false,
            SelfType::MutSelf => true,
        }
    }
}

impl Parse for SelfType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Token![&]) {
            if input.peek2(Token![mut]) && input.peek3(Token![self]) {
                input
                    .parse::<Token![&]>()
                    .expect("Something went wrong parsing SelfType a ");
                input
                    .parse::<Token![mut]>()
                    .expect("Something went wrong parsing SelfType b ");
                input
                    .parse::<Token![self]>()
                    .expect("Something went wrong parsing SelfType c");
                Ok(SelfType::MutSelf)
            } else {
                input
                    .parse::<Token![&]>()
                    .expect("Something went wrong parsing SelfType d");
                input
                    .parse::<Token![self]>()
                    .expect("Something went wrong parsing SelfType e");
                Ok(SelfType::RefSelf)
            }
        } else {
            input.parse::<Token![self]>()?;
            Ok(SelfType::Self_)
        }
    }
}

impl ToTokens for SelfType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tkns = match self {
            SelfType::Self_ => quote::quote! {self},
            SelfType::RefSelf => quote::quote! {&self},
            SelfType::MutSelf => quote::quote! {&mut self},
        };

        tokens.extend(tkns)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum SimpleType {
    BaseIdent(Ident),
    Ref {
        ampersand: Option<Token![&]>,
        mut_: Option<Token![mut]>,
        type_: Box<SimpleType>,
    },
}

impl SimpleType {
    /// Retrieves the base name of this simple type
    pub fn base_ident(&self) -> &Ident {
        match self {
            SimpleType::BaseIdent(b) => b,
            SimpleType::Ref { type_, .. } => type_.base_ident(),
        }
    }

    /// consumes self and produces the base ident
    pub fn into_base_ident(self) -> Ident {
        match self {
            SimpleType::BaseIdent(b) => b,
            SimpleType::Ref { type_, .. } => type_.into_base_ident(),
        }
    }

    pub fn mutate_base_ident<F: FnMut(&mut Ident)>(&mut self, mut f: F) {
        match self {
            SimpleType::BaseIdent(b) => f(b),
            SimpleType::Ref { type_, .. } => type_.mutate_base_ident(f),
        }
    }

    /// Consumes self and produces a simple type without any outer references
    /// e.g. `&MyType` -> `MyType`
    pub fn strip_outer_refs(self) -> Self {
        match self {
            SimpleType::BaseIdent(_) => self,
            SimpleType::Ref { type_, .. } => *type_,
        }
    }

    pub fn is_any_ref(&self) -> bool {
        matches!(self, Self::Ref { .. })
    }

    pub fn is_mut_ref(&self) -> bool {
        matches!(self, Self::Ref { mut_, .. } if mut_.is_some())
    }
}

impl Parse for SimpleType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Token![&]) {
            Ok(Self::Ref {
                ampersand: input.parse().expect("Something went wrong"),
                mut_: input.parse()?,
                type_: input.parse()?,
            })
        } else {
            Ok(Self::BaseIdent(input.parse()?))
        }
    }
}

impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            SimpleType::BaseIdent(i) => tokens.extend(quote::quote!(#i)),
            SimpleType::Ref {
                ampersand,
                mut_,
                type_,
            } => tokens.extend(quote::quote!(#ampersand #mut_ #type_)),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
/// Raw argument expression argument received from the macro invocation
pub enum ArgType {
    Raw { paren: Paren, type_: SimpleType },
    Wrapped { paren: Paren, type_: SimpleType },
    Self_(SelfType),
}

#[allow(clippy::eval_order_dependence)]
impl Parse for ArgType {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Ident) {
            let ident: Ident = input.parse()?;
            let f;
            match ident.to_string().as_str() {
                "Raw" => Ok(Self::Raw { paren: parenthesized!(f in input), type_: f.parse()? }),
                "Wrapped" => Ok(Self::Wrapped { paren: parenthesized!(f in input), type_: f.parse()? }),
                _ => panic!("Invalid argument, valid arguments are: [Raw(type),Wrapped(type),self,&self,&mut self]"),
            }
        } else {
            Ok(Self::Self_(input.parse()?))
        }
    }
}

impl ToTokens for ArgType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ArgType::Raw { type_, .. } => tokens.extend(quote::quote!(Raw(#type_))),
            ArgType::Wrapped { type_, .. } => tokens.extend(quote::quote!(Wrapped(#type_))),
            ArgType::Self_(s) => s.to_tokens(tokens),
        };
    }
}

impl ArgType {
    /// retrieves the underlying argument type, if it's not a concrete type but a receiver, returns its type
    pub fn type_(&self) -> result::Result<&SimpleType, SelfType> {
        match self {
            Self::Raw { type_, .. } | Self::Wrapped { type_, .. } => Ok(type_),
            Self::Self_(s) => Err(*s),
        }
    }

    pub fn self_(&self) -> result::Result<SelfType, &SimpleType> {
        match self {
            Self::Raw { type_, .. } | Self::Wrapped { type_, .. } => Err(type_),
            Self::Self_(s) => Ok(*s),
        }
    }

    /// Retrieves the simple type or generates one  using [`SelfType::resolve_as`](`SelfType`) if this is a self type
    pub fn type_or_resolve<F: FnMut() -> SimpleType>(&self, mut f: F) -> Cow<SimpleType> {
        self.type_()
            .map(Cow::Borrowed)
            .unwrap_or_else(|self_| Cow::Owned(self_.resolve_as(f())))
    }

    pub fn is_any_ref(&self) -> bool {
        match self {
            ArgType::Raw { type_, .. } => type_.is_any_ref(),
            ArgType::Wrapped { type_, .. } => type_.is_any_ref(),
            ArgType::Self_(s) => s.is_any_ref(),
        }
    }

    pub fn is_mut_ref(&self) -> bool {
        match self {
            ArgType::Raw { type_, .. } => type_.is_mut_ref(),
            ArgType::Wrapped { type_, .. } => type_.is_mut_ref(),
            ArgType::Self_(s) => s.is_mut_ref(),
        }
    }

    pub fn is_wrapped(&self) -> bool {
        matches!(self, Self::Wrapped { .. })
    }

    pub fn is_raw(&self) -> bool {
        matches!(self, Self::Raw { .. })
    }

    pub fn is_self(&self) -> bool {
        matches!(self, Self::Self_ { .. })
    }
}
