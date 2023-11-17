use std::{collections::HashMap, str::FromStr, iter::once};

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


/// Describes the functional signature of a function from the `functions[..]` list
#[derive(Debug)]
struct Signature {
    inputs: Vec<Arg>,
    output: Arg,
    span: Span,
}

impl Signature {
    /// Creates a new signature struct
    /// if in_raw_function will set is_raw on all arguments and outputs to true
    /// if is_field_setter, output_attrs will be applied to the third argument of the function if it exists (the first non self or ctxt arg)
    fn new(proxied_type_path: Path, sig: syn::Signature, in_raw_function: bool, output_attrs: Vec<Attribute>) -> darling::Result<Self> {
        // convert output to FnArg
        let output_arg_name = Ident::new(PROXIED_OUT_ALIAS, sig.output.span());
        let span = sig.span();
        // if no return type specified use `()`
        let output_type = match sig.output {
            ReturnType::Default => Type::Tuple(TypeTuple { paren_token: Default::default(), elems: Default::default() }),
            ReturnType::Type(_, ty) => *ty,
        };



        let mut inputs = sig.inputs.into_iter()
            .map(|arg| Self::convert_fn_arg(arg, &proxied_type_path, in_raw_function))
            .collect::<darling::Result<Vec<_>>>()?;

        // convert to Arg structs
        let output = Self::convert_type(
            output_type, 
            &proxied_type_path, 
            in_raw_function, 
            output_attrs, 
            output_arg_name, 
            None)?;

        Ok(Self {
            inputs,
            output,
            span
        })
    }

    /// Convert a function argument into custom Arg struct by converting the type to SimpleType and parsing attributes
    fn convert_fn_arg(arg: FnArg, proxied_type_path: &Path, in_raw_function: bool) -> darling::Result<Arg> {
        let type_map = HashMap::from_iter(
            [(proxied_type_path.segments.last().unwrap().clone().ident,None)]
        );

        Ok(match arg{
            FnArg::Receiver(ref receiver) => {
                let type_ = SimpleType::new_from_fn_arg(PROXY_PREFIX, &arg, proxied_type_path, &type_map)?;
                let attrs = ArgAttributes::from_attributes(&receiver.attrs)?;
                Arg::new(attrs, Ident::new(SELF_ALIAS, receiver.span()), receiver.mutability, type_, in_raw_function)
            },
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                let (mutability, arg_name) = match pat.as_ref() {
                    Pat::Ident(PatIdent { mutability, ident, ..}) => (mutability, ident),
                    _ => return Err(darling::Error::custom("Unsupported parameter pattern")),
                };

                Self::convert_type(*ty, proxied_type_path, in_raw_function, attrs, arg_name.clone(), *mutability)?
            },
        })
    }

    /// Convert a type corresponding to an argument into an Arg struct by converting it to a Simple type and parsing the given attributes
    fn convert_type(ty: Type, proxied_type_path: &Path, in_raw_function: bool, attrs: Vec<Attribute>, arg_name: Ident, mutability: Option<Mut>) -> darling::Result<Arg> {
        let mut type_map = HashMap::from_iter(
            [(proxied_type_path.segments.last().unwrap().clone().ident,None)]
        );
        let is_proxy = attrs.iter().any(|a| a.path().is_ident("proxy"));
        let attrs = ArgAttributes::from_attributes(&attrs)?;
        let type_ = if is_proxy && attrs.map.is_empty() {
            SimpleType::new_from_contextual_type_proxy_all(PROXY_PREFIX, &ty, proxied_type_path)?
        } else {
            type_map.extend(attrs.map.iter().map(|(a,b)| (a.clone(),Some(b.clone())))); 
            SimpleType::new_from_contextual_type(PROXY_PREFIX, &ty, proxied_type_path, &type_map)?
        };

    
        Ok(Arg::new(attrs, arg_name, mutability, type_, in_raw_function))
    }
    
}

#[derive(Debug, FromAttributes)]
#[darling(attributes(proxy))]
struct ArgAttributes {
    #[darling(default)]
    map: HashMap<Ident, Ident>
}



