use std::borrow::Cow;

use bevy_mod_scripting_common::{
    implementor::WrapperImplementor,
    input::{DeriveFlag, ProxyMeta, ProxyFlags, ProxyTypeNameMeta},
    newtype::Newtype,
    utils::{attribute_to_string_lit, ident_to_type_path},
};
use implementor::LuaImplementor;
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, emit_error, proc_macro_error};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::Mut,
    Attribute, DeriveInput, Error, FnArg, Lit, Meta, MetaList, NestedMeta, Pat, PatType, Path,
    PathArguments, PathSegment, TraitItemMethod, Type, TypePath, PatIdent,
};

pub(crate) mod derive_flags;
pub(crate) mod implementor;
pub(crate) mod impls;
pub(crate) mod lua_method;

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
const PROXY_PREFIX: &str = "Lua";

#[derive(Debug)]
struct FunctionArgMeta {
    is_receiver: bool,
    is_ref: bool,
    is_mut_ref: bool,
    is_a_lua_proxy: bool,
    mutable: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    arg_type: Type,
    arg_name: Ident,
    span: Span,
}

impl FunctionArgMeta {
    /// Creates a new meta structure corresponding to the given function argument.
    /// Resolves receivers with the given proxy name.
    fn new_from_fn_arg(proxy_type_name_meta: &ProxyTypeNameMeta, fn_arg: &FnArg) -> Self {
        let is_a_lua_proxy;
        let arg_type;
        let arg_name;
        let mutable;
        let is_receiver;
        let is_ref;
        let is_mut_ref;
        match fn_arg {
            FnArg::Receiver(receiver) => {
                is_receiver = true;
                is_a_lua_proxy = true;
                arg_type = Type::Path(TypePath{ qself: None, path: proxy_type_name_meta.get_proxy_type_identifier().clone().into()});

                if let Some(_mut) = receiver.mutability {
                    mutable = Some(_mut)
                } else {
                    mutable = None
                }
                is_ref = receiver.reference.is_some();
                is_mut_ref = receiver.mutability.is_some();

                arg_name = format_ident!("{}", SELF_ALIAS);
            }
            FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                is_receiver = false;
                let proxy_attr = attrs
                    .iter()
                    .find_map(|attr| attr.path.is_ident("proxy").then_some(attr));

                is_a_lua_proxy = proxy_attr.is_some();
                mutable = proxy_attr.is_some().then(|| parse_quote!(mut));

                let passed_proxy_name = proxy_attr
                    .map(|attr| attr.parse_meta().unwrap_or_else(|err| abort!(attr, err)))
                    .map(|meta| match meta {
                        Meta::Path(_) => {
                            ProxyTypeNameMeta::proxy_type_to_ident(PROXY_PREFIX,ty,proxy_type_name_meta.get_proxied_type_identifier()).unwrap_or_else(|(span,err)| abort!(span,err))
                        },
                        Meta::List(MetaList{ nested, .. }) => {
                            if let Some(NestedMeta::Lit(Lit::Str(proxy_name))) = nested.first() {
                                let string = proxy_name.token().to_string();
                                return format_ident!("{}", string[1..string.len()-1])
                            }
                            abort!(nested, "Expected single item attribute list containing proxy name as in: `proxy(\"proxy_name\")`");
                        },
                        Meta::NameValue(name_val) => abort!(name_val, "Expected single item attribute list containing proxy name as in: `proxy(\"proxy_name\")`")
                    });

                if let Some(name) = passed_proxy_name {
                    arg_type = Type::Path(ident_to_type_path(name));
                } else {
                    arg_type = *ty.clone();
                }

                match ty.as_ref() {
                    Type::Reference(t) => {
                        is_ref = true;
                        is_mut_ref = t.mutability.is_some();
                    },
                    _ => {
                        is_ref = false;
                        is_mut_ref = false;
                    },
                }

                arg_name = match pat.as_ref() {
                    Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                    Pat::Wild(_) => abort!(pat, "Cannot use `_` as identifier for proxy function"),
                    _ => abort!(pat, "Unsupported parameter pattern"),
                };

            }
        }

