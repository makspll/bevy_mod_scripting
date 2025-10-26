//! Core functionality for the bevy_mod_scripting framework.
//!
//! Contains language agnostic systems and types for handling scripting in bevy.

use crate::{
    callbacks::ScriptCallbacksPlugin,
    config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    context::{ContextLoadFn, ContextReloadFn},
    event::ScriptErrorEvent,
    handler::script_error_logger,
    pipeline::ScriptLoadingPipeline,
};
use bevy_app::{App, Plugin, PostUpdate};
use bevy_asset::{AssetApp, Handle};
use bevy_ecs::schedule::IntoScheduleConfigs;
use bevy_ecs::{
    reflect::{AppTypeRegistry, ReflectComponent},
    schedule::SystemSet,
};
use bevy_log::error;
use bevy_mod_scripting_asset::{Language, LanguageExtensions, ScriptAsset, ScriptAssetLoader};

use bevy_mod_scripting_bindings::{
    AppReflectAllocator, AppScheduleRegistry, AppScriptFunctionRegistry,
    DummyScriptFunctionRegistry, DynamicScriptComponentPlugin, MarkAsCore, ReflectReference,
    ScriptTypeRegistration, ScriptValue, ThreadWorldContainer, garbage_collector,
};
use context::{Context, ContextInitializer, ContextPreHandlingInitializer};
use event::{ScriptCallbackEvent, ScriptCallbackResponseEvent};
use handler::HandlerFn;
use runtime::{Runtime, RuntimeInitializer};
use script::{ContextPolicy, ScriptComponent, ScriptContext};

pub mod callbacks;
pub mod commands;
pub mod config;
pub mod context;
pub mod error;
pub mod event;
pub mod extractors;
pub mod handler;
pub mod pipeline;
pub mod runtime;
pub mod script;
pub mod script_system;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
/// Labels for various BMS systems
pub enum ScriptingSystemSet {
    /// Systems which handle the processing of asset events for script assets, and dispatching internal script asset events
    ScriptAssetDispatch,
    /// Systems which read incoming internal script asset events and produce script lifecycle commands
    ScriptCommandDispatch,
    // /// Systems which read entity removal events and remove contexts associated with them
    // EntityRemoval,
    /// One time runtime initialization systems
    RuntimeInitialization,
    /// Systems which handle the garbage collection of allocated values
    GarbageCollection,
}

/// Types which act like scripting plugins, by selecting a context and runtime
/// Each individual combination of context and runtime has specific infrastructure built for it and does not interact with other scripting plugins
///
/// When implementing a new scripting plugin, also ensure the following implementations exist:
/// - [`Plugin`] for the plugin, both [`Plugin::build`] and [`Plugin::finish`] methods need to be dispatched to the underlying [`ScriptingPlugin`] struct
/// - [`AsMut<ScriptingPlugin<Self>>`] for the plugin struct
pub trait IntoScriptPluginParams: 'static + GetPluginThreadConfig<Self> {
    /// The language of the scripts
    const LANGUAGE: Language;
    /// The context type used for the scripts
    type C: Context;
    /// The runtime type used for the scripts
    type R: Runtime;

    /// Build the runtime
    fn build_runtime() -> Self::R;

    /// Returns the handler function for the plugin
    fn handler() -> HandlerFn<Self>;

    /// Returns the context loader function for the plugin
    fn context_loader() -> ContextLoadFn<Self>;

    /// Returns the context reloader function for the plugin
    fn context_reloader() -> ContextReloadFn<Self>;
}

/// Bevy plugin enabling scripting within the bevy mod scripting framework
pub struct ScriptingPlugin<P: IntoScriptPluginParams> {
    /// Functions configuring the runtime after it is created
    pub runtime_initializers: Vec<RuntimeInitializer<P>>,

    /// The strategy used to assign contexts to scripts
    pub context_policy: ContextPolicy,

    /// The language this plugin declares
    pub language: Language,

    /// Declares the file extensions this plugin supports
    pub supported_extensions: Vec<&'static str>,

    /// initializers for the contexts, run when loading the script
    pub context_initializers: Vec<ContextInitializer<P>>,

    /// initializers for the contexts run every time before handling events
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,

    /// Whether to emit responses from core script callbacks like `on_script_loaded` or `on_script_unloaded`.
    pub emit_responses: bool,

    /// The settings customising the processing (loading, unloading etc.) pipeline for this plugin
    pub processing_pipeline_plugin: ScriptLoadingPipeline<P>,
}

impl<P> std::fmt::Debug for ScriptingPlugin<P>
where
    P: IntoScriptPluginParams,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptingPlugin")
            .field("context_policy", &self.context_policy)
            .field("language", &self.language)
            .field("context_initializers", &self.context_initializers)
            .field(
                "context_pre_handling_initializers",
                &self.context_pre_handling_initializers,
            )
            .field("emit_responses", &self.emit_responses)
            .finish()
    }
}

