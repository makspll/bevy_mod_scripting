

use proc_macro2::Span;
use syn::{*, parse::*, punctuated::*, token::{*},Type};

use crate::{ops::*,lua_method::{LuaMethodType, MethodMacroArg},TokenStream2, utils::impl_parse_enum};



use quote::ToTokens;

use super::arg::ArgType;




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
    pub self_: Option<(ArgType,Token![:])>,
    pub args: Punctuated<ArgType,Token![,]>,
    pub out: Option<ArgType>,
}

impl ToTokens for AutoMethod {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let docstring = self.docstring.iter();
        let id = &self.ident;
        let args = &self.args;
        let self_ = self.self_.as_ref().map(|(a,_)| quote::quote!(#a:));
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
        let o = Ok(Self{
            docstring: Attribute::parse_outer(input)?,
            ident: input.parse()?,
            paren: parenthesized!(f in input),
            self_: {
                let parser = |p : ParseStream| 
                    Ok::<_,syn::Error>((p.parse::<ArgType>()?,p.parse::<Token![:]>()?));  
                let fork = f.fork();
                if let Ok(_) = parser(&fork) {
                    Some(parser(&f).expect("Something went wrong"))
                } else {
                    None
                }
            },
            args: f.parse_terminated(ArgType::parse)?,
            out: if input.peek(Token![->]) {input.parse::<Token![->]>()?;Some(input.parse()?)} else {None}
        });
        o
    }
}

#[derive(PartialEq,Eq,Hash)]
pub(crate) struct AutoField {
    pub docstring: Vec<Attribute>,
    pub member: Member,
    pub colon: Token![:],
    pub type_: ArgType,
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
