use std::{any::TypeId, borrow::Cow, default, marker::PhantomData};

use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, FunctionArgInfo, FunctionCallContext, FunctionInfo, FunctionKey,
    InteropError, IntoNamespace, Namespace, Primitive, ReflectReference, ScriptValue,
    ThreadWorldContainer, WorldExtensions,
};
use bevy_platform::collections::{HashMap, HashSet};
use bevy_reflect::{Reflect, TypeInfo, TypeRegistry, impl_reflect_opaque};
use wasmtime::{
    Engine,
    component::{Linker, Resource, ResourceType, Val, types::ComponentFunc},
};

use crate::{WasmtimeStoreData, script_value_to_val, to_interop_error, val_to_script_value};
use heck::ToKebabCase;

const WIT_KEYWORDS: &[&str] = &[
    "use",
    "type",
    "resource",
    "record",
    "flags",
    "variant",
    "enum",
    "func",
    "static",
    "interface",
    "world",
    "import",
    "export",
    "package",
    "constructor",
    "include",
    "with",
    "from",
    "as",
    "future",
    "stream",
    "list",
    "option",
    "result",
    "tuple",
    "borrow",
    "own",
    "u8",
    "u16",
    "u32",
    "u64",
    "s8",
    "s16",
    "s32",
    "s64",
    "float32",
    "float64",
    "char",
    "bool",
    "string",
];

pub fn to_wit_ident(name: &str) -> String {
    let ident = name
        .replace("::", "-")
        .replace('_', "-")
        .replace(' ', "-")
        .to_kebab_case();

    if WIT_KEYWORDS.contains(&ident.as_str()) {
        format!("%{ident}")
    } else {
        ident
    }
}

pub fn to_canonical_abi_func_name(
    key: &FunctionKey,
    type_reg: &TypeRegistry,
    info: &FunctionInfo,
) -> Cow<'static, str> {
    match key.namespace {
        Namespace::Global => key.name.clone(),
        Namespace::OnType(type_id) => {
            let ident = type_reg
                .get(type_id)
                .map(|reg| reg.type_info().type_path_table().ident().unwrap())
                .or_else(|| {
                    if type_id == TypeId::of::<World>() {
                        Some("world")
                    } else {
                        None
                    }
                })
                .expect("missing type");
            let type_name = to_wit_ident(ident);
            let is_method = info.is_method();
            let prefix = if is_method { "[method]" } else { "[static]" };
            let fname = to_wit_ident(&key.name);
            format!("{prefix}{type_name}.{fname}").into()
        }
    }
}

const REFLECT_REF_RES_NAME: &str = "reflect-ref";

/// Build and populate the wasmtime component linker from the BMS function registry.  
pub fn build_linker(linker: &mut Linker<WasmtimeStoreData>) -> Result<(), InteropError> {
    let world = ThreadWorldContainer.try_get_context()?.world;
    let function_registry = world.script_function_registry();
    let function_registry = function_registry.read();

    // build special reflect-ref resource
    // let reflect_ref_interface = namespace_to_interface(ReflectReference::into_namespace());

    // let mut reflect_ref_inst = linker.instance(&reflect_ref_interface).unwrap();
    // reflect_ref_inst
    //     .resource(
    //         REFLECT_REF_RES_NAME,
    //         ResourceType::host::<ReflectReference>(),
    //         |mut ctx, res| {
    //             // TODO: do we need a custom drop ?
    //             // ctx.data_mut().passed_references.remove(&res);
    //             Ok(())
    //         },
    //     )
    //     .unwrap();

    // reflect_ref_inst.func_wrap(
    //     "[method]reflect-ref.display",
    //     |a, r: (Resource<ReflectReference>,)| {
    //         let ref_ = a.data().get_ref(&r.0).unwrap();
    //         println!("ref: {ref_:?}");
    //         Ok(())
    //     },
    // );

    // Group functions by interface name
    // let mut by_interface: HashMap<String, Vec<(FunctionKey, DynamicScriptFunction)>> =
    //     HashMap::new();
    // for (key, func) in registry.iter_all() {
    //     let interface_name = namespace_to_interface(key.namespace);
    //     by_interface
    //         .entry(interface_name)
    //         .or_default()
    //         .push((key.clone(), func.clone()));
    // }

    // Register each interface
    let registry = world.type_registry();
    let registry = registry.read();
    let mut instance_linker = linker
        .instance("bms:scripting/types")
        .map_err(|e| to_interop_error(e))?;

    for type_ in registry.iter() {
        if let Some(ident) = type_.type_info().type_path_table().ident()
            && type_.type_info().generics().is_empty()
            // https://github.com/bevyengine/bevy/issues/24235
            && !type_.type_info().type_path().contains("<")
        {
            let ident = to_wit_ident(ident);
            println!(
                "{ident}: {}, {:?}",
                type_.type_info().type_path(),
                type_.type_info().generics()
            );
            instance_linker
                .resource(&ident, ResourceType::host::<ReflectReference>(), |a, b| {
                    Ok(())
                })
                .map_err(|e| to_interop_error(e))?;
            let type_namespace = Namespace::OnType(type_.type_id());
            let mut registered_already = HashSet::<String>::default();
            for (func_name, func) in function_registry.iter_namespace(type_namespace) {
                registered_already.insert(func_name.name.to_string());
                link_function(&registry, &mut instance_linker, func_name, func)?;
            }
            // TODO: exclude primitives from here
            for (func_name, func) in
                function_registry.iter_namespace(ReflectReference::into_namespace())
            {
                if (registered_already.contains(&func_name.name.to_string())) {
                    continue;
                }
                let mut fake_key = func_name.clone();
                fake_key.namespace = type_namespace;
                link_function(&registry, &mut instance_linker, &fake_key, func)?;
            }
        }
    }

    for (func_name, func) in function_registry.iter_namespace(Namespace::Global) {
        link_function(&registry, &mut instance_linker, func_name, func)?;
    }

    wasmtime_wasi::p2::add_to_linker_sync(linker).map_err(to_interop_error)?;
    Ok(())
}

fn link_function(
    registry: &std::sync::RwLockReadGuard<'_, TypeRegistry>,
    instance_linker: &mut wasmtime::component::LinkerInstance<'_, WasmtimeStoreData>,
    func_name: &FunctionKey,
    func: &DynamicScriptFunction,
) -> Result<(), InteropError> {
    let func_clone = func.clone();
    let canonical_abi_name = to_canonical_abi_func_name(&func_name, registry, &func.info);
    instance_linker
        .func_new(
            &canonical_abi_name,
            into_wasm_function_registration(func_clone),
        )
        .map_err(|e| to_interop_error(e))?;
    Ok(())
}

fn into_wasm_function_registration(
    func: DynamicScriptFunction,
) -> impl Fn(
    wasmtime::StoreContextMut<WasmtimeStoreData>,
    ComponentFunc,
    &[Val],
    &mut [Val],
) -> wasmtime::error::Result<()> {
    move |mut store: wasmtime::StoreContextMut<WasmtimeStoreData>,
          _,
          args: &[Val],
          results: &mut [Val]|
          -> wasmtime::error::Result<()> {
        let script_args: Vec<ScriptValue> = args
            .iter()
            .map(|v| val_to_script_value(v, &mut store))
            .collect();

        let ctx = FunctionCallContext::new(Language::Wasmtime);

        let result = func
            .call(script_args, ctx)
            .map_err(|e| wasmtime::error::format_err!("{e:?}"))?;

        if !results.is_empty() {
            results[0] = script_value_to_val(result, &mut store);
        }

        Ok(())
    }
}