/// Struct for holding argument/output information for functions passed via `functions[..]` meta
#[derive(Debug)]
struct Arg {
    attrs: ArgAttributes,
    mutability: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    name: Ident,
    /// variant specific data enumeration
    type_: SimpleType,
    /// if an argument is raw, it's passed without any unwrapping to the handler function
    /// if an argument isn't annotated with the `proxy` flag it is technically raw, but this is different for receiver and output arguments
    is_raw: bool,
    span: Span,
}

impl Arg {

    fn new(attrs: ArgAttributes, name: Ident, mutability: Option<Mut>, type_: SimpleType, is_raw: bool) -> Self{
        Self{
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
            Ok(Some(LuaSimpleTypeArgumentUnwrapper::new(name.clone(), name.span()).visit(&self.type_)?))
        }
    }

    fn arg_signature_generic(&self, expecting_receiver: bool, expecting_ctxt: bool) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
        assert!(!(expecting_receiver && expecting_ctxt));


        let _mut = &self.mutability;
        let name = &self.name;
        let type_ = if expecting_ctxt {
            parse_quote!(&bevy_mod_scripting_lua::prelude::Lua)
        } else {
            LuaTypeConstructorVisitor::new(true,self.type_.contains_proxy_type()).visit(&self.type_)
        };
        let forced_ref = expecting_receiver
            .then(|| Some(quote_spanned!(self.span=>
                & #_mut
            )));

