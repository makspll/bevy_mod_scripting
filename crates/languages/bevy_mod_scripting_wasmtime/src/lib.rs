//! Wasmtime Component Model scripting plugin for bevy_mod_scripting.  
//!  
//! Guests compile to WebAssembly components using WIT bindings generated from the LAD file.  
//! The host implements all BMS functions as component model imports, with ReflectReferences  
//! represented as opaque resource handles.  
#![allow(
    clippy::all,
    missing_docs,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unwrap_used,
    reason = "cooking"
)]
use std::{any::TypeId, collections::HashMap, sync::Arc};

use bevy_app::{App, Plugin};
use bevy_ecs::{reflect::AppTypeRegistry, world::WorldId};
use bevy_log::{error, trace};
use bevy_mod_scripting_asset::{Language, ScriptAsset};
use bevy_mod_scripting_bindings::{
    AppScriptGlobalsRegistry, InteropError, Namespace, ReflectReference, ScriptValue,
    ThreadWorldContainer, WorldExtensions,
    function::script_function::{
        AppScriptFunctionRegistry, DynamicScriptFunction, FunctionCallContext,
    },
};
use bevy_mod_scripting_core::{
    IntoScriptPluginParams, ScriptingPlugin,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    event::CallbackLabel,
    make_plugin_config_static,
    script::ContextPolicy,
};
use bevy_mod_scripting_script::ScriptAttachment;
use parking_lot::Mutex;
use wasmtime::{Config, Engine};
use wasmtime::{
    Store,
    component::{Component, Instance, Linker, Val},
};

make_plugin_config_static!(WasmtimeScriptingPlugin);

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
    world_id: WorldId,
}

impl WasmtimeStoreData {
    fn new(world_id: WorldId) -> Self {
        Self {
            resources: HashMap::new(),
            next_id: 1,
            world_id,
        }
    }

    fn push_ref(&mut self, r: ReflectReference) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);
        self.resources.insert(id, r);
        id
    }

    fn get_ref(&self, id: u32) -> Option<&ReflectReference> {
        self.resources.get(&id)
    }
}

/// Shared wasmtime runtime.  
pub struct WasmtimeRuntime {
    pub engine: Engine,
    /// Cached linker, populated once from the BMS function registry.  
    pub linker: Mutex<Option<Linker<WasmtimeStoreData>>>,
}

impl Default for WasmtimeRuntime {
    fn default() -> Self {
        Self::new()
    }
}
impl WasmtimeRuntime {
    fn new() -> Self {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config).expect("wasmtime Engine creation failed");
        Self {
            engine,
            linker: Mutex::new(None),
        }
    }
}

/// Convert a `ScriptValue` to a wasmtime component `Val`.  
fn script_value_to_val(sv: ScriptValue, data: &mut WasmtimeStoreData) -> Val {
    match sv {
        ScriptValue::Unit => Val::Tuple(vec![]),
        ScriptValue::Bool(b) => Val::Bool(b),
        ScriptValue::Integer(i) => Val::S64(i),
        ScriptValue::Float(f) => Val::Float64(f),
        ScriptValue::String(s) => Val::String(s.into_owned().into()),
        ScriptValue::List(list) => Val::List(
            list.into_iter()
                .map(|v| script_value_to_val(v, data))
                .collect(),
        ),
        ScriptValue::Reference(r) => Val::U32(data.push_ref(r)),
        ScriptValue::Error(e) => {
            Val::Result(Err(Some(Box::new(Val::String(format!("{e:?}").into())))))
        }
        ScriptValue::Function(_) | ScriptValue::FunctionMut(_) => Val::Tuple(vec![]),
        ScriptValue::Map(map) => Val::List(
            map.into_iter()
                .map(|(k, v)| Val::Tuple(vec![Val::String(k.into()), script_value_to_val(v, data)]))
                .collect(),
        ),
        _ => todo!(),
    }
}

