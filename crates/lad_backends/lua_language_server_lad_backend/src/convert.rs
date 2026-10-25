use bevy_mod_scripting_bindings_domain::{ScriptOperatorNames, type_kinds};
use indexmap::{IndexMap, IndexSet};
use ladfile::{
    ArgumentVisitor, LadFile, LadFunction, LadTypeId, LadTypeKind, ReflectionPrimitiveKind,
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
    let bound_shared = bound_shared_functions(ladfile);
    let mut modules = vec![];
    let (classes, with_statics) = class_modules(ladfile, &bound_shared);
    modules.extend(classes);
    modules.extend(generic_type_modules(ladfile));
    modules.extend(fields_modules(ladfile, &bound_shared));

    // `import_namespace` needs the namespace classes, so they are built before the globals
    let namespaces = namespace_modules(ladfile, &with_statics);
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

/// The lua classes, and documented functions, of each type which is not an instantiation of a generic type.
///
/// `StaticTypeRef` is left out, its functions are shared by all types, see [`shared_type_modules`].
fn type_lua_classes(
    ladfile: &ladfile::LadFile,
    bound_shared: &IndexMap<LadTypeId, Vec<LadFunction>>,
) -> IndexMap<LadTypeId, TypeClasses> {
    let static_type_ref = static_type_ref_id(ladfile);
    ladfile
        .documented_types()
        .filter(|(type_id, _)| Some(*type_id) != static_type_ref)
        .map(|(type_id, lad_type)| {
            let bound = bound_shared
                .get(type_id)
                .map(Vec::as_slice)
                .unwrap_or_default();
            (
                type_id.clone(),
                type_to_lua_class(type_id, lad_type, ladfile, bound),
            )
        })
        .collect()
}

/// The modules of the classes of a type (see [`TypeClasses`]), followed by the modules of its enum variants.
///
/// Also returns the types which have a statics class, the others have no static functions or variants.
fn class_modules(
    ladfile: &ladfile::LadFile,
    bound_shared: &IndexMap<LadTypeId, Vec<LadFunction>>,
) -> (Vec<LuaModule>, IndexSet<LadTypeId>) {
    let mut modules = vec![];
    let mut with_statics = IndexSet::new();
    for (type_id, classes) in type_lua_classes(ladfile, bound_shared) {
        let Some(lad_type) = ladfile.types.get(&type_id) else {
            continue;
        };
        modules.push(LuaModule {
            name: lad_type.identifier.to_owned(),
            table: lad_type.identifier.to_owned(),
            namespace: lad_type.namespace.clone(),
            class: Some(classes.instance),
            functions: classes.instance_functions,
            ..Default::default()
        });
        if !classes.static_functions.is_empty() || !classes.statics.fields.is_empty() {
            modules.push(LuaModule {
                name: classes.statics.name.clone(),
                table: statics_table(&lad_type.identifier),
                namespace: lad_type.namespace.clone(),
                class: Some(classes.statics),
                functions: classes.static_functions,
                ..Default::default()
            });
            with_statics.insert(type_id.clone());
        }
        modules.extend(enum_variant_modules(ladfile, &type_id, lad_type));
    }
    (modules, with_statics)
}

/// A class per generic type, with its parameters as generics.
///
/// Generic types are opaque, so a class with the parameters is all there is to them, except for the kinds of types
/// (i.e. `ComponentType<A>`), which carry the functions shared by all types of the kind, see [`shared_kind_fields`].
fn generic_type_modules(ladfile: &ladfile::LadFile) -> Vec<LuaModule> {
    ladfile
        .generic_types
        .iter()
        .map(|(id, generic_type)| {
            let (parents, fields) = match type_kind(id) {
                Some(kind) => (vec![], shared_kind_fields(ladfile, kind)),
                None => (vec![REFLECT_REFERENCE_CLASS.to_string()], vec![]),
            };
            LuaModule {
                name: generic_type.identifier.clone(),
                table: generic_type.identifier.clone(),
                namespace: generic_type.namespace.clone(),
                class: Some(LuaClass {
                    name: qualified(&generic_type.namespace, &generic_type.identifier),
                    parents,
                    fields,
                    generics: generic_type.params.clone(),
                    documentation: generic_type.documentation.clone(),
                    ..Default::default()
                }),
                type_only: true,
                ..Default::default()
            }
        })
        .collect()
}

/// The fields class of each type or variant constructors take fields for, plus `no_fields` for the rest.
fn fields_modules(
    ladfile: &ladfile::LadFile,
    bound_shared: &IndexMap<LadTypeId, Vec<LadFunction>>,
) -> Vec<LuaModule> {
    // the tables of fields constructors take, which get a class each
    let mut fields_references = FieldsReferences::default();
    for function in ladfile
        .functions
        .values()
        .chain(bound_shared.values().flatten())
    {
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

/// The classes of a type: one for its values, with their fields, operators and methods,
/// and one for the type itself, with its static functions and variants. See [`TypeClasses`].
fn type_to_lua_class(
    lad_type_id: &LadTypeId,
    lad_type: &ladfile::LadTypeDefinition,
    ladfile: &ladfile::LadFile,
    bound_shared: &[LadFunction],
) -> TypeClasses {
    let documentation = ladfile
        .get_type_documentation(lad_type_id)
        .map(ToOwned::to_owned);

    let mut lua_fields = vec![];
    let mut lua_functions = vec![];
    let mut static_functions = vec![];
    let mut static_fields = vec![];
    let mut lua_operators = vec![];
    let mut parents = vec![];
    let name = lua_class_name(ladfile, lad_type_id);

    {
        {
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
                    // functions are found through the type and through its values alike, so they are declared once
                    // on the statics class, which the class of the values extends
                    static_functions.push(lua_function);
                }
            }

            // the shared functions whose signature depends on the type, i.e. `new` taking its fields
            for function in bound_shared {
                match lad_function_to_lua_function(ladfile, function) {
                    Ok(lua_function) => static_functions.push(lua_function),
                    Err(err) => log::warn!(
                        "Error converting function {} for type {lad_type_id}: {err}. Ignoring function.",
                        function.identifier
                    ),
                }
            }

            // the variants of an enum are accessible as `Enum.Variant`, unless a function with the same name takes priority.
            // Each variant has a class of its own, see `enum_variant_modules`
            if let ladfile::LadTypeLayout::Enum(variants) = &lad_type.layout {
                for variant in variants {
                    let variant_name = lad_variant_name(variant);
                    if lua_functions
                        .iter()
                        .chain(&static_functions)
                        .any(|f| f.name == variant_name)
                    {
                        continue;
                    }
                    static_fields.push(ClassField {
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
    }

    let statics = LuaClass {
        name: statics_class_name(&name),
        parents: vec![],
        fields: static_fields,
        generics: vec![],
        documentation: Some(format!(
            "The functions and variants of `{name}`, as reached through the type, i.e. `{}.function()` in scripts.",
            lad_type.identifier
        )),
        exact: false,
        operators: vec![],
    };
    if !static_functions.is_empty() || !statics.fields.is_empty() {
        parents.push(statics.name.clone());
    }
    let instance = LuaClass {
        name,
        parents,
        fields: lua_fields,
        generics: vec![],
        documentation,
        exact: false,
        operators: lua_operators,
    };
    TypeClasses {
        instance,
        instance_functions: lua_functions,
        statics,
        static_functions,
    }
}

/// The classes of a type, see [`type_to_lua_class`].
///
/// What scripts get for a type, i.e. `import_namespace("math").Vec3`, is a union of its statics class and
/// the classes of the functions shared by all types of its kinds, see [`type_table_type`].
struct TypeClasses {
    /// The class of values of the type
    instance: LuaClass,
    /// The methods, called on values
    instance_functions: Vec<FunctionSignature>,
    /// The class of the type itself
    statics: LuaClass,
    /// The functions called on the type
    static_functions: Vec<FunctionSignature>,
}

/// The class of the static functions and variants of the type with the given class, i.e. `math.Vec3.Statics`.
fn statics_class_name(class_name: &str) -> String {
    format!("{class_name}.Statics")
}

/// The local table the static functions of a type are declared on, i.e. `Vec3_statics`.
fn statics_table(identifier: &str) -> String {
    format!("{identifier}_statics")
}

/// The type of the table scripts get for a type, i.e. `import_namespace("math").Vec3`:
/// its statics class, and the class of the shared functions of each kind of type it is, i.e. `ComponentType<transform.Transform>`.
///
/// LuaLS only resolves the generic parameter of the shared classes when they are named directly, not through a subclass,
/// so this is a union rather than a class extending them.
fn type_table_type(ladfile: &LadFile, type_id: &LadTypeId, has_statics: bool) -> LuaType {
    let name = lua_class_name(ladfile, type_id);
    let mut types = vec![];
    if has_statics {
        types.push(LuaType::Alias(statics_class_name(&name)));
    }
    if let Some(lad_type) = ladfile.types.get(type_id) {
        for kind in type_kinds::ALL {
            if lad_type.metadata.is_of_kind(kind) {
                types.push(LuaType::Generic {
                    name: kind.to_owned(),
                    args: vec![LuaType::Alias(name.clone())],
                });
            }
        }
    }
    LuaType::Union(types)
}

/// The kind of types, one of [`type_kinds`], the generic type with the given id stands for, if any.
fn type_kind(generic_type: &LadTypeId) -> Option<&'static str> {
    type_kinds::ALL
        .into_iter()
        .find(|kind| *generic_type == LadTypeId::new_string_id((*kind).into()))
}

/// The type whose functions are shared by all types, `StaticTypeRef`.
fn static_type_ref_id(ladfile: &LadFile) -> Option<&LadTypeId> {
    ladfile
        .types
        .iter()
        .find(|(_, lad_type)| {
            lad_type.metadata.mapped_to_primitive_kind
                == Some(ReflectionPrimitiveKind::StaticTypeRef)
        })
        .map(|(type_id, _)| type_id)
}

/// The functions shared by all types, registered on `StaticTypeRef`. Their first argument is the type, filled in by BMS.
fn shared_functions(ladfile: &LadFile) -> Vec<&LadFunction> {
    static_type_ref_id(ladfile)
        .and_then(|type_id| ladfile.types.get(type_id))
        .map(|lad_type| {
            lad_type
                .associated_functions
                .iter()
                .filter_map(|id| ladfile.functions.get(id))
                .collect()
        })
        .unwrap_or_default()
}

/// The kinds of types a shared function is called on, i.e. `[ComponentType]` for `get_on`, from the generic type of its first argument.
fn receiver_kinds(_ladfile: &LadFile, function: &LadFunction) -> Vec<&'static str> {
    let mut kinds = vec![];
    if let Some(receiver) = function.arguments.first() {
        receiver.kind.for_each(&mut |kind| {
            if let LadTypeKind::Generic { generic_type, .. } = kind
                && let Some(kind) = type_kind(generic_type)
            {
                kinds.push(kind);
            }
        });
    }
    if kinds.is_empty() {
        kinds.push(type_kinds::TYPE);
    }
    kinds
}

/// Whether the signature of a shared function can't be expressed with LuaLS generics, i.e. it takes the fields of the type,
/// in which case it is bound to each type instead, see [`bind_shared_function`].
fn needs_binding(function: &LadFunction) -> bool {
    let mut fields = false;
    for arg in function
        .arguments
        .iter()
        .chain(std::iter::once(&function.return_type))
    {
        arg.kind.for_each(&mut |kind| {
            fields |= matches!(kind, LadTypeKind::Fields { of, .. } if matches!(**of, LadTypeKind::Param(_)))
        });
    }
    fields
}

/// A shared function as seen on the given type: without the type argument, with its generic parameters replaced by the type.
fn bind_shared_function(function: &LadFunction, type_id: &LadTypeId) -> LadFunction {
    let mut bound = function.clone();
    if !bound.arguments.is_empty() {
        bound.arguments.remove(0);
    }
    for arg in bound
        .arguments
        .iter_mut()
        .chain(std::iter::once(&mut bound.return_type))
    {
        arg.kind = bind_kind(&arg.kind, type_id);
    }
    bound.generics.clear();
    bound.namespace = ladfile::LadFunctionNamespace::Type(type_id.clone());
    bound
}

/// Replaces the generic parameters in the kind with the given type.
fn bind_kind(kind: &LadTypeKind, type_id: &LadTypeId) -> LadTypeKind {
    let bind = |inner: &LadTypeKind| Box::new(bind_kind(inner, type_id));
    let bind_all = |inner: &[LadTypeKind]| inner.iter().map(|i| bind_kind(i, type_id)).collect();
    match kind {
        LadTypeKind::Param(_) => LadTypeKind::Val(type_id.clone()),
        LadTypeKind::Fields { of, variant } => LadTypeKind::Fields {
            of: bind(of),
            variant: variant.clone(),
        },
        LadTypeKind::Generic { generic_type, args } => LadTypeKind::Generic {
            generic_type: generic_type.clone(),
            args: bind_all(args),
        },
        LadTypeKind::Option(inner) => LadTypeKind::Option(bind(inner)),
        LadTypeKind::Vec(inner) => LadTypeKind::Vec(bind(inner)),
        LadTypeKind::HashSet(inner) => LadTypeKind::HashSet(bind(inner)),
        LadTypeKind::InteropResult(inner) => LadTypeKind::InteropResult(bind(inner)),
        LadTypeKind::Array(inner, size) => LadTypeKind::Array(bind(inner), *size),
        LadTypeKind::HashMap(key, value) => LadTypeKind::HashMap(bind(key), bind(value)),
        LadTypeKind::Tuple(inner) => LadTypeKind::Tuple(bind_all(inner)),
        LadTypeKind::Union(inner) => LadTypeKind::Union(bind_all(inner)),
        LadTypeKind::TupleWithRest { head, rest } => LadTypeKind::TupleWithRest {
            head: bind_all(head),
            rest: bind(rest),
        },
        other => other.clone(),
    }
}

/// The shared functions which need binding (see [`needs_binding`]), bound to each documented type whose kind they apply to.
///
/// Functions of the type itself with the same name take priority, as they do in scripts.
fn bound_shared_functions(ladfile: &LadFile) -> IndexMap<LadTypeId, Vec<LadFunction>> {
    let shared = shared_functions(ladfile)
        .into_iter()
        .filter(|function| needs_binding(function))
        .collect::<Vec<_>>();
    let static_type_ref = static_type_ref_id(ladfile);
    ladfile
        .documented_types()
        .filter(|(type_id, _)| Some(*type_id) != static_type_ref)
        .map(|(type_id, lad_type)| {
            let own = |name: &str| {
                lad_type
                    .associated_functions
                    .iter()
                    .filter_map(|id| ladfile.functions.get(id))
                    .any(|function| function.identifier == name)
            };
            let bound = shared
                .iter()
                .filter(|function| {
                    receiver_kinds(ladfile, function)
                        .iter()
                        .any(|kind| lad_type.metadata.is_of_kind(kind))
                })
                .filter(|function| !own(&function.identifier))
                .map(|function| bind_shared_function(function, type_id))
                .collect();
            (type_id.clone(), bound)
        })
        .collect()
}

/// The functions shared by all types of a kind, as fields of the kind's generic class, i.e. `get_on` on `ComponentType<A>`.
///
/// The functions are fields, so the class's generic parameter, the type, is resolved where the class is named
/// with its argument, see [`type_table_type`]. Shared functions which need binding are on each type instead.
fn shared_kind_fields(ladfile: &LadFile, kind: &str) -> Vec<ClassField> {
    shared_functions(ladfile)
        .into_iter()
        .filter(|function| {
            !needs_binding(function) && receiver_kinds(ladfile, function).contains(&kind)
        })
        .filter_map(|function| {
            // the type is filled in by BMS, scripts call the function without it
            let mut without_type = function.clone();
            if !without_type.arguments.is_empty() {
                without_type.arguments.remove(0);
            }
            match lad_function_to_lua_function(ladfile, &without_type) {
                Ok(signature) => Some(ClassField {
                    name: signature.name,
                    ty: LuaType::Function(FunctionSignatureShort {
                        parameters: signature
                            .params
                            .into_iter()
                            .map(|param| (param.name, param.ty))
                            .collect(),
                        return_type: Box::new(
                            signature.returns.into_iter().next().unwrap_or(LuaType::Any),
                        ),
                    }),
                    scope: FieldScope::Public,
                    optional: false,
                    description: signature.documentation,
                }),
                Err(err) => {
                    log::warn!(
                        "Error converting shared function {}: {err}. Ignoring function.",
                        function.identifier
                    );
                    None
                }
            }
        })
        .collect()
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

    let signature = |erase_generics: bool| -> Result<_, anyhow::Error> {
        let mut visitor = LuaTypeVisitor {
            ladfile,
            erase_generics,
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
                    visitor.visit(&a.kind)?,
                    matches!(a.kind, LadTypeKind::Option(..)),
                    a.documentation.as_ref().map(|d| d.to_string()),
                ))
            })
            .collect::<Result<Vec<_>, anyhow::Error>>()?;
        let returns = visitor.visit(&function.return_type.kind)?;
        Ok((params, returns))
    };
    let (params, returns) = signature(false)?;

    // the type of a generic parameter is only known when it is given by a type, i.e. `import_namespace("math").Vec3`,
    // types looked up at runtime (i.e. with `get_type_by_name`) give values of an unknown type
    let mut takes_types = false;
    for arg in &function.arguments {
        arg.kind.for_each(&mut |kind| {
            takes_types |= matches!(kind, LadTypeKind::Generic { generic_type, .. } if type_kind(generic_type).is_some());
        });
    }
    let mut overloads = vec![];
    if takes_types {
        let (params, returns) = signature(true)?;
        overloads.push(FunctionSignature {
            name: name.clone(),
            params,
            returns: vec![returns],
            ..Default::default()
        });
    }

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
    LuaTypeVisitor {
        ladfile,
        erase_generics: false,
    }
    .visit(lad_type)
}

/// Converts LAD types to Lua types.
pub struct LuaTypeVisitor<'a> {
    /// The file the types are from
    pub ladfile: &'a LadFile,
    /// Whether generic parameters are replaced by what they are at runtime, i.e. for types looked up at runtime
    pub erase_generics: bool,
}

