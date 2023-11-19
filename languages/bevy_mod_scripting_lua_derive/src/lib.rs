use bevy_mod_scripting_common::{input::*, utils::doc_attribute_to_string_lit};
use syn::{parse_macro_input, DeriveInput, Variant};
use syn::{
    parse_quote, spanned::Spanned, AttrStyle, Attribute, Field, Meta, Path, Token, TraitItemFn,
};

use crate::function::*;
use crate::signature::*;
use darling::{FromAttributes, FromDeriveInput};
use function::FunctionAttributes;
use proc_macro::TokenStream;
use proc_macro2::*;
use quote::*;
pub(crate) mod arg;
pub(crate) mod function;
pub(crate) mod signature;
pub(crate) mod visitor;

const SELF_ALIAS: &str = "_self";
const PROXIED_OUT_ALIAS: &str = "__proxied_out";
const PROXY_OUT_ALIAS: &str = "__proxy_out";
const PROXY_PREFIX: &str = "Lua";
const VALID_META_METHODS: [&str; 27] = [
    "Add", "Sub", "Mul", "Div", "Mod", "Pow", "Unm", "IDiv", "BAnd", "BOr", "BXor", "BNot", "Shl",
    "Shr", "Concat", "Len", "Eq", "Lt", "Le", "Index", "NewIndex", "Call", "ToString", "Pairs",
    "IPairs", "Iter", "Close",
];

/// Takes in field with all the required meta and converts it into a
/// TraitItemFn representation
fn convert_field_to_lua_accessor(
    idx: usize,
    field: &Field,
    is_setter: bool,
) -> darling::Result<TraitItemFn> {
    let field_name = field
        .ident
        .clone()
        .unwrap_or_else(|| format_ident!("_{}", idx));
    let field_type = &field.ty;
    let attrs = &field.attrs;
    let mut setter_args: Option<proc_macro2::TokenStream> = None;
    if let Some(attr) = attrs.iter().find(|attr| attr.meta.path().is_ident("lua")) {
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("output") {
                nested.parse_nested_meta(|nested| {
                    setter_args = Some(nested.input.parse()?);
                    Ok(())
                })?
            }
            Ok(())
        })?;
    }
    let setter_arg_attrs = setter_args.map(|tokens| Attribute {
        pound_token: Token![#](field.span()),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList {
            path: Ident::new("proxy", field.span()).into(),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens,
        }),
    });
    let trait_item_method: TraitItemFn = if is_setter {
        parse_quote! {
            #[lua(kind="FieldSetterMethod", raw)]
            #(#attrs)*
            fn #field_name (&mut self, lua: &Lua, #setter_arg_attrs other: #field_type);
        }
    } else {
        parse_quote! {
            #[lua(kind="FieldGetterMethod", raw)]
            #(#attrs)*
            fn #field_name (&self, lua: &Lua) -> #field_type;
        }
    };

    Ok(trait_item_method)
}

