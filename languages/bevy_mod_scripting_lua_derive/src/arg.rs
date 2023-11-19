use std::collections::HashMap;

use bevy_mod_scripting_common::input::{SimpleType, VisitSimpleType};
use darling::FromAttributes;
use proc_macro2::{Ident, Span};
use quote::{quote, quote_spanned};
use syn::{parse_quote, token::Mut};

use crate::visitor::{LuaSimpleTypeArgumentUnwrapper, LuaTypeConstructorVisitor};

#[derive(Debug, FromAttributes)]
#[darling(attributes(proxy))]
pub struct ArgAttributes {
    #[darling(default)]
    pub map: HashMap<Ident, Ident>,
}

/// Struct for holding argument/output information for functions passed via `functions[..]` meta
#[derive(Debug)]
pub struct Arg {
    pub attrs: ArgAttributes,
    pub mutability: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    pub name: Ident,
    /// variant specific data enumeration
    pub type_: SimpleType,
    /// if an argument is raw, it's passed without any unwrapping to the handler function
    /// if an argument isn't annotated with the `proxy` flag it is technically raw, but this is different for receiver and output arguments
    pub is_raw: bool,
    pub span: Span,
}

impl Arg {
    pub fn new(
        attrs: ArgAttributes,
        name: Ident,
        mutability: Option<Mut>,
        type_: SimpleType,
        is_raw: bool,
    ) -> Self {
        Self {
            attrs,
            mutability,
            span: name.span(),
            name,
            type_,
            is_raw,
        }
    }

    /// Unpacks non-reference proxy parameters (using the `inner` method) yielding expressions which correspond to the proxied type with conversion errors being
    /// handled by the try `?` operator.
    pub fn unpack_parameter(&self) -> syn::Result<Option<proc_macro2::TokenStream>> {
        let name = &self.name;
        if self.is_raw {
            // raw parameters DO NOT get unpacked, they get passed directly to the handling method as is
            Ok(None)
        } else {
            // if a proxy parameter is to be passed by value we use inner (which requires Clone to be supported)
            Ok(Some(
                LuaSimpleTypeArgumentUnwrapper::new(name.clone(), name.span())
                    .visit(&self.type_)?,
            ))
        }
    }

    fn arg_signature_generic(
        &self,
        expecting_receiver: bool,
        expecting_ctxt: bool,
    ) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        assert!(!(expecting_receiver && expecting_ctxt));

        let _mut = &self.mutability;
        let name = &self.name;
        let type_ = if expecting_ctxt {
            parse_quote!(&bevy_mod_scripting_lua::prelude::Lua)
        } else {
            LuaTypeConstructorVisitor::new(true, self.type_.contains_proxy_type())
                .visit(&self.type_)
        };
        let forced_ref = expecting_receiver.then(|| {
            Some(quote_spanned!(self.span=>
                & #_mut
            ))
        });

        let name_part = quote_spanned!(self.span=>
            #_mut #name
        );
        let type_part = quote_spanned!(self.span=>
            #forced_ref #type_
        );
        (name_part, type_part)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a receiver type argument.
    /// generates using an additional outer reference.
    pub fn arg_signature_receiver(&self) -> proc_macro2::TokenStream {
        let (name, type_) = self.arg_signature_generic(true, false);
        quote!(#name : #type_)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a Lua context type argument.
    /// generates using an additional outer reference.
    pub fn arg_signature_context(&self) -> proc_macro2::TokenStream {
        let (name, type_) = self.arg_signature_generic(false, true);
        quote!(#name : #type_)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a non-receiver non-context argument.
    /// generates the type to match the argument received.
    /// The output is split into the name and type parts
    pub fn arg_signature(&self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        self.arg_signature_generic(false, false)
    }
}
