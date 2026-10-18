//! Rune integration for the bevy_mod_scripting system.
//!
//! Rune is a statically resolved language, which scripts of BMS don't fit into as is, see [`types`] for how member access works.
//! Differences from Lua scripts, which are ported by hand:
//! * there are no top level statements, `pub fn main()` runs when a script loads, callbacks are `pub fn`s as in Lua
//! * there are no globals shared between callbacks, `state()` is a store per context, see [`modules::StateStore`]
//! * `catch(|| ..)` takes the place of `pcall`, returning a `Result`
//! * functions of BMS called with the method syntax on a value which is a function member of a namespace, see [`types::RuneNamespace`]
//! * there is no `math.pi` etc. in Rune, they are in the `math` module, where `math::PI` is the constant
//! * Rune has no unary minus or power operator for types, use the `neg` and `pow` functions
//! * operators with a value on the right (`2 * v`) are not supported, use `v * 2` or the reflected functions, i.e. `v.rmul(2)`
//! * indexes are 0 based
//! * functions which are values (`let f = v.length;`) are called as `f()` when they take a known number of up to 5 arguments, other ones (variadic functions, exports, functions from scripts) with `f.call(..)`
//! * closures passed to BMS (`register_callback`, `into_script_function`) can only capture numbers, strings, lists and maps of them, since BMS calls them from any thread
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use ::bevy_app::Plugin;
use bevy_app::App;
use bevy_ecs::world::{Mut, WorldId};
use bevy_log::trace;
use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{InteropError, WorldExtensions, script_value::ScriptValue};
use bevy_mod_scripting_core::{
    IntoScriptPluginParams, ScriptingPlugin,
    callbacks::ScriptCallbacks,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    event::CallbackLabel,
    make_plugin_config_static,
    script::ContextPolicy,
};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_mod_scripting_world::ThreadWorldContainer;
pub use rune;
use rune::{
    Context, Diagnostics, Module, Source, Sources, Unit, Vm,
    runtime::{Function, RuntimeContext},
};

pub mod error;
pub mod modules;
pub mod names;
pub mod types;
pub mod value;

pub use error::IntoInteropError;

use crate::{
    error::RuneError,
    modules::StateStore,
    names::{Fingerprint, collect_names},
    value::{call_rune_function, call_sync_function},
};

make_plugin_config_static!(RuneScriptingPlugin);

/// Adds functions to the Rune context, i.e. for tests.
pub type ModuleInitializer = fn(&mut Module) -> Result<(), rune::ContextError>;

/// The runtime of the Rune plugin, which holds what is added to every context.
#[derive(Default)]
pub struct RuneRuntime {
    modules: std::sync::RwLock<Vec<ModuleInitializer>>,
}

impl RuneRuntime {
    /// Adds a module initializer, run whenever a Rune context is built, in the root module scripts see.
    pub fn add_module(&self, initializer: ModuleInitializer) {
        self.modules
            .write()
            .unwrap_or_else(|e| e.into_inner())
            .push(initializer);
    }
}

/// A Rune context, a compiled script and what it runs with.
///
/// The context is rebuilt when new names are registered in the app, i.e. fields of types registered by scripts, see [`types`].
/// This keeps the compiled script, and the [`modules::StateStore`].
pub struct RuneContext {
    unit: Arc<Unit>,
    runtime: Arc<RuntimeContext>,
    state: StateStore,
    fingerprint: Fingerprint,
}

impl std::fmt::Debug for RuneContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RuneContext")
            .field("fingerprint", &self.fingerprint)
            .finish_non_exhaustive()
    }
}

impl RuneContext {
    /// A virtual machine running the script
    pub fn vm(&self) -> Vm {
        Vm::new(self.runtime.clone(), self.unit.clone())
    }

    /// The compiled script
    pub fn unit(&self) -> &Arc<Unit> {
        &self.unit
    }
}

impl Deref for RuneContext {
    type Target = Arc<RuntimeContext>;

    fn deref(&self) -> &Self::Target {
        &self.runtime
    }
}

impl DerefMut for RuneContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.runtime
    }
}

impl IntoScriptPluginParams for RuneScriptingPlugin {
    type C = RuneContext;
    type R = RuneRuntime;
    const LANGUAGE: Language = Language::Rune;

    fn build_runtime() -> Self::R {
        RuneRuntime::default()
    }

