

use proc_macro2::Span;
use syn::{*, parse::*, punctuated::*, token::{*},Type};

use crate::{ops::*,lua_method::{LuaMethodType, MethodMacroArg},TokenStream2, utils::impl_parse_enum};



use quote::ToTokens;




impl_parse_enum!(input,ident:
#[derive(PartialEq,Eq,Hash)]
pub(crate) enum DeriveFlag {
    
    DebugToString => {Ok(Self::DebugToString{ident})},
    DisplayToString => {Ok(Self::DisplayToString{ident})},
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
    AutoMethods {
        paren: Paren,
        methods: Punctuated<AutoMethod,Token![,]>
    } => {
        let f; 
        Ok(Self::AutoMethods{
            ident, 
            paren: parenthesized!(f in input), 
            methods: f.parse_terminated(AutoMethod::parse)? 
        })
    },
    FromLuaProxy{} => {Ok(Self::FromLuaProxy{ident})},
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


#[derive(PartialEq,Eq,Hash,Debug)]
pub(crate) struct MethodMacroInvokation{
   pub target: TypePath,
   pub arrow: Token![->],
   pub identifier: LuaMethodType,
   pub paren: Paren,
   pub args : Punctuated<MethodMacroArg,Token![,]>
}

impl Parse for MethodMacroInvokation {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            target: input.parse()?,
            arrow: input.parse()?,
            identifier: input.parse()?,
            paren: parenthesized!(f in input),
            args: f.parse_terminated(MethodMacroArg::parse)?,
        })
    }
}


#[derive(PartialEq,Eq,Hash)]
pub(crate) struct AutoMethod {
    pub docstring: Vec<Attribute>,
    pub ident: Ident,
    pub paren: Paren,
    pub self_: Option<(Receiver,Option<Token![,]>)>,
    pub args: Punctuated<Type,Token![,]>,
    pub out: Option<Type>,
}

impl ToTokens for AutoMethod {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let docstring = self.docstring.iter();
        let id = &self.ident;
        let self_ = self.self_.as_ref().map(|(r,c)| 
            quote::quote!{
                #r #c
            }
        );
        let args = &self.args;
        let out = self.out.as_ref().map(|t| quote::quote!{-> #t});
        tokens.extend(quote::quote!{
            #(#docstring)*
            #id(#self_ #args) #out
        })
    }
}



impl Parse for AutoMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            docstring: Attribute::parse_outer(input)?,
            ident: input.parse()?,
            paren: parenthesized!(f in input),
            self_: f.fork().parse::<Receiver>().ok()
                .and_then(|_| f.parse().ok())
                .and_then(|v| Some((v,f.parse().ok()?))),
            args: f.parse_terminated(Type::parse)?,
            out: if input.peek(Token![->]) {input.parse::<Token![->]>()?;Some(input.parse()?)} else {None},
        })
    }
}

#[derive(PartialEq,Eq,Hash)]
pub(crate) struct AutoField {
    pub docstring: Vec<Attribute>,
    pub member: Member,
    pub colon: Token![:],
    pub type_: Type,
}

impl Parse for AutoField {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self{
            docstring:  Attribute::parse_outer(input)?,
            member: input.parse()?,
            colon: input.parse()?,
            type_: input.parse()?,
        })
    }
}

impl ToTokens for AutoField {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let docstring = self.docstring.iter();
        let id = &self.member;
        let type_ = &self.type_;
       
        tokens.extend(quote::quote!{
            #(#docstring)*
            #id : #type_
        })    
    }
}
