use std::borrow::Cow;

use bevy_mod_scripting_common::{
    implementor::WrapperImplementor,
    input::{DeriveFlag, ProxyMeta, IdentifierRenamingVisitor, SimpleType, UnitPath, ProxyType, VisitSimpleType},
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
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::{Mut, Paren},
    Attribute, DeriveInput, Error, FnArg, Lit, Meta, MetaList, NestedMeta, Pat, PatType, Path,
    PathArguments, PathSegment, TraitItemMethod, Type, TypePath, PatIdent, visit_mut::VisitMut, TypeTuple, ReturnType};

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
const OUT_ALIAS: &str = "__out";
const PROXY_PREFIX: &str = "Lua";

/// categorises and provides meta data about the broad categories of function arguments/ outputs which are
/// handled by the macro
#[derive(Debug)]
enum ArgVariant {
    NonProxy{
        arg_type: Type,
    },
    Proxy{
        proxy_type: SimpleType,
    },
}

#[derive(Debug)]
struct FunctionArgMeta {
    mutable: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    arg_name: Ident,
    span: Span,
    /// variant specific data enumeration
    variant_data: ArgVariant,
}

impl FunctionArgMeta {
    /// Creates a new meta structure corresponding to the given function argument.
    /// Resolves receivers with the given proxy name.
    fn new_from_fn_arg(simple_type: &SimpleType, fn_arg: &FnArg) -> Self {
        let arg_name;
        let mutable;
        let variant_data;

        match fn_arg {
            FnArg::Receiver(receiver) => {
                variant_data = ArgVariant::Proxy { proxy_type: simple_type.clone() };

                if let Some(_mut) = receiver.mutability {
                    mutable = Some(_mut)
                } else {
                    mutable = None
                }

                arg_name = format_ident!("{}", SELF_ALIAS);
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
                        Meta::Path(_) => simple_type.clone(),
                        other => abort!(other.span(), "Expected single item attribute list containing proxy name as in: `proxy(\"proxy_name\")`")
                    });

                if let Some(proxy_type) = passed_proxy_type {
                    variant_data = ArgVariant::Proxy { proxy_type }
                } else {
                    variant_data = ArgVariant::NonProxy { arg_type: *ty.clone()}
                }


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
            span: fn_arg.span(),
            variant_data,
        }
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        simple_type: &SimpleType,
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


