//! Traits and types for managing script contexts.

use bevy_ecs::resource::Resource;

use crate::{
    IntoScriptPluginParams,
    bindings::{ThreadWorldContainer, WorldContainer, WorldGuard},
    error::ScriptError,
    script::ScriptAttachment,
};

/// A trait that all script contexts must implement.
///
/// Contexts are not required to be `Sync` as they are internally stored behind a `Mutex` but they must satisfy `Send` so they can be
/// freely sent between threads.
pub trait Context: 'static + Send {}
impl<T: 'static + Send> Context for T {}

/// Initializer run once after creating a context but before executing it for the first time as well as after re-loading the script
pub type ContextInitializer<P> =
    fn(&ScriptAttachment, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Initializer run every time before executing or loading/re-loading a script
pub type ContextPreHandlingInitializer<P> =
    fn(&ScriptAttachment, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Settings concerning the creation and assignment of script contexts as well as their initialization.
#[derive(Resource)]
pub struct ContextLoadingSettings<P: IntoScriptPluginParams> {
    /// Whether to emit responses from core script_callbacks like `on_script_loaded` or `on_script_unloaded`.
    /// By default, this is `false` and responses are not emitted.
    pub emit_responses: bool,
    /// Initializers run once after creating a context but before executing it for the first time
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// Initializers run every time before executing or loading a script
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ContextLoadingSettings<P> {
    fn default() -> Self {
        Self {
            emit_responses: false,
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<T: IntoScriptPluginParams> Clone for ContextLoadingSettings<T> {
    fn clone(&self) -> Self {
        Self {
            emit_responses: self.emit_responses,
            context_initializers: self.context_initializers.clone(),
            context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
        }
    }
}
/// A strategy for loading contexts
pub type ContextLoadFn<P> = fn(
    attachment: &ScriptAttachment,
    content: &[u8],
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &<P as IntoScriptPluginParams>::R,
) -> Result<<P as IntoScriptPluginParams>::C, ScriptError>;

/// A strategy for reloading contexts
pub type ContextReloadFn<P> = fn(
    attachment: &ScriptAttachment,
    content: &[u8],
    previous_context: &mut <P as IntoScriptPluginParams>::C,
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &<P as IntoScriptPluginParams>::R,
) -> Result<(), ScriptError>;

/// A utility trait for types implementing `IntoScriptPluginParams`.
///
/// Provides methods for initializing and reloading script contexts using the plugin's context loader and reloader functions.
pub trait ScriptingLoader<P: IntoScriptPluginParams> {
    /// Loads a script context using the provided loader function
    fn load(
        attachment: &ScriptAttachment,
        content: &[u8],
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<P::C, ScriptError>;

    /// Reloads a script context using the provided reloader function
    fn reload(
        attachment: &ScriptAttachment,
        content: &[u8],
        previous_context: &mut P::C,
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<(), ScriptError>;
}

impl<P: IntoScriptPluginParams> ScriptingLoader<P> for P {
    fn load(
        attachment: &ScriptAttachment,
        content: &[u8],
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<P::C, ScriptError> {
        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            ThreadWorldContainer.set_world(world)?;
            Self::context_loader()(
                attachment,
                content,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }

    fn reload(
        attachment: &ScriptAttachment,
        content: &[u8],
        previous_context: &mut P::C,
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<(), ScriptError> {
        WorldGuard::with_existing_static_guard(world, |world| {
            ThreadWorldContainer.set_world(world)?;
            Self::context_reloader()(
                attachment,
                content,
                previous_context,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }
}
