use crate::event::ScriptErrorEvent;
use asset::{
    AssetPathToLanguageMapper, Language, ScriptAsset, ScriptAssetLoader, ScriptAssetSettings,
    ScriptMetadataStore,
};
use bevy::prelude::*;
use bindings::{
    function::script_function::AppScriptFunctionRegistry, script_value::ScriptValue,
    AppReflectAllocator, ReflectAllocator, ReflectReference, ScriptTypeRegistration,
    WorldCallbackAccess,
};
use context::{
    Context, ContextAssigner, ContextBuilder, ContextInitializer, ContextLoadingSettings,
    ContextPreHandlingInitializer, ScriptContexts,
};
use docs::{Documentation, DocumentationFragment};
use event::ScriptCallbackEvent;
use handler::{CallbackSettings, HandlerFn};

use runtime::{Runtime, RuntimeContainer, RuntimeInitializer, RuntimeSettings};
use script::Scripts;
use systems::{
    garbage_collector, initialize_runtime, insert_script_metadata, remove_script_metadata,
    sync_script_data, ScriptingSystemSet,
};

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
pub mod systems;

/// Types which act like scripting plugins, by selecting a context and runtime
/// Each individual combination of context and runtime has specific infrastructure built for it and does not interact with other scripting plugins
pub trait IntoScriptPluginParams: 'static {
    const LANGUAGE: Language;
    type C: Context;
    type R: Runtime;

    fn build_runtime() -> Self::R;

    // fn supported_language() -> Language;
}

/// Bevy plugin enabling scripting within the bevy mod scripting framework
pub struct ScriptingPlugin<P: IntoScriptPluginParams> {
    /// Settings for the runtime
    pub runtime_settings: Option<RuntimeSettings<P>>,
    /// The handler used for executing callbacks in scripts
    pub callback_handler: Option<HandlerFn<P>>,
    /// The context builder for loading contexts
    pub context_builder: Option<ContextBuilder<P>>,
    /// The context assigner for assigning contexts to scripts, if not provided default strategy of keeping each script in its own context is used
    pub context_assigner: Option<ContextAssigner<P>>,
    pub language_mapper: Option<AssetPathToLanguageMapper>,

    /// initializers for the contexts, run when loading the script
    pub context_initializers: Vec<ContextInitializer<P>>,
    /// initializers for the contexts run every time before handling events
    pub context_pre_handling_initializers: Vec<ContextPreHandlingInitializer<P>>,
}

