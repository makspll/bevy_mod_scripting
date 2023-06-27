use std::{collections::{HashMap, HashSet}, str::FromStr};

use bevy_mod_scripting_common::{
    implementor::WrapperImplementor,
    input::{DeriveFlag, ProxyMeta, IdentifierRenamingVisitor, SimpleType, UnitPath, ProxyType, VisitSimpleType, DuoPath, TypeConstructorVisitor, StdTypeIdent},
    newtype::Newtype,
    utils::attribute_to_string_lit,
};
use implementor::LuaImplementor;
// use impls::{impl_enum, impl_struct};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::{abort, emit_error, proc_macro_error, ResultExt};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use strum::{EnumString, Display, EnumIter, IntoEnumIterator};
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, spanned::Spanned, token::{Mut, Paren},
    Attribute, DeriveInput, FnArg, Lit, Meta, MetaList, NestedMeta, Pat, PatType, TraitItemMethod, Type, TypePath, PatIdent, visit_mut::VisitMut, TypeTuple, ReturnType, MetaNameValue};

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

#[derive(Debug)]
struct FunctionArgMeta {
    mutable: Option<Mut>,
    /// the type of the argument, only suported patterns are allowed
    arg_name: Ident,
    span: Span,
    /// variant specific data enumeration
    arg_type: SimpleType,
}