/// Convert a wasmtime component `Val` to a `ScriptValue`.  
fn val_to_script_value(val: &Val, data: &mut WasmtimeStoreData) -> ScriptValue {
    match val {
        Val::Bool(b) => ScriptValue::Bool(*b),
        Val::S64(i) => ScriptValue::Integer(*i),
        Val::U32(i) => {
            if let Some(r) = data.get_ref(*i).cloned() {
                ScriptValue::Reference(r)
            } else {
                ScriptValue::Integer(*i as i64)
            }
        }
        Val::Float64(f) => ScriptValue::Float(*f),
        Val::String(s) => ScriptValue::String(s.to_string().into()),
        Val::List(list) => {
            ScriptValue::List(list.iter().map(|v| val_to_script_value(v, data)).collect())
        }
        Val::Tuple(items) if items.is_empty() => ScriptValue::Unit,
        Val::Tuple(items) => {
            ScriptValue::List(items.iter().map(|v| val_to_script_value(v, data)).collect())
        }
        _ => ScriptValue::Unit,
    }
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
fn namespace_to_interface(namespace: Namespace, world_id: WorldId) -> String {
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
fn build_linker(
    engine: &Engine,
    world_id: WorldId,
) -> Result<Linker<WasmtimeStoreData>, InteropError> {
    let mut linker = Linker::new(engine);

    let world = ThreadWorldContainer.try_get_context()?.world;
    let registry = world.script_function_registry();
    let registry = registry.read();

    // Group functions by interface name
    let mut by_interface: HashMap<String, Vec<(String, DynamicScriptFunction)>> = HashMap::new();
    for (key, func) in registry.iter_all() {
        let interface_name = namespace_to_interface(key.namespace, world_id);
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
                          args: &[Val],
                          results: &mut [Val]|
                          -> anyhow::Result<()> {
                        let script_args: Vec<ScriptValue> = args
                            .iter()
                            .map(|v| val_to_script_value(v, store.data_mut()))
                            .collect();

                        let ctx = FunctionCallContext::new(Language::Unknown);

                        let result = func_clone
                            .call(script_args, ctx)
                            .map_err(|e| anyhow::anyhow!("{e:?}"))?;

                        if !results.is_empty() {
                            results[0] = script_value_to_val(result, store.data_mut());
                        }

                        Ok(())
                    },
                )
                .map_err(|e| to_interop_error(e))?;
        }
    }

    Ok(linker)
}

fn to_interop_error(error: anyhow::Error) -> InteropError {
    InteropError::external_boxed(error.into_boxed_dyn_error())
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
            *guard = Some(build_linker(&runtime.engine, world_id)?);
        }
    }

    let component = Component::new(&runtime.engine, content).map_err(|e| to_interop_error(e))?;

    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new(world_id));

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
    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new(world_id));
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

/// The wasmtime handler for events.  
pub fn wasmtime_handler(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback_label: &CallbackLabel,
    context: &mut WasmtimeContext,
    world_id: WorldId,
) -> Result<ScriptValue, InteropError> {
    let config = WasmtimeScriptingPlugin::readonly_configuration(world_id);

    config
        .pre_handling_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    // Get the exported function by callback name
    let func = context
        .instance
        .get_func(&mut context.store, callback_label.as_ref());
    let func = match func {
        Some(f) => f,
        None => {
            trace!(
                "Context {} is not subscribed to callback {}",
                context_key,
                callback_label.as_ref()
            );
            return Ok(ScriptValue::Unit);
        }
    };

    // Convert args to Val
    let input_vals: Vec<Val> = args
        .into_iter()
        .map(|v| script_value_to_val(v, context.store.data_mut()))
        .collect();

    // Call the function
    let mut output_vals = vec![Val::Tuple(vec![])]; // Placeholder for result  
    func.call(&mut context.store, &input_vals, &mut output_vals)
        .map_err(|e| to_interop_error(e))?;

    // Convert result back to ScriptValue
    let result = output_vals
        .first()
        .map(|v| val_to_script_value(v, context.store.data_mut()))
        .unwrap_or(ScriptValue::Unit);

    Ok(result)
}

/// The wasmtime scripting plugin.  
pub struct WasmtimeScriptingPlugin {
    /// The internal scripting plugin  
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl IntoScriptPluginParams for WasmtimeScriptingPlugin {
    type C = WasmtimeContext;
    type R = WasmtimeRuntime;
    const LANGUAGE: Language = Language::Unknown; // TODO: Add Language::Wasm  

    fn build_runtime() -> Self::R {
        WasmtimeRuntime::new()
    }

    fn handler() -> bevy_mod_scripting_core::handler::HandlerFn<Self> {
        wasmtime_handler
    }

    fn context_loader() -> bevy_mod_scripting_core::context::ContextLoadFn<Self> {
        wasmtime_context_load
    }

    fn context_reloader() -> bevy_mod_scripting_core::context::ContextReloadFn<Self> {
        wasmtime_context_reload
    }
}

impl AsMut<ScriptingPlugin<Self>> for WasmtimeScriptingPlugin {
    fn as_mut(&mut self) -> &mut ScriptingPlugin<Self> {
        &mut self.scripting_plugin
    }
}

impl Default for WasmtimeScriptingPlugin {
    fn default() -> Self {
        Self {
            scripting_plugin: ScriptingPlugin {
                runtime_initializers: vec![],
                context_policy: ContextPolicy::default(),
                language: Self::LANGUAGE,
                supported_extensions: vec!["wasm"],
                context_initializers: vec![],
                context_pre_handling_initializers: vec![],
                emit_responses: false,
                processing_pipeline_plugin: Default::default(),
            },
        }
    }
}

impl Plugin for WasmtimeScriptingPlugin {
    fn build(&self, app: &mut App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        self.scripting_plugin.finish(app);
    }
}
