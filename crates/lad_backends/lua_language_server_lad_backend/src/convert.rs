use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use indexmap::{IndexMap, IndexSet};
use ladfile::{LadFieldOrVariableKind, LadFile, LadFunction, LadTypeId, ReflectionPrimitiveKind};

use crate::{
    keywords::ForbiddenKeywords,
    lua_declaration_file::{
        ClassField, FunctionParam, FunctionSignature, LuaClass, LuaDefinitionFile, LuaModule,
        LuaOperator, LuaOperatorKind, LuaPrimitiveType, LuaType, TypeInstance,
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

    // the tables of fields constructors take, which get a class each
    let mut fields_references = IndexSet::new();
    for function in ladfile.functions.values() {
        collect_function_fields_references(function, &mut fields_references);
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
            class: Some(lua_class.clone()),
            functions,
            ..Default::default()
        });
        definition_file
            .modules
            .extend(enum_variant_modules(ladfile, type_id, lad_type));
    }

    // the fields of an enum extend the fields of each of its variants
    for (type_id, variant) in fields_references.clone() {
        if let (None, Some(ladfile::LadTypeLayout::Enum(variants))) =
            (&variant, ladfile.types.get(&type_id).map(|t| &t.layout))
        {
            for v in variants.iter().filter(|v| lad_variant_has_fields(v)) {
                fields_references.insert((type_id.clone(), Some(lad_variant_name(v).to_string())));
            }
        }
    }

    for (type_id, variant) in fields_references {
        if !has_fields_class(ladfile, &type_id) {
            continue;
        }
        match fields_modules(ladfile, &type_id, variant.as_deref()) {
            Ok(modules) => definition_file.modules.extend(modules),
            Err(e) => log::warn!("Error generating the fields of {type_id}: {e}. Ignoring."),
        }
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

    for (_, function) in ladfile
        .functions
        .iter()
        .filter(|(_, function)| function.namespace.is_global())
    {
        let converted = match lad_function_to_lua_function(ladfile, function) {
            Ok(converted) => converted,
            Err(err) => {
                log::warn!(
                    "Error generating global function {}: {err}. Ignoring function.",
                    function.identifier,
                );
                continue;
            }
        };
        globals_module.functions.push(converted)
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
                // variants are added as fields below, once the functions which take priority are known
                ladfile::LadTypeLayout::Enum(_) => {}
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

            // the variants of an enum are accessible as `Enum.Variant`, unless a function with the same name takes priority.
            // Each variant has a class of its own, see `enum_variant_modules`
            if let ladfile::LadTypeLayout::Enum(variants) = &lad_type.layout {
                for variant in variants {
                    let variant_name = lad_variant_name(variant);
                    if lua_functions.iter().any(|f| f.name == variant_name) {
                        continue;
                    }
                    lua_fields.push(ClassField {
                        name: variant_name.to_string(),
                        ty: LuaType::Alias(variant_class_name(&name, variant_name)),
                        scope: crate::lua_declaration_file::FieldScope::Public,
                        optional: false,
                        description: None,
                    });
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
            exact: false,
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
            Ok(FunctionParam::new(
                match ForbiddenKeywords::is_forbidden_err(&ident) {
                    Ok(_) => ident,
                    Err(_) => format!("_{ident}"),
                },
                lad_instance_to_lua_type(ladfile, &a.kind)?,
                matches!(a.kind, LadFieldOrVariableKind::Option(..)),
                a.documentation.as_ref().map(|d| d.to_string()),
            ))
        })
        .collect::<Result<Vec<_>, anyhow::Error>>()?;

    let has_self = match &function.namespace {
        ladfile::LadFunctionNamespace::Type(lad_type_id) => {
            function.arguments.first().is_some_and(|a| match &a.kind {
                LadFieldOrVariableKind::Ref(i)
                | LadFieldOrVariableKind::Mut(i)
                | LadFieldOrVariableKind::Val(i) => lad_type_id == i,
                _ => false,
            })
        }
        // functions on variants are called on the variant, i.e. `MyEnum.Variant:new{}`
        ladfile::LadFunctionNamespace::Variant { .. } => !function.arguments.is_empty(),
        ladfile::LadFunctionNamespace::Global => false,
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
        has_self,
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
                    "Generic fields are not supported: {lad_type_id}",
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
        ladfile::LadFieldOrVariableKind::UntypedTuple => LuaType::Variadic(Box::new(
            lad_primitive_to_lua_type(&ReflectionPrimitiveKind::ScriptValue),
        )),
        ladfile::LadFieldOrVariableKind::Array(lad_type_kind, _) => {
            LuaType::Array(Box::new(lad_instance_to_lua_type(ladfile, lad_type_kind)?))
        }
        ladfile::LadFieldOrVariableKind::Union(lad_type_kinds) => {
            LuaType::Union(to_lua_many(ladfile, lad_type_kinds)?)
        }
        ladfile::LadFieldOrVariableKind::Unknown(_) => LuaType::Any,
        ladfile::LadFieldOrVariableKind::Fields { type_id, variant } => {
            if has_fields_class(ladfile, type_id) {
                LuaType::Alias(fields_class_name(ladfile, type_id, variant.as_deref())?)
            } else {
                // opaque types can only be built from their default, i.e. `{}`
                LuaType::Primitive(LuaPrimitiveType::Table)
            }
        }
    })
}

