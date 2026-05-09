use bevy_ecs::{reflect::AppTypeRegistry, world::WorldId};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, FunctionCallContext, InteropError, Namespace, ReflectReference,
    ScriptValue, ThreadWorldContainer, WorldExtensions,
};
use bevy_mod_scripting_core::config::GetPluginThreadConfig;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;
use wasmtime::{Engine, Store, component::*};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::{WasmtimeScriptingPlugin, script_value_to_val, val_to_script_value};

/// Per-script execution context.  
pub struct WasmtimeContext {
    pub store: Store<WasmtimeStoreData>,
    pub instance: Instance,
    /// Retained for hot-reload: re-instantiate from the same component bytes.  
    pub component: Component,
}

/// Host-side data stored in each wasmtime Store.  
pub struct WasmtimeStoreData {
    resources: HashMap<u32, ReflectReference>,
    next_id: u32,
    wasi_ctx: WasiCtx,
    resource_table: ResourceTable,
}

impl WasiView for WasmtimeStoreData {
    fn ctx(&mut self) -> wasmtime_wasi::WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.resource_table,
        }
    }
}

impl WasmtimeStoreData {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
            next_id: 1,
            wasi_ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            resource_table: ResourceTable::new(),
        }
    }

    pub fn push_ref(&mut self, r: ReflectReference) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.resources.insert(id, r);
        id
    }

    pub fn get_ref(&self, id: u32) -> Option<&ReflectReference> {
        self.resources.get(&id)
    }
}

/// Load a wasmtime context from compiled `.wasm` component bytes.  
pub fn wasmtime_context_load(
    attachment: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<WasmtimeContext, InteropError> {
    let config = WasmtimeScriptingPlugin::readonly_configuration(world_id);
    let runtime = config.runtime;

    // Ensure the linker is populated (lazy, once)
    {
        let mut guard = runtime.linker.lock();
        if guard.is_none() {
            *guard = Some(build_linker(&runtime.engine)?);
        }
    }

    let component = Component::new(&runtime.engine, content).map_err(|e| to_interop_error(e))?;

    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new());

    let linker_guard = runtime.linker.lock();
    let linker = linker_guard.as_ref().unwrap();
    let instance = linker
        .instantiate(&mut store, &component)
        .map_err(|e| to_interop_error(e))?;

    let mut context = WasmtimeContext {
        store,
        instance,
        component,
    };

    for init in config.context_initialization_callbacks {
        init(attachment, &mut context)?;
    }

    Ok(context)
}

/// Reload a wasmtime context (hot-reload: re-instantiate from the same component).  
pub fn wasmtime_context_reload(
    attachment: &ScriptAttachment,
    content: &[u8],
    previous_context: &mut WasmtimeContext,
    world_id: WorldId,
) -> Result<(), InteropError> {
    // Re-compile the component (in case it changed)
    let config = WasmtimeScriptingPlugin::readonly_configuration(world_id);
    let runtime = config.runtime;

    let component = Component::new(&runtime.engine, content).map_err(|e| to_interop_error(e))?;

    // Re-instantiate with the same linker
    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new());
    let linker_guard = runtime.linker.lock();
    let linker = linker_guard.as_ref().unwrap();
    let instance = linker
        .instantiate(&mut store, &component)
        .map_err(|e| to_interop_error(e))?;

    // Replace the context
    previous_context.store = store;
    previous_context.instance = instance;
    previous_context.component = component;

    // Re-run initializers
    for init in config.context_initialization_callbacks {
        init(attachment, previous_context)?;
    }

    Ok(())
}

/// Convert a type name to a WIT-compatible kebab-case interface name.  
fn to_wit_ident(name: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;
    for (i, c) in name.chars().enumerate() {
        if c == '_' || c == ':' || c == ' ' {
            if !result.is_empty() && !result.ends_with('-') {
                result.push('-');
            }
            prev_upper = false;
        } else if c.is_uppercase() {
            if i > 0 && !prev_upper && !result.is_empty() && !result.ends_with('-') {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }
    result.trim_end_matches('-').to_string()
}

/// Get the WIT interface name for a given namespace.  
fn namespace_to_interface(namespace: Namespace) -> String {
    match namespace {
        Namespace::Global => "bms:scripting/globals".to_string(),
        Namespace::OnType(type_id) => {
            let type_name = ThreadWorldContainer
                .try_get_context()
                .ok()
                .and_then(|ctx| {
                    ctx.world
                        .with_resource(|r: &AppTypeRegistry| {
                            let registry = r.read();
                            registry
                                .get_type_info(type_id)
                                .and_then(|info| info.type_path_table().ident())
                                .map(|s| s.to_string())
                        })
                        .ok()
                })
                .flatten()
                .unwrap_or_else(|| format!("type-{type_id:?}"));
            format!("bms:scripting/{}", to_wit_ident(&type_name))
        }
    }
}

/// Build and populate the wasmtime component linker from the BMS function registry.  
fn build_linker(engine: &Engine) -> Result<Linker<WasmtimeStoreData>, InteropError> {
    let mut linker = Linker::new(engine);

    let world = ThreadWorldContainer.try_get_context()?.world;
    let registry = world.script_function_registry();
    let registry = registry.read();

    // Group functions by interface name
    let mut by_interface: HashMap<String, Vec<(String, DynamicScriptFunction)>> = HashMap::new();
    for (key, func) in registry.iter_all() {
        let interface_name = namespace_to_interface(key.namespace);
        by_interface
            .entry(interface_name)
            .or_default()
            .push((key.name.to_string(), func.clone()));
    }

    // Register each interface
    for (interface_name, functions) in by_interface {
        let mut instance_linker = linker
            .instance(&interface_name)
            .map_err(|e| to_interop_error(e))?;

        for (func_name, func) in functions {
            let func_clone = func.clone();
            instance_linker
                .func_new(
                    &func_name,
                    move |mut store: wasmtime::StoreContextMut<WasmtimeStoreData>,
                          _,
                          args: &[Val],
                          results: &mut [Val]|
                          -> wasmtime::error::Result<()> {
                        let script_args: Vec<ScriptValue> = args
                            .iter()
                            .map(|v| val_to_script_value(v, store.data_mut()))
                            .collect();

                        let ctx = FunctionCallContext::new(Language::Unknown);

                        let result = func_clone
                            .call(script_args, ctx)
                            .map_err(|e| wasmtime::error::format_err!("{e:?}"))?;

                        if !results.is_empty() {
                            results[0] = script_value_to_val(result, store.data_mut());
                        }

                        Ok(())
                    },
                )
                .map_err(|e| to_interop_error(e))?;
        }
    }

    wasmtime_wasi::p2::add_to_linker_sync(&mut linker).map_err(to_interop_error)?;

    Ok(linker)
}

pub fn to_interop_error(error: wasmtime::Error) -> InteropError {
    InteropError::external_boxed(error.into_boxed_dyn_error())
}
