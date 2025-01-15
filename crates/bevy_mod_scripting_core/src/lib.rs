use crate::event::ScriptErrorEvent;
use asset::{
    configure_asset_systems, configure_asset_systems_for_plugin, AssetPathToLanguageMapper,
    Language, ScriptAsset, ScriptAssetLoader, ScriptAssetSettings,
};
use bevy::prelude::*;
use bindings::{
    function::script_function::AppScriptFunctionRegistry, garbage_collector,
    script_value::ScriptValue, AppReflectAllocator, ReflectAllocator, ReflectReference,
    ScriptTypeRegistration, WorldCallbackAccess,
};
use context::{
    Context, ContextAssigner, ContextBuilder, ContextInitializer, ContextLoadingSettings,
    ContextPreHandlingInitializer, ScriptContexts,
};
use docs::{Documentation, DocumentationFragment};
use event::ScriptCallbackEvent;
use handler::{CallbackSettings, HandlerFn};
use runtime::{initialize_runtime, Runtime, RuntimeContainer, RuntimeInitializer, RuntimeSettings};
use script::Scripts;

pub mod asset;
pub mod bindings;
pub mod commands;
pub mod context;
pub mod docs;
pub mod error;
pub mod event;
pub mod handler;
pub mod reflection_extensions;
pub mod runtime;
pub mod script;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
/// Labels for various BMS systems
pub enum ScriptingSystemSet {
    // Post Setup processes
    RuntimeInitialization,

    // Post Update processes
    GarbageCollection,

    ScriptMetadataInsertion,
    ScriptCommandDispatch,
    ScriptMetadataRemoval,
}

/// Types which act like scripting plugins, by selecting a context and runtime
/// Each individual combination of context and runtime has specific infrastructure built for it and does not interact with other scripting plugins
pub trait IntoScriptPluginParams: 'static {
    const LANGUAGE: Language;
    type C: Context;
    type R: Runtime;

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
    /// Initializers will be run every time a context is loaded or re-loaded
    pub fn add_context_initializer(&mut self, initializer: ContextInitializer<P>) -> &mut Self {
        self.context_initializers.push(initializer);
        self
    }

    /// Adds a context pre-handling initializer to the plugin.
    ///
    /// Initializers will be run every time before handling events.
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
    type P: IntoScriptPluginParams;
    fn add_context_initializer(self, initializer: ContextInitializer<Self::P>) -> Self;
    fn add_context_pre_handling_initializer(
        self,
        initializer: ContextPreHandlingInitializer<Self::P>,
    ) -> Self;
    fn add_runtime_initializer(self, initializer: RuntimeInitializer<Self::P>) -> Self;
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
        (initialize_runtime::<P>).in_set(ScriptingSystemSet::RuntimeInitialization),
    );

    configure_asset_systems_for_plugin::<P>(app);
}

/// Register all types that need to be accessed via reflection
fn register_types(app: &mut App) {
    app.register_type::<WorldCallbackAccess>();
    app.register_type::<ScriptValue>();
    app.register_type::<ScriptTypeRegistration>();
    app.register_type::<ReflectReference>();
}

pub trait AddRuntimeInitializer {
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

pub trait StoreDocumentation<D: DocumentationFragment> {
    /// Adds a documentation fragment to the documentation store.
    fn add_documentation_fragment(&mut self, fragment: D) -> &mut Self;
    /// Consumes all the stored documentation fragments, and merges them into one, then generates the documentation.
    fn generate_docs(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl<D: DocumentationFragment> StoreDocumentation<D> for App {
    fn add_documentation_fragment(&mut self, fragment: D) -> &mut Self {
        self.world_mut()
            .init_non_send_resource::<Documentation<D>>();
        self.world_mut()
            .non_send_resource_mut::<Documentation<D>>()
            .as_mut()
            .fragments
            .push(fragment);
        self
    }

    fn generate_docs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut docs = match self
            .world_mut()
            .remove_non_send_resource::<Documentation<D>>()
        {
            Some(docs) => docs,
            None => return Ok(()),
        };

        let mut top_fragment = match docs.fragments.pop() {
            Some(fragment) => fragment,
            None => return Ok(()),
        };

        for fragment in docs.fragments.into_iter() {
            top_fragment = top_fragment.merge(fragment);
        }

        top_fragment.gen_docs()
    }
}
