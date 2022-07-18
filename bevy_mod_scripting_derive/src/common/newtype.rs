
use indexmap::IndexSet;
use proc_macro2::{TokenStream, Span};
use syn::{*, punctuated::*, token::*, parse::{ParseStream, Parse}, spanned::Spanned};

use crate::{lua_method::{LuaMethod}, DeriveFlag, utils::impl_parse_enum};
use quote::{ToTokens};

use super::{WrapperFunction};


pub(crate)  struct NewtypeArgs {
    pub docstring : Vec<Attribute>,
    pub base_type: TypePath,
    pub type_colon : Token![:],
    pub base_type_ident: Ident,
    pub wrapper_type: Ident,
    pub variation: NewtypeVariation,
    pub colon: Option<Token![:]>,
    pub flags: IndexSet<DeriveFlag>
}

impl ToTokens for NewtypeArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let docstrings = self.docstring.iter();
        let base_type = &self.base_type;
        let variation = &self.variation;
        let colon = (self.flags.len() != 0).then(|| quote::quote!{:}).unwrap_or_default();
        let flags = self.flags.iter();
        tokens.extend(quote::quote!{
            #(#docstrings)*
            #base_type : #variation #(#flags)+*
        })
    }
}


impl Parse for NewtypeArgs {
    fn parse(input: ParseStream) -> Result<Self>{
        let docstring = Attribute::parse_outer(input)?;
        let base_type : TypePath = input.parse()?;
        let short_base_type : String = base_type.path.segments.last().ok_or(input.error("Path does not have identifier"))?.ident.to_string();
        let short_wrapper_type : String = format!("Lua{}",short_base_type);
        let sbt_ident = Ident::new(&short_base_type,base_type.span());
        let swt_ident = Ident::new(&short_wrapper_type, base_type.span());
        let mut colon : Option<Token![:]> = None;
        Ok(Self {
            docstring,
            wrapper_type:swt_ident,
            base_type_ident: sbt_ident,
            base_type,
            type_colon: input.parse()?,
            variation: input.parse()?,
            colon: if input.peek(Token![:]){colon = Some(input.parse()?); colon} else {None},
            flags: {
                if colon.is_some(){
                    Punctuated::<DeriveFlag, Token![+]>::parse_separated_nonempty(input)?
                    .into_iter()
                    .collect::<IndexSet<DeriveFlag>>()
                } else {
                    IndexSet::default()
                }
            },
        })
    }
}


impl_parse_enum!(input,ident:
/// What kind of lua wrapper to implement for the newtype
pub(crate) enum NewtypeVariation {
    /// Value types refer to types which implement clone and are
    /// copied when received as lua paramters to a function
    Value => {Ok(Self::Value{ident})},
    /// Ref types do not implement clone and must be passed by value
    /// hence destructuring their original value when they're passed to a lua function.
    /// Use for expensive to copy types.
    Ref => {Ok(Self::Ref{ident})},
    /// Things which have built-in support
    Primitive => {Ok(Self::Primitive{ident})},
}
);


pub(crate) struct WrapperFunctionList<T : WrapperFunction>{
    pub impl_: Token![impl],
    pub braces : Brace,
    pub functions: Punctuated<T,Token![;]>
}

impl <T: WrapperFunction>ToTokens for WrapperFunctionList<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let functions = &self.functions;
        tokens.extend(quote::quote!{
            impl {
                #functions
            }
        })
    }
}

impl <T: WrapperFunction>Parse for WrapperFunctionList<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        let f;
        Ok(Self{
            impl_: input.parse()?,
            braces: braced!(f in input),
            functions: f.parse_terminated(T::parse)?,
        })
    }
}

pub(crate) struct Newtype {
    pub args: NewtypeArgs,
    pub additional_lua_functions: Option<WrapperFunctionList<LuaMethod>>
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = &self.args;
        let functions = &self.additional_lua_functions;
        tokens.extend(quote::quote!(
            {#args #functions}
        ))
    }
}

impl Parse for Newtype {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            args: input.parse()?,
            additional_lua_functions: if input.peek(Token![impl]) && !input.peek2(Token![fn]){
                    Some(input.parse()?)
                } else {
                    None
                }

        })
    }
}

