use proc_macro2::{Span, TokenStream};
use quote::{quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Token,
};

use crate::utils::impl_parse_enum;

use super::{arg::ArgType};

impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash,Clone,Debug)]
pub enum OpName {
    Add => {Ok(Self::Add{ident})},
    Sub => {Ok(Self::Sub{ident})},
    Mul => {Ok(Self::Mul{ident})},
    Div => {Ok(Self::Div{ident})},
    Rem => {Ok(Self::Rem{ident})},
    Neg => {Ok(Self::Neg{ident})},

}

impl MathOpName {
    pub fn to_rlua_metamethod_path(&self) -> TokenStream {
        match self {
            OpName::Add { ident } => quote_spanned!(ident.span()=> Add),
            OpName::Sub { ident } => quote_spanned!(ident.span()=> Sub),
            OpName::Mul { ident } => quote_spanned!(ident.span()=> Mul),
            OpName::Div { ident } => quote_spanned!(ident.span()=> Div),
            OpName::Rem { ident } => quote_spanned!(ident.span()=> Mod),
            OpName::Neg { ident } => quote_spanned!(ident.span()=> Unm),

        }
    }

    pub fn to_rust_method_ident(&self) -> TokenStream {
        match self {
            Self::Add{ident} => quote_spanned!{ident.span()=> add},
            Self::Sub{ident} => quote_spanned!{ident.span()=> sub},
            Self::Mul{ident} => quote_spanned!{ident.span()=> mul},
            Self::Div{ident} => quote_spanned!{ident.span()=> div},
            Self::Rem{ident} => quote_spanned!{ident.span()=> rem},
            Self::Neg{ident} => quote_spanned!{ident.span()=> neg},

        }
    }
}
);

/// Left or Right
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

use std::fmt;
impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Side::Left => f.write_str("Left"),
            Side::Right => f.write_str("Right"),
        }
    }
}

impl Side {
    pub fn opposite(&self) -> Self {
        match self {
            Side::Left => Self::Right,
            Side::Right => Self::Left,
        }
    }
}

/// Represents either a unary or binary expression, where at least one side has the receiver (self) type
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct OpExpr {
    pub left: Option<ArgType>,
    pub op: OpName,
    pub right: ArgType,
    pub arrow: Token![->],
    pub return_type: ArgType,
}

impl Parse for OpExpr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let s = Self {
            left: (&input.fork())
                .parse::<OpName>()
                .is_err()
                .then(|| Ok::<_, syn::Error>(Some(input.parse()?)))
                .unwrap_or(Ok(None))?,
            op: input.parse()?,
            right: input.parse()?,
            arrow: input.parse()?,
            return_type: input.parse()?,
        };

        if s.has_receiver() {
            Ok(s)
        } else {
            Err(syn::Error::new_spanned(s, "Invalid expression, binary/unary expressions expect at least one side to be one of: [self,&self,&mut self]"))
        }
    }
}

impl ToTokens for OpExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let left = &self.left;
        let op = &self.op;
        let right = &self.right;
        let return_type = &self.return_type;

        tokens.extend(quote::quote! {
            #left #op #right -> #return_type
        })
    }
}

impl OpExpr {
    pub fn is_binary(&self) -> bool {
        self.left.is_some()
    }

    fn has_receiver(&self) -> bool {
        if let Some(ArgType::Self_(_)) = self.left {
            true
        } else if let ArgType::Self_(_) = self.right {
            true
        } else {
            true
        }
    }

    pub fn has_receiver_on_side(&self, side: Side) -> bool {
        match side {
            Side::Left => self.left.as_ref().map(|t| t.is_self()).unwrap_or_default(),
            Side::Right => self.right.is_self(),
        }
    }

    /// Maps the given side if it exists (right is guaranteed to exist, left is not)
    pub fn map_side<O, F: FnOnce(&ArgType) -> O>(&self, side: Side, f: F) -> Option<O> {
        match side {
            Side::Left => self.left.as_ref().map(f),
            Side::Right => Some(f(&self.right)),
        }
    }

    /// call map_side on both Left and Right sides and return the results as a tuple
    pub fn map_both<O, F: FnMut(&ArgType, Side) -> O>(&self, mut f: F) -> (Option<O>, O) {
        (
            self.map_side(Side::Left, |a| f(a, Side::Left)),
            self.map_side(Side::Right, |a| f(a, Side::Right))
                .expect("Cannot happen"),
        )
    }
}