/// The class scripts see for enum variants, which variant classes extend.
const ENUM_VARIANT_CLASS: &str = "EnumVariantRef";

fn lad_variant_name(variant: &ladfile::LadVariant) -> &str {
    match variant {
        ladfile::LadVariant::TupleStruct { name, .. }
        | ladfile::LadVariant::Struct { name, .. }
        | ladfile::LadVariant::Unit { name } => name,
    }
}

fn lad_variant_has_fields(variant: &ladfile::LadVariant) -> bool {
    match variant {
        ladfile::LadVariant::TupleStruct { fields, .. } => !fields.is_empty(),
        ladfile::LadVariant::Struct { fields, .. } => !fields.is_empty(),
        ladfile::LadVariant::Unit { .. } => false,
    }
}

/// The class of a variant of an enum, i.e. `MyEnum.Variant`, which is what `MyEnum.Variant` is in scripts.
fn variant_class_name(enum_name: &str, variant: &str) -> String {
    format!("{enum_name}.{variant}")
}

/// Whether the type has fields to describe, opaque types do not.
fn has_fields_class(ladfile: &LadFile, type_id: &LadTypeId) -> bool {
    ladfile
        .types
        .get(type_id)
        .is_some_and(|t| !matches!(t.layout, ladfile::LadTypeLayout::Opaque))
}

/// The class describing the fields of a type, or of a variant of an enum, i.e. `MyType.Fields` or `MyEnum.Variant.Fields`.
fn fields_class_name(
    ladfile: &LadFile,
    type_id: &LadTypeId,
    variant: Option<&str>,
) -> Result<String, anyhow::Error> {
    if ladfile.get_type_generics(type_id).is_some() {
        return Err(anyhow::anyhow!(
            "Fields of generic types are not supported: {type_id}"
        ));
    }
    let name = ladfile.get_type_identifier(type_id, None);
    Ok(match variant {
        Some(variant) => format!("{}.Fields", variant_class_name(&name, variant)),
        None => format!("{name}.Fields"),
    })
}

/// Collects the types, and variants, whose fields are referenced by the given kind.
fn collect_fields_references(
    kind: &LadFieldOrVariableKind,
    out: &mut IndexSet<(LadTypeId, Option<String>)>,
) {
    match kind {
        LadFieldOrVariableKind::Fields { type_id, variant } => {
            out.insert((type_id.clone(), variant.as_ref().map(|v| v.to_string())));
        }
        LadFieldOrVariableKind::Option(inner)
        | LadFieldOrVariableKind::Vec(inner)
        | LadFieldOrVariableKind::HashSet(inner)
        | LadFieldOrVariableKind::InteropResult(inner)
        | LadFieldOrVariableKind::Array(inner, _) => collect_fields_references(inner, out),
        LadFieldOrVariableKind::HashMap(key, value) => {
            collect_fields_references(key, out);
            collect_fields_references(value, out);
        }
        LadFieldOrVariableKind::Tuple(inner) | LadFieldOrVariableKind::Union(inner) => {
            for kind in inner {
                collect_fields_references(kind, out);
            }
        }
        _ => {}
    }
}

