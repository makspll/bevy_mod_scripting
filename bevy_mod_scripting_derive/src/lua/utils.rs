
/// Utility for parsing enums based on the variant identifier first
macro_rules! impl_parse_enum {
    (
        $input_stream:ident,$parsed_ident:ident:
        $(#[$($meta:meta)*])?
        $vis:vis enum $name:ident {
            $(
                $field:ident $({$($arg_name:ident : $arg_type:ty),*})? => {$($parser:tt)*}
            ),*
            $(,)?
        }

        $(
            impl $_:ident {
                $($other_impls:tt)*
            }  
        )?
    ) => { 
        $(#[$($meta)*])?
        $vis enum $name {
            $(
                $field {
                    ident: Ident,
                    $($($arg_name :  $arg_type),*)?
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
                ts.extend(quote!{#ident});
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
                            Self::$field{$($($arg_name),*,)? ..} => stringify!($field)
                        ),*
                    }
                }

                $(
                pub fn [<is_ $field:snake>](&self) -> bool{
                    if let Self::$field{$($($arg_name),*,)? ..} = self{
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
use syn::Attribute;



pub fn attribute_to_string_lit(attrs: &Attribute) -> TokenStream{
    attrs.tokens.clone()
        .into_iter()
        .skip(1)
        .collect()
}