impl FunctionArgMeta {
    /// Creates a new meta structure corresponding to the given function argument.
    /// Resolves receivers with the given proxy name.
    fn new_from_fn_arg(proxied_type_identifier: &Ident,fn_arg: &FnArg) -> Self {
        let arg_name;
        let mutable;
        let arg_type;
        // the proxied type is always proxied with the `Lua` prefix
        let mut proxy_ident_map = HashMap::from_iter(
            [(proxied_type_identifier.clone(),None)].into_iter()
        );
        match fn_arg {
            FnArg::Receiver(receiver) => {

                arg_type = SimpleType::new_from_fn_arg(PROXY_PREFIX, fn_arg, proxied_type_identifier, &proxy_ident_map).unwrap_or_abort();

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
                        Meta::Path(_) => SimpleType::new_from_contextual_type(PROXY_PREFIX, ty, proxied_type_identifier, &proxy_ident_map).unwrap_or_abort(),
                        // #[proxy(TypeName1=ProxyType1, TypeName2=ProxyType2, ..)]  
                        Meta::List(MetaList { nested, .. }) => {
                            // collect all the types passed in the meta as identifiers
                            let idents = nested.iter().map(|nested_meta| {
                                match nested_meta {
                                    NestedMeta::Meta(Meta::Path(path)) => 
                                        (path.get_ident().unwrap_or_else(|| abort!(path,"Expected identifier")).clone(),None),
                                    NestedMeta::Meta(Meta::NameValue(MetaNameValue{path, lit: Lit::Str(lit_str), ..})) => 
                                        (path.get_ident().unwrap_or_else(|| abort!(path,"Expected identifier")).clone(), Some(lit_str.parse().unwrap_or_abort())),
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
            span: fn_arg.span(),
            arg_type,
        }
    }

    /// Similar to [`Self::new_from_fn_arg`] but without an option of getting a receiver argument type
    fn new_from_type(
        proxied_type_identifier: &Ident,
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


        Self::new_from_fn_arg(proxied_type_identifier, &FnArg::Typed(pat_ty))
    }

    /// Unpacks non-reference proxy parameters (using the `inner` method) yielding expressions which correspond to the proxied type with conversion errors being
    /// handled by the try `?` operator.
    pub fn unpack_parameter(&self) -> proc_macro2::TokenStream {
        let name = &self.arg_name;
        // if a proxy parameter is to be passed by value we use inner (which requires Clone to be supported)
        LuaArgumentUnwrapper::new(name.clone(), name.span()).visit(&self.arg_type)
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
                            } else {
                                abort!(p, "Invalid Function Type, expected one of: {}", 
                                    FunctionType::iter().map(|ft| ft.to_string()).collect::<Vec<_>>().join(", "))
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
                        .map(|arg| FunctionArgMeta::new_from_fn_arg(&proxied_type_identifier, arg))
                        .collect(),
                    output_meta: FunctionArgMeta::new_from_type(
                            &proxied_type_identifier,
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
                let self_type = LuaTypeConstructorVisitor::new(true,true).visit(&meta.arg_type);

                self_arg = Some(quote_spanned!(meta.span=> , #self_name : & #_mut #self_type));
            }
        }

        let (args, arg_types) = args
            .map(|fn_arg| {
                let _mut = &fn_arg.mutable;
                let name = &fn_arg.arg_name;

                // strip outer refs if the type is a proxy, we cannot have any references in type position
                // we can still have reference semantics since we have a proxy object, however we still pass it by value
                let type_path = LuaTypeConstructorVisitor::new(true,true).visit(&fn_arg.arg_type);
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

        let function_name = self.name;

        // unpack all parameters which need to be unpacked via `.inner` calls, we deal with reference proxies later 
        let unpacked_parameters = self.arg_meta
            .iter()
            .map(FunctionArgMeta::unpack_parameter)
            .collect::<Vec<_>>();

        let param_names = self.arg_meta.iter()
            .map(|arg| &arg.arg_name).collect::<Vec<_>>();

        let unpacked_parameter_declarations = quote_spanned!{self.body.span()=>
            #(let mut #param_names = #unpacked_parameters;)*
        };

        let proxied_method_call = 
        match (self.fn_type.expects_receiver(),&self.body.default){
            (_, Some(body)) => {
                
                let stmts = body.stmts.iter().cloned().map(|mut s| {
                    IdentifierRenamingVisitor{
                        target: "self",
                        replacement: SELF_ALIAS,
                    }.visit_stmt_mut(&mut s);
                    s
                });

                quote_spanned!{body.span()=>
                    (||{
                        #(#stmts)*
                    })()
                }            
            },
            (true, None) => {
                // this removes the first argument taken to be the receiver from the iterator for the next step
                let (first_arg, other_args) = param_names.split_first().unwrap_or_else(|| abort!(self.name,"Proxied functions of the type: {} expect a receiver argument (i.e. self)", self.fn_type));
                // since we removed the receiver we can pass the rest of the parameters here;
                quote_spanned! {self.body.sig.paren_token.span=>
                    #first_arg.#function_name(#(#other_args),*)
                }
            },
            (false, None) => {
                quote_spanned! {self.body.sig.paren_token.span=>
                    #proxied_name::#function_name(#(#param_names),*)
                }
            },
        };

        
        // if the output is also a proxied type, we need to wrap the result in a proxy
        let constructor_wrapped_full_call= {
                let out_ident = format_ident!("{OUT_ALIAS}");

                let constructor_wrapped_expression = ProxyConstructorVisitor::new(out_ident.clone(), self.body.span()).visit(&self.output_meta.arg_type);

                // the type before we wrap it in a proxy
                let proxied_output_type = LuaTypeConstructorVisitor::new(false,false).visit(&self.output_meta.arg_type);
                // the type after we wrap it in a proxy
                let output_type = LuaTypeConstructorVisitor::new(true,false).visit(&self.output_meta.arg_type);
                
                // determine if we need to wrap the output in an Ok() statement
                let last_statement = match &self.output_meta.arg_type {
                    SimpleType::DuoPath(DuoPath{ ident , ..}) if *ident == "Result" => quote_spanned! {self.body.span()=>__output},
                    _ => quote_spanned! {self.body.span()=>Ok(__output)}
                };
                quote_spanned! {self.body.span()=>
                    let #out_ident : #proxied_output_type = #proxied_method_call;
                    let __output : #output_type = #constructor_wrapped_expression;
                    #last_statement
                }    
        };
        
        // for every argument which is a reference, we need a separate sort of call,
        // we cannot use `v.inner()` since this operates over values, we must use `val_mut` or `val` to get a reference to the wrapped
        // structure for the duration of the call 
        let reference_unpacked_constructor_wrapped_full_call = self.arg_meta.iter()
            .fold(constructor_wrapped_full_call, |acc, arg_meta| {
                // if let ArgVariant::Proxy{proxy_type} = &arg_meta.variant_data {
                    if !arg_meta.arg_type.has_outer_ref(){
                        return acc;
                    }

                    let method_call = if arg_meta.arg_type.has_outer_mut_ref() {
                        format_ident!("val_mut")
                    } else {
                        format_ident!("val")
                    };

                    let arg_name = &arg_meta.arg_name;

                    quote_spanned!{self.body.span()=>{
                        #arg_name.#method_call(|#arg_name| {#acc})?
                    }}
                }
            // }
        );

        quote!(   
            #unpacked_parameter_declarations
            #reference_unpacked_constructor_wrapped_full_call
        )

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
}

impl FunctionType {
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
    let tealr = quote!(bevy_mod_scripting_lua::tealr);

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
        let body: proc_macro2::TokenStream = fn_meta.generate_mlua_body(&proxied_name);
        let closure = quote_spanned! {body.span()=>
            |#args| {
                #body
            }
        };

        let tealr_function = format_ident!("{}", fn_meta.fn_type.get_tealr_function());
        let signature = fn_meta
            .fn_type
            .is_meta()
            .then(|| {
                let name = fn_meta.name;
                let valid_meta_methods : HashSet<&'static str> = HashSet::from_iter([
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
                ].into_iter());

                // check is valid meta method if not use custom name
                if valid_meta_methods.contains(name.to_string().as_str()) {
                    quote!(#tealr::mlu::mlua::MetaMethod::#name)
                } else {
                    let std_string = name.to_string();
                    quote!(#tealr::mlu::mlua::MetaMethod::Custom(#std_string.to_string()))
                }
            })
            .unwrap_or_else(|| fn_meta.name.to_string().to_token_stream());

        quote_spanned! {body.span()=>
            #(#method_documentation_calls)*
            methods.#tealr_function(#signature, #closure);
        }
    });


    let a = quote_spanned! {meta.span=>

        #definition

        #tealr_type_implementations

        #[automatically_derived]
        #[allow(unused_parens,unused_braces,unused_mut)]
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
    .into();
    a
}


/// Generates an unwrapping expression which can be used to assign the unwrapped proxy to a variable.
struct LuaArgumentUnwrapper {
    arg_name: Ident,
    span: Span,
}

impl LuaArgumentUnwrapper {
    pub fn new(arg_name: Ident, span: Span) -> Self {
        Self {
            arg_name,
            span,
        }
    }
}

impl VisitSimpleType<proc_macro2::TokenStream> for LuaArgumentUnwrapper {
    fn visit_unit(&mut self, _: bool) -> proc_macro2::TokenStream {
        quote_spanned!(self.span=> ())
    }

    fn visit_proxy_type(&mut self, _: &ProxyType, is_child_of_reference: bool) -> proc_macro2::TokenStream {
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
}

/// Wrapper around the `TypeConstructorVisitor` which generates a syn::Type from a `SimpleType`.
/// This is used to handle special cases such as when encountering an outer `Result<T,E>` where E needs to specifically be converted to an `mlua::Error` on the proxy side
struct LuaTypeConstructorVisitor {
    pub general_visitor: TypeConstructorVisitor,
}

impl LuaTypeConstructorVisitor {
    pub fn new(generate_proxy_type: bool, strip_outer_ref: bool) -> Self {
        Self {
            general_visitor: TypeConstructorVisitor::new(generate_proxy_type,strip_outer_ref),
        }
    }
}

impl VisitSimpleType<Type> for LuaTypeConstructorVisitor {
    fn visit_unit(&mut self, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_unit(is_child_of_reference)
    }

    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_proxy_type(proxy_type, is_child_of_reference)
    }

    fn visit_type(&mut self, _type: &Type, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_type(_type, is_child_of_reference)
    }

    fn visit_unit_path(&mut self, unit_path: &UnitPath, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_unit_path(unit_path, is_child_of_reference)
    }

    fn visit_duo_path(&mut self, duo_path: &DuoPath, is_child_of_reference: bool) -> Type {
        // this will only trigger for top level types, the deeper nesting is handled by the general visitor
        // outer Result<T,E> needs to be converted to Result<T,mlua::Error> when converting to a proxy_type
        let tealr = quote!(bevy_mod_scripting_lua::tealr);

        if duo_path.std_type_ident.is_some_and(|i| i == StdTypeIdent::Result)  && self.general_visitor.generate_proxy_type {
            let ident = &duo_path.ident;
            let lt_token = duo_path.lt_token;
            let gt_token = duo_path.gt_token;
            let left = self.visit_simple_type(&duo_path.left, false);
            parse_quote!(#ident #lt_token #left, #tealr::mlu::mlua::Error #gt_token)
        } else {
            self.general_visitor.visit_duo_path(duo_path, is_child_of_reference)
        }
    }

    fn visit_reference(&mut self, reference: &bevy_mod_scripting_common::input::Reference, is_child_of_reference: bool) -> Type {
        self.general_visitor.visit_reference(reference, is_child_of_reference)
    }

    
}

/// `maps` a simple type recursively, expanding the type into a series of map/iter/etc calls where the leaf types are operating over
/// unwrapped proxy types (the inner types).
/// 
/// requires arg_name to be a valid identifier refering to the name of the variable containing a value with the SimpleType being mapped.
/// The returned token stream will be an expression. 
struct ProxyConstructorVisitor {
    arg_name: Ident,
    span: Span,
}

impl ProxyConstructorVisitor {
    pub fn new(arg_name: Ident, span: Span) -> Self {
        Self {
            arg_name,
            span,
        }
    }
}

impl VisitSimpleType<proc_macro2::TokenStream> for ProxyConstructorVisitor {
    

    fn visit_unit_path(&mut self, unit_path: &UnitPath, is_child_of_reference: bool) -> proc_macro2::TokenStream {
        match unit_path.std_type_ident {
            Some(StdTypeIdent::Option) => {
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;
                quote_spanned!(self.span=>
                    #arg_name.map(|#arg_name| #inner)
                )
            },
            Some(StdTypeIdent::Vec) => { 
                let inner = self.visit_simple_type(&unit_path.inner, false);
                let arg_name = &self.arg_name;
                let function = is_child_of_reference
                    .then(|| quote_spanned!(self.span=>as_ref().map(|#arg_name| #inner)))
                    .unwrap_or_else(|| quote_spanned!(self.span=>map(|#arg_name| #inner)));

                quote_spanned!(self.span=>
                    #arg_name.into_iter().#function.collect::<Vec<_>>()
                )
            }
            Some(unsupported_std_type) => abort!(unit_path.ident, "`{}` is not yet supported", unsupported_std_type),
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
            Some(unsupported_std_type) => abort!(duo_path.ident, "`{}` is not yet supported", unsupported_std_type),
            _ => abort!(duo_path.ident, "Unsupported type") 
        }
        
    }

    fn visit_unit(&mut self, _: bool) -> proc_macro2::TokenStream {
        quote_spanned!(self.span=>
            ()
        )
    }

    fn visit_proxy_type(&mut self, proxy_type: &ProxyType, _: bool) -> proc_macro2::TokenStream {
        let proxy_ident = &proxy_type.proxy_ident;
        let arg_name = & self.arg_name;
        quote_spanned!{self.span=>
            #proxy_ident::new(#arg_name)
        }
    }

    fn visit_type(&mut self, _type: &Type, _: bool) -> proc_macro2::TokenStream {
        self.arg_name.to_token_stream()
    }

}