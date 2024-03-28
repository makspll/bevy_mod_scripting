/// Utility for parsing enums based on the variant identifier first
macro_rules! impl_parse_enum {
    (
        $input_stream:ident,$parsed_ident:ident:
        $($(#[$($meta:meta)*])+)?
        $vis:vis enum $name:ident {
            $(
                $($(#[$($meta_inner:meta)*])+)?
                $field:ident $(
                    {
                        $(
                            $arg_name:ident : $arg_type:ty
                        ),*
                    }

                )? => {$($parser:tt)*}
            ),*
            $(,)?
        }

        $(
            impl $_:ident {
                $($other_impls:tt)*
            }
        )?
    ) => {
        $($(#[$($meta)*])+)?
        $vis enum $name {
            $(
                $($(#[$($meta_inner)*])+)?
                $field {
                    /// The identifier of the enum variant
                    ident: syn::Ident,
                    $(
                        $(
                            $arg_name :  $arg_type
                        ),*
                    )?
                }
            ),*

        }
        #[allow(clippy::mixed_read_write_in_expression)]
        impl Parse for $name {
            fn parse($input_stream: ParseStream) -> Result<Self,syn::Error> {
                let $parsed_ident : syn::Ident = $input_stream.parse()?;

                match $parsed_ident.to_string().as_str() {
                    $(
                     stringify!($field) => {$($parser)*},
                    )*
                    _ => Err(syn::Error::new_spanned($parsed_ident, format!("Invalid derive flag, try one of [{}]",Self::variants()))),
                }
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, ts: &mut proc_macro2::TokenStream) {
                let ident = syn::Ident::new(self.to_str(),Span::call_site());
                ts.extend(quote::quote_spanned!{syn::spanned::Spanned::span(ts)=> #ident});
            }
        }

        #[allow(unused_variables)]
        impl $name {
            paste::paste!{
                $($($other_impls)*)?

                pub fn variants() -> &'static str{
                    concat!($(
                        stringify!($field),","
                    ),*
                    )
                }

                pub fn to_str(&self) -> &'static str {
                    match self {
                        $(
                            Self::$field{ident,$($($arg_name),*)?} => stringify!($field)
                        ),*
                    }
                }

                $(
                pub fn [<is_ $field:snake>](&self) -> bool{
                    if let Self::$field{ident,$($($arg_name),*)?} = self{
                        return true
                    } else {
                        return false
                    }
                }
                )*
            }

        }
    };
}

use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    Attribute, Path, PathArguments, PathSegment, Type, TypePath,
};

pub fn doc_attribute_to_string_lit(attrs: &Attribute) -> Option<TokenStream> {
    attrs
        .meta
        .require_name_value()
        .map(|v| v.value.to_token_stream())
        .ok()
}

pub fn ident_to_type_path(ident: Ident) -> TypePath {
    TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                ident,
                arguments: PathArguments::None,
            }]
            .into_iter()
            .collect(),
        },
    }
}
/// Converts the given ToTokens into token stream, stringifies it and removes whitespace
pub fn stringify_token_group<T: ToTokens>(t: &T) -> String {
    let mut k = t.into_token_stream().to_string();
    k.retain(|c| !c.is_whitespace());
    k
}

/// Converts simple type to base string (i.e. one which has a single type identifier)
pub fn type_base_string(t: &Type) -> Option<String> {
    match t {
        Type::Paren(v) => type_base_string(&v.elem),
        Type::Path(p) => Some(p.path.segments.last()?.ident.to_string()),
        Type::Ptr(p) => type_base_string(&p.elem),
        Type::Reference(r) => type_base_string(&r.elem),
        Type::Slice(v) => type_base_string(&v.elem),
        _ => None,
    }
}

#[derive(Default, Debug, Clone)]
pub struct EmptyToken;

impl Parse for EmptyToken {
    fn parse(_: ParseStream) -> Result<Self, syn::Error> {
        Ok(Self)
    }
}
impl ToTokens for EmptyToken {
    fn to_tokens(&self, _: &mut TokenStream) {}
}
