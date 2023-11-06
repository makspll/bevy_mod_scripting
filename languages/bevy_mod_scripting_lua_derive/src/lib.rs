use std::{collections::HashMap, str::FromStr};

use bevy_mod_scripting_common::{input::*, utils::doc_attribute_to_string_lit};

use darling::{FromDeriveInput, FromMeta, FromAttributes, util::{Override, Flag}};
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use proc_macro2::*;
use quote::*;
use strum::*;
use syn::{*, spanned::*, token::{Mut, Paren, Bracket}, punctuated::Punctuated, visit_mut::VisitMut, parse::ParseBuffer};
use visitor::{LuaSimpleTypeArgumentUnwrapper, LuaTypeConstructorVisitor};

use crate::visitor::LuaSimpleTypeWrapper;
pub(crate) mod visitor;


const SELF_ALIAS: &str = "_self";
const PROXIED_OUT_ALIAS: &str = "__proxied_out";
const PROXY_OUT_ALIAS: &str = "__proxy_out";
const PROXY_PREFIX: &str = "Lua";
const VALID_META_METHODS : [&str; 27] = [
    "Add",
    "Sub",
    "Mul",
    "Div",
    "Mod",
    "Pow",
    "Unm",
    "IDiv",
    "BAnd",
    "BOr",
    "BXor",
    "BNot",
    "Shl",
    "Shr",
    "Concat",
    "Len",
    "Eq",
    "Lt",
    "Le",
    "Index",
    "NewIndex",
    "Call",
    "ToString",
    "Pairs",
    "IPairs",
    "Iter",
    "Close",
];


#[derive(Debug, FromAttributes)]
#[darling(attributes(proxy))]
struct LuaProxyArgMeta {
    #[darling(default)]
    map: HashMap<Ident, Ident>
}




#[derive(Debug)]
struct FunctionArgMeta {
    mutable: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    arg_name: Ident,
    span: Span,
    /// variant specific data enumeration
    arg_type: SimpleType,
    /// if an argument is raw, it's passed without any unwrapping to the handler function
    /// if an argument isn't annotated with the `proxy` flag it is technically raw, but this is different for receiver and output arguments
    is_raw: bool,
    
}

impl FunctionArgMeta {
    /// Creates a new meta structure corresponding to the given function argument.
    /// Resolves receivers with the given proxy name.
    /// Uses given span if available, otherwise uses the span of the function argument
    fn new_from_fn_arg(proxied_type_path: &Path, fn_arg: &FnArg, span: Option<Span>, in_raw_function: bool) -> darling::Result<Self> {
        let arg_name;
        let mutable;
        let arg_type;
        let span = span.unwrap_or(fn_arg.span());
        let proxied_type_path = &mut proxied_type_path.clone();
        proxied_type_path.segments.iter_mut().for_each(|v| v.ident.set_span(span));
        let is_raw = in_raw_function;

        // the proxied type is always proxied with the `Lua` prefix
        let mut proxy_ident_map = HashMap::from_iter(
            [(proxied_type_path.segments.last().unwrap().clone().ident,None)]
        );
        match fn_arg {
            FnArg::Receiver(receiver) => {

                arg_type = SimpleType::new_from_fn_arg(PROXY_PREFIX, fn_arg, proxied_type_path, &proxy_ident_map)?;
                mutable = receiver.mutability;
                arg_name = format_ident!("{}", SELF_ALIAS, span=span);
            }
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                
                let is_proxy = attrs.iter().any(|a| a.path().is_ident("proxy"));
                let arg_meta = LuaProxyArgMeta::from_attributes(attrs)?;
                mutable = parse_quote!(mut);
                
                if is_proxy && arg_meta.map.is_empty() {
                    arg_type = SimpleType::new_from_contextual_type_proxy_all(PROXY_PREFIX, ty, proxied_type_path)?;
                } else {
                    proxy_ident_map.extend(arg_meta.map.into_iter().map(|(a,b)| (a,Some(b)))); 
                    arg_type = SimpleType::new_from_contextual_type(PROXY_PREFIX, ty, proxied_type_path, &proxy_ident_map)?;
                }

                arg_name = match pat.as_ref() {
                    Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                    Pat::Wild(_) => return Err(syn::Error::new_spanned(pat, "Cannot use `_` as identifier for proxy function").into()),
                    _ => return Err(syn::Error::new_spanned(pat, "Unsupported parameter pattern").into()),
                };

            }
        }