/// Given a function with correct meta and the name of the proxied type will generate mlua statement
/// which will register the given function within an appropriate mlua container `UserDataMethods` or `UserDataFields`
/// i.e.:
/// ```rust,ignore
/// /// docs
/// fields.#tealr_function(#signature, #closure)
/// // or
///
/// /// docs
/// methods.#tealr_function(#signature, #closure)
/// ```
/// depending on if the function is a field accessor or a method/function
fn generate_mlua_registration_code(
    proxied_type_path: &Path,
    function_def: TraitItemFn,
) -> darling::Result<proc_macro2::TokenStream> {
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    let attrs = FunctionAttributes::from_attributes(&function_def.attrs)?;
    // if skipping return no-op
    if attrs.skip.is_present() {
        return Ok(Default::default());
    };

    let method_documentation_calls = attrs
        .doc
        .iter()
        .map(|tkns| quote_spanned!(function_def.span()=>methods.document_type(#tkns);))
        .collect::<proc_macro2::TokenStream>();

    let function_name = function_def.sig.ident.clone();
    let output_attrs = attrs
        .output
        .clone()
        .map(|meta| {
            let meta = meta.require_list()?.parse_args::<Meta>()?;
            Ok::<_, syn::Error>(vec![Attribute {
                pound_token: Token![#](meta.span()),
                style: AttrStyle::Outer,
                bracket_token: Default::default(),
                meta,
            }])
        })
        .transpose()?
        .unwrap_or_default();
    let signature = Signature::new(
        proxied_type_path.clone(),
        function_def.sig,
        attrs.raw.is_present(),
        output_attrs,
    )?;
    let function = Function::new(
        function_name.clone(),
        attrs,
        function_def.default,
        signature,
    )?;

    let args = function.generate_mlua_args()?;

    let body = function.generate_mlua_body(proxied_type_path)?;
    let closure = quote_spanned! {body.span()=>
        |#args| {
            #body
        }
    };

    let tealr_function = format_ident!(
        "{}",
        function.attrs.kind.get_tealr_function(),
        span = body.span()
    );
    let signature = function
        .attrs
        .kind
        .is_meta()
        .then(|| {
            // check is valid meta method if not use custom name
            if VALID_META_METHODS.contains(&function_name.to_string().as_str()) {
                quote!(#tealr::mlu::mlua::MetaMethod::#function_name)
            } else {
                let std_string = function_name.to_string();
                quote!(#tealr::mlu::mlua::MetaMethod::Custom(#std_string.to_string()))
            }
        })
        .unwrap_or_else(|| function_name.to_string().to_token_stream());

    let container_ident = if function.attrs.kind.is_field() {
        format_ident!("fields", span = body.span())
    } else {
        format_ident!("methods", span = body.span())
    };

    Ok(quote_spanned! {body.span()=>
        #method_documentation_calls
        #container_ident.#tealr_function(#signature, #closure);
    })
}

#[proc_macro_derive(LuaProxy, attributes(lua, proxy))]
pub fn impl_lua_proxy(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let meta: ProxyInput = match ProxyInput::from_derive_input(&derive_input) {
        Ok(v) => v,
        Err(e) => return darling::Error::write_errors(e).into(),
    };

    if meta.proxy_name.is_some() {
        // throw error
        return syn::Error::new(
            derive_input.span(),
            "The `name` attribute is not supported for lua proxies",
        )
        .to_compile_error()
        .into();
    }

    let proxied_type_path: syn::Path = meta.remote.unwrap_or(meta.ident.clone().into());
    let proxied_type_str = proxied_type_path.segments.last().unwrap().ident.to_string();
    let proxy_type_ident = format_ident!("{PROXY_PREFIX}{}", &meta.ident);
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    // optional clone extensions
    let opt_with_clone = meta
        .derive
        .clone
        .is_present()
        .then_some(quote_spanned! {derive_input.span()=>with Clone})
        .unwrap_or_default();

    let opt_from_lua_proxy = meta.derive.clone.is_present().then_some(
        quote_spanned!{derive_input.span()=>
            impl bevy_script_api::lua::FromLuaProxy<'_> for #proxied_type_path {
                fn from_lua_proxy<'lua>(lua_value: #tealr::mlu::mlua::Value<'lua>, _: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<Self> {
                    if let #tealr::mlu::mlua::Value::UserData(ud) = lua_value{
                        let wrapper = ud.borrow::<#proxy_type_ident>()?;
                        Ok(std::ops::Deref::deref(&wrapper).inner()?)
                    } else {
                        Err(#tealr::mlu::mlua::Error::FromLuaConversionError{
                            from: "Value",
                            to: #proxied_type_str,
                            message: None
                        })
                    }
                }
            }
        }
    ).unwrap_or_default();

    // optionally add debug implementation
    let opt_debug_impl = meta.derive.debug.is_present().then_some(
        quote_spanned!{derive_input.span()=>
            impl std::fmt::Debug for #proxy_type_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))}    
            }
        }
    );

    // generate type level tealr documentation calls
    let type_level_document_calls = meta
        .attrs
        .iter()
        .filter(|&a| a.meta.path().is_ident("doc"))
        .map(doc_attribute_to_string_lit)
        .map(|tkns| quote_spanned!(derive_input.span()=>methods.document_type(#tkns);));

    // generate method equivalents for each field, i.e. unify fields and methods as both can be represented as functions
    let field_methods: Vec<TraitItemFn> = match meta.data {
        darling::ast::Data::<Variant, Field>::Struct(fields) => {
            let mut out: Vec<_> = Default::default();
            let mut errors = darling::Error::accumulator();

            out.extend(
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, field)| {
                        errors.handle_in(|| convert_field_to_lua_accessor(idx, field, false))
                    })
                    .collect::<Vec<_>>(),
            );

            out.extend(
                fields
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, field)| {
                        errors.handle_in(|| convert_field_to_lua_accessor(idx, field, true))
                    })
                    .collect::<Vec<_>>(),
            );

            // short circuit if found any errors
            if let Err(e) = errors.finish() {
                return e.write_errors().into();
            }

            out
        }
        _ => panic!("Enums or Unions are not supported"),
    };

    let mut errors = darling::Error::accumulator();

    // generate both tealr documentation and instantiations of functions and field getters/setters
    let methods = meta
        .functions
        .0
        .into_iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path, v)))
        .collect::<Vec<_>>();

    let fields = field_methods
        .into_iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path, v)))
        .collect::<Vec<_>>();

    // stop if any errors so far
    if let Err(e) = errors.finish() {
        return e.write_errors().into();
    }

    quote_spanned! {derive_input.span()=>

        bevy_script_api::make_script_wrapper!(#proxied_type_path as #proxy_type_ident #opt_with_clone);

        bevy_script_api::impl_tealr_type!(#proxy_type_ident);

        #opt_debug_impl

        #opt_from_lua_proxy

        #[automatically_derived]
        #[allow(unused_parens, unused_braces, unused_mut, unused_variables)]
        #[allow(clippy::all)]
        impl #tealr::mlu::TealData for #proxy_type_ident {
            fn add_methods<'lua, T: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                #(#type_level_document_calls)*
                #(#methods)*
            }

            fn add_fields<'lua, T: #tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
                #(#fields)*
            }
        }

        #[allow(clippy::all, unused_variables)]
        impl bevy_script_api::lua::LuaProxyable for #proxied_type_path {
            fn ref_to_lua<'lua>(self_ : bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>> {
                <#proxy_type_ident as #tealr::mlu::mlua::ToLua>::to_lua(#proxy_type_ident::new_ref(self_),lua)
            }

            fn apply_lua<'lua>(self_ : &mut bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua, new_val: #tealr::mlu::mlua::Value<'lua>) -> #tealr::mlu::mlua::Result<()> {
                if let #tealr::mlu::mlua::Value::UserData(v) = new_val {
                    let other = v.borrow::<#proxy_type_ident>()?;
                    let other = &other;

                    other.apply_self_to_base(self_)?;
                    Ok(())
                } else {
                    Err(#tealr::mlu::mlua::Error::RuntimeError(
                        "Error in assigning to custom user data".to_owned(),
                    ))
                }
            }
        }

        #[allow(clippy::all, unused_variables)]
        impl bevy_script_api::lua::ToLuaProxy<'_> for #proxied_type_path {
            fn to_lua_proxy<'lua>(self, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>>{
                <#proxy_type_ident as #tealr::mlu::mlua::ToLua>::to_lua(#proxy_type_ident::new(self),lua)
            }
        }

    }
    .into()
}

#[cfg(test)]
mod test {

    use crate::function::FunctionAttributes;
    use darling::FromAttributes;
    use syn::TraitItemFn;

    #[test]
    fn test_parse_function_attributes_parses() {
        let function = "
            #[lua(output(proxy))] 
            fn asd(#[proxy] arg: String, #[proxy(Type=\"LuaType\")] arg2: (String, Type)) -> String;
        ";
        let trait_fn: TraitItemFn = syn::parse_str(function).unwrap();

        FunctionAttributes::from_attributes(&trait_fn.attrs).unwrap();
    }
}
