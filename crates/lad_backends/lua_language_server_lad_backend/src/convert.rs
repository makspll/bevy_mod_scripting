use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use indexmap::IndexMap;
use ladfile::{LadFieldOrVariableKind, LadFile, LadFunction, LadTypeId, ReflectionPrimitiveKind};

use crate::{
    keywords::ForbiddenKeywords,
    lua_declaration_file::{
        ClassField, FunctionParam, FunctionSignature, LuaClass, LuaDefinitionFile, LuaModule,
        LuaOperator, LuaOperatorKind, LuaPrimitiveType, LuaType,
    },
};

pub fn convert_ladfile_to_lua_declaration_file(
    ladfile: &ladfile::LadFile,
) -> Result<LuaDefinitionFile, anyhow::Error> {
    let mut definition_file = LuaDefinitionFile {
        modules: vec![],
        diagnostics: vec![],
    };

    // // ignore primitive types
    // let exclude_primitives = true;
    let rust_types = ladfile.polymorphizied_types(false);

    // convert each rust type to a lua class with generics

    let mut lua_classes: IndexMap<LadTypeId, (LuaClass, Vec<FunctionSignature>)> =
        IndexMap::with_capacity(ladfile.types.len());

    for (key, types) in rust_types.iter() {
        // right now one class == one lad type id, when we can properly denormorphize types, we will
        // be able to have multiple lad type ids per class
        let lua_classes_for_type =
            match convert_polymorphic_type_to_lua_classes(key, types.iter().copied(), ladfile) {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("{e}");
                    continue;
                }
            };

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

    let mut globals_module = LuaModule {
        name: "globals".to_string(),
        ..Default::default()
    };
    for (name, instance) in ladfile.globals.iter() {
        let class = match lad_instance_to_lua_type(ladfile, &instance.type_kind) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Error generating global {name}: {e}. Using `any` type");
                LuaType::Any
            }
        };

        let description = if instance.is_static {
            "A static class allowing calls through the \".\" operator only. "
        } else {
            "An global instance of this type"
        };

        // ignore primitives
        if matches!(class, LuaType::Primitive(..)) {
            continue;
        }

        globals_module
            .globals
            .push(crate::lua_declaration_file::TypeInstance {
                name: name.to_string(),
                definition: class,
                description: Some(description.into()),
            })
    }
    definition_file.modules.push(globals_module);

    Ok(definition_file)
}

const GENERIC_PLACEHOLDERS: [&str; 10] = ["T", "U", "V", "W", "X", "Y", "Z", "A", "B", "C"];

// /// Splits the given class into two,
// /// - The first one containing all of its non-static + static functions
// /// - The second one only containing its static functions
// pub fn split_static_class_out(class: LuaClass) -> (LuaClass, LuaClass) {
//     let static_class = class.clone();
//     static_class.

//     (class, static_class)
// }

