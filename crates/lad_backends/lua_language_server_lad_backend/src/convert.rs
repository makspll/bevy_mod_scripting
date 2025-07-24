use indexmap::IndexMap;
use ladfile::LadTypeId;

use crate::lua_declaration_file::{LuaClass, LuaDefinitionFile, LuaModule};

pub fn convert_ladfile_to_lua_declaration_file(
    ladfile: ladfile::LadFile,
) -> Result<LuaDefinitionFile, anyhow::Error> {
    let mut definition_file = LuaDefinitionFile {
        modules: vec![],
        diagnostics: vec![],
    };

    let rust_types = ladfile.polymorphizied_types();
    // convert each rust type to a lua class with generics

    let mut lua_classes: IndexMap<LadTypeId, LuaClass> =
        IndexMap::with_capacity(ladfile.types.len());

    for (key, types) in rust_types.iter() {
        // right now one class == one lad type id, when we can properly denormorphize types, we will
        // be able to have multiple lad type ids per class
        let lua_classes_for_type = convert_polymorphic_type_to_lua_classes(key, types, &ladfile);
        lua_classes.extend(lua_classes_for_type);
    }

    for (type_id, lad_type) in ladfile.types.iter() {
        let lua_class = lua_classes
            .get(type_id)
            .ok_or_else(|| anyhow::anyhow!("Lua class not found for type ID: {}", type_id))?;
        definition_file.modules.push(LuaModule {
            name: lad_type.identifier.to_owned(),
            classes: vec![lua_class.clone()],
            ..Default::default()
        });
    }

    Ok(definition_file)
}

const GENERIC_PLACEHOLDERS: [&str; 10] = ["T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C"];

// TODO: once https://github.com/bevyengine/bevy/issues/17117 is solved, we will be able to figure out
// where the generic types are actually used, for now we only know what they are per type.
pub fn convert_polymorphic_type_to_lua_classes(
    polymorphic_type_key: &ladfile::PolymorphicTypeKey,
    monomorphized_types: &[&ladfile::LadTypeId],
    ladfile: &ladfile::LadFile,
) -> Vec<(LadTypeId, LuaClass)> {
    let mut types = Vec::default();
    for &lad_type_id in monomorphized_types {
        let generics = GENERIC_PLACEHOLDERS[0..polymorphic_type_key.arity]
            .iter()
            .map(ToString::to_string)
            .collect();

        let documentation = ladfile
            .get_type_documentation(lad_type_id)
            .map(ToOwned::to_owned);

        let class = LuaClass {
            name: polymorphic_type_key.identifier.to_string(),
            exact: true,
            generics,
            documentation,
            operators: vec![], // TODO: Find operators
            parents: vec![],   // not needed
            fields: vec![],    // TODO: Find fields
        };

        types.push((lad_type_id.clone(), class));
    }
    types
}

// pub fn lad_type_to_lua_type(lad_type: &ladfile::LadType) -> LuaClass {
//     let mut class = LuaClass {
//         name: lad_type.identifier.to_owned(),
//         exact: true,
//         parents: vec![],
//         generics: lad_type.generics.iter().map(),
//         operators: vec![], // TODO: Find operators
//         documentation: lad_type.documentation.clone(),
//         fields: todo!(),
//     };

//     // match &lad_type.layout {
//     //     ladfile::LadTypeLayout::Opaque => todo!(),
//     //     ladfile::LadTypeLayout::MonoVariant(lad_variant) => {

//     //     },
//     //     ladfile::LadTypeLayout::Enum(lad_variants) => todo!(),
//     // }

//     class
// }
