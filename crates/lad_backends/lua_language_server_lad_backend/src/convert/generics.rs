//! The classes of generic types, see [`generic_type_modules`].

use crate::lua_declaration_file::{ClassField, FieldScope, LuaClass, LuaModule, LuaType};

use super::{enum_variant_class, lad_variant_name, layout_fields, qualified, variant_class_name};

/// A class per generic type, with its parameters as generics and its fields in terms of them, i.e. `---@field context T`.
///
/// Like enums, generic enums have a field per variant, of the variant's class, which is generic over the same parameters.
pub(super) fn generic_type_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for generic_type in ladfile.generic_types.values() {
        let name = qualified(&generic_type.namespace, &generic_type.identifier);
        let params = || {
            generic_type
                .params
                .iter()
                .map(|param| LuaType::Alias(param.clone()))
                .collect::<Vec<_>>()
        };
        let mut fields = layout_fields(ladfile, &generic_type.layout);
        let mut variant_modules = vec![];
        if let ladfile::LadTypeLayout::Enum(variants) = &generic_type.layout {
            for variant in variants.iter().map(lad_variant_name) {
                let variant_class = variant_class_name(&name, variant);
                fields.push(ClassField {
                    name: variant.to_string(),
                    ty: LuaType::Generic {
                        name: variant_class.clone(),
                        args: params(),
                    },
                    scope: FieldScope::Public,
                    optional: false,
                    description: None,
                });
                variant_modules.push(LuaModule {
                    name: variant_class.clone(),
                    table: format!("{}.{variant}", generic_type.identifier),
                    namespace: generic_type.namespace.clone(),
                    class: Some(LuaClass {
                        name: variant_class,
                        parents: vec![enum_variant_class(ladfile)],
                        generics: generic_type.params.clone(),
                        documentation: Some(format!("The `{variant}` variant of `{name}`.")),
                        ..Default::default()
                    }),
                    type_only: true,
                    ..Default::default()
                });
            }
        }
        modules.push(LuaModule {
            name: generic_type.identifier.clone(),
            table: generic_type.identifier.clone(),
            namespace: generic_type.namespace.clone(),
            class: Some(LuaClass {
                name,
                parents: vec!["ReflectReference".to_string()],
                fields,
                generics: generic_type.params.clone(),
                documentation: generic_type.documentation.clone(),
                ..Default::default()
            }),
            type_only: true,
            ..Default::default()
        });
        modules.extend(variant_modules);
    }
    modules
}
