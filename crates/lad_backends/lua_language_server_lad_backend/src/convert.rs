mod generics;

use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use generics::{
    erase_generics, generic_type_modules, instantiation_class_name, mentions_generic, names_generic,
};
use indexmap::{IndexMap, IndexSet};
use ladfile::{
    ArgumentVisitor, LadFile, LadFunction, LadNamedType, LadTypeId, LadTypeKind,
    ReflectionPrimitiveKind,
};

use crate::{
    keywords::ForbiddenKeywords,
    lua_declaration_file::{
        ClassField, FieldScope, FunctionParam, FunctionSignature, LuaClass, LuaDefinitionFile,
        LuaModule, LuaOperator, LuaOperatorKind, LuaPrimitiveType, LuaType, TypeInstance,
    },
};

/// Converts a LAD file to a declaration file per namespace, keyed by namespace, the root namespace (`""`, with the builtins) first.
///
/// Classes are named after their namespace, i.e. `math.Vec3`, like `import_namespace("math").Vec3` in scripts.
pub fn convert_ladfile_to_lua_declaration_files(
    ladfile: &ladfile::LadFile,
) -> Result<IndexMap<String, LuaDefinitionFile>, anyhow::Error> {
    let mut modules = vec![];
    modules.extend(class_modules(ladfile));
    modules.extend(generic_type_modules(ladfile));
    modules.extend(fields_modules(ladfile));

    // `import_namespace` needs the namespace classes, so they are built before the globals
    let namespaces = namespace_modules(ladfile);
    let mut globals_module = globals_module(ladfile);
    link_import_namespace(&mut globals_module, &namespaces);
    modules.extend(namespaces);
    modules.push(globals_module);

    let mut files: IndexMap<String, LuaDefinitionFile> = IndexMap::new();
    files.insert(String::new(), LuaDefinitionFile::default());
    for module in modules {
        files
            .entry(module.namespace.clone())
            .or_default()
            .modules
            .push(module);
    }
    Ok(files)
}

/// The name of the class of a type: qualified by its namespace, i.e. `math.Vec3`, like `import_namespace("math").Vec3` in scripts.
pub fn lua_class_name(ladfile: &LadFile, type_id: &LadTypeId) -> String {
    if let Some(lad_type) = ladfile.types.get(type_id) {
        qualified(&lad_type.namespace, &lua_type_name(lad_type))
    } else if let Some(generic_type) = ladfile.generic_types.get(type_id) {
        qualified(&generic_type.namespace, &generic_type.identifier)
    } else {
        type_id.to_string()
    }
}

/// The name of a type in its namespace, its identifier. Instantiations of generic types which aren't declared (see
/// `LadFileBuilder::add_generic_type`) each get a class of their own, named after their arguments, i.e. `Range.f32` for
/// `Range<f32>` and `MeshExtractableData.Vec_String` for `MeshExtractableData<Vec<String>>`.
fn lua_type_name(lad_type: &ladfile::LadTypeDefinition) -> String {
    if lad_type.generics.is_empty() {
        return lad_type.identifier.clone();
    }
    let path = lad_type.path.as_str();
    let args = path
        .find('<')
        .and_then(|start| path.rfind('>').map(|end| &path[start + 1..end]))
        .unwrap_or_default();
    format!("{}.{}", lad_type.identifier, short_type_name(args))
}

/// A type path by its identifiers, usable in a class name, i.e. `Vec_String` for `alloc::vec::Vec<alloc::string::String>`, and `()` as `Unit`.
fn short_type_name(path: &str) -> String {
    path.replace("()", "Unit")
        .split(|c: char| !(c.is_alphanumeric() || c == '_' || c == ':'))
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.rsplit("::").next().unwrap_or(segment))
        .collect::<Vec<_>>()
        .join("_")
}

/// The Lua table a type's functions are declared on, its name with `_` for `.`, i.e. `Range_f32` for `core.Range.f32`.
fn lua_table_name(lad_type: &ladfile::LadTypeDefinition) -> String {
    lua_type_name(lad_type).replace('.', "_")
}

/// `name` in the given namespace, i.e. `math.Vec3`. Names in the root namespace stay as they are.
fn qualified(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        name.to_owned()
    } else {
        format!("{namespace}.{name}")
    }
}

fn type_lua_classes(
    ladfile: &ladfile::LadFile,
) -> IndexMap<LadTypeId, (LuaClass, Vec<ClassField>, Vec<FunctionSignature>)> {
    ladfile
        .types
        .iter()
        .map(|(type_id, lad_type)| {
            (
                type_id.clone(),
                type_to_lua_class(type_id, lad_type, ladfile),
            )
        })
        .collect()
}

fn class_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for (type_id, (mut lua_class, variant_fields, functions)) in type_lua_classes(ladfile) {
        let Some(lad_type) = ladfile.types.get(&type_id) else {
            continue;
        };
        let module =
            |class: LuaClass, functions: Vec<FunctionSignature>, type_only: bool| LuaModule {
                name: lua_type_name(lad_type),
                table: lua_table_name(lad_type),
                namespace: lad_type.namespace.clone(),
                class: Some(class),
                functions,
                type_only,
                ..Default::default()
            };
        let Some(registration) = registration_class(ladfile, lad_type) else {
            lua_class.fields.extend(variant_fields);
            modules.push(module(lua_class, functions, false));
            modules.extend(enum_variant_modules(ladfile, &type_id, lad_type));
            continue;
        };

        // values and the type in its namespace share the functions, which `X.Functions` holds:
        // ---@class transform.Transform : ReflectReference, transform.Transform.Functions
        // ---@class transform.Transform.Static : transform.Transform.Functions, bms.ScriptComponentRegistration
        // ---@alias transform.Transform.Type transform.Transform.Static | bms.ScriptComponentRegistration<transform.Transform>
        // the alias is the type of the table, the generic registration in it lets LuaLS infer `A` in generic functions,
        // i.e. `world.get_component(entity, Transform)`, see `registration_of`
        let name = lua_class.name.clone();
        let functions_class = format!("{name}.Functions");
        let static_class = format!("{name}.Static");
        modules.push(module(
            LuaClass {
                name: functions_class.clone(),
                documentation: Some(format!(
                    "The functions of `{name}`, on its values and its type alike."
                )),
                ..Default::default()
            },
            functions,
            false,
        ));
        lua_class
            .parents
            .push(LuaType::Alias(functions_class.clone()));
        modules.push(module(lua_class, vec![], true));
        let mut type_module = module(
            LuaClass {
                name: static_class.clone(),
                // the type's functions come first, so they shadow the registration's, i.e. its `new`
                parents: vec![
                    LuaType::Alias(functions_class),
                    LuaType::Generic {
                        name: registration.clone(),
                        args: vec![LuaType::Alias(name.clone())],
                    },
                ],
                fields: variant_fields,
                documentation: Some(format!(
                    "The type `{name}` in its namespace, see `{name}.Type`."
                )),
                ..Default::default()
            },
            vec![],
            true,
        );
        type_module.aliases.push(TypeInstance {
            name: format!("{name}.Type"),
            definition: LuaType::Union(vec![
                LuaType::Alias(static_class),
                LuaType::Generic {
                    name: registration,
                    args: vec![LuaType::Alias(name.clone())],
                },
            ]),
            description: Some(format!(
                "The type `{name}`, as `import_namespace` and `world.get_type_by_name` give it."
            )),
        });
        modules.push(type_module);
        modules.extend(enum_variant_modules(ladfile, &type_id, lad_type));
    }
    modules
}