    fn handler() -> bevy_mod_scripting_core::handler::HandlerFn<Self> {
        rune_handler
    }

    fn context_loader() -> bevy_mod_scripting_core::context::ContextLoadFn<Self> {
        rune_context_load
    }

    fn context_reloader() -> bevy_mod_scripting_core::context::ContextReloadFn<Self> {
        rune_context_reload
    }
}

// necessary for automatic config goodies
impl AsMut<ScriptingPlugin<Self>> for RuneScriptingPlugin {
    fn as_mut(&mut self) -> &mut ScriptingPlugin<RuneScriptingPlugin> {
        &mut self.scripting_plugin
    }
}

/// The rune scripting plugin. Used to add rune scripting to a bevy app within the context of the BMS framework.
pub struct RuneScriptingPlugin {
    /// The internal scripting plugin
    pub scripting_plugin: ScriptingPlugin<Self>,
}

impl Default for RuneScriptingPlugin {
    fn default() -> Self {
        RuneScriptingPlugin {
            scripting_plugin: ScriptingPlugin {
                runtime_initializers: Vec::default(),
                supported_extensions: vec!["rn"],
                // the builtins are installed when the Rune context is built, see `build_rune_context`
                context_initializers: vec![],
                context_pre_handling_initializers: vec![],
                language: Language::Rune,
                context_policy: ContextPolicy::default(),
                emit_responses: false,
                processing_pipeline_plugin: Default::default(),
            },
        }
    }
}

impl Plugin for RuneScriptingPlugin {
    fn build(&self, app: &mut App) {
        self.scripting_plugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        self.scripting_plugin.finish(app);
    }
}

fn register_callback(callback: String, function: Function) -> Result<(), InteropError> {
    let world = ThreadWorldContainer.try_get_context()?.world;
    let attachment = world.current_attachment().0.ok_or_else(|| {
        InteropError::str("Cannot register callback, missing script attachment context.")
    })?;
    // the callback is stored where any thread can call it, see `rune_function_to_script`
    let function = function.into_sync().map_err(|error| {
        InteropError::external(RuneError(format!(
            "this callback can't be registered, closures can only capture numbers, strings, lists and maps of them: {error}"
        )))
    })?;

    world.with_resource_mut(|res: Mut<ScriptCallbacks<RuneScriptingPlugin>>| {
        let mut callbacks = res.callbacks.write();
        callbacks.insert(
            (attachment.clone(), callback),
            Arc::new(
                move |args: Vec<ScriptValue>, context: &mut RuneContext, world_id: WorldId| {
                    let pre_handling_callbacks =
                        RuneScriptingPlugin::readonly_configuration(world_id)
                            .pre_handling_callbacks;
                    pre_handling_callbacks
                        .iter()
                        .try_for_each(|init| init(&attachment, context))?;
                    call_sync_function(&function, args.into())
                },
            ),
        )
    })?;
    Ok(())
}

fn context_error(error: impl std::fmt::Display) -> InteropError {
    InteropError::external(RuneError(format!("failed to set up Rune: {error}")))
}

/// Builds the Rune context with everything scripts can use.
fn build_rune_context(
    names: &std::collections::BTreeSet<&'static str>,
    state: StateStore,
    runtime: &RuneRuntime,
) -> Result<Context, InteropError> {
    let mut context = Context::with_default_modules().map_err(context_error)?;
    let mut module = Module::new();
    types::register_types(&mut module, names).map_err(context_error)?;
    modules::register_globals(&mut module, state, register_callback).map_err(context_error)?;
    for initializer in runtime
        .modules
        .read()
        .unwrap_or_else(|e| e.into_inner())
        .iter()
    {
        initializer(&mut module).map_err(context_error)?;
    }
    context.install(module).map_err(context_error)?;
    context
        .install(modules::math_module().map_err(context_error)?)
        .map_err(context_error)?;
    Ok(context)
}

/// The names and fingerprint of the world, if it is available.
fn world_names() -> (std::collections::BTreeSet<&'static str>, Fingerprint) {
    match ThreadWorldContainer.try_get_context() {
        Ok(context) => (
            collect_names(&context.world),
            Fingerprint::of(&context.world),
        ),
        Err(_) => Default::default(),
    }
}

