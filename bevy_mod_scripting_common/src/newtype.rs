use std::collections::HashSet;

use indexmap::IndexSet;
use proc_macro2::TokenStream;
use syn::{
    braced,
    parse::{Parse, ParseStream},
    punctuated::*,
    spanned::Spanned,
    token::*,
    Attribute, Ident, Token, TypePath,
};

use crate::{derive_flag::DeriveFlag, utils::EmptyToken};
use quote::ToTokens;

pub struct NewtypeArgs {
    pub docstring: Vec<Attribute>,
    pub base_type: TypePath,
    pub type_colon: Token![:],
    pub base_type_ident: Ident,
    pub wrapper_type: Ident,
    pub flags: IndexSet<DeriveFlag>,
}

impl NewtypeArgs {
    /// Verify the given derive flags
    pub fn verify(self) -> Result<Self, syn::Error> {
        let mut fields = None;
        let mut methods = None;

        self.flags.iter().for_each(|f| match f {
            DeriveFlag::Fields { .. } => fields = Some(f),
            DeriveFlag::Methods { .. } => methods = Some(f),
            _ => {}
        });

        let mut seen_identifiers: HashSet<&Ident> = HashSet::default();
        // verify there aren't any name clashes
        if let (
            Some(DeriveFlag::Fields { fields, .. }),
            Some(DeriveFlag::Methods { methods, .. }),
        ) = (fields, methods)
        {
            for m in methods {
                if seen_identifiers.contains(&m.ident) {
                    return Err(syn::Error::new_spanned(
                        m,
                        format!(
                            "Method name `{}` clashes with another field or method",
                            &m.ident
                        ),
                    ));
                }
                seen_identifiers.insert(&m.ident);
            }

            for f in fields {
                match &f.member {
                    syn::Member::Named(n) => {
                        let contains = if let Some(v) = &f.parsed_attrs.script_name {
                            seen_identifiers.contains(v)
                        } else {
                            seen_identifiers.contains(n)
                        };
                        if contains {
                            return Err(syn::Error::new_spanned(
                                n,
                                format!("Field name `{}` clashes with another field or method, use #[rename(\"new_name\")] to rename the field on script side", n),
                            ));
                        }
                        seen_identifiers.insert(n);
                    }
                    syn::Member::Unnamed(_) => {}
                }
            }
        }

        Ok(self)
    }
}

impl ToTokens for NewtypeArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let docstrings = self.docstring.iter();
        let base_type = &self.base_type;
        let flags = self.flags.iter();
        tokens.extend(quote::quote! {
            #(#docstrings)*
            #base_type : #(#flags)+*
        })
    }
}

impl Parse for NewtypeArgs {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let docstring = Attribute::parse_outer(input)?;
        let base_type: TypePath = input.parse()?;
        let short_base_type: String = base_type
            .path
            .segments
            .last()
            .ok_or_else(|| input.error("Path does not have identifier"))?
            .ident
            .to_string();
        let short_wrapper_type: String = format!("Lua{}", short_base_type);
        let sbt_ident = Ident::new(&short_base_type, base_type.span());
        let swt_ident = Ident::new(&short_wrapper_type, base_type.span());
        let out = Self {
            docstring,
            wrapper_type: swt_ident,
            base_type_ident: sbt_ident,
            base_type,
            type_colon: input.parse()?,
            flags: Punctuated::<DeriveFlag, Token![+]>::parse_separated_nonempty(input)?
                .into_iter()
                .collect::<IndexSet<DeriveFlag>>(),
        };
        out.verify()
    }
}

pub struct WrapperFunctionList {
    pub label: Ident,
    pub impl_: Token![impl],
    pub braces: Brace,
    pub functions: TokenStream,
}

impl ToTokens for WrapperFunctionList {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.label;
        let functions = &self.functions;
        tokens.extend(quote::quote! {
            #ident impl {
                #functions
            }
        })
    }
}

#[allow(clippy::mixed_read_write_in_expression)]
impl Parse for WrapperFunctionList {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let f;
        Ok(Self {
            label: input.parse()?,
            impl_: input.parse()?,
            braces: braced!(f in input),
            functions: f.parse()?,
        })
    }
}

pub struct Newtype {
    pub args: NewtypeArgs,
    pub impl_blocks: Punctuated<WrapperFunctionList, EmptyToken>,
}

impl ToTokens for Newtype {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let args = &self.args;
        let functions = &self.impl_blocks;
        tokens.extend(quote::quote!(
            {#args #functions}
        ))
    }
}

impl Parse for Newtype {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self {
            args: input.parse()?,
            impl_blocks: Punctuated::parse_terminated(input)?,
        })
    }
}
