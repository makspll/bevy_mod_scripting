//! `unsafe_c_overlay!`: a `#[repr(C)]` overlay over a foreign type, checked at compile time.
//!
//! ```text
//! #[probe(<const constructor of T>)]
//! T as Overlay {
//!     #[probe = <value>] field: Type,        // value must be what the constructor stored there
//!     #[opaque] #[probe = _] hidden: Type,   // `_` skips the probe, `#[opaque]` skips the type check
//!     #[pad] _p0: [u8; N],                   // padding, never read
//! }
//! ```
//!
//! Generates the overlay struct, `FfiField for T`, and const assertions for layout, types and probes.

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    Attribute, Expr, Ident, LitStr, Meta, Token, Type, braced,
    parse::{Parse, ParseStream},
    parse2,
};

struct Overlay {
    /// The `#[probe(..)]` constructors, in order
    probes: Vec<Expr>,
    /// The C type name, `#[c_name = ".."]` or the overlay's ident
    c_name: LitStr,
    target: Type,
    name: Ident,
    fields: Vec<Field>,
}

struct Field {
    ident: Ident,
    kind: Kind,
}

/// A field's probe values as written, resolved against the number of type-level probes after parsing.
#[allow(clippy::large_enum_variant, reason = "parse-time only")]
enum FieldProbe {
    /// `#[probe = v]` or `#[probe = _]`
    Single(Option<Expr>),
    /// `#[probe(v0, v1, ..)]`, one value (or `_`) per type-level probe
    List(Vec<Option<Expr>>, Span),
}

/// `_` means "not probed".
fn probe_value(value: &Expr) -> Option<Expr> {
    match value {
        Expr::Infer(_) => None,
        value => Some(value.clone()),
    }
}

#[allow(clippy::large_enum_variant, reason = "parse-time only")]
enum Kind {
    /// `#[pad] ident: [u8; len]`
    Pad { len: Expr },
    /// A scalar or a type with an overlay.
    Data {
        ty: Type,
        /// as written, resolved into `probes` once the number of type-level probes is known
        written: FieldProbe,
        /// one expected value per type-level probe, `None` where the field isn't probed
        probes: Vec<Option<Expr>>,
        opaque: bool,
    },
}

impl Parse for Overlay {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mut probes, mut c_name) = (Vec::new(), None);
        for attr in input.call(Attribute::parse_outer)? {
            match &attr.meta {
                Meta::List(_) if attr.path().is_ident("probe") => {
                    probes.push(attr.parse_args::<Expr>()?);
                }
                Meta::NameValue(nv) if nv.path.is_ident("c_name") && c_name.is_none() => {
                    let lit = match &nv.value {
                        Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(lit),
                            ..
                        }) => lit.clone(),
                        value => {
                            return Err(syn::Error::new_spanned(
                                value,
                                "expected a string, `#[c_name = \"Name\"]`",
                            ));
                        }
                    };
                    let value = lit.value();
                    let mut chars = value.chars();
                    let valid = chars
                        .next()
                        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
                        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
                    if !valid {
                        return Err(syn::Error::new_spanned(lit, "not a C identifier"));
                    }
                    c_name = Some(lit);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "expected `#[probe(<const constructor>)]` (one or more) and optionally `#[c_name = \"Name\"]`",
                    ));
                }
            }
        }
        let target: Type = input.parse()?;
        input.parse::<Token![as]>()?;
        let name: Ident = input.parse()?;
        if probes.is_empty() {
            return Err(syn::Error::new_spanned(
                &target,
                "`#[probe(<const constructor>)]` is required",
            ));
        }
        let c_name = c_name.unwrap_or_else(|| LitStr::new(&name.to_string(), name.span()));
        let content;
        braced!(content in input);
        let mut fields: Vec<Field> = content
            .parse_terminated(Field::parse, Token![,])?
            .into_iter()
            .collect();

        // one expected value per type-level probe
        for Field { ident, kind } in &mut fields {
            let Kind::Data {
                written,
                probes: resolved,
                ..
            } = kind
            else {
                continue;
            };
            *resolved = match written {
                FieldProbe::Single(None) => vec![None; probes.len()],
                FieldProbe::Single(Some(value)) if probes.len() == 1 => vec![Some(value.clone())],
                FieldProbe::Single(Some(_)) => {
                    return Err(syn::Error::new_spanned(
                        &*ident,
                        format!(
                            "`{ident}` needs one value per probe, `#[probe(v0, .., v{})]`",
                            probes.len() - 1
                        ),
                    ));
                }
                FieldProbe::List(values, _) if values.len() == probes.len() => values.clone(),
                FieldProbe::List(values, span) => {
                    return Err(syn::Error::new(
                        *span,
                        format!(
                            "`{ident}` has {} probe values, but there are {} probes",
                            values.len(),
                            probes.len()
                        ),
                    ));
                }
            };
        }

        Ok(Self {
            probes,
            c_name,
            target,
            name,
            fields,
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;

        let (mut pad, mut opaque, mut probe) = (false, false, None);
        for attr in attrs {
            match &attr.meta {
                Meta::Path(p) if p.is_ident("pad") => pad = true,
                Meta::Path(p) if p.is_ident("opaque") => opaque = true,
                Meta::NameValue(nv) if nv.path.is_ident("probe") => {
                    probe = Some(FieldProbe::Single(probe_value(&nv.value)));
                }
                Meta::List(_) if attr.path().is_ident("probe") => {
                    let values = attr.parse_args_with(
                        syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated,
                    )?;
                    probe = Some(FieldProbe::List(
                        values.iter().map(probe_value).collect(),
                        syn::spanned::Spanned::span(&attr),
                    ));
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "expected `#[probe = <value>]`, `#[probe(<value per probe>, ..)]`, `#[probe = _]`, `#[opaque]` or `#[pad]`",
                    ));
                }
            }
        }

        let kind = if pad {
            if probe.is_some() || opaque {
                return Err(syn::Error::new_spanned(
                    &ident,
                    "`#[pad]` fields take no other attributes",
                ));
            }
            match &ty {
                Type::Array(array) if array.elem.to_token_stream().to_string() == "u8" => {
                    Kind::Pad {
                        len: array.len.clone(),
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        &ty,
                        "`#[pad]` fields must be `[u8; N]`",
                    ));
                }
            }
        } else {
            let written = probe.ok_or_else(|| {
                syn::Error::new_spanned(
                    &ident,
                    format!("missing probe for `{ident}`, use `#[probe = _]` to skip"),
                )
            })?;
            Kind::Data {
                ty,
                written,
                probes: Vec::new(),
                opaque,
            }
        };
        Ok(Self { ident, kind })
    }
}

