/// This module contains both `SimpleType` and `syn::Type` visitors to help us with the code generation.
use bevy_mod_scripting_common::input::*;
use proc_macro2::Span;
use proc_macro_error::*;
use quote::*;
use syn::*;

/// Generates an unwrapping expression which can be used to assign the unwrapped proxy to a variable.
/// the argument `#[proxy] arg: MyType` will generate the following expression:
/// ```rust,ignore
/// arg.inner()?;
/// ```
pub(crate) struct LuaSimpleTypeArgumentUnwrapper {
    arg_name: Ident,
    span: Span,
}

impl LuaSimpleTypeArgumentUnwrapper {
    pub fn new(arg_name: Ident, span: Span) -> Self {
        Self { arg_name, span }
    }
}

impl VisitSimpleType<proc_macro2::TokenStream> for LuaSimpleTypeArgumentUnwrapper {
    fn visit_unit(&mut self, _: bool) -> proc_macro2::TokenStream {
        quote_spanned!(self.span=> ())
    }

    fn visit_proxy_type(
        &mut self,
        _: &ProxyType,
        is_child_of_reference: bool,
    ) -> proc_macro2::TokenStream {
        let arg_name: &Ident = &self.arg_name;

        if is_child_of_reference {
            quote_spanned!(self.span=> #arg_name)
        } else {
            quote_spanned!(self.span=> #arg_name.inner()?)
        }
    }

    fn visit_type(&mut self, _type: &Type, _: bool) -> proc_macro2::TokenStream {
        let arg_name: &Ident = &self.arg_name;
        quote_spanned!(self.span=> #arg_name)
    }

    fn visit_unit_path(&mut self, unit_path: &UnitPath, _: bool) -> proc_macro2::TokenStream {
        match unit_path.std_type_ident {
            Some(StdTypeIdent::Option) => {
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;
                quote_spanned!(self.span=>
                    #arg_name.map(|#arg_name| Ok::<_,bevy_mod_scripting_lua::tealr::mlu::mlua::Error>(#inner)).transpose()?
                )
            }
            Some(StdTypeIdent::Vec) => {
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;
                quote_spanned!(self.span=>
                    #arg_name.into_iter().map(|#arg_name| Ok(#inner)).collect::<Result<Vec<_>,bevy_mod_scripting_lua::tealr::mlu::mlua::Error>>()?
                )
            }
            Some(unsupported_std_type) => abort!(
                unit_path.ident,
                "`{}` is not yet supported",
                unsupported_std_type
            ),
            _ => abort!(unit_path.ident, "Unsupported type"),
        }
    }
}

/// `maps` a simple type recursively, expanding the type into a series of map/iter/etc calls where the leaf types are operating over
/// unwrapped proxied types (the inner types) and the output expression produces a wrapped proxy type.
///
/// requires arg_name to be a valid identifier refering to the name of the variable containing a value with the SimpleType being mapped.
/// The returned token stream will be an expression.
pub(crate) struct LuaSimpleTypeWrapper {
    arg_name: Ident,
    span: Span,
}

impl LuaSimpleTypeWrapper {
    pub fn new(arg_name: Ident, span: Span) -> Self {
        Self { arg_name, span }
    }
}

impl VisitSimpleType<proc_macro2::TokenStream> for LuaSimpleTypeWrapper {
    fn visit_unit_path(&mut self, unit_path: &UnitPath, _: bool) -> proc_macro2::TokenStream {
        match unit_path.std_type_ident {
            Some(StdTypeIdent::Option) => {
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;
                quote_spanned!(self.span=>
                    #arg_name.map(|#arg_name| #inner)
                )
            }
            Some(StdTypeIdent::Vec) => {
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;

                quote_spanned!(self.span=>
                    #arg_name.into_iter().map(|#arg_name| #inner).collect::<Vec<_>>()
                )
            }
            Some(unsupported_std_type) => abort!(
                unit_path.ident,
                "`{}` is not yet supported",
                unsupported_std_type
            ),
            _ => abort!(unit_path.ident, "Unsupported type"),
        }
    }

    fn visit_duo_path(&mut self, duo_path: &DuoPath, _: bool) -> proc_macro2::TokenStream {
        let tealr = quote!(bevy_mod_scripting_lua::tealr);

        match duo_path.std_type_ident {
            Some(StdTypeIdent::Result) => {
                let left = self.visit_simple_type(&duo_path.left, false);
                let right = self.visit_simple_type(&duo_path.right, false);
                let arg_name = &self.arg_name;
                quote_spanned!(self.span=>
                    #arg_name.map(|#arg_name| #left).map_err(|#arg_name| #tealr::mlu::mlua::Error::external(#right))
                )
            }
            Some(unsupported_std_type) => abort!(
                duo_path.ident,
                "`{}` is not yet supported",
                unsupported_std_type
            ),
            _ => abort!(duo_path.ident, "Unsupported type"),
        }
    }

    fn visit_unit(&mut self, _: bool) -> proc_macro2::TokenStream {
        quote_spanned!(self.span=>
            ()
        )
    }

    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, _: bool) -> proc_macro2::TokenStream {
        let proxy_ident = &proxy_type.proxy_ident;
        let arg_name = &self.arg_name;
        quote_spanned! {self.span=>
            #proxy_ident::new(#arg_name)
        }
    }

    fn visit_type(&mut self, _type: &Type, _: bool) -> proc_macro2::TokenStream {
        self.arg_name.to_token_stream()
    }
}

/// Wrapper around the `TypeConstructorVisitor` which generates a syn::Type from a `SimpleType`.
/// This is used to handle special cases such as when encountering an outer `Result<T,E>` where E needs to specifically be converted to an `mlua::Error` on the proxy side
pub(crate) struct LuaTypeConstructorVisitor {
    pub general_visitor: TypeConstructorVisitor,
}

impl LuaTypeConstructorVisitor {
    pub fn new(generate_proxy_type: bool, strip_outer_ref: bool) -> Self {
        Self {
            general_visitor: TypeConstructorVisitor::new(generate_proxy_type, strip_outer_ref),
        }
    }
}

impl VisitSimpleType<Type> for LuaTypeConstructorVisitor {
    fn visit_unit(&mut self, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_unit(is_child_of_reference)
    }

    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, is_child_of_reference: bool) -> Type {
        self.general_visitor
            .visit_proxy_type(proxy_type, is_child_of_reference)
    }

    fn visit_type(&mut self, _type: &Type, is_child_of_reference: bool) -> Type {
        self.general_visitor
            .visit_type(_type, is_child_of_reference)
    }

    fn visit_unit_path(&mut self, unit_path: &UnitPath, is_child_of_reference: bool) -> Type {
        self.general_visitor
            .visit_unit_path(unit_path, is_child_of_reference)
    }

    fn visit_duo_path(&mut self, duo_path: &DuoPath, is_child_of_reference: bool) -> Type {
        // this will only trigger for top level types, the deeper nesting is handled by the general visitor
        // outer Result<T,E> needs to be converted to Result<T,mlua::Error> when converting to a proxy_type
        let tealr = quote!(bevy_mod_scripting_lua::tealr);

        if duo_path
            .std_type_ident
            .is_some_and(|i| i == StdTypeIdent::Result)
            && self.general_visitor.generate_proxy_type
        {
            let ident = &duo_path.ident;
            let lt_token = duo_path.lt_token;
            let gt_token = duo_path.gt_token;
            let left = self.visit_simple_type(&duo_path.left, false);
            parse_quote!(#ident #lt_token #left, #tealr::mlu::mlua::Error #gt_token)
        } else {
            self.general_visitor
                .visit_duo_path(duo_path, is_child_of_reference)
        }
    }

    fn visit_reference(
        &mut self,
        reference: &bevy_mod_scripting_common::input::Reference,
        is_child_of_reference: bool,
    ) -> Type {
        self.general_visitor
            .visit_reference(reference, is_child_of_reference)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::visitor::LuaSimpleTypeArgumentUnwrapper;

    use super::LuaSimpleTypeWrapper;
    use bevy_mod_scripting_common::input::{SimpleType, VisitSimpleType};
    use proc_macro2::Span;
    use quote::*;
    use syn::parse_quote;

    #[test]
    pub fn test_lua_argument_wrapper_simple_proxy() {
        let expected = quote_spanned!(Span::call_site()=>
            LuaMyType::new(arg)
        );

        let mut visitor = LuaSimpleTypeWrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(MyType),
                &HashMap::from_iter([(format_ident!("MyType"), None)]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    pub fn test_lua_argument_wrapper_non_proxy() {
        let expected = quote_spanned!(Span::call_site()=>
            arg
        );

        let mut visitor = LuaSimpleTypeWrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(MyType),
                &HashMap::from_iter([]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    pub fn test_lua_argument_wrapper_vec() {
        let expected = quote_spanned!(Span::call_site()=>
            arg.into_iter().map(|arg| LuaMyType::new(arg)).collect::<Vec<_>>()
        );

        let mut visitor = LuaSimpleTypeWrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(Vec<MyType>),
                &HashMap::from_iter([(format_ident!("MyType"), None)]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    pub fn test_lua_argument_unwrapper_simple_proxy() {
        let expected = quote_spanned!(Span::call_site()=>
            arg.inner()?
        );

        let mut visitor =
            LuaSimpleTypeArgumentUnwrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(MyType),
                &HashMap::from_iter([(format_ident!("MyType"), None)]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    pub fn test_lua_argument_unwrapper_non_proxy() {
        let expected = quote_spanned!(Span::call_site()=>
            arg
        );

        let mut visitor =
            LuaSimpleTypeArgumentUnwrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(MyType),
                &HashMap::from_iter([]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }

    #[test]
    pub fn test_lua_argument_unwrapper_vec() {
        let expected = quote_spanned!(Span::call_site()=>
            arg.into_iter().map(|arg| Ok(arg.inner()?)).collect::<Result<Vec<_>, bevy_mod_scripting_lua::tealr::mlu::mlua::Error>>()?
        );

        let mut visitor =
            LuaSimpleTypeArgumentUnwrapper::new(format_ident!("arg"), Span::call_site());

        let output = visitor.visit(
            &SimpleType::new_from_fully_specified_type(
                "Lua",
                &parse_quote!(Vec<MyType>),
                &HashMap::from_iter([(format_ident!("MyType"), None)]),
            )
            .unwrap(),
        );

        assert_eq!(output.to_string(), expected.to_string())
    }
}