fn compile(
    context_key: &ScriptAttachment,
    content: &[u8],
    state: StateStore,
    world_id: WorldId,
) -> Result<RuneContext, InteropError> {
    let (names, fingerprint) = world_names();
    let config = RuneScriptingPlugin::readonly_configuration(world_id);
    let context = build_rune_context(&names, state.clone(), config.runtime)?;
    let runtime = Arc::new(context.runtime().map_err(context_error)?);

    let name = context_key
        .script()
        .path()
        .map(|p| p.to_string())
        .unwrap_or_else(|| "script".to_owned());
    let text = std::str::from_utf8(content).map_err(context_error)?;
    let mut sources = Sources::new();
    sources
        .insert(Source::new(name, text).map_err(context_error)?)
        .map_err(context_error)?;
    let mut diagnostics = Diagnostics::new();
    let unit = rune::prepare(&mut sources)
        .with_context(&context)
        .with_diagnostics(&mut diagnostics)
        .build();
    let unit = match unit {
        Ok(unit) => unit,
        Err(_) => {
            let mut writer = rune::termcolor::NoColor::new(Vec::<u8>::new());
            let _ = diagnostics.emit(&mut writer, &sources);
            let message = String::from_utf8_lossy(&writer.into_inner()).into_owned();
            return Err(InteropError::external(RuneError(format!(
                "failed to compile the script:\n{message}"
            ))));
        }
    };

    Ok(RuneContext {
        unit: Arc::new(unit),
        runtime,
        state,
        fingerprint,
    })
}

/// Runs the initializers and the script's `main`, see the crate documentation.
fn initialize(
    context: &mut RuneContext,
    context_key: &ScriptAttachment,
    world_id: WorldId,
) -> Result<(), InteropError> {
    let config = RuneScriptingPlugin::readonly_configuration(world_id);
    config
        .context_initialization_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;
    config
        .pre_handling_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    if let Ok(main) = context.vm().lookup_function(["main"]) {
        call_rune_function(&main, Default::default())?;
    }
    Ok(())
}

#[profiling::function]
/// Load a rune context from a script
pub fn rune_context_load(
    context_key: &ScriptAttachment,
    content: &[u8],
    world_id: WorldId,
) -> Result<RuneContext, InteropError> {
    let mut context = compile(context_key, content, Default::default(), world_id)?;
    initialize(&mut context, context_key, world_id)?;
    Ok(context)
}

#[profiling::function]
/// Reload a rune context from a script, the script is replaced and the state is kept
pub fn rune_context_reload(
    context_key: &ScriptAttachment,
    content: &[u8],
    old_ctxt: &mut RuneContext,
    world_id: WorldId,
) -> Result<(), InteropError> {
    let new = compile(context_key, content, old_ctxt.state.clone(), world_id)?;
    old_ctxt.unit = new.unit;
    old_ctxt.runtime = new.runtime;
    old_ctxt.fingerprint = new.fingerprint;
    initialize(old_ctxt, context_key, world_id)
}

/// Rebuilds what the context runs with, if new names were registered in the app since it was built.
fn refresh_names(context: &mut RuneContext, world_id: WorldId) -> Result<(), InteropError> {
    let Ok(thread_context) = ThreadWorldContainer.try_get_context() else {
        return Ok(());
    };
    let fingerprint = Fingerprint::of(&thread_context.world);
    if fingerprint == context.fingerprint {
        return Ok(());
    }
    let names = collect_names(&thread_context.world);
    let config = RuneScriptingPlugin::readonly_configuration(world_id);
    let rune_context = build_rune_context(&names, context.state.clone(), config.runtime)?;
    context.runtime = Arc::new(rune_context.runtime().map_err(context_error)?);
    context.fingerprint = fingerprint;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
/// The rune handler for events
pub fn rune_handler(
    args: Vec<ScriptValue>,
    context_key: &ScriptAttachment,
    callback_label: &CallbackLabel,
    context: &mut RuneContext,
    world_id: WorldId,
) -> Result<ScriptValue, InteropError> {
    let config = RuneScriptingPlugin::readonly_configuration(world_id);

    config
        .pre_handling_callbacks
        .iter()
        .try_for_each(|init| init(context_key, context))?;

    refresh_names(context, world_id)?;

    let handler = match context.vm().lookup_function([callback_label.as_ref()]) {
        Ok(handler) => handler,
        // not subscribed to this event type
        Err(_) => {
            trace!(
                "Context {} is not subscribed to callback {}",
                context_key,
                callback_label.as_ref()
            );
            return Ok(ScriptValue::Unit);
        }
    };

    call_rune_function(&handler, args.into())
}