        let name_part = quote_spanned!(self.span=>
            #_mut #name
        );
        let type_part = quote_spanned!(self.span=>
            #forced_ref #type_
        );
        (name_part,type_part)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a receiver type argument.
    /// generates using an additional outer reference.
    pub fn arg_signature_receiver(&self) -> proc_macro2::TokenStream {
        let (name,type_) = self.arg_signature_generic(true, false);
        quote!(#name : #type_)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a Lua context type argument.
    /// generates using an additional outer reference.
    pub fn arg_signature_context(&self) -> proc_macro2::TokenStream {
        let (name,type_) = self.arg_signature_generic(false, true);
        quote!(#name : #type_)
    }

    /// Generates the arg signature in an mlua `UserDataFields` or `UserDataMethods` closure for a non-receiver non-context argument.
    /// generates the type to match the argument received. 
    /// The output is split into the name and type parts
    pub fn arg_signature(&self) -> (proc_macro2::TokenStream, proc_macro2::TokenStream){
        self.arg_signature_generic(false,false)
    }
}


/// The attributes which can be applied to lua functions using the 
/// `lua(..)` meta attribute
#[derive(Debug, FromAttributes)]
#[darling(attributes(lua))]
struct FunctionAttributes {
    /// Marks the function to be treated as raw meaning a lot of the wrapping and unwrapping is skipped, 
    /// a 'Lua' ctxt argument is then expected
    raw: Flag,

    /// The kind of function to generate on the proxy
    #[darling(default)]
    kind: FunctionKind,

    /// Marks this to be ignored, only used for fields as functions are opt-in
    skip: Flag,
    
    /// Meta to pass down to the output proxy or in case of fields
    /// used as the argument meta for type being get/set
    output: Option<Meta>,

    /// If passed will generate <T as Trait> statement before calling the method
    /// on the type
    as_trait: Option<Path>,

    #[darling(multiple)]
    doc: Vec<String>,
}


/// A struct corresponding to each function in the functions[...] meta list.
/// 
#[derive(Debug)]
struct Function {
    name: Ident,
    attrs: FunctionAttributes,
    sig: Signature,
    default: Option<Block>
}

impl Function {

    fn new(name: Ident, 
        attrs: FunctionAttributes, 
        default: Option<Block>,
        sig : Signature) -> darling::Result<Self> {
        Ok(Self {
            name,
            attrs,
            sig,
            default,
        })
    }


    /// Tries to retrieve the receiver argument from functions.
    /// If not expected returns None and Some otherwise.
    /// If the function is of the wrong kind or does not have the correct signature an error is thrown
    fn self_arg(&self) -> syn::Result<Option<&Arg>> {
        if self.attrs.kind.expects_receiver() {
            self.get_self_arg().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    /// Returns an error if self arg is not there and returns it otherwise
    fn get_self_arg(&self) -> syn::Result<&Arg> {
        self.sig.inputs
            .first()
            .ok_or_else(|| 
                syn::Error::new(self.sig.span, 
                    format!("Expected receiver as first argument in the signature")))
    }

    /// Tries to retrieve the context argument from raw functions.
    /// If the function is not raw or doesn't have a correct signature an error is thrown
    fn ctxt_arg(&self) -> syn::Result<Option<&Arg>> {

        if self.attrs.raw.is_present() {
            self.get_ctxt_arg().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    /// Returns an error if no context argument is found in the correct place or returns it otherwise
    fn get_ctxt_arg(&self) -> syn::Result<&Arg> {
        let ctxt_idx = if self.attrs.kind.expects_receiver() {1} else {0};
        self.sig.inputs
            .get(ctxt_idx)
            .ok_or_else(|| syn::Error::new(self.sig.span, format!("Expected ctxt argument in the signature as argument number: `{}`", ctxt_idx + 1)))
    }

    /// Retrieves the rest of the arguments (after the receiver and context args)
    /// If they are expected, otherwise returns None.
    /// If arguments are expected but none are present Some(vec![]) is returned
    /// If input vec is shorter than expected, i.e. if the receiver should be there but isn't returns an Err
    fn other_arguments(&self) -> syn::Result<Option<impl Iterator<Item=&Arg>>> {
        if self.attrs.kind.expects_arguments_tuple() {
            self.get_other_arguments().map(Option::Some)
        } else {
            Ok(None)
        }
    }

    fn get_other_arguments(&self) -> syn::Result<impl Iterator<Item=&Arg>> {
        let other_args_idx = self.attrs.kind.expects_receiver() as usize 
        + self.attrs.raw.is_present() as usize;

        if self.sig.inputs.len() < other_args_idx {
            return Err(syn::Error::new(self.sig.span, format!("Signature too short, expected {other_args_idx} arguments before this one")))
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
    fn generate_mlua_args(&self) -> syn::Result<proc_macro2::TokenStream> {
        let self_arg = self.self_arg()?.map(Arg::arg_signature_receiver);
            
        let ctxt_arg = self.ctxt_arg()?
            .map(Arg::arg_signature_context)
            .unwrap_or_else(|| quote!(_));

        let other_args = self.other_arguments()?.map(|args| {
            let (other_arg_names, other_arg_types) = args
                .map(Arg::arg_signature)
                .unzip::<_,_,Vec<_>, Vec<_>>();

            quote_spanned!(self.sig.span=>
                (#(#other_arg_names),*) : (#(#other_arg_types),*)
            )
        });
        

        Ok(vec![Some(ctxt_arg), self_arg, other_args]
            .into_iter()
            .filter(Option::is_some)
            .collect::<Punctuated::<Option<proc_macro2::TokenStream>, Token![,]>>()
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
    fn generate_mlua_body_unwrapped_parameter_assignments(&self) -> syn::Result<proc_macro2::TokenStream> {        
        Ok(self.sig.inputs
            .iter()
            .map(Arg::unpack_parameter)
            .collect::<Result<Vec<_>>>()?
            .iter()
            .zip(self.sig.inputs.iter())
            .filter_map(|(unpacked_param, arg)| 
                unpacked_param.as_ref().map(|unpacked_param| {
                    let name = &arg.name;
                    quote_spanned!{name.span()=>let #name = #unpacked_param;}
                }
            ))
            .collect::<proc_macro2::TokenStream>())
    }

    /// Similar to generate_mlua_body_output but for functions, makes some more assumptions and directly generates wrapped/unwrapped output depending on what's necessary
    /// Does not require another wrapping step and can be directly put in a result as the final output of an mlua closure
    fn generate_mlua_body_output_field(&self, raw_output_ident: &Ident) -> syn::Result<proc_macro2::TokenStream>{
        let field_type = if self.attrs.kind.is_field_getter(){
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
        let field_name = &self.name;
        let field_name_str =  syn::Lit::Str(LitStr::new(&self.name.to_string(), self.name.span()));
        let proxy_output_type = LuaTypeConstructorVisitor::new(true,false).visit(&field_type.type_);

        Ok(match &field_type.type_ {
            // proxy, need to index into it then wrap the result
            // getter
            t if t.contains_proxy_type() 
                && self.attrs.kind.is_field_getter() => quote!(
                    let #raw_output_ident = #proxy_output_type::new_ref(#self_name.script_ref(#world_ptr).index(std::borrow::Cow::Borrowed(#field_name_str)));
                ),
            // setter
            t if t.contains_proxy_type() => {

                let first_arg_name = &self.get_other_arguments()?.next()
                    .ok_or_else(|| syn::Error::new(self.sig.span, "Field setter requires a single argument which is missing"))?
                    .name;
                quote!(
                    let #raw_output_ident = #first_arg_name.apply_self_to_base(&mut #self_name.script_ref(#world_ptr).index(std::borrow::Cow::Borrowed(#field_name_str)))?;
                )
            },

            // plain reflection, index into the ScriptRef with the field path
            // getter
            SimpleType::Type(syn::Type::Path(path)) 
                if path.path.is_ident("ReflectedValue") 
                && self.attrs.kind.is_field_getter() => todo!(),
            // setter
            SimpleType::Type(syn::Type::Path(path)) 
                if path.path.is_ident("ReflectedValue") => todo!(),

            // primitive use clone on the value and return it without a wrapper
            // getter
            _ if self.attrs.kind.is_field_getter() => quote!(
                    let #raw_output_ident = #self_name.val(|#self_name| #self_name.#field_name.clone())?;
                ),
            // setter
            _ => {
                let first_arg_name = &self.get_other_arguments()?.next()
                    .ok_or_else(|| syn::Error::new(self.sig.span, "Field setter requires a single argument which is missing"))?
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
    fn generate_mlua_body_raw_output(&self, raw_output_ident: &Ident, proxied_type_path: &Path) -> syn::Result<proc_macro2::TokenStream> {
        // generate function call on proxied type (operating over unwrapped parameters)
        // output will be stored in raw_output_ident with the proxied_type_path
        
        // the type before we wrap it in a proxy
        let raw_output_type = LuaTypeConstructorVisitor::new(false,false).visit(&self.sig.output.type_);

        match &self.default{
            Some(body) => {
                
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor{
                        target: "self",
                        replacement: SELF_ALIAS,
                    }.visit_stmt_mut(&mut s);
                    s
                });

                Ok(quote_spanned!{body.span()=>
                    let #raw_output_ident : #raw_output_type = 
                        (||{
                            #(#stmts)*
                        })();
                })          
            },
            None => {
                let function_name = &self.name;
                let param_names = self.sig.inputs.iter()
                    .map(|arg| &arg.name).collect::<Vec<_>>();

                // override this span, as otherwise spans propagate weird
                let mut proxied_name = proxied_type_path.clone();

                proxied_name.segments.iter_mut().for_each(|v| v.ident.set_span(self.sig.span));

                
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
    fn generate_mlua_body_proxy_output(&self, proxied_output_ident: &Ident, proxy_output_ident: &Ident) -> syn::Result<proc_macro2::TokenStream> {
        if self.sig.output.is_raw {
            return Ok(quote_spanned! {self.sig.span=>
                let #proxy_output_ident = #proxied_output_ident;
            })
        }
        
        // generate `new` calls as required to build proxy stored in out_ident
        let constructor_wrapped_expression = 
            LuaSimpleTypeWrapper::new(proxied_output_ident.clone(), proxied_output_ident.span())
                .visit(&self.sig.output.type_)?;

        // the type of the wrapped type (if wrapped)
        let proxy_output_type = LuaTypeConstructorVisitor::new(true,false).visit(&self.sig.output.type_);
        
        // the statement assigning the proxy output to proxy_output_ident
        Ok(quote_spanned! {self.sig.span=>
            let #proxy_output_ident : #proxy_output_type = #constructor_wrapped_expression;
        })
    }

    fn generate_mlua_body(&self, proxied_type_path: &Path) -> syn::Result<proc_macro2::TokenStream> {
        let unpacked_parameter_declarations = self.generate_mlua_body_unwrapped_parameter_assignments()?;

        let proxied_output_ident = format_ident!("{PROXIED_OUT_ALIAS}", span=self.sig.span);
        let proxy_output_ident = format_ident!("{PROXY_OUT_ALIAS}", span=self.sig.span);
        
        let raw_output_stmt = if self.attrs.kind.is_field() {
            self.generate_mlua_body_output_field(&proxied_output_ident)?
        } else {
            self.generate_mlua_body_raw_output(&proxied_output_ident, proxied_type_path)?
        };

        // for fields the output is expected to be raw anyway so this will just performa no-op
        let proxy_output_stmt = self.generate_mlua_body_proxy_output(&proxied_output_ident, &proxy_output_ident)?;

        // determine if we need to wrap the output in an Ok() statement
        let last_stm = match &self.sig.output.type_ {
            SimpleType::DuoPath(DuoPath{ ident , ..}) if *ident == "Result" => quote_spanned! {self.sig.span=>#proxy_output_ident},
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
        let conversion_body_surrounded_with_dereferening_stms = self.sig.inputs.iter()
            .fold(conversion_body_stms, |acc, arg_meta| {
                    // only proxy types which are directly inside a reference are supported as references
                    if !matches!(arg_meta.type_, SimpleType::Reference(Reference{ ref inner, .. }) 
                        if matches!(inner.as_ref(), SimpleType::ProxyType(_))){
                            return acc;
                    }
                    // raw arguments are passed directly to the handler function
                    if arg_meta.is_raw {
                        return acc;
                    }

                    let method_call = if arg_meta.type_.has_outer_mut_ref() {
                        format_ident!("val_mut", span=arg_meta.span)
                    } else {
                        format_ident!("val", span=arg_meta.span)
                    };

                    let arg_name = &arg_meta.name;

                    quote_spanned!{self.sig.span=>{
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
        if self.sig.output.type_.has_ref() && self.sig.output.type_.contains_proxy_type() {
            return Err(syn::Error::new(
                self.sig.output.span,
                "Lua proxy functions do not support non 'static types as return values yet".to_string())
            )
        }

        if self.attrs.raw.is_present() {
            let ctxt_arg_idx = if self.attrs.kind.expects_receiver() {1} else {0};

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

        if self.attrs.kind.expects_receiver() {
            if let Some(receiver) = definition.sig.receiver() {
                // check receiver mutability
                if self.attrs.kind.expects_mutable_receiver() != receiver.mutability.is_some() {
                    // if incorrect and this is a method correct
                    if self.attrs.kind.is_method() {
                        // swap mutability in the kind
                        self.attrs.kind = match self.attrs.kind {
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
                                self.attrs.kind,
                                if self.attrs.kind.expects_mutable_receiver() { "&mut self or mut self" } else { "&self or self" }
                            )
                        ));
                    }
                };
            } else {
                return Err(syn::Error::new(
                    definition.sig.paren_token.span.span(),
                    format!(
                        "Lua proxy functions of type: {}, require `self` argument",
                        self.attrs.kind
                    )
                ));
            }
        } else if definition.sig.receiver().is_some() {
            return Err(syn::Error::new_spanned(
                definition.sig.receiver().unwrap(),
                format!(
                    "Lua proxy functions of type: {}, do not expect a receiver argument",
                    self.attrs.kind
                )
            ));
        }

        Ok(())
    }
}

#[derive(Default, FromMeta, Display, EnumString, EnumIter, PartialEq, Eq, Clone, Copy, Debug)]
#[darling(rename_all="PascalCase")]
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

    fn is_field_getter(self) -> bool {
        self == FunctionKind::FieldGetterMethod 
    }

    fn is_field_setter(self) -> bool {
        self == FunctionKind::FieldSetterMethod 
    }

    /// Returns true if the mlua closure signature accepts a tuple for general 'Arguments' to the function
    /// I.e. arguments freely passed to the function by the caller. 
    fn expects_arguments_tuple(self) -> bool {
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


// #[derive(FromAttributes)]
// #[darling(attributes(lua))]
// struct FieldAtrrs {
//     proxy: 
// }


/// Takes in field with all the required meta and converts it into a 
/// TraitItemFn representation
fn convert_field_to_lua_accessor(idx: usize, field: &Field, is_setter: bool) -> darling::Result<TraitItemFn> {
    let field_name = field.ident.clone().unwrap_or_else(|| format_ident!("_{}", idx));
    let field_type = &field.ty;
    let attrs = &field.attrs;
    let mut setter_args : Option<proc_macro2::TokenStream> = None;
    if let Some(attr) =  attrs.iter().find(|attr| attr.meta.path().is_ident("lua")) {
        attr.parse_nested_meta(|nested| {
            if nested.path.is_ident("output"){
                nested.parse_nested_meta(|nested| {
                    setter_args = Some(nested.input.parse()?);
                    Ok(())
                })?
            }
            Ok(())
        })?;
    }
    let setter_arg_attrs = setter_args.map(|tokens| Attribute{
        pound_token: Token![#](field.span()),
        style: AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList{
            path: Ident::new("proxy", field.span()).into(),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens,
        })});
    let trait_item_method : TraitItemFn = if is_setter {

        parse_quote!{
            #[lua(kind="FieldSetterMethod", raw)]
            #(#attrs)*
            fn #field_name (&mut self, lua: &Lua, #setter_arg_attrs other: #field_type);
        }
    } else {
        parse_quote!{
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
fn generate_mlua_registration_code(proxied_type_path: &Path, function_def: TraitItemFn) -> darling::Result<proc_macro2::TokenStream> {
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    let attrs = FunctionAttributes::from_attributes(&function_def.attrs)?;
    // if skipping return no-op
    if attrs.skip.is_present() {
        return Ok(Default::default())
    };

    let method_documentation_calls = attrs.doc
        .iter()
        .map(|tkns| quote_spanned!(function_def.span()=>methods.document_type(#tkns);))
        .collect::<proc_macro2::TokenStream>();

    let function_name = function_def.sig.ident.clone();
    let output_attrs = attrs.output.clone().map(|meta| {
        let meta = meta.require_list()?.parse_args::<Meta>()?;
        Ok::<_,syn::Error>(vec![Attribute{ pound_token: Token![#](meta.span()), style: AttrStyle::Outer, bracket_token: Default::default(), meta }])
        }).transpose()?
        .unwrap_or_default();
    let signature = Signature::new(proxied_type_path.clone(), function_def.sig, attrs.raw.is_present(), output_attrs)?;
    let function = Function::new(function_name.clone(), attrs, function_def.default, signature)?;

    let args = function.generate_mlua_args()?;

    let body = function.generate_mlua_body(proxied_type_path)?;
    let closure = quote_spanned! {body.span()=>
        |#args| {
            #body
        }
    };

    let tealr_function = format_ident!("{}", function.attrs.kind.get_tealr_function(), span=body.span());
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
        format_ident!("fields", span=body.span())
    } else {
        format_ident!("methods", span=body.span())
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
            "The `name` attribute is not supported for lua proxies"
        ).to_compile_error().into();
    }

    let proxied_type_path : syn::Path = meta.remote.unwrap_or(meta.ident.clone().into());
    let proxied_type_str = proxied_type_path.segments.last().unwrap().ident.to_string();
    let proxy_type_ident = format_ident!("{PROXY_PREFIX}{}", &meta.ident);
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    // optional clone extensions
    let opt_with_clone = meta.derive.clone
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
    let methods =  meta.functions.0.into_iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path,v)))
        .collect::<Vec<_>>();

    let fields = field_methods.into_iter()
        .map(|v| errors.handle_in(|| generate_mlua_registration_code(&proxied_type_path,v)))
        .collect::<Vec<_>>();

    // stop if any errors so far
    if let Err(e) = errors.finish() {
        return e.write_errors().into();
    }

    let a = quote_spanned! {derive_input.span()=>

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
    .into();
    // panic!("{}",a);
    a
}


#[cfg(test)]
mod test {
    use darling::{FromAttributes, FromMeta};
    use quote::format_ident;
    use syn::{TraitItemFn, Meta};

    use crate::{FunctionAttributes, Arg, ArgAttributes};



    #[test]
    fn test_parse_function_attributes_parses(){
        let function = "
            #[lua(output(proxy))] 
            fn asd(#[proxy] arg: String, #[proxy(Type=\"LuaType\")] arg2: (String, Type)) -> String;
        ";
        let trait_fn : TraitItemFn = syn::parse_str(function).unwrap();

        FunctionAttributes::from_attributes(&trait_fn.attrs).unwrap();
    }
}