impl<P: IntoScriptPluginParams> Default for ScriptingPlugin<P> {
    fn default() -> Self {
        Self {
            runtime_initializers: Default::default(),
            context_policy: ContextPolicy::default(),
            language: Default::default(),
            supported_extensions: Default::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
            emit_responses: false,
            processing_pipeline_plugin: Default::default(),
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> Plugin for ScriptingPlugin<P> {
    fn build(&self, app: &mut App) {
        // initialize thread local configs

        let runtime = P::build_runtime();
        for initializer in &self.runtime_initializers {
            if let Err(e) = initializer(&runtime) {
                error!("Error initializing runtime: {:?}. Continuing.", e);
            }
        }

        let config = ScriptingPluginConfiguration::<P> {
            pre_handling_callbacks: Vec::leak(self.context_pre_handling_initializers.clone()),
            context_initialization_callbacks: Vec::leak(self.context_initializers.clone()),
            emit_responses: self.emit_responses,
            runtime: Box::leak(Box::new(runtime)),
            language_extensions: Box::leak(Box::new(LanguageExtensions::new(
                self.supported_extensions
                    .iter()
                    .map(|&ext| (ext, P::LANGUAGE.clone())),
            ))),
        };

        P::set_world_local_config(app.world().id(), config);

        app.insert_resource(ScriptContext::<P>::new(self.context_policy.clone()));
        app.register_asset_loader(ScriptAssetLoader::new(config.language_extensions));

        app.add_plugins((
            self.processing_pipeline_plugin.clone(),
            ScriptCallbacksPlugin::<P>::default(),
        ));

        register_types(app);
    }
}

impl<P: IntoScriptPluginParams> ScriptingPlugin<P> {
    /// Adds a context initializer to the plugin
    ///
    /// Initializers will be run every time a context is loaded or re-loaded and before any events are handled
    pub fn add_context_initializer(&mut self, initializer: ContextInitializer<P>) -> &mut Self {
        self.context_initializers.push(initializer);
        self
    }

    /// Adds a context pre-handling initializer to the plugin.
    ///
    /// Initializers will be run every time before handling events and after the context is loaded or re-loaded.
    pub fn add_context_pre_handling_initializer(
        &mut self,
        initializer: ContextPreHandlingInitializer<P>,
    ) -> &mut Self {
        self.context_pre_handling_initializers.push(initializer);
        self
    }

    /// Adds a runtime initializer to the plugin.
    ///
    /// Initializers will be run after the runtime is created, but before any contexts are loaded.
    pub fn add_runtime_initializer(&mut self, initializer: RuntimeInitializer<P>) -> &mut Self {
        self.runtime_initializers.push(initializer);
        self
    }

    /// Sets the script pipeline settings plugin
    pub fn set_pipeline_settings(&mut self, pipeline: ScriptLoadingPipeline<P>) -> &mut Self {
        self.processing_pipeline_plugin = pipeline;
        self
    }
}

/// Utility trait for configuring all scripting plugins.
pub trait ConfigureScriptPlugin {
    /// The type of the plugin to configure
    type P: IntoScriptPluginParams;

    /// Add a context initializer to the plugin
    fn add_context_initializer(self, initializer: ContextInitializer<Self::P>) -> Self;

    /// Add a context pre-handling initializer to the plugin
    fn add_context_pre_handling_initializer(
        self,
        initializer: ContextPreHandlingInitializer<Self::P>,
    ) -> Self;

    /// Add a runtime initializer to the plugin
    fn add_runtime_initializer(self, initializer: RuntimeInitializer<Self::P>) -> Self;

    /// Switch the context assigning strategy to the given policy.
    ///
    /// Some context policies might work in unexpected ways.
    /// For example, a single shared context might cause issues with scripts overriding each other's handlers.
    fn set_context_policy(self, context_policy: ContextPolicy) -> Self;

    /// Whether to emit responses from core script callbacks like `on_script_loaded` or `on_script_unloaded`.
    /// By default, this is `false` and responses are not emitted.
    ///
    /// You won't be able to react to these events until after contexts are fully loaded,
    /// but they might be useful for other purposes, such as debugging or logging.
    fn emit_core_callback_responses(self, emit_responses: bool) -> Self;

    /// Adds a supported file extension for the plugin's language.
    fn add_supported_extension(self, extension: &'static str) -> Self;

    /// removes a supported file extension for the plugin's language.
    fn remove_supported_extension(self, extension: &'static str) -> Self;

    /// Sets the script pipeline settings plugin
    fn set_pipeline_settings(self, pipeline: ScriptLoadingPipeline<Self::P>) -> Self;
}

impl<P: IntoScriptPluginParams + AsMut<ScriptingPlugin<P>>> ConfigureScriptPlugin for P {
    type P = P;

    fn add_context_initializer(mut self, initializer: ContextInitializer<Self::P>) -> Self {
        self.as_mut().add_context_initializer(initializer);
        self
    }

    fn add_context_pre_handling_initializer(
        mut self,
        initializer: ContextPreHandlingInitializer<Self::P>,
    ) -> Self {
        self.as_mut()
            .add_context_pre_handling_initializer(initializer);
        self
    }

    fn add_runtime_initializer(mut self, initializer: RuntimeInitializer<Self::P>) -> Self {
        self.as_mut().add_runtime_initializer(initializer);
        self
    }

    fn set_context_policy(mut self, policy: ContextPolicy) -> Self {
        self.as_mut().context_policy = policy;
        self
    }

    fn emit_core_callback_responses(mut self, emit_responses: bool) -> Self {
        self.as_mut().emit_responses = emit_responses;
        self
    }

    fn add_supported_extension(mut self, extension: &'static str) -> Self {
        self.as_mut().supported_extensions.push(extension);
        self
    }

    fn remove_supported_extension(mut self, extension: &'static str) -> Self {
        self.as_mut()
            .supported_extensions
            .retain(|&ext| ext != extension);
        self
    }

    fn set_pipeline_settings(mut self, pipeline: ScriptLoadingPipeline<P>) -> Self {
        self.as_mut().set_pipeline_settings(pipeline);
        self
    }
}

/// Ensures all types with `ReflectComponent` type data are pre-registered with component ID's
fn pre_register_components(app: &mut App) {
    let type_registry = app
        .world_mut()
        .get_resource_or_init::<AppTypeRegistry>()
        .clone();
    let type_registry = type_registry.read();

    let world = app.world_mut();
    for (_, data) in type_registry.iter_with_data::<ReflectComponent>() {
        data.register_component(world);
    }
}

/// A plugin defining shared settings between various scripting plugins
/// It is necessary to register this plugin for any of them to work
#[derive(Default)]
pub struct BMSScriptingInfrastructurePlugin {
    /// If set to true will not log all ScriptErrorEvents using bevy_log::error.
    ///
    /// you can opt out of this behavior if you want to log the errors in a different way.
    ///
    /// see the [`crate::handler::script_error_logger`] system.
    dont_log_script_event_errors: bool,
}

impl Plugin for BMSScriptingInfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScriptErrorEvent>()
            .add_event::<ScriptCallbackEvent>()
            .add_event::<ScriptCallbackResponseEvent>()
            .init_resource::<AppReflectAllocator>()
            .init_asset::<ScriptAsset>()
            .init_resource::<AppScriptFunctionRegistry>()
            .init_resource::<DummyScriptFunctionRegistry>()
            .insert_resource(AppScheduleRegistry::new());

        app.register_type::<ScriptAsset>();
        app.register_type::<Handle<ScriptAsset>>();
        app.register_type_data::<Handle<ScriptAsset>, MarkAsCore>();

        app.add_systems(
            PostUpdate,
            ((garbage_collector).in_set(ScriptingSystemSet::GarbageCollection),),
        );

        if !self.dont_log_script_event_errors {
            app.add_systems(PostUpdate, script_error_logger);
        }

        let _ = bevy_mod_scripting_display::GLOBAL_TYPE_INFO_PROVIDER
            .set(|| Some(&ThreadWorldContainer));

        DynamicScriptComponentPlugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        // Pre-register component IDs.
        pre_register_components(app);
        DynamicScriptComponentPlugin.finish(app);
    }
}

/// Register all types that need to be accessed via reflection
fn register_types(app: &mut App) {
    app.register_type::<ScriptValue>();
    app.register_type::<ScriptTypeRegistration>();
    app.register_type::<ReflectReference>();
    app.register_type::<ScriptComponent>();
}

#[cfg(test)]
mod test {
    use bevy_asset::AssetPlugin;
    use bevy_ecs::prelude::*;
    use bevy_reflect::Reflect;

    use super::*;

    #[test]
    fn test_reflect_component_is_preregistered_in_app_finalize() {
        let mut app = App::new();

        app.add_plugins(AssetPlugin::default());

        #[derive(Component, Reflect)]
        #[reflect(Component)]
        struct Comp;

        app.register_type::<Comp>();

        assert!(app.world_mut().component_id::<Comp>().is_none());

        BMSScriptingInfrastructurePlugin::default().finish(&mut app);

        assert!(app.world_mut().component_id::<Comp>().is_some());
    }
}
