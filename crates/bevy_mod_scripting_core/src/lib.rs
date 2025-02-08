//! Core functionality for the bevy_mod_scripting framework.
//!
//! Contains language agnostic systems and types for handling scripting in bevy.

use crate::event::ScriptErrorEvent;
use asset::{
    configure_asset_systems, configure_asset_systems_for_plugin, AssetPathToLanguageMapper,
    Language, ScriptAsset, ScriptAssetLoader, ScriptAssetSettings,
};
use bevy::prelude::*;
use bindings::{
    function::script_function::AppScriptFunctionRegistry, garbage_collector,
    script_value::ScriptValue, AppReflectAllocator, ReflectAllocator, ReflectReference,
    ScriptTypeRegistration,
};
use context::{
    Context, ContextAssigner, ContextBuilder, ContextInitializer, ContextLoadingSettings,
    ContextPreHandlingInitializer, ScriptContexts,
};
use error::ScriptError;
use event::ScriptCallbackEvent;
use handler::{CallbackSettings, HandlerFn};
use runtime::{initialize_runtime, Runtime, RuntimeContainer, RuntimeInitializer, RuntimeSettings};
use script::Scripts;

mod extractors;

pub mod asset;
pub mod bindings;
pub mod commands;
pub mod context;
pub mod docgen;
pub mod error;
pub mod event;
pub mod handler;
pub mod reflection_extensions;
pub mod runtime;
pub mod script;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
/// Labels for various BMS systems
pub enum ScriptingSystemSet {
    /// Systems which handle the processing of asset events for script assets, and dispatching internal script asset events
    ScriptAssetDispatch,
    /// Systems which read incoming internal script asset events and produce script lifecycle commands
    ScriptCommandDispatch,
    /// Systems which read incoming script asset events and remove metadata for removed assets
    ScriptMetadataRemoval,

    /// One time runtime initialization systems
    RuntimeInitialization,

    /// Systems which handle the garbage collection of allocated values
    GarbageCollection,
}

/// Types which act like scripting plugins, by selecting a context and runtime
/// Each individual combination of context and runtime has specific infrastructure built for it and does not interact with other scripting plugins
pub trait IntoScriptPluginParams: 'static {
    /// The language of the scripts
    const LANGUAGE: Language;
    /// The context type used for the scripts
    type C: Context;
    /// The runtime type used for the scripts
    type R: Runtime;

    /// Build the runtime
    fn build_runtime() -> Self::R;
}

/// Bevy plugin enabling scripting within the bevy mod scripting framework
pub struct ScriptingPlugin<P: IntoScriptPluginParams> {
    /// Settings for the runtime
    pub runtime_settings: RuntimeSettings<P>,
    /// The handler used for executing callbacks in scripts
    pub callback_handler: HandlerFn<P>,
    /// The context builder for loading contexts
    pub context_builder: ContextBuilder<P>,
    /// The context assigner for assigning contexts to scripts.
    pub context_assigner: ContextAssigner<P>,

    /// The asset path to language mapper for the plugin
    pub language_mapper: AssetPathToLanguageMapper,

    /// initializers for the contexts, run when loading the script
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// initializers for the contexts run every time before handling events
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Plugin for ScriptingPlugin<P> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.runtime_settings.clone())
            .insert_non_send_resource::<RuntimeContainer<P>>(RuntimeContainer {
                runtime: P::build_runtime(),
            })
            .init_non_send_resource::<ScriptContexts<P>>()
            .insert_resource::<CallbackSettings<P>>(CallbackSettings {
                callback_handler: self.callback_handler,
            })
            .insert_resource::<ContextLoadingSettings<P>>(ContextLoadingSettings {
                loader: self.context_builder.clone(),
                assigner: self.context_assigner.clone(),
                context_initializers: self.context_initializers.clone(),
                context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
            });

        register_script_plugin_systems::<P>(app);
        once_per_app_init(app);

        app.world_mut()
            .resource_mut::<ScriptAssetSettings>()
            .as_mut()
            .script_language_mappers
            .push(self.language_mapper);

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
        self.runtime_settings.initializers.push(initializer);
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

    /// Switch the context assigning strategy to a global context assigner.
    ///
    /// This means that all scripts will share the same context. This is useful for when you want to share data between scripts easilly.
    /// Be careful however as this also means that scripts can interfere with each other in unexpected ways!.
    fn enable_context_sharing(self);
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

    fn enable_context_sharing(mut self) {
        self.as_mut().context_assigner = ContextAssigner::new_global_context_assigner();
    }
}

// One of registration of things that need to be done only once per app
fn once_per_app_init(app: &mut App) {
    #[derive(Resource)]
    struct BMSInitialized;

    if app.world().contains_resource::<BMSInitialized>() {
        return;
    }

    app.insert_resource(BMSInitialized);

    app.add_event::<ScriptErrorEvent>()
        .add_event::<ScriptCallbackEvent>()
        .init_resource::<AppReflectAllocator>()
        .init_resource::<Scripts>()
        .init_asset::<ScriptAsset>()
        .init_resource::<AppScriptFunctionRegistry>()
        .register_asset_loader(ScriptAssetLoader {
            extensions: &[],
            preprocessor: None,
        });

    app.add_systems(
        PostUpdate,
        ((garbage_collector).in_set(ScriptingSystemSet::GarbageCollection),),
    );

    configure_asset_systems(app);
}

/// Systems registered per-language
fn register_script_plugin_systems<P: IntoScriptPluginParams>(app: &mut App) {
    app.add_systems(
        PostStartup,
        (initialize_runtime::<P>.pipe(|e: In<Result<(), ScriptError>>| {
            if let Err(e) = e.0 {
                error!("Error initializing runtime: {:?}", e);
            }
        }))
        .in_set(ScriptingSystemSet::RuntimeInitialization),
    );

    configure_asset_systems_for_plugin::<P>(app);
}

/// Register all types that need to be accessed via reflection
fn register_types(app: &mut App) {
    app.register_type::<ScriptValue>();
    app.register_type::<ScriptTypeRegistration>();
    app.register_type::<ReflectReference>();
}

/// Trait for adding a runtime initializer to an app
pub trait AddRuntimeInitializer {
    /// Adds a runtime initializer to the app
    fn add_runtime_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: RuntimeInitializer<P>,
    ) -> &mut Self;
}

impl AddRuntimeInitializer for App {
    fn add_runtime_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: RuntimeInitializer<P>,
    ) -> &mut Self {
        if !self.world_mut().contains_resource::<RuntimeSettings<P>>() {
            self.world_mut().init_resource::<RuntimeSettings<P>>();
        }
        self.world_mut()
            .resource_mut::<RuntimeSettings<P>>()
            .as_mut()
            .initializers
            .push(initializer);
        self
    }
}
