//! The classes of generic types (see [`generic_type_modules`]), and the untyped overloads of generic functions (see [`erase_generics`]).

use indexmap::IndexMap;
use ladfile::{
    LadFile, LadFunction, LadFunctionNamespace, LadGenericType, LadInstantiation, LadNamedType,
    LadTypeKind, ReflectionPrimitiveKind,
};

use crate::lua_declaration_file::{ClassField, FieldScope, LuaClass, LuaModule, LuaType};

use super::{
    enum_variant_class, lad_function_to_lua_function, lad_instance_to_lua_type, lad_variant_name,
    layout_fields, qualified, variant_class_name,
};

/// A class per generic type, with its parameters as generics and its fields in terms of them, i.e. `---@field context T`.
///
/// Like enums, generic enums have a field per variant, of the variant's class, which is generic over the same parameters.
/// The functions of an instantiation are on a subclass of the generic class per instantiation, see [`instantiation_class_name`].
pub(super) fn generic_type_modules(ladfile: &LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for generic_type in ladfile.generic_types.values() {
        let name = qualified(&generic_type.namespace, &generic_type.identifier);
        let mut fields = layout_fields(ladfile, &generic_type.layout);
        let (variant_fields, variant_modules) = variant_modules(ladfile, generic_type, &name);
        fields.extend(variant_fields);
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
        modules.extend(instantiation_modules(ladfile, generic_type, &name));
    }
    modules
}

/// The fields of the class of a generic enum for its variants, and the class of each variant, generic over the same
/// parameters.
fn variant_modules(
    ladfile: &LadFile,
    generic_type: &LadGenericType,
    name: &str,
) -> (Vec<ClassField>, Vec<LuaModule>) {
    let ladfile::LadTypeLayout::Enum(variants) = &generic_type.layout else {
        return (vec![], vec![]);
    };
    let params: Vec<_> = generic_type
        .params
        .iter()
        .map(|param| LuaType::Alias(param.clone()))
        .collect();
    let mut fields = vec![];
    let mut modules = vec![];
    for variant in variants.iter().map(lad_variant_name) {
        let variant_class = variant_class_name(name, variant);
        fields.push(ClassField {
            name: variant.to_string(),
            ty: LuaType::Generic {
                name: variant_class.clone(),
                args: params.clone(),
            },
            scope: FieldScope::Public,
            optional: false,
            description: None,
        });
        modules.push(LuaModule {
            name: variant_class.clone(),
            table: format!("{}.{variant}", generic_type.identifier),
            namespace: generic_type.namespace.clone(),
            class: Some(LuaClass {
                name: variant_class,
                parents: vec![LuaType::Alias(enum_variant_class(ladfile))],
                generics: generic_type.params.clone(),
                documentation: Some(format!("The `{variant}` variant of `{name}`.")),
                ..Default::default()
            }),
            type_only: true,
            ..Default::default()
        });
    }
    (fields, modules)
}

/// The subclasses of the instantiations with functions of their own, i.e. `pause` on `time.Time.Virtual : time.Time<time.Virtual>`,
/// and of their variants with functions, see [`instantiation_variant_functions`].
fn instantiation_modules(
    ladfile: &LadFile,
    generic_type: &LadGenericType,
    name: &str,
) -> Vec<LuaModule> {
    let mut modules = vec![];
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
        // variants with functions of their own get a class too, i.e. `asset.Handle.Image.Strong : asset.Handle.Strong<render.Image>`
        let mut fields = vec![];
        let mut variant_modules = vec![];
        for (variant, functions) in
            instantiation_variant_functions(ladfile, generic_type, instantiation)
        {
            let variant_class = variant_class_name(&class_name, variant);
            fields.push(ClassField {
                name: variant.to_string(),
                ty: LuaType::Alias(variant_class.clone()),
                scope: FieldScope::Public,
                optional: false,
                description: None,
            });
            let functions = functions
                .into_iter()
                .filter_map(|function| match lad_function_to_lua_function(ladfile, function) {
                    Ok(mut lua_function) => {
                        // the receiver is the variant itself
                        if let Some(receiver) = lua_function.params.first_mut() {
                            receiver.ty = LuaType::Alias(variant_class.clone());
                        }
                        Some(lua_function)
                    }
                    Err(err) => {
                        log::warn!(
                            "Error converting function {} for variant {variant_class}: {err}. Ignoring function.",
                            function.identifier
                        );
                        None
                    }
                })
                .collect();
            variant_modules.push(LuaModule {
                name: variant_class.clone(),
                table: variant_class.replace('.', "_"),
                namespace: generic_type.namespace.clone(),
                class: Some(LuaClass {
                    name: variant_class,
                    parents: vec![LuaType::Generic {
                        name: variant_class_name(name, variant),
                        args: args.clone(),
                    }],
                    documentation: Some(format!("The `{variant}` variant of `{class_name}`.")),
                    ..Default::default()
                }),
                functions,
                ..Default::default()
            });
        }
        modules.push(LuaModule {
            name: class_name.clone(),
            table: class_name.replace('.', "_"),
            namespace: generic_type.namespace.clone(),
            class: Some(LuaClass {
                name: class_name,
                parents: vec![LuaType::Generic {
                    name: name.to_owned(),
                    args,
                }],
                fields,
                generics: vec![],
                documentation: generic_type.documentation.clone(),
                ..Default::default()
            }),
            functions,
            ..Default::default()
        });
        modules.extend(variant_modules);
    }
    modules
}

/// The functions on the variants of the instantiation, by variant, i.e. `new` on `Strong` of `Handle<Image>`, see [`LadFunctionNamespace::Variant`].
fn instantiation_variant_functions<'a>(
    ladfile: &'a LadFile,
    generic_type: &LadGenericType,
    instantiation: &LadInstantiation,
) -> IndexMap<&'a str, Vec<&'a LadFunction>> {
    let mut by_variant = IndexMap::<_, Vec<_>>::new();
    for function in ladfile.functions.values() {
        if let LadFunctionNamespace::Variant {
            type_id:
                LadNamedType::Generic {
                    generic_type: id,
                    args,
                },
            variant,
        } = &function.namespace
            && args == &instantiation.args
            && ladfile
                .generic_types
                .get(id)
                .is_some_and(|g| std::ptr::eq(g, generic_type))
        {
            by_variant
                .entry(variant.as_ref())
                .or_default()
                .push(function);
        }
    }
    by_variant
}

/// The class of an instantiation of a generic type with functions of its own or on its variants, i.e. `time.Time.Virtual`
/// for `Time<Virtual>`, which extends the generic class applied to the arguments. Instantiations without functions have none.
///
/// It is named after the identifiers of the arguments, joined by `_` (`Time.Virtual`, `Foo.Image_Mesh`), unless they aren't
/// types of the file or clash with a variant. Its table is the class name with `_` for `.`, a local table, as the generic class
/// has none to nest it in.
pub(super) fn instantiation_class_name(
    ladfile: &LadFile,
    generic_type: &LadGenericType,
    instantiation: &LadInstantiation,
) -> Option<String> {
    if instantiation.associated_functions.is_empty()
        && instantiation_variant_functions(ladfile, generic_type, instantiation).is_empty()
    {
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

/// The kind as it is at runtime, without the given parameters of a generic function: types are what they are passed as,
/// i.e. a component registration, and values of a parameter are references.
pub(super) fn erase_generics(kind: &LadTypeKind, generics: &[String]) -> LadTypeKind {
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
