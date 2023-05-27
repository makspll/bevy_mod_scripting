use bevy_mod_scripting_common::{
    implementor::WrapperImplementor,
    input::{DeriveFlag, ProxyMeta},
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
    PathArguments, PathSegment, TraitItemMethod, Type, TypePath,
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

struct FunctionArgMeta {
    is_receiver: bool,
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
    fn new_from_fn_arg(proxy_name: &Ident, fn_arg: &FnArg) -> Self {
        let is_a_lua_proxy;
        let arg_type;
        let arg_name;
        let mutable;
        let is_receiver;
        match fn_arg {
            FnArg::Receiver(receiver) => {
                is_receiver = true;
                is_a_lua_proxy = true;
                arg_type = parse_quote!(#proxy_name);

                if let Some(_mut) = receiver.mutability {
                    mutable = Some(_mut)
                } else {
                    mutable = None
                }

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
                        Meta::Path(_) => generate_automatic_proxy_name(proxy_name),
                        Meta::List(MetaList{ nested, .. }) => {
                            if let Some(NestedMeta::Lit(Lit::Str(proxy_name))) = nested.first() {
                                return format_ident!("{}", proxy_name.token().to_string())
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

                arg_name = parse_quote!(#pat);
            }
        }

        FunctionArgMeta {
            is_receiver,
            is_a_lua_proxy,
            mutable,
            arg_type,
            arg_name,
            span: fn_arg.span(),
        }
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        proxy_name: &Ident,
        arg_name: Ident,
        arg_type: &Type,
        attrs: Vec<Attribute>,
    ) -> Self {
        let ty = Box::new(arg_type.clone());
        let pat_ty = PatType {
            attrs,
            pat: parse_quote!(#arg_name),
            colon_token: Default::default(),
            ty,
        };

        Self::new_from_fn_arg(proxy_name, &FnArg::Typed(pat_ty))
    }
}

struct FunctionMeta<'a> {
    name: &'a Ident,
    body: &'a TraitItemMethod,
    fn_type: FunctionType,
    arg_meta: Vec<FunctionArgMeta>,
    output_meta: Option<FunctionArgMeta>,
}

impl FunctionMeta<'_> {

    fn new<'a>(
        proxy_name: &'a Ident,
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
                        .map(|arg| FunctionArgMeta::new_from_fn_arg(proxy_name, arg))
                        .collect(),
                    output_meta: match &body.sig.output {
                        syn::ReturnType::Default => None,
                        syn::ReturnType::Type(_, t) => Some(FunctionArgMeta::new_from_type(
                            proxy_name,
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
                let type_path = &fn_arg.arg_type;
                (
                    quote_spanned!(fn_arg.span=>#name ),
                    quote_spanned!(fn_arg.span=>#_mut #type_path),
                )
            })
            .unzip::<_, _, Vec<proc_macro2::TokenStream>, Vec<proc_macro2::TokenStream>>();

        quote_spanned!(self.body.sig.inputs.span()=>_ #self_arg , (#(#args),*) : (#(#arg_types),*))
    }

    fn generate_mlua_body(&self, proxied_name: &Ident) -> proc_macro2::TokenStream {
        if let Some(body) = &self.body.default {
            return body.to_token_stream();
        }

        let parameters = self.arg_meta.iter().map(|arg| {
            let name = &arg.arg_name;
            if arg.is_a_lua_proxy {
                quote_spanned!(name.span()=>#name.inner()?)
            } else {
                quote_spanned!(name.span()=>#name)
            }
        });

        let function_name = self.name;

        let inner_method_call = if self.fn_type.expects_receiver() {
            let self_ident = format_ident!("{}", SELF_ALIAS);

            quote_spanned! {self.body.span()=>
                #self_ident.inner()?
                    .#function_name(#(#parameters),*)
            }
        } else {
            quote_spanned! {self.body.span()=>
                #proxied_name::#function_name(#(#parameters),*)
            }
        };

        if let Some(output_meta) = &self.output_meta {
            if output_meta.is_a_lua_proxy {
                let proxied_name = &output_meta.arg_type;
                return quote_spanned! {self.body.span()=>
                    Ok(#proxied_name::new(#inner_method_call))
                };
            }
        }

        quote_spanned!(self.body.span()=>Ok(#inner_method_call))
    }

    
    
    fn validate_function_definition(&self, definition: &TraitItemMethod) {
        if self.fn_type.expects_receiver() {
            if let Some(FnArg::Receiver(receiver)) = definition.sig.receiver() {
                // validate receiver
                if receiver.reference.is_some() {
                    emit_error!(receiver, "Proxy receivers can only be one of: `self` or `mut self`, Proxies have pass by value semantics.")
                }
                if self.fn_type.expects_mutable_receiver() && receiver.mutability.is_none() {
                    emit_error!(
                        receiver,
                        format!(
                            "Lua proxy functions of type: {}, require `mut self` argument",
                            self.fn_type.as_str()
                        )
                    );
                }
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

#[derive(PartialEq, Eq, Clone, Copy)]
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

    let proxy_name = if meta.proxy_name == meta.proxied_name {
        generate_automatic_proxy_name(&meta.proxy_name)
    } else {
        meta.proxy_name
    };

    let proxied_name = meta.proxied_name;
    let is_clonable = meta.proxy_flags.flags.contains(&DeriveFlag::Clone);

    // generate the type definition of the proxy
    let mut definition: proc_macro2::TokenStream;
    if is_clonable {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_name with Clone);
        };
    } else {
        definition = quote_spanned! {meta.span=>
            bevy_script_api::make_script_wrapper!(#proxied_name as #proxy_name);
        }
    }

    if meta.proxy_flags.flags.contains(&DeriveFlag::Debug) {
        definition.extend(quote_spanned!{meta.span=>
            impl std::fmt::Debug for #proxy_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))}    
            }
        });
    }

    let tealr_type_implementations = quote_spanned! {meta.span=>
        bevy_script_api::impl_tealr_type!(#proxy_name);
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
            .map(|ts| quote_spanned!(body.span()=>methods.document_type(#ts);));

        let fn_meta = FunctionMeta::new(&proxy_name, name, body);
        let args = fn_meta.generate_mlua_args();
        let body = fn_meta.generate_mlua_body(&proxied_name);
        let closure = quote_spanned! {body.span()=>
            |#args| {
                #body
            }
        };
        panic!("{}", closure);

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
        #[allow(unused_parens)]
        impl #tealr::mlu::TealData for #proxy_name {
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

fn generate_automatic_proxy_name(proxied_name: &Ident) -> Ident {
    format_ident!("Lua{}", proxied_name)
}