// TODO: once https://github.com/bevyengine/bevy/issues/17117 is solved, we will be able to figure out
// where the generic types are actually used, for now we only know what they are per type.
pub fn convert_polymorphic_type_to_lua_classes<'a>(
    polymorphic_type_key: &ladfile::PolymorphicTypeKey,
    monomorphized_types: impl Iterator<Item = &'a ladfile::LadTypeId>,
    ladfile: &ladfile::LadFile,
) -> Result<Vec<(LadTypeId, LuaClass, Vec<FunctionSignature>)>, anyhow::Error> {
    let monomorphized_types: Vec<_> = monomorphized_types.collect();
    if monomorphized_types.len() > 1 || polymorphic_type_key.arity != 0 {
        // TODO: support generics, currently bevy doesn't let you track back generic instantiations to their definition
        return Err(anyhow::anyhow!(
            "Type {} with arity {} is not supported yet, ignoring.",
            polymorphic_type_key.identifier,
            polymorphic_type_key.arity
        ));
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
        let mut lua_operators = vec![];
        let mut parents = vec![];
        let name = polymorphic_type_key.identifier.to_string();

        if let Some(lad_type) = ladfile.types.get(*lad_type_id) {
            // add fields for the type
            match &lad_type.layout {
                ladfile::LadTypeLayout::Opaque => {}
                ladfile::LadTypeLayout::MonoVariant(lad_variant) => match lad_variant {
                    ladfile::LadVariant::TupleStruct { name, fields } => {
                        for (idx, field) in fields.iter().enumerate() {
                            lua_fields.push(ClassField {
                                name: format!("[{}]", idx + 1),
                                ty: match lad_instance_to_lua_type(ladfile, &field.type_) {
                                    Ok(ty) => ty,
                                    Err(e) => {
                                        log::warn!(
                                            "error converting field {idx}: {e}. for tuple struct {name}"
                                        );
                                        LuaType::Any
                                    }
                                },
                                scope: crate::lua_declaration_file::FieldScope::Public,
                                optional: false,
                                description: None,
                            })
                        }
                    }
                    ladfile::LadVariant::Struct { name, fields } => {
                        for field in fields.iter() {
                            lua_fields.push(ClassField {
                                name: field.name.clone(),
                                ty: match lad_instance_to_lua_type(ladfile, &field.type_) {
                                    Ok(ty) => ty,
                                    Err(e) => {
                                        log::warn!(
                                            "error converting field {}: {e}. for struct {name}",
                                            field.name
                                        );
                                        LuaType::Any
                                    }
                                },
                                scope: crate::lua_declaration_file::FieldScope::Public,
                                optional: true,
                                description: None,
                            })
                        }
                    }
                    ladfile::LadVariant::Unit { .. } => {}
                },
                ladfile::LadTypeLayout::Enum(_) => {
                    // TODO: enums
                }
            }

            for function in &lad_type.associated_functions {
                if let Some(function) = ladfile.functions.get(function) {
                    let lua_function = match lad_function_to_lua_function(ladfile, function) {
                        Ok(func) => func,
                        Err(err) => {
                            log::warn!(
                                "Error converting function: {} on namespace {:?}: {err}. Using empty definition",
                                function.identifier,
                                function.namespace
                            );
                            FunctionSignature {
                                name: function.identifier.to_string().replace("-", "_"),
                                ..Default::default()
                            }
                        }
                    };

                    if function.metadata.is_operator {
                        match lua_function_to_operator(&lua_function) {
                            Some(Ok(op)) => lua_operators.push(op),
                            Some(Err(func)) => lua_functions.push(func),
                            None => {
                                log::warn!(
                                    "Error converting operator function: {} on namespace {:?}. Skipping",
                                    function.identifier,
                                    function.namespace
                                );
                            }
                        };
                    }
                    lua_functions.push(lua_function);
                }
            }

            if lad_type.metadata.is_reflect && name != "ReflectReference" {
                parents.push(String::from("ReflectReference"))
            }
            // if metadata.is_component {
            //     parents.push(String::from("ScriptComponentRegistration"))
            // }
            // if metadata.is_resource {
            //     parents.push(String::from("ScriptResourceRegistration"))
            // }
        }

        let class = LuaClass {
            name,
            parents,
            fields: lua_fields,
            generics,
            documentation,
            exact: true,
            operators: lua_operators,
        };

        types.push(((*lad_type_id).clone(), class, lua_functions));
    }
    Ok(types)
}

