
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
                    ident: Ident,
                    $(
                        $(
                            $arg_name :  $arg_type
                        ),*
                    )?
                }
            ),*
            
        }

        impl Parse for $name {
            fn parse($input_stream: ParseStream) -> Result<Self> {
                let $parsed_ident : Ident = $input_stream.parse()?;

                match $parsed_ident.to_string().as_str() {
                    $(
                     stringify!($field) => {$($parser)*},
                    )*
                    _ => Err(Error::new_spanned($parsed_ident, format!("Invalid derive flag, try one of [{}]",Self::variants()))),
                }
            }
        }

        impl ToTokens for $name {
            fn to_tokens(&self, ts: &mut proc_macro2::TokenStream) { 
                let ident = Ident::new(self.to_str(),Span::call_site());
                ts.extend(quote::quote_spanned!{syn::spanned::Spanned::span(ts)=> #ident});
            }
        }

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

pub(crate) use impl_parse_enum;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, TypePath, Type};



pub fn attribute_to_string_lit(attrs: &Attribute) -> TokenStream{
    attrs.tokens.clone()
        .into_iter()
        .skip(1)
        .collect()
}


/// Converts the given ToTokens into token stream, stringifies it and removes whitespace
pub fn stringify_token_group<T : ToTokens>(t : &T) -> String{
        let mut k = t.into_token_stream().to_string();
        k.retain(|c| !c.is_whitespace());
        k
}



/// Converts simple type to base string (i.e. one which has a single type identifier)
pub fn type_base_string(t : &Type) -> Option<String> {
    match t {
        Type::Paren(v) => type_base_string(&v.elem),
        Type::Path(p) => Some(p.path.segments.last()?.ident.to_string()),
        Type::Ptr(p) => type_base_string(&p.elem),
        Type::Reference(r) => type_base_string(&r.elem),
        Type::Slice(v) => type_base_string(&v.elem),
        _ => None,
    }
}