/// The fields class of each type or variant constructors take fields for, plus `no_fields` for the rest.
fn fields_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    // the tables of fields constructors take, which get a class each
    let mut fields_references = FieldsReferences::default();
    for function in ladfile.functions.values() {
        fields_references.visit_function(function);
    }
    let fields_references = fields_references.0;

    let mut modules = vec![];
    let mut no_fields = false;
    for (type_id, variant) in fields_references {
        let variant = variant.as_deref();
        if is_enum_payload(ladfile, &type_id, variant) {
            continue;
        }
        match lad_fields(ladfile, &type_id, variant) {
            Some(lad_variant) if lad_variant_has_fields(lad_variant) => {
                match fields_module(ladfile, &type_id, variant, lad_variant) {
                    Ok(module) => modules.push(module),
                    Err(e) => {
                        log::warn!("Error generating the fields of {type_id}: {e}. Ignoring.")
                    }
                }
            }
            _ => no_fields = true,
        }
    }
    if no_fields {
        modules.push(no_fields_module());
    }
    modules
}

/// The `globals` module: the builtin functions, the only globals.
fn globals_module(ladfile: &ladfile::LadFile) -> LuaModule {
    let mut globals_module = LuaModule {
        name: "globals".to_string(),
        ..Default::default()
    };
    for (_, function) in ladfile.builtin_functions() {
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
    globals_module
}

/// Gives `import_namespace` an overload per namespace, returning the namespace's class.
///
/// `import_namespace("namespace")` returns a table of the types in the namespace.
fn link_import_namespace(globals_module: &mut LuaModule, namespaces: &[LuaModule]) {
    if let Some(import) = globals_module
        .functions
        .iter_mut()
        .find(|f| f.name == "import_namespace")
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
}

// /// Splits the given class into two,
// /// - The first one containing all of its non-static + static functions
// /// - The second one only containing its static functions
// pub fn split_static_class_out(class: LuaClass) -> (LuaClass, LuaClass) {
//     let static_class = class.clone();
//     static_class.

//     (class, static_class)
// }

/// The class of a type, with its fields and operators, the fields of its table for its variants, and the functions documented on its table.
fn type_to_lua_class(
    lad_type_id: &LadTypeId,
    lad_type: &ladfile::LadTypeDefinition,
    ladfile: &ladfile::LadFile,
) -> (LuaClass, Vec<ClassField>, Vec<FunctionSignature>) {
    let documentation = ladfile
        .get_type_documentation(lad_type_id)
        .map(ToOwned::to_owned);

    let mut lua_fields = vec![];
    let mut variant_fields = vec![];
    let mut lua_functions = vec![];
    let mut lua_operators = vec![];
    let mut parents = vec![];
    let name = lua_class_name(ladfile, lad_type_id);

    {
        {
            // add fields for the type, the variants of enums are added as fields below, once the functions which take priority are known
            lua_fields.extend(layout_fields(ladfile, &lad_type.layout));

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

                    if let Some(operator) = function.metadata.operator {
                        match lua_function_to_operator(operator, &lua_function) {
                            Some(Ok(op)) => lua_operators.push(op),
                            Some(Err(func)) => lua_functions.push(func),
                            None => {
                                log::warn!(
                                    "Operator function: {} on namespace {:?} doesn't have the signature of its operator, documenting it as a function only",
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
                    variant_fields.push(ClassField {
                        name: variant_name.to_string(),
                        ty: LuaType::Alias(variant_class_name(&name, variant_name)),
                        scope: crate::lua_declaration_file::FieldScope::Public,
                        optional: false,
                        description: None,
                    });
                }
            }

            if lad_type.metadata.is_reflect && name != "ReflectReference" {
                parents.push(LuaType::Alias(String::from("ReflectReference")))
            }
        }
    }

    let class = LuaClass {
        name,
        parents,
        fields: lua_fields,
        // registrations are generic in the type they register, see `registration_of`
        generics: match is_registration_type(ladfile, lad_type_id) {
            true => vec!["T".to_owned()],
            false => vec![],
        },
        documentation,
        exact: false,
        operators: lua_operators,
    };
    (class, variant_fields, lua_functions)
}

/// The class of the registration a type is at runtime, as given by `import_namespace` or `world.get_type_by_name`,
/// i.e. `bms.ScriptComponentRegistration` for a component, see [`ladfile::LadTypeMetadata::registration`].
fn registration_class(ladfile: &LadFile, lad_type: &ladfile::LadTypeDefinition) -> Option<String> {
    let registration = lad_type.metadata.registration.as_ref()?;
    ladfile
        .types
        .contains_key(registration)
        .then(|| lua_class_name(ladfile, registration))
}

/// Whether the type is the registration of other types, i.e. `ScriptComponentRegistration`, whose classes are generic in
/// the type they register: `bms.ScriptComponentRegistration<transform.Transform>`.
fn is_registration_type(ladfile: &LadFile, type_id: &LadTypeId) -> bool {
    ladfile
        .types
        .values()
        .any(|lad_type| lad_type.metadata.registration.as_ref() == Some(type_id))
}

/// The registration a generic function takes a type as, i.e. `bms.ScriptComponentRegistration<A>`, from the
/// registrations the type is `erased` to at runtime, so LuaLS infers `A` from the registration passed.
fn registration_of(ladfile: &LadFile, erased: &LadTypeKind, param: &str) -> Option<LuaType> {
    match erased {
        LadTypeKind::Val(LadNamedType::Id(type_id)) if is_registration_type(ladfile, type_id) => {
            Some(LuaType::Generic {
                name: lua_class_name(ladfile, type_id),
                args: vec![LuaType::Alias(param.to_owned())],
            })
        }
        LadTypeKind::Union(kinds) => kinds
            .iter()
            .map(|kind| registration_of(ladfile, kind, param))
            .collect::<Option<Vec<_>>>()
            .map(LuaType::Union),
        _ => None,
    }
}

/// Registrations a function returns of an unknown type, i.e. from `get_type_by_name(name)`, register a `ReflectReference`:
/// `bms.ScriptComponentRegistration<ReflectReference>`, so generic functions given them return one rather than `nil`.
/// Only returns: LuaLS doesn't match typed registrations against params with an argument, i.e. `X.Type` against `A | B<ReflectReference>`.
fn widen_registrations(ladfile: &LadFile, ty: LuaType) -> LuaType {
    let registrations: IndexSet<String> = ladfile
        .types
        .values()
        .filter_map(|lad_type| lad_type.metadata.registration.as_ref())
        .map(|type_id| lua_class_name(ladfile, type_id))
        .collect();
    fn widen(ty: LuaType, registrations: &IndexSet<String>) -> LuaType {
        match ty {
            LuaType::Alias(name) if registrations.contains(&name) => LuaType::Generic {
                name,
                args: vec![LuaType::Alias("ReflectReference".into())],
            },
            LuaType::Union(types) => LuaType::Union(
                types
                    .into_iter()
                    .map(|ty| widen(ty, registrations))
                    .collect(),
            ),
            LuaType::Array(ty) => LuaType::Array(Box::new(widen(*ty, registrations))),
            ty => ty,
        }
    }
    widen(ty, &registrations)
}

/// The type of the table of a type in its namespace, i.e. `transform.Transform.Type` for `Transform`, see `class_modules`.
/// Types without a registration have no table of their own, they are their class.
fn type_table_type(ladfile: &LadFile, type_id: &LadTypeId) -> LuaType {
    let name = lua_class_name(ladfile, type_id);
    match ladfile
        .types
        .get(type_id)
        .and_then(|lad_type| registration_class(ladfile, lad_type))
    {
        Some(_) => LuaType::Alias(format!("{name}.Type")),
        None => LuaType::Alias(name),
    }
}

/// The fields of a struct or tuple struct, named or by their index, i.e. `[1]`. Enums and opaque types have none.
fn layout_fields(ladfile: &LadFile, layout: &ladfile::LadTypeLayout) -> Vec<ClassField> {
    let ladfile::LadTypeLayout::MonoVariant(lad_variant) = layout else {
        return vec![];
    };
    let mut lua_fields = vec![];
    match lad_variant {
        ladfile::LadVariant::TupleStruct { name, fields } => {
            for (idx, field) in fields.iter().enumerate() {
                lua_fields.push(ClassField {
                    name: format!("[{}]", idx + 1),
                    ty: match field_type(ladfile, &field.type_) {
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
                    ty: match field_type(ladfile, &field.type_) {
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
    }
    lua_fields
}

/// Converts the function implementing an operator to that operator, if it has the expected signature.
///
/// Operators Lua declarations can only express as metamethod fields, i.e. `__eq`, are returned as functions.
pub fn lua_function_to_operator(
    operator: ScriptOperatorNames,
    func: &FunctionSignature,
) -> Option<Result<LuaOperator, FunctionSignature>> {
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

    let erased = |kind: &LadTypeKind| {
        lad_instance_to_lua_type(ladfile, &erase_generics(kind, &function.generics))
    };
    let signature = |kind: &dyn Fn(&LadTypeKind) -> Result<LuaType, anyhow::Error>| -> Result<(Vec<FunctionParam>, LuaType), anyhow::Error> {
        let params = function
            .arguments
            .iter()
            .map(|a| {
                let ident = a.name.as_deref().unwrap_or_default().to_owned();
                Ok(FunctionParam::new(
                    match ForbiddenKeywords::is_forbidden_err(&ident) {
                        Ok(_) => ident,
                        Err(_) => format!("_{ident}"),
                    },
                    kind(&a.kind)?,
                    matches!(a.kind, LadTypeKind::Option(..)),
                    a.documentation.as_ref().map(|d| d.to_string()),
                ))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        let returns = widen_registrations(ladfile, kind(&function.return_type.kind)?);
        Ok((params, returns))
    };
    let overload = |(params, returns): (Vec<FunctionParam>, LuaType)| FunctionSignature {
        name: name.clone(),
        params,
        returns: vec![returns],
        ..Default::default()
    };

    // a type passed by name can't be inferred, so the function is declared untyped, with an overload per type naming it:
    // `---@overload fun(type_name: "bevy_transform::components::transform::Transform"): transform.Transform`
    if function
        .arguments
        .iter()
        .any(|argument| names_generic(&argument.kind, &function.generics))
    {
        let (params, returns) = signature(&erased)?;
        let overloads = ladfile
            .types
            .iter()
            .filter(|(type_id, lad_type)| {
                lad_type.metadata.is_reflect && ladfile.get_type_generics(type_id).is_none()
            })
            .map(|(type_id, lad_type)| {
                let instantiate = |kind: &LadTypeKind| {
                    if names_generic(kind, &function.generics) {
                        return Ok(LuaType::Literal(format!("\"{}\"", lad_type.path)));
                    }
                    let mut kind = kind.clone();
                    for param in &function.generics {
                        kind.replace_param(param, &LadTypeKind::Val(type_id.clone().into()));
                    }
                    lad_instance_to_lua_type(ladfile, &kind)
                };
                // the types of the file are registered, so they are always found
                let (params, returns) = signature(&instantiate)?;
                let returns = match returns {
                    LuaType::Union(types) => LuaType::Union(
                        types
                            .into_iter()
                            .filter(|ty| !matches!(ty, LuaType::Primitive(LuaPrimitiveType::Nil)))
                            .collect(),
                    ),
                    returns => returns,
                };
                Ok::<_, anyhow::Error>(overload((params, returns)))
            })
            .filter_map(|overload| {
                overload
                    .inspect_err(|e| log::warn!("Skipping an overload of {name}: {e}"))
                    .ok()
            })
            .collect();
        return Ok(FunctionSignature {
            name,
            params,
            returns: vec![returns],
            overloads,
            documentation: function.documentation.as_ref().map(|d| d.to_string()),
            has_self: function.receiver,
            ..Default::default()
        });
    }

    let (params, returns) = signature(&|kind| lad_instance_to_lua_type(ladfile, kind))?;

    // what a generic function takes can also be passed untyped, i.e. a handle of unknown type, which LuaLS can only
    // express as an overload, as overloads can't be generic. Not when the type itself is passed: types are their
    // registrations (see `registration_class`), so typed ones would match the overload too, widening what it returns
    let takes_generics = function
        .arguments
        .iter()
        .any(|argument| mentions_generic(&argument.kind, &function.generics));
    let takes_type = function.arguments.iter().any(|argument| {
        matches!(&argument.kind, LadTypeKind::Type { of: LadNamedType::Param { param }, .. } if function.generics.contains(param))
    });
    let overloads = match takes_generics && !takes_type {
        true => vec![overload(signature(&erased)?)],
        false => vec![],
    };

    Ok(FunctionSignature {
        name,
        params,
        returns: vec![returns],
        async_fn: false,
        deprecated: false,
        nodiscard: false,
        package: false,
        overloads,
        generics: function.generics.clone(),
        documentation: function.documentation.as_ref().map(|d| d.to_string()),
        has_self: function.receiver,
    })
}

pub fn to_lua_many(
    ladfile: &LadFile,
    lad_types: &[ladfile::LadTypeKind],
) -> Result<Vec<LuaType>, anyhow::Error> {
    let lua_types = lad_types
        .iter()
        .map(|lad_type| lad_instance_to_lua_type(ladfile, lad_type))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lua_types)
}

/// Converts a LAD type to its Lua type, see [`LuaTypeVisitor`].
pub fn lad_instance_to_lua_type(
    ladfile: &LadFile,
    lad_type: &ladfile::LadTypeKind,
) -> Result<LuaType, anyhow::Error> {
    LuaTypeVisitor { ladfile }.visit(lad_type)
}

/// Converts LAD types to Lua types.
pub struct LuaTypeVisitor<'a> {
    /// The file the types are from
    pub ladfile: &'a LadFile,
}

impl ArgumentVisitor for LuaTypeVisitor<'_> {
    type Output = Result<LuaType, anyhow::Error>;

    // instantiations of declared generic types are named by their generic type, see `visit_generic`, the types of the
    // file include instantiations of undeclared ones, which have classes of their own, see `lua_type_name`
    fn visit_type_id(&mut self, lad_type_id: &LadTypeId) -> Self::Output {
        Ok(LuaType::Alias(lua_class_name(self.ladfile, lad_type_id)))
    }

    // a type is passed as its registration, i.e. `bms.ScriptComponentRegistration<A>` for the parameter of a generic
    // function, and `transform.Transform.Type` for `Transform`, see `class_modules`
    fn visit_type(&mut self, of: &LadNamedType, erased: &LadTypeKind) -> Self::Output {
        match of {
            LadNamedType::Id(type_id) => Ok(type_table_type(self.ladfile, type_id)),
            LadNamedType::Param { param } => Ok(registration_of(self.ladfile, erased, param)
                .unwrap_or_else(|| LuaType::Alias(param.clone()))),
            LadNamedType::Generic { .. } => self.visit_named(of),
        }
    }

    fn visit_primitive(&mut self, primitive_kind: &ReflectionPrimitiveKind) -> Self::Output {
        Ok(lad_primitive_to_lua_type(primitive_kind))
    }

    fn visit_unknown(&mut self, _: &LadTypeId) -> Self::Output {
        Ok(LuaType::Any)
    }

    // errors are raised as Lua errors rather than returned, so the result is the value
    fn visit_interop_result(&mut self, inner: &ladfile::LadTypeKind) -> Self::Output {
        self.visit(inner)
    }

    fn visit_option(&mut self, inner: &ladfile::LadTypeKind) -> Self::Output {
        Ok(LuaType::Union(vec![
            self.visit(inner)?,
            LuaType::Primitive(LuaPrimitiveType::Nil),
        ]))
    }

    fn visit_vec(&mut self, inner: &ladfile::LadTypeKind) -> Self::Output {
        Ok(LuaType::Array(Box::new(self.visit(inner)?)))
    }

    fn visit_array(&mut self, inner: &ladfile::LadTypeKind, _size: usize) -> Self::Output {
        self.visit_vec(inner)
    }

    fn visit_hash_map(
        &mut self,
        key: &ladfile::LadTypeKind,
        value: &ladfile::LadTypeKind,
    ) -> Self::Output {
        Ok(LuaType::Dictionary {
            key: Box::new(self.visit(key)?),
            value: Box::new(self.visit(value)?),
        })
    }

    fn visit_hash_set(&mut self, inner: &ladfile::LadTypeKind) -> Self::Output {
        Ok(LuaType::Dictionary {
            key: Box::new(self.visit(inner)?),
            value: Box::new(LuaType::Primitive(LuaPrimitiveType::Boolean)),
        })
    }

    fn visit_tuple(&mut self, inner: &[ladfile::LadTypeKind]) -> Self::Output {
        Ok(if inner.is_empty() {
            LuaType::Primitive(LuaPrimitiveType::Nil)
        } else {
            LuaType::Tuple(to_lua_many(self.ladfile, inner)?)
        })
    }

    fn visit_untyped_tuple(&mut self) -> Self::Output {
        Ok(LuaType::Variadic(Box::new(lad_primitive_to_lua_type(
            &ReflectionPrimitiveKind::ScriptValue,
        ))))
    }

    fn visit_tuple_with_rest(
        &mut self,
        head: &[ladfile::LadTypeKind],
        _rest: &ladfile::LadTypeKind,
    ) -> Self::Output {
        // LuaLS has no variadic tuples, and an `[integer]` field would also type the head, so the rest is left untyped
        self.visit_tuple(head)
    }

    fn visit_union(&mut self, inner: &[ladfile::LadTypeKind]) -> Self::Output {
        Ok(LuaType::Union(to_lua_many(self.ladfile, inner)?))
    }

    fn visit_fields(&mut self, of: &LadNamedType, variant: Option<&str>) -> Self::Output {
        let ladfile = self.ladfile;
        let LadNamedType::Id(type_id) = of else {
            // the fields of generic types aren't given classes yet
            return Ok(LuaType::Primitive(LuaPrimitiveType::Table));
        };
        Ok(if is_enum_payload(ladfile, type_id, variant) {
            LuaType::Primitive(LuaPrimitiveType::Table)
        } else if lad_fields(ladfile, type_id, variant).is_some_and(lad_variant_has_fields) {
            LuaType::Alias(fields_class_name(ladfile, type_id, variant)?)
        } else {
            // i.e. unit structs and variants, or opaque types which can only be built from their default
            LuaType::Alias(NO_FIELDS_CLASS.to_string())
        })
    }

    // only in the layouts of generic types, whose classes declare the parameter, i.e. `---@class Handle<A>`
    fn visit_param(&mut self, param: &str) -> Self::Output {
        Ok(LuaType::Alias(param.to_owned()))
    }

    fn visit_generic(&mut self, generic_type: &LadTypeId, args: &[LadTypeKind]) -> Self::Output {
        let Some(generic) = self.ladfile.generic_types.get(generic_type) else {
            return Err(anyhow::anyhow!(
                "{generic_type} is not a generic type of the file"
            ));
        };
        // instantiations with functions of their own have a class of their own, extending this one
        if let Some(class) =
            self.ladfile
                .get_instantiation(generic_type, args)
                .and_then(|instantiation| {
                    instantiation_class_name(self.ladfile, generic, instantiation)
                })
        {
            return Ok(LuaType::Alias(class));
        }
        Ok(LuaType::Generic {
            name: qualified(&generic.namespace, &generic.identifier),
            args: args
                .iter()
                .map(|arg| self.visit(arg))
                .collect::<Result<_, _>>()?,
        })
    }
}

/// The class scripts see for enum variants, which variant classes extend: the class of `EnumVariantRef`, i.e. `bms.EnumVariantRef`.
fn enum_variant_class(ladfile: &LadFile) -> String {
    ladfile
        .types
        .iter()
        .find(|(_, lad_type)| lad_type.identifier == "EnumVariantRef")
        .map(|(type_id, _)| lua_class_name(ladfile, type_id))
        .unwrap_or_else(|| "EnumVariantRef".to_owned())
}

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

fn fields_class_name(
    ladfile: &LadFile,
    type_id: &LadTypeId,
    variant: Option<&str>,
) -> Result<String, anyhow::Error> {
    let name = lua_class_name(ladfile, type_id);
    Ok(match variant {
        Some(variant) => format!("{}.Fields", variant_class_name(&name, variant)),
        None => format!("{name}.Fields"),
    })
}

/// Collects the types, and variants, whose fields are referenced in function signatures.
#[derive(Default)]
struct FieldsReferences(IndexSet<(LadTypeId, Option<String>)>);

impl FieldsReferences {
    fn visit_function(&mut self, function: &LadFunction) {
        let mut collect = |kind: &LadTypeKind| {
            if let LadTypeKind::Fields {
                of: LadNamedType::Id(type_id),
                variant,
            } = kind
            {
                self.0
                    .insert((type_id.clone(), variant.as_deref().map(str::to_owned)));
            }
        };
        for arg in &function.arguments {
            arg.kind.for_each(&mut collect);
        }
        function.return_type.kind.for_each(&mut collect);
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
    let enum_name = lua_class_name(ladfile, type_id);
    let enum_table = lua_table_name(lad_type);
    let variant_parent = enum_variant_class(ladfile);
    // functions on the enum shadow variants of the same name
    let shadowed = |name: &str| {
        lad_type
            .associated_functions
            .iter()
            .filter_map(|id| ladfile.functions.get(id))
            .any(|f| f.identifier == name)
    };
    let variant_functions = ladfile.variant_functions(type_id).collect::<Vec<_>>();

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
                table: format!("{enum_table}.{variant_name}"),
                namespace: lad_type.namespace.clone(),
                class: Some(LuaClass {
                    name: class_name,
                    // a variant is accepted wherever its enum is
                    parents: vec![
                        LuaType::Alias(enum_name.clone()),
                        LuaType::Alias(variant_parent.clone()),
                    ],
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

    let field = |name: String, ty: &LadTypeKind, has_default: bool| ClassField {
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
        table: class_name.clone(),
        namespace: lad_type.namespace.clone(),
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
        table: NO_FIELDS_CLASS.to_string(),
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

/// The type of a field of a value. Reading a field gives primitives by value and unwraps `Option`s, but every other
/// value is a reference (see `into_script_erased`), so containers are also a `ReflectReference`, i.e.
/// `integer[] | ReflectReference`, with the container functions of the reference (`push`, `map_get`, ...).
fn field_type(ladfile: &LadFile, kind: &LadTypeKind) -> Result<LuaType, anyhow::Error> {
    let or_reference =
        |ty: LuaType| LuaType::Union(vec![ty, LuaType::Alias("ReflectReference".into())]);
    Ok(match kind {
        LadTypeKind::Option(inner) => {
            let mut types = match field_type(ladfile, inner)? {
                LuaType::Union(types) => types,
                ty => vec![ty],
            };
            types.push(LuaType::Primitive(LuaPrimitiveType::Nil));
            LuaType::Union(types)
        }
        LadTypeKind::Vec(..)
        | LadTypeKind::Array(..)
        | LadTypeKind::HashMap(..)
        | LadTypeKind::HashSet(..) => or_reference(lad_instance_to_lua_type(ladfile, kind)?),
        LadTypeKind::Tuple(items) if !items.is_empty() => {
            or_reference(lad_instance_to_lua_type(ladfile, kind)?)
        }
        kind => lad_instance_to_lua_type(ladfile, kind)?,
    })
}

/// The classes of the tables `import_namespace` returns, named `namespace.<name>`.
const NAMESPACE_CLASS_PREFIX: &str = "namespace.";

/// A class per namespace, for the table `import_namespace` returns: a field for each type and value in it, and its functions.
///
/// The root namespace (types without a crate, such as primitives) is left out, as are
/// script namespaces: they only exist at runtime so there is nothing to describe.
fn namespace_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    let usable_name = |name: &str| {
        ForbiddenKeywords::is_forbidden_err(name).is_ok()
            && name.chars().all(|c| c.is_alphanumeric() || c == '_')
    };
    let field = |name: &str, ty: LuaType| ClassField {
        name: name.to_string(),
        ty,
        scope: FieldScope::Public,
        optional: false,
        description: None,
    };

    ladfile
        .namespaces
        .iter()
        .filter(|(namespace, _)| !namespace.is_empty())
        .map(|(namespace, lad_namespace)| {
            let mut fields = vec![];
            for (name, type_id) in &lad_namespace.types {
                if usable_name(name) && ladfile.types.contains_key(type_id) {
                    fields.push(field(name, type_table_type(ladfile, type_id)));
                }
            }
            for (name, value) in &lad_namespace.values {
                if !usable_name(name) {
                    continue;
                }
                let ty = lad_instance_to_lua_type(ladfile, &value.type_kind).unwrap_or_else(|e| {
                    log::warn!("Error generating value {name} of namespace {namespace}: {e}. Using `any` type");
                    LuaType::Any
                });
                fields.push(field(name, ty));
            }
            let functions = lad_namespace
                .functions
                .values()
                .filter_map(|id| ladfile.functions.get(id))
                .filter_map(|function| match lad_function_to_lua_function(ladfile, function) {
                    Ok(function) => Some(function),
                    Err(err) => {
                        log::warn!(
                            "Error generating function {} of namespace {namespace}: {err}. Ignoring function.",
                            function.identifier
                        );
                        None
                    }
                })
                .collect();

            let name = format!("{NAMESPACE_CLASS_PREFIX}{namespace}");
            LuaModule {
                name: name.clone(),
                // a table of its own only to declare the functions on, the namespace class is what `import_namespace` returns
                table: name.replace('.', "_"),
                namespace: namespace.to_string(),
                class: Some(LuaClass {
                    name,
                    parents: vec![],
                    fields,
                    generics: vec![],
                    documentation: Some(format!(
                        "The `{namespace}` namespace, as returned by `import_namespace(\"{namespace}\")`."
                    )),
                    // LuaLS doesn't attach the functions declared on the table to an `(exact)` class
                    exact: false,
                    operators: vec![],
                }),
                functions,
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
        ReflectionPrimitiveKind::ScriptValue
        | ReflectionPrimitiveKind::StaticTypeRef
        | ReflectionPrimitiveKind::External(_) => return LuaType::Any,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use ladfile::{
        LadArgument, LadDefault, LadFunctionId, LadFunctionMetadata, LadFunctionNamespace,
        LadTypeLayout,
    };

    fn arg(name: &'static str, kind: LadTypeKind) -> LadArgument {
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
        return_kind: LadTypeKind,
    ) {
        let id = LadFunctionId::new_string_id(format!("{namespace:?}::new"));
        if let LadFunctionNamespace::Type(LadNamedType::Id(type_id)) = &namespace {
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
                    operator: None,
                    misc: Default::default(),
                },
                receiver: false,
                generics: vec![],
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
            LadFunctionNamespace::Type(plain.clone().into()),
            vec![arg(
                "payload",
                LadTypeKind::Fields {
                    of: plain.clone().into(),
                    variant: None,
                },
            )],
            LadTypeKind::Val(plain.into()),
        );

        // the enum level constructor, `EnumType.new{ variant = ..., ... }`
        let enum_type = type_id(&file, "EnumType");
        add_function(
            &mut file,
            LadFunctionNamespace::Type(enum_type.clone().into()),
            vec![arg(
                "payload",
                LadTypeKind::Fields {
                    of: enum_type.clone().into(),
                    variant: None,
                },
            )],
            LadTypeKind::Val(enum_type.clone().into()),
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
            LadFunctionNamespace::Type(variant_ref.clone().into()),
            vec![
                arg("variant", LadTypeKind::Val(variant_ref.clone().into())),
                arg(
                    "payload",
                    LadTypeKind::HashMap(
                        Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::String)),
                        Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::ScriptValue)),
                    ),
                ),
            ],
            LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference),
        );

        // and bound to each variant by the builder
        for variant in ["Unit", "Struct", "TupleStruct"] {
            add_function(
                &mut file,
                LadFunctionNamespace::Variant {
                    type_id: enum_type.clone().into(),
                    variant: variant.into(),
                },
                vec![
                    arg("variant", LadTypeKind::Val(variant_ref.clone().into())),
                    arg(
                        "payload",
                        LadTypeKind::Fields {
                            of: enum_type.clone().into(),
                            variant: Some(variant.into()),
                        },
                    ),
                ],
                LadTypeKind::Val(enum_type.clone().into()),
            );
        }
        file
    }

    /// All the declaration files, one after another
    fn render(file: &LadFile) -> String {
        convert_ladfile_to_lua_declaration_files(file)
            .unwrap()
            .values()
            .map(|declaration_file| {
                let context = tera::Context::from_serialize(declaration_file).unwrap();
                crate::templating::render_template("declaration_file.tera", &context)
                    .unwrap()
                    .replace('\r', "")
            })
            .collect()
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

        let new = block(
            &rendered,
            "---@param payload ladfile_builder.PlainStructType.Fields",
        );
        assert!(
            new.contains("---@return ladfile_builder.PlainStructType"),
            "{new}"
        );

        // the fields class is only a shape, there is no such global table
        let fields = block(
            &rendered,
            "---@class (exact) ladfile_builder.PlainStructType.Fields",
        );
        assert!(!rendered.contains("PlainStructType.Fields = {}"));

        // nothing can fill in the field, so it is required
        assert!(fields.contains("---@field  int_field integer"), "{fields}");

        // the type's default can
        let plain = type_id(&file, "PlainStructType");
        file.types[&plain].metadata.default = Some(LadDefault { variant: None });
        let rendered = render(&file);
        let fields = block(
            &rendered,
            "---@class (exact) ladfile_builder.PlainStructType.Fields",
        );
        assert!(
            fields.contains("---@field  int_field? integer")
                && fields.contains("---@field  [any] 'never'"),
            "{fields}"
        );

        // container fields of a value are references to the container, but constructors take a table
        let usize = || Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::Usize));
        let string = || Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::String));
        for (kind, field, constructor) in [
            (
                LadTypeKind::Vec(usize()),
                "integer[] | ReflectReference",
                "integer[]",
            ),
            (
                LadTypeKind::Option(Box::new(LadTypeKind::HashMap(string(), usize()))),
                "table<string, integer> | ReflectReference | nil",
                "table<string, integer> | nil",
            ),
        ] {
            let ladfile::LadTypeLayout::MonoVariant(ladfile::LadVariant::Struct { fields, .. }) =
                &mut file.types[&plain].layout
            else {
                panic!("PlainStructType is a struct");
            };
            fields[0].type_ = kind;
            let rendered = render(&file);
            let class = block(&rendered, "---@class ladfile_builder.PlainStructType");
            let line = format!("---@field  int_field? {field}\n");
            assert!(class.contains(&line), "missing {line} in {class}");
            let fields = block(
                &rendered,
                "---@class (exact) ladfile_builder.PlainStructType.Fields",
            );
            let line = format!("---@field  int_field? {constructor}\n");
            assert!(fields.contains(&line), "missing {line} in {fields}");
        }
    }

    #[test]
    fn import_returns_the_types_of_the_namespace() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let unit = type_id(&file, "UnitType");
        file.types[&unit].namespace = "other_crate".into();
        file.namespaces["ladfile_builder"]
            .types
            .shift_remove("UnitType");
        file.namespaces
            .entry("other_crate".into())
            .or_default()
            .types
            .insert("UnitType".into(), unit.clone());

        let import_namespace = LadFunctionId::new_string_id("import_namespace".into());
        file.functions.insert(
            import_namespace,
            LadFunction {
                namespace: LadFunctionNamespace::Global,
                identifier: "import_namespace".into(),
                overload_index: None,
                arguments: vec![arg(
                    "path",
                    LadTypeKind::Primitive(ReflectionPrimitiveKind::String),
                )],
                return_type: LadArgument {
                    kind: LadTypeKind::Primitive(ReflectionPrimitiveKind::ScriptValue),
                    documentation: None,
                    name: None,
                },
                documentation: None,
                metadata: LadFunctionMetadata {
                    operator: None,
                    misc: Default::default(),
                },
                receiver: false,
                generics: vec![],
            },
        );
        let rendered = render(&file);

        let namespace = block(&rendered, "---@class namespace.ladfile_builder");
        assert!(namespace.contains("---@field  PlainStructType ladfile_builder.PlainStructType"));
        assert!(!namespace.contains("---@field  UnitType"), "{namespace}");
        let other = block(&rendered, "---@class namespace.other_crate");
        assert!(
            other.contains("---@field  UnitType other_crate.UnitType"),
            "{other}"
        );

        // the root namespace has no class
        assert!(!rendered.contains("namespace.\n"), "{rendered}");

        for name in ["ladfile_builder", "other_crate"] {
            let line = format!("---@overload fun(path: \"{name}\"): namespace.{name}");
            assert!(rendered.contains(&line), "missing {line} in {rendered}");
        }

        // types are registrations, and are found by name like in `world.get_type_by_name`
        let registration = LadTypeId::new_string_id(
            "bevy_mod_scripting_bindings::query::ScriptTypeRegistration".into(),
        );
        let mut definition = file.types[&unit].clone();
        definition.identifier = "ScriptTypeRegistration".into();
        definition.path = registration.to_string();
        definition.namespace = "bms".into();
        file.types.insert(registration.clone(), definition);
        let plain = type_id(&file, "PlainStructType");
        file.types[&plain].metadata.registration = Some(registration.clone());
        let a = LadNamedType::Param { param: "A".into() };
        let type_of = |erased: LadTypeKind| LadTypeKind::Type {
            of: a.clone(),
            erased: Box::new(erased),
        };
        file.functions.insert(
            LadFunctionId::new_string_id("get_type_by_name".into()),
            LadFunction {
                namespace: LadFunctionNamespace::Global,
                identifier: "get_type_by_name".into(),
                overload_index: None,
                arguments: vec![arg(
                    "type_name",
                    type_of(LadTypeKind::Primitive(ReflectionPrimitiveKind::String)),
                )],
                return_type: LadArgument {
                    kind: LadTypeKind::Option(Box::new(type_of(LadTypeKind::Val(
                        registration.clone().into(),
                    )))),
                    documentation: None,
                    name: None,
                },
                documentation: None,
                metadata: LadFunctionMetadata {
                    operator: None,
                    misc: Default::default(),
                },
                receiver: false,
                generics: vec!["A".into()],
            },
        );
        // generic functions take the registration of the type, i.e. `world.get_component`
        file.functions.insert(
            LadFunctionId::new_string_id("get_value".into()),
            LadFunction {
                namespace: LadFunctionNamespace::Global,
                identifier: "get_value".into(),
                overload_index: None,
                arguments: vec![arg(
                    "registration",
                    type_of(LadTypeKind::Val(registration.clone().into())),
                )],
                return_type: LadArgument {
                    kind: LadTypeKind::Val(a.clone()),
                    documentation: None,
                    name: None,
                },
                documentation: None,
                metadata: LadFunctionMetadata {
                    operator: None,
                    misc: Default::default(),
                },
                receiver: false,
                generics: vec!["A".into()],
            },
        );
        let rendered = render(&file);

        // values and the type in its namespace share the functions, the type is also its registration
        for line in [
            "---@class ladfile_builder.PlainStructType.Functions\n",
            "---@class ladfile_builder.PlainStructType : ReflectReference,ladfile_builder.PlainStructType.Functions\n",
            "---@class ladfile_builder.PlainStructType.Static : ladfile_builder.PlainStructType.Functions,bms.ScriptTypeRegistration<ladfile_builder.PlainStructType>\n",
            "---@alias ladfile_builder.PlainStructType.Type ladfile_builder.PlainStructType.Static | bms.ScriptTypeRegistration<ladfile_builder.PlainStructType>\n",
            "---@class bms.ScriptTypeRegistration<T> : ReflectReference\n",
            "---@param registration bms.ScriptTypeRegistration<A>\n",
        ] {
            assert!(rendered.contains(line), "missing {line} in {rendered}");
        }
        let namespace = block(&rendered, "---@class namespace.ladfile_builder");
        assert!(
            namespace.contains("---@field  PlainStructType ladfile_builder.PlainStructType.Type\n"),
            "{namespace}"
        );

        let lookup = block(&rendered, "---@param type_name string");
        let path = &file.types[&type_id(&file, "PlainStructType")].path;
        for line in [
            format!(
                "---@overload fun(type_name: \"{path}\"): ladfile_builder.PlainStructType.Type\n"
            ),
            // a registration of an unknown type, so generic functions given it return a reference rather than `nil`
            "---@return bms.ScriptTypeRegistration<ReflectReference> | nil\n".into(),
        ] {
            assert!(lookup.contains(&line), "missing {line} in {lookup}");
        }
        assert!(!lookup.contains("---@generic"), "{lookup}");
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

        let enum_class = block(&rendered, "---@class ladfile_builder.EnumType ");
        assert!(
            enum_class.contains("---@field  Unit ladfile_builder.EnumType.Unit"),
            "{enum_class}"
        );
        assert!(
            enum_class.contains("---@field  Struct ladfile_builder.EnumType.Struct"),
            "{enum_class}"
        );

        // variants without fields get a class like any other, taking no fields
        assert!(
            rendered.contains(
                "---@class ladfile_builder.EnumType.Unit : ladfile_builder.EnumType,ladfile_builder.EnumVariantRef"
            ),
            "{rendered}"
        );
        // and share one class for their fields, of which there are none
        let unit = block(&rendered, "---@param variant ladfile_builder.EnumType.Unit");
        assert!(unit.contains("---@param payload NoFields"), "{unit}");
        assert!(
            unit.contains("---@return ladfile_builder.EnumType"),
            "{unit}"
        );
        assert!(
            !rendered.contains("ladfile_builder.EnumType.Unit.Fields"),
            "{rendered}"
        );
        let no_fields = block(&rendered, "---@class (exact) NoFields");
        assert!(
            no_fields.contains("---@field  [any] 'never'"),
            "{no_fields}"
        );

        let new = block(
            &rendered,
            "---@param variant ladfile_builder.EnumType.Struct",
        );
        assert!(
            new.contains("---@param payload ladfile_builder.EnumType.Struct.Fields"),
            "{new}"
        );
        assert!(new.contains("---@return ladfile_builder.EnumType"), "{new}");
        assert!(
            new.contains("function EnumType.Struct.new(variant,payload)"),
            "{new}"
        );

        let struct_fields = block(
            &rendered,
            "---@class (exact) ladfile_builder.EnumType.Struct.Fields",
        );
        assert!(
            struct_fields.contains("---@field  field? integer"),
            "{struct_fields}"
        );
        let tuple_fields = block(
            &rendered,
            "---@class (exact) ladfile_builder.EnumType.TupleStruct.Fields",
        );
        assert!(
            tuple_fields.contains("---@field  [1]? integer"),
            "{tuple_fields}"
        );
        assert!(
            tuple_fields.contains("---@field  [2] string"),
            "{tuple_fields}"
        );

        // unbound, the constructor is as generic as at runtime
        let generic = block(
            &rendered,
            "---@param variant ladfile_builder.EnumVariantRef",
        );
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
        assert!(new.contains("---@return ladfile_builder.EnumType"), "{new}");
        assert!(
            !rendered.contains("ladfile_builder.EnumType.Fields"),
            "{rendered}"
        );
    }

    #[test]
    fn functions_keep_their_documentation() {
        let rendered = render(&file_with_constructors());
        assert!(
            rendered.contains("--- Constructs a value.\n---@param payload"),
            "{rendered}"
        );
    }

    #[test]
    fn generic_types_are_declared_once_with_their_parameters() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let rendered = render(&file);

        let declarations = rendered
            .matches("---@class ladfile_builder.OpaqueGenericType<T> : ReflectReference")
            .count();
        assert_eq!(declarations, 1, "{rendered}");

        // the instantiations are described by the generic class, and the type has no table of its own
        assert!(
            !rendered.contains("---@class ladfile_builder.OpaqueGenericType\n"),
            "{rendered}"
        );
        assert!(!rendered.contains("OpaqueGenericType = {}"), "{rendered}");

        // and as there is nothing to construct, it is not in the namespace either
        let namespace = block(&rendered, "---@class namespace.ladfile_builder");
        assert!(!namespace.contains("OpaqueGenericType"), "{namespace}");

        // its fields are in terms of its parameter
        let class = block(
            &rendered,
            "---@class ladfile_builder.OpaqueGenericType<T> : ReflectReference",
        );
        assert!(class.contains("---@field  value? T"), "{class}");

        // as an enum, it has a field per variant, of the variant's class, generic over the same parameter
        let mut file = file;
        let generic_type = file.generic_types.values_mut().next().unwrap();
        generic_type.layout = LadTypeLayout::Enum(vec![
            ladfile::LadVariant::TupleStruct {
                name: "Some".into(),
                fields: vec![ladfile::LadField {
                    type_: LadTypeKind::Val(LadNamedType::Param { param: "T".into() }),
                    has_default: false,
                }],
            },
            ladfile::LadVariant::Unit {
                name: "None".into(),
            },
        ]);
        let rendered = render(&file);
        let class = block(
            &rendered,
            "---@class ladfile_builder.OpaqueGenericType<T> : ReflectReference",
        );
        assert!(
            class.contains("---@field  Some ladfile_builder.OpaqueGenericType.Some<T>")
                && class.contains("---@field  None ladfile_builder.OpaqueGenericType.None<T>"),
            "{class}"
        );
        assert!(
            rendered.contains(
                "---@class ladfile_builder.OpaqueGenericType.Some<T> : ladfile_builder.OpaqueGenericType<T>,EnumVariantRef"
            ),
            "{rendered}"
        );
    }

    #[test]
    fn functions_of_instantiations_are_on_a_subclass() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let generic_id =
            LadTypeId::new_string_id("ladfile_builder::test::OpaqueGenericType".into());
        let plain = type_id(&file, "PlainStructType");
        let applied = |arg: LadTypeKind| LadNamedType::Generic {
            generic_type: generic_id.clone(),
            args: vec![arg],
        };
        let of_plain = applied(LadTypeKind::Val(plain.clone().into()));
        let mut add = |name: &'static str, namespace: LadFunctionNamespace, ret: LadTypeKind| {
            let id = LadFunctionId::new_string_id(name.into());
            file.functions.insert(
                id.clone(),
                LadFunction {
                    namespace,
                    identifier: name.into(),
                    overload_index: None,
                    arguments: vec![arg("_self", LadTypeKind::Ref(of_plain.clone()))],
                    return_type: arg("return", ret),
                    documentation: None,
                    metadata: LadFunctionMetadata {
                        operator: None,
                        misc: Default::default(),
                    },
                    receiver: true,
                    generics: vec![],
                },
            );
            id
        };
        let only = add(
            "only_on_plain",
            LadFunctionNamespace::Type(of_plain.clone()),
            LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool),
        );
        add(
            "new",
            LadFunctionNamespace::Variant {
                type_id: of_plain.clone(),
                variant: "Some".into(),
            },
            LadTypeKind::Val(of_plain.clone()),
        );
        // and a generic function, generic over the type it is given, and over a generic type applied to it
        let a = || LadNamedType::Param { param: "A".into() };
        let get = LadFunctionId::new_string_id("get".into());
        file.functions.insert(
            get.clone(),
            LadFunction {
                namespace: LadFunctionNamespace::Type(plain.clone().into()),
                identifier: "get".into(),
                overload_index: None,
                arguments: vec![
                    arg(
                        "registration",
                        LadTypeKind::Type {
                            of: a(),
                            erased: Box::new(LadTypeKind::Val(plain.clone().into())),
                        },
                    ),
                    arg("handle", LadTypeKind::Val(applied(LadTypeKind::Val(a())))),
                ],
                return_type: arg(
                    "return",
                    LadTypeKind::Option(Box::new(LadTypeKind::Val(a()))),
                ),
                documentation: None,
                metadata: LadFunctionMetadata {
                    operator: None,
                    misc: Default::default(),
                },
                receiver: false,
                generics: vec!["A".into()],
            },
        );
        file.types
            .get_mut(&plain)
            .unwrap()
            .associated_functions
            .push(get);
        let generic_type = file.generic_types.get_mut(&generic_id).unwrap();
        generic_type.instantiations.push(ladfile::LadInstantiation {
            args: vec![LadTypeKind::Val(plain.clone().into())],
            associated_functions: vec![only.clone()],
            metadata: file.types[&plain].metadata.clone(),
        });
        // i.e. `Time<()>`
        let unit = LadTypeKind::Tuple(vec![]);
        generic_type.instantiations.push(ladfile::LadInstantiation {
            args: vec![unit.clone()],
            associated_functions: vec![only],
            metadata: file.types[&plain].metadata.clone(),
        });
        let rendered = render(&file);

        // the generic function returns the type it is given, which types are, so there is no untyped overload
        assert!(
            rendered.contains(
                "---@generic A\n---@param registration A\n---@param handle ladfile_builder.OpaqueGenericType<A>\n---@return A | nil\nfunction PlainStructType.get(registration,handle) end"
            ),
            "{rendered}"
        );

        // the function of an instantiation is on its subclass, which names the instantiation
        assert!(
            rendered.contains(
                "---@class ladfile_builder.OpaqueGenericType.PlainStructType : ladfile_builder.OpaqueGenericType<ladfile_builder.PlainStructType>"
            ),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "function ladfile_builder_OpaqueGenericType_PlainStructType.only_on_plain("
            ),
            "{rendered}"
        );
        // so are the functions of its variants, on a subclass of the generic variant class
        for expected in [
            "---@field  Some ladfile_builder.OpaqueGenericType.PlainStructType.Some",
            "---@class ladfile_builder.OpaqueGenericType.PlainStructType.Some : ladfile_builder.OpaqueGenericType.Some<ladfile_builder.PlainStructType>,ladfile_builder.OpaqueGenericType.PlainStructType",
            "---@param _self ladfile_builder.OpaqueGenericType.PlainStructType.Some",
            "function ladfile_builder_OpaqueGenericType_PlainStructType_Some.new(",
        ] {
            assert!(rendered.contains(expected), "{expected}\n{rendered}");
        }
        assert_eq!(
            lad_instance_to_lua_type(&file, &LadTypeKind::Val(of_plain))
                .map(|ty| format!("{ty:?}"))
                .unwrap(),
            format!(
                "{:?}",
                LuaType::Alias("ladfile_builder.OpaqueGenericType.PlainStructType".into())
            )
        );
        // a unit argument is named `Unit`
        assert!(
            rendered.contains("---@class ladfile_builder.OpaqueGenericType.Unit : ladfile_builder.OpaqueGenericType<nil>"),
            "{rendered}"
        );
    }

    #[test]
    fn instantiations_are_referenced_with_their_arguments() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let rendered = render(&file);

        // as a field, an argument and a return type
        assert!(
            rendered.contains("---@field  generic? ladfile_builder.OpaqueGenericType<integer>"),
            "{rendered}"
        );
        assert!(
            rendered.contains("---@param arg2 ladfile_builder.OpaqueGenericType<integer>"),
            "{rendered}"
        );
        assert!(
            rendered.contains("---@return ladfile_builder.OpaqueGenericType<integer>"),
            "{rendered}"
        );
    }

    #[test]
    fn generic_arguments_can_be_nested_or_unknown() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let generic_type =
            LadTypeId::new_string_id("ladfile_builder::test::OpaqueGenericType".into());
        let applied = |arg: LadTypeKind| {
            LadTypeKind::Val(LadNamedType::Generic {
                generic_type: generic_type.clone(),
                args: vec![arg],
            })
        };

        // an instantiation with an instantiation as its argument, and one with an argument the file doesn't know
        let nested = applied(applied(LadTypeKind::Primitive(
            ReflectionPrimitiveKind::Usize,
        )));
        let unknown = applied(LadTypeKind::Unknown(LadTypeId::new_string_id(
            "not::in::the::file".into(),
        )));

        assert_eq!(
            lad_instance_to_lua_type(&file, &nested)
                .map(|ty| format!("{ty:?}"))
                .unwrap(),
            format!(
                "{:?}",
                LuaType::Generic {
                    name: "ladfile_builder.OpaqueGenericType".into(),
                    args: vec![LuaType::Generic {
                        name: "ladfile_builder.OpaqueGenericType".into(),
                        args: vec![LuaType::Primitive(LuaPrimitiveType::Integer)],
                    }],
                }
            )
        );
        assert_eq!(
            lad_instance_to_lua_type(&file, &unknown)
                .map(|ty| format!("{ty:?}"))
                .unwrap(),
            format!(
                "{:?}",
                LuaType::Generic {
                    name: "ladfile_builder.OpaqueGenericType".into(),
                    args: vec![LuaType::Any],
                }
            )
        );
    }

    #[test]
    fn generic_types_nothing_declared_get_a_class_per_instantiation() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let generic_struct = type_id(&file, "GenericStructType");
        assert!(
            file.generic_types
                .values()
                .all(|g| g.identifier != "GenericStructType")
        );

        // the instantiation is named after its arguments, i.e. `GenericStructType.usize`
        let name = lua_class_name(&file, &generic_struct);
        assert!(
            name.starts_with("ladfile_builder.GenericStructType."),
            "{name}"
        );
        assert_eq!(
            format!(
                "{:?}",
                lad_instance_to_lua_type(&file, &LadTypeKind::Val(generic_struct.into())).unwrap()
            ),
            format!("{:?}", LuaType::Alias(name.clone()))
        );
        let rendered = render(&file);
        assert!(
            rendered.contains(&format!("---@class {name} : ")),
            "{rendered}"
        );
    }

    #[test]
    fn types_are_not_declared_as_globals() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let rendered = render(&file);

        // scripts reach types through their namespace, a global would only exist in the declarations.
        // Nested names like `Enum.Variant` are fields of the table of the enum
        let global_tables = rendered
            .lines()
            .filter(|line| line.ends_with(" = {}") && !line.starts_with("local "))
            .filter(|line| !line.contains('.'))
            .filter(|line| line.chars().next().is_some_and(char::is_uppercase))
            .collect::<Vec<_>>();
        assert!(global_tables.is_empty(), "{global_tables:?}");
        assert!(
            rendered.contains("local PlainStructType = {}"),
            "{rendered}"
        );
    }
}