        Ok(FunctionArgMeta {
            mutable,
            arg_name,
            span,
            arg_type,
            is_raw
        })
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        proxied_type_path: &Path,
        arg_name: Ident,
        arg_type: &Type,
        attrs: Option<Meta>,
        in_raw_function: bool,
    ) -> darling::Result<Self> {
        let ty = Box::new(arg_type.clone());
        let pat_ty = PatType {
            attrs: attrs.map(|attrs| vec![Attribute{
                style: syn::AttrStyle::Outer, 
                meta: attrs, 
                pound_token: syn::token::Pound(arg_name.span()), 
                bracket_token: syn::token::Bracket(arg_name.span())  
            }]).unwrap_or_default(),
            pat: Box::new(Pat::Ident(PatIdent{
                attrs:Vec::default(), 
                by_ref: None, 
                mutability: None, 
                ident: arg_name, 
                subpat: None })
            ),
            colon_token: Default::default(),
            ty,
        };

        let fn_arg = FnArg::Typed(pat_ty);
        Self::new_from_fn_arg(proxied_type_path, &fn_arg, Some(arg_type.span()), in_raw_function)
    }

    /// Unpacks non-reference proxy parameters (using the `inner` method) yielding expressions which correspond to the proxied type with conversion errors being
    /// handled by the try `?` operator.
    pub fn unpack_parameter(&self) -> syn::Result<Option<proc_macro2::TokenStream>> {
        let name = &self.arg_name;
        if self.is_raw {
            // raw parameters DO NOT get unpacked, they get passed directly to the handling method as is
            Ok(None)
        } else {
            // if a proxy parameter is to be passed by value we use inner (which requires Clone to be supported)
            Ok(Some(LuaSimpleTypeArgumentUnwrapper::new(name.clone(), name.span()).visit(&self.arg_type)?))
        }

    }


}


#[derive(Debug, FromAttributes)]
#[darling(attributes(lua))]
struct LuaFunctionAttribute {
    /// Marks the function to be treated as raw meaning a lot of the wrapping and unwrapping is skipped, 
    /// a 'Lua' ctxt argument is then expected
    raw: Flag,

    /// The kind of function to generate on the proxy
    #[darling(default, rename="kind")]
    lua_function_kind: FunctionKind,

    /// Meta to pass down to the output proxy
    output: Option<Meta>,

    /// If passed will generate <T as Trait> statement before calling the method
    /// on the type
    as_trait: Option<Path>,

    #[darling(multiple)]
    doc: Vec<String>,
}


#[derive(Debug)]
struct FunctionMeta<'a> {
    name: &'a Ident,
    body: &'a TraitItemFn,
    fn_type: FunctionKind,
    arg_meta: Vec<FunctionArgMeta>,
    output_meta: FunctionArgMeta,
    as_trait: Option<Path>,
    is_raw: bool,
}