pub fn unsafe_c_overlay(input: TokenStream) -> TokenStream {
    match parse2::<Overlay>(input) {
        Ok(overlay) => overlay.expand(),
        Err(err) => err.to_compile_error(),
    }
}

fn lit(value: String) -> LitStr {
    LitStr::new(&value, Span::call_site())
}

/// Errors point at the field, names resolve at the call site (fields can come from a `macro_rules!` expansion, whose
/// hygiene would hide the generated locals).
fn at_field(span: Span) -> Span {
    Span::call_site().located_at(span)
}

fn pretty(tokens: impl ToTokens) -> String {
    tokens.to_token_stream().to_string().replace(' ', "")
}

impl Overlay {
    fn expand(&self) -> TokenStream {
        let ffi = quote!(::bevy_mod_scripting_ffi);
        let p = quote!(#ffi::__private);
        let Self {
            target,
            name,
            fields,
            ..
        } = self;
        let target_str = pretty(target);
        let field_ty = |ty: &Type| quote!(<#ty as #ffi::FfiField>::Overlay);

        let struct_fields = fields.iter().map(|Field { ident, kind }| match kind {
            Kind::Pad { len } => quote!(#ident: [#p::MaybeUninit<u8>; #len]),
            Kind::Data { ty, .. } => {
                let doc = lit(format!("`{ident}` of `{target_str}`."));
                let ty = field_ty(ty);
                quote!(#[doc = #doc] pub #ident: #ty)
            }
        });

        // the declared layout, checked by `Layout::check` in the ffi crate
        let name_str = lit(name.to_string());
        let target_lit = lit(target_str.clone());
        let layout_fields = fields.iter().map(|Field { ident, kind }| {
            let field = lit(ident.to_string());
            match kind {
                Kind::Pad { len } => quote!(#p::Field::pad(#field, (#len) as usize)),
                Kind::Data { ty, .. } => {
                    quote!(#p::Field::data::<#ty>(#field, #p::offset_of!(#name, #ident)))
                }
            }
        });

        let type_checks = fields
            .iter()
            .filter_map(|Field { ident, kind }| match kind {
                Kind::Data {
                    ty, opaque: false, ..
                } => Some(quote_spanned!(at_field(ident.span())=> let _: &#ty = &value.#ident;)),
                _ => None,
            });

        let probes = self.probes.iter().enumerate().map(|(i, probe)| {
            let expected = fields.iter().map(|Field { kind, .. }| match kind {
                Kind::Data { ty, probes, .. } => match &probes[i] {
                    Some(value) => {
                        let text = lit(pretty(value));
                        quote!(::core::option::Option::Some(#p::Expected {
                            bytes: #p::bytes(&#p::ManuallyDrop::<#ty>::new(#value)),
                            text: #text,
                        }))
                    }
                    None => quote!(::core::option::Option::None),
                },
                Kind::Pad { .. } => quote!(::core::option::Option::None),
            });
            quote! {
                #p::Probe {
                    image: #p::bytes(&#p::ManuallyDrop::<#target>::new(#probe)),
                    expected: &[#(#expected),*],
                }
            }
        });
        let check = quote_spanned! {at_field(name.span())=>
            #name::__LAYOUT.check::<{ #p::size_of::<#target>() }>(&[#(#probes),*]);
        };

        let overlay_doc = lit(format!(
            "C overlay of `{target_str}`, generated and checked by `unsafe_c_overlay!`."
        ));

        // C declarations, concatenated at compile time from the field types' C names and the Rust layout
        let c_name = &self.c_name;
        let c_name_str = c_name.value();
        let body: Vec<TokenStream> = fields
            .iter()
            .flat_map(|Field { ident, kind }| {
                let field = lit(ident.to_string());
                match kind {
                    // computed pads can be empty (e.g. depending on glam's `scalar-math`), C has no empty arrays
                    Kind::Pad { len } => vec![quote! {
                        if (#len) as usize == 0 {
                            ""
                        } else {
                            #p::concatcp!(" uint8_t ", #field, "[", (#len) as usize, "];")
                        }
                    }],
                    Kind::Data { ty, .. } => vec![
                        quote!(" "),
                        quote!(<#ty as #ffi::FfiField>::C_DECL.name),
                        quote!(" "),
                        quote!(#field),
                        quote!(";"),
                    ],
                }
            })
            .collect();
        let offset_asserts: Vec<TokenStream> = fields
            .iter()
            .filter(|f| matches!(f.kind, Kind::Data { .. }))
            .flat_map(|Field { ident, .. }| {
                let open = lit(format!(
                    "_Static_assert(offsetof({c_name_str}, {ident}) == "
                ));
                let close = lit(format!(", \"{c_name_str}.{ident} offset\");\n"));
                vec![
                    quote!(#open),
                    quote!(#p::offset_of!(#name, #ident)),
                    quote!(#close),
                ]
            })
            .collect();
        let size_open = lit(format!("_Static_assert(sizeof({c_name_str}) == "));
        let size_close = lit(format!(", \"{c_name_str} size\");\n"));
        let align_open = lit(format!("_Static_assert(_Alignof({c_name_str}) == "));
        let align_close = lit(format!(", \"{c_name_str} align\");\n"));
        let deps = fields.iter().filter_map(|Field { kind, .. }| match kind {
            Kind::Data { ty, .. } => {
                Some(quote!((|| <#ty as #ffi::FfiField>::C_DECL) as fn() -> #ffi::CDecl))
            }
            Kind::Pad { .. } => None,
        });

        quote! {
            #[doc = #overlay_doc]
            #[repr(C)]
            #[derive(Clone, Copy)]
            #[allow(dead_code)]
            pub struct #name {
                // `[T; 0]` is zero-sized with the alignment of `T`, and a `#[repr(C)]` struct has the alignment of its
                // most-aligned field, so this gives the overlay exactly the alignment of the real type without moving
                // any field (both guaranteed by the Rust Reference):
                // https://doc.rust-lang.org/reference/type-layout.html#array-layout
                // https://doc.rust-lang.org/reference/type-layout.html#reprc-structs
                #[doc = "Zero-sized, gives the overlay the alignment of the real type, see \
                    <https://doc.rust-lang.org/reference/type-layout.html#array-layout> and \
                    <https://doc.rust-lang.org/reference/type-layout.html#reprc-structs>."]
                __align: [#target; 0],
                #(#struct_fields,)*
            }

            // SAFETY: promised by the `unsafe_c_overlay!` invocation, layout and probe checked below
            unsafe impl #ffi::FfiField for #target {
                type Overlay = #name;
                const INIT_MASK: &'static [bool] =
                    &#name::__LAYOUT.init_mask::<{ #p::size_of::<#target>() }>();
                const C_DECL: #ffi::CDecl = #ffi::CDecl {
                    name: #c_name,
                    cdef: #p::concatcp!(
                        "typedef struct __attribute__((aligned(", #p::align_of::<#name>(), "))) ", #c_name, " {",
                        #(#body,)*
                        " } ", #c_name, ";\n"
                    ),
                    header: #p::concatcp!(
                        "typedef struct BMS_ALIGN(", #p::align_of::<#name>(), ") ", #c_name, " {",
                        #(#body,)*
                        " } ", #c_name, ";\n",
                        #size_open, #p::size_of::<#name>(), #size_close,
                        #align_open, #p::align_of::<#name>(), #align_close,
                        #(#offset_asserts),*
                    ),
                    deps: &[#(#deps),*],
                };
            }

            impl #name {
                /// The declared layout, see `bevy_mod_scripting_ffi::layout`.
                #[doc(hidden)]
                pub const __LAYOUT: #p::Layout<'static> = #p::Layout {
                    overlay: #name_str,
                    target: #target_lit,
                    size: #p::size_of::<#target>(),
                    align: #p::align_of::<#target>(),
                    overlay_size: #p::size_of::<Self>(),
                    overlay_align: #p::align_of::<Self>(),
                    fields: &[#(#layout_fields),*],
                };
            }

            const _: () = {
                #[allow(dead_code)]
                fn field_types(value: &#target) {
                    #(#type_checks)*
                }

                #check
            };
        }
    }
}
