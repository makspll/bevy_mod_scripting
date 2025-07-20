//! Traits and types for managing script contexts.

use crate::{
    bindings::{ThreadWorldContainer, WorldContainer, WorldGuard},
    error::{InteropError, ScriptError},
    script::ContextKey,
    IntoScriptPluginParams,
};
use bevy::ecs::system::Resource;

/// A trait that all script contexts must implement.
///
/// Contexts are not required to be `Sync` as they are internally stored behind a `Mutex` but they must satisfy `Send` so they can be
/// freely sent between threads.
pub trait Context: 'static + Send {}
impl<T: 'static + Send> Context for T {}

/// Initializer run once after creating a context but before executing it for the first time as well as after re-loading the script
pub type ContextInitializer<P> =
    fn(&ContextKey, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Initializer run every time before executing or loading/re-loading a script
pub type ContextPreHandlingInitializer<P> =
    fn(&ContextKey, &mut <P as IntoScriptPluginParams>::C) -> Result<(), ScriptError>;

/// Settings concerning the creation and assignment of script contexts as well as their initialization.
#[derive(Resource)]
pub struct ContextLoadingSettings<P: IntoScriptPluginParams> {
    /// Defines the strategy used to load and reload contexts
    pub loader: ContextBuilder<P>,
    /// Defines the strategy used to assign contexts to scripts
    pub assignment_strategy: ContextAssignmentStrategy,
    /// Initializers run once after creating a context but before executing it for the first time
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// Initializers run every time before executing or loading a script
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ContextLoadingSettings<P> {
    fn default() -> Self {
        Self {
            loader: ContextBuilder::default(),
            assignment_strategy: Default::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<T: IntoScriptPluginParams> Clone for ContextLoadingSettings<T> {
    fn clone(&self) -> Self {
        Self {
            loader: self.loader.clone(),
            assignment_strategy: self.assignment_strategy,
            context_initializers: self.context_initializers.clone(),
            context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
        }
    }
}
/// A strategy for loading contexts
pub type ContextLoadFn<P> = fn(
    context_key: &ContextKey,
    content: &[u8],
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &<P as IntoScriptPluginParams>::R,
) -> Result<<P as IntoScriptPluginParams>::C, ScriptError>;

/// A strategy for reloading contexts
pub type ContextReloadFn<P> = fn(
    context_key: &ContextKey,
    content: &[u8],
    previous_context: &mut <P as IntoScriptPluginParams>::C,
    context_initializers: &[ContextInitializer<P>],
    pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
    runtime: &<P as IntoScriptPluginParams>::R,
) -> Result<(), ScriptError>;

/// A strategy for loading and reloading contexts
pub struct ContextBuilder<P: IntoScriptPluginParams> {
    /// The function to load a context
    pub load: ContextLoadFn<P>,
    /// The function to reload a context
    pub reload: ContextReloadFn<P>,
}

impl<P: IntoScriptPluginParams> Default for ContextBuilder<P> {
    fn default() -> Self {
        Self {
            load: |_, _, _, _, _| Err(InteropError::invariant("no context loader set").into()),
            reload: |_, _, _, _, _, _| {
                Err(InteropError::invariant("no context reloader set").into())
            },
        }
    }
}

impl<P: IntoScriptPluginParams> ContextBuilder<P> {
    /// load a context
    pub fn load(
        loader: ContextLoadFn<P>,
        context_key: &ContextKey,
        content: &[u8],
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<P::C, ScriptError> {
        WorldGuard::with_existing_static_guard(world.clone(), |world| {
            ThreadWorldContainer.set_world(world)?;
            (loader)(
                context_key,
                content,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }

    /// reload a context
    pub fn reload(
        reloader: ContextReloadFn<P>,
        context_key: &ContextKey,
        content: &[u8],
        previous_context: &mut P::C,
        context_initializers: &[ContextInitializer<P>],
        pre_handling_initializers: &[ContextPreHandlingInitializer<P>],
        world: WorldGuard,
        runtime: &P::R,
    ) -> Result<(), ScriptError> {
        WorldGuard::with_existing_static_guard(world, |world| {
            ThreadWorldContainer.set_world(world)?;
            (reloader)(
                context_key,
                content,
                previous_context,
                context_initializers,
                pre_handling_initializers,
                runtime,
            )
        })
    }
}

impl<P: IntoScriptPluginParams> Clone for ContextBuilder<P> {
    fn clone(&self) -> Self {
        Self {
            load: self.load,
            reload: self.reload,
        }
    }
}

/// The strategy used in assigning contexts to scripts
#[derive(Default, Clone, Copy)]
pub enum ContextAssignmentStrategy {
    /// Assign a new context to each script
    #[default]
    Individual,
    /// Share contexts with all other scripts
    Global,
}

impl ContextAssignmentStrategy {
    /// Returns true if there is one global context.
    pub fn is_global(&self) -> bool {
        matches!(self, ContextAssignmentStrategy::Global)
    }
}
