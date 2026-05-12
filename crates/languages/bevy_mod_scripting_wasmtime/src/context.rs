use bevy_ecs::{reflect::AppTypeRegistry, world::WorldId};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, FunctionCallContext, InteropError, IntoNamespace, Namespace,
    ReflectReference, ScriptValue, ThreadWorldContainer, WorldExtensions,
};
use bevy_mod_scripting_core::config::GetPluginThreadConfig;
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;
use wasmtime::{Engine, Store, component::*};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use crate::{WasmtimeScriptingPlugin, build_linker, script_value_to_val, val_to_script_value};

/// Per-script execution context.  
pub struct WasmtimeContext {
    pub store: Store<WasmtimeStoreData>,
    pub instance: Instance,
    /// Retained for hot-reload: re-instantiate from the same component bytes.  
    pub component: Component,
}

/// Host-side data stored in each wasmtime Store.  
pub struct WasmtimeStoreData {
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
            wasi_ctx: WasiCtxBuilder::new().inherit_stdio().build(),
            resource_table: ResourceTable::new(),
        }
    }

    pub fn push_ref(&mut self, r: ReflectReference) -> Resource<ReflectReference> {
        self.resource_table.push(r).unwrap()
    }

    pub fn get_ref(&self, id: &Resource<ReflectReference>) -> Option<&ReflectReference> {
        self.resource_table.get(id).ok()
    }
}

/// Load a wasmtime context from compiled `.wasm` component bytes.  
pub fn wasmtime_context_load(
    attachment: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<WasmtimeContext, InteropError> {
    let config = WasmtimeScriptingPlugin::readonly_configuration(world_id);
    let runtime = config.runtime.read();
    let component = Component::new(&runtime.engine, content).map_err(|e| to_interop_error(e))?;

    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new());

    let linker = &runtime.linker;
    let instance = linker.instantiate(&mut store, &component).map_err(|e| {
        println!("{e:#?}");
        to_interop_error(e)
    })?;

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
    let runtime = config.runtime.read();

    let component = Component::new(&runtime.engine, content).map_err(|e| to_interop_error(e))?;

    // Re-instantiate with the same linker
    let mut store = Store::new(&runtime.engine, WasmtimeStoreData::new());
    let linker = &runtime.linker;
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

pub fn to_interop_error(error: wasmtime::Error) -> InteropError {
    InteropError::external_boxed(error.into_boxed_dyn_error())
}
