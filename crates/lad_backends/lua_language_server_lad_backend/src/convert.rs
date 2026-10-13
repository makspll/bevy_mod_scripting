use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use indexmap::{IndexMap, IndexSet};
use ladfile::{
    ArgumentVisitor, LadFieldOrVariableKind, LadFile, LadFunction, LadTypeId,
    ReflectionPrimitiveKind,
};

use crate::{
    keywords::ForbiddenKeywords,
    lua_declaration_file::{
        ClassField, FieldScope, FunctionParam, FunctionSignature, LuaClass, LuaDefinitionFile,
        LuaModule, LuaOperator, LuaOperatorKind, LuaPrimitiveType, LuaType,
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
    let mut fields_references = FieldsReferences::default();
    for function in ladfile.functions.values() {
        fields_references.visit_function(function);
    }
    let fields_references = fields_references.0;

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

    let mut no_fields = false;
    for (type_id, variant) in fields_references {
        let variant = variant.as_deref();
        if is_enum_payload(ladfile, &type_id, variant) {
            continue;
        }
        match lad_fields(ladfile, &type_id, variant) {
            Some(lad_variant) if lad_variant_has_fields(lad_variant) => {
                match fields_module(ladfile, &type_id, variant, lad_variant) {
                    Ok(module) => definition_file.modules.push(module),
                    Err(e) => {
                        log::warn!("Error generating the fields of {type_id}: {e}. Ignoring.")
                    }
                }
            }
            _ => no_fields = true,
        }
    }
    if no_fields {
        definition_file.modules.push(no_fields_module());
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
                // superseded by `import`
                deprecated: name == "types",
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

    // `import("namespace")` returns a table of the types in the namespace
    let namespaces = namespace_modules(ladfile, &lua_classes);
    if let Some(import) = globals_module
        .functions
        .iter_mut()
        .find(|f| f.name == "import")
    {
        import.overloads = namespaces
            .iter()
            .filter_map(|module| {
                let class = module.class.as_ref()?;
                Some(FunctionSignature {
                    name: import.name.clone(),
                    params: vec![FunctionParam::new(
                        "path".to_string(),
                        LuaType::Literal(format!(
                            "\"{}\"",
                            class.name.strip_prefix(NAMESPACE_CLASS_PREFIX)?
                        )),
                        false,
                        None,
                    )],
                    returns: vec![LuaType::Alias(class.name.clone())],
                    ..Default::default()
                })
            })
            .collect();
    }
    definition_file.modules.extend(namespaces);
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
            let variant = variant.as_deref();
            if is_enum_payload(ladfile, type_id, variant) {
                LuaType::Primitive(LuaPrimitiveType::Table)
            } else if lad_fields(ladfile, type_id, variant).is_some_and(lad_variant_has_fields) {
                LuaType::Alias(fields_class_name(ladfile, type_id, variant)?)
            } else {
                // i.e. unit structs and variants, or opaque types which can only be built from their default
                LuaType::Alias(NO_FIELDS_CLASS.to_string())
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

/// Collects the types, and variants, whose fields are referenced in function signatures.
#[derive(Default)]
struct FieldsReferences(IndexSet<(LadTypeId, Option<String>)>);

impl ArgumentVisitor for FieldsReferences {
    fn visit_lad_type_id(&mut self, _: &LadTypeId) {}

    fn visit_lad_bms_primitive_kind(&mut self, _: &ReflectionPrimitiveKind) {}

    fn walk_fields(&mut self, type_id: &LadTypeId, variant: Option<&str>) {
        self.0.insert((type_id.clone(), variant.map(str::to_owned)));
    }
}

impl FieldsReferences {
    fn visit_function(&mut self, function: &LadFunction) {
        for arg in &function.arguments {
            self.visit(&arg.kind);
        }
        self.visit(&function.return_type.kind);
    }
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

/// The class describing the fields of a type or variant, as accepted by constructors.
///
/// Fields are optional exactly when a constructor can fill them in from a default, see [`ladfile::LadTypeDefinition::field_has_default`].
/// Undeclared fields are rejected, see [`unknown_fields_catch_all`].
fn fields_module(
    ladfile: &LadFile,
    type_id: &LadTypeId,
    variant: Option<&str>,
    lad_variant: &ladfile::LadVariant,
) -> Result<LuaModule, anyhow::Error> {
    let class_name = fields_class_name(ladfile, type_id, variant)?;
    let lad_type = ladfile
        .types
        .get(type_id)
        .ok_or_else(|| anyhow::anyhow!("Type not found: {type_id}"))?;
    let type_name = ladfile.get_type_identifier(type_id, None);

    let field = |name: String, ty: &LadFieldOrVariableKind, has_default: bool| ClassField {
        name,
        ty: lad_instance_to_lua_type(ladfile, ty).unwrap_or_else(|e| {
            log::warn!("error converting field of {class_name}: {e}");
            LuaType::Any
        }),
        scope: crate::lua_declaration_file::FieldScope::Public,
        optional: lad_type.field_has_default(variant, has_default),
        description: None,
    };

    let (mut fields, documentation): (Vec<_>, _) = match lad_variant {
        ladfile::LadVariant::Struct { fields, .. } => (
            fields
                .iter()
                .map(|f| field(f.name.clone(), &f.type_, f.has_default))
                .collect(),
            "keyed by name",
        ),
        ladfile::LadVariant::TupleStruct { fields, .. } => (
            fields
                .iter()
                .enumerate()
                .map(|(idx, f)| field(format!("[{}]", idx + 1), &f.type_, f.has_default))
                .collect(),
            "in order, i.e. `{ a, b }`",
        ),
        ladfile::LadVariant::Unit { .. } => (vec![], "of which there are none"),
    };
    fields.push(unknown_fields_catch_all());

    let subject = match variant {
        Some(variant) => format!("the `{variant}` variant of `{type_name}`"),
        None => format!("`{type_name}`"),
    };
    Ok(LuaModule {
        name: class_name.clone(),
        class: Some(LuaClass {
            name: class_name,
            parents: vec![],
            fields,
            generics: vec![],
            documentation: Some(format!(
                "The fields of {subject}, {documentation}. Optional fields are filled in from defaults."
            )),
            exact: true,
            operators: vec![],
        }),
        type_only: true,
        ..Default::default()
    })
}

/// The class of the fields of every type and variant without fields, i.e. `{}`, shared as they are all the same.
const NO_FIELDS_CLASS: &str = "NoFields";

fn no_fields_module() -> LuaModule {
    LuaModule {
        name: NO_FIELDS_CLASS.to_string(),
        class: Some(LuaClass {
            name: NO_FIELDS_CLASS.to_string(),
            parents: vec![],
            fields: vec![unknown_fields_catch_all()],
            generics: vec![],
            documentation: Some(
                "The fields of a type or enum variant without fields, i.e. `{}`.".to_string(),
            ),
            exact: true,
            operators: vec![],
        }),
        type_only: true,
        ..Default::default()
    }
}

/// The classes of the tables `import` returns, named `namespace.<name>`.
const NAMESPACE_CLASS_PREFIX: &str = "namespace.";

/// A class per native namespace, with a field for each type in it.
///
/// The root namespace (types without a crate, such as primitives) is left out, as are
/// script namespaces: they only exist at runtime so there is nothing to describe.
fn namespace_modules(
    ladfile: &ladfile::LadFile,
    lua_classes: &IndexMap<LadTypeId, (LuaClass, Vec<FunctionSignature>)>,
) -> Vec<LuaModule> {
    let mut namespaces: IndexMap<&str, Vec<ClassField>> = IndexMap::new();
    for (type_id, lad_type) in ladfile.types.iter() {
        let Some((class, _)) = lua_classes.get(type_id) else {
            continue;
        };
        let name = lad_type.identifier.as_str();
        if lad_type.namespace.is_empty()
            || !lad_type.metadata.is_reflect
            || !lad_type.generics.is_empty()
            || ForbiddenKeywords::is_forbidden_err(name).is_err()
            || !name.chars().all(|c| c.is_alphanumeric() || c == '_')
        {
            continue;
        }
        namespaces
            .entry(lad_type.namespace.as_str())
            .or_default()
            .push(ClassField {
                name: name.to_string(),
                ty: LuaType::Alias(class.name.clone()),
                scope: FieldScope::Public,
                optional: false,
                description: None,
            });
    }

    namespaces
        .into_iter()
        .map(|(namespace, fields)| {
            let name = format!("{NAMESPACE_CLASS_PREFIX}{namespace}");
            LuaModule {
                name: name.clone(),
                class: Some(LuaClass {
                    name,
                    parents: vec![],
                    fields,
                    generics: vec![],
                    documentation: Some(format!(
                        "The types in the `{namespace}` namespace, as returned by `import(\"{namespace}\")`."
                    )),
                    exact: true,
                    operators: vec![],
                }),
                type_only: true,
                ..Default::default()
            }
        })
        .collect()
}

/// The fields a constructor of the type takes, or of the variant for enums.
///
/// `None` if the fields cannot be described, i.e. for opaque types, or enums without a variant given.
fn lad_fields<'a>(
    ladfile: &'a LadFile,
    type_id: &LadTypeId,
    variant: Option<&str>,
) -> Option<&'a ladfile::LadVariant> {
    match (&ladfile.types.get(type_id)?.layout, variant) {
        (ladfile::LadTypeLayout::MonoVariant(v), None) => Some(v),
        (ladfile::LadTypeLayout::Enum(variants), Some(variant)) => {
            variants.iter().find(|v| lad_variant_name(v) == variant)
        }
        _ => None,
    }
}

/// Whether the payload of an enum's own constructor, i.e. `MyEnum.new{ variant = "Variant", ... }`, is meant.
///
/// The language server cannot narrow its fields by the `variant` given, so it is left as a table.
/// `MyEnum.Variant:new{...}` describes the fields of each variant.
fn is_enum_payload(ladfile: &LadFile, type_id: &LadTypeId, variant: Option<&str>) -> bool {
    variant.is_none()
        && ladfile
            .types
            .get(type_id)
            .is_some_and(|t| matches!(t.layout, ladfile::LadTypeLayout::Enum(_)))
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
    fn import_returns_the_types_of_the_namespace() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let unit = type_id(&file, "UnitType");
        file.types[&unit].namespace = "other_crate".into();

        let import = LadFunctionId::new_string_id("import".into());
        file.functions.insert(
            import,
            LadFunction {
                namespace: LadFunctionNamespace::Global,
                identifier: "import".into(),
                overload_index: None,
                arguments: vec![arg(
                    "path",
                    LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::String),
                )],
                return_type: LadArgument {
                    kind: LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::ScriptValue),
                    documentation: None,
                    name: None,
                },
                documentation: None,
                metadata: LadFunctionMetadata {
                    is_operator: false,
                    misc: Default::default(),
                },
            },
        );
        let rendered = render(&file);

        let namespace = block(&rendered, "---@class (exact) namespace.ladfile_builder");
        assert!(namespace.contains("---@field  PlainStructType PlainStructType"));
        assert!(!namespace.contains("UnitType"), "{namespace}");
        let other = block(&rendered, "---@class (exact) namespace.other_crate");
        assert!(other.contains("---@field  UnitType UnitType"), "{other}");

        // the root namespace has no class
        assert!(!rendered.contains("namespace.\n"), "{rendered}");

        for name in ["ladfile_builder", "other_crate"] {
            let line = format!("---@overload fun(path: \"{name}\"): namespace.{name}");
            assert!(rendered.contains(&line), "missing {line} in {rendered}");
        }
    }

    #[test]
    fn types_global_is_deprecated() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let plain = type_id(&file, "PlainStructType");
        for name in ["types", "other"] {
            file.globals.insert(
                name.into(),
                ladfile::LadInstance {
                    type_kind: LadFieldOrVariableKind::Val(plain.clone()),
                    is_static: false,
                },
            );
        }
        let rendered = render(&file);
        assert!(
            rendered.contains("---@deprecated use `import` instead\n---@type PlainStructType\n--- An global instance of this type\ntypes = {}"),
            "{rendered}"
        );
        assert!(!rendered.contains("---@deprecated use `import` instead\n---@type PlainStructType\n--- An global instance of this type\nother"));
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
        // and share one class for their fields, of which there are none
        let unit = block(&rendered, "---@param variant EnumType.Unit");
        assert!(unit.contains("---@param payload NoFields"), "{unit}");
        assert!(unit.contains("---@return EnumType"), "{unit}");
        assert!(!rendered.contains("EnumType.Unit.Fields"), "{rendered}");
        let no_fields = block(&rendered, "---@class (exact) NoFields");
        assert!(
            no_fields.contains("---@field  [any] 'never'"),
            "{no_fields}"
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

        // unbound, the constructor is as generic as at runtime
        let generic = block(&rendered, "---@param variant EnumVariantRef");
        assert!(
            generic.contains("---@param payload table<string, any>"),
            "{generic}"
        );
        assert!(generic.contains("---@return ReflectReference"), "{generic}");
    }

    #[test]
    fn enum_constructors_take_a_table() {
        let rendered = render(&file_with_constructors());

        // the language server cannot narrow the fields by the `variant` given, so they are not described
        let new = block(&rendered, "---@param payload table");
        assert!(new.contains("---@return EnumType"), "{new}");
        assert!(!rendered.contains("EnumType.Fields"), "{rendered}");
    }
}
