use bevy_mod_scripting_common::input::{
    DuoPath, IdentifierRenamingVisitor, Reference, SimpleType, VisitSimpleType,
};
use darling::{util::Flag, FromAttributes, FromMeta};
use proc_macro2::{Ident, Span};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use strum::{Display, EnumIter, EnumString};
use syn::{
    punctuated::Punctuated, spanned::Spanned, visit_mut::VisitMut, Block, LitInt, LitStr, Meta,
    Path, Token,
};
use vec1::Vec1;

use crate::{
    arg::Arg,
    signature::Signature,
    visitor::{LuaSimpleTypeWrapper, LuaTypeConstructorVisitor},
    PROXY_OUT_ALIAS, RAW_OUT_ALIAS, SELF_ALIAS,
};

#[derive(Default, FromMeta, Display, EnumString, EnumIter, PartialEq, Eq, Clone, Copy, Debug)]
#[darling(rename_all = "PascalCase")]
pub enum FunctionKind {
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
    pub fn expects_receiver(self) -> bool {
        self == FunctionKind::Method
            || self == FunctionKind::MetaMethod
            || self == FunctionKind::MutatingMethod
            || self == FunctionKind::MutatingMetaMethod
            || self == FunctionKind::FieldGetterMethod
            || self == FunctionKind::FieldSetterMethod
    }

    pub fn is_field(self) -> bool {
        self == FunctionKind::FieldGetterMethod || self == FunctionKind::FieldSetterMethod
    }

    pub fn is_field_getter(self) -> bool {
        self == FunctionKind::FieldGetterMethod
    }

    /// Returns true if the mlua closure signature accepts a tuple for general 'Arguments' to the function
    /// I.e. arguments freely passed to the function by the caller.
    pub fn expects_arguments_tuple(self) -> bool {
        self != FunctionKind::FieldGetterMethod
    }

    pub fn get_tealr_function(self) -> &'static str {
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

    pub fn is_meta(self) -> bool {
        self == FunctionKind::MetaMethod
            || self == FunctionKind::MetaFunction
            || self == FunctionKind::MutatingMetaMethod
            || self == FunctionKind::MutableMetaFunction
    }
}

/// The attributes which can be applied to lua functions using the
/// `lua(..)` meta attribute
#[derive(Debug, FromAttributes)]
#[darling(attributes(lua))]
pub struct FunctionAttributes {
    /// Marks the function to be treated as raw meaning a lot of the wrapping and unwrapping is skipped,
    /// a 'Lua' ctxt argument is then expected
    pub raw: Flag,

    /// Marks the function as a composite with the given ID, at least one another function with the same composite
    /// ID must exist resulting in a combined function being generated. The actual function to dispatch to will be decided based on
    /// the types of arguments. If the signature is invalid (i.e. doesn't allow us to dispatch) an error will be thrown
    #[darling(default)]
    pub composite: Option<String>,

    /// If passed provides the name of the metamethod to use for metamethod based functions
    /// the name of the function is used to decide what rust function to call in this case
    #[darling(default)]
    pub metamethod: Option<Ident>,

    /// The kind of function to generate on the proxy
    #[darling(default)]
    pub kind: FunctionKind,

    /// Marks this to be ignored, only used for fields as functions are opt-in
    pub skip: Flag,

    /// Meta to pass down to the output proxy or in case of fields
    /// used as the argument meta for type being get/set
    pub output: Option<Meta>,

    /// If passed will generate <T as Trait> statement before calling the method
    /// on the type
    pub as_trait: Option<Path>,

    #[darling(multiple)]
    pub doc: Vec<String>,
}

/// A function which combines the signatures of multiple functions,
/// and dispatches to the one which matches the signature if any
/// Useful for binary operators which can accept many types on both sides
#[derive(Debug)]
pub struct CompositeFunction {
    pub id: String,
    pub functions: Vec1<Function>,
}

/// A struct corresponding to each function in the functions[...] meta list.
///
#[derive(Debug)]
pub struct Function {
    pub name: Ident,
    pub attrs: FunctionAttributes,
    pub sig: Signature,
    pub default: Option<Block>,
    pub span: Span,
    pub is_unsafe: bool,
}

impl Function {
    pub fn new(
        name: Ident,
        attrs: FunctionAttributes,
        default: Option<Block>,
        sig: Signature,
        span: Span,
        is_unsafe: bool,
    ) -> darling::Result<Self> {
        Ok(Self {
            name,
            attrs,
            sig,
            default,
            span,
            is_unsafe,
        })
    }

