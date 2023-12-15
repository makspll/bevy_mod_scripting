// use bevy_mod_scripting_common::derive_data::{EnumData, ProxyFlag, StructData, StructField};
// use proc_macro2::{Ident, TokenStream};
// use quote::{format_ident, quote};
// use syn::Type;

// /// Replaces any `Self` occurences with the given ident
// /// # Example
// /// - `&[&Self]` -> `&[&MyType]`
// /// - `fn(&mut self)` -> `fn(&mut MyType)`
// pub fn resolve_self(type_: &Type, self_type: &Ident) -> Type {
//     match type_ {
//         Type::Array(old) => Type::Array(syn::TypeArray {
//             elem: Box::new(resolve_self(old.elem.as_ref(), self_type)),
//             ..old.clone()
//         }),
//         Type::BareFn(old) => Type::BareFn(syn::TypeBareFn {
//             inputs: old
//                 .inputs
//                 .iter()
//                 .map(|arg| syn::BareFnArg {
//                     ty: resolve_self(&arg.ty, self_type),
//                     ..arg.clone()
//                 })
//                 .collect(),
//             output: match old.output {
//                 syn::ReturnType::Default => syn::ReturnType::Default,
//                 syn::ReturnType::Type(rarrow, t) => {
//                     syn::ReturnType::Type(rarrow, Box::new(resolve_self(t.as_ref(), self_type)))
//                 }
//             },
//             ..old.clone()
//         }),
//         Type::Group(old) => Type::Group(syn::TypeGroup {
//             elem: Box::new(resolve_self(old.elem.as_ref(), self_type)),
//             ..old.clone()
//         }),
//         Type::Paren(old) => Type::Paren(syn::TypeParen {
//             elem: Box::new(resolve_self(old.elem.as_ref(), self_type)),
//             ..old.clone()
//         }),
//         Type::Path(old) => Type::Path(syn::TypePath {
//             qself: old.qself.map(|v| syn::QSelf {
//                 ty: Box::new(resolve_self(v.ty.as_ref(), self_type)),
//                 ..v.clone()
//             }),
//             path: syn::Path {
//                 segments: old.path.segments.iter().map(|seg| syn::PathSegment {
//                     ident: todo!(),
//                     arguments: todo!(),
//                 }),
//                 ..old.path.clone()
//             },
//         }),
//         Type::Ptr(_) => todo!(),
//         Type::Reference(_) => todo!(),
//         Type::Slice(_) => todo!(),
//         Type::TraitObject(_) => todo!(),
//         Type::Tuple(_) => todo!(),
//         Type::Verbatim(_) => todo!(),
//         old => old.clone(),
//     }
// }

// /// Produces the required statement to produce lua fields in the context of the add_fields method with a single `fields` argument:
// ///
// /// # Example
// /// ```rust,ignore
// /// fn add_fields<'lua, T: TealDataFields<'lua, Self>>(fields: &mut T) {
// ///    // generates statements which go in here terminated by a semicolon
// /// }
// ///
// /// ```
// pub fn make_field_stmt(field: &StructField) -> TokenStream {
//     if let syn::Type::Reference(reference) = field.data.ty {
//         return syn::Error::new_spanned(reference, "Reference fields are not supported")
//             .to_compile_error();
//     };
// }

// pub fn impl_struct(struct_data: StructData) -> TokenStream {
//     let proxy_name = format_ident!("Lua{}", struct_data.meta.base_type_name);
//     let base_type_name = struct_data.meta.base_type_name;

//     // make proxy definition and basic impls
//     let mut definition;

//     if struct_data.meta.proxy_flags.contains(&ProxyFlag::Clone) {
//         definition = quote! {
//             bevy_script_api::make_script_wrapper!(#base_type_name as #proxy_name with Clone);
//         }
//     } else {
//         definition = quote! {
//             bevy_script_api::make_script_wrapper!(#base_type_name as #proxy_name);
//         }
//     };

//     definition = quote! {
//         #definition
//         bevy_script_api::impl_tealr_type!(#proxy_name);
//     };

//     if struct_data.meta.proxy_flags.contains(&ProxyFlag::Debug) {
//         definition = quote! {
//             #definition
//             impl std::fmt::Debug for #proxy_name {
//                 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//                     self.val(|s| s.fmt(f)).unwrap_or_else(|_| f.write_str("Error while retrieving reference in `std::fmt::Debug`."))                    }
//             }
//         }
//     }

//     let tealr_root = quote!(bevy_mod_scripting_lua::tealr);
//     let type_documentation = struct_data.meta.docstrings;

//     let fields = struct_data.fields.iter().map(make_field_stmt);

//     // make the TealData impl
//     definition = quote! {
//         #definition

//         #[allow(unused_parens,unreachable_patterns,unused_variables,clippy::all)]
//         impl #tealr_root::mlu::TealData for #proxy_name {
//             fn add_methods<'lua, T: #tealr_root::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
//                 #(methods.document_type(#type_documentation));*;

//             }

//             fn add_fields<'lua, T: #tealr_root::mlu::TealDataFields<'lua, Self>>(fields: &mut T) {
//                 #(#fields)*
//             }
//         }

//         impl bevy_script_api::lua::LuaProxyable for #base_type_name {
//             fn ref_to_lua<'lua>(self_ : bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr_root::mlu::mlua::Lua) -> #tealr_root::mlu::mlua::Result<#tealr_root::mlu::mlua::Value<'lua>> {
//                 <#proxy_name as #tealr_root::mlu::mlua::IntoLua>::into_lua(#proxy_name::new_ref(self_),lua)
//             }

//             fn apply_lua<'lua>(self_ : &mut bevy_script_api::script_ref::ScriptRef, lua: &'lua #tealr_root::mlu::mlua::Lua, new_val: #tealr_root::mlu::mlua::Value<'lua>) -> #tealr_root::mlu::mlua::Result<()> {
//                 if let #tealr_root::mlu::mlua::Value::UserData(v) = new_val {
//                     let other = v.borrow::<#proxy_name>()?;
//                     let other = &other;

//                     other.apply_self_to_base(self_)?;
//                     Ok(())
//                 } else {
//                     Err(#tealr_root::mlu::mlua::Error::RuntimeError(
//                         "Error in assigning to custom user data".to_owned(),
//                     ))
//                 }
//             }
//         }

//         impl bevy_script_api::lua::ToLuaProxy<'_> for #base_type_name {
//             fn to_lua_proxy<'lua>(self, lua: &'lua #tealr_root::mlu::mlua::Lua) -> #tealr_root::mlu::mlua::Result<#tealr_root::mlu::mlua::Value<'lua>>{
//                 <#proxy_name as #tealr_root::mlu::mlua::IntoLua>::into_lua(#proxy_name::new(self),lua)
//             }
//         }
//     };

//     definition
// }

// pub fn impl_enum(enum_data: EnumData) -> TokenStream {
//     todo!()
// }