impl FunctionMeta<'_> {

    fn new<'a>(
        proxied_type_path: Path,
        name: &'a Ident,
        body: &'a TraitItemFn,
    ) -> darling::Result<FunctionMeta<'a>> {
        // interpret and validate function meta to instantiate function proxies


        let function_meta = LuaFunctionAttribute::from_attributes(&body.attrs)?;
        let is_raw = function_meta.raw;
        // if no output type is specified, it's set to the unit type `()`
        let output_type = match &body.sig.output {
            ReturnType::Default => Type::Tuple(TypeTuple{ paren_token: Paren::default(), elems: Punctuated::default() }),
            ReturnType::Type(_, t) => *t.to_owned(),
        };

        let mut fn_meta = FunctionMeta {
            name,
            body,
            fn_type: function_meta.lua_function_kind,
            arg_meta: body
                .sig
                .inputs
                .iter()
                .map(|arg| FunctionArgMeta::new_from_fn_arg(&proxied_type_path, arg, None, is_raw.is_present()))
                .collect::<darling::Result::<_>>()?,
            output_meta: FunctionArgMeta::new_from_type(
                    &proxied_type_path,
                    format_ident!("{PROXIED_OUT_ALIAS}", span=body.sig.output.span()),
                    &output_type,
                    function_meta.output.map(|meta| {
                            let list = meta.require_list()?.parse_args::<Meta>()?;
                            Ok::<_,syn::Error>(list)
                    }).transpose()?,
                    is_raw.is_present()
                )?,
            is_raw: is_raw.is_present(),
            as_trait: function_meta.as_trait
        };

        // validate the function against it's meta and correct any correctible mistakes
        fn_meta.validate_function_definition(body)?;

        Ok(fn_meta)
    }


    /// Converts the function's arguments into closure arguments for use in the closures given to mlua calls
    ///
    /// # Example
    /// ```rust,ignore
    /// // the function:
    /// fn foo(self, my_str : String){}
    /// // would convert to
    /// // | _, my_proxy: MyLua, (my_str): (String) |
    /// //   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ <- these bits
    /// ```
    fn generate_mlua_args(&self) -> syn::Result<proc_macro2::TokenStream> {
        let mut args = self.arg_meta.iter();

        let mut self_arg = None;
        if self.fn_type.expects_receiver() {
            if let Some(meta) = args.next() {
                let _mut = &meta.mutable;
                let self_name = &meta.arg_name;
                let self_type = LuaTypeConstructorVisitor::new(true,true).visit(&meta.arg_type);

                self_arg = Some(quote_spanned!(meta.span=> , #self_name : & #_mut #self_type));
            }
        }

        let ctxt_arg = 
            if self.is_raw {
                let meta = args.next().ok_or(syn::Error::new_spanned(self.body,"Expected `Lua` type argument before non-receiver arguments and after the receiver if any."))?;
                let name = &meta.arg_name;
                quote_spanned!(self.name.span()=>#name : &bevy_mod_scripting_lua::tealr::mlu::mlua::Lua)
            } else {
                quote_spanned!(self.name.span()=>_)
            };
        
        if self.fn_type.expects_arguments_other_than_self() {
            let (args, arg_types) = args
            .map(|fn_arg| {
                let _mut = &fn_arg.mutable;
                let name = &fn_arg.arg_name;
                // strip outer refs if the type contains a proxy, we cannot have any references in type position
                // we can still have reference semantics since we have a proxy object, however we still pass it by value
                let type_path = LuaTypeConstructorVisitor::new(true,fn_arg.arg_type.contains_proxy_type()).visit(&fn_arg.arg_type);
                (
                    quote_spanned!(fn_arg.span=>#_mut #name),
                    quote_spanned!(fn_arg.span=>#type_path),
                )
            })
            .unzip::<_, _, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>>();

            Ok(quote_spanned!(self.body.sig.inputs.span()=>#ctxt_arg #self_arg , (#(#args),*) : (#(#arg_types),*)))
        } else {
            Ok(quote_spanned!(self.body.sig.inputs.span()=>#ctxt_arg #self_arg))
        }

    }

    /// Takes all the argument identifiers passed into the function, generates assignments which shadow the original
    /// identifiers but modifies the parameter types if required by unpacking proxies. This is done via `.inner` calls on proxy wrappers
    /// 
    /// For example for the type `MyType` 
    /// `fn my_fn(self, #[proxy] other_ref: &Self, #[proxy] other: Self)`
    /// 
    /// will generate the following statements:
    /// ```rust,ignore 
    /// let _self : MyType = self.inner();
    /// let other_ref : LuaMyType = other; // note this one is kept as a wrapper, and dealt in another process
    /// let other : MyType = other_ref.inner();
    /// ```
    fn generate_mlua_body_unwrapped_parameter_assignments(&self) -> syn::Result<proc_macro2::TokenStream> {        
        Ok(self.arg_meta
            .iter()
            .map(FunctionArgMeta::unpack_parameter)
            .collect::<Result<Vec<_>>>()?
            .iter()
            .zip(self.arg_meta.iter())
            .filter_map(|(unpacked_param, param)| 
                unpacked_param.as_ref().map(|unpacked_param| {
                    let name = &param.arg_name;
                    quote_spanned!{name.span()=>let #name = #unpacked_param;}
                }
            ))
            .collect::<proc_macro2::TokenStream>())
    }


    /// Generates the statement which calls the proxied function with the same argument names as in the function declaration 
    /// and stores the output in a variable with the given identifier. Static methods, are called against the given `proxied_name`
    /// 
    /// For example for the type `MyType` with proxied ident `__proxied_out`
    /// `fn my_fn(self, #[proxy] other_ref: &Self, #[proxy] other: Self) -> Self`
    /// 
    /// will generate the following statement: 
    /// ```rust,ignore 
    /// let __proxied_out : MyType = self.my_fn(other_ref, other);
    /// ```
    fn generate_mlua_body_proxied_output_stmt(&self, proxied_output_ident: &Ident, proxied_type_path: &Path) -> syn::Result<proc_macro2::TokenStream> {
    // generate function call on proxied type (operating over unwrapped parameters)
        // output will be stored in out_ident with the proxied_output_type
        // the type before we wrap it in a proxy
        let proxied_output_type = LuaTypeConstructorVisitor::new(false,false).visit(&self.output_meta.arg_type);

        let function_name = self.name;
        let param_names = self.arg_meta.iter()
            .map(|arg| &arg.arg_name).collect::<Vec<_>>();
        match &self.body.default{
            Some(body) => {
                
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor{
                        target: "self",
                        replacement: SELF_ALIAS,
                    }.visit_stmt_mut(&mut s);
                    s
                });

                Ok(quote_spanned!{body.span()=>
                    let #proxied_output_ident : #proxied_output_type = 
                        (||{
                            #(#stmts)*
                        })();
                })          
            },
            None => {
                // override this span, as otherwise spans propagate weird
                let mut proxied_name = proxied_type_path.clone();
                proxied_name.segments.iter_mut().for_each(|v| v.ident.set_span(self.body.sig.ident.span()));
                let method_path = self.as_trait.as_ref()
                    .map(|trait_path| quote_spanned!(self.body.sig.paren_token.span=>
                        #trait_path::#function_name))
                    .unwrap_or(quote_spanned!(self.body.sig.paren_token.span=>
                        #proxied_name::#function_name
                    ));
                Ok(quote_spanned! {self.body.sig.paren_token.span=>
                    let #proxied_output_ident : #proxied_output_type = 
                        #method_path(#(#param_names),*);
                })
            },
        }
    }

    /// Generates a wrapping statement, which if the type present in the `proxied_output_ident` variable needs to be wrapped into a proxy constructor, will do so and assign 
    /// the output to the given `proxy_output_ident`.
    /// 
    /// For example for the type: `MyType` with `__proxy_out output` identifier
    /// the function: `fn my_fn(self, #[proxy] other_ref: &Self, #[proxy] other: Self) -> Self`
    /// will generate the following statement:
    /// ```rust,ignore 
    /// let __proxy_out : LuaMyType =  LuaMyType::new(__proxied_out);
    /// ```
    fn generate_mlua_body_proxy_output_stmt(&self, proxied_output_ident: &Ident, proxy_output_ident: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        if self.output_meta.is_raw {
            return Ok(quote_spanned! {self.body.span()=>
                let #proxy_output_ident = #proxied_output_ident;
            })
        }
        
        // generate `new` calls as required to build proxy stored in out_ident
        let constructor_wrapped_expression = 
            LuaSimpleTypeWrapper::new(proxied_output_ident.clone(), proxied_output_ident.span())
                .visit(&self.output_meta.arg_type)?;

        // the type of the wrapped type (if wrapped)
        let proxy_output_type = LuaTypeConstructorVisitor::new(true,false).visit(&self.output_meta.arg_type);
        
        // the statement assigning the proxy output to proxy_output_ident
        Ok(quote_spanned! {self.body.span()=>
            let #proxy_output_ident : #proxy_output_type = #constructor_wrapped_expression;
        })
    }

    fn generate_mlua_body(&self, proxied_type_path: &Path) -> syn::Result<proc_macro2::TokenStream> {
        let unpacked_parameter_declarations = self.generate_mlua_body_unwrapped_parameter_assignments()?;

        let proxied_output_ident = format_ident!("{PROXIED_OUT_ALIAS}", span=self.output_meta.span);
        let proxy_output_ident: Ident = format_ident!("{PROXY_OUT_ALIAS}", span=self.output_meta.span);

        let proxied_output_stmt = self.generate_mlua_body_proxied_output_stmt(&proxied_output_ident, proxied_type_path)?;
        let proxy_output_stmt = self.generate_mlua_body_proxy_output_stmt(&proxied_output_ident, &proxy_output_ident)?;

        // determine if we need to wrap the output in an Ok() statement
        let last_stm = match &self.output_meta.arg_type {
            SimpleType::DuoPath(DuoPath{ ident , ..}) if *ident == "Result" => quote_spanned! {self.body.span()=>#proxy_output_ident},
            _ => quote_spanned! {self.body.span()=>Ok(#proxy_output_ident)},
        };

        let conversion_body_stms = quote!(
            #proxied_output_stmt
            #proxy_output_stmt 
            #last_stm
        );
        
        // for every argument which is a reference, we need a separate sort of call,
        // we cannot use `v.inner()` since this operates over values, we must use `val_mut` or `val` to get a reference to the wrapped
        // structure for the duration of the call 
        let conversion_body_surrounded_with_dereferening_stms = self.arg_meta.iter()
            .fold(conversion_body_stms, |acc, arg_meta| {
                    // only proxy types which are directly inside a reference are supported as references
                    if !matches!(arg_meta.arg_type, SimpleType::Reference(Reference{ ref inner, .. }) 
                        if matches!(inner.as_ref(), SimpleType::ProxyType(_))){
                        return acc;
                    }
                    // raw arguments are passed directly to the handler function
                    if arg_meta.is_raw {
                        return acc;
                    }

                    let method_call = if arg_meta.arg_type.has_outer_mut_ref() {
                        format_ident!("val_mut", span=arg_meta.span)
                    } else {
                        format_ident!("val", span=arg_meta.span)
                    };

                    let arg_name = &arg_meta.arg_name;

                    quote_spanned!{self.body.span()=>{
                        #arg_name.#method_call(|#arg_name| {#acc})?
                    }}
                }
        );

        Ok(quote!(   
            #unpacked_parameter_declarations
            #conversion_body_surrounded_with_dereferening_stms
        ))
    }

    
    
    fn validate_function_definition(&mut self, definition: &TraitItemFn) -> syn::Result<()> {
        if self.output_meta.arg_type.has_ref() && self.output_meta.arg_type.contains_proxy_type() {
            return Err(syn::Error::new(
                self.output_meta.span,
                "Lua proxy functions do not support non 'static types as return values yet".to_string())
            )
        }

        if self.is_raw {
            let ctxt_arg_idx = if self.fn_type.expects_receiver() {1} else {0};

            let ctxt_arg = definition.sig.inputs.iter().nth(ctxt_arg_idx)
                .ok_or_else(|| syn::Error::new_spanned(&definition.sig.inputs, 
                        "Raw Lua proxy functions require a `&Lua` context argument following any receivers and before other arguments."))?;

            let success = match ctxt_arg {
                FnArg::Typed(PatType{ ty, .. }) => matches!(ty.as_ref(), &Type::Reference(_)),
                _ => false
            };

            if !success {
                return Err(syn::Error::new_spanned(ctxt_arg, "Raw Lua proxy functions require a `&Lua` reference after any receivers and before other arguments."))
            }
        }

        if self.fn_type.expects_receiver() {
            if let Some(receiver) = definition.sig.receiver() {
                // check receiver mutability
                if self.fn_type.expects_mutable_receiver() != receiver.mutability.is_some() {
                    // if incorrect and this is a method correct
                    if self.fn_type.is_method() {
                        // swap mutability in the kind
                        self.fn_type = match self.fn_type {
                            FunctionKind::Method => FunctionKind::MutatingMethod,
                            FunctionKind::MetaMethod => FunctionKind::MutatingMetaMethod,
                            FunctionKind::MutatingMethod => FunctionKind::Method,
                            FunctionKind::MutatingMetaMethod => FunctionKind::MetaMethod,
                            _ => unreachable!()
                        }
                    } else {
                        return Err(syn::Error::new_spanned(
                            receiver,
                            format!(
                                "Lua proxy functions of type: {}, require {} receiver, did you specify `kind` meta correctly?",
                                self.fn_type,
                                if self.fn_type.expects_mutable_receiver() { "&mut self or mut self" } else { "&self or self" }
                            )
                        ));
                    }
                };
            } else {
                return Err(syn::Error::new(
                    definition.sig.paren_token.span.span(),
                    format!(
                        "Lua proxy functions of type: {}, require `self` argument",
                        self.fn_type
                    )
                ));
            }
        } else if definition.sig.receiver().is_some() {
            return Err(syn::Error::new_spanned(
                definition.sig.receiver().unwrap(),
                format!(
                    "Lua proxy functions of type: {}, do not expect a receiver argument",
                    self.fn_type
                )
            ));
        }

        Ok(())
    }
}

#[derive(Default, FromMeta, Display, EnumString, EnumIter, PartialEq, Eq, Clone, Copy, Debug)]
enum FunctionKind {
    Function,
    MetaFunction,
    #[default]
    Method,
    MetaMethod,
    MutableFunction,
    MutableMetaFunction,
    MutatingMethod,
    MutatingMetaMethod,
    FieldGetterMethod,
    FieldSetterMethod,
}


impl FunctionKind {
    fn expects_receiver(self) -> bool {
        self == FunctionKind::Method
            || self == FunctionKind::MetaMethod
            || self == FunctionKind::MutatingMethod
            || self == FunctionKind::MutatingMetaMethod
            || self == FunctionKind::FieldGetterMethod
            || self == FunctionKind::FieldSetterMethod
    }

    /// True if this is a method of any kind (not field setter/getter)
    fn is_method(self) -> bool {
        self == FunctionKind::Method 
            || self == FunctionKind::MetaMethod
            || self == FunctionKind::MutatingMetaMethod
    }

    fn expects_mutable_receiver(self) -> bool {
        self == FunctionKind::MutatingMethod 
            || self == FunctionKind::FieldSetterMethod
    }

    fn is_field(self) -> bool {
        self == FunctionKind::FieldGetterMethod ||
            self == FunctionKind::FieldSetterMethod
    }

    fn expects_arguments_other_than_self(self) -> bool {
        self != FunctionKind::FieldGetterMethod 
    }

    fn get_tealr_function(self) -> &'static str {
        match self {
            FunctionKind::Function => "add_function",
            FunctionKind::MetaFunction => "add_meta_function",
            FunctionKind::Method => "add_method",
            FunctionKind::MetaMethod => "add_meta_method",
            FunctionKind::MutableFunction => "add_function_mut",
            FunctionKind::MutableMetaFunction => "add_meta_function_mut",
            FunctionKind::MutatingMethod => "add_method_mut",
            FunctionKind::MutatingMetaMethod => "add_meta_method_mut",
            FunctionKind::FieldGetterMethod => "add_field_method_get",
            FunctionKind::FieldSetterMethod => "add_field_method_set",
        }
    }

    fn is_meta(self) -> bool {
        self == FunctionKind::MetaMethod
            || self == FunctionKind::MetaFunction
            || self == FunctionKind::MutatingMetaMethod
            || self == FunctionKind::MutableMetaFunction
    }

}


/// Takes in field with all the required meta and converts it into a 
/// TraitItemFn representation
fn convert_field_to_lua_accessor(idx: usize, field: &Field, is_setter: bool) -> darling::Result<TraitItemFn> {
    let field_name = field.ident.clone().unwrap_or_else(|| format_ident!("_{}", idx));
    let field_type = &field.ty;
    let attrs = &field.attrs;
    
    let trait_item_method : TraitItemFn = if is_setter {
        parse_quote!{
            #[lua(kind="FieldSetterMethod", raw)]
            #(#attrs)*
            fn #field_name (&mut self, lua: &Lua, other: #field_type) {
                // let world_ptr = <bevy_mod_scripting_lua::tealr::mlu::mlua::Lua as bevy_script_api::common::bevy::GetWorld>::get_world(lua)?;

                // self.#field_name;
                ()
            }
        }
    } else {
        parse_quote!{
            #[lua(kind="FieldGetterMethod", raw)]
            #(#attrs)*
            fn #field_name (&self, lua: &Lua) -> #field_type {
                self.#field_name
            }
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
fn generate_mlua_registration_code(proxied_type_path: &Path, body: &TraitItemFn) -> darling::Result<proc_macro2::TokenStream> {
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    let function_meta = LuaFunctionAttribute::from_attributes(&body.attrs)?;
    let method_documentation_calls = function_meta.doc
        .iter()
        .map(|tkns| quote_spanned!(body.span()=>methods.document_type(#tkns);));


    let fn_meta = FunctionMeta::new(proxied_type_path.clone(), &body.sig.ident, body)?;

    let args = fn_meta.generate_mlua_args()?;

    let body = fn_meta.generate_mlua_body(proxied_type_path)?;
    let closure = quote_spanned! {body.span()=>
        |#args| {
            #body
        }
    };

    let tealr_function = format_ident!("{}", fn_meta.fn_type.get_tealr_function(), span=body.span());
    let signature = fn_meta
        .fn_type
        .is_meta()
        .then(|| {
            let name = fn_meta.name;

            // check is valid meta method if not use custom name
            if VALID_META_METHODS.contains(&name.to_string().as_str()) {
                quote!(#tealr::mlu::mlua::MetaMethod::#name)
            } else {
                let std_string = name.to_string();
                quote!(#tealr::mlu::mlua::MetaMethod::Custom(#std_string.to_string()))
            }
        })
        .unwrap_or_else(|| fn_meta.name.to_string().to_token_stream());

    let container_ident = if fn_meta.fn_type.is_field() {
        format_ident!("fields", span=body.span())
    } else {
        format_ident!("methods", span=body.span())
    };

    
    Ok(quote_spanned! {body.span()=>
        #(#method_documentation_calls)*
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
            "The `name` attribute is not supported for lua proxies"
        ).to_compile_error().into();
    }

    let proxied_type_path : syn::Path = meta.remote.unwrap_or(meta.ident.clone().into());
    let proxy_type_ident = format_ident!("{PROXY_PREFIX}{}", &meta.ident);

    // generate the type definition of the proxy
    let mut definition: proc_macro2::TokenStream;

    // generate main type with optional Clone extension
    if meta.derive.clone.is_present() {
        definition = quote_spanned! {derive_input.span()=>
            bevy_script_api::make_script_wrapper!(#proxied_type_path as #proxy_type_ident with Clone);
        };
    } else {
        definition = quote_spanned! {derive_input.span()=>
            bevy_script_api::make_script_wrapper!(#proxied_type_path as #proxy_type_ident);
        }
    }

    // optionally add debug implementation
    if meta.derive.debug.is_present() {
        definition.extend(quote_spanned!{derive_input.span()=>
            impl std::fmt::Debug for #proxy_type_ident {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))}    
            }
        });
    }

    let tealr_type_implementations = quote_spanned! {derive_input.span()=>
        bevy_script_api::impl_tealr_type!(#proxy_type_ident);
    };

    // generate type level tealr documentation calls
    let type_level_document_calls = meta
        .attrs
        .iter()
        .filter(|&a| a.meta.path().is_ident("doc")).map(doc_attribute_to_string_lit)
        .map(|tkns| quote_spanned!(derive_input.span()=>methods.document_type(#tkns);));

    // generate method equivalents for each field, i.e. unify fields and methods as both can be represented as functions
    let field_methods : Vec<TraitItemFn> = match meta.data {
        darling::ast::Data::<Variant,Field>::Struct(fields) => {
            let mut out : Vec<_> = Default::default();
            let mut errors = darling::Error::accumulator();

            out.extend(fields.iter()
                .enumerate()
                .filter_map(|(idx,field)| errors.handle_in(|| convert_field_to_lua_accessor(idx,field,false)))
                .collect::<Vec<_>>());
            
            out.extend(fields.iter()
                .enumerate()
                .filter_map(|(idx,field)| errors.handle_in(|| convert_field_to_lua_accessor(idx,field,true)))
                .collect::<Vec<_>>());
            
            // short circuit if found any errors
            if let Err(e) = errors.finish() {
                return e.write_errors().into();
            }

            out
        },
        _ => panic!("Enums or Unions are not supported")
    };


    let mut errors = darling::Error::accumulator();

    // generate both tealr documentation and instantiations of functions and field getters/setters
    let methods = meta.functions.iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path,v)))
        .collect::<Vec<_>>();

    let fields = field_methods.iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path,v)))
        .collect::<Vec<_>>();

    // stop if any errors so far
    if let Err(e) = errors.finish() {
        return e.write_errors().into();
    }
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    let a = quote_spanned! {derive_input.span()=>

        #definition

        #tealr_type_implementations

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
    .into();
    // panic!("{}",a);
    a
}


#[cfg(test)]
mod test {
    use darling::{FromAttributes, FromMeta};
    use quote::format_ident;
    use syn::{TraitItemFn, Meta};

    use crate::{LuaFunctionAttribute, FunctionArgMeta, LuaProxyArgMeta};



    #[test]
    fn test_parse_function_attributes_parses(){
        let function = "
            #[lua(output(proxy))] 
            fn asd(#[proxy] arg: String, #[proxy(Type=\"LuaType\")] arg2: (String, Type)) -> String;
        ";
        let trait_fn : TraitItemFn = syn::parse_str(function).unwrap();

        LuaFunctionAttribute::from_attributes(&trait_fn.attrs).unwrap();
    }
}