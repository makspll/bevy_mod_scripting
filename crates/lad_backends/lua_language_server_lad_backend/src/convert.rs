use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use indexmap::{IndexMap, IndexSet};
use ladfile::{
    ArgumentVisitor, LadFile, LadFunction, LadNamedType, LadTypeId, LadTypeKind,
    ReflectionPrimitiveKind,
};

use crate::{
    keywords::ForbiddenKeywords,
    lua_declaration_file::{
        ClassField, FieldScope, FunctionParam, FunctionSignature, FunctionSignatureShort, LuaClass,
        LuaDefinitionFile, LuaModule, LuaOperator, LuaOperatorKind, LuaPrimitiveType, LuaType,
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
        qualified(&lad_type.namespace, &lad_type.identifier)
    } else if let Some(generic_type) = ladfile.generic_types.get(type_id) {
        qualified(&generic_type.namespace, &generic_type.identifier)
    } else {
        type_id.to_string()
    }
}

/// `name` in the given namespace, i.e. `math.Vec3`. Names in the root namespace stay as they are.
fn qualified(namespace: &str, name: &str) -> String {
    if namespace.is_empty() {
        name.to_owned()
    } else {
        format!("{namespace}.{name}")
    }
}

/// The lua class, and documented functions, of each type which is not an instantiation of a generic type.
fn type_lua_classes(
    ladfile: &ladfile::LadFile,
) -> IndexMap<LadTypeId, (LuaClass, Vec<FunctionSignature>)> {
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

/// A module per class of a type, followed by the modules of its enum variants.
fn class_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for (type_id, (lua_class, functions)) in type_lua_classes(ladfile) {
        let Some(lad_type) = ladfile.types.get(&type_id) else {
            continue;
        };
        modules.push(LuaModule {
            name: lad_type.identifier.to_owned(),
            table: lad_type.identifier.to_owned(),
            namespace: lad_type.namespace.clone(),
            class: Some(lua_class),
            functions,
            ..Default::default()
        });
        modules.extend(enum_variant_modules(ladfile, &type_id, lad_type));
    }
    modules
}

/// A class per generic type, with its parameters as generics and its fields in terms of them, i.e. `---@field context T`.
///
/// Like enums, generic enums have a field per variant, of the variant's class, which is generic over the same parameters.
/// The functions of every instantiation are fields too, i.e. `---@field delta fun(self: time.Time<T>): time.Duration`, as LuaLS
/// keeps resolving the parameters of fields in subclasses, but not of methods. The functions only some instantiations have
/// are on a subclass per instantiation, see [`instantiation_class_name`].
fn generic_type_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for (generic_id, generic_type) in &ladfile.generic_types {
        let name = qualified(&generic_type.namespace, &generic_type.identifier);
        let params = || {
            generic_type
                .params
                .iter()
                .map(|param| LuaType::Alias(param.clone()))
                .collect::<Vec<_>>()
        };
        let mut fields = layout_fields(ladfile, &generic_type.layout);
        fields.extend(function_fields(
            ladfile,
            generic_type
                .associated_functions
                .iter()
                .filter_map(|id| ladfile.functions.get(id)),
        ));
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
                // the functions of the variant of every instantiation, i.e. its constructor
                let variant_functions = ladfile.functions.values().filter(|f| {
                    matches!(&f.namespace, ladfile::LadFunctionNamespace::Variant {
                        type_id: LadNamedType::Generic { generic_type, args },
                        variant: function_variant,
                    } if generic_type == generic_id
                        && function_variant == variant
                        && args.iter().all(|arg| matches!(arg, LadTypeKind::Val(LadNamedType::Param { .. }))))
                });
                variant_modules.push(LuaModule {
                    name: variant_class.clone(),
                    table: format!("{}.{variant}", generic_type.identifier),
                    namespace: generic_type.namespace.clone(),
                    class: Some(LuaClass {
                        name: variant_class,
                        parents: vec![LuaType::Alias(enum_variant_class(ladfile))],
                        fields: function_fields(ladfile, variant_functions),
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
                name: name.clone(),
                parents: vec![LuaType::Alias("ReflectReference".to_string())],
                fields,
                generics: generic_type.params.clone(),
                documentation: generic_type.documentation.clone(),
                ..Default::default()
            }),
            type_only: true,
            ..Default::default()
        });
        modules.extend(variant_modules);

        // the functions only some instantiations have, i.e. `pause` on `time.Time.Virtual : time.Time<time.Virtual>`
        for instantiation in &generic_type.instantiations {
            let Some(class_name) = instantiation_class_name(ladfile, generic_type, instantiation)
            else {
                continue;
            };
            let args = instantiation
                .args
                .iter()
                .map(|arg| lad_instance_to_lua_type(ladfile, arg))
                .collect::<Result<Vec<_>, _>>();
            let args = match args {
                Ok(args) => args,
                Err(err) => {
                    log::warn!(
                        "The arguments of an instantiation of {name} can't be converted: {err}. Its functions are not documented"
                    );
                    continue;
                }
            };
            let functions = instantiation
                .associated_functions
                .iter()
                .filter_map(|id| ladfile.functions.get(id))
                .filter_map(|function| match lad_function_to_lua_function(ladfile, function) {
                    Ok(function) => Some(function),
                    Err(err) => {
                        log::warn!(
                            "Error converting function {} for {class_name}: {err}. Ignoring function.",
                            function.identifier
                        );
                        None
                    }
                })
                .collect();
            modules.push(LuaModule {
                name: class_name.clone(),
                table: class_name.replace('.', "_"),
                namespace: generic_type.namespace.clone(),
                class: Some(LuaClass {
                    name: class_name,
                    parents: vec![LuaType::Generic {
                        name: name.clone(),
                        args,
                    }],
                    generics: vec![],
                    documentation: generic_type.documentation.clone(),
                    ..Default::default()
                }),
                functions,
                ..Default::default()
            });
        }
    }
    modules
}