        Self::new_from_fn_arg(simple_type, &FnArg::Typed(pat_ty))
    }

    /// Unpacks non-reference proxy parameters (using the `inner` method) yielding expressions which correspond to the proxied type with conversion errors being
    /// handled by the try `?` operator.
    pub fn unpack_parameter(&self) -> proc_macro2::TokenStream {
        let name = &self.arg_name;

        // if a proxy parameter is to be passed by value we use inner (which requires Clone to be supported)
        if matches!(&self.variant_data, ArgVariant::Proxy{proxy_type} if !proxy_type.has_outer_ref())  {
            quote_spanned!(name.span()=> #name.inner()?)
        } else {
            // if the parameter is not a proxy or a reference
            // we do not call anything as .inner()
            name.to_token_stream()
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
                        .map(|arg| (SimpleType::new_from_fn_arg(PROXY_PREFIX, arg, &proxied_type_identifier).unwrap_or_else(|(s,e)| abort!(s,e)), arg))
                        .map(|(simple_type,arg)| FunctionArgMeta::new_from_fn_arg(&simple_type, arg))
                        .collect(),
                    output_meta: FunctionArgMeta::new_from_type(
                            &SimpleType::new_from_contextual_type(PROXY_PREFIX, &output_type, &proxied_type_identifier).unwrap_or_else(|(s,e)| abort!(s,e)),
                            format_ident!("{OUT_ALIAS}"),
                            &output_type,
                            output_attrs,
                        )
                    ,
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
                let self_type = match &meta.variant_data {
                    ArgVariant::Proxy { proxy_type } => proxy_type.construct_proxy_type_without_outer_ref(),
                    ArgVariant::NonProxy { arg_type } => abort!(arg_type, "Function expects a receiver type as first argument which needs to be a proxy type") ,
                };

                self_arg = Some(quote_spanned!(meta.span=> , #self_name : & #_mut #self_type));
            }
        }

        let (args, arg_types) = args
            .map(|fn_arg| {
                let _mut = &fn_arg.mutable;
                let name = &fn_arg.arg_name;

                // strip outer refs if the type is a proxy, we cannot have any references in type position
                // we can still have reference semantics since we have a proxy object, however we still pass it by value
                let type_path = match &fn_arg.variant_data {
                    ArgVariant::Proxy { proxy_type } => proxy_type.construct_proxy_type_without_outer_ref(),
                    ArgVariant::NonProxy { arg_type } => arg_type.clone()
                };

                (
                    quote_spanned!(fn_arg.span=>#_mut #name),
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

        // unpack all parameters which need to be unpacked via `.inner` calls, we deal with reference proxies later 
        let mut unpacked_parameters = self.arg_meta
            .iter()
            .map(FunctionArgMeta::unpack_parameter);

        let function_name = self.name;

        let proxied_method_call = 
        match (self.fn_type.expects_receiver(),&self.body.default){
            (_, Some(body)) => {
                let param_names = self.arg_meta.iter().map(|arg| &arg.arg_name);
                
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor{
                        target: "self",
                        replacement: SELF_ALIAS,
                    }.visit_stmt_mut(&mut s);
                    s
                });

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
                    #first_arg.#function_name(#(#unpacked_parameters),*)
                }
            },
            (false, None) => {
                quote_spanned! {self.body.span()=>
                    #proxied_name::#function_name(#(#unpacked_parameters),*)
                }
            },
        };

        
        // if the output is also a proxied type, we need to wrap the result in a proxy
        let constructor_wrapped_full_call= match &self.output_meta.variant_data {
            ArgVariant::Proxy{proxy_type} => {
                let out_ident = format_ident!("{OUT_ALIAS}");

                let constructor_wrapped_expression = ProxyMapVisitor{ arg_name: out_ident.clone(), span: self.body.span() }.visit_simple_type(proxy_type);

                // the type before we wrap it in a proxy
                let proxied_output_type = proxy_type.construct_proxied_type();
                // the type after we wrap it in a proxy
                let output_type = proxy_type.construct_proxy_type();
                
                quote_spanned! {self.body.span()=>
                    let #out_ident : #proxied_output_type = #proxied_method_call;
                    let __output : #output_type = #constructor_wrapped_expression;
                    Ok(__output)
                }
            }
            ArgVariant::NonProxy{ arg_type } => {
                quote_spanned!{self.body.span()=>
                    let __output : #arg_type = #proxied_method_call;
                    Ok(__output)
                }
            }
        };
        
        // for every argument which is a reference, we need a separate sort of call,
        // we cannot use `v.inner()` since this operates over values, we must use `val_mut` or `val` to get a reference to the wrapped
        // structure for the duration of the call 
        let reference_unpacked_constructor_wrapped_full_call = self.arg_meta.iter()
            .fold(constructor_wrapped_full_call, |acc, arg_meta| {
                if let ArgVariant::Proxy{proxy_type} = &arg_meta.variant_data {
                    if !proxy_type.has_outer_ref(){
                        return acc;
                    }

                    let method_call = if proxy_type.has_outer_mut_ref() {
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

    // generate both tealr documentation and instantiations of functions
    let methods = meta.functions.iter().map(|(name, body)| {
        let method_documentation_calls = body
            .attrs
            .iter()
            .map(attribute_to_string_lit)
            .filter(|s| !s.is_empty())
            .map(|tkns| quote_spanned!(body.span()=>methods.document_type(#tkns);));


        let fn_meta = FunctionMeta::new(proxied_name.clone(), name, body);
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

        impl #tealr::mlu::TealData for #proxy_type {
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



/// `maps` a simple type recursively, expanding the type into a series of map calls where the leaf types are operating over
/// unwrapped proxy types (the inner types)
fn map_simple_type<A,B>(proxy_type : &SimpleType, 
        input_arg_ident: Cow<Ident>, 
        proxy_type_expansion: A,
        type_expansion: B) -> proc_macro2::TokenStream 
where
    A : Fn(&Ident, &Ident) -> proc_macro2::TokenStream,
    B : Fn(&Type) -> proc_macro2::TokenStream
    {
    match proxy_type {
        SimpleType::Unit => todo!("Unit type not supported"),
        SimpleType::UnitPath (UnitPath{ ident,inner, .. }) => {
            let inner = map_simple_type(inner.as_ref(), input_arg_ident.clone(), proxy_type_expansion, type_expansion);
            quote_spanned!(ident.span()=>
                #input_arg_ident.map(|#input_arg_ident| {#inner})
            )
        },
        SimpleType::Reference { .. } => todo!(),
        SimpleType::ProxyType(ProxyType{ proxied_ident, proxy_ident }) => proxy_type_expansion(proxied_ident, proxy_ident),
        SimpleType::Type(t) => type_expansion(t),
    }
}

/// `maps` a simple type recursively, expanding the type into a series of map/iter/etc calls where the leaf types are operating over
/// unwrapped proxy types (the inner types).
/// 
/// requires arg_name to be a valid identifier refering to the name of the variable containing a value with the SimpleType being mapped.
/// The returned token stream will be an expression. 
struct ProxyMapVisitor {
    arg_name: Ident,
    span: Span,
}

impl VisitSimpleType<proc_macro2::TokenStream> for ProxyMapVisitor {
    fn visit_unit_path(&self, unit_path: &UnitPath) -> proc_macro2::TokenStream {
        let inner = self.visit_simple_type(&unit_path.inner);
        let arg_name = &self.arg_name;
        quote_spanned!(self.span=>
            #arg_name.map(|#arg_name| {#inner})
        )
    }

    fn visit_unit(&self) -> proc_macro2::TokenStream {
        quote_spanned!(self.span=>
            ()
        )
    }

    fn visit_proxy_type(&self, proxy_type: &ProxyType) -> proc_macro2::TokenStream {
        let proxy_ident = &proxy_type.proxy_ident;
        let arg_name = &self.arg_name;
        quote_spanned!{self.span=>
            #proxy_ident::new(#arg_name)
        }
    }

    fn visit_type(&self, _type: &Type) -> proc_macro2::TokenStream {
        _type.to_token_stream()
    }
}