impl<P: IntoScriptPluginParams> Default for ScriptingPlugin<P>
where
    P::R: Default,
{
    fn default() -> Self {
        Self {
            runtime_settings: Default::default(),
            callback_handler: Default::default(),
            context_builder: Default::default(),
            context_assigner: Default::default(),
            language_mapper: Default::default(),
            context_initializers: Default::default(),
            context_pre_handling_initializers: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> Plugin for ScriptingPlugin<P> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.runtime_settings.as_ref().cloned().unwrap_or_default())
            .insert_non_send_resource::<RuntimeContainer<P>>(RuntimeContainer {
                runtime: P::build_runtime(),
            })
            .init_non_send_resource::<ScriptContexts<P>>()
            .insert_resource::<CallbackSettings<P>>(CallbackSettings {
                callback_handler: self.callback_handler,
            })
            .insert_resource::<ContextLoadingSettings<P>>(ContextLoadingSettings {
                loader: self.context_builder.clone(),
                assigner: Some(self.context_assigner.clone().unwrap_or_default()),
                context_initializers: vec![],
                context_pre_handling_initializers: vec![],
            });

        register_script_plugin_systems::<P>(app);
        once_per_app_init(app);

        if let Some(language_mapper) = &self.language_mapper {
            app.world_mut()
                .resource_mut::<ScriptAssetSettings>()
                .as_mut()
                .script_language_mappers
                .push(*language_mapper);
        }

        register_types(app);

        for initializer in self.context_initializers.iter() {
            app.add_context_initializer::<P>(*initializer);
        }

        for initializer in self.context_pre_handling_initializers.iter() {
            app.add_context_pre_handling_initializer::<P>(*initializer);
        }
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
        self.runtime_settings
            .get_or_insert_with(Default::default)
            .initializers
            .push(initializer);
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
        .init_resource::<ScriptAssetSettings>()
        .init_resource::<Scripts>()
        .init_resource::<ScriptMetadataStore>()
        .init_asset::<ScriptAsset>()
        .init_resource::<AppScriptFunctionRegistry>()
        .register_asset_loader(ScriptAssetLoader {
            extensions: &[],
            preprocessor: None,
        });

    app.add_systems(
        PostUpdate,
        (
            (garbage_collector).in_set(ScriptingSystemSet::GarbageCollection),
            (insert_script_metadata).in_set(ScriptingSystemSet::ScriptMetadataInsertion),
            (remove_script_metadata).in_set(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    )
    .configure_sets(
        PostUpdate,
        (
            ScriptingSystemSet::ScriptMetadataInsertion.after(bevy::asset::TrackAssets),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptMetadataInsertion)
                .before(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    );
}

/// Systems registered per-language
fn register_script_plugin_systems<P: IntoScriptPluginParams>(app: &mut App) {
    app.add_systems(
        PostStartup,
        (initialize_runtime::<P>).in_set(ScriptingSystemSet::RuntimeInitialization),
    )
    .add_systems(
        PostUpdate,
        ((sync_script_data::<P>).in_set(ScriptingSystemSet::ScriptCommandDispatch),),
    );
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

pub trait AddContextInitializer {
    fn add_context_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: ContextInitializer<P>,
    ) -> &mut Self;
}

impl AddContextInitializer for App {
    fn add_context_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: ContextInitializer<P>,
    ) -> &mut Self {
        self.world_mut()
            .init_resource::<ContextLoadingSettings<P>>();
        self.world_mut()
            .resource_mut::<ContextLoadingSettings<P>>()
            .as_mut()
            .context_initializers
            .push(initializer);
        self
    }
}

pub trait AddContextPreHandlingInitializer {
    fn add_context_pre_handling_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: ContextPreHandlingInitializer<P>,
    ) -> &mut Self;
}

impl AddContextPreHandlingInitializer for App {
    fn add_context_pre_handling_initializer<P: IntoScriptPluginParams>(
        &mut self,
        initializer: ContextPreHandlingInitializer<P>,
    ) -> &mut Self {
        self.world_mut()
            .resource_mut::<ContextLoadingSettings<P>>()
            .as_mut()
            .context_pre_handling_initializers
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

#[cfg(test)]
mod test {
    use asset::ScriptMetadataStore;

    use super::*;

    #[test]
    fn test_default_scripting_plugin_initializes_all_resources_correctly() {
        let mut app = App::new();

        #[derive(Default, Clone)]
        struct C;
        #[derive(Default, Clone)]
        struct R;

        struct Plugin;

        impl IntoScriptPluginParams for Plugin {
            type C = C;
            type R = R;
            const LANGUAGE: Language = Language::Unknown;

            fn build_runtime() -> Self::R {
                R
            }
        }

        app.add_plugins(AssetPlugin::default());
        app.add_plugins(ScriptingPlugin::<Plugin>::default());

        assert!(app.world().contains_resource::<Scripts>());
        assert!(app.world().contains_resource::<AppTypeRegistry>());
        assert!(app.world().contains_resource::<ScriptAssetSettings>());
        assert!(app.world().contains_resource::<RuntimeSettings<Plugin>>());
        assert!(app.world().contains_resource::<CallbackSettings<Plugin>>());
        assert!(app
            .world()
            .contains_resource::<ContextLoadingSettings<Plugin>>());
        assert!(app.world().contains_non_send::<RuntimeContainer<Plugin>>());
        assert!(app.world().contains_non_send::<ScriptContexts<Plugin>>());
        assert!(app.world().contains_resource::<ScriptMetadataStore>());
    }
}