/// The class of an instantiation of a generic type with functions of its own, i.e. `time.Time.Virtual` for `Time<Virtual>`,
/// which extends the generic class applied to the arguments. Instantiations without functions of their own have none.
///
/// It is named after the identifiers of the arguments, joined by `_` (`Time.Virtual`, `Foo.Image_Mesh`), unless they aren't
/// types of the file or clash with a variant. Its table is the class name with `_` for `.`, a local table, as the generic class
/// has none to nest it in.
fn instantiation_class_name(
    ladfile: &LadFile,
    generic_type: &ladfile::LadGenericType,
    instantiation: &ladfile::LadInstantiation,
) -> Option<String> {
    if instantiation.associated_functions.is_empty() {
        return None;
    }
    let generic_class = qualified(&generic_type.namespace, &generic_type.identifier);
    let suffix = instantiation
        .args
        .iter()
        .map(|arg| match arg {
            LadTypeKind::Val(LadNamedType::Id(type_id)) => {
                ladfile.types.get(type_id).map(|t| t.identifier.clone())
            }
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
        .map(|identifiers| identifiers.join("_"));
    // variant classes are named the same way, i.e. `asset.Handle.Strong`
    let clashes = |suffix: &String| {
        matches!(&generic_type.layout, ladfile::LadTypeLayout::Enum(variants)
            if variants.iter().any(|v| lad_variant_name(v) == suffix))
    };
    match suffix {
        Some(suffix) if !clashes(&suffix) => Some(variant_class_name(&generic_class, &suffix)),
        _ => {
            log::warn!(
                "An instantiation of {generic_class} with arguments {:?} has functions of its own, but no class to document them on: its arguments aren't types of the file, or are named like a variant",
                instantiation.args
            );
            None
        }
    }
}

/// Functions as fields of a class, i.e. `---@field delta fun(self: time.Time<T>): time.Duration`.
fn function_fields<'a>(
    ladfile: &LadFile,
    functions: impl Iterator<Item = &'a LadFunction>,
) -> Vec<ClassField> {
    functions
        .filter_map(|function| {
            let signature = match lad_function_to_lua_function(ladfile, function) {
                Ok(signature) => signature,
                Err(err) => {
                    log::warn!(
                        "Error converting function {}: {err}. Ignoring function.",
                        function.identifier
                    );
                    return None;
                }
            };
            let parameters = signature
                .params
                .into_iter()
                .enumerate()
                .map(|(idx, param)| {
                    // called with `:`, the receiver is `self`
                    let name = if idx == 0 && function.receiver {
                        "self".to_owned()
                    } else {
                        param.name
                    };
                    (name, param.ty)
                })
                .collect();
            Some(ClassField {
                name: signature.name,
                ty: LuaType::Function(FunctionSignatureShort {
                    parameters,
                    return_type: Box::new(
                        signature
                            .returns
                            .into_iter()
                            .next()
                            .unwrap_or(LuaType::Primitive(LuaPrimitiveType::Nil)),
                    ),
                }),
                scope: FieldScope::Public,
                optional: false,
                description: signature.documentation,
            })
        })
        .collect()
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

/// The class of a type, with its fields, operators and variants, and the functions documented on its table.
fn type_to_lua_class(
    lad_type_id: &LadTypeId,
    lad_type: &ladfile::LadTypeDefinition,
    ladfile: &ladfile::LadFile,
) -> (LuaClass, Vec<FunctionSignature>) {
    let documentation = ladfile
        .get_type_documentation(lad_type_id)
        .map(ToOwned::to_owned);

    let mut lua_fields = vec![];
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
                parents.push(LuaType::Alias(String::from("ReflectReference")))
            }
            // if metadata.is_component {
            //     parents.push(String::from("ScriptComponentRegistration"))
            // }
            // if metadata.is_resource {
            //     parents.push(String::from("ScriptResourceRegistration"))
            // }
        }
    }

    let class = LuaClass {
        name,
        parents,
        fields: lua_fields,
        generics: vec![],
        documentation,
        exact: false,
        operators: lua_operators,
    };
    (class, lua_functions)
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

    let signature = |erase: bool| -> Result<(Vec<FunctionParam>, LuaType), anyhow::Error> {
        let kind = |kind: &LadTypeKind| match erase {
            true => erase_generics(kind, &function.generics),
            false => kind.clone(),
        };
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
                    lad_instance_to_lua_type(ladfile, &kind(&a.kind))?,
                    matches!(a.kind, LadTypeKind::Option(..)),
                    a.documentation.as_ref().map(|d| d.to_string()),
                ))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        let returns = lad_instance_to_lua_type(ladfile, &kind(&function.return_type.kind))?;
        Ok((params, returns))
    };
    let (params, returns) = signature(false)?;

    // the types a generic function takes can also be passed untyped, i.e. a registration found by name,
    // which LuaLS can only express as an overload, as overloads can't be generic
    let mut takes_types = false;
    for argument in &function.arguments {
        argument
            .kind
            .for_each(&mut |kind| takes_types |= matches!(kind, LadTypeKind::Type { .. }));
    }
    let overloads = match takes_types {
        true => {
            let (params, returns) = signature(true)?;
            vec![FunctionSignature {
                name: name.clone(),
                params,
                returns: vec![returns],
                ..Default::default()
            }]
        }
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

/// The kind as it is at runtime, without the given parameters of a generic function: types are what they are passed as,
/// i.e. a component registration, and values of a parameter are references.
fn erase_generics(kind: &LadTypeKind, generics: &[String]) -> LadTypeKind {
    let is_generic = |named: &LadNamedType| matches!(named, LadNamedType::Param { param } if generics.contains(param));
    let mut kind = kind.clone();
    match &kind {
        LadTypeKind::Type { of, erased } if is_generic(of) => return (**erased).clone(),
        kind if kind.named().is_some_and(is_generic) => {
            return LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference);
        }
        _ => {}
    }
    for child in kind.children_mut() {
        *child = erase_generics(child, generics);
    }
    kind
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

    fn visit_type_id(&mut self, lad_type_id: &LadTypeId) -> Self::Output {
        let ladfile = self.ladfile;
        // instantiations of declared generic types are named by their generic type, see `visit_generic`
        if ladfile.get_type_generics(lad_type_id).is_none() {
            Ok(LuaType::Alias(lua_class_name(ladfile, lad_type_id)))
        } else {
            Err(anyhow::anyhow!(
                "Generic fields are not supported: {lad_type_id}",
            ))
        }
    }

    fn visit_primitive(&mut self, primitive_kind: &ReflectionPrimitiveKind) -> Self::Output {
        Ok(lad_primitive_to_lua_type(primitive_kind))
    }

    fn visit_unknown(&mut self, _: &LadTypeId) -> Self::Output {
        Ok(LuaType::Any)
    }

    // TODO: currently ignores the possibility of an error type, we should have a custom class abstraction here
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
    if ladfile.get_type_generics(type_id).is_some() {
        return Err(anyhow::anyhow!(
            "Fields of generic types are not supported: {type_id}"
        ));
    }
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
    let enum_table = lad_type.identifier.clone();
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
                    parents: vec![LuaType::Alias(variant_parent.clone())],
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
                    fields.push(field(name, LuaType::Alias(lua_class_name(ladfile, type_id))));
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
            .insert("UnitType".into(), unit);

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
                "---@class ladfile_builder.EnumType.Unit : ladfile_builder.EnumVariantRef"
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
            rendered
                .contains("---@class ladfile_builder.OpaqueGenericType.Some<T> : EnumVariantRef"),
            "{rendered}"
        );
    }

    #[test]
    fn functions_of_generic_types_are_generic_fields_or_on_a_subclass() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let generic_id =
            LadTypeId::new_string_id("ladfile_builder::test::OpaqueGenericType".into());
        let plain = type_id(&file, "PlainStructType");
        let applied = |arg: LadTypeKind| LadNamedType::Generic {
            generic_type: generic_id.clone(),
            args: vec![arg],
        };
        let param = || LadTypeKind::Val(LadNamedType::Param { param: "T".into() });
        let of_plain = applied(LadTypeKind::Val(plain.clone().into()));
        let mut add = |name: &'static str, namespace: LadNamedType, ret: LadTypeKind| {
            let id = LadFunctionId::new_string_id(name.into());
            file.functions.insert(
                id.clone(),
                LadFunction {
                    namespace: LadFunctionNamespace::Type(namespace.clone()),
                    identifier: name.into(),
                    overload_index: None,
                    arguments: vec![arg("_self", LadTypeKind::Ref(namespace))],
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
        // a function of every instantiation, and one only the instantiation with `PlainStructType` has
        let value = add("value", applied(param()), param());
        let only = add(
            "only_on_plain",
            of_plain.clone(),
            LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool),
        );
        // and a generic function, generic over the type it is given
        let a = || LadNamedType::Param { param: "A".into() };
        let get = LadFunctionId::new_string_id("get".into());
        file.functions.insert(
            get.clone(),
            LadFunction {
                namespace: LadFunctionNamespace::Type(plain.clone().into()),
                identifier: "get".into(),
                overload_index: None,
                arguments: vec![arg(
                    "registration",
                    LadTypeKind::Type {
                        of: a(),
                        erased: Box::new(LadTypeKind::Val(plain.clone().into())),
                    },
                )],
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
        generic_type.associated_functions.push(value);
        generic_type.instantiations.push(ladfile::LadInstantiation {
            args: vec![LadTypeKind::Val(plain.clone().into())],
            associated_functions: vec![only],
            metadata: file.types[&plain].metadata.clone(),
        });
        let rendered = render(&file);

        // the generic function returns the type it is given, or a reference when given it untyped
        assert!(
            rendered.contains(
                "---@generic A\n---@param registration A\n---@overload fun(registration: ladfile_builder.PlainStructType): ReflectReference | nil\n---@return A | nil\nfunction PlainStructType.get(registration) end"
            ),
            "{rendered}"
        );

        // the function of every instantiation is a field of the generic class, so subclasses keep its parameter
        let class = block(
            &rendered,
            "---@class ladfile_builder.OpaqueGenericType<T> : ReflectReference",
        );
        assert!(
            class.contains("---@field  value fun(self: ladfile_builder.OpaqueGenericType<T>): T"),
            "{class}"
        );

        // the one of an instantiation is on its subclass, which names the instantiation
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
        assert_eq!(
            lad_instance_to_lua_type(&file, &LadTypeKind::Val(of_plain))
                .map(|ty| format!("{ty:?}"))
                .unwrap(),
            format!(
                "{:?}",
                LuaType::Alias("ladfile_builder.OpaqueGenericType.PlainStructType".into())
            )
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
    fn generic_types_nothing_declared_stay_untyped() {
        let file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let generic_struct = type_id(&file, "GenericStructType");
        assert!(
            file.generic_types
                .values()
                .all(|g| g.identifier != "GenericStructType")
        );

        // there is no class for the type to refer to
        assert!(lad_instance_to_lua_type(&file, &LadTypeKind::Val(generic_struct.into())).is_err());
        let rendered = render(&file);
        assert!(
            !rendered.contains("---@class GenericStructType"),
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
