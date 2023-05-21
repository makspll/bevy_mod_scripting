use bevy_mod_scripting_common::utils::{attribute_to_string_lit, EmptyToken};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Paren,
    *,
};

// Because of the possibility wherein we define the meta method 
// type as both mutuable and static, this enum was created
// to ensure that an invalid state is not representable.
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(crate) enum RhaiMethodMetaType {
    Static,
    MutableSelf,
    ImmutableSelf,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub(crate) enum RhaiFieldFunction {
    Setter,
    Getter,
    Neither,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub(crate) struct RhaiMethodType {
    pub is_function: bool,
    pub field_function: RhaiFieldFunction,
    pub method_type: RhaiMethodMetaType,
    pub method_name: String,
    pub method: ItemFn,
}

impl RhaiMethodType {
    pub fn get_inner_tokens(&self) -> TokenStream {
        self.method_name.into_token_stream()
    }

    pub fn is_field(&self) -> bool {
        self.field_function != RhaiFieldFunction::Neither 
    }

    pub fn is_static(&self) -> bool {
        self.method_type == RhaiMethodMetaType::Static
    }
    
    pub fn is_mut(&self) -> bool {
        self.method_type == RhaiMethodMetaType::MutableSelf
    }
}

impl ToTokens for RhaiMethodType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let meta_type = self.method_type;
    }
}

impl Parse for RhaiMethodType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            is_function: false,
            field_function: RhaiFieldFunction::Setter,
            method_type: RhaiMethodMetaType::Static,
            method_name: String::from("yeet"),
            method: input.parse()?,
        })
    }
}

pub(crate) struct RhaiClosure {
    arrow: Token![=>],
    expr: ExprClosure,
}

impl RhaiClosure {
    pub fn to_applied_closure(&self) -> TokenStream {
        let expr = &self.expr;
        quote_spanned! {self.span()=>
            #expr
        }
    }
}

impl ToTokens for RhaiClosure {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let arrow = &self.arrow;
        let expr = &self.expr;
        tokens.extend(quote::quote! {
            #arrow #expr
        });
    }
}

pub(crate) struct RhaiMethod {
    pub docstring: Vec<Attribute>,
    pub method_type: RhaiMethodType,
    pub closure: RhaiClosure,
}

impl ToTokens for RhaiMethod {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ds: Punctuated<Attribute, EmptyToken> = self.docstring.iter().cloned().collect();
        let mt = self.method_type;
        let method = &self.closure;
        tokens.extend(quote::quote! {
            #ds #mt #method
        })
    }
}