impl ArgumentVisitor for LuaTypeVisitor<'_> {
    type Output = Result<LuaType, anyhow::Error>;

    fn visit_type_id(&mut self, lad_type_id: &LadTypeId) -> Self::Output {
        let ladfile = self.ladfile;
        if let Some((generic_type, args)) = ladfile.get_generic_instance(lad_type_id) {
            Ok(LuaType::Generic {
                name: qualified(&generic_type.namespace, &generic_type.identifier),
                args: args
                    .iter()
                    .map(|arg| lad_generic_argument_to_lua_type(ladfile, &arg.type_id))
                    .collect(),
            })
        } else if ladfile.get_type_generics(lad_type_id).is_none() {
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
        // a type of a kind, or what is passed for it at runtime: the generic signature takes the type of the kind,
        // i.e. `ComponentType<A>`, the erased overload what is passed at runtime, the other arms
        let is_kind = |kind: &LadTypeKind| matches!(kind, LadTypeKind::Generic { generic_type, .. } if type_kind(generic_type).is_some());
        let has_kind = inner.iter().any(is_kind);
        let erase = self.erase_generics;
        let inner = inner
            .iter()
            .filter(|kind| !has_kind || is_kind(kind) != erase)
            .map(|kind| self.visit(kind))
            .collect::<Result<_, _>>()?;
        Ok(LuaType::Union(inner))
    }

    fn visit_fields(&mut self, of: &LadTypeKind, variant: Option<&str>) -> Self::Output {
        let ladfile = self.ladfile;
        let (LadTypeKind::Val(type_id) | LadTypeKind::Ref(type_id) | LadTypeKind::Mut(type_id)) =
            of
        else {
            // LuaLS can't name the fields class of a generic parameter, see `bind_shared_function`
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

    fn visit_param(&mut self, param: &str) -> Self::Output {
        Ok(if self.erase_generics {
            LuaType::Alias(REFLECT_REFERENCE_CLASS.to_owned())
        } else {
            LuaType::Alias(param.to_owned())
        })
    }

    fn visit_generic(&mut self, generic_type: &LadTypeId, args: &[LadTypeKind]) -> Self::Output {
        let Some(generic) = self.ladfile.generic_types.get(generic_type) else {
            return Ok(LuaType::Alias(REFLECT_REFERENCE_CLASS.to_owned()));
        };
        Ok(LuaType::Generic {
            name: qualified(&generic.namespace, &generic.identifier),
            args: args
                .iter()
                .map(|arg| self.visit(arg))
                .collect::<Result<_, _>>()?,
        })
    }
}

/// The class every reflected value extends.
const REFLECT_REFERENCE_CLASS: &str = "ReflectReference";

/// The argument of a generic type can be any type, including primitives and types the file doesn't know about.
fn lad_generic_argument_to_lua_type(ladfile: &LadFile, type_id: &LadTypeId) -> LuaType {
    if let Some(primitive) = ladfile.primitive_kind(type_id) {
        return lad_primitive_to_lua_type(primitive);
    }
    if !ladfile.types.contains_key(type_id) {
        return LuaType::Any;
    }
    lad_instance_to_lua_type(ladfile, &ladfile::LadTypeKind::Val(type_id.clone()))
        .unwrap_or(LuaType::Any)
}

/// The class scripts see for enum variants, which variant classes extend: the class of `EnumVariantRef`, i.e. `bms.EnumVariantRef`.
fn enum_variant_class(ladfile: &LadFile) -> String {
    ladfile
        .documented_types()
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
            if let LadTypeKind::Fields { of, variant } = kind
                && let LadTypeKind::Val(type_id)
                | LadTypeKind::Ref(type_id)
                | LadTypeKind::Mut(type_id) = &**of
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
    // variants are reached through the type, so their tables are fields of its statics table
    let enum_table = statics_table(&lad_type.identifier);
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
                    parents: vec![variant_parent.clone()],
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
fn namespace_modules(
    ladfile: &ladfile::LadFile,
    with_statics: &IndexSet<LadTypeId>,
) -> Vec<LuaModule> {
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
                    let has_statics = with_statics.contains(type_id);
                    fields.push(field(name, type_table_type(ladfile, type_id, has_statics)));
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
            LadFunctionNamespace::Type(plain.clone()),
            vec![arg(
                "payload",
                LadTypeKind::Fields {
                    of: Box::new(LadTypeKind::Val(plain.clone())),
                    variant: None,
                },
            )],
            LadTypeKind::Val(plain),
        );

        // the enum level constructor, `EnumType.new{ variant = ..., ... }`
        let enum_type = type_id(&file, "EnumType");
        add_function(
            &mut file,
            LadFunctionNamespace::Type(enum_type.clone()),
            vec![arg(
                "payload",
                LadTypeKind::Fields {
                    of: Box::new(LadTypeKind::Val(enum_type.clone())),
                    variant: None,
                },
            )],
            LadTypeKind::Val(enum_type.clone()),
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
                arg("variant", LadTypeKind::Val(variant_ref.clone())),
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
                    type_id: enum_type.clone(),
                    variant: variant.into(),
                },
                vec![
                    arg("variant", LadTypeKind::Val(variant_ref.clone())),
                    arg(
                        "payload",
                        LadTypeKind::Fields {
                            of: Box::new(LadTypeKind::Val(enum_type.clone())),
                            variant: Some(variant.into()),
                        },
                    ),
                ],
                LadTypeKind::Val(enum_type.clone()),
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
    fn shared_functions_are_generic_classes_or_bound_to_each_type() {
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let plain = type_id(&file, "PlainStructType");
        file.types[&plain].metadata.is_component = true;

        // `StaticTypeRef`, carrying the functions shared by all types
        let static_type_ref = LadTypeId::new_string_id("StaticTypeRef".into());
        let mut definition = file.types[&type_id(&file, "UnitType")].clone();
        definition.identifier = "StaticTypeRef".into();
        definition.namespace = String::new();
        definition.layout = LadTypeLayout::Opaque;
        definition.associated_functions = vec![];
        definition.metadata.mapped_to_primitive_kind = Some(ReflectionPrimitiveKind::StaticTypeRef);
        file.types.insert(static_type_ref.clone(), definition);

        // the kinds of types are generic types, declared by the builder
        for kind in [type_kinds::TYPE, type_kinds::COMPONENT_TYPE] {
            file.generic_types.insert(
                LadTypeId::new_string_id(kind.into()),
                ladfile::LadGenericType {
                    identifier: kind.into(),
                    crate_: None,
                    path: kind.into(),
                    namespace: String::new(),
                    params: vec!["A".into()],
                    documentation: None,
                },
            );
        }
        let param = || LadTypeKind::Param("A".into());
        // the type of `A`, or what is passed at runtime
        let type_of = |kind: &str, carrier| {
            LadTypeKind::Union(vec![
                LadTypeKind::Generic {
                    generic_type: LadTypeId::new_string_id(kind.to_owned().into()),
                    args: vec![param()],
                },
                carrier,
            ])
        };
        let static_type = || LadTypeKind::Primitive(ReflectionPrimitiveKind::StaticTypeRef);
        let unit = type_id(&file, "UnitType");
        let mut add = |namespace: LadFunctionNamespace,
                       name: &'static str,
                       arguments: Vec<LadArgument>,
                       return_kind: LadTypeKind| {
            let id = LadFunctionId::new_string_id(format!("{namespace:?}::{name}"));
            if let LadFunctionNamespace::Type(type_id) = &namespace {
                file.types[type_id].associated_functions.push(id.clone());
            }
            file.functions.insert(
                id,
                LadFunction {
                    namespace,
                    identifier: name.into(),
                    overload_index: None,
                    arguments,
                    return_type: arg("return", return_kind),
                    documentation: None,
                    metadata: LadFunctionMetadata {
                        operator: None,
                        misc: Default::default(),
                    },
                    receiver: false,
                    generics: vec!["A".into()],
                },
            );
        };
        let entity = || {
            arg(
                "entity",
                LadTypeKind::Primitive(ReflectionPrimitiveKind::U64),
            )
        };
        add(
            LadFunctionNamespace::Type(static_type_ref.clone()),
            "new",
            vec![
                arg("type", type_of(type_kinds::TYPE, static_type())),
                arg(
                    "payload",
                    LadTypeKind::Fields {
                        of: Box::new(param()),
                        variant: None,
                    },
                ),
            ],
            param(),
        );
        add(
            LadFunctionNamespace::Type(static_type_ref.clone()),
            "get_on",
            vec![
                arg("type", type_of(type_kinds::COMPONENT_TYPE, static_type())),
                entity(),
            ],
            LadTypeKind::Option(Box::new(param())),
        );
        // and a generic function taking a type, which can also be looked up at runtime
        add(
            LadFunctionNamespace::Global,
            "get_component",
            vec![
                entity(),
                arg(
                    "registration",
                    type_of(type_kinds::COMPONENT_TYPE, LadTypeKind::Val(unit)),
                ),
            ],
            LadTypeKind::Option(Box::new(param())),
        );
        let rendered = render(&file);

        // functions which LuaLS generics can express are on the class of their kind, once
        let component_type = block(&rendered, "---@class ComponentType<A>");
        assert!(
            component_type.contains("---@field  get_on fun(entity: integer): A | nil"),
            "{component_type}"
        );
        assert!(!rendered.contains("function PlainStructType_statics.get_on"));

        // the ones taking the fields of the type are bound to each type
        let new = block(
            &rendered,
            "---@param payload ladfile_builder.PlainStructType.Fields",
        );
        assert!(
            new.contains("---@return ladfile_builder.PlainStructType")
                && new.contains("function PlainStructType_statics.new(payload)"),
            "{new}"
        );

        // a type is its statics and the shared functions of each kind it is
        let namespace = block(&rendered, "---@class namespace.ladfile_builder");
        assert!(
            namespace.contains("---@field  PlainStructType ladfile_builder.PlainStructType.Statics | Type<ladfile_builder.PlainStructType> | ComponentType<ladfile_builder.PlainStructType>\n"),
            "{namespace}"
        );
        assert!(
            !namespace.contains("ComponentType<ladfile_builder.UnitType>"),
            "{namespace}"
        );

        // generic functions infer the type from the type passed, the untyped overload covers types looked up at runtime
        let get_component = block(&rendered, "---@generic A");
        assert!(
            get_component.contains("---@param registration ComponentType<A>")
                && get_component.contains("---@overload fun(entity: integer, registration: ladfile_builder.UnitType): ReflectReference | nil")
                && get_component.contains("---@return A | nil"),
            "{get_component}"
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

        // a type is its statics, if it has any, and the shared functions of every type
        let namespace = block(&rendered, "---@class namespace.ladfile_builder");
        assert!(
            namespace.contains("---@field  TupleStructType Type<ladfile_builder.TupleStructType>\n")
                && namespace.contains("---@field  EnumType ladfile_builder.EnumType.Statics | Type<ladfile_builder.EnumType>\n"),
            "{namespace}"
        );
        assert!(!namespace.contains("---@field  UnitType"), "{namespace}");
        let other = block(&rendered, "---@class namespace.other_crate");
        assert!(
            other.contains("---@field  UnitType Type<other_crate.UnitType>"),
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

        // variants are reached through the type, not its values
        let enum_class = block(&rendered, "---@class ladfile_builder.EnumType.Statics");
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
            new.contains("function EnumType_statics.Struct.new(variant,payload)"),
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
        let mut file = ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap();
        let instantiation = type_id(&file, "OpaqueGenericType");

        // an instantiation with an instantiation as its argument, and one with an argument the file doesn't know
        let mut add_instantiation = |id: &str, argument: LadTypeId| {
            let mut definition = file.types[&instantiation].clone();
            definition.generics[0].type_id = argument;
            let id = LadTypeId::new_string_id(id.to_owned().into());
            file.types.insert(id.clone(), definition);
            id
        };
        let nested = add_instantiation("nested", instantiation.clone());
        let unknown = add_instantiation(
            "unknown",
            LadTypeId::new_string_id("not::in::the::file".into()),
        );

        assert_eq!(
            lad_instance_to_lua_type(&file, &LadTypeKind::Val(nested))
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
            lad_instance_to_lua_type(&file, &LadTypeKind::Val(unknown))
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
        assert!(file.types[&generic_struct].generic_type.is_none());

        // there is no class for the type to refer to
        assert!(lad_instance_to_lua_type(&file, &LadTypeKind::Val(generic_struct)).is_err());
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
