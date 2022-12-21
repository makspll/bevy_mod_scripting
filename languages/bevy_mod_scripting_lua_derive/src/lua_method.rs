use bevy_mod_scripting_common::utils::{attribute_to_string_lit, EmptyToken};
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    *,
};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub(crate) struct MethodMacroArg {
    pub ident: Ident,
    pub equals: Token![=],
    pub replacement: TypePath,
}

impl Parse for MethodMacroArg {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            equals: input.parse()?,
            replacement: input.parse()?,
        })
    }
}

impl ToTokens for MethodMacroArg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.ident;
        let rep = &self.replacement;
        tokens.extend(quote::quote! {
            #id = #rep
        })
    }
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub(crate) struct LuaMethodType {
    /// does it take &mut  self ?
    pub is_mut: bool,
    /// should it be inlined into the global API ?
    pub is_static: bool,
    /// is it part of the metatable?
    pub is_meta: bool,
    /// does it take self as first parameter?
    pub is_function: bool,
    /// is it a field setter
    pub is_field_getter: bool,
    /// is it a field getter
    pub is_field_setter: bool,

    /// if is_meta this will be Some
    meta_method: Option<TypePath>,
    /// if !is_meta this will be Some
    method_name: Option<LitStr>,
}

impl LuaMethodType {
    pub fn is_field(&self) -> bool {
        self.is_field_setter || self.is_field_getter
    }

    pub fn get_inner_tokens(&self) -> TokenStream {
        if self.is_meta {
            return self.meta_method.as_ref().unwrap().into_token_stream();
        } else {
            return self.method_name.as_ref().unwrap().into_token_stream();
        }
    }
}

impl Parse for LuaMethodType {
    fn parse(input: ParseStream) -> Result<Self> {
        let (is_field_setter, is_field_getter) = input
            .peek(Ident)
            .then(|| {
                let ident_str = input.parse::<Ident>().unwrap().to_string();

                (ident_str == "set", ident_str == "get")
            })
            .unwrap_or((false, false));

        let is_static = input
            .peek(Token![static])
            .then(|| input.parse::<Token![static]>().unwrap())
            .is_some();
        let is_mut = input
            .peek(Token![mut])
            .then(|| input.parse::<Token![mut]>().unwrap())
            .is_some();
        let is_function = input
            .peek(Token![fn])
            .then(|| input.parse::<Token![fn]>().unwrap())
            .is_some();

        let mut method_name = None;
        let mut meta_method = None;
        let mut is_meta = false;

        if input.peek(Paren) {
            // meta method
            let f;
            parenthesized!(f in input);
            is_meta = true;
            meta_method = Some(f.parse()?);
        } else {
            method_name = Some(input.parse()?);
        }

        Ok(Self {
            is_mut,
            is_static,
            is_meta,
            is_function,
            meta_method,
            method_name,
            is_field_getter,
            is_field_setter,
        })
    }
}

impl ToTokens for LuaMethodType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let is_static = self.is_static.then(|| Token![static](tokens.span()));
        let is_mut = self.is_mut.then(|| Token![mut](tokens.span()));
        let is_function = self.is_function.then(|| Token![fn](tokens.span()));
        let mut inner = self.get_inner_tokens();
        let field_ident = self
            .is_field_setter
            .then(|| format_ident!("set"))
            .or_else(|| self.is_field_getter.then(|| format_ident!("get")));

        if self.is_meta {
            inner = quote::quote! {
                (#inner)
            }
        };
        tokens.extend(quote::quote! {
           #field_ident #is_static #is_mut #is_function #inner
        })
    }
}

pub(crate) struct LuaClosure {
    arrow: Token![=>],
    expr: ExprClosure,
}

impl Parse for LuaClosure {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            arrow: input.parse()?,
            expr: input.parse()?,
        })
    }
}

impl LuaClosure {
    pub fn to_applied_closure(&self) -> TokenStream {
        let expr = &self.expr;
        quote_spanned! {self.span()=>
            #expr
        }
    }
}

impl ToTokens for LuaClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arrow = &self.arrow;
        let expr = &self.expr;
        tokens.extend(quote::quote! {
            #arrow #expr
        });
    }
}

pub(crate) struct LuaMethod {
    pub docstring: Vec<Attribute>,
    pub method_type: LuaMethodType,
    pub closure: LuaClosure,
}

impl Parse for LuaMethod {
    fn parse(input: ParseStream) -> Result<Self> {
        let docstring = Attribute::parse_outer(input)?;
        Ok(Self {
            docstring,
            method_type: input.parse()?,
            closure: input.parse()?,
        })
    }
}

impl ToTokens for LuaMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ds: Punctuated<Attribute, EmptyToken> = self.docstring.iter().cloned().collect();

        let mt = &self.method_type;
        let closure = &self.closure;
        tokens.extend(quote::quote! {
            #ds #mt #closure
        })
    }
}

impl LuaMethod {
    /// Generates the function call expression corresponding to the mlua
    /// UserData method which implements the given method or field
    ///
    /// For field setters and getters teh receiver must be an instance of
    /// [`UserDataFields`] and [`UserDataMethods`] otherwise
    pub fn to_call_expr(&self, receiver: &'static str) -> TokenStream {
        let closure = &self.closure.to_applied_closure();
        let receiver = Ident::new(receiver, Span::call_site());

        let ds: TokenStream = self
            .docstring
            .iter()
            .map(|v| {
                let ts: TokenStream = attribute_to_string_lit(v);
                if ts.is_empty() {
                    Default::default()
                } else {
                    quote_spanned! {self.span()=>
                        #receiver.document(#ts);
                    }
                }
            })
            .collect();
        let call_ident = if self.method_type.is_field_getter || self.method_type.is_field_setter {
            format_ident!(
                "add_field_method_{}",
                self.method_type
                    .is_field_getter
                    .then_some("get")
                    .or_else(|| self.method_type.is_field_setter.then_some("set"))
                    .unwrap()
            )
        } else {
            format_ident!(
                "add{}{}{}",
                if self.method_type.is_meta {
                    "_meta"
                } else {
                    ""
                },
                if self.method_type.is_function {
                    "_function"
                } else {
                    "_method"
                },
                if self.method_type.is_mut { "_mut" } else { "" },
            )
        };

        let inner_tokens = self.method_type.get_inner_tokens();

        quote_spanned! {self.span()=>
            #ds
            #receiver.#call_ident(#inner_tokens,#closure);
        }
    }
}
