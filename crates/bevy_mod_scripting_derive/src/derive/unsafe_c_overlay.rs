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
use quote::{ToTokens, format_ident, quote, quote_spanned};
use syn::{
    Attribute, Expr, Ident, LitStr, Meta, Token, Type, braced,
    parse::{Parse, ParseStream},
    parse2,
};

struct Overlay {
    probe: Expr,
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

#[allow(clippy::large_enum_variant, reason = "parse-time only")]
enum Kind {
    /// `#[pad] ident: [u8; len]`
    Pad { len: Expr },
    /// A scalar or a type with an overlay. `probe` is `None` for `#[probe = _]`.
    Data {
        ty: Type,
        probe: Option<Expr>,
        opaque: bool,
    },
}

impl Parse for Overlay {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mut probe, mut c_name) = (None, None);
        for attr in input.call(Attribute::parse_outer)? {
            match &attr.meta {
                Meta::List(_) if attr.path().is_ident("probe") && probe.is_none() => {
                    probe = Some(attr.parse_args::<Expr>()?);
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
                        "expected a single `#[probe(<const constructor>)]` and optionally `#[c_name = \"Name\"]`",
                    ));
                }
            }
        }
        let target: Type = input.parse()?;
        input.parse::<Token![as]>()?;
        let name: Ident = input.parse()?;
        let probe = probe.ok_or_else(|| {
            syn::Error::new_spanned(&target, "`#[probe(<const constructor>)]` is required")
        })?;
        let c_name = c_name.unwrap_or_else(|| LitStr::new(&name.to_string(), name.span()));
        let content;
        braced!(content in input);
        let fields = content
            .parse_terminated(Field::parse, Token![,])?
            .into_iter()
            .collect();
        Ok(Self {
            probe,
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
                    probe = Some(match &nv.value {
                        Expr::Infer(_) => None,
                        value => Some(value.clone()),
                    })
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "expected `#[probe = <value>]`, `#[probe = _]`, `#[opaque]` or `#[pad]`",
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
            let probe = probe.ok_or_else(|| {
                syn::Error::new_spanned(
                    &ident,
                    format!("missing probe for `{ident}`, use `#[probe = _]` to skip"),
                )
            })?;
            Kind::Data { ty, probe, opaque }
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

fn pretty(tokens: impl ToTokens) -> String {
    tokens.to_token_stream().to_string().replace(' ', "")
}

impl Overlay {
    fn expand(&self) -> TokenStream {
        let ffi = quote!(::bevy_mod_scripting_ffi);
        let p = quote!(#ffi::__private);
        let Self {
            probe,
            target,
            name,
            fields,
            ..
        } = self;
        let target_str = pretty(target);
        let field_ty = |ty: &Type| quote!(<#ty as #ffi::FfiField>::Overlay);
        let mask_of = |ty: &Type| quote!(<#ty as #ffi::FfiField>::INIT_MASK);

        let struct_fields = fields.iter().map(|Field { ident, kind }| match kind {
            Kind::Pad { len } => quote!(#ident: [#p::MaybeUninit<u8>; #len]),
            Kind::Data { ty, .. } => {
                let doc = lit(format!("`{ident}` of `{target_str}`."));
                let ty = field_ty(ty);
                quote!(#[doc = #doc] pub #ident: #ty)
            }
        });

        let mask_steps = fields.iter().map(|Field { kind, .. }| match kind {
            Kind::Pad { len } => quote!(at += #len;),
            Kind::Data { ty, .. } => {
                let mask = mask_of(ty);
                quote!(at = #p::mask_field(&mut mask, at, #mask);)
            }
        });

        let layout_steps = fields.iter().map(|Field { ident, kind }| match kind {
            Kind::Pad { len } => quote_spanned!(ident.span()=> at += #len;),
            Kind::Data { ty, .. } => {
                let ty = field_ty(ty);
                let msg = lit(format!(
                    "`{name}::{ident}` is misaligned, declare the padding before it with `#[pad]`"
                ));
                quote_spanned! {ident.span()=>
                    ::core::assert!(at % #p::align_of::<#ty>() == 0, #msg);
                    at += #p::size_of::<#ty>();
                }
            }
        });
        let size_msg = lit(format!(
            "the fields and pads of `{name}` don't add up to the size of `{target_str}`"
        ));
        let align_msg = lit(format!(
            "`{name}` has a different alignment than `{target_str}`"
        ));

        let type_checks = fields
            .iter()
            .filter_map(|Field { ident, kind }| match kind {
                Kind::Data {
                    ty, opaque: false, ..
                } => Some(quote_spanned!(ident.span()=> let _: &#ty = &value.#ident;)),
                _ => None,
            });

        // (ident, type, probe value const) of each probed field
        let probed: Vec<_> = fields
            .iter()
            .filter_map(|Field { ident, kind }| match kind {
                Kind::Data {
                    ty,
                    probe: Some(value),
                    ..
                } => Some((
                    ident,
                    ty,
                    value,
                    format_ident!("__PROBE_{}", ident.to_string().to_uppercase()),
                )),
                _ => None,
            })
            .collect();
        let probe_values = probed.iter().map(|(ident, ty, value, konst)| {
            let mask = mask_of(ty);
            let msg = lit(format!(
                "probe value for `{ident}` has a zero byte, use a value with every byte non-zero"
            ));
            quote_spanned! {ident.span()=>
                const #konst: #ty = #value;
                ::core::assert!(#p::all_nonzero(#p::bytes(&#konst), #mask), #msg);
            }
        });
        let (p_ref, mask_of_ref) = (&p, &mask_of);
        let distinct = probed.iter().enumerate().flat_map(|(i, (a, a_ty, _, a_const))| {
            probed[i + 1..].iter().map(move |(b, b_ty, _, b_const)| {
                let (p, mask_of) = (p_ref, mask_of_ref);
                let (a_mask, b_mask) = (mask_of(a_ty), mask_of(b_ty));
                let msg = lit(format!("probe values for `{a}` and `{b}` are equal"));
                quote_spanned! {b.span()=>
                    ::core::assert!(
                        !#p::same_value(#p::bytes(&#a_const), #a_mask, #p::bytes(&#b_const), #b_mask),
                        #msg
                    );
                }
            })
        });
        let probe_checks = probed.iter().map(|(ident, ty, value, konst)| {
            let mask = mask_of(ty);
            let msg = lit(format!(
                "probe failed: `{name}::{ident}` expected `{}`",
                pretty(value)
            ));
            quote_spanned! {ident.span()=>
                ::core::assert!(
                    #p::masked_eq(#p::bytes(&__PROBED.#ident), #p::bytes(&#konst), #mask),
                    #msg
                );
            }
        });

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
                    Kind::Pad { len } => vec![
                        quote!(" uint8_t "),
                        quote!(#field),
                        quote!("["),
                        quote!((#len) as usize),
                        quote!("];"),
                    ],
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
                const INIT_MASK: &'static [bool] = &{
                    #[allow(unused_assignments)]
                    const fn init_mask() -> [bool; #p::size_of::<#target>()] {
                        let mut mask = [false; #p::size_of::<#target>()];
                        let mut at = 0usize;
                        #(#mask_steps)*
                        mask
                    }
                    init_mask()
                };
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

            const _: () = {
                const LAYOUT: () = {
                    let mut at = 0usize;
                    #(#layout_steps)*
                    ::core::assert!(at == #p::size_of::<#target>(), #size_msg);
                    ::core::assert!(#p::size_of::<#name>() == #p::size_of::<#target>(), #size_msg);
                    ::core::assert!(#p::align_of::<#name>() == #p::align_of::<#target>(), #align_msg);
                };

                #[allow(dead_code)]
                fn field_types(value: &#target) {
                    #(#type_checks)*
                }

                union Cast {
                    real: #p::ManuallyDrop<#target>,
                    overlay: #p::ManuallyDrop<#name>,
                }

                const PROBES: () = {
                    #[allow(clippy::let_unit_value)]
                    let () = LAYOUT;
                    #(#probe_values)*
                    #(#distinct)*
                    // SAFETY: the layout check above passed, the overlay covers exactly the bytes of the real type
                    const __PROBED: #name = unsafe {
                        #p::ManuallyDrop::into_inner(Cast { real: #p::ManuallyDrop::new(#probe) }.overlay)
                    };
                    #(#probe_checks)*
                };

                PROBES
            };
        }
    }
}
