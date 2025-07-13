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

    let mut lua_classes: IndexMap<LadTypeId, LuaClass> =
        IndexMap::with_capacity(ladfile.types.len());

    for (type_id, lad_type) in ladfile.types.iter() {
        let lua_class = lad_type_to_lua_type(lad_type);
        lua_classes.insert(type_id.clone(), lua_class);
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

pub fn lad_type_to_lua_type(lad_type: &ladfile::LadType) -> LuaClass {
    let mut class = LuaClass {
        name: lad_type.identifier.to_owned(),
        ..Default::default()
    };

    // match &lad_type.layout {
    //     ladfile::LadTypeLayout::Opaque => todo!(),
    //     ladfile::LadTypeLayout::MonoVariant(lad_variant) => {

    //     },
    //     ladfile::LadTypeLayout::Enum(lad_variants) => todo!(),
    // }

    class
}
