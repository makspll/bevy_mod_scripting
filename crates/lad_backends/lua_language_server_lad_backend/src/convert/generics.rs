//! The classes of generic types (see [`generic_type_modules`]), and the untyped overloads of generic functions (see [`erase_generics`]).

use ladfile::{
    LadFile, LadFunction, LadGenericType, LadInstantiation, LadNamedType, LadTypeId, LadTypeKind,
    ReflectionPrimitiveKind,
};

use crate::lua_declaration_file::{
    ClassField, FieldScope, FunctionSignatureShort, LuaClass, LuaModule, LuaPrimitiveType, LuaType,
};

use super::{
    enum_variant_class, lad_function_to_lua_function, lad_instance_to_lua_type, lad_variant_name,
    layout_fields, qualified, variant_class_name,
};

/// A class per generic type, with its parameters as generics and its fields in terms of them, i.e. `---@field context T`.
///
/// Like enums, generic enums have a field per variant, of the variant's class, which is generic over the same parameters.
/// The functions of every instantiation are fields too, i.e. `---@field delta fun(self: time.Time<T>): time.Duration`, as LuaLS
/// keeps resolving the parameters of fields in subclasses, but not of methods. The functions only some instantiations have
/// are on a subclass per instantiation, see [`instantiation_class_name`].
pub(super) fn generic_type_modules(ladfile: &LadFile) -> Vec<LuaModule> {
    let mut modules = vec![];
    for (generic_id, generic_type) in &ladfile.generic_types {
        let name = qualified(&generic_type.namespace, &generic_type.identifier);
        let mut fields = layout_fields(ladfile, &generic_type.layout);
        fields.extend(function_fields(
            ladfile,
            generic_type
                .associated_functions
                .iter()
                .filter_map(|id| ladfile.functions.get(id)),
        ));
        let (variant_fields, variant_modules) =
            variant_modules(ladfile, generic_id, generic_type, &name);
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
/// parameters, with the functions of the variant of every instantiation, i.e. its constructor.
fn variant_modules(
    ladfile: &LadFile,
    generic_id: &LadTypeId,
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
        let variant_functions = ladfile.functions.values().filter(|f| {
            matches!(&f.namespace, ladfile::LadFunctionNamespace::Variant {
                type_id: LadNamedType::Generic { generic_type, args },
                variant: function_variant,
            } if generic_type == generic_id
                && function_variant == variant
                && args.iter().all(|arg| matches!(arg, LadTypeKind::Val(LadNamedType::Param { .. }))))
        });
        modules.push(LuaModule {
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
    (fields, modules)
}

/// The subclasses of the instantiations with functions of their own, i.e. `pause` on `time.Time.Virtual : time.Time<time.Virtual>`.
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
                generics: vec![],
                documentation: generic_type.documentation.clone(),
                ..Default::default()
            }),
            functions,
            ..Default::default()
        });
    }
    modules
}

/// The class of an instantiation of a generic type with functions of its own, i.e. `time.Time.Virtual` for `Time<Virtual>`,
/// which extends the generic class applied to the arguments. Instantiations without functions of their own have none.
///
/// It is named after the identifiers of the arguments, joined by `_` (`Time.Virtual`, `Foo.Image_Mesh`), unless they aren't
/// types of the file or clash with a variant. Its table is the class name with `_` for `.`, a local table, as the generic class
/// has none to nest it in.
pub(super) fn instantiation_class_name(
    ladfile: &LadFile,
    generic_type: &LadGenericType,
    instantiation: &LadInstantiation,
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

pub(super) fn erase_generics(kind: &LadTypeKind, generics: &[String]) -> LadTypeKind {
    let is_generic = |named: &LadNamedType| matches!(named, LadNamedType::Param { param } if generics.contains(param));
    let mut kind = kind.clone();
    match &kind {
        LadTypeKind::Type { of, erased } if is_generic(of) => return (**erased).clone(),
        // values of a parameter, or of a type applied to one like `Handle<A>`, are references
        kind if kind.named().is_some() && mentions_generic(kind, generics) => {
            return LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference);
        }
        _ => {}
    }
    for child in kind.children_mut() {
        *child = erase_generics(child, generics);
    }
    kind
}

/// Whether the kind uses one of the given parameters of a generic function.
pub(super) fn mentions_generic(kind: &LadTypeKind, generics: &[String]) -> bool {
    let mut mentions = false;
    kind.for_each(&mut |kind| {
        let named = match kind {
            LadTypeKind::Type { of, .. } | LadTypeKind::Fields { of, .. } => Some(of),
            kind => kind.named(),
        };
        mentions |=
            matches!(named, Some(LadNamedType::Param { param }) if generics.contains(param));
    });
    mentions
}