fn collect_function_fields_references(
    function: &LadFunction,
    out: &mut IndexSet<(LadTypeId, Option<String>)>,
) {
    for arg in &function.arguments {
        collect_fields_references(&arg.kind, out);
    }
    collect_fields_references(&function.return_type.kind, out);
}

/// A class per variant, i.e. `MyEnum.Variant`, carrying the functions the ladfile builder bound to that variant.
///
/// This is what lets `MyEnum.Variant:new{ ... }` know the fields of the variant, and that it returns a `MyEnum`.
fn enum_variant_modules(
    ladfile: &LadFile,
    type_id: &LadTypeId,
    lad_type: &ladfile::LadTypeDefinition,
) -> Vec<LuaModule> {
    let ladfile::LadTypeLayout::Enum(variants) = &lad_type.layout else {
        return vec![];
    };
    let enum_name = ladfile.get_type_identifier(type_id, None).to_string();
    // functions on the enum shadow variants of the same name
    let shadowed = |name: &str| {
        lad_type
            .associated_functions
            .iter()
            .filter_map(|id| ladfile.functions.get(id))
            .any(|f| f.identifier == name)
    };
    let variant_functions = ladfile
        .functions
        .values()
        .filter_map(|f| match &f.namespace {
            ladfile::LadFunctionNamespace::Variant {
                type_id: enum_id,
                variant,
            } if enum_id == type_id => Some((variant.as_ref(), f)),
            _ => None,
        })
        .collect::<Vec<_>>();

    variants
        .iter()
        .map(lad_variant_name)
        .filter(|variant_name| !shadowed(variant_name))
        .map(|variant_name| {
            let class_name = variant_class_name(&enum_name, variant_name);
            let functions = variant_functions
                .iter()
                .filter(|(variant, _)| *variant == variant_name)
                .filter_map(|(_, function)| {
                    match lad_function_to_lua_function(ladfile, function) {
                        Ok(mut lua_function) => {
                            // the receiver is the variant itself
                            if let Some(receiver) = lua_function.params.first_mut() {
                                receiver.ty = LuaType::Alias(class_name.clone());
                            }
                            Some(lua_function)
                        }
                        Err(err) => {
                            log::warn!(
                                "Error converting function {} for variant {class_name}: {err}. Ignoring function.",
                                function.identifier
                            );
                            None
                        }
                    }
                })
                .collect();

            LuaModule {
                name: class_name.clone(),
                class: Some(LuaClass {
                    name: class_name,
                    parents: vec![ENUM_VARIANT_CLASS.to_string()],
                    fields: vec![],
                    generics: vec![],
                    documentation: Some(format!(
                        "The `{variant_name}` variant of `{enum_name}`."
                    )),
                    exact: false,
                    operators: vec![],
                }),
                functions,
                ..Default::default()
            }
        })
        .collect()
}

/// Rejects fields which are not declared on a payload class.
///
/// LuaLS does not check `(exact)` classes against table literals passed as arguments yet,
/// see <https://github.com/LuaLS/lua-language-server/issues/1990>, this is the workaround suggested there:
/// any undeclared field has to be of a type no value has.
fn unknown_fields_catch_all() -> ClassField {
    ClassField {
        name: "[any]".to_string(),
        ty: LuaType::Literal("'never'".to_string()),
        scope: crate::lua_declaration_file::FieldScope::Public,
        optional: false,
        description: None,
    }
}

