use std::{collections::{HashMap, HashSet}, str::FromStr};

use bevy_mod_scripting_common::{
    input::*,
    implementor::*, 
    newtype::*, utils::attribute_to_string_lit,
};
use implementor::LuaImplementor;
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use proc_macro2::*;
use proc_macro_error::*;
use quote::*;
use strum::*;
use syn::{*, spanned::*, token::{Mut, Paren, Bracket}, punctuated::Punctuated, visit_mut::VisitMut};
use visitor::{LuaSimpleTypeArgumentUnwrapper, LuaTypeConstructorVisitor};

use crate::visitor::LuaSimpleTypeWrapper;

pub(crate) mod derive_flags;
pub(crate) mod implementor;
pub(crate) mod impls;
pub(crate) mod lua_method;
pub(crate) mod visitor;

#[proc_macro]
pub fn impl_lua_newtype(tokens: TokenStream) -> TokenStream {
    let newtype = parse_macro_input!(tokens as Newtype);
    let mut implementor = LuaImplementor::default();

    implementor
        .generate(newtype)
        .map_err(|e| e.to_compile_error())
        .unwrap_or_else(core::convert::identity)
        .into()
}

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
    fn new_from_fn_arg(proxied_type_identifier: &Ident, fn_arg: &FnArg, span: Option<Span>, in_raw_function: bool) -> Self {
        let arg_name;
        let mutable;
        let arg_type;
        let span = span.unwrap_or(fn_arg.span());
        let proxied_type_identifier = &mut proxied_type_identifier.clone();
        proxied_type_identifier.set_span(span);
        let is_raw = in_raw_function;

        // the proxied type is always proxied with the `Lua` prefix
        let mut proxy_ident_map = HashMap::from_iter(
            [(proxied_type_identifier.clone(),None)]
        );
        match fn_arg {
            FnArg::Receiver(receiver) => {

                arg_type = SimpleType::new_from_fn_arg(PROXY_PREFIX, fn_arg, proxied_type_identifier, &proxy_ident_map).unwrap_or_abort();
                // normally receivers are assumed to be wrapped, this let's us opt out of this behaviour
                if let Some(_mut) = receiver.mutability {
                    mutable = Some(_mut)
                } else {
                    mutable = None
                }

                arg_name = format_ident!("{}", SELF_ALIAS, span=span);
            }
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                let proxy_attr = attrs
                    .iter()
                    .find_map(|attr| attr.path.is_ident("proxy").then_some(attr));

                mutable = proxy_attr.is_some().then(|| parse_quote!(mut));

                let passed_proxy_type = proxy_attr
                    .map(|attr| attr.parse_meta().unwrap_or_else(|err| abort!(attr, err)))
                    .map(|meta| match meta {
                        // #[proxy] 
                        Meta::Path(_) => SimpleType::new_from_contextual_type(PROXY_PREFIX, ty, proxied_type_identifier, &proxy_ident_map).unwrap_or_abort(),
                        // #[proxy(TypeName1=ProxyType1, TypeName2=ProxyType2, ..)]  
                        Meta::List(MetaList { nested, .. }) => {
                            // collect all the types passed in the meta as identifiers
                            let idents = nested.iter().filter_map(|nested_meta| {
                                match nested_meta {
                                    NestedMeta::Meta(Meta::Path(path)) => 
                                        Some((path.get_ident().unwrap_or_else(|| abort!(path,"Expected identifier")).clone(),None)),
                                    NestedMeta::Meta(Meta::NameValue(MetaNameValue{path, lit: Lit::Str(lit_str), ..})) => 
                                        Some((path.get_ident().unwrap_or_else(|| abort!(path,"Expected identifier")).clone(), Some(lit_str.parse().unwrap_or_abort()))),
                                    _ => abort!(nested_meta.span(), "Expected proxy identifier mapping as in: `proxy(TypeName=ProxyType)` or `proxy(TypeName)` for `LuaTypeName`"),
                                }
                            }).collect::<Vec<(Ident,Option<Ident>)>>();
                            proxy_ident_map.extend(idents);
                            SimpleType::new_from_contextual_type(PROXY_PREFIX, ty, proxied_type_identifier, &proxy_ident_map).unwrap_or_abort()
                        },
                        other => abort!(other.span(), "Expected single item attribute list containing proxy name as in: `proxy(\"proxy_name\")`")
                    });

                let empty_idents = Default::default();
                arg_type = passed_proxy_type.unwrap_or_else(|| 
                    SimpleType::new_from_fully_specified_type(PROXY_PREFIX, ty, &empty_idents).unwrap_or_abort());


                arg_name = match pat.as_ref() {
                    Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                    Pat::Wild(_) => abort!(pat, "Cannot use `_` as identifier for proxy function"),
                    _ => abort!(pat, "Unsupported parameter pattern"),
                };

            }
        }

        FunctionArgMeta {
            mutable,
            arg_name,
            span,
            arg_type,
            is_raw
        }
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        proxied_type_identifier: &Ident,
        arg_name: Ident,
        arg_type: &Type,
        attrs: Vec<Attribute>,
        in_raw_function: bool,
    ) -> Self {
        let ty = Box::new(arg_type.clone());
        let pat_ty = PatType {
            attrs,
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
        Self::new_from_fn_arg(proxied_type_identifier, &fn_arg, Some(arg_type.span()), in_raw_function)
    }

    /// Unpacks non-reference proxy parameters (using the `inner` method) yielding expressions which correspond to the proxied type with conversion errors being
    /// handled by the try `?` operator.
    pub fn unpack_parameter(&self) -> Option<proc_macro2::TokenStream> {
        let name = &self.arg_name;
        if self.is_raw {
            // raw parameters DO NOT get unpacked, they get passed directly to the handling method as is
            None
        } else {
            // if a proxy parameter is to be passed by value we use inner (which requires Clone to be supported)
            Some(LuaSimpleTypeArgumentUnwrapper::new(name.clone(), name.span()).visit(&self.arg_type))
        }

    }


}

