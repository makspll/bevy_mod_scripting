use indexmap::IndexMap;
use ladfile::{ArgumentVisitor, LadFile, LadTypeId};

use crate::lua_declaration_file::{
    LuaClass, LuaDefinitionFile, LuaModule, LuaPrimitiveType, LuaType,
};

trait GetLuaIdentifier {
    fn get_lua_identifier(&self, key: LadTypeId) -> String;
}

impl GetLuaIdentifier for ladfile::LadFile {
    fn get_lua_identifier(&self, key: LadTypeId) -> String {
        ArgumentVisitor
    }
}

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

        let mut fields = vec![];
        if let Some(lad_type) = ladfile.types.get(lad_type_id) {
            // add fields for the type
            match lad_type.layout {
                ladfile::LadTypeLayout::Opaque => todo!(),
                ladfile::LadTypeLayout::MonoVariant(lad_variant) => todo!(),
                ladfile::LadTypeLayout::Enum(lad_variants) => todo!(),
            }
        }

        let class = LuaClass {
            name: polymorphic_type_key.identifier.to_string(),
            parents: vec![], // not needed
            fields: vec![],  // TODO: Find fields
            generics,
            documentation,
            exact: true,
            operators: vec![], // TODO: Find operators
        };

        types.push((lad_type_id.clone(), class));
    }
    types
}

pub fn to_lua_many(
    ladfile: &LadFile,
    lad_types: &[ladfile::LadTypeKind],
) -> Result<Vec<LuaType>, anyhow::Error> {
    let lua_types = lad_types
        .iter()
        .map(|lad_type| lad_instance_to_lua_type(&lad_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lua_types)
}

pub fn lad_instance_to_lua_type(
    ladfile: &LadFile,
    lad_type: &ladfile::LadTypeKind,
) -> Result<LuaType, anyhow::Error> {
    Ok(match &lad_type {
        ladfile::LadTypeKind::Primitive(prim) => lad_primitive_to_lua_type(prim),
        ladfile::LadTypeKind::Ref(lad_type_id)
        | ladfile::LadTypeKind::Mut(lad_type_id)
        | ladfile::LadTypeKind::Val(lad_type_id) => LuaType::Alias(ladfile.),
        ladfile::LadTypeKind::Option(lad_type_kind) => LuaType::Union(vec![
            lad_instance_to_lua_type(ladfile, lad_type_kind)?,
            LuaType::Primitive(LuaPrimitiveType::Nil),
        ]),
        ladfile::LadTypeKind::Vec(lad_type_kind) => {
            LuaType::Array(Box::new(lad_instance_to_lua_type(ladfile, lad_type_kind)?))
        }
        ladfile::LadTypeKind::HashMap(key, value) => LuaType::Dictionary {
            key: Box::new(lad_instance_to_lua_type(ladfile, key)?),
            value: Box::new(lad_instance_to_lua_type(ladfile, value)?),
        },
        ladfile::LadTypeKind::InteropResult(lad_type_kind) => {
            lad_instance_to_lua_type(ladfile, lad_type_kind)? // TODO: currently ignores the possibility of an error type, we should have a custom class abstraction here
        }
        ladfile::LadTypeKind::Tuple(lad_type_kinds) => {
            LuaType::Tuple(to_lua_many(ladfile, lad_type_kinds)?)
        }
        ladfile::LadTypeKind::Array(lad_type_kind, _) => {
            LuaType::Array(Box::new(lad_instance_to_lua_type(ladfile, lad_type_kind)?))
        }
        ladfile::LadTypeKind::Union(lad_type_kinds) => {
            LuaType::Union(to_lua_many(ladfile, lad_type_kinds)?)
        }
        ladfile::LadTypeKind::Unknown(_) => LuaType::Any,
    })
}

pub fn lad_primitive_to_lua_type(lad_primitive: &ladfile::LadBMSPrimitiveKind) -> LuaType {
    LuaType::Primitive(match lad_primitive {
        ladfile::LadBMSPrimitiveKind::Bool => LuaPrimitiveType::Boolean,
        ladfile::LadBMSPrimitiveKind::Isize
        | ladfile::LadBMSPrimitiveKind::I8
        | ladfile::LadBMSPrimitiveKind::I16
        | ladfile::LadBMSPrimitiveKind::I32
        | ladfile::LadBMSPrimitiveKind::I64
        | ladfile::LadBMSPrimitiveKind::I128
        | ladfile::LadBMSPrimitiveKind::Usize
        | ladfile::LadBMSPrimitiveKind::U8
        | ladfile::LadBMSPrimitiveKind::U16
        | ladfile::LadBMSPrimitiveKind::U32
        | ladfile::LadBMSPrimitiveKind::U64
        | ladfile::LadBMSPrimitiveKind::U128 => LuaPrimitiveType::Integer,
        ladfile::LadBMSPrimitiveKind::F32 | ladfile::LadBMSPrimitiveKind::F64 => {
            LuaPrimitiveType::Number
        }
        ladfile::LadBMSPrimitiveKind::Char
        | ladfile::LadBMSPrimitiveKind::Str
        | ladfile::LadBMSPrimitiveKind::String
        | ladfile::LadBMSPrimitiveKind::OsString
        | ladfile::LadBMSPrimitiveKind::PathBuf => LuaPrimitiveType::String,
        ladfile::LadBMSPrimitiveKind::FunctionCallContext => LuaPrimitiveType::Any,
        ladfile::LadBMSPrimitiveKind::DynamicFunction
        | ladfile::LadBMSPrimitiveKind::DynamicFunctionMut => LuaPrimitiveType::Function,
        ladfile::LadBMSPrimitiveKind::ReflectReference => {
            return LuaType::Alias("ReflectReference".to_string())
        }
    })
}
