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
