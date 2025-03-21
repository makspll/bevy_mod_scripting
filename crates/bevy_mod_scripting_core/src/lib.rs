//! Core functionality for the bevy_mod_scripting framework.
//!
//! Contains language agnostic systems and types for handling scripting in bevy.

use crate::event::ScriptErrorEvent;
use asset::{
    configure_asset_systems, configure_asset_systems_for_plugin, Language, ScriptAsset,
    ScriptAssetLoader, ScriptAssetSettings,
};
use bevy::prelude::*;
use bindings::{
    function::script_function::AppScriptFunctionRegistry,
    garbage_collector,
    globals::{core::CoreScriptGlobalsPlugin, AppScriptGlobalsRegistry},
    schedule::AppScheduleRegistry,
    script_value::ScriptValue,
    AppReflectAllocator, DynamicScriptComponentPlugin, ReflectAllocator, ReflectReference,
    ScriptTypeRegistration,
};
use commands::{AddStaticScript, RemoveStaticScript};
use context::{
    Context, ContextAssignmentStrategy, ContextBuilder, ContextInitializer, ContextLoadingSettings,
    ContextPreHandlingInitializer,
};
use error::ScriptError;
use event::ScriptCallbackEvent;
use handler::{CallbackSettings, HandlerFn};
use runtime::{initialize_runtime, Runtime, RuntimeContainer, RuntimeInitializer, RuntimeSettings};
use script::{ScriptComponent, ScriptId, Scripts, StaticScripts};

pub mod asset;
pub mod bindings;
pub mod commands;
pub mod context;
pub mod docgen;
pub mod error;
pub mod event;
pub mod extractors;
pub mod handler;
pub mod reflection_extensions;
pub mod runtime;
pub mod script;

pub(crate) mod private;

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
///
/// When implementing a new scripting plugin, also ensure the following implementations exist:
/// - [`Plugin`] for the plugin, both [`Plugin::build`] and [`Plugin::finish`] methods need to be dispatched to the underlying [`ScriptingPlugin`] struct
/// - [`AsMut<ScriptingPlugin<Self>`] for the plugin struct
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

    /// The strategy for assigning contexts to scripts
    pub context_assignment_strategy: ContextAssignmentStrategy,

    /// The language this plugin declares
    pub language: Language,
    /// Supported extensions to be added to the asset settings without the dot
    /// By default BMS populates a set of extensions for the languages it supports.
    pub additional_supported_extensions: &'static [&'static str],

    /// initializers for the contexts, run when loading the script
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// initializers for the contexts run every time before handling events
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ScriptingPlugin<P> {
    fn default() -> Self {
        Self {
            runtime_settings: Default::default(),
            callback_handler: CallbackSettings::<P>::default().callback_handler,
            context_builder: Default::default(),
            context_assignment_strategy: Default::default(),
            language: Default::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
            additional_supported_extensions: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> Plugin for ScriptingPlugin<P> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.runtime_settings.clone())
            .insert_resource::<RuntimeContainer<P>>(RuntimeContainer {
                runtime: P::build_runtime(),
            })
            .insert_resource::<CallbackSettings<P>>(CallbackSettings {
                callback_handler: self.callback_handler,
            })
            .insert_resource::<ContextLoadingSettings<P>>(ContextLoadingSettings {
                loader: self.context_builder.clone(),
                assignment_strategy: self.context_assignment_strategy,
                context_initializers: self.context_initializers.clone(),
                context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
            })
            .init_resource::<Scripts<P>>();

        register_script_plugin_systems::<P>(app);

        // add extension for the language to the asset loader
        once_per_app_init(app);

        if !self.additional_supported_extensions.is_empty() {
            app.add_supported_script_extensions(
                self.additional_supported_extensions,
                self.language.clone(),
            );
        }

        register_types(app);
    }

    fn finish(&self, app: &mut App) {
        once_per_app_finalize(app);
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
    /// Be careful however as this also means that scripts can interfere with each other in unexpected ways! Including overwriting each other's handlers.
    fn enable_context_sharing(self) -> Self;

    /// Set the set of extensions to be added for the plugin's language.
    ///
    /// This is useful for adding extensions that are not supported by default by BMS.
    fn set_additional_supported_extensions(self, extensions: &'static [&'static str]) -> Self;
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

    fn enable_context_sharing(mut self) -> Self {
        self.as_mut().context_assignment_strategy = ContextAssignmentStrategy::Global;
        self
    }

    fn set_additional_supported_extensions(mut self, extensions: &'static [&'static str]) -> Self {
        self.as_mut().additional_supported_extensions = extensions;
        self
    }
}