        FunctionArgMeta {
            is_receiver,
            is_a_lua_proxy,
            mutable,
            arg_type,
            arg_name,
            span: fn_arg.span(),
            is_ref,
            is_mut_ref,
        }
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        proxy_type_name_meta: &ProxyTypeNameMeta,
        arg_name: Ident,
        arg_type: &Type,
        attrs: Vec<Attribute>,
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


        Self::new_from_fn_arg(proxy_type_name_meta, &FnArg::Typed(pat_ty))
    }

    pub fn get_type_stripped_from_outer_references(&self) -> &Type {
        match &self.arg_type {
            Type::Reference(r) => &r.elem,
            _ => &self.arg_type,
        }
    }
}

#[derive(Debug)]
struct FunctionMeta<'a> {
    name: &'a Ident,
    body: &'a TraitItemMethod,
    fn_type: FunctionType,
    arg_meta: Vec<FunctionArgMeta>,
    output_meta: Option<FunctionArgMeta>,
}

impl FunctionMeta<'_> {

    fn new<'a>(
        proxy_type_name_meta: &ProxyTypeNameMeta,
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
                
                nested
                    .iter()
                    .for_each(|attr|{
                        if let NestedMeta::Meta(Meta::Path(p)) = attr {
                            let attr_str = p.get_ident().map(Ident::to_string).unwrap_or_default();
                            if let Ok(_fn_type) = FunctionType::from_str(&attr_str) {
                                fn_type = _fn_type;
                                return;
                            }
                        } else if let NestedMeta::Meta(Meta::List(list)) = attr {
                            if list.path.is_ident("output") {
                                for attr in list.nested.iter() {
                                    output_attrs.push(parse_quote!(#[#attr]))
                                }
                                return;
                            }
                        }
    
                        emit_error!(attr, "unknown or malformed lua proxy function attribute. Allowed attributes include: {}",
                            FunctionType::iterator()
                            .map(FunctionType::as_ident_str).collect::<Vec<_>>().join(","));
                    });
                let fn_meta = FunctionMeta {
                    name,
                    body,
                    fn_type,
                    arg_meta: body
                        .sig
                        .inputs
                        .iter()
                        .map(|arg| FunctionArgMeta::new_from_fn_arg(proxy_type_name_meta, arg))
                        .collect(),
                    output_meta: match &body.sig.output {
                        syn::ReturnType::Default => None,
                        syn::ReturnType::Type(_, t) => Some(FunctionArgMeta::new_from_type(
                            proxy_type_name_meta,
                            format_ident!("out"),
                            t,
                            output_attrs,
                        )),
                    },
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
                let self_type = &meta.arg_type;

                self_arg = Some(quote_spanned!(meta.span=> , #self_name : & #_mut #self_type));
            }
        }

        let (args, arg_types) = args
            .map(|fn_arg| {
                let _mut = &fn_arg.mutable;
                let name = &fn_arg.arg_name;

                // strip outer refs if the type is a proxy, we cannot have any references in type position
                // we can still have reference semantics since we have a proxy object which we pass by value
                let type_path = if fn_arg.is_a_lua_proxy {
                    fn_arg.get_type_stripped_from_outer_references()
                } else {
                    &fn_arg.arg_type
                };

                (
                    quote_spanned!(fn_arg.span=>#_mut #name ),
                    quote_spanned!(fn_arg.span=>#type_path),
                )
            })
            .unzip::<_, _, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>>();

        quote_spanned!(self.body.sig.inputs.span()=>_ #self_arg , (#(#args),*) : (#(#arg_types),*))
    }

    fn generate_mlua_body(&self, proxied_name: &Ident) -> proc_macro2::TokenStream {

        // override this span, as otherwise spans propagate weird
        let mut proxied_name = proxied_name.clone();
        proxied_name.set_span(self.body.sig.ident.span());

        // unpack all parameters which need to be unpacked via `.inner` calls, turn the rest into
        let mut unpacked_parameters = self.arg_meta.iter().map(|arg| {
            let name = &arg.arg_name;

            // if a parameter is to be passed by value we use inner (which requires Clone to be supported)
            if arg.is_a_lua_proxy && !arg.is_ref  {
                quote_spanned!(name.span()=> #name.inner()?)
            } else {
                // otherwise we depend on a later step to `turn` #name into an identifier for a reference within 
                // the context of a closure
                name.to_token_stream()
            }
        });

        let proxied_function_name = self.name;

        let proxied_method_call = 
        match (self.fn_type.expects_receiver(),&self.body.default){
            (_, Some(body)) => {
                let param_names = self.arg_meta.iter().map(|arg| &arg.arg_name);
                let stmts = body.stmts.iter();
                
                quote_spanned!{body.span()=>
                    {
                        #(let #param_names = #unpacked_parameters;)*
                        (||{
                            #(#stmts)*
                        })()
                    }
                }            
            },
            (true, None) => {
                // this removes the first argument taken to be the receiver from the iterator for the next step
                let first_arg = unpacked_parameters.next().unwrap_or_else(|| abort!(self.name,"Proxied functions of the type: {} expect a receiver argument (i.e. self)", self.fn_type.as_str()));

                // since we removed the receiver we can pass the rest of the parameters here;
                quote_spanned! {self.body.span()=>
                    #first_arg.#proxied_function_name(#(#unpacked_parameters),*)
                }
            },
            (false, None) => {
                quote_spanned! {self.body.span()=>
                    #proxied_name::#proxied_function_name(#(#unpacked_parameters),*)
                }
            },
        };

        
        // if the output is also a proxied type, we need to wrap the result in a proxy
        let constructor_wrapped_full_call= match &self.output_meta {
            Some(output_meta) if output_meta.is_a_lua_proxy => {
                let proxy_type = &output_meta.arg_type;
                quote_spanned! {self.body.span()=>
                    let __output : #proxy_type = #proxy_type::new(#proxied_method_call);
                    Ok(__output)
                }
            }
            Some(output_meta) => {
                let output_type = &output_meta.arg_type;
                quote_spanned!{self.body.span()=>
                    let __output : #output_type = #proxied_method_call;
                    Ok(__output)
                }
            },
            None => quote_spanned!{self.body.span()=>
                let __output : () = #proxied_method_call; 
                Ok(__output)
            }
        };
        
        // for every argument which is a reference, we need a separate sort of call,
        // we cannot use `v.inner()` since this operates over values, we must use `val_mut` or `val` to get a reference to the wrapped
        // structure for the duration of the call 
        let reference_unpacked_constructor_wrapped_full_call = self.arg_meta.iter()
            .fold(constructor_wrapped_full_call, |acc, arg_meta| {
                if arg_meta.is_ref {
                    let method_call = if arg_meta.is_mut_ref {
                        format_ident!("val_mut")
                    } else {
                        format_ident!("val")
                    };

                    let arg_name = &arg_meta.arg_name;

                    quote_spanned!{self.body.span()=>{
                        #arg_name.#method_call(|#arg_name| {#acc})?
                    }}
                } else {
                    acc
                }
            });
        reference_unpacked_constructor_wrapped_full_call

    }

    
    
    fn validate_function_definition(&self, definition: &TraitItemMethod) {
        if self.fn_type.expects_receiver() {
            if let Some(FnArg::Receiver(receiver)) = definition.sig.receiver() {
                // validate receiver
                if self.fn_type.expects_mutable_receiver() && receiver.mutability.is_none() {
                    emit_error!(
                        receiver,
                        format!(
                            "Lua proxy functions of type: {}, require `mut self` or `&mut self` argument",
                            self.fn_type.as_str()
                        )
                    );
                };
            } else {
                emit_error!(
                    definition,
                    format!(
                        "Lua proxy functions of type: {}, require `self` argument",
                        self.fn_type.as_str()
                    )
                )
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum FunctionType {
    Function,
    MetaFunction,
    Method,
    MetaMethod,
    MutableFunction,
    MutableMetaFunction,
    MutatingMethod,
    MutatingMetaMethod,
}

impl FunctionType {
    pub fn as_str(self) -> &'static str {
        match self {
            FunctionType::Function => "Function",
            FunctionType::MetaFunction => "Meta Function",
            FunctionType::Method => "Method",
            FunctionType::MetaMethod => "Meta Method",
            FunctionType::MutableFunction => "Mutating Function",
            FunctionType::MutableMetaFunction => "Mutable Meta Function",
            FunctionType::MutatingMethod => "Mutating Method",
            FunctionType::MutatingMetaMethod => "Mutating Meta Method",
        }
    }

    pub fn from_str(str: &str) -> Result<Self, ()> {
        match str {
            "Function" => Ok(FunctionType::Function),
            "MetaFunction" => Ok(FunctionType::MetaFunction),
            "Method" => Ok(FunctionType::Method),
            "MetaMethod" => Ok(FunctionType::MetaMethod),
            "MutableFunction" => Ok(FunctionType::MutableFunction),
            "MutableMetaFunction" => Ok(FunctionType::MutableMetaFunction),
            "MutatingMethod" => Ok(FunctionType::MutatingMethod),
            "MutatingMetaMethod" => Ok(FunctionType::MutatingMetaMethod),
            _ => Err(()),
        }
    }

    pub fn as_ident_str(self) -> String {
        self.as_str().split_whitespace().collect()
    }

    pub fn iterator() -> impl Iterator<Item = Self> {
        [
            FunctionType::Function,
            FunctionType::Method,
            FunctionType::MetaMethod,
            FunctionType::MetaFunction,
            FunctionType::MutableFunction,
            FunctionType::MutatingMethod,
            FunctionType::MutatingMetaMethod,
            FunctionType::MutableMetaFunction,
        ]
        .into_iter()
    }

    fn expects_receiver(self) -> bool {
        self == FunctionType::Method
            || self == FunctionType::MetaMethod
            || self == FunctionType::MutatingMethod
            || self == FunctionType::MutatingMetaMethod
    }

    fn expects_mutable_receiver(self) -> bool {
        self == FunctionType::MutatingMethod || self == FunctionType::MutatingMetaMethod
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

    let proxy_type_name_meta = ProxyTypeNameMeta::new(
        Type::Path(TypePath{ qself: None, path: meta.proxy_name.into() }),
        PROXY_PREFIX
    );

    let proxy_identifier = proxy_type_name_meta.get_proxy_type_identifier();

    let proxied_name = meta.proxied_name;
    let is_clonable = meta.proxy_flags.flags.contains(&DeriveFlag::Clone);

    // generate the type definition of the proxy
    let mut definition: proc_macro2::TokenStream;

    if is_clonable {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_identifier with Clone);
        };
    } else {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_identifier);
        }
    }

    if meta.proxy_flags.flags.contains(&DeriveFlag::Debug) {
        definition.extend(quote_spanned!{meta.span=>
            impl std::fmt::Debug for #proxy_identifier {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))}    
            }
        });
    }

    let tealr_type_implementations = quote_spanned! {meta.span=>
        bevy_script_api::impl_tealr_type!(#proxy_identifier);
    };

    // generate type level tealr documentation calls
    let type_level_document_calls = meta
        .docstrings
        .iter()
        .map(|tkns| quote_spanned!(meta.span=>methods.document_type(#tkns);));

    // generate both tealr documentation and instantiations of functions
    let methods = meta.functions.iter().map(|(name, body)| {
        let method_documentation_calls = body
            .attrs
            .iter()
            .map(attribute_to_string_lit)
            .filter(|s| !s.is_empty())
            .map(|tkns| quote_spanned!(body.span()=>methods.document_type(#tkns);));


        let fn_meta = FunctionMeta::new(&proxy_type_name_meta, name, body);
        let args = fn_meta.generate_mlua_args();
        let body = fn_meta.generate_mlua_body(&proxied_name);
        let closure = quote_spanned! {body.span()=>
            |#args| {
                #body
            }
        };

        let tealr_function = format_ident!("{}", fn_meta.fn_type.get_tealr_function());
        let signature = fn_meta
            .fn_type
            .is_meta()
            .then(|| fn_meta.name.to_token_stream())
            .unwrap_or_else(|| fn_meta.name.to_string().to_token_stream());

        quote_spanned! {body.span()=>
            #(#method_documentation_calls)*
            methods.#tealr_function(#signature, #closure);
        }
    });

    let tealr = quote!(bevy_mod_scripting_lua::tealr);

    quote_spanned! {meta.span=>

        #definition

        #tealr_type_implementations

        #[automatically_derived]
        #[allow(unused_parens,unused_braces)]
        #[allow(clippy::all)]

        impl #tealr::mlu::TealData for #proxy_identifier {
            fn add_methods<'lua, T: #tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
                #(#type_level_document_calls)*
                #(#methods)*
            }

            fn add_fields<'lua, T: #tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {

            }
        }

    }
    .into()
}