#[derive(Debug)]
struct FunctionMeta<'a> {
    name: &'a Ident,
    body: &'a TraitItemMethod,
    fn_type: FunctionType,
    arg_meta: Vec<FunctionArgMeta>,
    output_meta: FunctionArgMeta,
    is_raw: bool,
}

impl FunctionMeta<'_> {

    fn new<'a>(
        proxied_type_identifier: Ident,
        name: &'a Ident,
        body: &'a TraitItemMethod,
    ) -> FunctionMeta<'a> {
        // interpret and validate function meta to instantiate function proxies
        let meta = body
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("lua"))
            .unwrap_or_else(|| {
                abort!(
                    body,
                    "Lua proxy functions require `lua` meta attribute like so: `#[lua()]`"
                )
            })
            .parse_meta()
            .unwrap_or_else(|err| abort!(body, err));

        let mut output_attrs = Vec::default();

        match meta {
            Meta::List(MetaList { nested, .. }) => {
                let mut fn_type = FunctionType::Function;
                let mut is_raw = false;
                nested
                    .iter()
                    .for_each(|attr|{
                        match attr {
                            NestedMeta::Meta(Meta::Path(p)) if p.is_ident("raw") => {
                                is_raw = true;
                                return;
                            },
                            NestedMeta::Meta(Meta::Path(p)) => {
                                let attr_str = p.get_ident().map(Ident::to_string).unwrap_or_default();
                                if let Ok(_fn_type) = FunctionType::from_str(&attr_str) {
                                    fn_type = _fn_type;
                                    return;
                                } else {
                                    abort!(p, "Invalid Function Type, expected one of: {}", 
                                        FunctionType::iter().map(|ft| ft.to_string()).collect::<Vec<_>>().join(", "))
                                }
                            },
                            NestedMeta::Meta(Meta::List(list)) => {
                                    if list.path.is_ident("output") {
                                        for attr in list.nested.iter() {
                                            output_attrs.push(parse_quote!(#[#attr]))
                                        }
                                        return;
                                    }
                                },
                            _ => return
                        };
    
                        emit_error!(attr, "unknown or malformed lua proxy function attribute. Allowed attributes include: {}",
                            FunctionType::iter().map(|ft| ft.to_string()).collect::<Vec<_>>().join(", "));
                    });

                // if no output type is specified, it's set to the unit type `()`
                let output_type = match &body.sig.output {
                    ReturnType::Default => Type::Tuple(TypeTuple{ paren_token: Paren::default(), elems: Punctuated::default() }),
                    ReturnType::Type(_, t) => *t.to_owned(),
                };

                let fn_meta = FunctionMeta {
                    name,
                    body,
                    fn_type,
                    arg_meta: body
                        .sig
                        .inputs
                        .iter()
                        .map(|arg| FunctionArgMeta::new_from_fn_arg(&proxied_type_identifier, arg, None, is_raw))
                        .collect(),
                    output_meta: FunctionArgMeta::new_from_type(
                            &proxied_type_identifier,
                            format_ident!("{PROXIED_OUT_ALIAS}", span=body.sig.output.span()),
                            &output_type,
                            output_attrs,
                            is_raw
                        ),
                    is_raw
                };

                // validate the function against it's meta
                fn_meta.validate_function_definition(body);
    
                fn_meta
            }
            _ => abort!(
                body,
                "`lua` attribute must be a meta list of the form: `lua(elem1,elem2)`"
            ),
        }
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
    fn generate_mlua_args(&self) -> proc_macro2::TokenStream {
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
                let meta = args.next().expect_or_abort("Expected `Lua` type argument before non-receiver arguments and after the receiver if any.");
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

            quote_spanned!(self.body.sig.inputs.span()=>#ctxt_arg #self_arg , (#(#args),*) : (#(#arg_types),*))
        } else {
            quote_spanned!(self.body.sig.inputs.span()=>#ctxt_arg #self_arg)
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
    fn generate_mlua_body_unwrapped_parameter_assignments(&self) -> proc_macro2::TokenStream {        
        self.arg_meta
            .iter()
            .filter_map(|param| FunctionArgMeta::unpack_parameter(param).map(|unpacked_param| {
                let name = &param.arg_name;
                quote_spanned!{name.span()=>let #name = #unpacked_param;}
            }))
            .collect::<proc_macro2::TokenStream>()
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
    fn generate_mlua_body_proxied_output_stmt(&self, proxied_output_ident: &Ident, proxied_name: &Ident) -> proc_macro2::TokenStream {
    // generate function call on proxied type (operating over unwrapped parameters)
        // output will be stored in out_ident with the proxied_output_type
        // the type before we wrap it in a proxy
        let proxied_output_type = LuaTypeConstructorVisitor::new(false,false).visit(&self.output_meta.arg_type);

        let function_name = self.name;
        let param_names = self.arg_meta.iter()
            .map(|arg| &arg.arg_name).collect::<Vec<_>>();
        match (self.fn_type.expects_receiver(), &self.body.default){
            (_, Some(body)) => {
                
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor{
                        target: "self",
                        replacement: SELF_ALIAS,
                    }.visit_stmt_mut(&mut s);
                    s
                });

                quote_spanned!{body.span()=>
                    let #proxied_output_ident : #proxied_output_type = 
                        (||{
                            #(#stmts)*
                        })();
                }            
            },
            (true, None) => {
                // this removes the first argument taken to be the receiver from the iterator for the next step
                let (first_arg, other_args) = param_names.split_first().unwrap_or_else(|| abort!(self.name,"Proxied functions of the type: {} expect a receiver argument (i.e. self)", self.fn_type));
                // since we removed the receiver we can pass the rest of the parameters here;
                quote_spanned! {self.body.sig.paren_token.span=>
                    let #proxied_output_ident : #proxied_output_type = 
                        #first_arg.#function_name(#(#other_args),*);
                }
            },
            (false, None) => {
                // override this span, as otherwise spans propagate weird
                let mut proxied_name = proxied_name.clone();
                proxied_name.set_span(self.body.sig.ident.span());

                quote_spanned! {self.body.sig.paren_token.span=>
                    let #proxied_output_ident : #proxied_output_type = 
                        #proxied_name::#function_name(#(#param_names),*);
                }
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
    fn generate_mlua_body_proxy_output_stmt(&self, proxied_output_ident: &Ident, proxy_output_ident: &Ident) -> proc_macro2::TokenStream {
        if self.output_meta.is_raw {
            return quote_spanned! {self.body.span()=>
                let #proxy_output_ident = #proxied_output_ident;
            }
        }
        
        // generate `new` calls as required to build proxy stored in out_ident
        let constructor_wrapped_expression = 
            LuaSimpleTypeWrapper::new(proxied_output_ident.clone(), proxied_output_ident.span())
                .visit(&self.output_meta.arg_type);

        // the type of the wrapped type (if wrapped)
        let proxy_output_type = LuaTypeConstructorVisitor::new(true,false).visit(&self.output_meta.arg_type);
        
        // the statement assigning the proxy output to proxy_output_ident
        quote_spanned! {self.body.span()=>
            let #proxy_output_ident : #proxy_output_type = #constructor_wrapped_expression;
        }
    }

    fn generate_mlua_body(&self, proxied_name: &Ident) -> proc_macro2::TokenStream {
        let unpacked_parameter_declarations = self.generate_mlua_body_unwrapped_parameter_assignments();

        let proxied_output_ident = format_ident!("{PROXIED_OUT_ALIAS}", span=self.output_meta.span);
        let proxy_output_ident: Ident = format_ident!("{PROXY_OUT_ALIAS}", span=self.output_meta.span);

        let proxied_output_stmt = self.generate_mlua_body_proxied_output_stmt(&proxied_output_ident, proxied_name);
        let proxy_output_stmt = self.generate_mlua_body_proxy_output_stmt(&proxied_output_ident, &proxy_output_ident);

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

        quote!(   
            #unpacked_parameter_declarations
            #conversion_body_surrounded_with_dereferening_stms
        )
    }

    
    
    fn validate_function_definition(&self, definition: &TraitItemMethod) {
        if self.output_meta.arg_type.has_ref() && self.output_meta.arg_type.contains_proxy_type() {
            emit_error!(
                self.output_meta.span,
                format!(
                    "Lua proxy functions do not support non 'static types as return values yet"
                )
            )
        }

        if self.is_raw {
            let ctxt_arg_idx = if self.fn_type.expects_receiver() {1} else {0};

            let ctxt_arg = definition.sig.inputs.iter().nth(ctxt_arg_idx)
                .ok_or_else(|| syn::Error::new_spanned(&definition.sig.inputs, 
                        "Raw Lua proxy functions require a `&Lua` context argument following any receivers and before other arguments."))
                .unwrap_or_abort();

            let success = match ctxt_arg {
                FnArg::Typed(PatType{ ty, .. }) => matches!(ty.as_ref(), &Type::Reference(_)),
                _ => false
            };

            if !success {
                emit_error!(ctxt_arg, "Raw Lua proxy functions require a `&Lua` reference after any receivers and before other arguments.")
            }
        }

        if self.fn_type.expects_receiver() {
            if let Some(FnArg::Receiver(receiver)) = definition.sig.receiver() {
                // validate receiver
                if self.fn_type.expects_mutable_receiver() && receiver.mutability.is_none() {
                    emit_error!(
                        receiver,
                        format!(
                            "Lua proxy functions of type: {}, require `mut self` or `&mut self` argument",
                            self.fn_type
                        )
                    );
                };
            } else {
                emit_error!(
                    definition.sig.paren_token.span,
                    format!(
                        "Lua proxy functions of type: {}, require `self` argument",
                        self.fn_type
                    )
                )
            }
        } else if definition.sig.receiver().is_some() {
            emit_error!(
                definition.sig.receiver().unwrap(),
                format!(
                    "Lua proxy functions of type: {}, do not expect a receiver argument",
                    self.fn_type
                )
            )
        }
    }
}

#[derive(Display, EnumString, EnumIter, PartialEq, Eq, Clone, Copy, Debug)]
enum FunctionType {
    Function,
    MetaFunction,
    Method,
    MetaMethod,
    MutableFunction,
    MutableMetaFunction,
    MutatingMethod,
    MutatingMetaMethod,
    FieldGetterMethod,
    FieldSetterMethod,
}

impl FunctionType {
    fn expects_receiver(self) -> bool {
        self == FunctionType::Method
            || self == FunctionType::MetaMethod
            || self == FunctionType::MutatingMethod
            || self == FunctionType::MutatingMetaMethod
            || self == FunctionType::FieldGetterMethod
            || self == FunctionType::FieldSetterMethod
    }

    fn expects_mutable_receiver(self) -> bool {
        self == FunctionType::MutatingMethod 
            || self == FunctionType::FieldSetterMethod
    }

    fn is_field(self) -> bool {
        self == FunctionType::FieldGetterMethod ||
            self == FunctionType::FieldSetterMethod
    }

    fn expects_arguments_other_than_self(self) -> bool {
        self != FunctionType::FieldGetterMethod 
    }

    fn get_tealr_function(self) -> &'static str {
        match self {
            FunctionType::Function => "add_function",
            FunctionType::MetaFunction => "add_meta_function",
            FunctionType::Method => "add_method",
            FunctionType::MetaMethod => "add_meta_method",
            FunctionType::MutableFunction => "add_function_mut",
            FunctionType::MutableMetaFunction => "add_meta_function_mut",
            FunctionType::MutatingMethod => "add_method_mut",
            FunctionType::MutatingMetaMethod => "add_meta_method_mut",
            FunctionType::FieldGetterMethod => "add_field_method_get",
            FunctionType::FieldSetterMethod => "add_field_method_set",
        }
    }

    fn is_meta(self) -> bool {
        self == FunctionType::MetaMethod
            || self == FunctionType::MetaFunction
            || self == FunctionType::MutatingMetaMethod
            || self == FunctionType::MutableMetaFunction
    }

}

#[proc_macro_error]
#[proc_macro]
pub fn impl_lua_proxy(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);

    let meta: ProxyMeta = match derive_input.try_into() {
        Ok(meta) => meta,
        Err(err) => return err.to_compile_error().into(),
    };

    if meta.proxy_name.is_some() {
        // throw error
        abort!(
            meta.span,
            "The `proxy_name` attribute is not supported for lua proxies"
        );
    }

    let proxied_name = meta.proxied_name;

    let proxy_type = Type::Path(TypePath{ qself: None, path: format_ident!("{PROXY_PREFIX}{}",proxied_name).into() });

    let is_clonable = meta.proxy_flags.flags.contains(&DeriveFlag::Clone);

    // generate the type definition of the proxy
    let mut definition: proc_macro2::TokenStream;

    if is_clonable {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_type with Clone);
        };
    } else {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_type);
        }
    }

    if meta.proxy_flags.flags.contains(&DeriveFlag::Debug) {
        definition.extend(quote_spanned!{meta.span=>
            impl std::fmt::Debug for #proxy_type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))}    
            }
        });
    }

    let tealr_type_implementations = quote_spanned! {meta.span=>
        bevy_script_api::impl_tealr_type!(#proxy_type);
    };

    // generate type level tealr documentation calls
    let type_level_document_calls = meta
        .docstrings
        .iter()
        .map(|tkns| quote_spanned!(meta.span=>methods.document_type(#tkns);));
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    let field_methods : Vec<(Ident, TraitItemMethod)> = match meta.data {
        ProxyData::Struct { fields } => {
            let field_processor = |idx: usize, field: &Field, is_setter: bool| {          
                let field_name = field.ident.clone().unwrap_or_else(|| format_ident!("_{}", idx));
                let field_type = &field.ty;
                let docs = field.attrs.iter()
                    .filter(|attr| attr.path.is_ident("doc"))
                    .collect::<Vec<_>>();

                let mut lua_attr = field.attrs.iter()
                    .find(|attr| attr.path.is_ident("proxy"))
                    .map(|attr| attr.parse_meta().unwrap_or_abort())
                    .and_then(|meta| match meta {   
                        Meta::List(MetaList { nested, .. }) => nested.iter().find_map(|attr| match attr {
                                NestedMeta::Meta(l@Meta::List(MetaList { nested , .. })) 
                                    if l.path().is_ident("lua") => Some(nested.clone()),
                                _ => None
                            } ),
                        _ => None
                    })
                    .unwrap_or_else(Punctuated::<NestedMeta,Token![,]>::new);


                if is_setter {
                    lua_attr.extend::<Punctuated<NestedMeta, Token![,]>>(parse_quote!(lua(FieldSetterMethod)));
                } else {
                    lua_attr.extend::<Punctuated<NestedMeta, Token![,]>>(parse_quote!(lua(FieldGetterMethod)));
                }
                lua_attr.extend::<Punctuated<NestedMeta, Token![,]>>(parse_quote!(raw));


                let lua_attr = Attribute {
                    pound_token: Token![#](fields.span()), 
                    style: AttrStyle::Outer, 
                    bracket_token: Bracket::default(), 
                    path: Path{ leading_colon: None, segments: Default::default() }, 
                    tokens: lua_attr.to_token_stream() 
                };
    
                let trait_item_method : TraitItemMethod = if is_setter {
                    parse_quote!{
                        #lua_attr
                        #(#docs)*
                        fn #field_name (&mut self, lua: &Lua, other: #field_type) {
                            let world_ptr = <bevy_mod_scripting_lua::tealr::mlu::mlua::Lua as bevy_script_api::common::bevy::GetWorld>::get_world(lua)?;

                            self.#field_name;
                            ()
                        }
                    }
                } else {
                    parse_quote!{
                        #lua_attr
                        #(#docs)*
                        fn #field_name (&self, lua: &Lua) -> #field_type {
                            self.#field_name
                        }
                    }
                };

                (field_name, trait_item_method)
            };

            let mut out = fields.iter().enumerate().map(|(idx,field)| 
                field_processor(idx,field,false)).collect::<Vec<_>>();
            
            out.extend(fields.iter().enumerate().map(|(idx,field)| 
                field_processor(idx,field,true)).collect::<Vec<_>>());
            out
        },
    };


    // generate both tealr documentation and instantiations of functions and field getters/setters
    let (fields, methods) = meta.functions.iter()
        // treat field getters and setters as normal methods for the time being, separate later
        .chain(field_methods.iter().map(|(a,b)| (a,b)))
        .map(|(name, body)| {
        let method_documentation_calls = body
            .attrs
            .iter()
            .map(attribute_to_string_lit)
            .filter(|s| !s.is_empty())
            .map(|tkns| quote_spanned!(body.span()=>methods.document_type(#tkns);));


        let fn_meta = FunctionMeta::new(proxied_name.clone(), name, body);
        let args = fn_meta.generate_mlua_args();
        let body: proc_macro2::TokenStream = fn_meta.generate_mlua_body(&proxied_name);
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

        (fn_meta.fn_type.is_field(), 
            quote_spanned! {body.span()=>
                #(#method_documentation_calls)*
                #container_ident.#tealr_function(#signature, #closure);
            }
        )
    }).partition::<Vec<(bool, proc_macro2::TokenStream)>,_>(|(is_field,_)| *is_field);

    let fields = fields.iter().map(|(_, b)| b);
    let methods = methods.iter().map(|(_, b)| b);
    let a = quote_spanned! {meta.span=>

        #definition

        #tealr_type_implementations

        #[automatically_derived]
        #[allow(unused_parens, unused_braces, unused_mut, unused_variables)]
        #[allow(clippy::all)]
        impl #tealr::mlu::TealData for #proxy_type {
            fn add_methods<'lua, T: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                #(#type_level_document_calls)*
                #(#methods)*
            }

            fn add_fields<'lua, T: #tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
                #(#fields)*
            }
        }

        #[allow(clippy::all, unused_variables)]
        impl bevy_script_api::lua::LuaProxyable for #proxied_name {
            fn ref_to_lua<'lua>(self_ : bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>> {
                <#proxy_type as #tealr::mlu::mlua::ToLua>::to_lua(#proxy_type::new_ref(self_),lua)
            }

            fn apply_lua<'lua>(self_ : &mut bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr::mlu::mlua::Lua, new_val: #tealr::mlu::mlua::Value<'lua>) -> #tealr::mlu::mlua::Result<()> {
                if let #tealr::mlu::mlua::Value::UserData(v) = new_val {
                    let other = v.borrow::<#proxy_type>()?;
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
        impl bevy_script_api::lua::ToLuaProxy<'_> for #proxied_name {
            fn to_lua_proxy<'lua>(self, lua: &'lua #tealr::mlu::mlua::Lua) -> #tealr::mlu::mlua::Result<#tealr::mlu::mlua::Value<'lua>>{
                <#proxy_type as #tealr::mlu::mlua::ToLua>::to_lua(#proxy_type::new(self),lua)
            }
        }

    }
    .into();
    // panic!("{}", a);
    a
}