fn once_per_app_finalize(app: &mut App) {
    #[derive(Resource)]
    struct BMSFinalized;

    if app.world().contains_resource::<BMSFinalized>() {
        return;
    }
    app.insert_resource(BMSFinalized);

    // read extensions from asset settings
    let asset_settings_extensions = app
        .world_mut()
        .get_resource_or_init::<ScriptAssetSettings>()
        .supported_extensions;

    // convert extensions to static array
    bevy::log::info!(
        "Initializing BMS with Supported extensions: {:?}",
        asset_settings_extensions
    );

    app.register_asset_loader(ScriptAssetLoader {
        extensions: asset_settings_extensions,
        preprocessor: None,
    });

    // pre-register component id's
    pre_register_componnents(app);
}

/// Ensures all types with `ReflectComponent` type data are pre-registered with component ID's
fn pre_register_componnents(app: &mut App) {
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
        .init_resource::<StaticScripts>()
        .init_asset::<ScriptAsset>()
        .init_resource::<AppScriptFunctionRegistry>()
        .init_resource::<AppScriptGlobalsRegistry>()
        .insert_resource(AppScheduleRegistry::new());

    app.add_systems(
        PostUpdate,
        ((garbage_collector).in_set(ScriptingSystemSet::GarbageCollection),),
    );

    app.add_plugins((CoreScriptGlobalsPlugin, DynamicScriptComponentPlugin));

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
    app.register_type::<ScriptComponent>();
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

/// Trait for adding static scripts to an app
pub trait ManageStaticScripts {
    /// Registers a script id as a static script.
    ///
    /// Event handlers will run these scripts on top of the entity scripts.
    fn add_static_script(&mut self, script_id: impl Into<ScriptId>) -> &mut Self;

    /// Removes a script id from the list of static scripts.
    ///
    /// Does nothing if the script id is not in the list.
    fn remove_static_script(&mut self, script_id: impl Into<ScriptId>) -> &mut Self;
}

impl ManageStaticScripts for App {
    fn add_static_script(&mut self, script_id: impl Into<ScriptId>) -> &mut Self {
        AddStaticScript::new(script_id.into()).apply(self.world_mut());
        self
    }

    fn remove_static_script(&mut self, script_id: impl Into<ScriptId>) -> &mut Self {
        RemoveStaticScript::new(script_id.into()).apply(self.world_mut());
        self
    }
}

/// Trait for adding a supported extension to the script asset settings.
///
/// This is only valid in the plugin building phase, as the asset loader will be created in the `finalize` phase.
/// Any changes to the asset settings after that will not be reflected in the asset loader.
pub trait ConfigureScriptAssetSettings {
    /// Adds a supported extension to the asset settings
    ///
    /// This is only valid to call in the plugin building phase, as the asset loader will be created in the `finalize` phase.
    fn add_supported_script_extensions(
        &mut self,
        extensions: &[&'static str],
        language: Language,
    ) -> &mut Self;
}

impl ConfigureScriptAssetSettings for App {
    fn add_supported_script_extensions(
        &mut self,
        extensions: &[&'static str],
        language: Language,
    ) -> &mut Self {
        let mut asset_settings = self
            .world_mut()
            .get_resource_or_init::<ScriptAssetSettings>();

        let mut new_arr = Vec::from(asset_settings.supported_extensions);

        new_arr.extend(extensions);

        let new_arr_static = Vec::leak(new_arr);

        asset_settings.supported_extensions = new_arr_static;
        for extension in extensions {
            asset_settings
                .extension_to_language_map
                .insert(*extension, language.clone());
        }

        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_asset_extensions_correctly_accumulate() {
        let mut app = App::new();
        app.init_resource::<ScriptAssetSettings>();
        app.add_plugins(AssetPlugin::default());

        app.world_mut()
            .resource_mut::<ScriptAssetSettings>()
            .supported_extensions = &["lua", "rhai"];

        once_per_app_finalize(&mut app);

        let asset_loader = app
            .world()
            .get_resource::<AssetServer>()
            .expect("Asset loader not found");

        asset_loader
            .get_asset_loader_with_extension("lua")
            .await
            .expect("Lua loader not found");

        asset_loader
            .get_asset_loader_with_extension("rhai")
            .await
            .expect("Rhai loader not found");
    }

    #[test]
    fn test_reflect_component_is_preregistered_in_app_finalize() {
        let mut app = App::new();

        app.add_plugins(AssetPlugin::default());

        #[derive(Component, Reflect)]
        #[reflect(Component)]
        struct Comp;

        app.register_type::<Comp>();

        assert!(app.world_mut().component_id::<Comp>().is_none());

        once_per_app_finalize(&mut app);

        assert!(app.world_mut().component_id::<Comp>().is_some());
    }
}
