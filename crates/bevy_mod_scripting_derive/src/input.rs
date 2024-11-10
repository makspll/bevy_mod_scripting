use darling::{util::Flag, FromDeriveInput, FromMeta};
use proc_macro2::Ident;
use std::ops::{Deref, DerefMut};
use syn::{spanned::Spanned, visit_mut::VisitMut, Attribute, Field, TraitItemFn, Variant};

#[derive(FromMeta)]
pub struct BMSCorePath(pub syn::Path);

impl Default for BMSCorePath {
    fn default() -> Self {
        Self(syn::parse_quote!(bevy_mod_scripting::core))
    }
}

#[derive(FromMeta)]
pub struct BMSLuaPath(pub syn::Path);

impl Default for BMSLuaPath {
    fn default() -> Self {
        Self(syn::parse_quote!(bevy_mod_scripting::lua))
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(proxy), forward_attrs(allow, doc, cfg))]
pub struct ProxyInput {
    /// The name of the type for which we are generating a proxy (target type)
    pub ident: syn::Ident,
    /// The visibility of the target type
    pub vis: syn::Visibility,
    /// The generics on the target type
    pub generics: syn::Generics,
    /// The attributes on the target type
    pub attrs: Vec<Attribute>,

    /// The path to the type for which we are generating a proxy if it's a foreign type
    pub remote: Option<syn::Path>,

    /// if provided will call the function at this path to get the world callback access. Normally this is retrieved using a global variable.
    pub get_world_callback_access_fn: Option<syn::Path>,

    /// If set will use the given path as the type for the proxy instead of generating a new one
    /// Only used for the special world proxies, probably not useful for anything else, the macro assumes we have an inner ReflectReference in the wrapper
    pub proxy_as_type: Option<syn::Path>,

    /// The path to the bevy_mod_scripting_core crate
    #[darling(default)]
    pub bms_core_path: BMSCorePath,
    /// The path to the bevy_mod_scripting_lua crate
    #[darling(default)]
    pub bms_lua_path: BMSLuaPath,

    /// The name to use for the proxy type, if not provided the language derive macro
    /// will generate one using a standard prefix.
    #[darling(rename = "name")]
    pub proxy_name: Option<Ident>,

    /// The body of the type for which we are generating a proxy
    pub data: darling::ast::Data<Variant, Field>,

    /// A list of multi-lang function definitions to be generated on the proxy type
    #[darling(default)]
    pub functions: TraitItemFnsWrapper,
}

#[derive(Default)]
pub struct TraitItemFnsWrapper(pub Vec<TraitItemFn>);

impl FromMeta for TraitItemFnsWrapper {
    fn from_string(value: &str) -> darling::Result<Self> {
        let token_stream: proc_macro2::TokenStream = value.parse().map_err(syn::Error::from)?;
        let trait_items_vec = vec![syn::parse2(token_stream)?];
        Ok(TraitItemFnsWrapper(trait_items_vec))
    }

    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        Ok(TraitItemFnsWrapper(
            items
                .iter()
                .map(Self::from_nested_meta)
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .flat_map(|v| v.0.into_iter())
                .collect::<Vec<_>>(),
        ))
    }
}

impl Deref for TraitItemFnsWrapper {
    type Target = Vec<TraitItemFn>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for TraitItemFnsWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Replaces every occurence of an identifier with
/// the given string while preserving the original span
pub struct IdentifierRenamingVisitor<'a> {
    pub target: &'a str,
    pub replacement: &'a str,
}

impl VisitMut for IdentifierRenamingVisitor<'_> {
    fn visit_ident_mut(&mut self, i: &mut Ident) {
        if *i == self.target {
            *i = Ident::new(self.replacement, i.span());
        }
    }
}
