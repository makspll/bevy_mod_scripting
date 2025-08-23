//! Core functionality for the bevy_mod_scripting framework.
//!
//! Contains language agnostic systems and types for handling scripting in bevy.

use crate::{
    bindings::MarkAsCore,
    context::{ContextLoadFn, ContextReloadFn},
    event::ScriptErrorEvent,
};
use asset::{
    Language, ScriptAsset, ScriptAssetLoader, configure_asset_systems,
    configure_asset_systems_for_plugin,
};
use bevy_app::{App, Plugin, PostStartup, PostUpdate};
use bevy_asset::{AssetApp, Handle};
use bevy_ecs::{
    reflect::{AppTypeRegistry, ReflectComponent},
    resource::Resource,
    schedule::SystemSet,
    system::{Command, In},
};
use bevy_ecs::{schedule::IntoScheduleConfigs, system::IntoSystem};
use bevy_log::error;
use bevy_platform::collections::HashMap;
use bindings::{
    AppReflectAllocator, DynamicScriptComponentPlugin, ReflectAllocator, ReflectReference,
    ScriptTypeRegistration, function::script_function::AppScriptFunctionRegistry,
    garbage_collector, schedule::AppScheduleRegistry, script_value::ScriptValue,
};
use commands::{AddStaticScript, RemoveStaticScript};
use context::{Context, ContextInitializer, ContextLoadingSettings, ContextPreHandlingInitializer};
use error::ScriptError;
use event::{ScriptCallbackEvent, ScriptCallbackResponseEvent, ScriptEvent};
use handler::HandlerFn;
use runtime::{Runtime, RuntimeContainer, RuntimeInitializer, RuntimeSettings, initialize_runtime};
use script::{ContextPolicy, ScriptComponent, ScriptContext, StaticScripts};
use std::ops::{Deref, DerefMut};

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

    /// Returns the handler function for the plugin
    fn handler() -> HandlerFn<Self>;

    /// Returns the context loader function for the plugin
    fn context_loader() -> ContextLoadFn<Self>;

    /// Returns the context reloader function for the plugin
    fn context_reloader() -> ContextReloadFn<Self>;
}

/// Bevy plugin enabling scripting within the bevy mod scripting framework
pub struct ScriptingPlugin<P: IntoScriptPluginParams> {
    /// Settings for the runtime
    pub runtime_settings: RuntimeSettings<P>,

    /// The strategy used to assign contexts to scripts
    pub context_policy: ContextPolicy,

    /// The language this plugin declares
    pub language: Language,

    /// initializers for the contexts, run when loading the script
    pub context_initializers: Vec<ContextInitializer<P>>,

    /// initializers for the contexts run every time before handling events
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,

    /// Whether to emit responses from core script callbacks like `on_script_loaded` or `on_script_unloaded`.
    pub emit_responses: bool,
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
            runtime_settings: Default::default(),
            context_policy: ContextPolicy::default(),
            language: Default::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
            emit_responses: false,
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> Plugin for ScriptingPlugin<P> {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.runtime_settings.clone())
            .insert_resource::<RuntimeContainer<P>>(RuntimeContainer {
                runtime: P::build_runtime(),
            })
            .insert_resource::<ContextLoadingSettings<P>>(ContextLoadingSettings {
                context_initializers: self.context_initializers.clone(),
                context_pre_handling_initializers: self.context_pre_handling_initializers.clone(),
                emit_responses: self.emit_responses,
            });

        app.insert_resource(ScriptContext::<P>::new(self.context_policy.clone()));

        register_script_plugin_systems::<P>(app);

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
pub struct BMSScriptingInfrastructurePlugin;

impl Plugin for BMSScriptingInfrastructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScriptErrorEvent>()
            .add_event::<ScriptEvent>()
            .add_event::<ScriptCallbackEvent>()
            .add_event::<ScriptCallbackResponseEvent>()
            .init_resource::<AppReflectAllocator>()
            .init_resource::<StaticScripts>()
            .init_asset::<ScriptAsset>()
            .init_resource::<AppScriptFunctionRegistry>()
            .insert_resource(AppScheduleRegistry::new());

        app.register_type::<ScriptAsset>();
        app.register_type::<Handle<ScriptAsset>>();
        app.register_type_data::<Handle<ScriptAsset>, MarkAsCore>();

        app.add_systems(
            PostUpdate,
            ((garbage_collector).in_set(ScriptingSystemSet::GarbageCollection),),
        );

        app.add_plugins(configure_asset_systems);

        DynamicScriptComponentPlugin.build(app);
    }

    fn finish(&self, app: &mut App) {
        // Read extensions.
        let language_extensions = app
            .world_mut()
            .remove_resource::<LanguageExtensions>()
            .unwrap_or_default();
        app.register_asset_loader(ScriptAssetLoader::new(language_extensions));
        // Pre-register component IDs.
        pre_register_components(app);
        DynamicScriptComponentPlugin.finish(app);
    }
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

    app.add_plugins(configure_asset_systems_for_plugin::<P>);
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
    fn add_static_script(&mut self, script_id: impl Into<Handle<ScriptAsset>>) -> &mut Self;

    /// Removes a script id from the list of static scripts.
    ///
    /// Does nothing if the script id is not in the list.
    fn remove_static_script(&mut self, script_id: impl Into<Handle<ScriptAsset>>) -> &mut Self;
}

impl ManageStaticScripts for App {
    fn add_static_script(&mut self, script_id: impl Into<Handle<ScriptAsset>>) -> &mut Self {
        AddStaticScript::new(script_id.into()).apply(self.world_mut());
        self
    }

    fn remove_static_script(&mut self, script_id: impl Into<Handle<ScriptAsset>>) -> &mut Self {
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

/// Collect the language extensions supported during initialization.
///
/// NOTE: This resource is removed after plugin setup.
#[derive(Debug, Resource)]
pub struct LanguageExtensions(HashMap<&'static str, Language>);

impl Deref for LanguageExtensions {
    type Target = HashMap<&'static str, Language>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LanguageExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Default for LanguageExtensions {
    fn default() -> Self {
        LanguageExtensions(
            [
                ("lua", Language::Lua),
                ("luau", Language::Lua),
                ("rhai", Language::Rhai),
                ("rn", Language::Rune),
            ]
            .into_iter()
            .collect(),
        )
    }
}

impl ConfigureScriptAssetSettings for App {
    fn add_supported_script_extensions(
        &mut self,
        extensions: &[&'static str],
        language: Language,
    ) -> &mut Self {
        let mut language_extensions = self
            .world_mut()
            .get_resource_or_init::<LanguageExtensions>();

        for extension in extensions {
            language_extensions.insert(extension, language.clone());
        }
        self
    }
}

#[cfg(test)]
mod test {
    use bevy_asset::{AssetPlugin, AssetServer};
    use bevy_ecs::prelude::*;
    use bevy_reflect::Reflect;

    use super::*;

    #[tokio::test]
    async fn test_asset_extensions_correctly_accumulate() {
        let mut app = App::new();
        app.add_plugins(AssetPlugin::default());

        BMSScriptingInfrastructurePlugin.finish(&mut app);

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

        BMSScriptingInfrastructurePlugin.finish(&mut app);

        assert!(app.world_mut().component_id::<Comp>().is_some());
    }
}