/// The classes describing the fields of a type or variant, as accepted by constructors.
///
/// Fields are optional exactly when a constructor can fill them in from a default, see [`ladfile::LadTypeDefinition::field_has_default`].
/// Undeclared fields are rejected, see [`unknown_fields_catch_all`].
///
/// The fields of an enum, i.e. of `MyEnum.new{ variant = "Variant", ... }`, are a union of one class per variant,
/// each with its `variant` and extending that variant's fields.
fn fields_modules(
    ladfile: &LadFile,
    type_id: &LadTypeId,
    variant: Option<&str>,
) -> Result<Vec<LuaModule>, anyhow::Error> {
    let class_name = fields_class_name(ladfile, type_id, variant)?;
    let lad_type = ladfile
        .types
        .get(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type not found: {type_id}"))?;
    let type_name = ladfile.get_type_identifier(type_id, None);

    let class_module = |name: String, parents, fields, documentation: String| LuaModule {
        name: name.clone(),
        class: Some(LuaClass {
            name,
            parents,
            fields,
            generics: vec![],
            documentation: Some(documentation),
            exact: true,
            operators: vec![],
        }),
        type_only: true,
        ..Default::default()
    };

    if let (None, ladfile::LadTypeLayout::Enum(variants)) = (variant, &lad_type.layout) {
        let mut modules = vec![];
        let mut shapes = vec![];
        for lad_variant in variants {
            let variant_name = lad_variant_name(lad_variant);
            let shape_name = format!("{class_name}.{variant_name}");
            let mut fields = vec![ClassField {
                name: "variant".to_string(),
                ty: LuaType::Literal(format!("\"{variant_name}\"")),
                scope: crate::lua_declaration_file::FieldScope::Public,
                optional: false,
                description: None,
            }];
            // the variant's own fields, which already reject unknown fields
            let parents = if lad_variant_has_fields(lad_variant) {
                vec![fields_class_name(ladfile, type_id, Some(variant_name))?]
            } else {
                fields.push(unknown_fields_catch_all());
                vec![]
            };
            modules.push(class_module(
                shape_name.clone(),
                parents,
                fields,
                format!(
                    "The fields of `{type_name}` when constructing the `{variant_name}` variant."
                ),
            ));
            shapes.push(LuaType::Alias(shape_name));
        }
        modules.push(LuaModule {
            name: class_name.clone(),
            aliases: vec![TypeInstance {
                name: class_name,
                definition: LuaType::Union(shapes),
                description: Some(format!(
                    "The fields of `{type_name}`, keyed by name, along with the variant to construct. Optional fields are filled in from defaults."
                )),
            }],
            type_only: true,
            ..Default::default()
        });
        return Ok(modules);
    }

    let field = |name: String, ty: &LadFieldOrVariableKind, optional: bool| -> ClassField {
        ClassField {
            name,
            ty: lad_instance_to_lua_type(ladfile, ty).unwrap_or_else(|e| {
                log::warn!("error converting field of {class_name}: {e}");
                LuaType::Any
            }),
            scope: crate::lua_declaration_file::FieldScope::Public,
            optional,
            description: None,
        }
    };

    let lad_variant = match (&lad_type.layout, variant) {
        (ladfile::LadTypeLayout::MonoVariant(v), None) => Some(v),
        (ladfile::LadTypeLayout::Enum(variants), Some(variant)) => {
            variants.iter().find(|v| lad_variant_name(v) == variant)
        }
        _ => None,
    };
    let optional = |field_has_default: bool| lad_type.field_has_default(variant, field_has_default);

    let (mut fields, documentation): (Vec<_>, _) = match lad_variant {
        Some(ladfile::LadVariant::Struct { fields, .. }) => (
            fields
                .iter()
                .map(|f| field(f.name.clone(), &f.type_, optional(f.has_default)))
                .collect(),
            "keyed by name",
        ),
        Some(ladfile::LadVariant::TupleStruct { fields, .. }) => (
            fields
                .iter()
                .enumerate()
                .map(|(idx, f)| field(format!("[{}]", idx + 1), &f.type_, optional(f.has_default)))
                // the fields can also be keyed by their index as a string, i.e. `{ ["1"] = a }`.
                // These are only alternatives, so they are optional, and required fields are checked under their integer key
                .chain(
                    fields
                        .iter()
                        .enumerate()
                        .map(|(idx, f)| field(format!("[\"{}\"]", idx + 1), &f.type_, true)),
                )
                .collect(),
            "in order, i.e. `{ a, b }`",
        ),
        _ => (vec![], "of which there are none"),
    };
    fields.push(unknown_fields_catch_all());

    let subject = match variant {
        Some(variant) => format!("the `{variant}` variant of `{type_name}`"),
        None => format!("`{type_name}`"),
    };
    Ok(vec![class_module(
        class_name.clone(),
        vec![],
        fields,
        format!(
            "The fields of {subject}, {documentation}. Optional fields are filled in from defaults."
        ),
    )])
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

#[cfg(test)]
mod test {
    use super::*;
    use ladfile::{
        LadArgument, LadDefault, LadFunctionId, LadFunctionMetadata, LadFunctionNamespace,
        LadTypeLayout,
    };

    fn arg(name: &'static str, kind: LadFieldOrVariableKind) -> LadArgument {
        LadArgument {
            kind,
            documentation: None,
            name: Some(name.into()),
        }
    }

    fn add_function(
        file: &mut LadFile,
        namespace: LadFunctionNamespace,
        arguments: Vec<LadArgument>,
        return_kind: LadFieldOrVariableKind,
    ) {
        let id = LadFunctionId::new_string_id(format!("{namespace:?}::new"));
        if let LadFunctionNamespace::Type(type_id) = &namespace {
            file.types
                .get_mut(type_id)
                .unwrap()
                .associated_functions
                .push(id.clone());
        }
        file.functions.insert(
            id,
            LadFunction {
                namespace,
                identifier: "new".into(),
                overload_index: None,
                arguments,
                return_type: LadArgument {
                    kind: return_kind,
                    documentation: None,
                    name: None,
                },
                documentation: Some("Constructs a value.".into()),
                metadata: LadFunctionMetadata {
                    is_operator: false,
                    misc: Default::default(),
                },
            },
        );
    }

    fn type_id(file: &LadFile, identifier: &str) -> LadTypeId {
        file.types
            .iter()
            .find(|(_, t)| t.identifier == identifier)
            .map(|(id, _)| id.clone())
            .unwrap()
    }

    /// The example file, with constructors as the ladfile builder emits them
    fn file_with_constructors() -> LadFile {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();

        // a type level constructor, already bound to the type by the builder
        let plain = type_id(&file, "PlainStructType");
        add_function(
            &mut file,
            LadFunctionNamespace::Type(plain.clone()),
            vec![arg(
                "payload",
                LadFieldOrVariableKind::Fields {
                    type_id: plain.clone(),
                    variant: None,
                },
            )],
            LadFieldOrVariableKind::Val(plain),
        );

        // the enum level constructor, `EnumType.new{ variant = ..., ... }`
        let enum_type = type_id(&file, "EnumType");
        add_function(
            &mut file,
            LadFunctionNamespace::Type(enum_type.clone()),
            vec![arg(
                "payload",
                LadFieldOrVariableKind::Fields {
                    type_id: enum_type.clone(),
                    variant: None,
                },
            )],
            LadFieldOrVariableKind::Val(enum_type.clone()),
        );

        // the generic variant constructor, documented as it is at runtime
        let variant_ref = LadTypeId::new_string_id("EnumVariantRef".into());
        let mut definition = file.types[&type_id(&file, "UnitType")].clone();
        definition.identifier = "EnumVariantRef".into();
        definition.path = "EnumVariantRef".into();
        definition.layout = LadTypeLayout::Opaque;
        definition.associated_functions = vec![];
        file.types.insert(variant_ref.clone(), definition);
        add_function(
            &mut file,
            LadFunctionNamespace::Type(variant_ref.clone()),
            vec![
                arg("variant", LadFieldOrVariableKind::Val(variant_ref.clone())),
                arg(
                    "payload",
                    LadFieldOrVariableKind::HashMap(
                        Box::new(LadFieldOrVariableKind::Primitive(
                            ReflectionPrimitiveKind::String,
                        )),
                        Box::new(LadFieldOrVariableKind::Primitive(
                            ReflectionPrimitiveKind::ScriptValue,
                        )),
                    ),
                ),
            ],
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::ReflectReference),
        );

        // and bound to each variant by the builder
        for variant in ["Unit", "Struct", "TupleStruct"] {
            add_function(
                &mut file,
                LadFunctionNamespace::Variant {
                    type_id: enum_type.clone(),
                    variant: variant.into(),
                },
                vec![
                    arg("variant", LadFieldOrVariableKind::Val(variant_ref.clone())),
                    arg(
                        "payload",
                        LadFieldOrVariableKind::Fields {
                            type_id: enum_type.clone(),
                            variant: Some(variant.into()),
                        },
                    ),
                ],
                LadFieldOrVariableKind::Val(enum_type.clone()),
            );
        }
        file
    }

    fn render(file: &LadFile) -> String {
        let declaration_file = convert_ladfile_to_lua_declaration_file(file).unwrap();
        let context = tera::Context::from_serialize(&declaration_file).unwrap();
        crate::templating::render_template("declaration_file.tera", &context)
            .unwrap()
            .replace('\r', "")
    }

    /// The block of lines starting with the given line, up to the next blank line
    fn block<'a>(rendered: &'a str, start: &str) -> &'a str {
        let begin = rendered
            .find(start)
            .unwrap_or_else(|| panic!("missing `{start}` in:\n{rendered}"));
        let rest = &rendered[begin..];
        &rest[..rest.find("\n\n").unwrap_or(rest.len())]
    }

    /// Sets whether each field of the type has a default, in order of appearance
    fn set_field_defaults(file: &mut LadFile, identifier: &str, defaults: &[bool]) {
        let id = type_id(file, identifier);
        let mut defaults = defaults.iter().copied();
        let mut set = |has_default: &mut bool| *has_default = defaults.next().unwrap();
        let variants = match &mut file.types[&id].layout {
            LadTypeLayout::MonoVariant(variant) => std::slice::from_mut(variant),
            LadTypeLayout::Enum(variants) => variants.as_mut_slice(),
            LadTypeLayout::Opaque => &mut [],
        };
        for variant in variants {
            match variant {
                ladfile::LadVariant::Struct { fields, .. } => {
                    fields.iter_mut().for_each(|f| set(&mut f.has_default))
                }
                ladfile::LadVariant::TupleStruct { fields, .. } => {
                    fields.iter_mut().for_each(|f| set(&mut f.has_default))
                }
                ladfile::LadVariant::Unit { .. } => {}
            }
        }
    }

    #[test]
    fn type_constructors_take_and_return_the_type() {
        let mut file = file_with_constructors();
        set_field_defaults(&mut file, "PlainStructType", &[false]);
        let rendered = render(&file);

        let new = block(&rendered, "---@param payload PlainStructType.Fields");
        assert!(new.contains("---@return PlainStructType"), "{new}");

        // the fields class is only a shape, there is no such global table
        let fields = block(&rendered, "---@class (exact) PlainStructType.Fields");
        assert!(!rendered.contains("PlainStructType.Fields = {}"));

        // nothing can fill in the field, so it is required
        assert!(fields.contains("---@field  int_field integer"), "{fields}");

        // the type's default can
        let plain = type_id(&file, "PlainStructType");
        file.types[&plain].metadata.default = Some(LadDefault { variant: None });
        let rendered = render(&file);
        let fields = block(&rendered, "---@class (exact) PlainStructType.Fields");
        assert!(
            fields.contains("---@field  int_field? integer")
                && fields.contains("---@field  [any] 'never'"),
            "{fields}"
        );
    }

    #[test]
    fn variant_constructors_are_bound_to_each_variant() {
        let mut file = file_with_constructors();
        let enum_type = type_id(&file, "EnumType");
        let definition = &mut file.types[&enum_type];
        // the enum's default only provides the fields of its own variant
        definition.metadata.default = Some(LadDefault {
            variant: Some("Struct".into()),
        });
        // of the others, only fields whose types have a default are optional
        set_field_defaults(&mut file, "EnumType", &[false, true, false]);
        let rendered = render(&file);

        let enum_class = block(&rendered, "---@class EnumType ");
        assert!(
            enum_class.contains("---@field  Unit EnumType.Unit"),
            "{enum_class}"
        );
        assert!(
            enum_class.contains("---@field  Struct EnumType.Struct"),
            "{enum_class}"
        );

        // variants without fields get a class like any other, taking no fields
        assert!(
            rendered.contains("---@class EnumType.Unit : EnumVariantRef"),
            "{rendered}"
        );
        let unit = block(&rendered, "---@param variant EnumType.Unit");
        assert!(
            unit.contains("---@param payload EnumType.Unit.Fields"),
            "{unit}"
        );
        assert!(unit.contains("---@return EnumType"), "{unit}");
        let unit_fields = block(&rendered, "---@class (exact) EnumType.Unit.Fields");
        assert!(
            unit_fields.contains("of which there are none"),
            "{unit_fields}"
        );
        assert!(
            unit_fields.contains("---@field  [any] 'never'"),
            "{unit_fields}"
        );

        let new = block(&rendered, "---@param variant EnumType.Struct");
        assert!(
            new.contains("---@param payload EnumType.Struct.Fields"),
            "{new}"
        );
        assert!(new.contains("---@return EnumType"), "{new}");
        assert!(
            new.contains("function EnumType.Struct.new(variant,payload)"),
            "{new}"
        );

        let struct_fields = block(&rendered, "---@class (exact) EnumType.Struct.Fields");
        assert!(
            struct_fields.contains("---@field  field? integer"),
            "{struct_fields}"
        );
        let tuple_fields = block(&rendered, "---@class (exact) EnumType.TupleStruct.Fields");
        assert!(
            tuple_fields.contains("---@field  [1]? integer"),
            "{tuple_fields}"
        );
        assert!(
            tuple_fields.contains("---@field  [2] string"),
            "{tuple_fields}"
        );
        // string keys are accepted too, as optional alternatives
        assert!(
            tuple_fields.contains("---@field  [\"2\"]? string"),
            "{tuple_fields}"
        );

        // unbound, the constructor is as generic as at runtime
        let generic = block(&rendered, "---@param variant EnumVariantRef");
        assert!(
            generic.contains("---@param payload table<string, any>"),
            "{generic}"
        );
        assert!(generic.contains("---@return ReflectReference"), "{generic}");
    }

    #[test]
    fn enum_constructors_take_a_union_of_variants() {
        let rendered = render(&file_with_constructors());

        let new = block(&rendered, "---@param payload EnumType.Fields");
        assert!(new.contains("---@return EnumType"), "{new}");
        assert!(
            rendered.contains(
                "---@alias EnumType.Fields EnumType.Fields.Unit | EnumType.Fields.Struct | EnumType.Fields.TupleStruct"
            ),
            "{rendered}"
        );

        // variants with fields extend them, so they are emitted too
        let shape = block(
            &rendered,
            "---@class (exact) EnumType.Fields.Struct : EnumType.Struct.Fields",
        );
        assert!(shape.contains("---@field  variant \"Struct\""), "{shape}");
        let fields = block(&rendered, "---@class (exact) EnumType.Struct.Fields");
        assert!(fields.contains("---@field  [any] 'never'"), "{fields}");

        // unit variants have no fields to extend, so reject unknown ones themselves
        let unit = block(&rendered, "---@class (exact) EnumType.Fields.Unit");
        assert!(unit.contains("---@field  variant \"Unit\""), "{unit}");
        assert!(unit.contains("---@field  [any] 'never'"), "{unit}");
    }
}
