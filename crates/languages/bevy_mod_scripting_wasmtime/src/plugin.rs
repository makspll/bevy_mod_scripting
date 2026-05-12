use bevy_app::{App, Plugin};
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_core::{
    IntoScriptPluginParams, ScriptingPlugin,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    make_plugin_config_static,
    script::ContextPolicy,
};

use crate::{
    WasmtimeContext, WasmtimeRuntime, WasmtimeRuntimeInner, build_linker, wasmtime_context_load,
    wasmtime_context_reload, wasmtime_handler,
};

make_plugin_config_static!(WasmtimeScriptingPlugin);

/// The wasmtime scripting plugin.  
pub struct WasmtimeScriptingPlugin {
    /// The internal scripting plugin  
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl IntoScriptPluginParams for WasmtimeScriptingPlugin {
    type C = WasmtimeContext;
    type R = WasmtimeRuntime;
    const LANGUAGE: Language = Language::Wasmtime; // TODO: Add Language::Wasm  

    fn build_runtime() -> Self::R {
        WasmtimeRuntime::new(WasmtimeRuntimeInner::new())
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
                runtime_initializers: vec![|runtime: &WasmtimeRuntime| {
                    let mut runtime = runtime.write();
                    build_linker(&mut runtime.linker)?;
                    Ok(())
                }],
                context_policy: ContextPolicy::default(),
                language: Self::LANGUAGE,
                supported_extensions: vec!["wasm", "component.wasm"],
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