    /// Tries to retrieve the receiver argument from functions.
    /// If not expected returns None and Some otherwise.
    /// If the function is of the wrong kind or does not have the correct signature an error is thrown
    pub fn self_arg(&self) -> syn::Result<Option<&Arg>> {
        if self.attrs.kind.expects_receiver() {
            self.get_self_arg().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    /// Returns an error if self arg is not there and returns it otherwise
    pub fn get_self_arg(&self) -> syn::Result<&Arg> {
        self.sig.inputs.first().ok_or_else(|| {
            syn::Error::new(
                self.sig.span,
                "Expected receiver as first argument in the signature".to_string(),
            )
        })
    }

    /// Tries to retrieve the context argument from raw functions.
    /// If the function is not raw or doesn't have a correct signature an error is thrown
    pub fn ctxt_arg(&self) -> syn::Result<Option<&Arg>> {
        if self.attrs.raw.is_present() {
            self.get_ctxt_arg().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    /// Returns an error if no context argument is found in the correct place or returns it otherwise
    pub fn get_ctxt_arg(&self) -> syn::Result<&Arg> {
        let ctxt_idx = if self.attrs.kind.expects_receiver() {
            1
        } else {
            0
        };
        self.sig.inputs.get(ctxt_idx).ok_or_else(|| {
            syn::Error::new(
                self.sig.span,
                format!(
                    "Expected ctxt argument in the signature as argument number: `{}`",
                    ctxt_idx + 1
                ),
            )
        })
    }

    /// Retrieves the rest of the arguments (after the receiver and context args)
    /// If they are expected, otherwise returns None.
    /// If arguments are expected but none are present Some(vec![]) is returned
    /// If input vec is shorter than expected, i.e. if the receiver should be there but isn't returns an Err
    pub fn other_arguments(&self) -> syn::Result<Option<impl Iterator<Item = &Arg>>> {
        if self.attrs.kind.expects_arguments_tuple() {
            self.get_other_arguments().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    pub fn get_other_arguments(&self) -> syn::Result<impl Iterator<Item = &Arg>> {
        let other_args_idx =
            self.attrs.kind.expects_receiver() as usize + self.attrs.raw.is_present() as usize;

        if self.sig.inputs.len() < other_args_idx {
            return Err(syn::Error::new(
                self.sig.span,
                format!("Signature too short, expected {other_args_idx} arguments before this one"),
            ));
        }

        Ok(self.sig.inputs.iter().skip(other_args_idx))
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
    pub fn generate_mlua_args(&self) -> syn::Result<proc_macro2::TokenStream> {
        let self_arg = self.self_arg()?.map(Arg::arg_signature_receiver);

        let ctxt_arg = self
            .ctxt_arg()?
            .map(Arg::arg_signature_context)
            .unwrap_or_else(|| quote!(_));

        let other_args = self.other_arguments()?.map(|args| {
            let (other_arg_names, other_arg_types) =
                args.map(Arg::arg_signature).unzip::<_, _, Vec<_>, Vec<_>>();

            quote_spanned!(self.sig.span=>
                (#(mut #other_arg_names),*) : (#(#other_arg_types),*)
            )
        });

        Ok(vec![Some(ctxt_arg), self_arg, other_args]
            .into_iter()
            .filter(Option::is_some)
            .collect::<Punctuated<Option<proc_macro2::TokenStream>, Token![,]>>()
            .to_token_stream())
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
    fn generate_mlua_body_unwrapped_parameter_assignments(
        &self,
    ) -> syn::Result<proc_macro2::TokenStream> {
        Ok(self
            .sig
            .inputs
            .iter()
            .map(Arg::unpack_parameter)
            .collect::<syn::Result<Vec<_>>>()?
            .iter()
            .zip(self.sig.inputs.iter())
            .filter_map(|(unpacked_param, arg)| {
                unpacked_param.as_ref().map(|unpacked_param| {
                    let name = &arg.name;
                    quote_spanned! {name.span()=>let mut #name = #unpacked_param;}
                })
            })
            .collect::<proc_macro2::TokenStream>())
    }

    /// Similar to generate_mlua_body_output but for functions, makes some more assumptions and directly generates wrapped/unwrapped output depending on what's necessary
    /// Does not require another wrapping step and can be directly put in a result as the final output of an mlua closure
    fn generate_mlua_body_output_field(
        &self,
        raw_output_ident: &Ident,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let field_type = if self.attrs.kind.is_field_getter() {
            &self.sig.output
        } else {
            self.get_other_arguments()?.next()
                .ok_or_else(|| syn::Error::new(self.sig.span, format!("Setter lua method with no arguments, expected at least one argument in function: `{}`", self.name)))?
        };
        // we need to figure out which type of field access this is going to be
        let self_name = &self.get_self_arg()?.name;
        let ctxt_name = &self.get_ctxt_arg()?.name;
        let world_ptr = quote!(
            <bevy_mod_scripting_lua::tealr::mlu::mlua::Lua
                as bevy_script_api::common::bevy::GetWorld>
                ::get_world(#ctxt_name)?
        );

        let field_name = {
            let str_name = self.name.to_string();

            if str_name.starts_with('_') && str_name[1..].parse::<usize>().is_ok() {
                syn::Lit::Int(LitInt::new(&str_name[1..], self.name.span())).to_token_stream()
            } else {
                self.name.clone().to_token_stream()
            }
        };
        let field_name_str = syn::Lit::Str(LitStr::new(&self.name.to_string(), self.name.span()));
        let proxy_output_type =
            LuaTypeConstructorVisitor::new(true, false).visit(&field_type.type_);

        Ok(match &field_type.type_ {
            // proxy, need to index into it then wrap the result
            // getter
            t if t.contains_proxy_type() && self.attrs.kind.is_field_getter() => quote!(
                let #raw_output_ident = #proxy_output_type::new_ref(bevy_script_api::script_ref::ValueIndex::index(& #self_name.reflect_ref(#world_ptr), std::borrow::Cow::Borrowed(#field_name_str)));
            ),
            // setter
            t if t.contains_proxy_type() => {
                let first_arg_name = &self
                    .get_other_arguments()?
                    .next()
                    .ok_or_else(|| {
                        syn::Error::new(
                            self.sig.span,
                            "Field setter requires a single argument which is missing",
                        )
                    })?
                    .name;
                quote!(
                    let #raw_output_ident = #first_arg_name.apply_self_to_base(&mut bevy_script_api::script_ref::ValueIndex::index(& #self_name.reflect_ref(#world_ptr), std::borrow::Cow::Borrowed(#field_name_str)))?;
                )
            }

            // plain reflection, index into the ReflectReference with the field path
            // getter
            SimpleType::Type(syn::Type::Path(path))
                if path.path.is_ident("ReflectedValue") && self.attrs.kind.is_field_getter() =>
            {
                quote!(
                    let #raw_output_ident = bevy_script_api::script_ref::ValueIndex::index(& #self_name.reflect_ref(#world_ptr), std::borrow::Cow::Borrowed(#field_name_str));
                )
            }
            // setter
            SimpleType::Type(syn::Type::Path(path)) if path.path.is_ident("ReflectedValue") => {
                let first_arg_name = &self
                    .get_other_arguments()?
                    .next()
                    .ok_or_else(|| {
                        syn::Error::new(
                            self.sig.span,
                            "Field setter requires a single argument which is missing",
                        )
                    })?
                    .name;
                quote!(
                    let #raw_output_ident =  bevy_script_api::script_ref::ValueIndex::index(& #self_name.reflect_ref(#world_ptr), std::borrow::Cow::Borrowed(#field_name_str)).apply(&#first_arg_name.ref_)?;
                )
            }

            // primitive use clone on the value and return it without a wrapper
            // getter
            _ if self.attrs.kind.is_field_getter() => quote!(
                let #raw_output_ident = #self_name.val(|#self_name| #self_name.#field_name.clone())?;
            ),
            // setter
            _ => {
                let first_arg_name = &self
                    .get_other_arguments()?
                    .next()
                    .ok_or_else(|| {
                        syn::Error::new(
                            self.sig.span,
                            "Field setter requires a single argument which is missing",
                        )
                    })?
                    .name;
                quote!(
                    let #raw_output_ident = #self_name.val_mut(|#self_name| #self_name.#field_name = #first_arg_name)?;
                )
            }
        })
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
    fn generate_mlua_body_raw_output(
        &self,
        raw_output_ident: &Ident,
        proxied_type_path: &Path,
    ) -> syn::Result<proc_macro2::TokenStream> {
        // generate function call on proxied type (operating over unwrapped parameters)
        // output will be stored in raw_output_ident with the proxied_type_path

        // the type before we wrap it in a proxy
        let raw_output_type =
            LuaTypeConstructorVisitor::new(false, false).visit(&self.sig.output.type_);

        match &self.default {
            Some(body) => {
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor {
                        target: "self",
                        replacement: SELF_ALIAS,
                    }
                    .visit_stmt_mut(&mut s);
                    s
                });

                Ok(quote_spanned! {body.span()=>
                    let #raw_output_ident : #raw_output_type =
                        (||{
                            #(#stmts)*
                        })();
                })
            }
            None => {
                let function_name = &self.name;
                let param_names = self
                    .sig
                    .inputs
                    .iter()
                    .map(|arg| &arg.name)
                    .collect::<Vec<_>>();

                // override this span, as otherwise spans propagate weird
                let mut proxied_name = proxied_type_path.clone();

                proxied_name
                    .segments
                    .iter_mut()
                    .for_each(|v| v.ident.set_span(self.sig.span));

                let method_path = if let Some(trait_path) = &self.attrs.as_trait {
                    quote_spanned!(self.sig.span=>
                        #trait_path::#function_name
                    )
                } else {
                    quote_spanned!(self.sig.span=>
                        #proxied_name::#function_name
                    )
                };

                Ok(quote_spanned! {self.sig.span=>
                    let #raw_output_ident : #raw_output_type =
                        #method_path(#(#param_names),*);
                })
            }
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
    fn generate_mlua_body_proxy_output(
        &self,
        proxied_output_ident: &Ident,
        proxy_output_ident: &Ident,
    ) -> syn::Result<proc_macro2::TokenStream> {
        if self.sig.output.is_raw {
            return Ok(quote_spanned! {self.sig.span=>
                let #proxy_output_ident = #proxied_output_ident;
            });
        }

        // generate `new` calls as required to build proxy stored in out_ident
        let constructor_wrapped_expression =
            LuaSimpleTypeWrapper::new(proxied_output_ident.clone(), proxied_output_ident.span())
                .visit(&self.sig.output.type_)?;

        // the type of the wrapped type (if wrapped)
        let proxy_output_type =
            LuaTypeConstructorVisitor::new(true, false).visit(&self.sig.output.type_);

        // the statement assigning the proxy output to proxy_output_ident
        Ok(quote_spanned! {self.sig.span=>
            let #proxy_output_ident : #proxy_output_type = #constructor_wrapped_expression;
        })
    }

    pub fn generate_mlua_body(
        &self,
        proxied_type_path: &Path,
    ) -> syn::Result<proc_macro2::TokenStream> {
        let unpacked_parameter_declarations =
            self.generate_mlua_body_unwrapped_parameter_assignments()?;

        let raw_output_ident = format_ident!("{RAW_OUT_ALIAS}", span = self.sig.span);
        let proxy_output_ident = format_ident!("{PROXY_OUT_ALIAS}", span = self.sig.span);

        let raw_output_stmt = if self.attrs.kind.is_field() {
            self.generate_mlua_body_output_field(&raw_output_ident)?
        } else {
            self.generate_mlua_body_raw_output(&raw_output_ident, proxied_type_path)?
        };

        // for fields the output is expected to be raw anyway so this will just performa no-op
        let proxy_output_stmt =
            self.generate_mlua_body_proxy_output(&raw_output_ident, &proxy_output_ident)?;

        // determine if we need to wrap the output in an Ok() statement
        let last_stm = match &self.sig.output.type_ {
            SimpleType::DuoPath(DuoPath { ident, .. }) if *ident == "Result" => {
                quote_spanned! {self.sig.span=>#proxy_output_ident}
            }
            _ => quote_spanned! {self.sig.span=>Ok(#proxy_output_ident)},
        };

        let conversion_body_stms = quote!(
            #raw_output_stmt
            #proxy_output_stmt
            #last_stm
        );

        // for every argument which is a reference, we need a separate sort of call,
        // we cannot use `v.inner()` since this operates over values, we must use `val_mut` or `val` to get a reference to the wrapped
        // structure for the duration of the call
        let conversion_body_surrounded_with_dereferening_stms =
            self.sig
                .inputs
                .iter()
                .fold(conversion_body_stms, |acc, arg_meta| {
                    // only proxy types which are directly inside a reference are supported as references
                    if !matches!(arg_meta.type_, SimpleType::Reference(Reference{ ref inner, .. })
                        if matches!(inner.as_ref(), SimpleType::ProxyType(_)))
                    {
                        return acc;
                    }
                    // raw arguments are passed directly to the handler function
                    if arg_meta.is_raw {
                        return acc;
                    }

                    let method_call = if arg_meta.type_.has_outer_mut_ref() {
                        format_ident!("val_mut", span = arg_meta.span)
                    } else {
                        format_ident!("val", span = arg_meta.span)
                    };

                    let arg_name = &arg_meta.name;

                    quote_spanned! {self.sig.span=>{
                        #arg_name.#method_call(|mut #arg_name| {#acc})?
                    }}
                });
        let out = quote!(
            #unpacked_parameter_declarations
            #conversion_body_surrounded_with_dereferening_stms
        );

        if self.is_unsafe {
            Ok(quote_spanned! {self.sig.span=>
                unsafe {
                    #out
                }
            })
        } else {
            Ok(out)
        }
    }
}