/// converts a lua function to an operator if it matches the expected strucutre
pub fn lua_function_to_operator(
    func: &FunctionSignature,
) -> Option<Result<LuaOperator, FunctionSignature>> {
    let operator = ScriptOperatorNames::parse(&func.name)?;

    // first arg is implied to be `self`
    let (metamethod, second_arg, ret) = match operator {
        ScriptOperatorNames::Addition => (
            LuaOperatorKind::Add,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Subtraction => (
            LuaOperatorKind::Sub,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Multiplication => (
            LuaOperatorKind::Mul,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Division => (
            LuaOperatorKind::Div,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Remainder => (
            LuaOperatorKind::Mod,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Negation => (LuaOperatorKind::Unm, None, func.returns.first()?),
        ScriptOperatorNames::Exponentiation => (
            LuaOperatorKind::Pow,
            Some(func.params.get(1)?),
            func.returns.first()?,
        ),
        ScriptOperatorNames::Equality => {
            return Some(Err(FunctionSignature {
                name: "__eq".into(),
                ..func.clone()
            }));
        }
        ScriptOperatorNames::LessThanComparison => {
            return Some(Err(FunctionSignature {
                name: "__lt".into(),
                ..func.clone()
            }));
        }
        ScriptOperatorNames::Length => (LuaOperatorKind::Len, None, func.returns.first()?),
        ScriptOperatorNames::Iteration => {
            return Some(Err(FunctionSignature {
                name: "__pairs".into(),
                ..func.clone()
            }));
        }
        ScriptOperatorNames::DisplayPrint | ScriptOperatorNames::DebugPrint => {
            return Some(Err(FunctionSignature {
                name: "__tostring".into(),
                ..func.clone()
            }));
        }
    };

    Some(Ok(LuaOperator {
        operation: metamethod,
        param_type: second_arg.map(|a| a.ty.clone()),
        return_type: ret.clone(),
    }))
}

// /// some operators aren't fully supported by lua language server.
// /// we implement those by converting to metatable entries, to signal the existence
// pub fn lua_operator_to_special_function(op: &LuaOperator) -> Option<FunctionSignature> {
//     Some(match op.operation {
//         LuaOperatorKind::Eq => FunctionSignature {
//             name: "__eq",
//             params: ,
//             returns: (),
//             async_fn: (),
//             deprecated: (),
//             nodiscard: (),
//             package: (),
//             overloads: (),
//             generics: (),
//             documentation: (),
//             has_self: (),
//         },
//         LuaOperatorKind::ToString => todo!(),
//         LuaOperatorKind::Pairs => todo!(),
//         _ => return None,
//     })
// }

pub fn lad_function_to_lua_function(
    ladfile: &LadFile,
    function: &LadFunction,
) -> Result<FunctionSignature, anyhow::Error> {
    // overloads get a unique instantiation, but maybe that's wrong
    let name = function.identifier.to_string();
    ForbiddenKeywords::is_forbidden_err(&function.identifier)?;

    let params = function
        .arguments
        .iter()
        .filter(|a| {
            !matches!(
                a.kind,
                LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::FunctionCallContext)
            )
        })
        .enumerate()
        .map(|(idx, a)| {
            let ident = a
                .name
                .as_ref()
                .map(|v| v.to_string())
                .unwrap_or(format!("p{}", idx + 1));
            Ok(FunctionParam {
                name: match ForbiddenKeywords::is_forbidden_err(&ident) {
                    Ok(_) => ident,
                    Err(_) => format!("_{ident}"),
                },
                ty: lad_instance_to_lua_type(ladfile, &a.kind)?,
                optional: matches!(a.kind, LadFieldOrVariableKind::Option(..)),
                description: a.documentation.as_ref().map(|d| d.to_string()),
            })
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    let self_type = match &function.namespace {
        ladfile::LadFunctionNamespace::Type(lad_type_id) => function
            .arguments
            .first()
            .is_some_and(|a| match &a.kind {
                LadFieldOrVariableKind::Ref(i)
                | LadFieldOrVariableKind::Mut(i)
                | LadFieldOrVariableKind::Val(i) => lad_type_id == i,
                _ => false,
            })
            .then_some(lad_type_id),
        ladfile::LadFunctionNamespace::Global => None,
    };

    let returns = lad_instance_to_lua_type(ladfile, &function.return_type.kind)?;

    Ok(FunctionSignature {
        name,
        params,
        returns: vec![returns],
        async_fn: false,
        deprecated: false,
        nodiscard: false,
        package: false,
        overloads: vec![],
        generics: vec![],
        documentation: function.documentation.as_ref().map(|d| d.to_string()),
        has_self: self_type.is_some(),
    })
}

pub fn to_lua_many(
    ladfile: &LadFile,
    lad_types: &[ladfile::LadFieldOrVariableKind],
) -> Result<Vec<LuaType>, anyhow::Error> {
    let lua_types = lad_types
        .iter()
        .map(|lad_type| lad_instance_to_lua_type(ladfile, lad_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lua_types)
}

// pub fn lad_type_to_lua_type(
//     ladfile: &LadFile,
//     lad_type_id: LadTypeId,
// ) -> Result<LuaType, anyhow::Error> {
//     if let Some(primitive) = ladfile.primitives.get(&lad_type_id) {
//         Ok(lad_primitive_to_lua_type(&primitive.kind))
//     } else {
//         if ladfile.get_type_generics(&lad_type_id).is_some() {
//             return Err(anyhow::anyhow!(
//                 "Type contains generics: {}",
//                 ladfile.get_type_identifier(&lad_type_id, None)
//             ));
//         }
//         Ok(LuaType::Alias(
//             ladfile.get_type_identifier(&lad_type_id, None).to_string(),
//         ))
//     }
// }

pub fn lad_instance_to_lua_type(
    ladfile: &LadFile,
    lad_type: &ladfile::LadFieldOrVariableKind,
) -> Result<LuaType, anyhow::Error> {
    Ok(match &lad_type {
        ladfile::LadFieldOrVariableKind::Primitive(prim) => lad_primitive_to_lua_type(prim),
        ladfile::LadFieldOrVariableKind::Ref(lad_type_id)
        | ladfile::LadFieldOrVariableKind::Mut(lad_type_id)
        | ladfile::LadFieldOrVariableKind::Val(lad_type_id) => {
            if ladfile.get_type_generics(lad_type_id).is_none() {
                LuaType::Alias(ladfile.get_type_identifier(lad_type_id, None).to_string())
            } else {
                return Err(anyhow::anyhow!(
                    "Generic fields are not supported: {}",
                    lad_type_id
                ));
            }
        }
        ladfile::LadFieldOrVariableKind::Option(lad_type_kind) => LuaType::Union(vec![
            lad_instance_to_lua_type(ladfile, lad_type_kind)?,
            LuaType::Primitive(LuaPrimitiveType::Nil),
        ]),
        ladfile::LadFieldOrVariableKind::Vec(lad_type_kind) => {
            LuaType::Array(Box::new(lad_instance_to_lua_type(ladfile, lad_type_kind)?))
        }
        ladfile::LadFieldOrVariableKind::HashMap(key, value) => LuaType::Dictionary {
            key: Box::new(lad_instance_to_lua_type(ladfile, key)?),
            value: Box::new(lad_instance_to_lua_type(ladfile, value)?),
        },
        ladfile::LadFieldOrVariableKind::HashSet(key) => LuaType::Dictionary {
            key: Box::new(lad_instance_to_lua_type(ladfile, key)?),
            value: Box::new(LuaType::Primitive(LuaPrimitiveType::Boolean)),
        },
        ladfile::LadFieldOrVariableKind::InteropResult(lad_type_kind) => {
            lad_instance_to_lua_type(ladfile, lad_type_kind)? // TODO: currently ignores the possibility of an error type, we should have a custom class abstraction here
        }
        ladfile::LadFieldOrVariableKind::Tuple(lad_type_kinds) => {
            if lad_type_kinds.is_empty() {
                LuaType::Primitive(LuaPrimitiveType::Nil)
            } else {
                LuaType::Tuple(to_lua_many(ladfile, lad_type_kinds)?)
            }
        }
        ladfile::LadFieldOrVariableKind::Array(lad_type_kind, _) => {
            LuaType::Array(Box::new(lad_instance_to_lua_type(ladfile, lad_type_kind)?))
        }
        ladfile::LadFieldOrVariableKind::Union(lad_type_kinds) => {
            LuaType::Union(to_lua_many(ladfile, lad_type_kinds)?)
        }
        ladfile::LadFieldOrVariableKind::Unknown(_) => LuaType::Any,
    })
}

pub fn lad_primitive_to_lua_type(lad_primitive: &ReflectionPrimitiveKind) -> LuaType {
    LuaType::Primitive(match lad_primitive {
        ReflectionPrimitiveKind::Bool => LuaPrimitiveType::Boolean,
        ReflectionPrimitiveKind::Isize
        | ReflectionPrimitiveKind::I8
        | ReflectionPrimitiveKind::I16
        | ReflectionPrimitiveKind::I32
        | ReflectionPrimitiveKind::I64
        | ReflectionPrimitiveKind::I128
        | ReflectionPrimitiveKind::Usize
        | ReflectionPrimitiveKind::U8
        | ReflectionPrimitiveKind::U16
        | ReflectionPrimitiveKind::U32
        | ReflectionPrimitiveKind::U64
        | ReflectionPrimitiveKind::U128 => LuaPrimitiveType::Integer,
        ReflectionPrimitiveKind::F32 | ReflectionPrimitiveKind::F64 => LuaPrimitiveType::Number,
        ReflectionPrimitiveKind::Char
        | ReflectionPrimitiveKind::Str
        | ReflectionPrimitiveKind::String
        | ReflectionPrimitiveKind::OsString
        | ReflectionPrimitiveKind::PathBuf => LuaPrimitiveType::String,
        ReflectionPrimitiveKind::FunctionCallContext => return LuaType::Any,
        ReflectionPrimitiveKind::DynamicFunction | ReflectionPrimitiveKind::DynamicFunctionMut => {
            LuaPrimitiveType::Function
        }
        ReflectionPrimitiveKind::ReflectReference => {
            return LuaType::Alias("ReflectReference".to_string());
        }
        ReflectionPrimitiveKind::ScriptValue => return LuaType::Any,
        ReflectionPrimitiveKind::External(_) => return LuaType::Any,
    })
}
