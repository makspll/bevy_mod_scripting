use std::ops::Index;

use indexmap::IndexMap;
use ladfile::{LadFile, LadFunction, LadTypeId, LadTypeKind};

use crate::lua_declaration_file::{
    ClassField, FunctionParam, FunctionSignature, LuaClass, LuaDefinitionFile, LuaModule,
    LuaPrimitiveType, LuaType,
};

pub fn convert_ladfile_to_lua_declaration_file(
    ladfile: ladfile::LadFile,
) -> Result<LuaDefinitionFile, anyhow::Error> {
    let mut definition_file = LuaDefinitionFile {
        modules: vec![],
        diagnostics: vec![],
    };

    let rust_types = ladfile.polymorphizied_types();

    // convert each rust type to a lua class with generics

    let mut lua_classes: IndexMap<LadTypeId, (LuaClass, Vec<FunctionSignature>)> =
        IndexMap::with_capacity(ladfile.types.len());

    for (key, types) in rust_types.iter() {
        // right now one class == one lad type id, when we can properly denormorphize types, we will
        // be able to have multiple lad type ids per class
        let lua_classes_for_type = convert_polymorphic_type_to_lua_classes(key, types, &ladfile);

        lua_classes.extend(
            lua_classes_for_type
                .into_iter()
                .map(|(id, class, funcs)| (id, (class, funcs))),
        );
    }

    for (type_id, lad_type) in ladfile.types.iter() {
        let (lua_class, functions) = match lua_classes.get(type_id) {
            Some(val) => val.clone(),
            None => continue,
        };
        // TODO: support all types
        // .get(type_id)
        // .ok_or_else(|| anyhow::anyhow!("Lua class not found for type ID: {}", type_id))?;
        definition_file.modules.push(LuaModule {
            name: lad_type.identifier.to_owned(),
            classes: vec![lua_class.clone()],
            functions,
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
) -> Vec<(LadTypeId, LuaClass, Vec<FunctionSignature>)> {
    if monomorphized_types.len() > 1 || polymorphic_type_key.arity != 0 {
        // TODO: support generics, currently bevy doesn't let you track back generic instantiations to their definition
        return vec![];
    }

    let mut types = Vec::default();
    if let Some(lad_type_id) = monomorphized_types.first() {
        let generics = GENERIC_PLACEHOLDERS[0..polymorphic_type_key.arity]
            .iter()
            .map(ToString::to_string)
            .collect();

        let documentation = ladfile
            .get_type_documentation(lad_type_id)
            .map(ToOwned::to_owned);

        let mut lua_fields = vec![];
        let mut lua_functions = vec![];

        if let Some(lad_type) = ladfile.types.get(*lad_type_id) {
            // add fields for the type
            match &lad_type.layout {
                ladfile::LadTypeLayout::Opaque => {}
                ladfile::LadTypeLayout::MonoVariant(lad_variant) => match lad_variant {
                    ladfile::LadVariant::TupleStruct { name, fields } => {
                        for (idx, field) in fields.iter().enumerate() {
                            lua_fields.push(ClassField {
                                name: format!("[{}]", idx + 1),
                                ty: match lad_type_to_lua_type(ladfile, field.type_.clone()) {
                                    Ok(ty) => ty,
                                    Err(e) => panic!("{e}"),
                                },
                                scope: crate::lua_declaration_file::FieldScope::Public,
                                optional: true,
                                description: None,
                            })
                        }
                    }
                    ladfile::LadVariant::Struct { name, fields } => {
                        for field in fields.iter() {
                            lua_fields.push(ClassField {
                                name: field.name.clone(),
                                ty: match lad_type_to_lua_type(ladfile, field.type_.clone()) {
                                    Ok(ty) => ty,
                                    Err(e) => panic!("{e}"),
                                },
                                scope: crate::lua_declaration_file::FieldScope::Public,
                                optional: true,
                                description: None,
                            })
                        }
                    }
                    ladfile::LadVariant::Unit { name } => {}
                },
                ladfile::LadTypeLayout::Enum(lad_variants) => {}
            }

            for function in &lad_type.associated_functions {
                if let Some(function) = ladfile.functions.get(function) {
                    lua_functions.push(match lad_function_to_lua_function(ladfile, function) {
                        Ok(func) => func,
                        Err(err) => {
                            log::error!("Error converting function: {err}");
                            continue;
                        }
                    })
                }
            }
        }

        let class = LuaClass {
            name: polymorphic_type_key.identifier.to_string(),
            parents: vec![],    // not needed
            fields: lua_fields, // TODO: Find fields
            generics,
            documentation,
            exact: true,
            operators: vec![], // TODO: Find operators
        };

        types.push(((*lad_type_id).clone(), class, lua_functions));
    }
    types
}

pub fn lad_function_to_lua_function(
    ladfile: &LadFile,
    function: &LadFunction,
) -> Result<FunctionSignature, anyhow::Error> {
    let params = function
        .arguments
        .iter()
        .enumerate()
        .map(|(idx, a)| {
            Ok(FunctionParam {
                name: a
                    .name
                    .as_ref()
                    .map(|v| v.to_string())
                    .unwrap_or(format!("p{}", idx + 1)),
                ty: lad_instance_to_lua_type(ladfile, &a.kind)?,
                optional: matches!(a.kind, LadTypeKind::Option(..)),
                description: a.documentation.as_ref().map(|d| d.to_string()),
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    let returns = lad_instance_to_lua_type(ladfile, &function.return_type.kind)?;

    Ok(FunctionSignature {
        name: function.identifier.to_string(),
        params,
        returns: vec![returns],
        async_fn: false,
        deprecated: false,
        nodiscard: false,
        package: true,
        overloads: vec![],
        generics: vec![],
        documentation: function.documentation.as_ref().map(|d| d.to_string()),
    })
}

pub fn to_lua_many(
    ladfile: &LadFile,
    lad_types: &[ladfile::LadTypeKind],
) -> Result<Vec<LuaType>, anyhow::Error> {
    let lua_types = lad_types
        .iter()
        .map(|lad_type| lad_instance_to_lua_type(ladfile, &lad_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lua_types)
}

pub fn lad_type_to_lua_type(
    ladfile: &LadFile,
    lad_type_id: LadTypeId,
) -> Result<LuaType, anyhow::Error> {
    if let Some(primitive) = ladfile.primitives.get(&lad_type_id) {
        Ok(lad_primitive_to_lua_type(&primitive.kind))
    } else {
        Ok(LuaType::Alias(
            ladfile.get_type_identifier(&lad_type_id, None).to_string(),
        ))
    }
}

pub fn lad_instance_to_lua_type(
    ladfile: &LadFile,
    lad_type: &ladfile::LadTypeKind,
) -> Result<LuaType, anyhow::Error> {
    Ok(match &lad_type {
        ladfile::LadTypeKind::Primitive(prim) => lad_primitive_to_lua_type(prim),
        ladfile::LadTypeKind::Ref(lad_type_id)
        | ladfile::LadTypeKind::Mut(lad_type_id)
        | ladfile::LadTypeKind::Val(lad_type_id) => {
            LuaType::Alias(ladfile.get_type_identifier(lad_type_id, None).to_string())
        }
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
        ladfile::LadBMSPrimitiveKind::FunctionCallContext => return LuaType::Any,
        ladfile::LadBMSPrimitiveKind::DynamicFunction
        | ladfile::LadBMSPrimitiveKind::DynamicFunctionMut => LuaPrimitiveType::Function,
        ladfile::LadBMSPrimitiveKind::ReflectReference => {
            return LuaType::Alias("ReflectReference".to_string())
        }
    })
}
